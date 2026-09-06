//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en_accel/ktls_txrx.h
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
// Copyright (c) 2020, Mellanox Technologies inc. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_accel_tx_tls_state {
    pub tls_tisn: u32,
}

extern "C" {
    pub fn mlx5e_ktls_get_stop_room(mdev: *mut mlx5_core_dev, params: *mut mlx5e_params) -> u16;
}
extern "C" {
    pub fn mlx5e_ktls_handle_ctx_completion(wi: *mut mlx5e_icosq_wqe_info);
}
extern "C" {
    pub fn mlx5e_ktls_rx_handle_resync_list(sq: *mut mlx5e_icosq, budget: c_int) -> bool;
}

