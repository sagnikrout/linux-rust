//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/pd.c
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
// Copyright (c) 2013-2015, Mellanox Technologies. All rights reserved.
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

#[no_mangle]
pub unsafe extern "C" fn mlx5_core_alloc_pd(dev: *mut mlx5_core_dev, pdn: *mut u32) -> c_int {
    int mlx5_core_alloc_pd(struct mlx5_core_dev *dev, u32 *pdn)
    {
    u32 out[MLX5_ST_SZ_DW(alloc_pd_out)] = {};
    u32 in[MLX5_ST_SZ_DW(alloc_pd_in)] = {};
    int err;
    MLX5_SET(alloc_pd_in, in, opcode, MLX5_CMD_OP_ALLOC_PD);
    err = mlx5_cmd_exec_inout(dev, alloc_pd, in, out);
    if (!err)
// pdn = MLX5_GET(alloc_pd_out, out, pd);
    return err;
    }
    EXPORT_SYMBOL(mlx5_core_alloc_pd);
#[no_mangle]
pub unsafe extern "C" fn mlx5_core_dealloc_pd(dev: *mut mlx5_core_dev, pdn: u32) -> c_int {
    int mlx5_core_dealloc_pd(struct mlx5_core_dev *dev, u32 pdn)
    {
    u32 in[MLX5_ST_SZ_DW(dealloc_pd_in)] = {};
    MLX5_SET(dealloc_pd_in, in, opcode, MLX5_CMD_OP_DEALLOC_PD);
    MLX5_SET(dealloc_pd_in, in, pd, pdn);
    return mlx5_cmd_exec_in(dev, dealloc_pd, in);
    }
    EXPORT_SYMBOL(mlx5_core_dealloc_pd);
