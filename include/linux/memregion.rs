//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/memregion.h
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
#[derive(Copy, Clone)]
pub struct memregion_info {
    pub target_node: c_int,
    pub range: range,
}

extern "C" {
    pub fn memregion_alloc(gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn memregion_free(id: c_int);
}

//
// cpu_cache_invalidate_memregion - drop any CPU cached data for
// memregion
// @start: start physical address of the target memory region.
// @len: length of the target memory region. -1 for all the regions of
// the target type.
//
// Perform cache maintenance after a memory event / operation that
// changes the contents of physical memory in a cache-incoherent manner.
// For example, device memory technologies like NVDIMM and CXL have
// device secure erase, and dynamic region provision that can replace
// the memory mapped to a given physical address.
//
// Limit the functionality to architectures that have an efficient way
// to writeback and invalidate potentially terabytes of address space at
// once.  Note that this routine may or may not write back any dirty
// contents while performing the invalidation. It is only exported for
// the explicit usage of the NVDIMM and CXL modules in the 'DEVMEM'
// symbol namespace on bare platforms.
//
// Returns 0 on success or negative error code on a failure to perform
// the cache maintenance.
//

extern "C" {
    pub fn cpu_cache_invalidate_memregion(start: phys_addr_t, len: usize) -> c_int;
}
extern "C" {
    pub fn cpu_cache_has_invalidate_memregion() -> bool;
}

extern "C" {
    pub fn cpu_cache_invalidate_memregion(_arg: 0, _arg: -1) -> return;
}
