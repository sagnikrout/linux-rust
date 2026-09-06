//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/ib_user_verbs.h
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
// Copyright (c) 2005 Topspin Communications.  All rights reserved.
// Copyright (c) 2005, 2006 Cisco Systems.  All rights reserved.
// Copyright (c) 2005 PathScale, Inc.  All rights reserved.
// Copyright (c) 2006 Mellanox Technologies.  All rights reserved.
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
pub const IB_USER_VERBS_ABI_VERSION: c_int = 6;
pub const IB_USER_VERBS_CMD_THRESHOLD: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_uverbs_write_cmds {
    IB_USER_VERBS_CMD_GET_CONTEXT,
    IB_USER_VERBS_CMD_QUERY_DEVICE,
    IB_USER_VERBS_CMD_QUERY_PORT,
    IB_USER_VERBS_CMD_ALLOC_PD,
    IB_USER_VERBS_CMD_DEALLOC_PD,
    IB_USER_VERBS_CMD_CREATE_AH,
    IB_USER_VERBS_CMD_MODIFY_AH,
    IB_USER_VERBS_CMD_QUERY_AH,
    IB_USER_VERBS_CMD_DESTROY_AH,
    IB_USER_VERBS_CMD_REG_MR,
    IB_USER_VERBS_CMD_REG_SMR,
    IB_USER_VERBS_CMD_REREG_MR,
    IB_USER_VERBS_CMD_QUERY_MR,
    IB_USER_VERBS_CMD_DEREG_MR,
    IB_USER_VERBS_CMD_ALLOC_MW,
    IB_USER_VERBS_CMD_BIND_MW,
    IB_USER_VERBS_CMD_DEALLOC_MW,
    IB_USER_VERBS_CMD_CREATE_COMP_CHANNEL,
    IB_USER_VERBS_CMD_CREATE_CQ,
    IB_USER_VERBS_CMD_RESIZE_CQ,
    IB_USER_VERBS_CMD_DESTROY_CQ,
    IB_USER_VERBS_CMD_POLL_CQ,
    IB_USER_VERBS_CMD_PEEK_CQ,
    IB_USER_VERBS_CMD_REQ_NOTIFY_CQ,
    IB_USER_VERBS_CMD_CREATE_QP,
    IB_USER_VERBS_CMD_QUERY_QP,
    IB_USER_VERBS_CMD_MODIFY_QP,
    IB_USER_VERBS_CMD_DESTROY_QP,
    IB_USER_VERBS_CMD_POST_SEND,
    IB_USER_VERBS_CMD_POST_RECV,
    IB_USER_VERBS_CMD_ATTACH_MCAST,
    IB_USER_VERBS_CMD_DETACH_MCAST,
    IB_USER_VERBS_CMD_CREATE_SRQ,
    IB_USER_VERBS_CMD_MODIFY_SRQ,
    IB_USER_VERBS_CMD_QUERY_SRQ,
    IB_USER_VERBS_CMD_DESTROY_SRQ,
    IB_USER_VERBS_CMD_POST_SRQ_RECV,
    IB_USER_VERBS_CMD_OPEN_XRCD,
    IB_USER_VERBS_CMD_CLOSE_XRCD,
    IB_USER_VERBS_CMD_CREATE_XSRQ,
    IB_USER_VERBS_CMD_OPEN_QP,
}

// see IBA A19.4.1.1 Placement Types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_placement_type {
    IB_FLUSH_GLOBAL = 1U << 0,
    IB_FLUSH_PERSISTENT = 1U << 1,
}

// see IBA A19.4.1.2 Selectivity Level
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_selectivity_level {
    IB_FLUSH_RANGE = 0,
    IB_FLUSH_MR,
}

//
// Make sure that all structs defined in this file remain laid out so
// that they pack the same way on 32-bit and 64-bit architectures (to
// avoid incompatibility between 32-bit userspace and 64-bit kernels).
// Specifically:
// - Do not use pointer types -- pass pointers in __u64 instead.
// - Make sure that any structure larger than 4 bytes is padded to a
// multiple of 8 bytes.  Otherwise the structure size will be
// different between 32-bit and 64-bit architectures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_async_event_desc {
    pub element: __aligned_u64,
    pub /: *mut *mut __u32 event_type; / enum ib_event_type,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_comp_event_desc {
    pub cq_handle: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_cq_moderation_caps {
    pub max_cq_moderation_count: __u16,
    pub max_cq_moderation_period: __u16,
    pub reserved: __u32,
}

//
// All commands from userspace should start with a __u32 command field
// followed by __u16 in_words and out_words fields (which give the
// length of the command block and response buffer if any in 32-bit
// words).  The kernel driver will read these fields first and read
// the rest of the command struct based on these value.
//
pub const IB_USER_VERBS_CMD_COMMAND_MASK: c_uint = 0xff;
pub const IB_USER_VERBS_CMD_FLAG_EXTENDED: c_uint = 0x80000000u;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_cmd_hdr {
    pub command: __u32,
    pub in_words: __u16,
    pub out_words: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_cmd_hdr {
    pub response: __aligned_u64,
    pub provider_in_words: __u16,
    pub provider_out_words: __u16,
    pub cmd_hdr_reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_get_context {
    pub response: __aligned_u64,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_get_context_resp {
    pub async_fd: __u32,
    pub num_comp_vectors: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_query_device {
    pub response: __aligned_u64,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_query_device_resp {
    pub fw_ver: __aligned_u64,
    pub node_guid: __be64,
    pub sys_image_guid: __be64,
    pub max_mr_size: __aligned_u64,
    pub page_size_cap: __aligned_u64,
    pub vendor_id: __u32,
    pub vendor_part_id: __u32,
    pub hw_ver: __u32,
    pub max_qp: __u32,
    pub max_qp_wr: __u32,
    pub device_cap_flags: __u32,
    pub max_sge: __u32,
    pub max_sge_rd: __u32,
    pub max_cq: __u32,
    pub max_cqe: __u32,
    pub max_mr: __u32,
    pub max_pd: __u32,
    pub max_qp_rd_atom: __u32,
    pub max_ee_rd_atom: __u32,
    pub max_res_rd_atom: __u32,
    pub max_qp_init_rd_atom: __u32,
    pub max_ee_init_rd_atom: __u32,
    pub atomic_cap: __u32,
    pub max_ee: __u32,
    pub max_rdd: __u32,
    pub max_mw: __u32,
    pub max_raw_ipv6_qp: __u32,
    pub max_raw_ethy_qp: __u32,
    pub max_mcast_grp: __u32,
    pub max_mcast_qp_attach: __u32,
    pub max_total_mcast_qp_attach: __u32,
    pub max_ah: __u32,
    pub max_fmr: __u32,
    pub max_map_per_fmr: __u32,
    pub max_srq: __u32,
    pub max_srq_wr: __u32,
    pub max_srq_sge: __u32,
    pub max_pkeys: __u16,
    pub local_ca_ack_delay: __u8,
    pub phys_port_cnt: __u8,
    pub reserved: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_query_device {
    pub comp_mask: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_uverbs_odp_general_cap_bits {
    IB_UVERBS_ODP_SUPPORT          = 1 << 0,
    IB_UVERBS_ODP_SUPPORT_IMPLICIT = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_uverbs_odp_transport_cap_bits {
    IB_UVERBS_ODP_SUPPORT_SEND     = 1 << 0,
    IB_UVERBS_ODP_SUPPORT_RECV     = 1 << 1,
    IB_UVERBS_ODP_SUPPORT_WRITE    = 1 << 2,
    IB_UVERBS_ODP_SUPPORT_READ     = 1 << 3,
    IB_UVERBS_ODP_SUPPORT_ATOMIC   = 1 << 4,
    IB_UVERBS_ODP_SUPPORT_SRQ_RECV = 1 << 5,
    IB_UVERBS_ODP_SUPPORT_FLUSH    = 1 << 6,
    IB_UVERBS_ODP_SUPPORT_ATOMIC_WRITE     = 1 << 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_odp_caps {
    pub general_caps: __aligned_u64,
    pub rc_odp_caps: __u32,
    pub uc_odp_caps: __u32,
    pub ud_odp_caps: __u32,
    pub per_transport_caps: },
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_rss_caps {
// Corresponding bit will be set if qp type from
// 'enum ib_qp_type' is supported, e.g.
// supported_qpts |= 1 << IB_QPT_UD
//
    pub supported_qpts: __u32,
    pub max_rwq_indirection_tables: __u32,
    pub max_rwq_indirection_table_size: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_tm_caps {
// Max size of rendezvous request message
    pub max_rndv_hdr_size: __u32,
// Max number of entries in tag matching list
    pub max_num_tags: __u32,
// TM flags
    pub flags: __u32,
// Max number of outstanding list operations
    pub max_ops: __u32,
// Max number of SGE in tag matching entry
    pub max_sge: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_query_device_resp {
    pub base: ib_uverbs_query_device_resp,
    pub comp_mask: __u32,
    pub response_length: __u32,
    pub odp_caps: ib_uverbs_odp_caps,
    pub timestamp_mask: __aligned_u64,
    pub /: *mut *mut __aligned_u64 hca_core_clock; / in KHZ,
    pub device_cap_flags_ex: __aligned_u64,
    pub rss_caps: ib_uverbs_rss_caps,
    pub max_wq_type_rq: __u32,
    pub raw_packet_caps: __u32,
    pub tm_caps: ib_uverbs_tm_caps,
    pub cq_moderation_caps: ib_uverbs_cq_moderation_caps,
    pub max_dm_size: __aligned_u64,
    pub xrc_odp_caps: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_query_port {
    pub response: __aligned_u64,
    pub port_num: __u8,
    pub reserved: [__u8; 7],
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_query_port_resp {
    pub /: *mut *mut __u32 port_cap_flags; / see ib_uverbs_query_port_cap_flags,
    pub max_msg_sz: __u32,
    pub bad_pkey_cntr: __u32,
    pub qkey_viol_cntr: __u32,
    pub gid_tbl_len: __u32,
    pub pkey_tbl_len: __u16,
    pub lid: __u16,
    pub sm_lid: __u16,
    pub state: __u8,
    pub max_mtu: __u8,
    pub active_mtu: __u8,
    pub lmc: __u8,
    pub max_vl_num: __u8,
    pub sm_sl: __u8,
    pub subnet_timeout: __u8,
    pub init_type_reply: __u8,
    pub active_width: __u8,
    pub active_speed: __u8,
    pub phys_state: __u8,
    pub link_layer: __u8,
    pub /: *mut *mut __u8 flags; / see ib_uverbs_query_port_flags,
    pub reserved: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_alloc_pd {
    pub response: __aligned_u64,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_alloc_pd_resp {
    pub pd_handle: __u32,
    pub driver_data: [__u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_dealloc_pd {
    pub pd_handle: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_open_xrcd {
    pub response: __aligned_u64,
    pub fd: __u32,
    pub oflags: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_open_xrcd_resp {
    pub xrcd_handle: __u32,
    pub driver_data: [__u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_close_xrcd {
    pub xrcd_handle: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_reg_mr {
    pub response: __aligned_u64,
    pub start: __aligned_u64,
    pub length: __aligned_u64,
    pub hca_va: __aligned_u64,
    pub pd_handle: __u32,
    pub access_flags: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_reg_mr_resp {
    pub mr_handle: __u32,
    pub lkey: __u32,
    pub rkey: __u32,
    pub driver_data: [__u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_rereg_mr {
    pub response: __aligned_u64,
    pub mr_handle: __u32,
    pub flags: __u32,
    pub start: __aligned_u64,
    pub length: __aligned_u64,
    pub hca_va: __aligned_u64,
    pub pd_handle: __u32,
    pub access_flags: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_rereg_mr_resp {
    pub lkey: __u32,
    pub rkey: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_dereg_mr {
    pub mr_handle: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_alloc_mw {
    pub response: __aligned_u64,
    pub pd_handle: __u32,
    pub mw_type: __u8,
    pub reserved: [__u8; 3],
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_alloc_mw_resp {
    pub mw_handle: __u32,
    pub rkey: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_dealloc_mw {
    pub mw_handle: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_comp_channel {
    pub response: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_comp_channel_resp {
    pub fd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_cq {
    pub response: __aligned_u64,
    pub user_handle: __aligned_u64,
    pub cqe: __u32,
    pub comp_vector: __u32,
    pub comp_channel: __s32,
    pub reserved: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_uverbs_ex_create_cq_flags {
    IB_UVERBS_CQ_FLAGS_TIMESTAMP_COMPLETION = 1 << 0,
    IB_UVERBS_CQ_FLAGS_IGNORE_OVERRUN = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_create_cq {
    pub user_handle: __aligned_u64,
    pub cqe: __u32,
    pub comp_vector: __u32,
    pub comp_channel: __s32,
    pub comp_mask: __u32,
    pub /: *mut *mut __u32 flags; / bitmask of ib_uverbs_ex_create_cq_flags,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_cq_resp {
    pub cq_handle: __u32,
    pub cqe: __u32,
    pub driver_data: [__aligned_u64; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_create_cq_resp {
    pub base: ib_uverbs_create_cq_resp,
    pub comp_mask: __u32,
    pub response_length: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_resize_cq {
    pub response: __aligned_u64,
    pub cq_handle: __u32,
    pub cqe: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_resize_cq_resp {
    pub cqe: __u32,
    pub reserved: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_poll_cq {
    pub response: __aligned_u64,
    pub cq_handle: __u32,
    pub ne: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_uverbs_wc_opcode {
    IB_UVERBS_WC_SEND = 0,
    IB_UVERBS_WC_RDMA_WRITE = 1,
    IB_UVERBS_WC_RDMA_READ = 2,
    IB_UVERBS_WC_COMP_SWAP = 3,
    IB_UVERBS_WC_FETCH_ADD = 4,
    IB_UVERBS_WC_BIND_MW = 5,
    IB_UVERBS_WC_LOCAL_INV = 6,
    IB_UVERBS_WC_TSO = 7,
    IB_UVERBS_WC_FLUSH = 8,
    IB_UVERBS_WC_ATOMIC_WRITE = 9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_wc {
    pub wr_id: __aligned_u64,
    pub status: __u32,
    pub opcode: __u32,
    pub vendor_err: __u32,
    pub byte_len: __u32,
    pub imm_data: __be32,
    pub invalidate_rkey: __u32,
    pub ex: },
    pub qp_num: __u32,
    pub src_qp: __u32,
    pub wc_flags: __u32,
    pub pkey_index: __u16,
    pub slid: __u16,
    pub sl: __u8,
    pub dlid_path_bits: __u8,
    pub port_num: __u8,
    pub reserved: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_poll_cq_resp {
    pub count: __u32,
    pub reserved: __u32,
    pub wc: [ib_uverbs_wc; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_req_notify_cq {
    pub cq_handle: __u32,
    pub solicited_only: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_destroy_cq {
    pub response: __aligned_u64,
    pub cq_handle: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_destroy_cq_resp {
    pub comp_events_reported: __u32,
    pub async_events_reported: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_global_route {
    pub dgid: [__u8; 16],
    pub flow_label: __u32,
    pub sgid_index: __u8,
    pub hop_limit: __u8,
    pub traffic_class: __u8,
    pub reserved: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ah_attr {
    pub grh: ib_uverbs_global_route,
    pub dlid: __u16,
    pub sl: __u8,
    pub src_path_bits: __u8,
    pub static_rate: __u8,
    pub is_global: __u8,
    pub port_num: __u8,
    pub reserved: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_qp_attr {
    pub qp_attr_mask: __u32,
    pub qp_state: __u32,
    pub cur_qp_state: __u32,
    pub path_mtu: __u32,
    pub path_mig_state: __u32,
    pub qkey: __u32,
    pub rq_psn: __u32,
    pub sq_psn: __u32,
    pub dest_qp_num: __u32,
    pub qp_access_flags: __u32,
    pub ah_attr: ib_uverbs_ah_attr,
    pub alt_ah_attr: ib_uverbs_ah_attr,
// ib_qp_cap
    pub max_send_wr: __u32,
    pub max_recv_wr: __u32,
    pub max_send_sge: __u32,
    pub max_recv_sge: __u32,
    pub max_inline_data: __u32,
    pub pkey_index: __u16,
    pub alt_pkey_index: __u16,
    pub en_sqd_async_notify: __u8,
    pub sq_draining: __u8,
    pub max_rd_atomic: __u8,
    pub max_dest_rd_atomic: __u8,
    pub min_rnr_timer: __u8,
    pub port_num: __u8,
    pub timeout: __u8,
    pub retry_cnt: __u8,
    pub rnr_retry: __u8,
    pub alt_port_num: __u8,
    pub alt_timeout: __u8,
    pub reserved: [__u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_qp {
    pub response: __aligned_u64,
    pub user_handle: __aligned_u64,
    pub pd_handle: __u32,
    pub send_cq_handle: __u32,
    pub recv_cq_handle: __u32,
    pub srq_handle: __u32,
    pub max_send_wr: __u32,
    pub max_recv_wr: __u32,
    pub max_send_sge: __u32,
    pub max_recv_sge: __u32,
    pub max_inline_data: __u32,
    pub sq_sig_all: __u8,
    pub qp_type: __u8,
    pub is_srq: __u8,
    pub reserved: __u8,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_uverbs_create_qp_mask {
    IB_UVERBS_CREATE_QP_MASK_IND_TABLE = 1UL << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_create_qp {
    pub user_handle: __aligned_u64,
    pub pd_handle: __u32,
    pub send_cq_handle: __u32,
    pub recv_cq_handle: __u32,
    pub srq_handle: __u32,
    pub max_send_wr: __u32,
    pub max_recv_wr: __u32,
    pub max_send_sge: __u32,
    pub max_recv_sge: __u32,
    pub max_inline_data: __u32,
    pub sq_sig_all: __u8,
    pub qp_type: __u8,
    pub is_srq: __u8,
    pub reserved: __u8,
    pub comp_mask: __u32,
    pub create_flags: __u32,
    pub rwq_ind_tbl_handle: __u32,
    pub source_qpn: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_open_qp {
    pub response: __aligned_u64,
    pub user_handle: __aligned_u64,
    pub pd_handle: __u32,
    pub qpn: __u32,
    pub qp_type: __u8,
    pub reserved: [__u8; 7],
    pub driver_data: [__aligned_u64; ],
}

// also used for open response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_qp_resp {
    pub qp_handle: __u32,
    pub qpn: __u32,
    pub max_send_wr: __u32,
    pub max_recv_wr: __u32,
    pub max_send_sge: __u32,
    pub max_recv_sge: __u32,
    pub max_inline_data: __u32,
    pub reserved: __u32,
    pub driver_data: [__u32; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_create_qp_resp {
    pub base: ib_uverbs_create_qp_resp,
    pub comp_mask: __u32,
    pub response_length: __u32,
}

//
// This struct needs to remain a multiple of 8 bytes to keep the
// alignment of the modify QP parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_qp_dest {
    pub dgid: [__u8; 16],
    pub flow_label: __u32,
    pub dlid: __u16,
    pub reserved: __u16,
    pub sgid_index: __u8,
    pub hop_limit: __u8,
    pub traffic_class: __u8,
    pub sl: __u8,
    pub src_path_bits: __u8,
    pub static_rate: __u8,
    pub is_global: __u8,
    pub port_num: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_query_qp {
    pub response: __aligned_u64,
    pub qp_handle: __u32,
    pub attr_mask: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_query_qp_resp {
    pub dest: ib_uverbs_qp_dest,
    pub alt_dest: ib_uverbs_qp_dest,
    pub max_send_wr: __u32,
    pub max_recv_wr: __u32,
    pub max_send_sge: __u32,
    pub max_recv_sge: __u32,
    pub max_inline_data: __u32,
    pub qkey: __u32,
    pub rq_psn: __u32,
    pub sq_psn: __u32,
    pub dest_qp_num: __u32,
    pub qp_access_flags: __u32,
    pub pkey_index: __u16,
    pub alt_pkey_index: __u16,
    pub qp_state: __u8,
    pub cur_qp_state: __u8,
    pub path_mtu: __u8,
    pub path_mig_state: __u8,
    pub sq_draining: __u8,
    pub max_rd_atomic: __u8,
    pub max_dest_rd_atomic: __u8,
    pub min_rnr_timer: __u8,
    pub port_num: __u8,
    pub timeout: __u8,
    pub retry_cnt: __u8,
    pub rnr_retry: __u8,
    pub alt_port_num: __u8,
    pub alt_timeout: __u8,
    pub sq_sig_all: __u8,
    pub reserved: [__u8; 5],
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_modify_qp {
    pub dest: ib_uverbs_qp_dest,
    pub alt_dest: ib_uverbs_qp_dest,
    pub qp_handle: __u32,
    pub attr_mask: __u32,
    pub qkey: __u32,
    pub rq_psn: __u32,
    pub sq_psn: __u32,
    pub dest_qp_num: __u32,
    pub qp_access_flags: __u32,
    pub pkey_index: __u16,
    pub alt_pkey_index: __u16,
    pub qp_state: __u8,
    pub cur_qp_state: __u8,
    pub path_mtu: __u8,
    pub path_mig_state: __u8,
    pub en_sqd_async_notify: __u8,
    pub max_rd_atomic: __u8,
    pub max_dest_rd_atomic: __u8,
    pub min_rnr_timer: __u8,
    pub port_num: __u8,
    pub timeout: __u8,
    pub retry_cnt: __u8,
    pub rnr_retry: __u8,
    pub alt_port_num: __u8,
    pub alt_timeout: __u8,
    pub reserved: [__u8; 2],
    pub driver_data: [__aligned_u64; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_modify_qp {
    pub base: ib_uverbs_modify_qp,
    pub rate_limit: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_modify_qp_resp {
    pub comp_mask: __u32,
    pub response_length: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_destroy_qp {
    pub response: __aligned_u64,
    pub qp_handle: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_destroy_qp_resp {
    pub events_reported: __u32,
}

//
// The ib_uverbs_sge structure isn't used anywhere, since we assume
// the ib_sge structure is packed the same way on 32-bit and 64-bit
// architectures in both kernel and user space.  It's just here to
// document the ABI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_sge {
    pub addr: __aligned_u64,
    pub length: __u32,
    pub lkey: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_uverbs_wr_opcode {
    IB_UVERBS_WR_RDMA_WRITE = 0,
    IB_UVERBS_WR_RDMA_WRITE_WITH_IMM = 1,
    IB_UVERBS_WR_SEND = 2,
    IB_UVERBS_WR_SEND_WITH_IMM = 3,
    IB_UVERBS_WR_RDMA_READ = 4,
    IB_UVERBS_WR_ATOMIC_CMP_AND_SWP = 5,
    IB_UVERBS_WR_ATOMIC_FETCH_AND_ADD = 6,
    IB_UVERBS_WR_LOCAL_INV = 7,
    IB_UVERBS_WR_BIND_MW = 8,
    IB_UVERBS_WR_SEND_WITH_INV = 9,
    IB_UVERBS_WR_TSO = 10,
    IB_UVERBS_WR_RDMA_READ_WITH_INV = 11,
    IB_UVERBS_WR_MASKED_ATOMIC_CMP_AND_SWP = 12,
    IB_UVERBS_WR_MASKED_ATOMIC_FETCH_AND_ADD = 13,
    IB_UVERBS_WR_FLUSH = 14,
    IB_UVERBS_WR_ATOMIC_WRITE = 15,
// Review enum ib_wr_opcode before modifying this
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_send_wr {
    pub wr_id: __aligned_u64,
    pub num_sge: __u32,
    pub /: *mut *mut __u32 opcode; / see enum ib_uverbs_wr_opcode,
    pub send_flags: __u32,
    pub imm_data: __be32,
    pub invalidate_rkey: __u32,
    pub ex: },
    pub remote_addr: __aligned_u64,
    pub rkey: __u32,
    pub reserved: __u32,
    pub rdma: },
    pub remote_addr: __aligned_u64,
    pub compare_add: __aligned_u64,
    pub swap: __aligned_u64,
    pub rkey: __u32,
    pub reserved: __u32,
    pub atomic: },
    pub ah: __u32,
    pub remote_qpn: __u32,
    pub remote_qkey: __u32,
    pub reserved: __u32,
    pub ud: },
    pub wr: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_post_send {
    pub response: __aligned_u64,
    pub qp_handle: __u32,
    pub wr_count: __u32,
    pub sge_count: __u32,
    pub wqe_size: __u32,
    pub send_wr: [ib_uverbs_send_wr; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_post_send_resp {
    pub bad_wr: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_recv_wr {
    pub wr_id: __aligned_u64,
    pub num_sge: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_post_recv {
    pub response: __aligned_u64,
    pub qp_handle: __u32,
    pub wr_count: __u32,
    pub sge_count: __u32,
    pub wqe_size: __u32,
    pub recv_wr: [ib_uverbs_recv_wr; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_post_recv_resp {
    pub bad_wr: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_post_srq_recv {
    pub response: __aligned_u64,
    pub srq_handle: __u32,
    pub wr_count: __u32,
    pub sge_count: __u32,
    pub wqe_size: __u32,
    pub recv: [ib_uverbs_recv_wr; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_post_srq_recv_resp {
    pub bad_wr: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_ah {
    pub response: __aligned_u64,
    pub user_handle: __aligned_u64,
    pub pd_handle: __u32,
    pub reserved: __u32,
    pub attr: ib_uverbs_ah_attr,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_ah_resp {
    pub ah_handle: __u32,
    pub driver_data: [__u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_destroy_ah {
    pub ah_handle: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_attach_mcast {
    pub gid: [__u8; 16],
    pub qp_handle: __u32,
    pub mlid: __u16,
    pub reserved: __u16,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_detach_mcast {
    pub gid: [__u8; 16],
    pub qp_handle: __u32,
    pub mlid: __u16,
    pub reserved: __u16,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_hdr {
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
// followed by flow_spec
    pub flow_spec_data: [__aligned_u64; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_eth_filter {
    pub dst_mac: [__u8; 6],
    pub src_mac: [__u8; 6],
    pub ether_type: __be16,
    pub vlan_tag: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_eth {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_ipv4_filter {
    pub src_ip: __be32,
    pub dst_ip: __be32,
    pub proto: __u8,
    pub tos: __u8,
    pub ttl: __u8,
    pub flags: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_ipv4 {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_tcp_udp_filter {
    pub dst_port: __be16,
    pub src_port: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_tcp_udp {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_ipv6_filter {
    pub src_ip: [__u8; 16],
    pub dst_ip: [__u8; 16],
    pub flow_label: __be32,
    pub next_hdr: __u8,
    pub traffic_class: __u8,
    pub hop_limit: __u8,
    pub reserved: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_ipv6 {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_action_tag {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_action_drop {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_action_handle {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_action_count {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_tunnel_filter {
    pub tunnel_id: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_tunnel {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_esp_filter {
    pub spi: __u32,
    pub seq: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_esp {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_gre_filter {
// c_ks_res0_ver field is bits 0-15 in offset 0 of a standard GRE header:
// bit 0 - C - checksum bit.
// bit 1 - reserved. set to 0.
// bit 2 - key bit.
// bit 3 - sequence number bit.
// bits 4:12 - reserved. set to 0.
// bits 13:15 - GRE version.
//
    pub c_ks_res0_ver: __be16,
    pub protocol: __be16,
    pub key: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_gre {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_mpls_filter {
// The field includes the entire MPLS label:
// bits 0:19 - label field.
// bits 20:22 - traffic class field.
// bits 23 - bottom of stack bit.
// bits 24:31 - ttl field.
//
    pub label: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec_mpls {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_attr {
    pub type: __u32,
    pub size: __u16,
    pub priority: __u16,
    pub num_of_specs: __u8,
    pub reserved: [__u8; 2],
    pub port: __u8,
    pub flags: __u32,
// Following are the optional layers according to user request
// struct ib_flow_spec_xxx
// struct ib_flow_spec_yyy
//
    pub flow_specs: [ib_uverbs_flow_spec_hdr; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_flow {
    pub comp_mask: __u32,
    pub qp_handle: __u32,
    pub flow_attr: ib_uverbs_flow_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_flow_resp {
    pub comp_mask: __u32,
    pub flow_handle: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_destroy_flow {
    pub comp_mask: __u32,
    pub flow_handle: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_srq {
    pub response: __aligned_u64,
    pub user_handle: __aligned_u64,
    pub pd_handle: __u32,
    pub max_wr: __u32,
    pub max_sge: __u32,
    pub srq_limit: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_xsrq {
    pub response: __aligned_u64,
    pub user_handle: __aligned_u64,
    pub srq_type: __u32,
    pub pd_handle: __u32,
    pub max_wr: __u32,
    pub max_sge: __u32,
    pub srq_limit: __u32,
    pub max_num_tags: __u32,
    pub xrcd_handle: __u32,
    pub cq_handle: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_create_srq_resp {
    pub srq_handle: __u32,
    pub max_wr: __u32,
    pub max_sge: __u32,
    pub srqn: __u32,
    pub driver_data: [__u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_modify_srq {
    pub srq_handle: __u32,
    pub attr_mask: __u32,
    pub max_wr: __u32,
    pub srq_limit: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_query_srq {
    pub response: __aligned_u64,
    pub srq_handle: __u32,
    pub reserved: __u32,
    pub driver_data: [__aligned_u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_query_srq_resp {
    pub max_wr: __u32,
    pub max_sge: __u32,
    pub srq_limit: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_destroy_srq {
    pub response: __aligned_u64,
    pub srq_handle: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_destroy_srq_resp {
    pub events_reported: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_create_wq {
    pub comp_mask: __u32,
    pub wq_type: __u32,
    pub user_handle: __aligned_u64,
    pub pd_handle: __u32,
    pub cq_handle: __u32,
    pub max_wr: __u32,
    pub max_sge: __u32,
    pub /: *mut *mut __u32 create_flags; / Use enum ib_wq_flags,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_create_wq_resp {
    pub comp_mask: __u32,
    pub response_length: __u32,
    pub wq_handle: __u32,
    pub max_wr: __u32,
    pub max_sge: __u32,
    pub wqn: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_destroy_wq {
    pub comp_mask: __u32,
    pub wq_handle: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_destroy_wq_resp {
    pub comp_mask: __u32,
    pub response_length: __u32,
    pub events_reported: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_modify_wq {
    pub attr_mask: __u32,
    pub wq_handle: __u32,
    pub wq_state: __u32,
    pub curr_wq_state: __u32,
    pub /: *mut *mut __u32 flags; / Use enum ib_wq_flags,
    pub /: *mut *mut __u32 flags_mask; / Use enum ib_wq_flags,
}

// Prevent memory allocation rather than max expected size
pub const IB_USER_VERBS_MAX_LOG_IND_TBL_SIZE: c_uint = 0x0d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_create_rwq_ind_table {
    pub comp_mask: __u32,
    pub log_ind_tbl_size: __u32,
// Following are the wq handles according to log_ind_tbl_size
// wq_handle1
// wq_handle2
//
    pub wq_handles: [__u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_create_rwq_ind_table_resp {
    pub comp_mask: __u32,
    pub response_length: __u32,
    pub ind_tbl_handle: __u32,
    pub ind_tbl_num: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_destroy_rwq_ind_table {
    pub comp_mask: __u32,
    pub ind_tbl_handle: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_cq_moderation {
    pub cq_count: __u16,
    pub cq_period: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ex_modify_cq {
    pub cq_handle: __u32,
    pub attr_mask: __u32,
    pub attr: ib_uverbs_cq_moderation,
    pub reserved: __u32,
}

pub const IB_DEVICE_NAME_MAX: c_int = 64;
//
// bits 9, 15, 16, 19, 22, 27, 30, 31, 32, 33, 35 and 37 may be set by old
// kernels and should not be used.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_uverbs_device_cap_flags {
    IB_UVERBS_DEVICE_RESIZE_MAX_WR = 1 << 0,
    IB_UVERBS_DEVICE_BAD_PKEY_CNTR = 1 << 1,
    IB_UVERBS_DEVICE_BAD_QKEY_CNTR = 1 << 2,
    IB_UVERBS_DEVICE_RAW_MULTI = 1 << 3,
    IB_UVERBS_DEVICE_AUTO_PATH_MIG = 1 << 4,
    IB_UVERBS_DEVICE_CHANGE_PHY_PORT = 1 << 5,
    IB_UVERBS_DEVICE_UD_AV_PORT_ENFORCE = 1 << 6,
    IB_UVERBS_DEVICE_CURR_QP_STATE_MOD = 1 << 7,
    IB_UVERBS_DEVICE_SHUTDOWN_PORT = 1 << 8,
// IB_UVERBS_DEVICE_INIT_TYPE = 1 << 9, (not in use)
    IB_UVERBS_DEVICE_PORT_ACTIVE_EVENT = 1 << 10,
    IB_UVERBS_DEVICE_SYS_IMAGE_GUID = 1 << 11,
    IB_UVERBS_DEVICE_RC_RNR_NAK_GEN = 1 << 12,
    IB_UVERBS_DEVICE_SRQ_RESIZE = 1 << 13,
    IB_UVERBS_DEVICE_N_NOTIFY_CQ = 1 << 14,
    IB_UVERBS_DEVICE_MEM_WINDOW = 1 << 17,
    IB_UVERBS_DEVICE_UD_IP_CSUM = 1 << 18,
    IB_UVERBS_DEVICE_XRC = 1 << 20,
    IB_UVERBS_DEVICE_MEM_MGT_EXTENSIONS = 1 << 21,
    IB_UVERBS_DEVICE_MEM_WINDOW_TYPE_2A = 1 << 23,
    IB_UVERBS_DEVICE_MEM_WINDOW_TYPE_2B = 1 << 24,
    IB_UVERBS_DEVICE_RC_IP_CSUM = 1 << 25,
// Deprecated. Please use IB_UVERBS_RAW_PACKET_CAP_IP_CSUM.
    IB_UVERBS_DEVICE_RAW_IP_CSUM = 1 << 26,
    IB_UVERBS_DEVICE_MANAGED_FLOW_STEERING = 1 << 29,
// Deprecated. Please use IB_UVERBS_RAW_PACKET_CAP_SCATTER_FCS.
    IB_UVERBS_DEVICE_RAW_SCATTER_FCS = 1ULL << 34,
    IB_UVERBS_DEVICE_PCI_WRITE_END_PADDING = 1ULL << 36,
// Flush placement types
    IB_UVERBS_DEVICE_FLUSH_GLOBAL = 1ULL << 38,
    IB_UVERBS_DEVICE_FLUSH_PERSISTENT = 1ULL << 39,
// Atomic write attributes
    IB_UVERBS_DEVICE_ATOMIC_WRITE = 1ULL << 40,
// CoCo guest with DMA bounce buffering required
    IB_UVERBS_DEVICE_CC_DMA_BOUNCE = 1ULL << 41,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_uverbs_raw_packet_caps {
    IB_UVERBS_RAW_PACKET_CAP_CVLAN_STRIPPING = 1 << 0,
    IB_UVERBS_RAW_PACKET_CAP_SCATTER_FCS = 1 << 1,
    IB_UVERBS_RAW_PACKET_CAP_IP_CSUM = 1 << 2,
    IB_UVERBS_RAW_PACKET_CAP_DELAY_DROP = 1 << 3,
}

//
// struct ib_uverbs_clock_info - timecounter state shared with userspace
//
// Drivers that use a software timecounter over a free-running hardware
// cycle counter can map this page read-only into userspace, allowing
// conversion of hardware timestamps to system time without a syscall.
//
// Synchronization uses a sequence counter (@sign): the kernel sets bit 0
// before updating, then advances by 2 after. Userspace must retry the read
// if @sign is odd or changed during the read.
//
// @sign:            Sequence counter (bit 0 = update in progress)
// @resv:            Reserved
// @nsec:            Nanoseconds at last update
// @cycles:          Cycle counter value at last update
// @frac:            Fractional nanoseconds at last update
// @mult:            Cycle-to-nanosecond multiplier
// @shift:           Cycle-to-nanosecond shift
// @mask:            Cycle counter bitmask
// @overflow_period: Max interval (nsec) between reads before counter wraps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_clock_info {
    pub sign: __u32,
    pub resv: __u32,
    pub nsec: __aligned_u64,
    pub cycles: __aligned_u64,
    pub frac: __aligned_u64,
    pub mult: __u32,
    pub shift: __u32,
    pub mask: __aligned_u64,
    pub overflow_period: __aligned_u64,
}
