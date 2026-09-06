//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/hwbm.h
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
pub struct hwbm_pool {
// Capacity of the pool
    pub size: c_int,
// Size of the buffers managed
    pub frag_size: c_int,
// Number of buffers currently used by this pool
    pub buf_num: c_int,
// constructor called during allocation
    pub buf): *mut *mut *mut int (construct)(struct hwbm_pool bm_pool, void,
// protect access to the buffer counter
    pub buf_lock: mutex,
// private data
    pub priv: *mut c_void,
}

extern "C" {
    pub fn hwbm_buf_free(bm_pool: *mut hwbm_pool, buf: *mut c_void);
}
extern "C" {
    pub fn hwbm_pool_refill(bm_pool: *mut hwbm_pool, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn hwbm_pool_add(bm_pool: *mut hwbm_pool, buf_num: c_uint) -> c_int;
}

