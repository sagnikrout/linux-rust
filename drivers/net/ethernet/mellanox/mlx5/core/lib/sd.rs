//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/sd.h
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
// Copyright (c) 2023, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

pub const MLX5_SD_MIN_GROUP_SZ: c_int = 2;
pub const MLX5_SD_MAX_GROUP_SZ: c_int = 2;
extern "C" {
    pub fn mlx5_sd_is_primary(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_sd_pf_num_get(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_sd_ch_ix_get_dev_ix(dev: *mut mlx5_core_dev, ch_ix: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_sd_ch_ix_get_vec_ix(dev: *mut mlx5_core_dev, ch_ix: c_int) -> c_int;
}

extern "C" {
    pub fn mlx5_sd_is_supported(dev: *mut mlx5_core_dev) -> bool;
}

extern "C" {
    pub fn mlx5_sd_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_sd_cleanup(dev: *mut mlx5_core_dev);
}

extern "C" {
    pub fn mlx5_sd_eswitch_mode_set(dev: *mut mlx5_core_dev, mlx5_mode: u16);
}

