//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/sf/cmd.c
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
// Copyright (c) 2020 Mellanox Technologies Ltd

#[no_mangle]
pub unsafe extern "C" fn mlx5_cmd_alloc_sf(dev: *mut mlx5_core_dev, function_id: u16) -> c_int {
    int mlx5_cmd_alloc_sf(struct mlx5_core_dev *dev, u16 function_id)
    {
    u32 out[MLX5_ST_SZ_DW(alloc_sf_out)] = {};
    u32 in[MLX5_ST_SZ_DW(alloc_sf_in)] = {};
    MLX5_SET(alloc_sf_in, in, opcode, MLX5_CMD_OP_ALLOC_SF);
    MLX5_SET(alloc_sf_in, in, function_id, function_id);
    return mlx5_cmd_exec(dev, in, sizeof(in), out, sizeof(out));
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_cmd_dealloc_sf(dev: *mut mlx5_core_dev, function_id: u16) -> c_int {
    int mlx5_cmd_dealloc_sf(struct mlx5_core_dev *dev, u16 function_id)
    {
    u32 out[MLX5_ST_SZ_DW(dealloc_sf_out)] = {};
    u32 in[MLX5_ST_SZ_DW(dealloc_sf_in)] = {};
    MLX5_SET(dealloc_sf_in, in, opcode, MLX5_CMD_OP_DEALLOC_SF);
    MLX5_SET(dealloc_sf_in, in, function_id, function_id);
    return mlx5_cmd_exec(dev, in, sizeof(in), out, sizeof(out));
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_cmd_sf_enable_hca(dev: *mut mlx5_core_dev, func_id: u16) -> c_int {
    int mlx5_cmd_sf_enable_hca(struct mlx5_core_dev *dev, u16 func_id)
    {
    u32 out[MLX5_ST_SZ_DW(enable_hca_out)] = {};
    u32 in[MLX5_ST_SZ_DW(enable_hca_in)] = {};
    MLX5_SET(enable_hca_in, in, opcode, MLX5_CMD_OP_ENABLE_HCA);
    MLX5_SET(enable_hca_in, in, function_id, func_id);
    MLX5_SET(enable_hca_in, in, embedded_cpu_function, 0);
    return mlx5_cmd_exec(dev, &in, sizeof(in), &out, sizeof(out));
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_cmd_sf_disable_hca(dev: *mut mlx5_core_dev, func_id: u16) -> c_int {
    int mlx5_cmd_sf_disable_hca(struct mlx5_core_dev *dev, u16 func_id)
    {
    u32 out[MLX5_ST_SZ_DW(disable_hca_out)] = {};
    u32 in[MLX5_ST_SZ_DW(disable_hca_in)] = {};
    MLX5_SET(disable_hca_in, in, opcode, MLX5_CMD_OP_DISABLE_HCA);
    MLX5_SET(disable_hca_in, in, function_id, func_id);
    MLX5_SET(enable_hca_in, in, embedded_cpu_function, 0);
    return mlx5_cmd_exec(dev, in, sizeof(in), out, sizeof(out));
    }
