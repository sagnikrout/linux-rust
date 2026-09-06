//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/pool.h
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
pub const MLX5HWS_POOL_STC_LOG_SZ: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_pool_type {
    MLX5HWS_POOL_TYPE_STE,
    MLX5HWS_POOL_TYPE_STC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_pool_chunk {
    pub offset: c_int,
    pub order: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_pool_resource {
    pub pool: *mut mlx5hws_pool,
    pub base_id: u32,
    pub range: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_pool_flags {
// Managed by a buddy allocator. If this is not set only allocations of
// order 0 are supported.
//
    MLX5HWS_POOL_FLAG_BUDDY = BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_pool_optimize {
    MLX5HWS_POOL_OPTIMIZE_NONE = 0x0,
    MLX5HWS_POOL_OPTIMIZE_ORIG = 0x1,
    MLX5HWS_POOL_OPTIMIZE_MIRROR = 0x2,
    MLX5HWS_POOL_OPTIMIZE_MAX = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_pool_attr {
    pub pool_type: mlx5hws_pool_type,
    pub table_type: mlx5hws_table_type,
    pub flags: mlx5hws_pool_flags,
    pub opt_type: mlx5hws_pool_optimize,
// Allocation size once memory is depleted
    pub alloc_log_sz: usize,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_db_type {
// Uses a bitmap, supports only allocations of order 0.
    MLX5HWS_POOL_DB_TYPE_BITMAP,
// Entries are managed using a buddy mechanism.
    MLX5HWS_POOL_DB_TYPE_BUDDY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_pool_db {
    pub type: mlx5hws_db_type,
    pub bitmap: *mut c_ulong,
    pub buddy: *mut mlx5hws_buddy_mem,
}

extern "C" {
    pub fn void(pool: *mut *mut mlx5hws_pool_unint_db)(struct mlx5hws_pool) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_pool {
    pub ctx: *mut mlx5hws_context,
    pub type: mlx5hws_pool_type,
    pub flags: mlx5hws_pool_flags,
    pub /: *mut *mut mutex lock; / protect the pool,
    pub alloc_log_sz: usize,
    pub available_elems: usize,
    pub tbl_type: mlx5hws_table_type,
    pub opt_type: mlx5hws_pool_optimize,
    pub resource: *mut mlx5hws_pool_resource,
    pub mirror_resource: *mut mlx5hws_pool_resource,
    pub db: mlx5hws_pool_db,
// Functions
    pub p_db_uninit: mlx5hws_pool_unint_db,
    pub p_get_chunk: mlx5hws_pool_db_get_chunk,
    pub p_put_chunk: mlx5hws_pool_db_put_chunk,
}

extern "C" {
    pub fn mlx5hws_pool_destroy(pool: *mut mlx5hws_pool);
}
