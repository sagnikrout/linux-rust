//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/fs_pool.h
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
// Copyright (c) 2024 NVIDIA Corporation & Affiliates

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_bulk {
    pub pool_list: list_head,
    pub bulk_len: c_int,
    pub bitmask: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_pool_index {
    pub fs_bulk: *mut mlx5_fs_bulk,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_pool_ops {
    pub bulk): *mut *mut *mut int (bulk_destroy)(struct mlx5_core_dev dev, struct mlx5_fs_bulk,
    pub pool_ctx): *mut c_void,
    pub pool): *mut *mut void (update_threshold)(struct mlx5_fs_pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_pool {
    pub dev: *mut mlx5_core_dev,
    pub pool_ctx: *mut c_void,
    pub ops: *const mlx5_fs_pool_ops,
    pub /: *mut *mut mutex pool_lock; / protects pool lists,
    pub fully_used: list_head,
    pub partially_used: list_head,
    pub unused: list_head,
    pub available_units: c_int,
    pub used_units: c_int,
    pub threshold: c_int,
}

extern "C" {
    pub fn mlx5_fs_bulk_init(fs_bulk: *mut mlx5_fs_bulk, bulk_len: c_int);
}
extern "C" {
    pub fn mlx5_fs_bulk_cleanup(fs_bulk: *mut mlx5_fs_bulk);
}
extern "C" {
    pub fn mlx5_fs_bulk_get_free_amount(bulk: *mut mlx5_fs_bulk) -> c_int;
}
extern "C" {
    pub fn mlx5_fs_pool_cleanup(pool: *mut mlx5_fs_pool);
}
