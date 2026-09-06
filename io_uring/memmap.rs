//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/memmap.h
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
pub const IORING_MAP_OFF_PARAM_REGION: c_uint = 0x20000000ULL;
pub const IORING_MAP_OFF_ZCRX_REGION: c_uint = 0x30000000ULL;
pub const IORING_OFF_ZCRX_SHIFT: c_int = 16;

extern "C" {
    pub fn io_uring_nommu_mmap_capabilities(file: *mut file) -> c_uint;
}

extern "C" {
    pub fn io_uring_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn io_free_region(user: *mut user_struct, mr: *mut io_mapped_region);
}
//
// Once published mmap can find it without holding only the ->mmap_lock
// and not ->uring_lock.
//
// dst_region = *src_region;
