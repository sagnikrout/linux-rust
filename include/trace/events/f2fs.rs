//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/f2fs.h
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
// Number of read folio order buckets emitted by the f2fs_iostat tracepoint.
// TP_printk() cannot loop, so the field count is fixed here and must be >=
// the largest possible NR_PAGE_ORDERS (14 on arm64 with 64K pages). The
// BUILD_BUG_ON() in f2fs_update_read_folio_count() enforces this.
//
pub const F2FS_IOSTAT_RD_FOLIO_ORDERS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_iostat_latency {
    pub peak_lat: c_uint,
    pub avg_lat: c_uint,
    pub cnt: c_uint,
}

//
// Replace the spaces in filenames and cmdlines
// because this screws up the tooling that parses
// the traces.
//

// This part must be outside protection
