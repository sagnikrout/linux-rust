//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en_accel/ktls.h
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

extern "C" {
    pub fn MLX5_CAP_TLS(_arg: mdev, _arg: tls_1_2_aes_gcm_128) -> return;
}
extern "C" {
    pub fn MLX5_CAP_TLS(_arg: mdev, _arg: tls_1_2_aes_gcm_256) -> return;
}
extern "C" {
    pub fn mlx5e_ktls_build_netdev(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_ktls_init_tx(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_ktls_cleanup_tx(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_ktls_init_rx(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_ktls_cleanup_rx(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_ktls_set_feature_rx(netdev: *mut net_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn mlx5e_ktls_rx_resync_destroy_resp_list(resp_list: *mut mlx5e_ktls_resync_resp);
}
extern "C" {
    pub fn mlx5e_is_ktls_rx(mdev: *mut mlx5_core_dev) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tls_sw_stats {
    pub tx_tls_ctx: core::sync::atomic::AtomicI64,
    pub tx_tls_del: core::sync::atomic::AtomicI64,
    pub tx_tls_pool_alloc: core::sync::atomic::AtomicI64,
    pub tx_tls_pool_free: core::sync::atomic::AtomicI64,
    pub rx_tls_ctx: core::sync::atomic::AtomicI64,
    pub rx_tls_del: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tls_debugfs {
    pub dfs: *mut dentry,
    pub dfs_tx: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tls {
    pub mdev: *mut mlx5_core_dev,
    pub sw_stats: mlx5e_tls_sw_stats,
    pub rx_wq: *mut workqueue_struct,
    pub tx_pool: *mut mlx5e_tls_tx_pool,
    pub dek_pool: *mut mlx5_crypto_dek_pool,
    pub debugfs: mlx5e_tls_debugfs,
}

extern "C" {
    pub fn mlx5e_ktls_init(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_ktls_cleanup(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_ktls_get_count(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_ktls_get_strings(priv: *mut mlx5e_priv, data: *mut u8);
}
extern "C" {
    pub fn mlx5e_ktls_get_stats(priv: *mut mlx5e_priv, data: *mut u64);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

