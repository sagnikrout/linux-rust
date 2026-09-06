//! Automatically rewritten from C to Rust
//! Source: block/partitions/cmdline.c
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
// Copyright (C) 2013 HUAWEI
// Author: Cai Zhiyong <caizhiyong@huawei.com>
//
// Read block device partition table from the command line.
// Typically used for fixed block (eMMC) embedded devices.
// It has no MBR, so saves storage space. Bootloader can be easily accessed
// by absolute address of data on the block device.
// Users can easily change the partition.
//
// The format for the command line is just like mtdparts.
//
// For further information, see "Documentation/block/cmdline-partition.rst"
//

// partition flags
pub const PF_RDONLY: c_uint = 0x01 /* Device is read only */;
pub const PF_POWERUP_LOCK: c_uint = 0x02 /* Always locked after reset */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdline_subpart {
    pub /: *mut *mut char name[BDEVNAME_SIZE]; / partition name, such as 'rootfs',
    pub from: sector_t,
    pub size: sector_t,
    pub flags: c_int,
    pub next_subpart: *mut cmdline_subpart,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdline_parts {
    pub /: *mut *mut char name[BDEVNAME_SIZE]; / block device, such as 'mmcblk0',
    pub nr_subparts: c_uint,
    pub subpart: *mut cmdline_subpart,
    pub next_parts: *mut cmdline_parts,
}

#[no_mangle]
unsafe extern "C" fn parse_subpart(subpart: *mut cmdline_subpart, partdef: *mut c_char) -> c_int {
    static int parse_subpart(struct cmdline_subpart **subpart, char *partdef)
    {
    let mut ret: c_int = 0;
    struct cmdline_subpart *new_subpart;
// subpart = NULL;
    new_subpart = kzalloc_obj(struct cmdline_subpart);
    if (!new_subpart)
    return -ENOMEM;
    if (*partdef == '-') {
    new_subpart.size = (sector_t)(~0ULL);
    partdef++;
    } else {
    new_subpart.size = (sector_t)memparse(partdef, &partdef);
    if (new_subpart.size < (sector_t)PAGE_SIZE) {
    pr_warn("cmdline partition size is invalid.");
    ret = -EINVAL;
    goto fail;
    }
    }
    if (*partdef == '@') {
    partdef++;
    new_subpart.from = (sector_t)memparse(partdef, &partdef);
    } else {
    new_subpart.from = (sector_t)(~0ULL);
    }
    if (*partdef == '(') {
    partdef++;
    char *next = strsep(&partdef, ")");
    if (!next) {
    pr_warn("cmdline partition format is invalid.");
    ret = -EINVAL;
    goto fail;
    }
    strscpy(new_subpart.name, next, sizeof(new_subpart.name));
    } else
    new_subpart.name[0] = '\0';
    new_subpart.flags = 0;
    if (!strncmp(partdef, "ro", 2)) {
    new_subpart.flags |= PF_RDONLY;
    partdef += 2;
    }
    if (!strncmp(partdef, "lk", 2)) {
    new_subpart.flags |= PF_POWERUP_LOCK;
    partdef += 2;
    }
// subpart = new_subpart;
    return 0;
    fail:
    kfree(new_subpart);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn free_subpart(parts: *mut cmdline_parts) {
    static void free_subpart(struct cmdline_parts *parts)
    {
    struct cmdline_subpart *subpart;
    while (parts.subpart) {
    subpart = parts.subpart;
    parts.subpart = subpart.next_subpart;
    kfree(subpart);
    }
    }
#[no_mangle]
unsafe extern "C" fn parse_parts(parts: *mut cmdline_parts, bdevdef: *mut c_char) -> c_int {
    static int parse_parts(struct cmdline_parts **parts, char *bdevdef)
    {
    let mut ret: c_int = -EINVAL;
    char *next;
    struct cmdline_subpart **next_subpart;
    struct cmdline_parts *newparts;
// parts = NULL;
    newparts = kzalloc_obj(struct cmdline_parts);
    if (!newparts)
    return -ENOMEM;
    next = strsep(&bdevdef, ":");
    if (!next) {
    pr_warn("cmdline partition has no block device.");
    goto fail;
    }
    strscpy(newparts.name, next, sizeof(newparts.name));
    newparts.nr_subparts = 0;
    next_subpart = &newparts.subpart;
    while ((next = strsep(&bdevdef, ","))) {
    ret = parse_subpart(next_subpart, next);
    if (ret)
    goto fail;
    newparts.nr_subparts++;
    next_subpart = &(*next_subpart).next_subpart;
    }
    if (!newparts.subpart) {
    pr_warn("cmdline partition has no valid partition.");
    ret = -EINVAL;
    goto fail;
    }
// parts = newparts;
    return 0;
    fail:
    free_subpart(newparts);
    kfree(newparts);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cmdline_parts_free(parts: *mut cmdline_parts) {
    static void cmdline_parts_free(struct cmdline_parts **parts)
    {
    struct cmdline_parts *next_parts;
    while (*parts) {
    next_parts = (*parts).next_parts;
    free_subpart(*parts);
    kfree(*parts);
// parts = next_parts;
    }
    }
    static int cmdline_parts_parse(struct cmdline_parts **parts,
    const char *cmdline)
    {
    int ret;
    char *buf;
    char *pbuf;
    char *next;
    struct cmdline_parts **next_parts;
// parts = NULL;
    pbuf = buf = kstrdup(cmdline, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    next_parts = parts;
    while ((next = strsep(&pbuf, ";"))) {
    ret = parse_parts(next_parts, next);
    if (ret)
    goto fail;
    next_parts = &(*next_parts).next_parts;
    }
    if (!*parts) {
    pr_warn("cmdline partition has no valid partition.");
    ret = -EINVAL;
    goto fail;
    }
    ret = 0;
    done:
    kfree(buf);
    return ret;
    fail:
    cmdline_parts_free(parts);
    goto done;
    }
    static struct cmdline_parts *cmdline_parts_find(struct cmdline_parts *parts,
    const char *bdev)
    {
    while (parts && strncmp(bdev, parts.name, sizeof(parts.name)))
    parts = parts.next_parts;
    return parts;
    }
    static char *cmdline;
    static struct cmdline_parts *bdev_parts;
    static int add_part(int slot, struct cmdline_subpart *subpart,
    struct parsed_partitions *state)
    {
    struct partition_meta_info *info;
    if (slot >= state.limit)
    return 1;
    put_partition(state, slot, subpart.from >> 9,
    subpart.size >> 9);
    if (subpart.flags & PF_RDONLY)
    state.parts[slot].flags |= ADDPART_FLAG_READONLY;
    info = &state.parts[slot].info;
    strscpy(info.volname, subpart.name, sizeof(info.volname));
    seq_buf_printf(&state.pp_buf, "(%s)", info.volname);
    state.parts[slot].has_info = true;
    return 0;
    }
    static int cmdline_parts_set(struct cmdline_parts *parts, sector_t disk_size,
    struct parsed_partitions *state)
    {
    let mut from: sector_t = 0;
    struct cmdline_subpart *subpart;
    let mut slot: c_int = 1;
    for (subpart = parts.subpart; subpart;
    subpart = subpart.next_subpart, slot++) {
    if (subpart.from == (sector_t)(~0ULL))
    subpart.from = from;
    else
    from = subpart.from;
    if (from >= disk_size)
    break;
    if (subpart.size > (disk_size - from))
    subpart.size = disk_size - from;
    from += subpart.size;
    if (add_part(slot, subpart, state))
    break;
    }
    return slot;
    }
#[no_mangle]
unsafe extern "C" fn cmdline_parts_setup(s: *mut c_char) -> int __init {
    static int __init cmdline_parts_setup(char *s)
    {
    cmdline = s;
    return 1;
    }
    __setup("blkdevparts=", cmdline_parts_setup);
    static bool has_overlaps(sector_t from, sector_t size,
    sector_t from2, sector_t size2)
    {
    let mut end: sector_t = from + size;
    let mut end2: sector_t = from2 + size2;
    if (from >= from2 && from < end2)
    return true;
    if (end > from2 && end <= end2)
    return true;
    if (from2 >= from && from2 < end)
    return true;
    if (end2 > from && end2 <= end)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn overlaps_warns_header() {
    static inline void overlaps_warns_header(void)
    {
    pr_warn("Overlapping partitions are used in command line partitions.");
    pr_warn("Don't use filesystems on overlapping partitions:");
    }
#[no_mangle]
unsafe extern "C" fn cmdline_parts_verifier(slot: c_int, state: *mut parsed_partitions) {
    static void cmdline_parts_verifier(int slot, struct parsed_partitions *state)
    {
    int i;
    let mut header: bool = true;
    for (; slot < state.limit && state.parts[slot].has_info; slot++) {
    for (i = slot+1; i < state.limit && state.parts[i].has_info;
    i++) {
    if (has_overlaps(state.parts[slot].from,
    state.parts[slot].size,
    state.parts[i].from,
    state.parts[i].size)) {
    if (header) {
    header = false;
    overlaps_warns_header();
    }
    pr_warn("%s[%llu,%llu] overlaps with "
    "%s[%llu,%llu].",
    state.parts[slot].info.volname,
    (u64)state.parts[slot].from << 9,
    (u64)state.parts[slot].size << 9,
    state.parts[i].info.volname,
    (u64)state.parts[i].from << 9,
    (u64)state.parts[i].size << 9);
    }
    }
    }
    }
//
// Purpose: allocate cmdline partitions.
// Returns:
// -1 if unable to read the partition table
// 0 if this isn't our partition table
// 1 if successful
//
#[no_mangle]
pub unsafe extern "C" fn cmdline_partition(state: *mut parsed_partitions) -> c_int {
    int cmdline_partition(struct parsed_partitions *state)
    {
    sector_t disk_size;
    struct cmdline_parts *parts;
    if (cmdline) {
    if (bdev_parts)
    cmdline_parts_free(&bdev_parts);
    if (cmdline_parts_parse(&bdev_parts, cmdline)) {
    cmdline = core::ptr::null_mut();
    return -1;
    }
    cmdline = core::ptr::null_mut();
    }
    if (!bdev_parts)
    return 0;
    parts = cmdline_parts_find(bdev_parts, state.disk.disk_name);
    if (!parts)
    return 0;
    disk_size = get_capacity(state.disk) << 9;
    cmdline_parts_set(parts, disk_size, state);
    cmdline_parts_verifier(1, state);
    seq_buf_puts(&state.pp_buf, "\n");
    return 1;
    }
