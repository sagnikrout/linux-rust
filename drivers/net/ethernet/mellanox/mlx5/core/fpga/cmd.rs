//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/fpga/cmd.h
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
// Copyright (c) 2017, Mellanox Technologies, Ltd.  All rights reserved.
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_fpga_id {
    MLX5_FPGA_NEWTON = 0,
    MLX5_FPGA_EDISON = 1,
    MLX5_FPGA_MORSE = 2,
    MLX5_FPGA_MORSEQ = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_fpga_image {
    MLX5_FPGA_IMAGE_USER = 0,
    MLX5_FPGA_IMAGE_FACTORY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_fpga_status {
    MLX5_FPGA_STATUS_SUCCESS = 0,
    MLX5_FPGA_STATUS_FAILURE = 1,
    MLX5_FPGA_STATUS_IN_PROGRESS = 2,
    MLX5_FPGA_STATUS_NONE = 0xFFFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fpga_query {
    pub admin_image: mlx5_fpga_image,
    pub oper_image: mlx5_fpga_image,
    pub status: mlx5_fpga_status,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_fpga_qpc_field_select {
    MLX5_FPGA_QPC_STATE = BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fpga_qp_counters {
    pub rx_ack_packets: u64,
    pub rx_send_packets: u64,
    pub tx_ack_packets: u64,
    pub tx_send_packets: u64,
    pub rx_total_drop: u64,
}

extern "C" {
    pub fn mlx5_fpga_caps(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_fpga_query(dev: *mut mlx5_core_dev, query: *mut mlx5_fpga_query) -> c_int;
}
extern "C" {
    pub fn mlx5_fpga_ctrl_op(dev: *mut mlx5_core_dev, op: u8) -> c_int;
}
extern "C" {
    pub fn mlx5_fpga_sbu_caps(dev: *mut mlx5_core_dev, caps: *mut c_void, size: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_fpga_query_qp(dev: *mut mlx5_core_dev, fpga_qpn: u32, fpga_qpc: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mlx5_fpga_destroy_qp(dev: *mut mlx5_core_dev, fpga_qpn: u32) -> c_int;
}
