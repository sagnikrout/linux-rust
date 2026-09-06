//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mlx5/devx.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2019-2020, Mellanox Technologies inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devx_obj {
    pub ib_dev: *mut mlx5_ib_dev,
    pub obj_id: u64,
    pub /: *mut *mut u32 dinlen; / destroy inbox length,
    pub dinbox: [u32; MLX5_MAX_DESTROY_INBOX_SIZE_DW],
    pub flags: u32,
    pub mkey: mlx5_ib_mkey,
    pub core_dct: mlx5_core_dct,
    pub core_cq: mlx5_core_cq,
    pub flow_counter_bulk_size: u32,
}

extern "C" {
    pub fn mlx5_ib_devx_create(dev: *mut mlx5_ib_dev, is_user: bool, req_ucaps: u64) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_devx_destroy(dev: *mut mlx5_ib_dev, uid: u16);
}
extern "C" {
    pub fn mlx5_ib_devx_init(dev: *mut mlx5_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_devx_cleanup(dev: *mut mlx5_ib_dev);
}
extern "C" {
    pub fn mlx5_ib_ufile_hw_cleanup(ufile: *mut ib_uverbs_file);
}

