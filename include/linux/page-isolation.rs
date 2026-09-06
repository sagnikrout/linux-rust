//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/page-isolation.h
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
// Pageblock isolation modes:
// PB_ISOLATE_MODE_MEM_OFFLINE - isolate to offline (!allocate) memory
// e.g., skip over PageHWPoison() pages and
// PageOffline() pages. Unmovable pages will be
// reported in this mode.
// PB_ISOLATE_MODE_CMA_ALLOC   - isolate for CMA allocations
// PB_ISOLATE_MODE_OTHER       - isolate for other purposes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pb_isolate_mode {
    PB_ISOLATE_MODE_MEM_OFFLINE,
    PB_ISOLATE_MODE_CMA_ALLOC,
    PB_ISOLATE_MODE_OTHER,
}

extern "C" {
    pub fn pageblock_isolate_and_move_free_pages(zone: *mut zone, page: *mut page) -> bool;
}
extern "C" {
    pub fn pageblock_unisolate_and_move_free_pages(zone: *mut zone, page: *mut page) -> bool;
}
extern "C" {
    pub fn undo_isolate_page_range(start_pfn: c_ulong, end_pfn: c_ulong);
}
