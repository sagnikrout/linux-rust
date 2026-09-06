//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/numa.h
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

// optionally keep NUMA memory info available post init

// Macro flag: #define __initdata_or_meminfo

extern "C" {
    pub fn alloc_node_data(nid: c_int) -> void __init;
}
extern "C" {
    pub fn alloc_offline_node_data(nid: c_int) -> void __init;
}
// Generic implementation available
extern "C" {
    pub fn numa_nearest_node(node: c_int, state: c_uint) -> c_int;
}
extern "C" {
    pub fn nearest_node_nodemask(node: c_int, mask: *mut nodemask_t) -> c_int;
}

extern "C" {
    pub fn memory_add_physaddr_to_nid(start: u64) -> c_int;
}

extern "C" {
    pub fn phys_to_target_node(start: u64) -> c_int;
}

extern "C" {
    pub fn numa_fill_memblks(start: u64, end: u64) -> c_int;
}

