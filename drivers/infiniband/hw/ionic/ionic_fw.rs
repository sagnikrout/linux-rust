//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/ionic/ionic_fw.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018-2025, Advanced Micro Devices, Inc.

// common for ib spec
pub const IONIC_EXP_DBELL_SZ: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_mrid_bits {
    IONIC_MRID_INDEX_SHIFT		= 8,
}

// common to all versions
// wqe scatter gather element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_sge {
    pub va: __be64,
    pub len: __be32,
    pub lkey: __be32,
}

// admin queue mr type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_mr_flags {
// bits that determine mr access
    IONIC_MRF_LOCAL_WRITE		= BIT(0),
    IONIC_MRF_REMOTE_WRITE		= BIT(1),
    IONIC_MRF_REMOTE_READ		= BIT(2),
    IONIC_MRF_REMOTE_ATOMIC		= BIT(3),
    IONIC_MRF_MW_BIND		= BIT(4),
    IONIC_MRF_ZERO_BASED		= BIT(5),
    IONIC_MRF_ON_DEMAND		= BIT(6),
    IONIC_MRF_PB			= BIT(7),
    IONIC_MRF_ACCESS_MASK		= BIT(12) - 1,

// bits that determine mr type
    IONIC_MRF_UKEY_EN		= BIT(13),
    IONIC_MRF_IS_MW			= BIT(14),
    IONIC_MRF_INV_EN		= BIT(15),

// base flags combinations for mr types
    IONIC_MRF_USER_MR		= 0,
    IONIC_MRF_PHYS_MR		= (IONIC_MRF_UKEY_EN |
    IONIC_MRF_INV_EN),
    IONIC_MRF_MW_1			= (IONIC_MRF_UKEY_EN |
    IONIC_MRF_IS_MW),
    IONIC_MRF_MW_2			= (IONIC_MRF_UKEY_EN |
    IONIC_MRF_IS_MW |
    IONIC_MRF_INV_EN),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_qp_flags {
// bits that determine qp access
    IONIC_QPF_REMOTE_WRITE		= BIT(0),
    IONIC_QPF_REMOTE_READ		= BIT(1),
    IONIC_QPF_REMOTE_ATOMIC		= BIT(2),

// bits that determine other qp behavior
    IONIC_QPF_SQ_PB			= BIT(6),
    IONIC_QPF_RQ_PB			= BIT(7),
    IONIC_QPF_SQ_SPEC		= BIT(8),
    IONIC_QPF_RQ_SPEC		= BIT(9),
    IONIC_QPF_REMOTE_PRIVILEGED	= BIT(10),
    IONIC_QPF_SQ_DRAINING		= BIT(11),
    IONIC_QPF_SQD_NOTIFY		= BIT(12),
    IONIC_QPF_SQ_CMB		= BIT(13),
    IONIC_QPF_RQ_CMB		= BIT(14),
    IONIC_QPF_PRIVILEGED		= BIT(15),
}

// cqe non-admin status indicated in status_length field when err bit is set
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_status {
    IONIC_STS_OK,
    IONIC_STS_LOCAL_LEN_ERR,
    IONIC_STS_LOCAL_QP_OPER_ERR,
    IONIC_STS_LOCAL_PROT_ERR,
    IONIC_STS_WQE_FLUSHED_ERR,
    IONIC_STS_MEM_MGMT_OPER_ERR,
    IONIC_STS_BAD_RESP_ERR,
    IONIC_STS_LOCAL_ACC_ERR,
    IONIC_STS_REMOTE_INV_REQ_ERR,
    IONIC_STS_REMOTE_ACC_ERR,
    IONIC_STS_REMOTE_OPER_ERR,
    IONIC_STS_RETRY_EXCEEDED,
    IONIC_STS_RNR_RETRY_EXCEEDED,
    IONIC_STS_XRC_VIO_ERR,
    IONIC_STS_LOCAL_SGL_INV_ERR,
}

// admin queue qp type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_qp_type {
    IONIC_QPT_RC,
    IONIC_QPT_UC,
    IONIC_QPT_RD,
    IONIC_QPT_UD,
    IONIC_QPT_SRQ,
    IONIC_QPT_XRC_INI,
    IONIC_QPT_XRC_TGT,
    IONIC_QPT_XRC_SRQ,
}

// admin queue qp state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_qp_state {
    IONIC_QPS_RESET,
    IONIC_QPS_INIT,
    IONIC_QPS_RTR,
    IONIC_QPS_RTS,
    IONIC_QPS_SQD,
    IONIC_QPS_SQE,
    IONIC_QPS_ERR,
}

// fw abi v1
// data payload part of v1 wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_v1_pld {
    pub sgl: [ionic_sge; 2],
    pub spec32: [__be32; 8],
    pub spec16: [__be16; 16],
    pub data: [__u8; 32],
}

// completion queue v1 cqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_cqe {
    pub cmd_idx: __be16,
    pub cmd_op: __u8,
    pub rsvd: [__u8; 17],
    pub old_sq_cindex: __le16,
    pub old_rq_cq_cindex: __le16,
    pub admin: },
    pub wqe_idx_timestamp: __le64,
    pub src_qpn_op: __be32,
    pub src_mac: [__u8; 6],
    pub vlan_tag: __be16,
    pub imm_data_rkey: __be32,
    pub recv: },
    pub rsvd: [__u8; 4],
    pub msg_msn: __be32,
    pub rsvd2: [__u8; 8],
    pub npg_wqe_idx_timestamp: __le64,
    pub send: },
}

// bits for cqe wqe_idx and timestamp
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_v1_cqe_wqe_idx_timestamp_bits {
    IONIC_V1_CQE_WQE_IDX_MASK	= 0xffff,
    IONIC_V1_CQE_TIMESTAMP_SHIFT	= 16,
}

// bits for cqe recv
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_v1_cqe_src_qpn_bits {
    IONIC_V1_CQE_RECV_QPN_MASK	= 0xffffff,
    IONIC_V1_CQE_RECV_OP_SHIFT	= 24,

// MASK could be 0x3, but need 0x1f for makeshift values:
// OP_TYPE_RDMA_OPER_WITH_IMM, OP_TYPE_SEND_RCVD
//
    IONIC_V1_CQE_RECV_OP_MASK	= 0x1f,
    IONIC_V1_CQE_RECV_OP_SEND	= 0,
    IONIC_V1_CQE_RECV_OP_SEND_INV	= 1,
    IONIC_V1_CQE_RECV_OP_SEND_IMM	= 2,
    IONIC_V1_CQE_RECV_OP_RDMA_IMM	= 3,

    IONIC_V1_CQE_RECV_IS_IPV4	= BIT(7 + IONIC_V1_CQE_RECV_OP_SHIFT),
    IONIC_V1_CQE_RECV_IS_VLAN	= BIT(6 + IONIC_V1_CQE_RECV_OP_SHIFT),
}

// bits for cqe qid_type_flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_v1_cqe_qtf_bits {
    IONIC_V1_CQE_COLOR		= BIT(0),
    IONIC_V1_CQE_ERROR		= BIT(1),
    IONIC_V1_CQE_TYPE_SHIFT		= 5,
    IONIC_V1_CQE_TYPE_MASK		= 0x7,
    IONIC_V1_CQE_QID_SHIFT		= 8,

    IONIC_V1_CQE_TYPE_ADMIN		= 0,
    IONIC_V1_CQE_TYPE_RECV		= 1,
    IONIC_V1_CQE_TYPE_SEND_MSN	= 2,
    IONIC_V1_CQE_TYPE_SEND_NPG	= 3,
}

extern "C" {
    pub fn be32_to_cpu(_arg: cqe->qid_type_flags) -> return;
}
// v1 base wqe header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_base_hdr {
    pub wqe_idx: __le64,
    pub op: __u8,
    pub num_sge_key: __u8,
    pub flags: __be16,
    pub imm_data_key: __be32,
}

// v1 receive wqe body
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_recv_bdy {
    pub rsvd: [__u8; 16],
    pub pld: ionic_v1_pld,
}

// v1 send/rdma wqe body (common, has sgl)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_common_bdy {
    pub ah_id: __be32,
    pub dest_qpn: __be32,
    pub dest_qkey: __be32,
    pub send: },
    pub remote_va_high: __be32,
    pub remote_va_low: __be32,
    pub remote_rkey: __be32,
    pub rdma: },
}

// v1 atomic wqe body
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_atomic_bdy {
    pub remote_va_high: __be32,
    pub remote_va_low: __be32,
    pub remote_rkey: __be32,
    pub swap_add_high: __be32,
    pub swap_add_low: __be32,
    pub compare_high: __be32,
    pub compare_low: __be32,
    pub rsvd: [__u8; 4],
    pub sge: ionic_sge,
}

// v1 reg mr wqe body
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_reg_mr_bdy {
    pub va: __be64,
    pub length: __be64,
    pub offset: __be64,
    pub dma_addr: __be64,
    pub map_count: __be32,
    pub flags: __be16,
    pub dir_size_log2: __u8,
    pub page_size_log2: __u8,
    pub rsvd: [__u8; 8],
}

// v1 bind mw wqe body
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_bind_mw_bdy {
    pub va: __be64,
    pub length: __be64,
    pub lkey: __be32,
    pub flags: __be16,
    pub rsvd: [__u8; 26],
}

// v1 send/recv wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_wqe {
    pub base: ionic_v1_base_hdr,
    pub recv: ionic_v1_recv_bdy,
    pub common: ionic_v1_common_bdy,
    pub atomic: ionic_v1_atomic_bdy,
    pub reg_mr: ionic_v1_reg_mr_bdy,
    pub bind_mw: ionic_v1_bind_mw_bdy,
}

// queue pair v1 send opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_v1_op {
    IONIC_V1_OP_SEND,
    IONIC_V1_OP_SEND_INV,
    IONIC_V1_OP_SEND_IMM,
    IONIC_V1_OP_RDMA_READ,
    IONIC_V1_OP_RDMA_WRITE,
    IONIC_V1_OP_RDMA_WRITE_IMM,
    IONIC_V1_OP_ATOMIC_CS,
    IONIC_V1_OP_ATOMIC_FA,
    IONIC_V1_OP_REG_MR,
    IONIC_V1_OP_LOCAL_INV,
    IONIC_V1_OP_BIND_MW,

// flags
    IONIC_V1_FLAG_FENCE		= BIT(0),
    IONIC_V1_FLAG_SOL		= BIT(1),
    IONIC_V1_FLAG_INL		= BIT(2),
    IONIC_V1_FLAG_SIG		= BIT(3),

// flags last four bits for sgl spec format
    IONIC_V1_FLAG_SPEC32		= (1u << 12),
    IONIC_V1_FLAG_SPEC16		= (2u << 12),
    IONIC_V1_SPEC_FIRST_SGE		= 2,
}

// queue pair v2 send opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_v2_op {
    IONIC_V2_OPSL_OUT          = 0x20,
    IONIC_V2_OPSL_IMM          = 0x40,
    IONIC_V2_OPSL_INV          = 0x80,

    IONIC_V2_OP_SEND           = 0x0 | IONIC_V2_OPSL_OUT,
    IONIC_V2_OP_SEND_IMM       = IONIC_V2_OP_SEND | IONIC_V2_OPSL_IMM,
    IONIC_V2_OP_SEND_INV       = IONIC_V2_OP_SEND | IONIC_V2_OPSL_INV,

    IONIC_V2_OP_RDMA_WRITE     = 0x1 | IONIC_V2_OPSL_OUT,
    IONIC_V2_OP_RDMA_WRITE_IMM = IONIC_V2_OP_RDMA_WRITE | IONIC_V2_OPSL_IMM,

    IONIC_V2_OP_RDMA_READ      = 0x2,

    IONIC_V2_OP_ATOMIC_CS      = 0x4,
    IONIC_V2_OP_ATOMIC_FA      = 0x5,
    IONIC_V2_OP_REG_MR         = 0x6,
    IONIC_V2_OP_LOCAL_INV      = 0x7,
    IONIC_V2_OP_BIND_MW        = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_stats_hdr {
    pub dma_addr: __le64,
    pub length: __le32,
    pub id_ver: __le32,
    pub type_state: __u8,
    pub __packed: },
pub const IONIC_ADMIN_STATS_HDRS_IN_V1_LEN: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_create_ah {
    pub dma_addr: __le64,
    pub length: __le32,
    pub pd_id: __le32,
    pub id_ver: __le32,
    pub dbid_flags: __le16,
    pub csum_profile: __u8,
    pub crypto: __u8,
    pub __packed: },
pub const IONIC_ADMIN_CREATE_AH_IN_V1_LEN: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_destroy_ah {
    pub ah_id: __le32,
    pub __packed: },
pub const IONIC_ADMIN_DESTROY_AH_IN_V1_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_query_ah {
    pub dma_addr: __le64,
    pub __packed: },
pub const IONIC_ADMIN_QUERY_AH_IN_V1_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_create_mr {
    pub va: __le64,
    pub length: __le64,
    pub pd_id: __le32,
    pub id_ver: __le32,
    pub tbl_index: __le32,
    pub map_count: __le32,
    pub dma_addr: __le64,
    pub dbid_flags: __le16,
    pub pt_type: __u8,
    pub dir_size_log2: __u8,
    pub page_size_log2: __u8,
    pub __packed: },
pub const IONIC_ADMIN_CREATE_MR_IN_V1_LEN: c_int = 45;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_destroy_mr {
    pub mr_id: __le32,
    pub __packed: },
pub const IONIC_ADMIN_DESTROY_MR_IN_V1_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_create_cq {
    pub eq_id: __le32,
    pub depth_log2: __u8,
    pub stride_log2: __u8,
    pub dir_size_log2_rsvd: __u8,
    pub page_size_log2: __u8,
    pub cq_flags: __le32,
    pub id_ver: __le32,
    pub tbl_index: __le32,
    pub map_count: __le32,
    pub dma_addr: __le64,
    pub dbid_flags: __le16,
    pub __packed: },
pub const IONIC_ADMIN_CREATE_CQ_IN_V1_LEN: c_int = 34;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_destroy_cq {
    pub cq_id: __le32,
    pub __packed: },
pub const IONIC_ADMIN_DESTROY_CQ_IN_V1_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_create_qp {
    pub pd_id: __le32,
    pub priv_flags: __be32,
    pub sq_cq_id: __le32,
    pub sq_depth_log2: __u8,
    pub sq_stride_log2: __u8,
    pub sq_dir_size_log2_rsvd: __u8,
    pub sq_page_size_log2: __u8,
    pub sq_tbl_index_xrcd_id: __le32,
    pub sq_map_count: __le32,
    pub sq_dma_addr: __le64,
    pub rq_cq_id: __le32,
    pub rq_depth_log2: __u8,
    pub rq_stride_log2: __u8,
    pub rq_dir_size_log2_rsvd: __u8,
    pub rq_page_size_log2: __u8,
    pub rq_tbl_index_srq_id: __le32,
    pub rq_map_count: __le32,
    pub rq_dma_addr: __le64,
    pub id_ver: __le32,
    pub dbid_flags: __le16,
    pub type_state: __u8,
    pub rsvd: __u8,
    pub __packed: },
pub const IONIC_ADMIN_CREATE_QP_IN_V1_LEN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_destroy_qp {
    pub qp_id: __le32,
    pub __packed: },
pub const IONIC_ADMIN_DESTROY_QP_IN_V1_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_mod_qp {
    pub attr_mask: __be32,
    pub dcqcn_profile: __u8,
    pub tfp_csum_profile: __u8,
    pub access_flags: __be16,
    pub rq_psn: __le32,
    pub sq_psn: __le32,
    pub qkey_dest_qpn: __le32,
    pub rate_limit_kbps: __le32,
    pub pmtu: __u8,
    pub retry: __u8,
    pub rnr_timer: __u8,
    pub retry_timeout: __u8,
    pub rsq_depth: __u8,
    pub rrq_depth: __u8,
    pub pkey_id: __le16,
    pub ah_id_len: __le32,
    pub en_pcp: __u8,
    pub ip_dscp: __u8,
    pub rsvd2: __u8,
    pub type_state: __u8,
    pub rsvd1: __le16,
}

pub const IONIC_ADMIN_MODIFY_QP_IN_V1_LEN: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_query_qp {
    pub hdr_dma_addr: __le64,
    pub sq_dma_addr: __le64,
    pub rq_dma_addr: __le64,
    pub ah_id: __le32,
    pub id_ver: __le32,
    pub dbid_flags: __le16,
    pub __packed: },
pub const IONIC_ADMIN_QUERY_QP_IN_V1_LEN: c_int = 34;
pub const ADMIN_WQE_STRIDE: c_int = 64;
pub const ADMIN_WQE_HDR_LEN: c_int = 4;
// admin queue v1 wqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_admin_wqe {
    pub op: __u8,
    pub rsvd: __u8,
    pub len: __le16,
    pub stats: ionic_admin_stats_hdr,
    pub create_ah: ionic_admin_create_ah,
    pub destroy_ah: ionic_admin_destroy_ah,
    pub query_ah: ionic_admin_query_ah,
    pub create_mr: ionic_admin_create_mr,
    pub destroy_mr: ionic_admin_destroy_mr,
    pub create_cq: ionic_admin_create_cq,
    pub destroy_cq: ionic_admin_destroy_cq,
    pub create_qp: ionic_admin_create_qp,
    pub destroy_qp: ionic_admin_destroy_qp,
    pub mod_qp: ionic_admin_mod_qp,
    pub query_qp: ionic_admin_query_qp,
    pub cmd: },
}

// side data for query qp
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_admin_query_qp_sq {
    pub rnr_timer: __u8,
    pub retry_timeout: __u8,
    pub access_perms_flags: __be16,
    pub rsvd: __be16,
    pub pkey_id: __be16,
    pub qkey_dest_qpn: __be32,
    pub rate_limit_kbps: __be32,
    pub rq_psn: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_admin_query_qp_rq {
    pub state_pmtu: __u8,
    pub retry_rnrtry: __u8,
    pub rrq_depth: __u8,
    pub rsq_depth: __u8,
    pub sq_psn: __be32,
    pub access_perms_flags: __be16,
    pub rsvd: __be16,
}

// admin queue v1 opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_v1_admin_op {
    IONIC_V1_ADMIN_NOOP,
    IONIC_V1_ADMIN_CREATE_CQ,
    IONIC_V1_ADMIN_CREATE_QP,
    IONIC_V1_ADMIN_CREATE_MR,
    IONIC_V1_ADMIN_STATS_HDRS,
    IONIC_V1_ADMIN_STATS_VALS,
    IONIC_V1_ADMIN_DESTROY_MR,
    IONIC_V1_ADMIN_RSVD_7,		/* RESIZE_CQ */
    IONIC_V1_ADMIN_DESTROY_CQ,
    IONIC_V1_ADMIN_MODIFY_QP,
    IONIC_V1_ADMIN_QUERY_QP,
    IONIC_V1_ADMIN_DESTROY_QP,
    IONIC_V1_ADMIN_DEBUG,
    IONIC_V1_ADMIN_CREATE_AH,
    IONIC_V1_ADMIN_QUERY_AH,
    IONIC_V1_ADMIN_MODIFY_DCQCN,
    IONIC_V1_ADMIN_DESTROY_AH,
    IONIC_V1_ADMIN_QP_STATS_HDRS,
    IONIC_V1_ADMIN_QP_STATS_VALS,
    IONIC_V1_ADMIN_OPCODES_MAX,
}

// admin queue v1 cqe status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_v1_admin_status {
    IONIC_V1_ASTS_OK,
    IONIC_V1_ASTS_BAD_CMD,
    IONIC_V1_ASTS_BAD_INDEX,
    IONIC_V1_ASTS_BAD_STATE,
    IONIC_V1_ASTS_BAD_TYPE,
    IONIC_V1_ASTS_BAD_ATTR,
    IONIC_V1_ASTS_MSG_TOO_BIG,
}

// event queue v1 eqe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_eqe {
    pub evt: __be32,
}

// bits for cqe queue_type_flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_v1_eqe_evt_bits {
    IONIC_V1_EQE_COLOR		= BIT(0),
    IONIC_V1_EQE_TYPE_SHIFT		= 1,
    IONIC_V1_EQE_TYPE_MASK		= 0x7,
    IONIC_V1_EQE_CODE_SHIFT		= 4,
    IONIC_V1_EQE_CODE_MASK		= 0xf,
    IONIC_V1_EQE_QID_SHIFT		= 8,

// cq events
    IONIC_V1_EQE_TYPE_CQ		= 0,
// cq normal events
    IONIC_V1_EQE_CQ_NOTIFY		= 0,
// cq error events
    IONIC_V1_EQE_CQ_ERR		= 8,

// qp and srq events
    IONIC_V1_EQE_TYPE_QP		= 1,
// qp normal events
    IONIC_V1_EQE_SRQ_LEVEL		= 0,
    IONIC_V1_EQE_SQ_DRAIN		= 1,
    IONIC_V1_EQE_QP_COMM_EST	= 2,
    IONIC_V1_EQE_QP_LAST_WQE	= 3,
// qp error events
    IONIC_V1_EQE_QP_ERR		= 8,
    IONIC_V1_EQE_QP_ERR_REQUEST	= 9,
    IONIC_V1_EQE_QP_ERR_ACCESS	= 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_tfp_csum_profiles {
    IONIC_TFP_CSUM_PROF_ETH_IPV4_UDP				= 0,
    IONIC_TFP_CSUM_PROF_ETH_QTAG_IPV4_UDP				= 1,
    IONIC_TFP_CSUM_PROF_ETH_IPV6_UDP				= 2,
    IONIC_TFP_CSUM_PROF_ETH_QTAG_IPV6_UDP				= 3,
    IONIC_TFP_CSUM_PROF_IPV4_UDP_VXLAN_ETH_QTAG_IPV4_UDP		= 4,
    IONIC_TFP_CSUM_PROF_IPV4_UDP_VXLAN_ETH_QTAG_IPV6_UDP		= 5,
    IONIC_TFP_CSUM_PROF_QTAG_IPV4_UDP_VXLAN_ETH_QTAG_IPV4_UDP	= 6,
    IONIC_TFP_CSUM_PROF_QTAG_IPV4_UDP_VXLAN_ETH_QTAG_IPV6_UDP	= 7,
    IONIC_TFP_CSUM_PROF_ETH_QTAG_IPV4_UDP_ESP_IPV4_UDP		= 8,
    IONIC_TFP_CSUM_PROF_ETH_QTAG_IPV4_ESP_UDP			= 9,
    IONIC_TFP_CSUM_PROF_ETH_QTAG_IPV4_UDP_ESP_UDP			= 10,
    IONIC_TFP_CSUM_PROF_ETH_QTAG_IPV6_ESP_UDP			= 11,
    IONIC_TFP_CSUM_PROF_ETH_QTAG_IPV4_UDP_CSUM			= 12,
}

extern "C" {
    pub fn be32_to_cpu(_arg: eqe->evt) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_v1_stat_bits {
    IONIC_V1_STAT_TYPE_SHIFT	= 28,
    IONIC_V1_STAT_TYPE_NONE		= 0,
    IONIC_V1_STAT_TYPE_8		= 1,
    IONIC_V1_STAT_TYPE_LE16		= 2,
    IONIC_V1_STAT_TYPE_LE32		= 3,
    IONIC_V1_STAT_TYPE_LE64		= 4,
    IONIC_V1_STAT_TYPE_BE16		= 5,
    IONIC_V1_STAT_TYPE_BE32		= 6,
    IONIC_V1_STAT_TYPE_BE64		= 7,
    IONIC_V1_STAT_OFF_MASK		= BIT(IONIC_V1_STAT_TYPE_SHIFT) - 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_v1_stat {
    pub be_type_off: __be32,
    pub type_off: u32,
}
