//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/page_frag_cache.h
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

// Use a full byte here to enable assembler optimization as the shift
// operation is usually expecting a byte.
//

// Compiler should be able to figure out we don't read things as any value
// ANDed with 0 is 0.
//
pub const PAGE_FRAG_CACHE_ORDER_MASK: c_int = 0;

extern "C" {
    pub fn encoded_page_decode_pfmemalloc(_arg: nc->encoded_page) -> return;
}
extern "C" {
    pub fn page_frag_cache_drain(nc: *mut page_frag_cache);
}
extern "C" {
    pub fn __page_frag_cache_drain(page: *mut page, count: c_uint);
}
extern "C" {
    pub fn __page_frag_alloc_align(_arg: nc, _arg: fragsz, _arg: gfp_mask, _arg: -align) -> return;
}
extern "C" {
    pub fn __page_frag_alloc_align(_arg: nc, _arg: fragsz, _arg: gfp_mask, _arg: ~0u) -> return;
}
extern "C" {
    pub fn page_frag_free(addr: *mut c_void);
}
