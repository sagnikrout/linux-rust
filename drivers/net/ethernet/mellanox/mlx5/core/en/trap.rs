//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/trap.h
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
// Copyright (c) 2020, Mellanox Technologies

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_trap {
// data path
    pub rq: mlx5e_rq,
    pub tir: mlx5e_tir,
    pub napi: napi_struct,
    pub pdev: *mut device,
    pub netdev: *mut net_device,
    pub mkey_be: __be32,
// data path - accessed per napi poll
    pub stats: *mut mlx5e_ch_stats,
// control
    pub priv: *mut mlx5e_priv,
    pub mdev: *mut mlx5_core_dev,
    pub MLX5E_CHANNEL_NUM_STATES): DECLARE_BITMAP(state,,
    pub params: mlx5e_params,
    pub rq_param: mlx5e_rq_param,
}

extern "C" {
    pub fn mlx5e_close_trap(trap: *mut mlx5e_trap);
}
extern "C" {
    pub fn mlx5e_deactivate_trap(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_handle_trap_event(priv: *mut mlx5e_priv, trap_ctx: *mut mlx5_trap_ctx) -> c_int;
}
extern "C" {
    pub fn mlx5e_apply_traps(priv: *mut mlx5e_priv, enable: bool) -> c_int;
}
