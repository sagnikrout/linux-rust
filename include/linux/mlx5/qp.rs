//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/qp.h
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

// UMR (3 WQE_BB's) + SIG (3 WQE_BB's) + PSV (mem) + PSV (wire)

pub const MLX5_DIF_SIZE: c_int = 8;
pub const MLX5_STRIDE_BLOCK_OP: c_uint = 0x400;
pub const MLX5_CPY_GRD_MASK: c_uint = 0xc0;
pub const MLX5_CPY_APP_MASK: c_uint = 0x30;
pub const MLX5_CPY_REF_MASK: c_uint = 0x0f;

pub const MLX5_BSF_APPTAG_ESCAPE: c_uint = 0x1;
pub const MLX5_BSF_APPREF_ESCAPE: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_qp_optpar {
    MLX5_QP_OPTPAR_ALT_ADDR_PATH		= 1 << 0,
    MLX5_QP_OPTPAR_RRE			= 1 << 1,
    MLX5_QP_OPTPAR_RAE			= 1 << 2,
    MLX5_QP_OPTPAR_RWE			= 1 << 3,
    MLX5_QP_OPTPAR_PKEY_INDEX		= 1 << 4,
    MLX5_QP_OPTPAR_Q_KEY			= 1 << 5,
    MLX5_QP_OPTPAR_RNR_TIMEOUT		= 1 << 6,
    MLX5_QP_OPTPAR_PRIMARY_ADDR_PATH	= 1 << 7,
    MLX5_QP_OPTPAR_SRA_MAX			= 1 << 8,
    MLX5_QP_OPTPAR_RRA_MAX			= 1 << 9,
    MLX5_QP_OPTPAR_PM_STATE			= 1 << 10,
    MLX5_QP_OPTPAR_RETRY_COUNT		= 1 << 12,
    MLX5_QP_OPTPAR_RNR_RETRY		= 1 << 13,
    MLX5_QP_OPTPAR_ACK_TIMEOUT		= 1 << 14,
    MLX5_QP_OPTPAR_LAG_TX_AFF		= 1 << 15,
    MLX5_QP_OPTPAR_PRI_PORT			= 1 << 16,
    MLX5_QP_OPTPAR_SRQN			= 1 << 18,
    MLX5_QP_OPTPAR_CQN_RCV			= 1 << 19,
    MLX5_QP_OPTPAR_DC_HS			= 1 << 20,
    MLX5_QP_OPTPAR_DC_KEY			= 1 << 21,
    MLX5_QP_OPTPAR_PP_INDEX			= 1 << 22,
    MLX5_QP_OPTPAR_COUNTER_SET_ID		= 1 << 25,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_qp_state {
    MLX5_QP_STATE_RST			= 0,
    MLX5_QP_STATE_INIT			= 1,
    MLX5_QP_STATE_RTR			= 2,
    MLX5_QP_STATE_RTS			= 3,
    MLX5_QP_STATE_SQER			= 4,
    MLX5_QP_STATE_SQD			= 5,
    MLX5_QP_STATE_ERR			= 6,
    MLX5_QP_STATE_SQ_DRAINING		= 7,
    MLX5_QP_STATE_SUSPENDED			= 9,
    MLX5_QP_NUM_STATE,
    MLX5_QP_STATE,
    MLX5_QP_STATE_BAD,
}

// TODO REM
// params1
// params2

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_fmr_seg {
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
pub struct mlx5_wqe_ctrl_seg {
    pub opmod_idx_opcode: __be32,
    pub qpn_ds: __be32,
    pub signature: u8,
    pub rsvd: [u8; 2],
    pub fm_ce_se: u8,
    pub general_id: __be32,
    pub imm: __be32,
    pub umr_mkey: __be32,
    pub tis_tir_num: __be32,
}

pub const MLX5_WQE_CTRL_DS_MASK: c_uint = 0x3f;
pub const MLX5_WQE_CTRL_QPN_MASK: c_uint = 0xffffff00;
pub const MLX5_WQE_CTRL_QPN_SHIFT: c_int = 8;
pub const MLX5_WQE_DS_UNITS: c_int = 16;
pub const MLX5_WQE_CTRL_OPCODE_MASK: c_uint = 0xff;
pub const MLX5_WQE_CTRL_WQE_INDEX_MASK: c_uint = 0x00ffff00;
pub const MLX5_WQE_CTRL_WQE_INDEX_SHIFT: c_int = 8;
// Metadata bits 0-7 are used by timestamping
// Base shift for metadata bits used by IPsec and MACsec
pub const MLX5_ETH_WQE_FT_META_SHIFT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_eth_seg {
    pub swp_outer_l4_offset: u8,
    pub swp_outer_l3_offset: u8,
    pub swp_inner_l4_offset: u8,
    pub swp_inner_l3_offset: u8,
    pub cs_flags: u8,
    pub swp_flags: u8,
    pub mss: __be16,
    pub flow_table_metadata: __be32,
    pub sz: __be16,
    pub start: [u8; 2],
    pub data): DECLARE_FLEX_ARRAY(u8,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_xrc_seg {
    pub xrc_srqn: __be32,
    pub rsvd: [u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_masked_atomic_seg {
    pub swap_add: __be64,
    pub compare: __be64,
    pub swap_add_mask: __be64,
    pub compare_mask: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_base_av {
    pub qkey: __be32,
    pub reserved: __be32,
    pub qkey: },
    pub dc_key: __be64,
    pub key: },
    pub dqp_dct: __be32,
    pub stat_rate_sl: u8,
    pub fl_mlid: u8,
    pub rlid: __be16,
    pub udp_sport: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_av {
    pub qkey: __be32,
    pub reserved: __be32,
    pub qkey: },
    pub dc_key: __be64,
    pub key: },
    pub dqp_dct: __be32,
    pub stat_rate_sl: u8,
    pub fl_mlid: u8,
    pub rlid: __be16,
    pub udp_sport: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_ah {
    pub ibah: ib_ah,
    pub av: mlx5_av,
    pub xmit_port: u8,
}

extern "C" {
    pub fn container_of(_arg: ibah, mlx5_ib_ah: struct, _arg: ibah) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_datagram_seg {
    pub av: mlx5_av,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_raddr_seg {
    pub raddr: __be64,
    pub rkey: __be32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_atomic_seg {
    pub swap_add: __be64,
    pub compare: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_data_seg {
    pub byte_count: __be32,
    pub lkey: __be32,
    pub addr: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_umr_ctrl_seg {
    pub flags: u8,
    pub rsvd0: [u8; 3],
    pub xlt_octowords: __be16,
    pub xlt_offset: __be16,
    pub bsf_octowords: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_seg_set_psv {
    pub psv_num: __be32,
    pub syndrome: __be16,
    pub status: __be16,
    pub transient_sig: __be32,
    pub ref_tag: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_seg_get_psv {
    pub rsvd: [u8; 19],
    pub num_psv: u8,
    pub l_key: __be32,
    pub va: __be64,
    pub psv_index: [__be32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_seg_check_psv {
    pub rsvd0: [u8; 2],
    pub err_coalescing_op: __be16,
    pub rsvd1: [u8; 2],
    pub xport_err_op: __be16,
    pub rsvd2: [u8; 2],
    pub xport_err_mask: __be16,
    pub rsvd3: [u8; 7],
    pub num_psv: u8,
    pub l_key: __be32,
    pub va: __be64,
    pub psv_index: [__be32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_rwqe_sig {
    pub rsvd0: [u8; 4],
    pub signature: u8,
    pub rsvd1: [u8; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_signature_seg {
    pub rsvd0: [u8; 4],
    pub signature: u8,
    pub rsvd1: [u8; 11],
}

pub const MLX5_WQE_INLINE_SEG_BYTE_COUNT_MASK: c_uint = 0x3ff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_inline_seg {
    pub byte_count: __be32,
    pub data: [__be32; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_sig_type {
    MLX5_DIF_CRC = 0x1,
    MLX5_DIF_IPCS = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_bsf_inl {
    pub vld_refresh: __be16,
    pub dif_apptag: __be16,
    pub dif_reftag: __be32,
    pub sig_type: u8,
    pub rp_inv_seed: u8,
    pub rsvd: [u8; 3],
    pub dif_inc_ref_guard_check: u8,
    pub dif_app_bitmask_check: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_bsf {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_bsf_basic {
    pub bsf_size_sbs: u8,
    pub check_byte_mask: u8,
    pub copy_byte_mask: u8,
    pub bs_selector: u8,
    pub rsvd_wflags: u8,
    pub wire: },
    pub bs_selector: u8,
    pub rsvd_mflags: u8,
    pub mem: },
    pub raw_data_size: __be32,
    pub w_bfs_psv: __be32,
    pub m_bfs_psv: __be32,
    pub basic: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_bsf_ext {
    pub t_init_gen_pro_size: __be32,
    pub rsvd_epi_size: __be32,
    pub w_tfs_psv: __be32,
    pub m_tfs_psv: __be32,
    pub ext: },
    pub w_inl: mlx5_bsf_inl,
    pub m_inl: mlx5_bsf_inl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_mtt {
    pub ptag: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_klm {
    pub bcount: __be32,
    pub key: __be32,
    pub va: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ksm {
    pub reserved: __be32,
    pub key: __be32,
    pub va: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_stride_block_entry {
    pub stride: __be16,
    pub bcount: __be16,
    pub key: __be32,
    pub va: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_stride_block_ctrl_seg {
    pub bcount_per_cycle: __be32,
    pub op: __be32,
    pub repeat_count: __be32,
    pub rsvd: u16,
    pub num_entries: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_flow_update_ctrl_seg {
    pub flow_idx_update: __be32,
    pub dest_handle: __be32,
    pub reserved0: [u8; 40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_header_modify_argument_update_seg {
    pub argument_list: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_core_qp {
    pub /: *mut *mut mlx5_core_rsc_common common; / must be first,
    pub int): *mut *mut *mut void (event) (struct mlx5_core_qp ,,
    pub qpn: c_int,
    pub dbg: *mut mlx5_rsc_debug,
    pub pid: c_int,
    pub uid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_core_dct {
    pub mqp: mlx5_core_qp,
    pub drained: completion,
}

extern "C" {
    pub fn mlx5_debug_qp_add(dev: *mut mlx5_core_dev, qp: *mut mlx5_core_qp) -> c_int;
}
extern "C" {
    pub fn mlx5_debug_qp_remove(dev: *mut mlx5_core_dev, qp: *mut mlx5_core_qp);
}
