//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/memory-tiers.h
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
// Each tier cover a abstrace distance chunk size of 128
//
pub const MEMTIER_CHUNK_BITS: c_int = 7;

//
// Smaller abstract distance values imply faster (higher) memory tiers. Offset
// the DRAM adistance so that we can accommodate devices with a slightly lower
// adistance value (slightly faster) than default DRAM adistance to be part of
// the same memory tier.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_dev_type {
// list of memory types that are part of same tier as this type
    pub tier_sibling: list_head,
// list of memory types that are managed by one driver
    pub list: list_head,
// abstract distance for this specific memory type
    pub adistance: c_int,
// Nodes of same abstract distance
    pub nodes: nodemask_t,
    pub kref: kref,
}

extern "C" {
    pub fn put_memory_type(memtype: *mut memory_dev_type);
}
extern "C" {
    pub fn init_node_memory_type(node: c_int, default_type: *mut memory_dev_type);
}
extern "C" {
    pub fn clear_node_memory_type(node: c_int, memtype: *mut memory_dev_type);
}
extern "C" {
    pub fn register_mt_adistance_algorithm(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_mt_adistance_algorithm(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn mt_calc_adistance(node: c_int, adist: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mt_perf_to_adistance(perf: *mut access_coordinate, adist: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mt_put_memory_types(memory_types: *mut list_head);
}

extern "C" {
    pub fn next_demotion_node(node: c_int, allowed_mask: *const nodemask_t) -> c_int;
}
extern "C" {
    pub fn node_get_allowed_targets(pgdat: *mut pg_data_t, targets: *mut nodemask_t);
}
extern "C" {
    pub fn node_is_toptier(node: c_int) -> bool;
}

// targets = NODE_MASK_NONE;

//
// CONFIG_NUMA implementation returns non NULL error.
//
// targets = NODE_MASK_NONE;

