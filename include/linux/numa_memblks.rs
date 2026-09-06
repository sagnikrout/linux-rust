//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/numa_memblks.h
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
    pub fn numa_set_distance(from: c_int, to: c_int, distance: c_int) -> void __init;
}
extern "C" {
    pub fn numa_reset_distance() -> void __init;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct numa_memblk {
    pub start: u64,
    pub end: u64,
    pub nid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct numa_meminfo {
    pub nr_blks: c_int,
    pub blk: [numa_memblk; NR_NODE_MEMBLKS],
}

extern "C" {
    pub fn numa_add_memblk(nodeid: c_int, start: u64, end: u64) -> int __init;
}
extern "C" {
    pub fn numa_add_reserved_memblk(nid: c_int, start: u64, end: u64) -> int __init;
}
extern "C" {
    pub fn numa_remove_memblk_from(idx: c_int, mi: *mut numa_meminfo) -> void __init;
}
extern "C" {
    pub fn numa_cleanup_meminfo(mi: *mut numa_meminfo) -> int __init;
}

extern "C" {
    pub fn numa_emu_cmdline(str: *mut c_char) -> c_int;
}
extern "C" {
    pub fn numa_emu_dma_end() -> u64 __init;
}

extern "C" {
    pub fn phys_to_target_node(start: u64) -> c_int;
}

extern "C" {
    pub fn memory_add_physaddr_to_nid(start: u64) -> c_int;
}

