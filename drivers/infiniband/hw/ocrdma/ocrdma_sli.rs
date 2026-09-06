//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/ocrdma/ocrdma_sli.h
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


// This file is part of the Emulex RoCE Device Driver for
// RoCE (RDMA over Converged Ethernet) adapters.
// Copyright (C) 2012-2015 Emulex. All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.emulex.com
//
// This software is available to you under a choice of one of two licenses.
// You may choose to be licensed under the terms of the GNU General Public
// License (GPL) Version 2, available from the file COPYING in the main
// directory of this source tree, or the BSD license below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// - Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// - Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
// BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
// OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
// ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Contact Information:
// linux-drivers@emulex.com
//
// Emulex
// 3333 Susan Street
// Costa Mesa, CA 92626
//
pub const OCRDMA_SUBSYS_ROCE: c_int = 10;
pub const OCRDMA_SUBSYS_COMMON: c_int = 1;
pub const OCRDMA_MAX_SGID: c_int = 16;
pub const OCRDMA_MAX_QP: c_int = 2048;
pub const OCRDMA_MAX_CQ: c_int = 2048;
pub const OCRDMA_MAX_STAG: c_int = 16384;
pub const OCRDMA_DB_CQ_RING_ID_MASK: c_uint = 0x3FF	/* bits 0 - 9 */;
pub const OCRDMA_DB_CQ_RING_ID_EXT_MASK: c_uint = 0x0C00	/* bits 10-11 of qid at 12-11 */;
// qid #2 msbits at 12-11
pub const OCRDMA_DB_CQ_RING_ID_EXT_MASK_SHIFT: c_uint = 0x1;

// Rearm bit

// solicited bit

pub const OCRDMA_EQ_ID_MASK: c_uint = 0x1FF	/* bits 0 - 8 */;
pub const OCRDMA_EQ_ID_EXT_MASK: c_uint = 0x3e00	/* bits 9-13 */;

// Clear the interrupt for this eq

// Must be 1

// Number of event entries processed

// Rearm bit

pub const OCRDMA_MQ_ID_MASK: c_uint = 0x7FF	/* bits 0 - 10 */;
// Number of entries posted

pub const OCRDMA_MIN_HPAGE_SIZE: c_int = 4096;
pub const OCRDMA_MIN_Q_PAGE_SIZE: c_int = 4096;
pub const OCRDMA_MAX_Q_PAGES: c_int = 8;
pub const OCRDMA_SLI_ASIC_ID_OFFSET: c_uint = 0x9C;
pub const OCRDMA_SLI_ASIC_REV_MASK: c_uint = 0x000000FF;
pub const OCRDMA_SLI_ASIC_GEN_NUM_MASK: c_uint = 0x0000FF00;
pub const OCRDMA_SLI_ASIC_GEN_NUM_SHIFT: c_uint = 0x08;
//

//
pub const OCRDMA_MAX_Q_PAGE_SIZE_CNT: c_int = 8;

pub const MAX_OCRDMA_QP_PAGES: c_int = 8;

pub const OCRDMA_CREATE_CQ_MAX_PAGES: c_int = 4;
pub const OCRDMA_DPP_CQE_SIZE: c_int = 4;
pub const OCRDMA_GEN2_MAX_CQE: c_int = 1024;
pub const OCRDMA_GEN2_CQ_PAGE_SIZE: c_int = 4096;
pub const OCRDMA_GEN2_WQE_SIZE: c_int = 256;
pub const OCRDMA_MAX_CQE: c_int = 4095;
pub const OCRDMA_CQ_PAGE_SIZE: c_int = 16384;
pub const OCRDMA_WQE_SIZE: c_int = 128;
pub const OCRDMA_WQE_STRIDE: c_int = 8;
pub const OCRDMA_WQE_ALIGN_BYTES: c_int = 16;

// mailbox cmd header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_mbx_hdr {
    pub subsys_op: u32,
    pub /: *mut *mut u32 timeout; / in seconds,
    pub cmd_len: u32,
    pub rsvd_version: u32,
}

// mailbox cmd response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_mbx_rsp {
    pub subsys_op: u32,
    pub status: u32,
    pub rsp_len: u32,
    pub add_rsp_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_mqe_sge {
    pub pa_lo: u32,
    pub pa_hi: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_mqe_hdr {
    pub spcl_sge_cnt_emb: u32,
    pub pyld_len: u32,
    pub tag_lo: u32,
    pub tag_hi: u32,
    pub rsvd3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_mqe_emb_cmd {
    pub mch: ocrdma_mbx_hdr,
    pub pyld: [u8; 220],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_mqe {
    pub hdr: ocrdma_mqe_hdr,
    pub emb_req: ocrdma_mqe_emb_cmd,
    pub sge: [ocrdma_mqe_sge; 19],
    pub nonemb_req: },
    pub cmd: [u8; 236],
    pub rsp: ocrdma_mbx_rsp,
    pub u: },
}

pub const OCRDMA_EQ_LEN: c_int = 4096;
pub const OCRDMA_MQ_CQ_LEN: c_int = 256;
pub const OCRDMA_MQ_LEN: c_int = 128;
pub const PAGE_SHIFT_4K: c_int = 12;

// Returns number of pages spanned by the data starting at the given addr

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_delete_q_req {
    pub req: ocrdma_mbx_hdr,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_pa {
    pub lo: u32,
    pub hi: u32,
}

pub const MAX_OCRDMA_EQ_PAGES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_eq_req {
    pub req: ocrdma_mbx_hdr,
    pub num_pages: u32,
    pub valid: u32,
    pub cnt: u32,
    pub delay: u32,
    pub rsvd: u32,
    pub pa: [ocrdma_pa; MAX_OCRDMA_EQ_PAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_eq_rsp {
    pub rsp: ocrdma_mbx_rsp,
    pub vector_eqid: u32,
}

pub const OCRDMA_EQ_MINOR_OTHER: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrmda_set_eqd {
    pub eq_id: u32,
    pub phase: u32,
    pub delay_multiplier: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_modify_eqd_cmd {
    pub req: ocrdma_mbx_hdr,
    pub num_eq: u32,
    pub set_eqd: [ocrmda_set_eqd; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_modify_eqd_req {
    pub hdr: ocrdma_mqe_hdr,
    pub cmd: ocrdma_modify_eqd_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_modify_eq_delay_rsp {
    pub hdr: ocrdma_mbx_rsp,
    pub rsvd0: u32,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_mcqe {
    pub status: u32,
    pub tag_lo: u32,
    pub tag_hi: u32,
    pub valid_ae_cmpl_cons: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_ae_mcqe {
    pub qpvalid_qpid: u32,
    pub cqvalid_cqid: u32,
    pub evt_tag: u32,
    pub valid_ae_event: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_ae_pvid_mcqe {
    pub tag_enabled: u32,
    pub event_tag: u32,
    pub rsvd1: u32,
    pub rsvd2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_ae_mpa_mcqe {
    pub req_id: u32,
    pub w1: u32,
    pub w2: u32,
    pub valid_ae_event: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_ae_qp_mcqe {
    pub qp_id_state: u32,
    pub w1: u32,
    pub w2: u32,
    pub valid_ae_event: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocrdma_async_event_code {
    OCRDMA_ASYNC_LINK_EVE_CODE	= 0x01,
    OCRDMA_ASYNC_GRP5_EVE_CODE	= 0x05,
    OCRDMA_ASYNC_RDMA_EVE_CODE	= 0x14
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocrdma_async_grp5_events {
    OCRDMA_ASYNC_EVENT_QOS_VALUE	= 0x01,
    OCRDMA_ASYNC_EVENT_COS_VALUE	= 0x02,
    OCRDMA_ASYNC_EVENT_PVID_STATE	= 0x03
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OCRDMA_ASYNC_EVENT_TYPE {
    OCRDMA_CQ_ERROR			= 0x00,
    OCRDMA_CQ_OVERRUN_ERROR		= 0x01,
    OCRDMA_CQ_QPCAT_ERROR		= 0x02,
    OCRDMA_QP_ACCESS_ERROR		= 0x03,
    OCRDMA_QP_COMM_EST_EVENT	= 0x04,
    OCRDMA_SQ_DRAINED_EVENT		= 0x05,
    OCRDMA_DEVICE_FATAL_EVENT	= 0x08,
    OCRDMA_SRQCAT_ERROR		= 0x0E,
    OCRDMA_SRQ_LIMIT_EVENT		= 0x0F,
    OCRDMA_QP_LAST_WQE_EVENT	= 0x10,

    OCRDMA_MAX_ASYNC_ERRORS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_ae_lnkst_mcqe {
    pub speed_state_ptn: u32,
    pub qos_reason_falut: u32,
    pub evt_tag: u32,
    pub valid_ae_event: u32,
}

// mailbox command request and responses
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_mbx_query_config {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub qp_srq_cq_ird_ord: u32,
    pub max_pd_ca_ack_delay: u32,
    pub max_recv_send_sge: u32,
    pub max_ird_ord_per_qp: u32,
    pub max_shared_ird_ord: u32,
    pub max_mr: u32,
    pub max_mr_size_hi: u32,
    pub max_mr_size_lo: u32,
    pub max_num_mr_pbl: u32,
    pub max_mw: u32,
    pub max_fmr: u32,
    pub max_pages_per_frmr: u32,
    pub max_mcast_group: u32,
    pub max_mcast_qp_attach: u32,
    pub max_total_mcast_qp_attach: u32,
    pub wqe_rqe_stride_max_dpp_cqs: u32,
    pub max_srq_rpir_qps: u32,
    pub max_dpp_pds_credits: u32,
    pub max_dpp_credits_pds_per_pd: u32,
    pub max_wqes_rqes_per_q: u32,
    pub max_cq_cqes_per_cq: u32,
    pub max_srq_rqe_sge: u32,
    pub max_wr_rd_sge: u32,
    pub ird_pgsz_num_pages: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_fw_ver_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub running_ver: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_fw_conf_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub config_num: u32,
    pub asic_revision: u32,
    pub phy_port: u32,
    pub fn_mode: u32,
    pub mode: u32,
    pub nic_wqid_base: u32,
    pub nic_wq_tot: u32,
    pub prot_wqid_base: u32,
    pub prot_wq_tot: u32,
    pub prot_rqid_base: u32,
    pub prot_rqid_tot: u32,
    pub rsvd: [u32; 6],
    pub ulp: [}; 2],
    pub fn_capabilities: u32,
    pub rsvd1: u32,
    pub rsvd2: u32,
    pub base_eqid: u32,
    pub max_eq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_get_phy_info_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub ityp_ptyp: u32,
    pub misc_params: u32,
    pub ftrdtl_exphydtl: u32,
    pub fspeed_aspeed: u32,
    pub future_use: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_get_link_speed_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub pflt_pps_ld_pnum: u32,
    pub qos_lsp: u32,
    pub res_lnk_st: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_cq_cmd {
    pub req: ocrdma_mbx_hdr,
    pub pgsz_pgcnt: u32,
    pub ev_cnt_flags: u32,
    pub eqn: u32,
    pub pdid_cqecnt: u32,
    pub rsvd6: u32,
    pub pa: [ocrdma_pa; OCRDMA_CREATE_CQ_MAX_PAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_cq {
    pub hdr: ocrdma_mqe_hdr,
    pub cmd: ocrdma_create_cq_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_cq_cmd_rsp {
    pub rsp: ocrdma_mbx_rsp,
    pub cq_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_cq_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_create_cq_cmd_rsp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_mq_req {
    pub req: ocrdma_mbx_hdr,
    pub cqid_pages: u32,
    pub async_event_bitmap: u32,
    pub async_cqid_ringsize: u32,
    pub valid: u32,
    pub async_cqid_valid: u32,
    pub rsvd: u32,
    pub pa: [ocrdma_pa; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_mq_rsp {
    pub rsp: ocrdma_mbx_rsp,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_destroy_cq {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub bypass_flush_qid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_destroy_cq_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
}

pub const MAX_OCRDMA_IRD_PAGES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocrdma_qp_flags {
    OCRDMA_QP_MW_BIND	= 1,
    OCRDMA_QP_LKEY0		= (1 << 1),
    OCRDMA_QP_FAST_REG	= (1 << 2),
    OCRDMA_QP_INB_RD	= (1 << 6),
    OCRDMA_QP_INB_WR	= (1 << 7),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocrdma_qp_state {
    OCRDMA_QPS_RST		= 0,
    OCRDMA_QPS_INIT		= 1,
    OCRDMA_QPS_RTR		= 2,
    OCRDMA_QPS_RTS		= 3,
    OCRDMA_QPS_SQE		= 4,
    OCRDMA_QPS_SQ_DRAINING	= 5,
    OCRDMA_QPS_ERR		= 6,
    OCRDMA_QPS_SQD		= 7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_qp_req {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub type_pgsz_pdn: u32,
    pub max_wqe_rqe: u32,
    pub max_sge_send_write: u32,
    pub max_sge_recv_flags: u32,
    pub max_ord_ird: u32,
    pub num_wq_rq_pages: u32,
    pub wqe_rqe_size: u32,
    pub wq_rq_cqid: u32,
    pub wq_addr: [ocrdma_pa; MAX_OCRDMA_QP_PAGES],
    pub rq_addr: [ocrdma_pa; MAX_OCRDMA_QP_PAGES],
    pub dpp_credits_cqid: u32,
    pub rpir_lkey: u32,
    pub ird_addr: [ocrdma_pa; MAX_OCRDMA_IRD_PAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_qp_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub qp_id: u32,
    pub max_wqe_rqe: u32,
    pub max_sge_send_write: u32,
    pub max_sge_recv: u32,
    pub max_ord_ird: u32,
    pub sq_rq_id: u32,
    pub dpp_response: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_destroy_qp {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub qp_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_destroy_qp_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_qp_params {
    pub id: u32,
    pub max_wqe_rqe: u32,
    pub max_sge_send_write: u32,
    pub max_sge_recv_flags: u32,
    pub max_ord_ird: u32,
    pub wq_rq_cqid: u32,
    pub hop_lmt_rq_psn: u32,
    pub tclass_sq_psn: u32,
    pub ack_to_rnr_rtc_dest_qpn: u32,
    pub path_mtu_pkey_indx: u32,
    pub rnt_rc_sl_fl: u32,
    pub sgid: [u8; 16],
    pub dgid: [u8; 16],
    pub dmac_b0_to_b3: u32,
    pub vlan_dmac_b4_to_b5: u32,
    pub qkey: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_modify_qp {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub params: ocrdma_qp_params,
    pub flags: u32,
    pub rdma_flags: u32,
    pub num_outstanding_atomic_rd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_modify_qp_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub max_wqe_rqe: u32,
    pub max_ord_ird: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_query_qp {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
pub const OCRDMA_QUERY_UP_QP_ID_SHIFT: c_int = 0;
pub const OCRDMA_QUERY_UP_QP_ID_MASK: c_uint = 0xFFFFFF;
    pub qp_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_query_qp_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub params: ocrdma_qp_params,
    pub dpp_credits_cqid: u32,
    pub rbq_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_srq {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub pgsz_pdid: u32,
    pub max_sge_rqe: u32,
    pub pages_rqe_sz: u32,
    pub rq_addr: [ocrdma_pa; MAX_OCRDMA_SRQ_PAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_srq_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub id: u32,
    pub max_sge_rqe_allocated: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_modify_srq {
    pub hdr: ocrdma_mqe_hdr,
    pub rep: ocrdma_mbx_rsp,
    pub id: u32,
    pub limit_max_rqe: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_query_srq {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_rsp,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_query_srq_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_rsp,
    pub max_rqe_pdid: u32,
    pub srq_lmt_max_sge: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_destroy_srq {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_rsp,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_alloc_pd {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub enable_dpp_rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_alloc_pd_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub dpp_page_pdid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_dealloc_pd {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_dealloc_pd_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_alloc_pd_range {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub enable_dpp_rsvd: u32,
    pub pd_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_alloc_pd_range_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub dpp_page_pdid: u32,
    pub pd_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_dealloc_pd_range {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub start_pd_id: u32,
    pub pd_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_dealloc_pd_range_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_alloc_lkey {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub pdid: u32,
    pub pbl_sz_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_alloc_lkey_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub lrkey: u32,
    pub num_pbl_rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_dealloc_lkey {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub lkey: u32,
    pub rsvd_frmr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_dealloc_lkey_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
}

pub const MAX_OCRDMA_PBL_SIZE: c_int = 65536;
pub const MAX_OCRDMA_PBL_PER_LKEY: c_int = 32767;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_reg_nsmr {
    pub hdr: ocrdma_mqe_hdr,
    pub cmd: ocrdma_mbx_hdr,
    pub fr_mr: u32,
    pub num_pbl_pdid: u32,
    pub flags_hpage_pbe_sz: u32,
    pub totlen_low: u32,
    pub totlen_high: u32,
    pub fbo_low: u32,
    pub fbo_high: u32,
    pub va_loaddr: u32,
    pub va_hiaddr: u32,
    pub pbl: [ocrdma_pa; MAX_OCRDMA_NSMR_PBL],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_reg_nsmr_cont {
    pub hdr: ocrdma_mqe_hdr,
    pub cmd: ocrdma_mbx_hdr,
    pub lrkey: u32,
    pub num_pbl_offset: u32,
    pub last: u32,
    pub pbl: [ocrdma_pa; MAX_OCRDMA_NSMR_PBL],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_pbe {
    pub pa_hi: u32,
    pub pa_lo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_reg_nsmr_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub lrkey: u32,
    pub num_pbl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_reg_nsmr_cont_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub lrkey_key_index: u32,
    pub num_pbl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_alloc_mw {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub pdid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_alloc_mw_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub lrkey_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_attach_mcast {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub qp_id: u32,
    pub mgid: [u8; 16],
    pub mac_b0_to_b3: u32,
    pub vlan_mac_b4_to_b5: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_attach_mcast_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_detach_mcast {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub qp_id: u32,
    pub mgid: [u8; 16],
    pub mac_b0_to_b3: u32,
    pub vlan_mac_b4_to_b5: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_detach_mcast_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
}

pub const OCRDMA_AH_TBL_PAGES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_ah_tbl {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub ah_conf: u32,
    pub tbl_addr: [ocrdma_pa; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_ah_tbl_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
    pub ahid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_delete_ah_tbl {
    pub hdr: ocrdma_mqe_hdr,
    pub req: ocrdma_mbx_hdr,
    pub ahid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_delete_ah_tbl_rsp {
    pub hdr: ocrdma_mqe_hdr,
    pub rsp: ocrdma_mbx_rsp,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum major_code {
    OCRDMA_MAJOR_CODE_COMPLETION    = 0x00,
    OCRDMA_MAJOR_CODE_SENTINAL      = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_eqe {
    pub id_valid: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OCRDMA_CQE_STATUS {
    OCRDMA_CQE_SUCCESS = 0,
    OCRDMA_CQE_LOC_LEN_ERR,
    OCRDMA_CQE_LOC_QP_OP_ERR,
    OCRDMA_CQE_LOC_EEC_OP_ERR,
    OCRDMA_CQE_LOC_PROT_ERR,
    OCRDMA_CQE_WR_FLUSH_ERR,
    OCRDMA_CQE_MW_BIND_ERR,
    OCRDMA_CQE_BAD_RESP_ERR,
    OCRDMA_CQE_LOC_ACCESS_ERR,
    OCRDMA_CQE_REM_INV_REQ_ERR,
    OCRDMA_CQE_REM_ACCESS_ERR,
    OCRDMA_CQE_REM_OP_ERR,
    OCRDMA_CQE_RETRY_EXC_ERR,
    OCRDMA_CQE_RNR_RETRY_EXC_ERR,
    OCRDMA_CQE_LOC_RDD_VIOL_ERR,
    OCRDMA_CQE_REM_INV_RD_REQ_ERR,
    OCRDMA_CQE_REM_ABORT_ERR,
    OCRDMA_CQE_INV_EECN_ERR,
    OCRDMA_CQE_INV_EEC_STATE_ERR,
    OCRDMA_CQE_FATAL_ERR,
    OCRDMA_CQE_RESP_TIMEOUT_ERR,
    OCRDMA_CQE_GENERAL_ERR,

    OCRDMA_MAX_CQE_ERR
}

// w0
// w1
// w2
// w3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_cqe {
// w0 to w2
    pub wqeidx: u32,
    pub bytes_xfered: u32,
    pub qpn: u32,
    pub wq: },
    pub lkey_immdt: u32,
    pub rxlen: u32,
    pub buftag_qpn: u32,
    pub rq: },
    pub lkey_immdt: u32,
    pub rxlen_pkey: u32,
    pub buftag_qpn: u32,
    pub ud: },
    pub word_0: u32,
    pub word_1: u32,
    pub qpn: u32,
    pub cmn: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_sge {
    pub addr_hi: u32,
    pub addr_lo: u32,
    pub lrkey: u32,
    pub len: u32,
}

// Stag flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OCRDMA_WQE_OPCODE {
    OCRDMA_WRITE		= 0x06,
    OCRDMA_READ		= 0x0C,
    OCRDMA_RESV0		= 0x02,
    OCRDMA_SEND		= 0x00,
    OCRDMA_CMP_SWP		= 0x14,
    OCRDMA_BIND_MW		= 0x10,
    OCRDMA_FR_MR            = 0x11,
    OCRDMA_RESV1		= 0x0A,
    OCRDMA_LKEY_INV		= 0x15,
    OCRDMA_FETCH_ADD	= 0x13,
    OCRDMA_POST_RQ		= 0x12
}

// header WQE for all the SQ and RQ operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_hdr_wqe {
    pub cw: u32,
    pub rsvd_tag: u32,
    pub rsvd_lkey_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_ewqe_ud_hdr {
    pub rsvd_dest_qpn: u32,
    pub qkey: u32,
    pub rsvd_ahid: u32,
    pub hdr_type: u32,
}

// extended wqe followed by hdr_wqe for Fast Memory register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_ewqe_fr {
    pub va_hi: u32,
    pub va_lo: u32,
    pub fbo_hi: u32,
    pub fbo_lo: u32,
    pub size_sge: u32,
    pub num_sges: u32,
    pub rsvd: u32,
    pub rsvd2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_eth_basic {
    pub dmac: [u8; 6],
    pub smac: [u8; 6],
    pub eth_type: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_eth_vlan {
    pub dmac: [u8; 6],
    pub smac: [u8; 6],
    pub eth_type: __be16,
    pub vlan_tag: __be16,
    pub roce_eth_type: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_grh {
    pub tclass_flow: __be32,
    pub pdid_hoplimit: __be32,
    pub sgid: [u8; 16],
    pub dgid: [u8; 16],
    pub rsvd: u16,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_av {
    pub eth_hdr: ocrdma_eth_vlan,
    pub grh: ocrdma_grh,
    pub valid: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_rsrc_stats {
    pub dpp_pds: u32,
    pub non_dpp_pds: u32,
    pub rc_dpp_qps: u32,
    pub uc_dpp_qps: u32,
    pub ud_dpp_qps: u32,
    pub rc_non_dpp_qps: u32,
    pub rsvd: u32,
    pub uc_non_dpp_qps: u32,
    pub ud_non_dpp_qps: u32,
    pub rsvd1: u32,
    pub srqs: u32,
    pub rbqs: u32,
    pub r64K_nsmr: u32,
    pub r64K_to_2M_nsmr: u32,
    pub r2M_to_44M_nsmr: u32,
    pub r44M_to_1G_nsmr: u32,
    pub r1G_to_4G_nsmr: u32,
    pub nsmr_count_4G_to_32G: u32,
    pub r32G_to_64G_nsmr: u32,
    pub r64G_to_128G_nsmr: u32,
    pub r128G_to_higher_nsmr: u32,
    pub embedded_nsmr: u32,
    pub frmr: u32,
    pub prefetch_qps: u32,
    pub ondemand_qps: u32,
    pub phy_mr: u32,
    pub mw: u32,
    pub rsvd2: [u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_db_err_stats {
    pub sq_doorbell_errors: u32,
    pub cq_doorbell_errors: u32,
    pub rq_srq_doorbell_errors: u32,
    pub cq_overflow_errors: u32,
    pub rsvd: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_wqe_stats {
    pub large_send_rc_wqes_lo: u32,
    pub large_send_rc_wqes_hi: u32,
    pub large_write_rc_wqes_lo: u32,
    pub large_write_rc_wqes_hi: u32,
    pub rsvd: [u32; 4],
    pub read_wqes_lo: u32,
    pub read_wqes_hi: u32,
    pub frmr_wqes_lo: u32,
    pub frmr_wqes_hi: u32,
    pub mw_bind_wqes_lo: u32,
    pub mw_bind_wqes_hi: u32,
    pub invalidate_wqes_lo: u32,
    pub invalidate_wqes_hi: u32,
    pub rsvd1: [u32; 2],
    pub dpp_wqe_drops: u32,
    pub rsvd2: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_tx_stats {
    pub send_pkts_lo: u32,
    pub send_pkts_hi: u32,
    pub write_pkts_lo: u32,
    pub write_pkts_hi: u32,
    pub read_pkts_lo: u32,
    pub read_pkts_hi: u32,
    pub read_rsp_pkts_lo: u32,
    pub read_rsp_pkts_hi: u32,
    pub ack_pkts_lo: u32,
    pub ack_pkts_hi: u32,
    pub send_bytes_lo: u32,
    pub send_bytes_hi: u32,
    pub write_bytes_lo: u32,
    pub write_bytes_hi: u32,
    pub read_req_bytes_lo: u32,
    pub read_req_bytes_hi: u32,
    pub read_rsp_bytes_lo: u32,
    pub read_rsp_bytes_hi: u32,
    pub ack_timeouts: u32,
    pub rsvd: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_tx_qp_err_stats {
    pub local_length_errors: u32,
    pub local_protection_errors: u32,
    pub local_qp_operation_errors: u32,
    pub retry_count_exceeded_errors: u32,
    pub rnr_retry_count_exceeded_errors: u32,
    pub rsvd: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_rx_stats {
    pub roce_frame_bytes_lo: u32,
    pub roce_frame_bytes_hi: u32,
    pub roce_frame_icrc_drops: u32,
    pub roce_frame_payload_len_drops: u32,
    pub ud_drops: u32,
    pub qp1_drops: u32,
    pub psn_error_request_packets: u32,
    pub psn_error_resp_packets: u32,
    pub rnr_nak_timeouts: u32,
    pub rnr_nak_receives: u32,
    pub roce_frame_rxmt_drops: u32,
    pub nak_count_psn_sequence_errors: u32,
    pub rc_drop_count_lookup_errors: u32,
    pub rq_rnr_naks: u32,
    pub srq_rnr_naks: u32,
    pub roce_frames_lo: u32,
    pub roce_frames_hi: u32,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_rx_qp_err_stats {
    pub nak_invalid_request_errors: u32,
    pub nak_remote_operation_errors: u32,
    pub nak_count_remote_access_errors: u32,
    pub local_length_errors: u32,
    pub local_protection_errors: u32,
    pub local_qp_operation_errors: u32,
    pub rsvd: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_tx_dbg_stats {
    pub data: [u32; 100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_rx_dbg_stats {
    pub data: [u32; 200],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_rdma_stats_req {
    pub hdr: ocrdma_mbx_hdr,
    pub reset_stats: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_rdma_stats_resp {
    pub hdr: ocrdma_mbx_hdr,
    pub act_rsrc_stats: ocrdma_rsrc_stats,
    pub th_rsrc_stats: ocrdma_rsrc_stats,
    pub db_err_stats: ocrdma_db_err_stats,
    pub wqe_stats: ocrdma_wqe_stats,
    pub tx_stats: ocrdma_tx_stats,
    pub tx_qp_err_stats: ocrdma_tx_qp_err_stats,
    pub rx_stats: ocrdma_rx_stats,
    pub rx_qp_err_stats: ocrdma_rx_qp_err_stats,
    pub tx_dbg_stats: ocrdma_tx_dbg_stats,
    pub rx_dbg_stats: ocrdma_rx_dbg_stats,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_hba_attribs {
    pub flashrom_version_string: [u8; 32],
    pub manufacturer_name: [u8; 32],
    pub supported_modes: u32,
    pub rsvd_eprom_verhi_verlo: u32,
    pub mbx_ds_ver: u32,
    pub epfw_ds_ver: u32,
    pub ncsi_ver_string: [u8; 12],
    pub default_extended_timeout: u32,
    pub controller_model_number: [u8; 32],
    pub controller_description: [u8; 64],
    pub controller_serial_number: [u8; 32],
    pub ip_version_string: [u8; 32],
    pub firmware_version_string: [u8; 32],
    pub bios_version_string: [u8; 32],
    pub redboot_version_string: [u8; 32],
    pub driver_version_string: [u8; 32],
    pub fw_on_flash_version_string: [u8; 32],
    pub functionalities_supported: u32,
    pub guid0_asicrev_cdblen: u32,
    pub generational_guid: [u8; 12],
    pub portcnt_guid15: u32,
    pub mfuncdev_iscsi_ldtout: u32,
    pub ptpnum_maxdoms_hbast_cv: u32,
    pub firmware_post_status: u32,
    pub hba_mtu: [u32; 8],
    pub res_asicgen_iscsi_feaures: u32,
    pub rsvd1: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_controller_attrib {
    pub hba_attribs: mgmt_hba_attribs,
    pub pci_did_vid: u32,
    pub pci_ssid_svid: u32,
    pub ityp_fnum_devnum_bnum: u32,
    pub uid_hi: u32,
    pub uid_lo: u32,
    pub res_nnetfil: u32,
    pub rsvd0: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_get_ctrl_attribs_rsp {
    pub hdr: ocrdma_mbx_hdr,
    pub ctrl_attribs: mgmt_controller_attrib,
}

pub const OCRDMA_SUBSYS_DCBX: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OCRDMA_DCBX_OPCODE {
    OCRDMA_CMD_GET_DCBX_CONFIG = 0x01
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OCRDMA_DCBX_PARAM_TYPE {
    OCRDMA_PARAMETER_TYPE_ADMIN	= 0x00,
    OCRDMA_PARAMETER_TYPE_OPER	= 0x01,
    OCRDMA_PARAMETER_TYPE_PEER	= 0x02
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OCRDMA_DCBX_PROTO {
    OCRDMA_PROTO_SELECT_L2	= 0x00,
    OCRDMA_PROTO_SELECT_L4	= 0x01
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OCRDMA_DCBX_APP_PARAM {
    OCRDMA_APP_PARAM_APP_PROTO_MASK = 0xFFFF,
    OCRDMA_APP_PARAM_PROTO_SEL_MASK = 0xFF,
    OCRDMA_APP_PARAM_PROTO_SEL_SHIFT = 0x10,
    OCRDMA_APP_PARAM_VALID_MASK	= 0xFF,
    OCRDMA_APP_PARAM_VALID_SHIFT	= 0x18
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OCRDMA_DCBX_STATE_FLAGS {
    OCRDMA_STATE_FLAG_ENABLED	= 0x01,
    OCRDMA_STATE_FLAG_ADDVERTISED	= 0x02,
    OCRDMA_STATE_FLAG_WILLING	= 0x04,
    OCRDMA_STATE_FLAG_SYNC		= 0x08,
    OCRDMA_STATE_FLAG_UNSUPPORTED	= 0x40000000,
    OCRDMA_STATE_FLAG_NEG_FAILD	= 0x80000000
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OCRDMA_TCV_AEV_OPV_ST {
    OCRDMA_DCBX_TC_SUPPORT_MASK	= 0xFF,
    OCRDMA_DCBX_TC_SUPPORT_SHIFT	= 0x18,
    OCRDMA_DCBX_APP_ENTRY_SHIFT	= 0x10,
    OCRDMA_DCBX_OP_PARAM_SHIFT	= 0x08,
    OCRDMA_DCBX_STATE_MASK		= 0xFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_app_parameter {
    pub valid_proto_app: u32,
    pub oui: u32,
    pub app_prio: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_dcbx_cfg {
    pub tcv_aev_opv_st: u32,
    pub tc_state: u32,
    pub pfc_state: u32,
    pub qcn_state: u32,
    pub appl_state: u32,
    pub ll_state: u32,
    pub tc_bw: [u32; 2],
    pub tc_prio: [u32; 8],
    pub pfc_prio: [u32; 2],
    pub app_param: [ocrdma_app_parameter; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_get_dcbx_cfg_req {
    pub hdr: ocrdma_mbx_hdr,
    pub param_type: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_get_dcbx_cfg_rsp {
    pub hdr: ocrdma_mbx_rsp,
    pub cfg: ocrdma_dcbx_cfg,
    pub __packed: },
