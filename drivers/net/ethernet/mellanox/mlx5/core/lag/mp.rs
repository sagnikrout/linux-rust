//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lag/mp.h
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
// Copyright (c) 2019 Mellanox Technologies.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_lag_port_affinity {
    MLX5_LAG_NORMAL_AFFINITY,
    MLX5_LAG_P1_AFFINITY,
    MLX5_LAG_P2_AFFINITY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lag_mp {
    pub fib_nb: notifier_block,
    pub /: *const *const *const void mfi; / used in tracking fib events,
    pub priority: u32,
    pub dst: u32,
    pub dst_len: c_int,
    pub fib: },
    pub wq: *mut workqueue_struct,
}

extern "C" {
    pub fn mlx5_lag_mp_reset(ldev: *mut mlx5_lag);
}
extern "C" {
    pub fn mlx5_lag_mp_init(ldev: *mut mlx5_lag) -> c_int;
}
extern "C" {
    pub fn mlx5_lag_mp_cleanup(ldev: *mut mlx5_lag);
}
extern "C" {
    pub fn mlx5_lag_is_multipath(dev: *mut mlx5_core_dev) -> bool;
}

