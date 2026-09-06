//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/ttm/ttm_pool.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2020 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Christian König
//

//
// struct ttm_pool_type - Pool for a certain memory type
//
// @pool: the pool we belong to, might be NULL for the global ones
// @order: the allocation order our pages have
// @caching: the caching type our pages have
// @shrinker_list: our place on the global shrinker list
// @pages: the lru_list of pages in the pool
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_pool_type {
    pub pool: *mut ttm_pool,
    pub order: c_uint,
    pub caching: ttm_caching,
    pub shrinker_list: list_head,
    pub pages: list_lru,
}

//
// struct ttm_pool - Pool for all caching and orders
//
// @dev: the device we allocate pages for
// @nid: which numa node to use
// @alloc_flags: TTM_ALLOCATION_POOL_* flags
// @caching: pools for each caching/order
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_pool {
    pub dev: *mut device,
    pub nid: c_int,
    pub alloc_flags: c_uint,
    pub orders: [ttm_pool_type; NR_PAGE_ORDERS],
    pub caching: [}; TTM_NUM_CACHING_TYPES],
}

extern "C" {
    pub fn ttm_pool_free(pool: *mut ttm_pool, tt: *mut ttm_tt);
}
extern "C" {
    pub fn ttm_pool_fini(pool: *mut ttm_pool);
}
extern "C" {
    pub fn ttm_pool_debugfs(pool: *mut ttm_pool, m: *mut seq_file) -> c_int;
}
extern "C" {
    pub fn ttm_pool_drop_backed_up(tt: *mut ttm_tt);
}
extern "C" {
    pub fn ttm_pool_mgr_init(num_pages: c_ulong) -> c_int;
}
extern "C" {
    pub fn ttm_pool_mgr_fini();
}
