//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/zsmalloc.h
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


//
// zsmalloc memory allocator
//
// Copyright (C) 2011  Nitin Gupta
// Copyright (C) 2012, 2013 Minchan Kim
//
// This code is released using a dual license strategy: BSD/GPL
// You can choose the license that better fits your requirements.
//
// Released under the terms of 3-clause BSD License
// Released under the terms of GNU General Public License Version 2.0
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zs_pool_stats {
// How many pages were migrated (freed)
    pub pages_compacted: atomic_long_t,
}

extern "C" {
    pub fn zs_destroy_pool(pool: *mut zs_pool);
}
extern "C" {
    pub fn zs_free(pool: *mut zs_pool, obj: c_ulong);
}
extern "C" {
    pub fn zs_huge_class_size(pool: *mut zs_pool) -> usize;
}
extern "C" {
    pub fn zs_get_total_pages(pool: *mut zs_pool) -> c_ulong;
}
extern "C" {
    pub fn zs_compact(pool: *mut zs_pool) -> c_ulong;
}
extern "C" {
    pub fn zs_lookup_class_index(pool: *mut zs_pool, size: c_uint) -> c_uint;
}
extern "C" {
    pub fn zs_pool_stats(pool: *mut zs_pool, stats: *mut zs_pool_stats);
}
extern "C" {
    pub fn zs_obj_read_sg_end(pool: *mut zs_pool, handle: c_ulong);
}
