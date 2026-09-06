//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/fpga/conn.h
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
// Copyright (c) 2017 Mellanox Technologies. All rights reserved.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fpga_conn {
    pub fdev: *mut mlx5_fpga_device,
    pub buf): *mut *mut *mut void (recv_cb)(void cb_arg, struct mlx5_fpga_dma_buf,
    pub cb_arg: *mut c_void,
// FPGA QP
    pub fpga_qpc: [u32; MLX5_ST_SZ_DW(fpga_qpc)],
    pub fpga_qpn: u32,
// CQ
    pub wq: mlx5_cqwq,
    pub wq_ctrl: mlx5_wq_ctrl,
    pub mcq: mlx5_core_cq,
    pub tasklet: tasklet_struct,
    pub cq: },
// QP
    pub active: bool,
    pub sgid_index: c_int,
    pub wq: mlx5_wq_qp,
    pub wq_ctrl: mlx5_wq_ctrl,
    pub qpn: u32,
    pub /: *mut *mut spinlock_t lock; / Protects all SQ state,
    pub pc: c_uint,
    pub cc: c_uint,
    pub size: c_uint,
    pub bufs: *mut mlx5_fpga_dma_buf,
    pub backlog: list_head,
    pub sq: },
    pub pc: c_uint,
    pub cc: c_uint,
    pub size: c_uint,
    pub bufs: *mut mlx5_fpga_dma_buf,
    pub rq: },
    pub qp: },
}

extern "C" {
    pub fn mlx5_fpga_conn_device_init(fdev: *mut mlx5_fpga_device) -> c_int;
}
extern "C" {
    pub fn mlx5_fpga_conn_device_cleanup(fdev: *mut mlx5_fpga_device);
}
extern "C" {
    pub fn mlx5_fpga_conn_destroy(conn: *mut mlx5_fpga_conn);
}
