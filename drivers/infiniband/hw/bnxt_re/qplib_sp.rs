//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bnxt_re/qplib_sp.h
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
// Broadcom NetXtreme-E RoCE driver.
//
// Copyright (c) 2016 - 2017, Broadcom. All rights reserved.  The term
// Broadcom refers to Broadcom Limited and/or its subsidiaries.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// BSD license below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS''
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS
// BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
// BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE
// OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN
// IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Description: Slow Path Operators (header)
//

pub const BNXT_QPLIB_RESERVED_QP_WRS: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_dev_attr {
pub const FW_VER_ARR_LEN: c_int = 4;
    pub fw_ver: [u8; FW_VER_ARR_LEN],
pub const BNXT_QPLIB_NUM_GIDS_SUPPORTED: c_int = 256;
    pub max_sgid: u16,
    pub max_mrw: u16,
    pub max_qp: u32,
pub const BNXT_QPLIB_MAX_OUT_RD_ATOM: c_int = 126;
    pub max_qp_rd_atom: u32,
    pub max_qp_init_rd_atom: u32,
    pub max_qp_wqes: u32,
    pub max_qp_sges: u32,
    pub max_cq: u32,
pub const BNXT_QPLIB_MAX_CQ_WQES: c_uint = 0xfffff;
    pub max_cq_wqes: u32,
    pub max_cq_sges: u32,
    pub max_mr: u32,
    pub max_mr_size: u64,
    pub max_pd: u32,
    pub max_mw: u32,
    pub max_raw_ethy_qp: u32,
    pub max_ah: u32,
    pub max_srq: u32,
    pub max_srq_wqes: u32,
    pub max_srq_sges: u32,
    pub max_pkey: u32,
    pub max_inline_data: u32,
    pub l2_db_size: u32,
    pub tqm_alloc_reqs: [u8; MAX_TQM_ALLOC_REQ],
    pub is_atomic: bool,
    pub dev_cap_flags: u16,
    pub dev_cap_flags2: u16,
    pub max_dpi: u32,
    pub rate_limit_min: u16,
    pub rate_limit_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_pd {
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_gid {
    pub data: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_gid_info {
    pub gid: bnxt_qplib_gid,
    pub vlan_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_ah {
    pub dgid: bnxt_qplib_gid,
    pub pd: *mut bnxt_qplib_pd,
    pub id: u32,
    pub sgid_index: u8,
// For Query AH if the hw table and SW table are differnt
    pub host_sgid_index: u8,
    pub traffic_class: u8,
    pub flow_label: u32,
    pub hop_limit: u8,
    pub sl: u8,
    pub dmac: [u8; 6],
    pub vlan_id: u16,
    pub nw_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_mrw {
    pub pd: *mut bnxt_qplib_pd,
    pub type: c_int,
    pub access_flags: u32,
pub const BNXT_QPLIB_MR_ACCESS_MASK: c_uint = 0xFF;
pub const BNXT_QPLIB_FR_PMR: c_uint = 0x80000000;
    pub lkey: u32,
    pub rkey: u32,
pub const BNXT_QPLIB_RSVD_LKEY: c_uint = 0xFFFFFFFF;
    pub va: u64,
    pub total_size: u64,
    pub npages: u32,
    pub flags: u16,
    pub mr_handle: u64,
    pub hwq: bnxt_qplib_hwq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_frpl {
    pub max_pg_ptrs: c_int,
    pub hwq: bnxt_qplib_hwq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_roce_stats {
    pub to_retransmits: u64,
    pub seq_err_naks_rcvd: u64,
// seq_err_naks_rcvd is 64 b
    pub max_retry_exceeded: u64,
// max_retry_exceeded is 64 b
    pub rnr_naks_rcvd: u64,
// rnr_naks_rcvd is 64 b
    pub missing_resp: u64,
    pub unrecoverable_err: u64,
// unrecoverable_err is 64 b
    pub bad_resp_err: u64,
// bad_resp_err is 64 b
    pub local_qp_op_err: u64,
// local_qp_op_err is 64 b
    pub local_protection_err: u64,
// local_protection_err is 64 b
    pub mem_mgmt_op_err: u64,
// mem_mgmt_op_err is 64 b
    pub remote_invalid_req_err: u64,
// remote_invalid_req_err is 64 b
    pub remote_access_err: u64,
// remote_access_err is 64 b
    pub remote_op_err: u64,
// remote_op_err is 64 b
    pub dup_req: u64,
// dup_req is 64 b
    pub res_exceed_max: u64,
// res_exceed_max is 64 b
    pub res_length_mismatch: u64,
// res_length_mismatch is 64 b
    pub res_exceeds_wqe: u64,
// res_exceeds_wqe is 64 b
    pub res_opcode_err: u64,
// res_opcode_err is 64 b
    pub res_rx_invalid_rkey: u64,
// res_rx_invalid_rkey is 64 b
    pub res_rx_domain_err: u64,
// res_rx_domain_err is 64 b
    pub res_rx_no_perm: u64,
// res_rx_no_perm is 64 b
    pub res_rx_range_err: u64,
// res_rx_range_err is 64 b
    pub res_tx_invalid_rkey: u64,
// res_tx_invalid_rkey is 64 b
    pub res_tx_domain_err: u64,
// res_tx_domain_err is 64 b
    pub res_tx_no_perm: u64,
// res_tx_no_perm is 64 b
    pub res_tx_range_err: u64,
// res_tx_range_err is 64 b
    pub res_irrq_oflow: u64,
// res_irrq_oflow is 64 b
    pub res_unsup_opcode: u64,
// res_unsup_opcode is 64 b
    pub res_unaligned_atomic: u64,
// res_unaligned_atomic is 64 b
    pub res_rem_inv_err: u64,
// res_rem_inv_err is 64 b
    pub res_mem_error: u64,
// res_mem_error is 64 b
    pub res_srq_err: u64,
// res_srq_err is 64 b
    pub res_cmp_err: u64,
// res_cmp_err is 64 b
    pub res_invalid_dup_rkey: u64,
// res_invalid_dup_rkey is 64 b
    pub res_wqe_format_err: u64,
// res_wqe_format_err is 64 b
    pub res_cq_load_err: u64,
// res_cq_load_err is 64 b
    pub res_srq_load_err: u64,
// res_srq_load_err is 64 b
    pub res_tx_pci_err: u64,
// res_tx_pci_err is 64 b
    pub res_rx_pci_err: u64,
// res_rx_pci_err is 64 b
    pub res_oos_drop_count: u64,
// res_oos_drop_count
    pub active_qp_count_p0: u64,
// port 0 active qps
    pub active_qp_count_p1: u64,
// port 1 active qps
    pub active_qp_count_p2: u64,
// port 2 active qps
    pub active_qp_count_p3: u64,
// port 3 active qps
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_ext_stat {
    pub tx_atomic_req: u64,
    pub tx_read_req: u64,
    pub tx_read_res: u64,
    pub tx_write_req: u64,
    pub tx_send_req: u64,
    pub tx_roce_pkts: u64,
    pub tx_roce_bytes: u64,
    pub rx_atomic_req: u64,
    pub rx_read_req: u64,
    pub rx_read_res: u64,
    pub rx_write_req: u64,
    pub rx_send_req: u64,
    pub rx_roce_pkts: u64,
    pub rx_roce_bytes: u64,
    pub rx_roce_good_pkts: u64,
    pub rx_roce_good_bytes: u64,
    pub rx_out_of_buffer: u64,
    pub rx_out_of_sequence: u64,
    pub tx_cnp: u64,
    pub rx_cnp: u64,
    pub rx_ecn_marked: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_cc_param_ext {
    pub ext_mask: u64,
    pub inact_th_hi: u16,
    pub min_delta_cnp: u16,
    pub init_cp: u16,
    pub tr_update_mode: u8,
    pub tr_update_cyls: u8,
    pub fr_rtt: u8,
    pub ai_rate_incr: u8,
    pub rr_rtt_th: u16,
    pub ar_cr_th: u16,
    pub cr_min_th: u16,
    pub bw_avg_weight: u8,
    pub cr_factor: u8,
    pub cr_th_max_cp: u16,
    pub cp_bias_en: u8,
    pub cp_bias: u8,
    pub cnp_ecn: u8,
    pub rtt_jitter_en: u8,
    pub bytes_per_usec: u16,
    pub cc_cr_reset_th: u16,
    pub cr_width: u8,
    pub min_quota: u8,
    pub max_quota: u8,
    pub abs_max_quota: u8,
    pub tr_lb: u16,
    pub cr_prob_fac: u8,
    pub tr_prob_fac: u8,
    pub fair_cr_th: u16,
    pub red_div: u8,
    pub cnp_ratio_th: u8,
    pub ai_ext_rtt: u16,
    pub exp_crcp_ratio: u8,
    pub low_rate_en: u8,
    pub cpcr_update_th: u16,
    pub ai_rtt_th1: u16,
    pub ai_rtt_th2: u16,
    pub cf_rtt_th: u16,
    pub /: *mut *mut u16 sc_cr_th1; / severe congestion cr threshold 1,
    pub /: *mut *mut u16 sc_cr_th2; / severe congestion cr threshold 2,
    pub l64B_per_rtt: u32,
    pub cc_ack_bytes: u8,
    pub reduce_cf_rtt_th: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_cc_param {
    pub alt_vlan_pcp: u8,
    pub qp1_tos_dscp: u8,
    pub alt_tos_dscp: u16,
    pub cc_mode: u8,
    pub enable: u8,
    pub inact_th: u16,
    pub init_cr: u16,
    pub init_tr: u16,
    pub rtt: u16,
    pub g: u8,
    pub nph_per_state: u8,
    pub time_pph: u8,
    pub pkts_pph: u8,
    pub tos_ecn: u8,
    pub tos_dscp: u8,
    pub tcp_cp: u16,
    pub cc_ext: bnxt_qplib_cc_param_ext,
    pub mask: u32,
}

extern "C" {
    pub fn bnxt_qplib_get_dev_attr(rcfw: *mut bnxt_qplib_rcfw) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_free_mrw(res: *mut bnxt_qplib_res, mr: *mut bnxt_qplib_mrw) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_query_version(rcfw: *mut bnxt_qplib_rcfw);
}
extern "C" {
    pub fn bnxt_qplib_create_flow(res: *mut bnxt_qplib_res) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_destroy_flow(res: *mut bnxt_qplib_res) -> c_int;
}
pub const BNXT_VAR_MAX_WQE: c_int = 4352;
pub const BNXT_VAR_MAX_SLOT_ALIGN: c_int = 256;
pub const BNXT_VAR_MAX_SGE: c_int = 13;
pub const BNXT_RE_MAX_RQ_WQES: c_int = 65536;
pub const BNXT_RE_MAX_SQ_SLOTS: c_int = 65536;
pub const BNXT_STATIC_MAX_SGE: c_int = 6;
