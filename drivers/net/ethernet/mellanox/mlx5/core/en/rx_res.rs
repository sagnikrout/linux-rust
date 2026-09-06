//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/rx_res.h
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
// Copyright (c) 2021, Mellanox Technologies inc. All rights reserved.

pub const MLX5E_MAX_NUM_RSS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_rx_res_features {
    MLX5E_RX_RES_FEATURE_INNER_FT = BIT(0),
    MLX5E_RX_RES_FEATURE_PTP = BIT(1),
    MLX5E_RX_RES_FEATURE_MULTI_VHCA = BIT(2),
    MLX5E_RX_RES_FEATURE_SELF_LB_BLOCK = BIT(3),
}

// Setup
extern "C" {
    pub fn mlx5e_rx_res_destroy(res: *mut mlx5e_rx_res);
}
// TIRN getters for flow steering
extern "C" {
    pub fn mlx5e_rx_res_get_tirn_direct(res: *mut mlx5e_rx_res, ix: c_uint) -> u32;
}
extern "C" {
    pub fn mlx5e_rx_res_get_tirn_rss(res: *mut mlx5e_rx_res, tt: mlx5_traffic_types) -> u32;
}
extern "C" {
    pub fn mlx5e_rx_res_get_tirn_rss_inner(res: *mut mlx5e_rx_res, tt: mlx5_traffic_types) -> u32;
}
extern "C" {
    pub fn mlx5e_rx_res_get_tirn_ptp(res: *mut mlx5e_rx_res) -> u32;
}
extern "C" {
    pub fn mlx5e_rx_res_get_rqtn_direct(res: *mut mlx5e_rx_res, ix: c_uint) -> u32;
}
extern "C" {
    pub fn mlx5e_rx_res_get_max_nch(res: *mut mlx5e_rx_res) -> c_uint;
}
extern "C" {
    pub fn mlx5_rx_res_rss_inner_ft_support(res: *mut mlx5e_rx_res) -> bool;
}
// Activate/deactivate API
extern "C" {
    pub fn mlx5e_rx_res_channels_activate(res: *mut mlx5e_rx_res, chs: *mut mlx5e_channels);
}
extern "C" {
    pub fn mlx5e_rx_res_channels_deactivate(res: *mut mlx5e_rx_res);
}
// Configuration API
extern "C" {
    pub fn mlx5e_rx_res_rss_init(res: *mut mlx5e_rx_res, rss_idx: u32, init_nch: c_uint) -> c_int;
}
extern "C" {
    pub fn mlx5e_rx_res_rss_destroy(res: *mut mlx5e_rx_res, rss_idx: u32) -> c_int;
}
extern "C" {
    pub fn mlx5e_rx_res_rss_cnt(res: *mut mlx5e_rx_res) -> c_int;
}
extern "C" {
    pub fn mlx5e_rx_res_rss_index(res: *mut mlx5e_rx_res, rss: *mut mlx5e_rss) -> c_int;
}
// Workaround for hairpin
extern "C" {
    pub fn mlx5e_rx_res_get_current_hash(res: *mut mlx5e_rx_res) -> mlx5e_rss_params_hash;
}
// Accel TIRs
