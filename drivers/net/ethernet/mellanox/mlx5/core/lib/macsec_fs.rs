//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/macsec_fs.h
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

// Bit31 - 30: MACsec marker, Bit15-0: MACsec id

// MACsec TX flow steering

// MACsec fs_id handling for steering

// MACsec fs_id uses 4 bits, supports up to 16 interfaces
pub const MLX5_MACSEC_NUM_OF_SUPPORTED_INTERFACES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_macsec_rule_attrs {
    pub sci: sci_t,
    pub macsec_obj_id: u32,
    pub assoc_num: u8,
    pub action: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_macsec_stats {
    pub macsec_rx_pkts: u64,
    pub macsec_rx_bytes: u64,
    pub macsec_rx_pkts_drop: u64,
    pub macsec_rx_bytes_drop: u64,
    pub macsec_tx_pkts: u64,
    pub macsec_tx_bytes: u64,
    pub macsec_tx_pkts_drop: u64,
    pub macsec_tx_bytes_drop: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_macsec_action {
    MLX5_ACCEL_MACSEC_ACTION_ENCRYPT,
    MLX5_ACCEL_MACSEC_ACTION_DECRYPT,
}

extern "C" {
    pub fn mlx5_macsec_fs_cleanup(macsec_fs: *mut mlx5_macsec_fs);
}
extern "C" {
    pub fn mlx5_macsec_fs_get_stats_fill(macsec_fs: *mut mlx5_macsec_fs, macsec_stats: *mut c_void);
}
extern "C" {
    pub fn mlx5_macsec_fs_get_fs_id_from_hashtable(macsec_fs: *mut mlx5_macsec_fs, sci: *mut sci_t) -> u32;
}

