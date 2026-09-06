//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mempool.h
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
// memory buffer pool support
//

extern "C" {
    pub fn void(element: *mut mempool_free_t)(void, pool_data: *mut c_void) -> typedef;
}
extern "C" {
    pub fn mempool_exit(pool: *mut mempool);
}

extern "C" {
    pub fn mempool_resize(pool: *mut mempool, new_min_nr: c_int) -> c_int;
}
extern "C" {
    pub fn mempool_destroy(pool: *mut mempool);
}

extern "C" {
    pub fn mempool_free(element: *mut c_void, pool: *mut mempool);
}
//
// A mempool_alloc_t and mempool_free_t that get the memory from
// a slab cache that is passed in through pool_data.
// Note: the slab cache may not have a ctor function.
//
extern "C" {
    pub fn mempool_free_slab(element: *mut c_void, pool_data: *mut c_void);
}

//
// a mempool_alloc_t and a mempool_free_t to kmalloc and kfree the
// amount of memory specified by pool_data
//
extern "C" {
    pub fn mempool_kfree(element: *mut c_void, pool_data: *mut c_void);
}

//
// A mempool_alloc_t and mempool_free_t for a simple page allocator that
// allocates pages of the order specified by pool_data
//
extern "C" {
    pub fn mempool_free_pages(element: *mut c_void, pool_data: *mut c_void);
}

