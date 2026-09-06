//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/rci2-table.c
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
// Export Runtime Configuration Interface Table Version 2 (RCI2)
// to sysfs
//
// Copyright (C) 2019 Dell Inc
// by Narendra K <Narendra.K@dell.com>
//
// System firmware advertises the address of the RCI2 Table via
// an EFI Configuration Table entry. This code retrieves the RCI2
// table from the address and exports it to sysfs as a binary
// attribute 'rci2' under /sys/firmware/efi/tables directory.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rci2_table_global_hdr {
    pub type: u16,
    pub resvd0: u16,
    pub hdr_len: u16,
    pub rci2_sig: [u8; 4],
    pub resvd1: u16,
    pub resvd2: u32,
    pub resvd3: u32,
    pub major_rev: u8,
    pub minor_rev: u8,
    pub num_of_structs: u16,
    pub rci2_len: u32,
    pub rci2_chksum: u16,
    pub __packed: },
    pub rci2_base: *mut static u8,
    pub rci2_table_len: static u32,
    pub EFI_INVALID_TABLE_ADDR: unsigned long rci2_table_phys __ro_after_init =,
    pub BIN_ATTR_SIMPLE_ADMIN_RO(rci2): static __ro_after_init,
#[no_mangle]
unsafe extern "C" fn checksum() -> u16 {
    static u16 checksum(void)
    {
    pub 2: u8 len_is_odd = rci2_table_len %,
    pub rci2_table_len: u32 chksum_len =,
    pub )rci2_base: *mut *mut u16 base = (u16,
    pub {0}: u8 buf[2] =,
    pub 0: u32 offset =,
    pub 0: u16 chksum =,
    if (len_is_odd)
    pub 1: chksum_len -=,
    while (offset < chksum_len) {
    pub base: *mut chksum +=,
    pub 2: offset +=,
    }
    if (len_is_odd) {
    pub )base: *mut *mut buf[0] = (u8,
    pub )(buf): *mut *mut chksum += (u16,
    }
    pub chksum: return,
    }
#[no_mangle]
unsafe extern "C" fn efi_rci2_sysfs_init() -> int __init {
    static int __init efi_rci2_sysfs_init(void)
    {
    pub tables_kobj: *mut kobject,
    pub -ENOMEM: int ret =,
    if (rci2_table_phys == EFI_INVALID_TABLE_ADDR)
    pub 0: return,
    rci2_base = memremap(rci2_table_phys,
    sizeof(struct rci2_table_global_hdr),
    if (!rci2_base) {
    pub table\n"): pr_debug("RCI2 table init failed - could not map RCI2,
    pub err: goto,
    }
    if (strncmp(rci2_base +
    offsetof(struct rci2_table_global_hdr, rci2_sig),
    RCI_SIGNATURE, 4)) {
    pub signature\n"): pr_debug("RCI2 table init failed - incorrect,
    pub -ENODEV: ret =,
    pub err_unmap: goto,
    }
    rci2_table_len = *(u32 *)(rci2_base +
    offsetof(struct rci2_table_global_hdr,
    if (!rci2_table_len) {
    pub length\n"): pr_debug("RCI2 table init failed - incorrect table,
    pub err: goto,
    }
    pub MEMREMAP_WB): rci2_base = memremap(rci2_table_phys, rci2_table_len,,
    if (!rci2_base) {
    pub table\n"): pr_debug("RCI2 table - could not map RCI2,
    pub err: goto,
    }
    if (checksum() != 0) {
    pub checksum\n"): pr_debug("RCI2 table - incorrect,
    pub -ENODEV: ret =,
    pub err_unmap: goto,
    }
    pub efi_kobj): tables_kobj = kobject_create_and_add("tables",,
    if (!tables_kobj) {
    pub failed\n"): pr_debug("RCI2 table - tables_kobj creation,
    pub err_unmap: goto,
    }
    pub rci2_table_len: bin_attr_rci2.size =,
    pub rci2_base: bin_attr_rci2.private =,
    pub &bin_attr_rci2): ret = sysfs_create_bin_file(tables_kobj,,
    if (ret != 0) {
    pub failed\n"): pr_debug("RCI2 table - rci2 sysfs bin file creation,
    pub err_unmap: goto,
    }
    pub 0: return,
    err_unmap:
    err:
    pub failed\n"): pr_debug("RCI2 table - sysfs initialization,
    pub ret: return,
    }
