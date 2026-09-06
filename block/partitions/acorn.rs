//! Automatically rewritten from C to Rust
//! Source: block/partitions/acorn.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 1996-2000 Russell King.
//
// Scan ADFS partitions on hard disk drives.  Unfortunately, there
// isn't a standard for partitioning drives on Acorn machines, so
// every single manufacturer of SCSI and IDE cards created their own
// method.
//

//
// Partition types. (Oh for reusability)
//
pub const PARTITION_RISCIX_MFM: c_int = 1;
pub const PARTITION_RISCIX_SCSI: c_int = 2;
pub const PARTITION_LINUX: c_int = 9;

    defined(CONFIG_ACORN_PARTITION_ADFS)
    static struct adfs_discrecord *
    adfs_partition(struct parsed_partitions *state, char *name, char *data,
    unsigned long first_sector, int slot)
    {
    struct adfs_discrecord *dr;
    unsigned int nr_sects;
    if (adfs_checkbblk(data))
    return core::ptr::null_mut();
    dr = (struct adfs_discrecord *)(data + 0x1c0);
    if (dr.disc_size == 0 && dr.disc_size_high == 0)
    return core::ptr::null_mut();
    nr_sects = (le32_to_cpu(dr.disc_size_high) << 23) |
    (le32_to_cpu(dr.disc_size) >> 9);
    if (name) {
    seq_buf_printf(&state.pp_buf, " [%s]", name);
    }
    put_partition(state, slot, first_sector, nr_sects);
    return dr;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscix_part {
    pub start: __le32,
    pub length: __le32,
    pub one: __le32,
    pub name: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscix_record {
    pub magic: __le32,

    pub date: __le32,
    pub part: [riscix_part; 8],
}

    defined(CONFIG_ACORN_PARTITION_ADFS)
    static int riscix_partition(struct parsed_partitions *state,
    unsigned long first_sect, int slot,
    unsigned long nr_sects)
    {
    Sector sect;
    struct riscix_record *rr;
    rr = read_part_sector(state, first_sect, &sect);
    if (!rr)
    return -1;
    seq_buf_puts(&state.pp_buf, " [RISCiX]");
    if (rr.magic == RISCIX_MAGIC) {
    let mut size: c_ulong = min(nr_sects, 2);
    int part;
    seq_buf_puts(&state.pp_buf, " <");
    put_partition(state, slot++, first_sect, size);
    for (part = 0; part < 8; part++) {
    if (rr.part[part].one &&
    memcmp(rr.part[part].name, "All\0", 4)) {
    put_partition(state, slot++,
    le32_to_cpu(rr.part[part].start),
    le32_to_cpu(rr.part[part].length));
    seq_buf_printf(&state.pp_buf, "(%s)", rr.part[part].name);
    }
    }
    seq_buf_puts(&state.pp_buf, " >\n");
    } else {
    put_partition(state, slot++, first_sect, nr_sects);
    }
    put_dev_sector(sect);
    return slot;
    }

pub const LINUX_NATIVE_MAGIC: c_uint = 0xdeafa1de;
pub const LINUX_SWAP_MAGIC: c_uint = 0xdeafab1e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_part {
    pub magic: __le32,
    pub start_sect: __le32,
    pub nr_sects: __le32,
}

    defined(CONFIG_ACORN_PARTITION_ADFS)
    static int linux_partition(struct parsed_partitions *state,
    unsigned long first_sect, int slot,
    unsigned long nr_sects)
    {
    Sector sect;
    struct linux_part *linuxp;
    let mut size: c_ulong = min(nr_sects, 2);
    seq_buf_puts(&state.pp_buf, " [Linux]");
    put_partition(state, slot++, first_sect, size);
    linuxp = read_part_sector(state, first_sect, &sect);
    if (!linuxp)
    return -1;
    seq_buf_puts(&state.pp_buf, " <");
    while (linuxp.magic == cpu_to_le32(LINUX_NATIVE_MAGIC) ||
    linuxp.magic == cpu_to_le32(LINUX_SWAP_MAGIC)) {
    if (slot == state.limit)
    break;
    put_partition(state, slot++, first_sect +
    le32_to_cpu(linuxp.start_sect),
    le32_to_cpu(linuxp.nr_sects));
    linuxp ++;
    }
    seq_buf_puts(&state.pp_buf, " >");
    put_dev_sector(sect);
    return slot;
    }

#[no_mangle]
pub unsafe extern "C" fn adfspart_check_CUMANA(state: *mut parsed_partitions) -> c_int {
    int adfspart_check_CUMANA(struct parsed_partitions *state)
    {
    let mut first_sector: c_ulong = 0;
    let mut start_blk: c_uint = 0;
    Sector sect;
    unsigned char *data;
    char *name = "CUMANA/ADFS";
    let mut first: c_int = 1;
    let mut slot: c_int = 1;
//
// Try Cumana style partitions - sector 6 contains ADFS boot block
// with pointer to next 'drive'.
//
// There are unknowns in this code - is the 'cylinder number' of the
// next partition relative to the start of this one - I'm assuming
// it is.
//
// Also, which ID did Cumana use?
//
// This is totally unfinished, and will require more work to get it
// going. Hence it is totally untested.
//
    do {
    struct adfs_discrecord *dr;
    unsigned int nr_sects;
    data = read_part_sector(state, start_blk * 2 + 6, &sect);
    if (!data)
    return -1;
    if (slot == state.limit)
    break;
    dr = adfs_partition(state, name, data, first_sector, slot++);
    if (!dr)
    break;
    name = core::ptr::null_mut();
    nr_sects = (data[0x1fd] + (data[0x1fe] << 8)) *
    (dr.heads + (dr.lowsector & 0x40 ? 1 : 0)) *
    dr.secspertrack;
    if (!nr_sects)
    break;
    first = 0;
    first_sector += nr_sects;
    start_blk += nr_sects >> (BLOCK_SIZE_BITS - 9);
    nr_sects = 0; /* hmm - should be partition size */
    switch (data[0x1fc] & 15) {
    case 0: /* No partition / ADFS? */
    break;

    case PARTITION_RISCIX_SCSI:
// RISCiX - we don't know how to find the next one.
    slot = riscix_partition(state, first_sector, slot,
    nr_sects);
    break;

    case PARTITION_LINUX:
    slot = linux_partition(state, first_sector, slot,
    nr_sects);
    break;
    }
    put_dev_sector(sect);
    if (slot == -1)
    return -1;
    } while (1);
    put_dev_sector(sect);
    return first ? 0 : 1;
    }

//
// Purpose: allocate ADFS partitions.
//
// Params : hd		- pointer to gendisk structure to store partition info.
// dev		- device number to access.
//
// Returns: -1 on error, 0 for no ADFS boot sector, 1 for ok.
//
// Alloc  : hda  = whole drive
// hda1 = ADFS partition on first drive.
// hda2 = non-ADFS partition.
//
#[no_mangle]
pub unsafe extern "C" fn adfspart_check_ADFS(state: *mut parsed_partitions) -> c_int {
    int adfspart_check_ADFS(struct parsed_partitions *state)
    {
    unsigned long start_sect, nr_sects, sectscyl, heads;
    Sector sect;
    unsigned char *data;
    struct adfs_discrecord *dr;
    unsigned char id;
    let mut slot: c_int = 1;
    data = read_part_sector(state, 6, &sect);
    if (!data)
    return -1;
    dr = adfs_partition(state, "ADFS", data, 0, slot++);
    if (!dr) {
    put_dev_sector(sect);
    return 0;
    }
    heads = dr.heads + ((dr.lowsector >> 6) & 1);
    sectscyl = dr.secspertrack * heads;
    start_sect = ((data[0x1fe] << 8) + data[0x1fd]) * sectscyl;
    id = data[0x1fc] & 15;
    put_dev_sector(sect);
//
// Work out start of non-adfs partition.
//
    nr_sects = get_capacity(state.disk) - start_sect;
    if (start_sect) {
    switch (id) {

    case PARTITION_RISCIX_SCSI:
    case PARTITION_RISCIX_MFM:
    riscix_partition(state, start_sect, slot,
    nr_sects);
    break;

    case PARTITION_LINUX:
    linux_partition(state, start_sect, slot,
    nr_sects);
    break;
    }
    }
    seq_buf_puts(&state.pp_buf, "\n");
    return 1;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ics_part {
    pub start: __le32,
    pub size: __le32,
}

    static int adfspart_check_ICSLinux(struct parsed_partitions *state,
    unsigned long block)
    {
    Sector sect;
    unsigned char *data = read_part_sector(state, block, &sect);
    let mut result: c_int = 0;
    if (data) {
    if (memcmp(data, "LinuxPart", 9) == 0)
    result = 1;
    put_dev_sector(sect);
    }
    return result;
    }
//
// Check for a valid ICS partition using the checksum.
//
#[no_mangle]
pub unsafe extern "C" fn valid_ics_sector(data: *const c_uchar) -> c_int {
    static inline int valid_ics_sector(const unsigned char *data)
    {
    unsigned long sum;
    int i;
    for (i = 0, sum = 0x50617274; i < 508; i++)
    sum += data[i];
    sum -= le32_to_cpu(*(__le32 *)(&data[508]));
    let mut sum: return = = 0;
    }
//
// Purpose: allocate ICS partitions.
// Params : hd		- pointer to gendisk structure to store partition info.
// dev		- device number to access.
// Returns: -1 on error, 0 for no ICS table, 1 for partitions ok.
// Alloc  : hda  = whole drive
// hda1 = ADFS partition 0 on first drive.
// hda2 = ADFS partition 1 on first drive.
// ..etc..
//
#[no_mangle]
pub unsafe extern "C" fn adfspart_check_ICS(state: *mut parsed_partitions) -> c_int {
    int adfspart_check_ICS(struct parsed_partitions *state)
    {
    const unsigned char *data;
    const struct ics_part *p;
    int slot;
    Sector sect;
//
// Try ICS style partitions - sector 0 contains partition info.
//
    data = read_part_sector(state, 0, &sect);
    if (!data)
    return -1;
    if (!valid_ics_sector(data)) {
    put_dev_sector(sect);
    return 0;
    }
    seq_buf_puts(&state.pp_buf, " [ICS]");
    for (slot = 1, p = (const struct ics_part *)data; p.size; p++) {
    let mut start: u32 = le32_to_cpu(p.start);
    s32 size = le32_to_cpu(p.size); /* yes, it's signed. */
    if (slot == state.limit)
    break;
//
// Negative sizes tell the RISC OS ICS driver to ignore
// this partition - in effect it says that this does not
// contain an ADFS filesystem.
//
    if (size < 0) {
    size = -size;
//
// Our own extension - We use the first sector
// of the partition to identify what type this
// partition is.  We must not make this visible
// to the filesystem.
//
    if (size > 1 && adfspart_check_ICSLinux(state, start)) {
    start += 1;
    size -= 1;
    }
    }
    if (size)
    put_partition(state, slot++, start, size);
    }
    put_dev_sector(sect);
    seq_buf_puts(&state.pp_buf, "\n");
    return 1;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptec_part {
    pub unused1: __le32,
    pub unused2: __le32,
    pub start: __le32,
    pub size: __le32,
    pub unused5: __le32,
    pub type: [c_char; 8],
}

#[no_mangle]
pub unsafe extern "C" fn valid_ptec_sector(data: *const c_uchar) -> c_int {
    static inline int valid_ptec_sector(const unsigned char *data)
    {
    let mut checksum: c_uchar = 0x2a;
    int i;
//
// If it looks like a PC/BIOS partition, then it
// probably isn't PowerTec.
//
    if (data[510] == 0x55 && data[511] == 0xaa)
    return 0;
    for (i = 0; i < 511; i++)
    checksum += data[i];
    let mut checksum: return = = data[511];
    }
//
// Purpose: allocate ICS partitions.
// Params : hd		- pointer to gendisk structure to store partition info.
// dev		- device number to access.
// Returns: -1 on error, 0 for no ICS table, 1 for partitions ok.
// Alloc  : hda  = whole drive
// hda1 = ADFS partition 0 on first drive.
// hda2 = ADFS partition 1 on first drive.
// ..etc..
//
#[no_mangle]
pub unsafe extern "C" fn adfspart_check_POWERTEC(state: *mut parsed_partitions) -> c_int {
    int adfspart_check_POWERTEC(struct parsed_partitions *state)
    {
    Sector sect;
    const unsigned char *data;
    const struct ptec_part *p;
    let mut slot: c_int = 1;
    int i;
    data = read_part_sector(state, 0, &sect);
    if (!data)
    return -1;
    if (!valid_ptec_sector(data)) {
    put_dev_sector(sect);
    return 0;
    }
    seq_buf_puts(&state.pp_buf, " [POWERTEC]");
    for (i = 0, p = (const struct ptec_part *)data; i < 12; i++, p++) {
    let mut start: u32 = le32_to_cpu(p.start);
    let mut size: u32 = le32_to_cpu(p.size);
    if (size)
    put_partition(state, slot++, start, size);
    }
    put_dev_sector(sect);
    seq_buf_puts(&state.pp_buf, "\n");
    return 1;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eesox_part {
    pub magic: [c_char; 6],
    pub name: [c_char; 10],
    pub start: __le32,
    pub unused6: __le32,
    pub unused7: __le32,
    pub unused8: __le32,
}

//
// Guess who created this format?
//
    static const char eesox_name[] = {
    'N', 'e', 'i', 'l', ' ',
    'C', 'r', 'i', 't', 'c', 'h', 'e', 'l', 'l', ' ', ' '
    };
//
// EESOX SCSI partition format.
//
// This is a goddamned awful partition format.  We don't seem to store
// the size of the partition in this table, only the start addresses.
//
// There are two possibilities where the size comes from:
// 1. The individual ADFS boot block entries that are placed on the disk.
// 2. The start address of the next entry.
//
#[no_mangle]
pub unsafe extern "C" fn adfspart_check_EESOX(state: *mut parsed_partitions) -> c_int {
    int adfspart_check_EESOX(struct parsed_partitions *state)
    {
    Sector sect;
    const unsigned char *data;
    unsigned char buffer[256];
    struct eesox_part *p;
    let mut start: sector_t = 0;
    int i, slot = 1;
    data = read_part_sector(state, 7, &sect);
    if (!data)
    return -1;
//
// "Decrypt" the partition table.  God knows why...
//
    for (i = 0; i < 256; i++)
    buffer[i] = data[i] ^ eesox_name[i & 15];
    put_dev_sector(sect);
    for (i = 0, p = (struct eesox_part *)buffer; i < 8; i++, p++) {
    sector_t next;
    if (memcmp(p.magic, "Eesox", 6))
    break;
    next = le32_to_cpu(p.start);
    if (i)
    put_partition(state, slot++, start, next - start);
    start = next;
    }
    if (i != 0) {
    sector_t size;
    size = get_capacity(state.disk);
    put_partition(state, slot++, start, size - start);
    seq_buf_puts(&state.pp_buf, "\n");
    }
    return i ? 1 : 0;
    }
