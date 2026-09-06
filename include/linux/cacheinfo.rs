//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cacheinfo.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cache_type {
    CACHE_TYPE_NOCACHE = 0,
    CACHE_TYPE_INST = BIT(0),
    CACHE_TYPE_DATA = BIT(1),
    CACHE_TYPE_SEPARATE = CACHE_TYPE_INST | CACHE_TYPE_DATA,
    CACHE_TYPE_UNIFIED = BIT(2),
}

//
// struct cacheinfo - represent a cache leaf node
// @id: This cache's id. It is unique among caches with the same (type, level).
// @type: type of the cache - data, inst or unified
// @level: represents the hierarchy in the multi-level cache
// @coherency_line_size: size of each cache line usually representing
// the minimum amount of data that gets transferred from memory
// @number_of_sets: total number of sets, a set is a collection of cache
// lines sharing the same index
// @ways_of_associativity: number of ways in which a particular memory
// block can be placed in the cache
// @physical_line_partition: number of physical cache lines sharing the
// same cachetag
// @size: Total size of the cache
// @shared_cpu_map: logical cpumask representing all the cpus sharing
// this cache node
// @attributes: bitfield representing various cache attributes
// @fw_token: Unique value used to determine if different cacheinfo
// structures represent a single hardware cache instance.
// @disable_sysfs: indicates whether this node is visible to the user via
// sysfs or not
// @priv: pointer to any private data structure specific to particular
// cache design
//
// While @of_node, @disable_sysfs and @priv are used for internal book
// keeping, the remaining members form the core properties of the cache
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cacheinfo {
    pub id: c_uint,
    pub type: cache_type,
    pub level: c_uint,
    pub coherency_line_size: c_uint,
    pub number_of_sets: c_uint,
    pub ways_of_associativity: c_uint,
    pub physical_line_partition: c_uint,
    pub size: c_uint,
    pub shared_cpu_map: cpumask_t,
    pub attributes: c_uint,

    pub fw_token: *mut c_void,
    pub disable_sysfs: bool,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_cacheinfo {
    pub info_list: *mut cacheinfo,
    pub per_cpu_data_slice_size: c_uint,
    pub num_levels: c_uint,
    pub num_leaves: c_uint,
    pub cpu_map_populated: bool,
    pub early_ci_levels: bool,
}

extern "C" {
    pub fn early_cache_level(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn init_cache_level(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn init_of_cache_level(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn populate_cache_leaves(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn cache_setup_acpi(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn last_level_cache_is_valid(cpu: c_uint) -> bool;
}
extern "C" {
    pub fn last_level_cache_is_shared(cpu_x: c_uint, cpu_y: c_uint) -> bool;
}
extern "C" {
    pub fn fetch_cache_info(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn detect_cache_attributes(cpu: c_uint) -> c_int;
}

//
// acpi_get_cache_info() is only called on ACPI enabled
// platforms using the PPTT for topology. This means that if
// the platform supports other firmware configuration methods
// we need to stub out the call when ACPI is disabled.
// ACPI enabled platforms not using PPTT won't be making calls
// to this function so we need not worry about them.
//

//
// Get the cacheinfo structure for the cache associated with @cpu at
// level @level.
// cpuhp lock must be held.
//
// Get the id of the cache associated with @cpu at level @level.
// cpuhp lock must be held.
//

