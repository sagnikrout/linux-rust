//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/mlx5_ifc_fpga.h
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
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_shell_caps_bits {
    pub max_num_qps: [u8; 0x10],
    pub reserved_at_10: [u8; 0x8],
    pub total_rcv_credits: [u8; 0x8],
    pub reserved_at_20: [u8; 0xe],
    pub qp_type: [u8; 0x2],
    pub reserved_at_30: [u8; 0x5],
    pub rae: [u8; 0x1],
    pub rwe: [u8; 0x1],
    pub rre: [u8; 0x1],
    pub reserved_at_38: [u8; 0x4],
    pub dc: [u8; 0x1],
    pub ud: [u8; 0x1],
    pub uc: [u8; 0x1],
    pub rc: [u8; 0x1],
    pub reserved_at_40: [u8; 0x1a],
    pub log_ddr_size: [u8; 0x6],
    pub max_fpga_qp_msg_size: [u8; 0x20],
    pub reserved_at_80: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_cap_bits {
    pub fpga_id: [u8; 0x8],
    pub fpga_device: [u8; 0x18],
    pub register_file_ver: [u8; 0x20],
    pub fpga_ctrl_modify: [u8; 0x1],
    pub reserved_at_41: [u8; 0x5],
    pub access_reg_query_mode: [u8; 0x2],
    pub reserved_at_48: [u8; 0x6],
    pub access_reg_modify_mode: [u8; 0x2],
    pub reserved_at_50: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub image_version: [u8; 0x20],
    pub image_date: [u8; 0x20],
    pub image_time: [u8; 0x20],
    pub shell_version: [u8; 0x20],
    pub reserved_at_100: [u8; 0x80],
    pub shell_caps: mlx5_ifc_fpga_shell_caps_bits,
    pub reserved_at_380: [u8; 0x8],
    pub ieee_vendor_id: [u8; 0x18],
    pub sandbox_product_version: [u8; 0x10],
    pub sandbox_product_id: [u8; 0x10],
    pub sandbox_basic_caps: [u8; 0x20],
    pub reserved_at_3e0: [u8; 0x10],
    pub sandbox_extended_caps_len: [u8; 0x10],
    pub sandbox_extended_caps_addr: [u8; 0x40],
    pub fpga_ddr_start_addr: [u8; 0x40],
    pub fpga_cr_space_start_addr: [u8; 0x40],
    pub fpga_ddr_size: [u8; 0x20],
    pub fpga_cr_space_size: [u8; 0x20],
    pub reserved_at_500: [u8; 0x300],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_ctrl_bits {
    pub reserved_at_0: [u8; 0x8],
    pub operation: [u8; 0x8],
    pub reserved_at_10: [u8; 0x8],
    pub status: [u8; 0x8],
    pub reserved_at_20: [u8; 0x8],
    pub flash_select_admin: [u8; 0x8],
    pub reserved_at_30: [u8; 0x8],
    pub flash_select_oper: [u8; 0x8],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_error_event_bits {
    pub reserved_at_0: [u8; 0x40],
    pub reserved_at_40: [u8; 0x18],
    pub syndrome: [u8; 0x8],
    pub reserved_at_60: [u8; 0x80],
}

pub const MLX5_FPGA_ACCESS_REG_SIZE_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_access_reg_bits {
    pub reserved_at_0: [u8; 0x20],
    pub reserved_at_20: [u8; 0x10],
    pub size: [u8; 0x10],
    pub address: [u8; 0x40],
    pub data: [u8; 0][0x8],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_fpga_qp_state {
    MLX5_FPGA_QPC_STATE_INIT    = 0x0,
    MLX5_FPGA_QPC_STATE_ACTIVE  = 0x1,
    MLX5_FPGA_QPC_STATE_ERROR   = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_fpga_qp_type {
    MLX5_FPGA_QPC_QP_TYPE_SHELL_QP    = 0x0,
    MLX5_FPGA_QPC_QP_TYPE_SANDBOX_QP  = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_fpga_qp_service_type {
    MLX5_FPGA_QPC_ST_RC  = 0x0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_qpc_bits {
    pub state: [u8; 0x4],
    pub reserved_at_4: [u8; 0x1b],
    pub qp_type: [u8; 0x1],
    pub reserved_at_20: [u8; 0x4],
    pub st: [u8; 0x4],
    pub reserved_at_28: [u8; 0x10],
    pub traffic_class: [u8; 0x8],
    pub ether_type: [u8; 0x10],
    pub prio: [u8; 0x3],
    pub dei: [u8; 0x1],
    pub vid: [u8; 0xc],
    pub reserved_at_60: [u8; 0x20],
    pub reserved_at_80: [u8; 0x8],
    pub next_rcv_psn: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x8],
    pub next_send_psn: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x10],
    pub pkey: [u8; 0x10],
    pub reserved_at_e0: [u8; 0x8],
    pub remote_qpn: [u8; 0x18],
    pub reserved_at_100: [u8; 0x15],
    pub rnr_retry: [u8; 0x3],
    pub reserved_at_118: [u8; 0x5],
    pub retry_count: [u8; 0x3],
    pub reserved_at_120: [u8; 0x20],
    pub reserved_at_140: [u8; 0x10],
    pub remote_mac_47_32: [u8; 0x10],
    pub remote_mac_31_0: [u8; 0x20],
    pub remote_ip: [u8; 16][0x8],
    pub reserved_at_200: [u8; 0x40],
    pub reserved_at_240: [u8; 0x10],
    pub fpga_mac_47_32: [u8; 0x10],
    pub fpga_mac_31_0: [u8; 0x20],
    pub fpga_ip: [u8; 16][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_create_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub fpga_qpc: mlx5_ifc_fpga_qpc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_create_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub fpga_qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub fpga_qpc: mlx5_ifc_fpga_qpc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_modify_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub fpga_qpn: [u8; 0x18],
    pub field_select: [u8; 0x20],
    pub fpga_qpc: mlx5_ifc_fpga_qpc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_modify_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_query_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub fpga_qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_query_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub fpga_qpc: mlx5_ifc_fpga_qpc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_query_qp_counters_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub clear: [u8; 0x1],
    pub reserved_at_41: [u8; 0x7],
    pub fpga_qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_query_qp_counters_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub rx_ack_packets: [u8; 0x40],
    pub rx_send_packets: [u8; 0x40],
    pub tx_ack_packets: [u8; 0x40],
    pub tx_send_packets: [u8; 0x40],
    pub rx_total_drop: [u8; 0x40],
    pub reserved_at_1c0: [u8; 0x1c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_destroy_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub fpga_qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_destroy_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fpga_qp_error_event_bits {
    pub reserved_at_0: [u8; 0x40],
    pub reserved_at_40: [u8; 0x18],
    pub syndrome: [u8; 0x8],
    pub reserved_at_60: [u8; 0x60],
    pub reserved_at_c0: [u8; 0x8],
    pub fpga_qpn: [u8; 0x18],
}
