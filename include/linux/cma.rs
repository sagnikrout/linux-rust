//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cma.h
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

pub const CMA_MAX_NAME: c_int = 64;
//
// the buddy -- especially pageblock merging and alloc_contig_range()
// -- can deal with only some pageblocks of a higher-order page being
// MIGRATE_CMA, we can use pageblock_nr_pages.
//

extern "C" {
    pub fn cma_get_base(cma: *const cma) -> phys_addr_t;
}
extern "C" {
    pub fn cma_get_size(cma: *const cma) -> c_ulong;
}
extern "C" {
    pub fn cma_release(cma: *mut cma, pages: *const page, count: c_ulong) -> bool;
}
extern "C" {
    pub fn cma_for_each_area(cma: *mut *mut int (it)(struct cma, data): *mut c_void, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cma_intersects(cma: *mut cma, start: c_ulong, end: c_ulong) -> bool;
}
extern "C" {
    pub fn cma_reserve_pages_on_error(cma: *mut cma);
}
