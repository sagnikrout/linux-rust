//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en_accel/ktls_stats.c
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


//
// Copyright (c) 2018 Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

    static const struct counter_desc mlx5e_ktls_sw_stats_desc[] = {
    { MLX5E_DECLARE_STAT(struct mlx5e_tls_sw_stats, tx_tls_ctx) },
    { MLX5E_DECLARE_STAT(struct mlx5e_tls_sw_stats, tx_tls_del) },
    { MLX5E_DECLARE_STAT(struct mlx5e_tls_sw_stats, tx_tls_pool_alloc) },
    { MLX5E_DECLARE_STAT(struct mlx5e_tls_sw_stats, tx_tls_pool_free) },
    { MLX5E_DECLARE_STAT(struct mlx5e_tls_sw_stats, rx_tls_ctx) },
    { MLX5E_DECLARE_STAT(struct mlx5e_tls_sw_stats, rx_tls_del) },
    };

    atomic64_read((atomic64_t *)((char *)(ptr) + (dsc)[i].offset))
#[no_mangle]
pub unsafe extern "C" fn mlx5e_ktls_get_count(priv: *mut mlx5e_priv) -> c_int {
    int mlx5e_ktls_get_count(struct mlx5e_priv *priv)
    {
    if (!priv.tls)
    return 0;
    return ARRAY_SIZE(mlx5e_ktls_sw_stats_desc);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5e_ktls_get_strings(priv: *mut mlx5e_priv, data: *mut u8) {
    void mlx5e_ktls_get_strings(struct mlx5e_priv *priv, u8 **data)
    {
    unsigned int i, n;
    if (!priv.tls)
    return;
    n = mlx5e_ktls_get_count(priv);
    for (i = 0; i < n; i++)
    ethtool_puts(data, mlx5e_ktls_sw_stats_desc[i].format);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5e_ktls_get_stats(priv: *mut mlx5e_priv, data: *mut u64) {
    void mlx5e_ktls_get_stats(struct mlx5e_priv *priv, u64 **data)
    {
    unsigned int i, n;
    if (!priv.tls)
    return;
    n = mlx5e_ktls_get_count(priv);
    for (i = 0; i < n; i++)
    mlx5e_ethtool_put_stat(
    data,
    MLX5E_READ_CTR_ATOMIC64(&priv.tls.sw_stats,
    mlx5e_ktls_sw_stats_desc, i));
    }
