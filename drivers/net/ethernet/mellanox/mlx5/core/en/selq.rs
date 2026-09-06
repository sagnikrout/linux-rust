//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/selq.h
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
// Copyright (c) 2021, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_selq {
    pub active: *mut mlx5e_selq_params __rcu,
    pub standby: *mut mlx5e_selq_params,
    pub /: *mut *mut *mut mutex state_lock; / points to priv->state_lock,
    pub is_prepared: bool,
}

extern "C" {
    pub fn mlx5e_selq_init(selq: *mut mlx5e_selq, state_lock: *mut mutex) -> c_int;
}
extern "C" {
    pub fn mlx5e_selq_cleanup(selq: *mut mlx5e_selq);
}
extern "C" {
    pub fn mlx5e_selq_prepare_params(selq: *mut mlx5e_selq, params: *mut mlx5e_params);
}
extern "C" {
    pub fn mlx5e_selq_prepare_htb(selq: *mut mlx5e_selq, htb_maj_id: u16, htb_defcls: u16);
}
extern "C" {
    pub fn mlx5e_selq_is_htb_enabled(selq: *mut mlx5e_selq) -> bool;
}
extern "C" {
    pub fn mlx5e_selq_apply(selq: *mut mlx5e_selq);
}
extern "C" {
    pub fn mlx5e_selq_cancel(selq: *mut mlx5e_selq);
}
