//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/e820/api.h
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

extern "C" {
    pub fn e820__mapped_raw_any(start: u64, end: u64, type: e820_type) -> bool;
}
extern "C" {
    pub fn e820__mapped_any(start: u64, end: u64, type: e820_type) -> bool;
}
extern "C" {
    pub fn e820__mapped_all(start: u64, end: u64, type: e820_type) -> bool;
}
extern "C" {
    pub fn e820__range_add(start: u64, size: u64, type: e820_type);
}
extern "C" {
    pub fn e820__range_update(start: u64, size: u64, old_type: e820_type, new_type: e820_type) -> u64;
}
extern "C" {
    pub fn e820__range_remove(start: u64, size: u64, filter_type: e820_type);
}
extern "C" {
    pub fn e820__range_update_table(t: *mut e820_table, start: u64, size: u64, old_type: e820_type, new_type: e820_type) -> u64;
}
extern "C" {
    pub fn e820__update_table(table: *mut e820_table) -> c_int;
}
extern "C" {
    pub fn e820__update_table_print();
}
extern "C" {
    pub fn e820__end_of_ram_pfn() -> c_ulong;
}
extern "C" {
    pub fn e820__end_of_low_ram_pfn() -> c_ulong;
}
extern "C" {
    pub fn e820__memblock_alloc_reserved(size: u64, align: u64) -> u64;
}
extern "C" {
    pub fn e820__memblock_setup();
}
extern "C" {
    pub fn e820__finish_early_params();
}
extern "C" {
    pub fn e820__reserve_resources();
}
extern "C" {
    pub fn e820__reserve_resources_late();
}
extern "C" {
    pub fn e820__memory_setup();
}
extern "C" {
    pub fn e820__memory_setup_extended(phys_addr: u64, data_len: u32);
}
extern "C" {
    pub fn e820__setup_pci_gap();
}
extern "C" {
    pub fn e820__reallocate_tables();
}
extern "C" {
    pub fn e820__register_nosave_regions(limit_pfn: c_ulong);
}
extern "C" {
    pub fn e820__get_entry_type(start: u64, end: u64) -> c_int;
}
//
// Returns true iff the specified range [start,end) is completely contained inside
// the ISA region.
//
