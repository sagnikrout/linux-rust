//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/mlx4-abi.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB)
//
// Copyright (c) 2007 Cisco Systems, Inc. All rights reserved.
// Copyright (c) 2007, 2008 Mellanox Technologies. All rights reserved.
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

//
// Increment this value if any changes that break userspace ABI
// compatibility are made.
//
pub const MLX4_IB_UVERBS_NO_DEV_CAPS_ABI_VERSION: c_int = 3;
pub const MLX4_IB_UVERBS_ABI_VERSION: c_int = 4;
//
// Make sure that all structs defined in this file remain laid out so
// that they pack the same way on 32-bit and 64-bit architectures (to
// avoid incompatibility between 32-bit userspace and 64-bit kernels).
// In particular do not use pointer types -- pass pointers in __u64
// instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_alloc_ucontext_resp_v3 {
    pub qp_tab_size: __u32,
    pub bf_reg_size: __u16,
    pub bf_regs_per_page: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_alloc_ucontext_resp {
    pub dev_caps: __u32,
    pub qp_tab_size: __u32,
    pub bf_reg_size: __u16,
    pub bf_regs_per_page: __u16,
    pub cqe_size: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_alloc_pd_resp {
    pub pdn: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_create_cq {
    pub buf_addr: __aligned_u64,
    pub db_addr: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_create_cq_resp {
    pub cqn: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_resize_cq {
    pub buf_addr: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_create_srq {
    pub buf_addr: __aligned_u64,
    pub db_addr: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_create_srq_resp {
    pub srqn: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_create_qp_rss {
    pub /: *mut *mut __aligned_u64 rx_hash_fields_mask; / Use enum mlx4_ib_rx_hash_fields,
    pub /: *mut *mut __u8 rx_hash_function; / Use enum mlx4_ib_rx_hash_function_flags,
    pub reserved: [__u8; 7],
    pub rx_hash_key: [__u8; 40],
    pub comp_mask: __u32,
    pub reserved1: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_create_qp {
    pub buf_addr: __aligned_u64,
    pub db_addr: __aligned_u64,
    pub log_sq_bb_count: __u8,
    pub log_sq_stride: __u8,
    pub sq_no_prefetch: __u8,
    pub reserved: __u8,
    pub inl_recv_sz: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_create_wq {
    pub buf_addr: __aligned_u64,
    pub db_addr: __aligned_u64,
    pub log_range_size: __u8,
    pub reserved: [__u8; 3],
    pub comp_mask: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_modify_wq {
    pub comp_mask: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_create_rwq_ind_tbl_resp {
    pub response_length: __u32,
    pub reserved: __u32,
}

// RX Hash function flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_ib_rx_hash_function_flags {
    MLX4_IB_RX_HASH_FUNC_TOEPLITZ	= 1 << 0,
}

//
// RX Hash flags, these flags allows to set which incoming packet's field should
// participates in RX Hash. Each flag represent certain packet's field,
// when the flag is set the field that is represented by the flag will
// participate in RX Hash calculation.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_ib_rx_hash_fields {
    MLX4_IB_RX_HASH_SRC_IPV4	= 1 << 0,
    MLX4_IB_RX_HASH_DST_IPV4	= 1 << 1,
    MLX4_IB_RX_HASH_SRC_IPV6	= 1 << 2,
    MLX4_IB_RX_HASH_DST_IPV6	= 1 << 3,
    MLX4_IB_RX_HASH_SRC_PORT_TCP	= 1 << 4,
    MLX4_IB_RX_HASH_DST_PORT_TCP	= 1 << 5,
    MLX4_IB_RX_HASH_SRC_PORT_UDP	= 1 << 6,
    MLX4_IB_RX_HASH_DST_PORT_UDP	= 1 << 7,
    MLX4_IB_RX_HASH_INNER		= 1ULL << 31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_rss_caps {
    pub /: *mut *mut __aligned_u64 rx_hash_fields_mask; / enum mlx4_ib_rx_hash_fields,
    pub /: *mut *mut __u8 rx_hash_function; / enum mlx4_ib_rx_hash_function_flags,
    pub reserved: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum query_device_resp_mask {
    MLX4_IB_QUERY_DEV_RESP_MASK_CORE_CLOCK_OFFSET = 1UL << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_tso_caps {
    pub /: *mut *mut __u32 max_tso; / Maximum tso payload size in bytes,
// Corresponding bit will be set if qp type from
// 'enum ib_qp_type' is supported.
//
    pub supported_qpts: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_uverbs_ex_query_device_resp {
    pub comp_mask: __u32,
    pub response_length: __u32,
    pub hca_core_clock_offset: __aligned_u64,
    pub max_inl_recv_sz: __u32,
    pub reserved: __u32,
    pub rss_caps: mlx4_ib_rss_caps,
    pub tso_caps: mlx4_ib_tso_caps,
}
