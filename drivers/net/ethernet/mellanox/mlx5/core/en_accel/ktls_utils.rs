//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en_accel/ktls_utils.h
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

extern "C" {
    pub fn mlx5e_ktls_del_tx(netdev: *mut net_device, tls_ctx: *mut tls_context);
}
extern "C" {
    pub fn mlx5e_ktls_del_rx(netdev: *mut net_device, tls_ctx: *mut tls_context);
}
extern "C" {
    pub fn mlx5e_ktls_rx_resync(netdev: *mut net_device, sk: *mut sock, seq: u32, rcd_sn: *mut u8);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5e_crypto_info {
    pub crypto_info: tls_crypto_info,
    pub crypto_info_128: tls12_crypto_info_aes_gcm_128,
    pub crypto_info_256: tls12_crypto_info_aes_gcm_256,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_set_tls_static_params_wqe {
    pub ctrl: mlx5_wqe_ctrl_seg,
    pub uctrl: mlx5_wqe_umr_ctrl_seg,
    pub mkc: mlx5_mkey_seg,
    pub params: mlx5_wqe_tls_static_params_seg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_set_tls_progress_params_wqe {
    pub ctrl: mlx5_wqe_ctrl_seg,
    pub params: mlx5_wqe_tls_progress_params_seg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_get_tls_progress_params_wqe {
    pub ctrl: mlx5_wqe_ctrl_seg,
    pub psv: mlx5_seg_get_psv,
}

