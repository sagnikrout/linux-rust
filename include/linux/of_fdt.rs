//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/of_fdt.h
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
// Definitions for working with the Flattened Device Tree data format
//
// Copyright 2009 Benjamin Herrenschmidt, IBM Corp
// benh@kernel.crashing.org
//

// Definitions used by the flattened device tree
pub const OF_DT_HEADER: c_uint = 0xd00dfeed	/* marker */;

// For scanning an arbitrary device-tree at any time
// TBD: Temporary export of fdt globals - remove when code fully merged
// Other Prototypes
extern "C" {
    pub fn of_flat_dt_translate_address(node: c_ulong) -> u64;
}
extern "C" {
    pub fn of_fdt_limit_memory(limit: c_int);
}

// For scanning the flat device-tree at boot time
extern "C" {
    pub fn of_flat_dt_is_compatible(node: c_ulong, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn of_get_flat_dt_root() -> c_ulong;
}
extern "C" {
    pub fn of_get_flat_dt_phandle(node: c_ulong) -> u32;
}
extern "C" {
    pub fn early_init_dt_scan_chosen(cmdline: *mut c_char) -> c_int;
}
extern "C" {
    pub fn early_init_dt_scan_memory() -> c_int;
}
extern "C" {
    pub fn early_init_dt_check_for_usable_mem_range();
}
extern "C" {
    pub fn early_init_dt_scan_chosen_stdout() -> c_int;
}
extern "C" {
    pub fn early_init_fdt_scan_reserved_mem();
}
extern "C" {
    pub fn early_init_fdt_reserve_self();
}
extern "C" {
    pub fn early_init_dt_add_memory_arch(base: u64, size: u64);
}
extern "C" {
    pub fn dt_mem_next_cell(s: c_int, cellp: *const __be32) -> u64;
}
// Early flat tree scan hooks
extern "C" {
    pub fn early_init_dt_scan_root() -> c_int;
}
extern "C" {
    pub fn early_init_dt_scan(dt_virt: *mut c_void, dt_phys: phys_addr_t) -> bool;
}
extern "C" {
    pub fn early_init_dt_verify(dt_virt: *mut c_void, dt_phys: phys_addr_t) -> bool;
}
extern "C" {
    pub fn early_init_dt_scan_nodes();
}
// Other Prototypes
extern "C" {
    pub fn unflatten_device_tree();
}
extern "C" {
    pub fn unflatten_and_copy_device_tree();
}
extern "C" {
    pub fn early_init_devtree(: *mut c_void);
}
extern "C" {
    pub fn early_get_first_memblock_info(: *mut c_void, : *mut phys_addr_t);
}

