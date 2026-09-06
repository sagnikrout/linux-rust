//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/mlx5.h
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
// Copyright (c) 2017, Mellanox Technologies, Ltd.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

extern "C" {
    pub fn mlx5_init_reserved_gids(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_cleanup_reserved_gids(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_core_reserve_gids(dev: *mut mlx5_core_dev, count: c_uint) -> c_int;
}
extern "C" {
    pub fn mlx5_core_unreserve_gids(dev: *mut mlx5_core_dev, count: c_uint);
}
extern "C" {
    pub fn mlx5_core_reserved_gid_alloc(dev: *mut mlx5_core_dev, gid_index: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_core_reserved_gid_free(dev: *mut mlx5_core_dev, gid_index: c_int);
}
extern "C" {
    pub fn mlx5_crdump_enable(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_crdump_disable(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_crdump_collect(dev: *mut mlx5_core_dev, cr_data: *mut u32) -> c_int;
}
