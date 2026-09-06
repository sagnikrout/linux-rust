//! Automatically rewritten from C to Rust
//! Source: tools/firmware/ihex2fw.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-only
//
// Parser/loader for IHEX formatted data.
//
// Copyright © 2008 David Woodhouse <dwmw2@infradead.org>
// Copyright © 2005 Jan Harkes <jaharkes@cs.cmu.edu>
//

// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ihex_binrec {
    pub /: *mut *mut *mut ihex_binrec next; / not part of the real data structure,
    pub addr: u32,
    pub len: u16,
    pub data: [u8; ],
}

//
// nybble/hex are little helpers to parse hexadecimal numbers to a byte value
//
#[no_mangle]
unsafe extern "C" fn nybble(n: u8) -> u8 {
    static uint8_t nybble(const uint8_t n)
    {
    if      (n >= '0' && n <= '9') return n - '0';
    else if (n >= 'A' && n <= 'F') return n - ('A' - 10);
    else if (n >= 'a' && n <= 'f') return n - ('a' - 10);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hex(data: *const u8, crc: *mut u8) -> u8 {
    static uint8_t hex(const uint8_t *data, uint8_t *crc)
    {
    let mut val: u8 = (nybble(data[0]) << 4) | nybble(data[1]);
// crc += val;
    return val;
    }
    static int process_ihex(uint8_t *data, ssize_t size);
    static void file_record(struct ihex_binrec *record);
    static int output_records(int outfd);
    let mut sort_records: static int = 0;
    let mut wide_records: static int = 0;
    let mut include_jump: static int = 0;
#[no_mangle]
unsafe extern "C" fn usage() -> c_int {
    static int usage(void)
    {
    fprintf(stderr, "ihex2fw: Convert ihex files into binary "
    "representation for use by Linux kernel\n");
    fprintf(stderr, "usage: ihex2fw [<options>] <src.HEX> <dst.fw>\n");
    fprintf(stderr, "       -w: wide records (16-bit length)\n");
    fprintf(stderr, "       -s: sort records by address\n");
    fprintf(stderr, "       -j: include records for CS:IP/EIP address\n");
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int infd, outfd;
    struct stat st;
    uint8_t *data;
    int opt;
    while ((opt = getopt(argc, argv, "wsj")) != -1) {
    switch (opt) {
    case 'w':
    wide_records = 1;
    break;
    case 's':
    sort_records = 1;
    break;
    case 'j':
    include_jump = 1;
    break;
    default:
    return usage();
    }
    }
    if (optind + 2 != argc)
    return usage();
    if (!strcmp(argv[optind], "-"))
    infd = 0;
    else
    infd = open(argv[optind], O_RDONLY);
    if (infd == -1) {
    fprintf(stderr, "Failed to open source file: %s",
    strerror(errno));
    return usage();
    }
    if (fstat(infd, &st)) {
    perror("stat");
    return 1;
    }
    data = mmap(core::ptr::null_mut(), st.st_size, PROT_READ, MAP_SHARED, infd, 0);
    if (data == MAP_FAILED) {
    perror("mmap");
    return 1;
    }
    if (!strcmp(argv[optind+1], "-"))
    outfd = 1;
    else
    outfd = open(argv[optind+1], O_TRUNC|O_CREAT|O_WRONLY, 0644);
    if (outfd == -1) {
    fprintf(stderr, "Failed to open destination file: %s",
    strerror(errno));
    return usage();
    }
    if (process_ihex(data, st.st_size))
    return 1;
    return output_records(outfd);
    }
#[no_mangle]
unsafe extern "C" fn process_ihex(data: *mut u8, size: isize) -> c_int {
    static int process_ihex(uint8_t *data, ssize_t size)
    {
    struct ihex_binrec *record;
    size_t record_size;
    let mut offset: u32 = 0;
    uint32_t data32;
    uint8_t type, crc = 0, crcbyte = 0;
    int i, j;
    let mut line: c_int = 1;
    int len;
    i = 0;
    next_record:
// search for the start of record character
    while (i < size) {
    if (data[i] == '\n') line++;
    if (data[i++] == ':') break;
    }
// Minimum record length would be about 10 characters
    if (i + 10 > size) {
    fprintf(stderr, "Can't find valid record at line %d\n", line);
    return -EINVAL;
    }
    len = hex(data + i, &crc); i += 2;
    if (wide_records) {
    len <<= 8;
    len += hex(data + i, &crc); i += 2;
    }
    record_size = ALIGN(sizeof(*record) + len, 4);
    record = malloc(record_size);
    if (!record) {
    fprintf(stderr, "out of memory for records\n");
    return -ENOMEM;
    }
    memset(record, 0, record_size);
    record.len = len;
// now check if we have enough data to read everything
    if (i + 8 + (record.len * 2) > size) {
    fprintf(stderr, "Not enough data to read complete record at line %d\n",
    line);
    return -EINVAL;
    }
    record.addr  = hex(data + i, &crc) << 8; i += 2;
    record.addr |= hex(data + i, &crc); i += 2;
    type = hex(data + i, &crc); i += 2;
    for (j = 0; j < record.len; j++, i += 2)
    record.data[j] = hex(data + i, &crc);
// check CRC
    crcbyte = hex(data + i, &crc); i += 2;
    if (crc != 0) {
    fprintf(stderr, "CRC failure at line %d: got 0x%X, expected 0x%X\n",
    line, crcbyte, (unsigned char)(crcbyte-crc));
    return -EINVAL;
    }
// Done reading the record
    switch (type) {
    case 0:
// old style EOF record?
    if (!record.len)
    break;
    record.addr += offset;
    file_record(record);
    goto next_record;
    case 1: /* End-Of-File Record */
    if (record.addr || record.len) {
    fprintf(stderr, "Bad EOF record (type 01) format at line %d",
    line);
    return -EINVAL;
    }
    break;
    case 2: /* Extended Segment Address Record (HEX86) */
    case 4: /* Extended Linear Address Record (HEX386) */
    if (record.addr || record.len != 2) {
    fprintf(stderr, "Bad HEX86/HEX386 record (type %02X) at line %d\n",
    type, line);
    return -EINVAL;
    }
// We shouldn't really be using the offset for HEX86 because
// the wraparound case is specified quite differently.
    offset = record.data[0] << 8 | record.data[1];
    offset <<= (type == 2 ? 4 : 16);
    goto next_record;
    case 3: /* Start Segment Address Record */
    case 5: /* Start Linear Address Record */
    if (record.addr || record.len != 4) {
    fprintf(stderr, "Bad Start Address record (type %02X) at line %d\n",
    type, line);
    return -EINVAL;
    }
    memcpy(&data32, &record.data[0], sizeof(data32));
    data32 = htonl(data32);
    memcpy(&record.data[0], &data32, sizeof(data32));
// These records contain the CS/IP or EIP where execution
// starts. If requested output this as a record.
    if (include_jump)
    file_record(record);
    goto next_record;
    default:
    fprintf(stderr, "Unknown record (type %02X)\n", type);
    return -EINVAL;
    }
    return 0;
    }
    static struct ihex_binrec *records;
#[no_mangle]
unsafe extern "C" fn file_record(record: *mut ihex_binrec) {
    static void file_record(struct ihex_binrec *record)
    {
    struct ihex_binrec **p = &records;
    while ((*p) && (!sort_records || (*p).addr < record.addr))
    p = &((*p).next);
    record.next = *p;
// p = record;
    }
#[no_mangle]
unsafe extern "C" fn ihex_binrec_size(p: *mut ihex_binrec) -> u16 {
    static uint16_t ihex_binrec_size(struct ihex_binrec *p)
    {
    return p.len + sizeof(p.addr) + sizeof(p.len);
    }
#[no_mangle]
unsafe extern "C" fn output_records(outfd: c_int) -> c_int {
    static int output_records(int outfd)
    {
    unsigned char zeroes[6] = {0, 0, 0, 0, 0, 0};
    struct ihex_binrec *p = records;
    while (p) {
    let mut writelen: u16 = ALIGN(ihex_binrec_size(p), 4);
    p.addr = htonl(p.addr);
    p.len = htons(p.len);
    if (write(outfd, &p.addr, writelen) != writelen)
    return 1;
    p = p.next;
    }
// EOF record is zero length, since we don't bother to represent
    the type field in the binary version */
    if (write(outfd, zeroes, 6) != 6)
    return 1;
    return 0;
    }
