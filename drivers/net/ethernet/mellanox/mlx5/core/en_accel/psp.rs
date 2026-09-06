//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en_accel/psp.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_psp_stats {
    pub psp_rx_pkts: u64,
    pub psp_rx_bytes: u64,
    pub psp_rx_pkts_auth_fail: u64,
    pub psp_rx_bytes_auth_fail: u64,
    pub psp_rx_pkts_frame_err: u64,
    pub psp_rx_bytes_frame_err: u64,
    pub psp_rx_pkts_drop: u64,
    pub psp_rx_bytes_drop: u64,
    pub psp_tx_pkts: u64,
    pub psp_tx_bytes: u64,
    pub psp_tx_pkts_drop: u64,
    pub psp_tx_bytes_drop: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_psp {
    pub psd: *mut psp_dev,
    pub caps: psp_dev_caps,
    pub fs: *mut mlx5e_psp_fs,
    pub tx_key_cnt: core::sync::atomic::AtomicI32,
    pub tx_drop: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn mlx5_accel_psp_fs_cleanup_rx_tables(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5_accel_psp_fs_cleanup_tx_tables(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_psp_register(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_psp_unregister(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_psp_init(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_psp_cleanup(priv: *mut mlx5e_priv);
}

