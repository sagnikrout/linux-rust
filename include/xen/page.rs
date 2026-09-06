//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/page.h
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

// The hypercall interface supports only 4KB page
pub const XEN_PAGE_SHIFT: c_int = 12;

//
// We assume that PAGE_SIZE is a multiple of XEN_PAGE_SIZE
// XXX: Add a BUILD_BUG_ON?
//

// Return the GFN associated to the first 4KB of the page
extern "C" {
    pub fn pfn_to_gfn(_arg: page_to_xen_pfn(page)) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_memory_region {
    pub start_pfn: c_ulong,
    pub n_pfns: c_ulong,
}

