//! Automatically rewritten from C to Rust
//! Source: samples/vfs/test-statx.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Test the statx() system call.
//
// Note that the output of this program is intended to look like the output of
// /bin/stat where possible.
//
// Copyright (C) 2015 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// Macro flag: #define _GNU_SOURCE
// Macro flag: #define _ATFILE_SOURCE

// Work around glibc header silliness

    struct statx;
    struct statx_timestamp;

pub const AT_STATX_SYNC_TYPE: c_uint = 0x6000;
pub const AT_STATX_SYNC_AS_STAT: c_uint = 0x0000;
pub const AT_STATX_FORCE_SYNC: c_uint = 0x2000;
pub const AT_STATX_DONT_SYNC: c_uint = 0x4000;

#[no_mangle]
pub unsafe extern "C" fn __attribute__(_arg: (unused)) -> static {
    static __attribute__((unused))
    ssize_t statx(int dfd, const char *filename, unsigned flags,
    unsigned int mask, struct statx *buffer)
    {
    return syscall(__NR_statx, dfd, filename, flags, mask, buffer);
    }
#[no_mangle]
unsafe extern "C" fn print_time(field: *const c_char, ts: *mut statx_timestamp) {
    static void print_time(const char *field, struct statx_timestamp *ts)
    {
    struct tm tm;
    time_t tim;
    char buffer[100];
    int len;
    tim = ts.tv_sec;
    if (!localtime_r(&tim, &tm)) {
    perror("localtime_r");
    exit(1);
    }
    len = strftime(buffer, 100, "%F %T", &tm);
    if (len == 0) {
    perror("strftime");
    exit(1);
    }
    printf("%s", field);
    fwrite(buffer, 1, len, stdout);
    printf(".%09u", ts.tv_nsec);
    len = strftime(buffer, 100, "%z", &tm);
    if (len == 0) {
    perror("strftime2");
    exit(1);
    }
    fwrite(buffer, 1, len, stdout);
    printf("\n");
    }
#[no_mangle]
unsafe extern "C" fn dump_statx(stx: *mut statx) {
    static void dump_statx(struct statx *stx)
    {
    char buffer[256], ft = '?';
    printf("results=%x\n", stx.stx_mask);
    printf(" ");
    if (stx.stx_mask & STATX_SIZE)
    printf(" Size: %-15llu", (unsigned long long)stx.stx_size);
    if (stx.stx_mask & STATX_BLOCKS)
    printf(" Blocks: %-10llu", (unsigned long long)stx.stx_blocks);
    printf(" IO Block: %-6llu", (unsigned long long)stx.stx_blksize);
    if (stx.stx_mask & STATX_TYPE) {
    switch (stx.stx_mode & S_IFMT) {
    case S_IFIFO:	printf("  FIFO\n");			ft = 'p'; break;
    case S_IFCHR:	printf("  character special file\n");	ft = 'c'; break;
    case S_IFDIR:	printf("  directory\n");		ft = 'd'; break;
    case S_IFBLK:	printf("  block special file\n");	ft = 'b'; break;
    case S_IFREG:	printf("  regular file\n");		ft = '-'; break;
    case S_IFLNK:	printf("  symbolic link\n");		ft = 'l'; break;
    case S_IFSOCK:	printf("  socket\n");			ft = 's'; break;
    default:
    printf(" unknown type (%o)\n", stx.stx_mode & S_IFMT);
    break;
    }
    } else {
    printf(" no type\n");
    }
    sprintf(buffer, "%02x:%02x", stx.stx_dev_major, stx.stx_dev_minor);
    printf("Device: %-15s", buffer);
    if (stx.stx_mask & STATX_INO)
    printf(" Inode: %-11llu", (unsigned long long) stx.stx_ino);
    if (stx.stx_mask & STATX_NLINK)
    printf(" Links: %-5u", stx.stx_nlink);
    if (stx.stx_mask & STATX_TYPE) {
    switch (stx.stx_mode & S_IFMT) {
    case S_IFBLK:
    case S_IFCHR:
    printf(" Device type: %u,%u",
    stx.stx_rdev_major, stx.stx_rdev_minor);
    break;
    }
    }
    printf("\n");
    if (stx.stx_mask & STATX_MODE)
    printf("Access: (%04o/%c%c%c%c%c%c%c%c%c%c)  ",
    stx.stx_mode & 07777,
    ft,
    stx.stx_mode & S_IRUSR ? 'r' : '-',
    stx.stx_mode & S_IWUSR ? 'w' : '-',
    stx.stx_mode & S_IXUSR ? 'x' : '-',
    stx.stx_mode & S_IRGRP ? 'r' : '-',
    stx.stx_mode & S_IWGRP ? 'w' : '-',
    stx.stx_mode & S_IXGRP ? 'x' : '-',
    stx.stx_mode & S_IROTH ? 'r' : '-',
    stx.stx_mode & S_IWOTH ? 'w' : '-',
    stx.stx_mode & S_IXOTH ? 'x' : '-');
    if (stx.stx_mask & STATX_UID)
    printf("Uid: %5d   ", stx.stx_uid);
    if (stx.stx_mask & STATX_GID)
    printf("Gid: %5d\n", stx.stx_gid);
    if (stx.stx_mask & STATX_ATIME)
    print_time("Access: ", &stx.stx_atime);
    if (stx.stx_mask & STATX_MTIME)
    print_time("Modify: ", &stx.stx_mtime);
    if (stx.stx_mask & STATX_CTIME)
    print_time("Change: ", &stx.stx_ctime);
    if (stx.stx_mask & STATX_BTIME)
    print_time(" Birth: ", &stx.stx_btime);
    if (stx.stx_attributes_mask) {
    unsigned char bits, mbits;
    int loop, byte;
    static char attr_representation[64 + 1] =
// STATX_ATTR_ flags:
    "????????"	/* 63-56 */
    "????????"	/* 55-48 */
    "????????"	/* 47-40 */
    "????????"	/* 39-32 */
    "????????"	/* 31-24	0x00000000-ff000000 */
    "????????"	/* 23-16	0x00000000-00ff0000 */
    "???me???"	/* 15- 8	0x00000000-0000ff00 */
    "?dai?c??"	/*  7- 0	0x00000000-000000ff */
    ;
    printf("Attributes: %016llx (",
    (unsigned long long)stx.stx_attributes);
    for (byte = 64 - 8; byte >= 0; byte -= 8) {
    bits = stx.stx_attributes >> byte;
    mbits = stx.stx_attributes_mask >> byte;
    for (loop = 7; loop >= 0; loop--) {
    let mut bit: c_int = byte + loop;
    if (!(mbits & 0x80))
    putchar('.');	/* Not supported */
#[no_mangle]
pub unsafe extern "C" fn if(0x80: bits &) -> else {
    else if (bits & 0x80)
    putchar(attr_representation[63 - bit]);
    else
    putchar('-');	/* Not set */
    bits <<= 1;
    mbits <<= 1;
    }
    if (byte)
    putchar(' ');
    }
    printf(")\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn dump_hex(data: *mut c_ulonglong, from: c_int, to: c_int) {
    static void dump_hex(unsigned long long *data, int from, int to)
    {
    unsigned offset, print_offset = 1, col = 0;
    from /= 8;
    to = (to + 7) / 8;
    for (offset = from; offset < to; offset++) {
    if (print_offset) {
    printf("%04x: ", offset * 8);
    print_offset = 0;
    }
    printf("%016llx", data[offset]);
    col++;
    if ((col & 3) == 0) {
    printf("\n");
    print_offset = 1;
    } else {
    printf(" ");
    }
    }
    if (!print_offset)
    printf("\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct statx stx;
    int ret, raw = 0, atflag = AT_SYMLINK_NOFOLLOW;
    let mut mask: c_uint = STATX_BASIC_STATS | STATX_BTIME;
    for (argv++; *argv; argv++) {
    if (strcmp(*argv, "-F") == 0) {
    atflag &= ~AT_STATX_SYNC_TYPE;
    atflag |= AT_STATX_FORCE_SYNC;
    continue;
    }
    if (strcmp(*argv, "-D") == 0) {
    atflag &= ~AT_STATX_SYNC_TYPE;
    atflag |= AT_STATX_DONT_SYNC;
    continue;
    }
    if (strcmp(*argv, "-L") == 0) {
    atflag &= ~AT_SYMLINK_NOFOLLOW;
    continue;
    }
    if (strcmp(*argv, "-O") == 0) {
    mask &= ~STATX_BASIC_STATS;
    continue;
    }
    if (strcmp(*argv, "-A") == 0) {
    atflag |= AT_NO_AUTOMOUNT;
    continue;
    }
    if (strcmp(*argv, "-R") == 0) {
    raw = 1;
    continue;
    }
    memset(&stx, 0xbf, sizeof(stx));
    ret = statx(AT_FDCWD, *argv, atflag, mask, &stx);
    printf("statx(%s) = %d\n", *argv, ret);
    if (ret < 0) {
    perror(*argv);
    exit(1);
    }
    if (raw)
    dump_hex((unsigned long long *)&stx, 0, sizeof(stx));
    dump_statx(&stx);
    }
    return 0;
    }
