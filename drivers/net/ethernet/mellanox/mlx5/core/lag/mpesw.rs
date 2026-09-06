//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lag/mpesw.h
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
// Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lag_mpesw {
    pub mpesw_work: work_struct,
    pub pf_metadata: [u32; MLX5_MAX_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpesw_op {
    MLX5_MPESW_OP_ENABLE,
    MLX5_MPESW_OP_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_mpesw_work_st {
    pub work: work_struct,
    pub lag: *mut mlx5_lag,
    pub op: mpesw_op,
    pub comp: completion,
    pub result: c_int,
}

extern "C" {
    pub fn mlx5_lag_is_mpesw(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_lag_mpesw_disable(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_lag_mpesw_enable(dev: *mut mlx5_core_dev) -> c_int;
}

extern "C" {
    pub fn mlx5_lag_disable_mpesw(ldev: *mut mlx5_lag);
}
extern "C" {
    pub fn mlx5_mpesw_sd_devcoms_lock(ldev: *mut mlx5_lag);
}
extern "C" {
    pub fn mlx5_mpesw_sd_devcoms_unlock(ldev: *mut mlx5_lag);
}

extern "C" {
    pub fn mlx5_mpesw_speed_update_work(work: *mut work_struct);
}

