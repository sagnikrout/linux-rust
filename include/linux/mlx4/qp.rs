//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx4/qp.h
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
// Copyright (c) 2007 Cisco Systems, Inc.  All rights reserved.
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

pub const MLX4_INVALID_LKEY: c_uint = 0x100;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_qp_optpar {
    MLX4_QP_OPTPAR_ALT_ADDR_PATH		= 1 << 0,
    MLX4_QP_OPTPAR_RRE			= 1 << 1,
    MLX4_QP_OPTPAR_RAE			= 1 << 2,
    MLX4_QP_OPTPAR_RWE			= 1 << 3,
    MLX4_QP_OPTPAR_PKEY_INDEX		= 1 << 4,
    MLX4_QP_OPTPAR_Q_KEY			= 1 << 5,
    MLX4_QP_OPTPAR_RNR_TIMEOUT		= 1 << 6,
    MLX4_QP_OPTPAR_PRIMARY_ADDR_PATH	= 1 << 7,
    MLX4_QP_OPTPAR_SRA_MAX			= 1 << 8,
    MLX4_QP_OPTPAR_RRA_MAX			= 1 << 9,
    MLX4_QP_OPTPAR_PM_STATE			= 1 << 10,
    MLX4_QP_OPTPAR_RETRY_COUNT		= 1 << 12,
    MLX4_QP_OPTPAR_RNR_RETRY		= 1 << 13,
    MLX4_QP_OPTPAR_ACK_TIMEOUT		= 1 << 14,
    MLX4_QP_OPTPAR_SCHED_QUEUE		= 1 << 16,
    MLX4_QP_OPTPAR_COUNTER_INDEX		= 1 << 20,
    MLX4_QP_OPTPAR_VLAN_STRIPPING		= 1 << 21,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_qp_state {
    MLX4_QP_STATE_RST			= 0,
    MLX4_QP_STATE_INIT			= 1,
    MLX4_QP_STATE_RTR			= 2,
    MLX4_QP_STATE_RTS			= 3,
    MLX4_QP_STATE_SQER			= 4,
    MLX4_QP_STATE_SQD			= 5,
    MLX4_QP_STATE_ERR			= 6,
    MLX4_QP_STATE_SQ_DRAINING		= 7,
    MLX4_QP_NUM_STATE
}

// params1
// params2
// offset of mlx4_rss_context within mlx4_qp_context.pri_path
// offset of being RSS indirection QP within mlx4_qp_context.flags
pub const MLX4_EN_RSS_KEY_SIZE: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_rss_context {
    pub base_qpn: __be32,
    pub default_qpn: __be32,
    pub reserved: u16,
    pub hash_fn: u8,
    pub flags: u8,
    pub sizeof(__be32)]: __be32 rss_key[MLX4_EN_RSS_KEY_SIZE /,
    pub base_qpn_udp: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_qp_path {
    pub fl: u8,
    pub vlan_control: u8,
    pub control: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_qp_context {
    pub flags: __be32,
    pub pd: __be32,
    pub mtu_msgmax: u8,
    pub rq_size_stride: u8,
    pub sq_size_stride: u8,
    pub rlkey_roce_mode: u8,
    pub usr_page: __be32,
    pub local_qpn: __be32,
    pub remote_qpn: __be32,
    pub pri_path: mlx4_qp_path,
    pub alt_path: mlx4_qp_path,
    pub params1: __be32,
    pub reserved1: u32,
    pub next_send_psn: __be32,
    pub cqn_send: __be32,
    pub roce_entropy: __be16,
    pub reserved2: [__be16; 3],
    pub last_acked_psn: __be32,
    pub ssn: __be32,
    pub params2: __be32,
    pub rnr_nextrecvpsn: __be32,
    pub xrcd: __be32,
    pub cqn_recv: __be32,
    pub db_rec_addr: __be64,
    pub qkey: __be32,
    pub srqn: __be32,
    pub msn: __be32,
    pub rq_wqe_counter: __be16,
    pub sq_wqe_counter: __be16,
    pub reserved3: u32,
    pub rate_limit_params: __be16,
    pub reserved4: u8,
    pub qos_vport: u8,
    pub param3: __be32,
    pub nummmcpeers_basemkey: __be32,
    pub log_page_size: u8,
    pub reserved5: [u8; 2],
    pub mtt_base_addr_h: u8,
    pub mtt_base_addr_l: __be32,
    pub reserved6: [u32; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_update_qp_context {
    pub qp_mask: __be64,
    pub primary_addr_path_mask: __be64,
    pub secondary_addr_path_mask: __be64,
    pub reserved1: u64,
    pub qp_context: mlx4_qp_context,
    pub reserved2: [u64; 58],
}

// Which firmware version adds support for NEC (NoErrorCompletion) bit

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx4_wqe_qpn_vlan {
    pub vlan_tag: __be16,
    pub ins_vlan: u8,
    pub fence_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_ctrl_seg {
    pub owner_opcode: __be32,
    pub qpn_vlan: mlx4_wqe_qpn_vlan,
//
// High 24 bits are SRC remote buffer; low 8 bits are flags:
// [7]   SO (strong ordering)
// [5]   TCP/UDP checksum
// [4]   IP checksum
// [3:2] C (generate completion queue entry)
// [1]   SE (solicited event)
// [0]   FL (force loopback)
//
    pub srcrb_flags: __be32,
    pub srcrb_flags16: [__be16; 2],
}

//
// imm is immediate data for send/RDMA write w/ immediate;
// also invalidation key for send with invalidate; input
// modifier for WQEs on CCQs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_mlx_seg {
    pub owner: u8,
    pub reserved1: [u8; 2],
    pub opcode: u8,
    pub sched_prio: __be16,
    pub reserved2: u8,
    pub size: u8,
//
// [17]    VL15
// [16]    SLR
// [15:12] static rate
// [11:8]  SL
// [4]     ICRC
// [3:2]   C
// [0]     FL (force loopback)
//
    pub flags: __be32,
    pub rlid: __be16,
    pub reserved3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_datagram_seg {
    pub av: [__be32; 8],
    pub dqpn: __be32,
    pub qkey: __be32,
    pub vlan: __be16,
    pub mac: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_lso_seg {
    pub mss_hdr_size: __be32,
    pub header: [__be32; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_wqe_bind_seg_flags2 {
    MLX4_WQE_BIND_ZERO_BASED = (1 << 30),
    MLX4_WQE_BIND_TYPE_2     = (1 << 31),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_bind_seg {
    pub flags1: __be32,
    pub flags2: __be32,
    pub new_rkey: __be32,
    pub lkey: __be32,
    pub addr: __be64,
    pub length: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_fmr_seg {
    pub flags: __be32,
    pub mem_key: __be32,
    pub buf_list: __be64,
    pub start_addr: __be64,
    pub reg_len: __be64,
    pub offset: __be32,
    pub page_size: __be32,
    pub reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_fmr_ext_seg {
    pub flags: u8,
    pub reserved: u8,
    pub app_mask: __be16,
    pub wire_app_tag: __be16,
    pub mem_app_tag: __be16,
    pub wire_ref_tag_base: __be32,
    pub mem_ref_tag_base: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_local_inval_seg {
    pub reserved1: u64,
    pub mem_key: __be32,
    pub reserved2: u32,
    pub reserved3: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_raddr_seg {
    pub raddr: __be64,
    pub rkey: __be32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_atomic_seg {
    pub swap_add: __be64,
    pub compare: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_masked_atomic_seg {
    pub swap_add: __be64,
    pub compare: __be64,
    pub swap_add_mask: __be64,
    pub compare_mask: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_data_seg {
    pub byte_count: __be32,
    pub lkey: __be32,
    pub addr: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqe_inline_seg {
    pub byte_count: __be32,
    pub data: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_update_qp_attr {
    MLX4_UPDATE_QP_SMAC		= 1 << 0,
    MLX4_UPDATE_QP_VSD		= 1 << 1,
    MLX4_UPDATE_QP_RATE_LIMIT	= 1 << 2,
    MLX4_UPDATE_QP_QOS_VPORT	= 1 << 3,
    MLX4_UPDATE_QP_ETH_SRC_CHECK_MC_LB      = 1 << 4,
    MLX4_UPDATE_QP_SUPPORTED_ATTRS	= (1 << 5) - 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_update_qp_params_flags {
    MLX4_UPDATE_QP_PARAMS_FLAGS_ETH_CHECK_MC_LB     = 1 << 0,
    MLX4_UPDATE_QP_PARAMS_FLAGS_VSD_ENABLE		= 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_update_qp_params {
    pub smac_index: u8,
    pub qos_vport: u8,
    pub flags: u32,
    pub rate_unit: u16,
    pub rate_val: u16,
}

extern "C" {
    pub fn radix_tree_lookup(_arg: &dev->qp_table_tree, 1): qpn & (dev->caps.num_qps -) -> return;
}
extern "C" {
    pub fn mlx4_qp_remove(dev: *mut mlx4_dev, qp: *mut mlx4_qp);
}
extern "C" {
    pub fn mlx4_qp_roce_entropy(dev: *mut mlx4_dev, qpn: u32) -> u16;
}
extern "C" {
    pub fn mlx4_put_qp(qp: *mut mlx4_qp);
}
