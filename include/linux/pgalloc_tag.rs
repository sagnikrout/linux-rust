//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pgalloc_tag.h
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
// page allocation tagging
//

pub type pgalloc_tag_idx = u16;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pgtag_ref_handle {
    pub /: *mut *mut *mut codetag_ref ref; / reference in page extension,
    pub /: *mut *mut *mut page page; / reference in page flags,
}

// Reserved indexes
pub const CODETAG_ID_NULL: c_int = 0;
pub const CODETAG_ID_EMPTY: c_int = 1;
pub const CODETAG_ID_FIRST: c_int = 2;

extern "C" {
    pub fn module_tag_to_idx(_arg: tag) -> return;
}
// Should be called only if mem_alloc_profiling_enabled()
extern "C" {
    pub fn __clear_page_tag_ref(page: *mut page);
}
// Should be called only if mem_alloc_profiling_enabled()
extern "C" {
    pub fn __pgalloc_tag_get(_arg: page) -> return;
}
extern "C" {
    pub fn pgalloc_tag_split(folio: *mut folio, old_order: c_int, new_order: c_int);
}
extern "C" {
    pub fn pgalloc_tag_swap(new: *mut folio, old: *mut folio);
}
extern "C" {
    pub fn alloc_tag_sec_init() -> void __init;
}

