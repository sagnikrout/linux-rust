//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/percpu.h
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

// enough to cover all DEFINE_PER_CPUs in modules

pub const PERCPU_MODULE_RESERVE: c_int = 0;

// minimum unit size, also is the maximum supported allocation size

// minimum allocation size and shift in bytes
pub const PCPU_MIN_ALLOC_SHIFT: c_int = 2;

//
// The PCPU_BITMAP_BLOCK_SIZE must be the same size as PAGE_SIZE as the
// updating of hints is used to manage the nr_empty_pop_pages in both
// the chunk and globally.
//

pub const PERCPU_DYNAMIC_SIZE_SHIFT: c_int = 10;

//
// Percpu allocator can serve percpu allocations before slab is
// initialized which allows slab to depend on the percpu allocator.
// The following parameter decide how much resource to preallocate
// for this.  Keep PERCPU_DYNAMIC_RESERVE equal to or larger than
// PERCPU_DYNAMIC_EARLY_SIZE.
//

//
// PERCPU_DYNAMIC_RESERVE indicates the amount of free area to piggy
// back on the first chunk for dynamic percpu allocation if arch is
// manually allocating and mapping it for faster access (as a part of
// large page mapping for example).
//
// The following values give between one and two pages of free space
// after typical minimal boot (2-way SMP, single disk and NIC) with
// both defconfig and a distro config on x86_64 and 32.  More
// intelligent way to determine this would be nice.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_group_info {
    pub /: *mut *mut int nr_units; / aligned # of units,
    pub /: *mut *mut unsigned long base_offset; / base address offset,
    pub empty: *mut *mut *mut unsigned int cpu_map; / unit->cpu map,,
// entries contain NR_CPUS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_alloc_info {
    pub static_size: usize,
    pub reserved_size: usize,
    pub dyn_size: usize,
    pub unit_size: usize,
    pub atom_size: usize,
    pub alloc_size: usize,
    pub /: *mut *mut size_t __ai_size; / internal, don't use,
    pub /: *mut *mut int nr_groups; / 0 if grouping unnecessary,
    pub groups: [pcpu_group_info; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcpu_fc {
    PCPU_FC_AUTO,
    PCPU_FC_EMBED,
    PCPU_FC_PAGE,

    PCPU_FC_NR,
}

extern "C" {
    pub fn int(cpu: pcpu_fc_cpu_to_node_fn_t)(int) -> typedef;
}
extern "C" {
    pub fn int(from: pcpu_fc_cpu_distance_fn_t)(unsigned int, to: c_uint) -> typedef;
}
extern "C" {
    pub fn pcpu_free_alloc_info(ai: *mut pcpu_alloc_info) -> void __init;
}

extern "C" {
    pub fn pcpu_populate_pte(addr: c_ulong) -> void __init;
}

extern "C" {
    pub fn __is_kernel_percpu_address(addr: c_ulong, can_addr: *mut c_ulong) -> bool;
}
extern "C" {
    pub fn is_kernel_percpu_address(addr: c_ulong) -> bool;
}

extern "C" {
    pub fn setup_per_cpu_areas() -> void __init;
}

extern "C" {
    pub fn free_percpu(__pdata: *mut void __percpu);
}
extern "C" {
    pub fn per_cpu_ptr_to_phys(addr: *mut c_void) -> phys_addr_t;
}
extern "C" {
    pub fn pcpu_nr_pages() -> c_ulong;
}
