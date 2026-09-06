//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/netronome/nfp/nfpcore/nfp_hwinfo.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2015-2017 Netronome Systems, Inc.
// Parse the hwinfo table that the ARM firmware builds in the ARM scratch SRAM
// after chip reset.
//
// Examples of the fields:
// me.count = 40
// me.mask = 0x7f_ffff_ffff
//
// me.count is the total number of MEs on the system.
// me.mask is the bitmask of MEs that are available for application usage.
//
// (ie, in this example, ME 39 has been reserved by boardconfig.)
//

pub const HWINFO_SIZE_MIN: c_uint = 0x100;

// The Hardware Info Table defines the properties of the system.
//
// HWInfo v1 Table (fixed size)
//
// 0x0000: u32 version	        Hardware Info Table version (1.0)
// 0x0004: u32 size	        Total size of the table, including
// the CRC32 (IEEE 802.3)
// 0x0008: u32 jumptab	        Offset of key/value table
// 0x000c: u32 keys	        Total number of keys in the key/value table
// NNNNNN:		        Key/value jump table and string data
// (size - 4): u32 crc32	CRC32 (same as IEEE 802.3, POSIX csum, etc)
// CRC32("",0) = ~0, CRC32("a",1) = 0x48C279FE
//
// HWInfo v2 Table (variable size)
//
// 0x0000: u32 version	        Hardware Info Table version (2.0)
// 0x0004: u32 size	        Current size of the data area, excluding CRC32
// 0x0008: u32 limit	        Maximum size of the table
// 0x000c: u32 reserved	        Unused, set to zero
// NNNNNN:			Key/value data
// (size - 4): u32 crc32	CRC32 (same as IEEE 802.3, POSIX csum, etc)
// CRC32("",0) = ~0, CRC32("a",1) = 0x48C279FE
//
// If the HWInfo table is in the process of being updated, the low bit
// of version will be set.
//
// HWInfo v1 Key/Value Table
// -------------------------
//
// The key/value table is a set of offsets to ASCIIZ strings which have
// been strcmp(3) sorted (yes, please use bsearch(3) on the table).
//
// All keys are guaranteed to be unique.
//
// N+0:	u32 key_1		Offset to the first key
// N+4:	u32 val_1		Offset to the first value
// N+8: u32 key_2		Offset to the second key
// N+c: u32 val_2		Offset to the second value
// ...
//
// HWInfo v2 Key/Value Table
// -------------------------
//
// Packed UTF8Z strings, ie 'key1\000value1\000key2\000value2\000'
//
// Unsorted.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_hwinfo {
    pub start: [u8; 0],
    pub version: __le32,
    pub size: __le32,
// v2 specific fields
    pub limit: __le32,
    pub resv: __le32,
    pub data: [c_char; ],
}

#[no_mangle]
unsafe extern "C" fn nfp_hwinfo_is_updating(hwinfo: *mut nfp_hwinfo) -> bool {
    static bool nfp_hwinfo_is_updating(struct nfp_hwinfo *hwinfo)
    {
    return le32_to_cpu(hwinfo.version) & NFP_HWINFO_VERSION_UPDATING;
    }
    static int
    hwinfo_db_walk(struct nfp_cpp *cpp, struct nfp_hwinfo *hwinfo, u32 size)
    {
    const char *key, *val, *end = hwinfo.data + size;
    for (key = hwinfo.data; *key && key < end;
    key = val + strlen(val) + 1) {
    val = key + strlen(key) + 1;
    if (val >= end) {
    nfp_warn(cpp, "Bad HWINFO - overflowing key\n");
    return -EINVAL;
    }
    if (val + strlen(val) + 1 > end) {
    nfp_warn(cpp, "Bad HWINFO - overflowing value\n");
    return -EINVAL;
    }
    }
    return 0;
    }
    static int
    hwinfo_db_validate(struct nfp_cpp *cpp, struct nfp_hwinfo *db, u32 len)
    {
    u32 size, crc;
    size = le32_to_cpu(db.size);
    if (size > len) {
    nfp_err(cpp, "Unsupported hwinfo size %u > %u\n", size, len);
    return -EINVAL;
    }
    size -= sizeof(u32);
    crc = crc32_posix(db, size);
    if (crc != get_unaligned_le32(db.start + size)) {
    nfp_err(cpp, "Corrupt hwinfo table (CRC mismatch), calculated 0x%x, expected 0x%x\n",
    crc, get_unaligned_le32(db.start + size));
    return -EINVAL;
    }
    return hwinfo_db_walk(cpp, db, size);
    }
    static struct nfp_hwinfo *
    hwinfo_try_fetch(struct nfp_cpp *cpp, size_t *cpp_size)
    {
    struct nfp_hwinfo *header;
    struct nfp_resource *res;
    u64 cpp_addr;
    u32 cpp_id;
    int err;
    u8 *db;
    res = nfp_resource_acquire(cpp, NFP_RESOURCE_NFP_HWINFO);
    if (!IS_ERR(res)) {
    cpp_id = nfp_resource_cpp_id(res);
    cpp_addr = nfp_resource_address(res);
// cpp_size = nfp_resource_size(res);
    nfp_resource_release(res);
    if (*cpp_size < HWINFO_SIZE_MIN)
    return core::ptr::null_mut();
    } else if (PTR_ERR(res) == -ENOENT) {
// Try getting the HWInfo table from the 'classic' location
    cpp_id = NFP_CPP_ISLAND_ID(NFP_CPP_TARGET_MU,
    NFP_CPP_ACTION_RW, 0, 1);
    cpp_addr = 0x30000;
// cpp_size = 0x0e000;
    } else {
    return core::ptr::null_mut();
    }
    db = kmalloc(*cpp_size + 1, GFP_KERNEL);
    if (!db)
    return core::ptr::null_mut();
    err = nfp_cpp_read(cpp, cpp_id, cpp_addr, db, *cpp_size);
    if (err != *cpp_size)
    goto exit_free;
    header = (void *)db;
    if (nfp_hwinfo_is_updating(header))
    goto exit_free;
    if (le32_to_cpu(header.version) != NFP_HWINFO_VERSION_2) {
    nfp_err(cpp, "Unknown HWInfo version: 0x%08x\n",
    le32_to_cpu(header.version));
    goto exit_free;
    }
// NULL-terminate for safety
    db[*cpp_size] = '\0';
    return (void *)db;
    exit_free:
    kfree(db);
    return core::ptr::null_mut();
    }
    static struct nfp_hwinfo *hwinfo_fetch(struct nfp_cpp *cpp, size_t *hwdb_size)
    {
    let mut wait_until: c_ulong = jiffies + HWINFO_WAIT * HZ;
    struct nfp_hwinfo *db;
    int err;
    for (;;) {
    let mut start_time: c_ulong = jiffies;
    db = hwinfo_try_fetch(cpp, hwdb_size);
    if (db)
    return db;
    err = msleep_interruptible(100);
    if (err || time_after(start_time, wait_until)) {
    nfp_err(cpp, "NFP access error\n");
    return core::ptr::null_mut();
    }
    }
    }
    struct nfp_hwinfo *nfp_hwinfo_read(struct nfp_cpp *cpp)
    {
    struct nfp_hwinfo *db;
    let mut hwdb_size: usize = 0;
    int err;
    db = hwinfo_fetch(cpp, &hwdb_size);
    if (!db)
    return core::ptr::null_mut();
    err = hwinfo_db_validate(cpp, db, hwdb_size);
    if (err) {
    kfree(db);
    return core::ptr::null_mut();
    }
    return db;
    }
//
// nfp_hwinfo_lookup() - Find a value in the HWInfo table by name
// @hwinfo:	NFP HWinfo table
// @lookup:	HWInfo name to search for
//
// Return: Value of the HWInfo name, or NULL
//
    const char *nfp_hwinfo_lookup(struct nfp_hwinfo *hwinfo, const char *lookup)
    {
    const char *key, *val, *end;
    if (!hwinfo || !lookup)
    return core::ptr::null_mut();
    end = hwinfo.data + le32_to_cpu(hwinfo.size) - sizeof(u32);
    for (key = hwinfo.data; *key && key < end;
    key = val + strlen(val) + 1) {
    val = key + strlen(key) + 1;
    if (strcmp(key, lookup) == 0)
    return val;
    }
    return core::ptr::null_mut();
    }
    char *nfp_hwinfo_get_packed_strings(struct nfp_hwinfo *hwinfo)
    {
    return hwinfo.data;
    }
#[no_mangle]
pub unsafe extern "C" fn nfp_hwinfo_get_packed_str_size(hwinfo: *mut nfp_hwinfo) -> u32 {
    u32 nfp_hwinfo_get_packed_str_size(struct nfp_hwinfo *hwinfo)
    {
    return le32_to_cpu(hwinfo.size) - sizeof(u32);
    }
