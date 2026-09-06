//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en_accel/macsec_stats.c
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

    static const struct counter_desc mlx5e_macsec_hw_stats_desc[] = {
    { MLX5E_DECLARE_STAT(struct mlx5_macsec_stats, macsec_rx_pkts) },
    { MLX5E_DECLARE_STAT(struct mlx5_macsec_stats, macsec_rx_bytes) },
    { MLX5E_DECLARE_STAT(struct mlx5_macsec_stats, macsec_rx_pkts_drop) },
    { MLX5E_DECLARE_STAT(struct mlx5_macsec_stats, macsec_rx_bytes_drop) },
    { MLX5E_DECLARE_STAT(struct mlx5_macsec_stats, macsec_tx_pkts) },
    { MLX5E_DECLARE_STAT(struct mlx5_macsec_stats, macsec_tx_bytes) },
    { MLX5E_DECLARE_STAT(struct mlx5_macsec_stats, macsec_tx_pkts_drop) },
    { MLX5E_DECLARE_STAT(struct mlx5_macsec_stats, macsec_tx_bytes_drop) },
    };

#[no_mangle]
pub unsafe extern "C" fn MLX5E_DECLARE_STATS_GRP_OP_NUM_STATS(_arg: macsec_hw) -> static {
    static MLX5E_DECLARE_STATS_GRP_OP_NUM_STATS(macsec_hw)
    {
    if (!priv.macsec)
    return 0;
    if (mlx5e_is_macsec_device(priv.mdev))
    return NUM_MACSEC_HW_COUNTERS;
    return 0;
    }
    static MLX5E_DECLARE_STATS_GRP_OP_UPDATE_STATS(macsec_hw) {}
#[no_mangle]
pub unsafe extern "C" fn MLX5E_DECLARE_STATS_GRP_OP_FILL_STRS(_arg: macsec_hw) -> static {
    static MLX5E_DECLARE_STATS_GRP_OP_FILL_STRS(macsec_hw)
    {
    unsigned int i;
    if (!priv.macsec)
    return;
    if (!mlx5e_is_macsec_device(priv.mdev))
    return;
    for (i = 0; i < NUM_MACSEC_HW_COUNTERS; i++)
    ethtool_puts(data, mlx5e_macsec_hw_stats_desc[i].format);
    }
#[no_mangle]
pub unsafe extern "C" fn MLX5E_DECLARE_STATS_GRP_OP_FILL_STATS(_arg: macsec_hw) -> static {
    static MLX5E_DECLARE_STATS_GRP_OP_FILL_STATS(macsec_hw)
    {
    struct mlx5_macsec_fs *macsec_fs;
    int i;
    if (!priv.macsec)
    return;
    if (!mlx5e_is_macsec_device(priv.mdev))
    return;
    macsec_fs = priv.mdev.macsec_fs;
    mlx5_macsec_fs_get_stats_fill(macsec_fs, mlx5_macsec_fs_get_stats(macsec_fs));
    for (i = 0; i < NUM_MACSEC_HW_COUNTERS; i++)
    mlx5e_ethtool_put_stat(
    data, MLX5E_READ_CTR64_CPU(
    mlx5_macsec_fs_get_stats(macsec_fs),
    mlx5e_macsec_hw_stats_desc, i));
    }
    MLX5E_DEFINE_STATS_GRP(macsec_hw, 0);
