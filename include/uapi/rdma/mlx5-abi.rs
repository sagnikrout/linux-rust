//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/mlx5-abi.h
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

// Increment this value if any changes that break userspace ABI
// compatibility are made.
//
pub const MLX5_IB_UVERBS_ABI_VERSION: c_int = 1;
// Make sure that all structs defined in this file remain laid out so
// that they pack the same way on 32-bit and 64-bit architectures (to
// avoid incompatibility between 32-bit userspace and 64-bit kernels).
// In particular do not use pointer types -- pass pointers in __u64
// instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_alloc_ucontext_req {
    pub total_num_bfregs: __u32,
    pub num_low_latency_bfregs: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_lib_caps {
    MLX5_LIB_CAP_4K_UAR	= (__u64)1 << 0,
    MLX5_LIB_CAP_DYN_UAR	= (__u64)1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_alloc_uctx_v2_flags {
    MLX5_IB_ALLOC_UCTX_DEVX	= 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_alloc_ucontext_req_v2 {
    pub total_num_bfregs: __u32,
    pub num_low_latency_bfregs: __u32,
    pub flags: __u32,
    pub comp_mask: __u32,
    pub max_cqe_version: __u8,
    pub reserved0: __u8,
    pub reserved1: __u16,
    pub reserved2: __u32,
    pub lib_caps: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_alloc_ucontext_resp_mask {
    MLX5_IB_ALLOC_UCONTEXT_RESP_MASK_CORE_CLOCK_OFFSET = 1UL << 0,
    MLX5_IB_ALLOC_UCONTEXT_RESP_MASK_DUMP_FILL_MKEY    = 1UL << 1,
    MLX5_IB_ALLOC_UCONTEXT_RESP_MASK_ECE               = 1UL << 2,
    MLX5_IB_ALLOC_UCONTEXT_RESP_MASK_SQD2RTS           = 1UL << 3,
    MLX5_IB_ALLOC_UCONTEXT_RESP_MASK_REAL_TIME_TS	   = 1UL << 4,
    MLX5_IB_ALLOC_UCONTEXT_RESP_MASK_MKEY_UPDATE_TAG   = 1UL << 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_user_cmds_supp_uhw {
    MLX5_USER_CMDS_SUPP_UHW_QUERY_DEVICE = 1 << 0,
    MLX5_USER_CMDS_SUPP_UHW_CREATE_AH    = 1 << 1,
}

// The eth_min_inline response value is set to off-by-one vs the FW
// returned value to allow user-space to deal with older kernels.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_user_inline_mode {
    MLX5_USER_INLINE_MODE_NA,
    MLX5_USER_INLINE_MODE_NONE,
    MLX5_USER_INLINE_MODE_L2,
    MLX5_USER_INLINE_MODE_IP,
    MLX5_USER_INLINE_MODE_TCP_UDP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_alloc_ucontext_resp {
    pub qp_tab_size: __u32,
    pub bf_reg_size: __u32,
    pub tot_bfregs: __u32,
    pub cache_line_size: __u32,
    pub max_sq_desc_sz: __u16,
    pub max_rq_desc_sz: __u16,
    pub max_send_wqebb: __u32,
    pub max_recv_wr: __u32,
    pub max_srq_recv_wr: __u32,
    pub num_ports: __u16,
    pub flow_action_flags: __u16,
    pub comp_mask: __u32,
    pub response_length: __u32,
    pub cqe_version: __u8,
    pub cmds_supp_uhw: __u8,
    pub eth_min_inline: __u8,
    pub clock_info_versions: __u8,
    pub hca_core_clock_offset: __aligned_u64,
    pub log_uar_size: __u32,
    pub num_uars_per_page: __u32,
    pub num_dyn_bfregs: __u32,
    pub dump_fill_mkey: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_alloc_pd_resp {
    pub pdn: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_tso_caps {
    pub /: *mut *mut __u32 max_tso; / Maximum tso payload size in bytes,
// Corresponding bit will be set if qp type from
// 'enum ib_qp_type' is supported, e.g.
// supported_qpts |= 1 << IB_QPT_UD
//
    pub supported_qpts: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_rss_caps {
    pub /: *mut *mut __aligned_u64 rx_hash_fields_mask; / enum mlx5_rx_hash_fields,
    pub /: *mut *mut __u8 rx_hash_function; / enum mlx5_rx_hash_function_flags,
    pub reserved: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_cqe_comp_res_format {
    MLX5_IB_CQE_RES_FORMAT_HASH	= 1 << 0,
    MLX5_IB_CQE_RES_FORMAT_CSUM	= 1 << 1,
    MLX5_IB_CQE_RES_FORMAT_CSUM_STRIDX = 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_cqe_comp_caps {
    pub max_num: __u32,
    pub /: *mut *mut __u32 supported_format; / enum mlx5_ib_cqe_comp_res_format,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_packet_pacing_cap_flags {
    MLX5_IB_PP_SUPPORT_BURST	= 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_packet_pacing_caps {
    pub qp_rate_limit_min: __u32,
    pub /: *mut *mut __u32 qp_rate_limit_max; / In kpbs,
// Corresponding bit will be set if qp type from
// 'enum ib_qp_type' is supported, e.g.
// supported_qpts |= 1 << IB_QPT_RAW_PACKET
//
    pub supported_qpts: __u32,
    pub /: *mut *mut __u8 cap_flags; / enum mlx5_ib_packet_pacing_cap_flags,
    pub reserved: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_mpw_caps {
    MPW_RESERVED		= 1 << 0,
    MLX5_IB_ALLOW_MPW	= 1 << 1,
    MLX5_IB_SUPPORT_EMPW	= 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_sw_parsing_offloads {
    MLX5_IB_SW_PARSING = 1 << 0,
    MLX5_IB_SW_PARSING_CSUM = 1 << 1,
    MLX5_IB_SW_PARSING_LSO = 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_sw_parsing_caps {
    pub /: *mut *mut __u32 sw_parsing_offloads; / enum mlx5_ib_sw_parsing_offloads,
// Corresponding bit will be set if qp type from
// 'enum ib_qp_type' is supported, e.g.
// supported_qpts |= 1 << IB_QPT_RAW_PACKET
//
    pub supported_qpts: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_striding_rq_caps {
    pub min_single_stride_log_num_of_bytes: __u32,
    pub max_single_stride_log_num_of_bytes: __u32,
    pub min_single_wqe_log_num_of_strides: __u32,
    pub max_single_wqe_log_num_of_strides: __u32,
// Corresponding bit will be set if qp type from
// 'enum ib_qp_type' is supported, e.g.
// supported_qpts |= 1 << IB_QPT_RAW_PACKET
//
    pub supported_qpts: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_dci_streams_caps {
    pub max_log_num_concurent: __u8,
    pub max_log_num_errored: __u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_query_dev_resp_flags {
// Support 128B CQE compression
    MLX5_IB_QUERY_DEV_RESP_FLAGS_CQE_128B_COMP = 1 << 0,
    MLX5_IB_QUERY_DEV_RESP_FLAGS_CQE_128B_PAD  = 1 << 1,
    MLX5_IB_QUERY_DEV_RESP_PACKET_BASED_CREDIT_MODE = 1 << 2,
    MLX5_IB_QUERY_DEV_RESP_FLAGS_SCAT2CQE_DCT = 1 << 3,
    MLX5_IB_QUERY_DEV_RESP_FLAGS_OOO_DP = 1 << 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_tunnel_offloads {
    MLX5_IB_TUNNELED_OFFLOADS_VXLAN  = 1 << 0,
    MLX5_IB_TUNNELED_OFFLOADS_GRE    = 1 << 1,
    MLX5_IB_TUNNELED_OFFLOADS_GENEVE = 1 << 2,
    MLX5_IB_TUNNELED_OFFLOADS_MPLS_GRE = 1 << 3,
    MLX5_IB_TUNNELED_OFFLOADS_MPLS_UDP = 1 << 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_query_device_resp {
    pub comp_mask: __u32,
    pub response_length: __u32,
    pub tso_caps: mlx5_ib_tso_caps,
    pub rss_caps: mlx5_ib_rss_caps,
    pub cqe_comp_caps: mlx5_ib_cqe_comp_caps,
    pub packet_pacing_caps: mlx5_packet_pacing_caps,
    pub mlx5_ib_support_multi_pkt_send_wqes: __u32,
    pub /: *mut *mut __u32 flags; / Use enum mlx5_ib_query_dev_resp_flags,
    pub sw_parsing_caps: mlx5_ib_sw_parsing_caps,
    pub striding_rq_caps: mlx5_ib_striding_rq_caps,
    pub /: *mut *mut __u32 tunnel_offloads_caps; / enum mlx5_ib_tunnel_offloads,
    pub dci_streams_caps: mlx5_ib_dci_streams_caps,
    pub reserved: __u16,
    pub reg_c0: mlx5_ib_uapi_reg,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_create_cq_flags {
    MLX5_IB_CREATE_CQ_FLAGS_CQE_128B_PAD	= 1 << 0,
    MLX5_IB_CREATE_CQ_FLAGS_UAR_PAGE_INDEX  = 1 << 1,
    MLX5_IB_CREATE_CQ_FLAGS_REAL_TIME_TS	= 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_cq {
    pub buf_addr: __aligned_u64,
    pub db_addr: __aligned_u64,
    pub cqe_size: __u32,
    pub cqe_comp_en: __u8,
    pub cqe_comp_res_format: __u8,
    pub flags: __u16,
    pub uar_page_index: __u16,
    pub reserved0: __u16,
    pub reserved1: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_cq_resp {
    pub cqn: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_resize_cq {
    pub buf_addr: __aligned_u64,
    pub cqe_size: __u16,
    pub reserved0: __u16,
    pub reserved1: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_srq {
    pub buf_addr: __aligned_u64,
    pub db_addr: __aligned_u64,
    pub flags: __u32,
    pub /: *mut *mut __u32 reserved0; / explicit padding (optional on i386),
    pub uidx: __u32,
    pub reserved1: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_srq_resp {
    pub srqn: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_qp_dci_streams {
    pub log_num_concurent: __u8,
    pub log_num_errored: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_qp {
    pub buf_addr: __aligned_u64,
    pub db_addr: __aligned_u64,
    pub sq_wqe_count: __u32,
    pub rq_wqe_count: __u32,
    pub rq_wqe_shift: __u32,
    pub flags: __u32,
    pub uidx: __u32,
    pub bfreg_index: __u32,
    pub sq_buf_addr: __aligned_u64,
    pub access_key: __aligned_u64,
}

// RX Hash function flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_rx_hash_function_flags {
    MLX5_RX_HASH_FUNC_TOEPLITZ	= 1 << 0,
}

//
// RX Hash flags, these flags allows to set which incoming packet's field should
// participates in RX Hash. Each flag represent certain packet's field,
// when the flag is set the field that is represented by the flag will
// participate in RX Hash calculation.
// Note: *IPV4 and *IPV6 flags can't be enabled together on the same QP
// and *TCP and *UDP flags can't be enabled together on the same QP.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_rx_hash_fields {
    MLX5_RX_HASH_SRC_IPV4	= 1 << 0,
    MLX5_RX_HASH_DST_IPV4	= 1 << 1,
    MLX5_RX_HASH_SRC_IPV6	= 1 << 2,
    MLX5_RX_HASH_DST_IPV6	= 1 << 3,
    MLX5_RX_HASH_SRC_PORT_TCP	= 1 << 4,
    MLX5_RX_HASH_DST_PORT_TCP	= 1 << 5,
    MLX5_RX_HASH_SRC_PORT_UDP	= 1 << 6,
    MLX5_RX_HASH_DST_PORT_UDP	= 1 << 7,
    MLX5_RX_HASH_IPSEC_SPI		= 1 << 8,
// Save bits for future fields
    MLX5_RX_HASH_INNER		= (1UL << 31),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_qp_rss {
    pub /: *mut *mut __aligned_u64 rx_hash_fields_mask; / enum mlx5_rx_hash_fields,
    pub /: *mut *mut __u8 rx_hash_function; / enum mlx5_rx_hash_function_flags,
    pub /: *mut *mut __u8 rx_key_len; / valid only for Toeplitz,
    pub reserved: [__u8; 6],
    pub /: *mut *mut __u8 rx_hash_key[128]; / valid only for Toeplitz,
    pub comp_mask: __u32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_create_qp_resp_mask {
    MLX5_IB_CREATE_QP_RESP_MASK_TIRN = 1UL << 0,
    MLX5_IB_CREATE_QP_RESP_MASK_TISN = 1UL << 1,
    MLX5_IB_CREATE_QP_RESP_MASK_RQN  = 1UL << 2,
    MLX5_IB_CREATE_QP_RESP_MASK_SQN  = 1UL << 3,
    MLX5_IB_CREATE_QP_RESP_MASK_TIR_ICM_ADDR  = 1UL << 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_qp_resp {
    pub bfreg_index: __u32,
    pub ece_options: __u32,
    pub comp_mask: __u32,
    pub tirn: __u32,
    pub tisn: __u32,
    pub rqn: __u32,
    pub sqn: __u32,
    pub reserved1: __u32,
    pub tir_icm_addr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_alloc_mw {
    pub comp_mask: __u32,
    pub num_klms: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_create_wq_mask {
    MLX5_IB_CREATE_WQ_STRIDING_RQ	= (1 << 0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_wq {
    pub buf_addr: __aligned_u64,
    pub db_addr: __aligned_u64,
    pub rq_wqe_count: __u32,
    pub rq_wqe_shift: __u32,
    pub user_index: __u32,
    pub flags: __u32,
    pub comp_mask: __u32,
    pub single_stride_log_num_of_bytes: __u32,
    pub single_wqe_log_num_of_strides: __u32,
    pub two_byte_shift_en: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_ah_resp {
    pub response_length: __u32,
    pub dmac: [__u8; ETH_ALEN],
    pub reserved: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_burst_info {
    pub max_burst_sz: __u32,
    pub typical_pkt_sz: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_modify_qp_mask {
    MLX5_IB_MODIFY_QP_OOO_DP = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_modify_qp {
    pub comp_mask: __u32,
    pub burst_info: mlx5_ib_burst_info,
    pub ece_options: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_modify_qp_resp {
    pub response_length: __u32,
    pub dctn: __u32,
    pub ece_options: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_wq_resp {
    pub response_length: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_rwq_ind_tbl_resp {
    pub response_length: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_modify_wq {
    pub comp_mask: __u32,
    pub reserved: __u32,
}

//
// deprecated, see struct ib_uverbs_clock_info from ib_user_verbs.h
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_mmap_cmd {
    MLX5_IB_MMAP_REGULAR_PAGE               = 0,
    MLX5_IB_MMAP_GET_CONTIGUOUS_PAGES       = 1,
    MLX5_IB_MMAP_WC_PAGE                    = 2,
    MLX5_IB_MMAP_NC_PAGE                    = 3,
// 5 is chosen in order to be compatible with old versions of libmlx5
    MLX5_IB_MMAP_CORE_CLOCK                 = 5,
    MLX5_IB_MMAP_ALLOC_WC                   = 6,
    MLX5_IB_MMAP_CLOCK_INFO                 = 7,
    MLX5_IB_MMAP_DEVICE_MEM                 = 8,
}

// Bit indexes for the mlx5_alloc_ucontext_resp.clock_info_versions bitmap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_flow_counters_desc {
    pub description: __u32,
    pub index: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_flow_counters_data {
    pub counters_data): *mut *mut RDMA_UAPI_PTR(struct mlx5_ib_flow_counters_desc ,,
    pub ncounters: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_create_flow {
    pub ncounters_data: __u32,
    pub reserved: __u32,
//
// Following are counters data based on ncounters_data, each
// entry in the data[] should match a corresponding counter object
// that was pointed by a counters spec upon the flow creation
//
    pub data: [mlx5_ib_flow_counters_data; ],
}
