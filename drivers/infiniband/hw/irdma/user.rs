//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/user.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2015 - 2020 Intel Corporation

pub const IRDMA_MAX_MR_SIZE: c_uint = 0x200000000000ULL;
pub const IRDMA_ACCESS_FLAGS_LOCALREAD: c_uint = 0x01;
pub const IRDMA_ACCESS_FLAGS_LOCALWRITE: c_uint = 0x02;
pub const IRDMA_ACCESS_FLAGS_REMOTEREAD_ONLY: c_uint = 0x04;
pub const IRDMA_ACCESS_FLAGS_REMOTEREAD: c_uint = 0x05;
pub const IRDMA_ACCESS_FLAGS_REMOTEWRITE_ONLY: c_uint = 0x08;
pub const IRDMA_ACCESS_FLAGS_REMOTEWRITE: c_uint = 0x0a;
pub const IRDMA_ACCESS_FLAGS_BIND_WINDOW: c_uint = 0x10;
pub const IRDMA_ACCESS_FLAGS_ZERO_BASED: c_uint = 0x20;
pub const IRDMA_ACCESS_FLAGS_ALL: c_uint = 0x3f;
pub const IRDMA_OP_TYPE_RDMA_WRITE: c_uint = 0x00;
pub const IRDMA_OP_TYPE_RDMA_READ: c_uint = 0x01;
pub const IRDMA_OP_TYPE_SEND: c_uint = 0x03;
pub const IRDMA_OP_TYPE_SEND_INV: c_uint = 0x04;
pub const IRDMA_OP_TYPE_SEND_SOL: c_uint = 0x05;
pub const IRDMA_OP_TYPE_SEND_SOL_INV: c_uint = 0x06;
pub const IRDMA_OP_TYPE_RDMA_WRITE_SOL: c_uint = 0x0d;
pub const IRDMA_OP_TYPE_BIND_MW: c_uint = 0x08;
pub const IRDMA_OP_TYPE_FAST_REG_NSMR: c_uint = 0x09;
pub const IRDMA_OP_TYPE_INV_STAG: c_uint = 0x0a;
pub const IRDMA_OP_TYPE_RDMA_READ_INV_STAG: c_uint = 0x0b;
pub const IRDMA_OP_TYPE_NOP: c_uint = 0x0c;
pub const IRDMA_OP_TYPE_ATOMIC_FETCH_AND_ADD: c_uint = 0x0f;
pub const IRDMA_OP_TYPE_ATOMIC_COMPARE_AND_SWAP: c_uint = 0x11;
pub const IRDMA_OP_TYPE_REC: c_uint = 0x3e;
pub const IRDMA_OP_TYPE_REC_IMM: c_uint = 0x3f;
pub const IRDMA_FLUSH_MAJOR_ERR: c_int = 1;
pub const IRDMA_SRQFLUSH_RSVD_MAJOR_ERR: c_uint = 0xfffe;
// Async Events codes
pub const IRDMA_AE_AMP_UNALLOCATED_STAG: c_uint = 0x0102;
pub const IRDMA_AE_AMP_INVALID_STAG: c_uint = 0x0103;
pub const IRDMA_AE_AMP_BAD_QP: c_uint = 0x0104;
pub const IRDMA_AE_AMP_BAD_PD: c_uint = 0x0105;
pub const IRDMA_AE_AMP_BAD_STAG_KEY: c_uint = 0x0106;
pub const IRDMA_AE_AMP_BAD_STAG_INDEX: c_uint = 0x0107;
pub const IRDMA_AE_AMP_BOUNDS_VIOLATION: c_uint = 0x0108;
pub const IRDMA_AE_AMP_RIGHTS_VIOLATION: c_uint = 0x0109;
pub const IRDMA_AE_AMP_TO_WRAP: c_uint = 0x010a;
pub const IRDMA_AE_AMP_FASTREG_VALID_STAG: c_uint = 0x010c;
pub const IRDMA_AE_AMP_FASTREG_MW_STAG: c_uint = 0x010d;
pub const IRDMA_AE_AMP_FASTREG_INVALID_RIGHTS: c_uint = 0x010e;
pub const IRDMA_AE_AMP_FASTREG_INVALID_LENGTH: c_uint = 0x0110;
pub const IRDMA_AE_AMP_INVALIDATE_SHARED: c_uint = 0x0111;
pub const IRDMA_AE_AMP_INVALIDATE_NO_REMOTE_ACCESS_RIGHTS: c_uint = 0x0112;
pub const IRDMA_AE_AMP_INVALIDATE_MR_WITH_BOUND_WINDOWS: c_uint = 0x0113;
pub const IRDMA_AE_AMP_MWBIND_VALID_STAG: c_uint = 0x0114;
pub const IRDMA_AE_AMP_MWBIND_OF_MR_STAG: c_uint = 0x0115;
pub const IRDMA_AE_AMP_MWBIND_TO_ZERO_BASED_STAG: c_uint = 0x0116;
pub const IRDMA_AE_AMP_MWBIND_TO_MW_STAG: c_uint = 0x0117;
pub const IRDMA_AE_AMP_MWBIND_INVALID_RIGHTS: c_uint = 0x0118;
pub const IRDMA_AE_AMP_MWBIND_INVALID_BOUNDS: c_uint = 0x0119;
pub const IRDMA_AE_AMP_MWBIND_TO_INVALID_PARENT: c_uint = 0x011a;
pub const IRDMA_AE_AMP_MWBIND_BIND_DISABLED: c_uint = 0x011b;
pub const IRDMA_AE_PRIV_OPERATION_DENIED: c_uint = 0x011c;
pub const IRDMA_AE_AMP_INVALIDATE_TYPE1_MW: c_uint = 0x011d;
pub const IRDMA_AE_AMP_MWBIND_ZERO_BASED_TYPE1_MW: c_uint = 0x011e;
pub const IRDMA_AE_AMP_FASTREG_INVALID_PBL_HPS_CFG: c_uint = 0x011f;
pub const IRDMA_AE_AMP_MWBIND_WRONG_TYPE: c_uint = 0x0120;
pub const IRDMA_AE_AMP_FASTREG_PBLE_MISMATCH: c_uint = 0x0121;
pub const IRDMA_AE_UDA_XMIT_DGRAM_TOO_LONG: c_uint = 0x0132;
pub const IRDMA_AE_UDA_XMIT_BAD_PD: c_uint = 0x0133;
pub const IRDMA_AE_UDA_XMIT_DGRAM_TOO_SHORT: c_uint = 0x0134;
pub const IRDMA_AE_UDA_L4LEN_INVALID: c_uint = 0x0135;
pub const IRDMA_AE_BAD_CLOSE: c_uint = 0x0201;
pub const IRDMA_AE_RDMAP_ROE_BAD_LLP_CLOSE: c_uint = 0x0202;
pub const IRDMA_AE_CQ_OPERATION_ERROR: c_uint = 0x0203;
pub const IRDMA_AE_RDMA_READ_WHILE_ORD_ZERO: c_uint = 0x0205;
pub const IRDMA_AE_STAG_ZERO_INVALID: c_uint = 0x0206;
pub const IRDMA_AE_IB_RREQ_AND_Q1_FULL: c_uint = 0x0207;
pub const IRDMA_AE_IB_INVALID_REQUEST: c_uint = 0x0208;
pub const IRDMA_AE_SRQ_LIMIT: c_uint = 0x0209;
pub const IRDMA_AE_WQE_UNEXPECTED_OPCODE: c_uint = 0x020a;
pub const IRDMA_AE_WQE_INVALID_PARAMETER: c_uint = 0x020b;
pub const IRDMA_AE_WQE_INVALID_FRAG_DATA: c_uint = 0x020c;
pub const IRDMA_AE_IB_REMOTE_ACCESS_ERROR: c_uint = 0x020d;
pub const IRDMA_AE_IB_REMOTE_OP_ERROR: c_uint = 0x020e;
pub const IRDMA_AE_SRQ_CATASTROPHIC_ERROR: c_uint = 0x020f;
pub const IRDMA_AE_WQE_LSMM_TOO_LONG: c_uint = 0x0220;
pub const IRDMA_AE_ATOMIC_ALIGNMENT: c_uint = 0x0221;
pub const IRDMA_AE_ATOMIC_MASK: c_uint = 0x0222;
pub const IRDMA_AE_INVALID_REQUEST: c_uint = 0x0223;
pub const IRDMA_AE_PCIE_ATOMIC_DISABLE: c_uint = 0x0224;
pub const IRDMA_AE_DDP_INVALID_MSN_GAP_IN_MSN: c_uint = 0x0301;
pub const IRDMA_AE_DDP_UBE_DDP_MESSAGE_TOO_LONG_FOR_AVAILABLE_BUFFER: c_uint = 0x0303;
pub const IRDMA_AE_DDP_UBE_INVALID_DDP_VERSION: c_uint = 0x0304;
pub const IRDMA_AE_DDP_UBE_INVALID_MO: c_uint = 0x0305;
pub const IRDMA_AE_DDP_UBE_INVALID_MSN_NO_BUFFER_AVAILABLE: c_uint = 0x0306;
pub const IRDMA_AE_DDP_UBE_INVALID_QN: c_uint = 0x0307;
pub const IRDMA_AE_DDP_NO_L_BIT: c_uint = 0x0308;
pub const IRDMA_AE_RDMAP_ROE_INVALID_RDMAP_VERSION: c_uint = 0x0311;
pub const IRDMA_AE_RDMAP_ROE_UNEXPECTED_OPCODE: c_uint = 0x0312;
pub const IRDMA_AE_ROE_INVALID_RDMA_READ_REQUEST: c_uint = 0x0313;
pub const IRDMA_AE_ROE_INVALID_RDMA_WRITE_OR_READ_RESP: c_uint = 0x0314;
pub const IRDMA_AE_ROCE_RSP_LENGTH_ERROR: c_uint = 0x0316;
pub const IRDMA_AE_ROCE_EMPTY_MCG: c_uint = 0x0380;
pub const IRDMA_AE_ROCE_BAD_MC_IP_ADDR: c_uint = 0x0381;
pub const IRDMA_AE_ROCE_BAD_MC_QPID: c_uint = 0x0382;
pub const IRDMA_AE_MCG_QP_PROTOCOL_MISMATCH: c_uint = 0x0383;
pub const IRDMA_AE_INVALID_ARP_ENTRY: c_uint = 0x0401;
pub const IRDMA_AE_INVALID_TCP_OPTION_RCVD: c_uint = 0x0402;
pub const IRDMA_AE_STALE_ARP_ENTRY: c_uint = 0x0403;
pub const IRDMA_AE_INVALID_AH_ENTRY: c_uint = 0x0406;
pub const IRDMA_AE_LLP_CLOSE_COMPLETE: c_uint = 0x0501;
pub const IRDMA_AE_LLP_CONNECTION_RESET: c_uint = 0x0502;
pub const IRDMA_AE_LLP_FIN_RECEIVED: c_uint = 0x0503;
pub const IRDMA_AE_LLP_RECEIVED_MARKER_AND_LENGTH_FIELDS_DONT_MATCH: c_uint = 0x0504;
pub const IRDMA_AE_LLP_RECEIVED_MPA_CRC_ERROR: c_uint = 0x0505;
pub const IRDMA_AE_LLP_SEGMENT_TOO_SMALL: c_uint = 0x0507;
pub const IRDMA_AE_LLP_SYN_RECEIVED: c_uint = 0x0508;
pub const IRDMA_AE_LLP_TERMINATE_RECEIVED: c_uint = 0x0509;
pub const IRDMA_AE_LLP_TOO_MANY_RETRIES: c_uint = 0x050a;
pub const IRDMA_AE_LLP_TOO_MANY_KEEPALIVE_RETRIES: c_uint = 0x050b;
pub const IRDMA_AE_LLP_DOUBT_REACHABILITY: c_uint = 0x050c;
pub const IRDMA_AE_LLP_CONNECTION_ESTABLISHED: c_uint = 0x050e;
pub const IRDMA_AE_LLP_TOO_MANY_RNRS: c_uint = 0x050f;
pub const IRDMA_AE_RESOURCE_EXHAUSTION: c_uint = 0x0520;
pub const IRDMA_AE_RESET_SENT: c_uint = 0x0601;
pub const IRDMA_AE_TERMINATE_SENT: c_uint = 0x0602;
pub const IRDMA_AE_RESET_NOT_SENT: c_uint = 0x0603;
pub const IRDMA_AE_LCE_QP_CATASTROPHIC: c_uint = 0x0700;
pub const IRDMA_AE_LCE_FUNCTION_CATASTROPHIC: c_uint = 0x0701;
pub const IRDMA_AE_LCE_CQ_CATASTROPHIC: c_uint = 0x0702;
pub const IRDMA_AE_REMOTE_QP_CATASTROPHIC: c_uint = 0x0703;
pub const IRDMA_AE_LOCAL_QP_CATASTROPHIC: c_uint = 0x0704;
pub const IRDMA_AE_RCE_QP_CATASTROPHIC: c_uint = 0x0705;
pub const IRDMA_AE_QP_SUSPEND_COMPLETE: c_uint = 0x0900;
pub const IRDMA_AE_CQP_DEFERRED_COMPLETE: c_uint = 0x0901;
pub const IRDMA_AE_ADAPTER_CATASTROPHIC: c_uint = 0x0B0B;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_device_caps_const {
    IRDMA_WQE_SIZE =			4,
    IRDMA_CQP_WQE_SIZE =			8,
    IRDMA_CQE_SIZE =			4,
    IRDMA_EXTENDED_CQE_SIZE =		8,
    IRDMA_AEQE_SIZE =			2,
    IRDMA_CEQE_SIZE =			1,
    IRDMA_CQP_CTX_SIZE =			8,
    IRDMA_SHADOW_AREA_SIZE =		8,
    IRDMA_QUERY_FPM_BUF_SIZE =		200,
    IRDMA_COMMIT_FPM_BUF_SIZE =		208,
    IRDMA_GATHER_STATS_BUF_SIZE =		1024,
    IRDMA_MIN_IW_QP_ID =			0,
    IRDMA_MAX_IW_QP_ID =			262143,
    IRDMA_MIN_IW_SRQ_ID =			0,
    IRDMA_MIN_CEQID =			0,
    IRDMA_MAX_CEQID =			1023,
    IRDMA_CEQ_MAX_COUNT =			IRDMA_MAX_CEQID + 1,
    IRDMA_MIN_CQID =			0,
    IRDMA_MAX_CQID =			524287,
    IRDMA_MIN_AEQ_ENTRIES =			1,
    IRDMA_MAX_AEQ_ENTRIES =			524287,
    IRDMA_MAX_AEQ_ENTRIES_GEN_3 =           262144,
    IRDMA_MIN_CEQ_ENTRIES =			1,
    IRDMA_MAX_CEQ_ENTRIES =			262143,
    IRDMA_MIN_CQ_SIZE =			1,
    IRDMA_MAX_CQ_SIZE =			1048575,
    IRDMA_DB_ID_ZERO =			0,
    IRDMA_MAX_WQ_FRAGMENT_COUNT =		13,
    IRDMA_MAX_SGE_RD =			13,
    IRDMA_MAX_OUTBOUND_MSG_SIZE =		2147483647,
    IRDMA_MAX_INBOUND_MSG_SIZE =		2147483647,
    IRDMA_MAX_PUSH_PAGE_COUNT =		1024,
    IRDMA_MAX_PE_ENA_VF_COUNT =		32,
    IRDMA_MAX_VF_FPM_ID =			47,
    IRDMA_MAX_SQ_PAYLOAD_SIZE =		2145386496,
    IRDMA_MAX_INLINE_DATA_SIZE =		101,
    IRDMA_MAX_WQ_ENTRIES =			32768,
    IRDMA_Q2_BUF_SIZE =			256,
    IRDMA_QP_CTX_SIZE =			256,
    IRDMA_MAX_PDS =				262144,
    IRDMA_MIN_WQ_SIZE_GEN2 =                8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_addressing_type {
    IRDMA_ADDR_TYPE_ZERO_BASED = 0,
    IRDMA_ADDR_TYPE_VA_BASED   = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_flush_opcode {
    FLUSH_INVALID = 0,
    FLUSH_GENERAL_ERR,
    FLUSH_PROT_ERR,
    FLUSH_REM_ACCESS_ERR,
    FLUSH_LOC_QP_OP_ERR,
    FLUSH_REM_OP_ERR,
    FLUSH_LOC_LEN_ERR,
    FLUSH_FATAL_ERR,
    FLUSH_RETRY_EXC_ERR,
    FLUSH_MW_BIND_ERR,
    FLUSH_REM_INV_REQ_ERR,
    FLUSH_RNR_RETRY_EXC_ERR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_qp_event_type {
    IRDMA_QP_EVENT_CATASTROPHIC,
    IRDMA_QP_EVENT_ACCESS_ERR,
    IRDMA_QP_EVENT_REQ_ERR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_cmpl_status {
    IRDMA_COMPL_STATUS_SUCCESS = 0,
    IRDMA_COMPL_STATUS_FLUSHED,
    IRDMA_COMPL_STATUS_INVALID_WQE,
    IRDMA_COMPL_STATUS_QP_CATASTROPHIC,
    IRDMA_COMPL_STATUS_REMOTE_TERMINATION,
    IRDMA_COMPL_STATUS_INVALID_STAG,
    IRDMA_COMPL_STATUS_BASE_BOUND_VIOLATION,
    IRDMA_COMPL_STATUS_ACCESS_VIOLATION,
    IRDMA_COMPL_STATUS_INVALID_PD_ID,
    IRDMA_COMPL_STATUS_WRAP_ERROR,
    IRDMA_COMPL_STATUS_STAG_INVALID_PDID,
    IRDMA_COMPL_STATUS_RDMA_READ_ZERO_ORD,
    IRDMA_COMPL_STATUS_QP_NOT_PRIVLEDGED,
    IRDMA_COMPL_STATUS_STAG_NOT_INVALID,
    IRDMA_COMPL_STATUS_INVALID_PHYS_BUF_SIZE,
    IRDMA_COMPL_STATUS_INVALID_PHYS_BUF_ENTRY,
    IRDMA_COMPL_STATUS_INVALID_FBO,
    IRDMA_COMPL_STATUS_INVALID_LEN,
    IRDMA_COMPL_STATUS_INVALID_ACCESS,
    IRDMA_COMPL_STATUS_PHYS_BUF_LIST_TOO_LONG,
    IRDMA_COMPL_STATUS_INVALID_VIRT_ADDRESS,
    IRDMA_COMPL_STATUS_INVALID_REGION,
    IRDMA_COMPL_STATUS_INVALID_WINDOW,
    IRDMA_COMPL_STATUS_INVALID_TOTAL_LEN,
    IRDMA_COMPL_STATUS_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_cmpl_notify {
    IRDMA_CQ_COMPL_EVENT     = 0,
    IRDMA_CQ_COMPL_SOLICITED = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_qp_caps {
    IRDMA_WRITE_WITH_IMM = 1,
    IRDMA_SEND_WITH_IMM  = 2,
    IRDMA_ROCE	     = 4,
    IRDMA_PUSH_MODE      = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ring {
    pub head: u32,
    pub tail: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cqe {
    pub buf: [__le64; IRDMA_CQE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_extended_cqe {
    pub buf: [__le64; IRDMA_EXTENDED_CQE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_post_send {
    pub sg_list: *mut ib_sge,
    pub num_sges: u32,
    pub qkey: u32,
    pub dest_qp: u32,
    pub ah_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_post_rq_info {
    pub wr_id: u64,
    pub sg_list: *mut ib_sge,
    pub num_sges: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_rdma_write {
    pub lo_sg_list: *mut ib_sge,
    pub num_lo_sges: u32,
    pub rem_addr: ib_sge,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_rdma_read {
    pub lo_sg_list: *mut ib_sge,
    pub num_lo_sges: u32,
    pub rem_addr: ib_sge,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_bind_window {
    pub mr_stag: irdma_stag,
    pub bind_len: u64,
    pub va: *mut c_void,
    pub addressing_type: irdma_addressing_type,
    pub ena_reads:1: bool,
    pub ena_writes:1: bool,
    pub mw_stag: irdma_stag,
    pub mem_window_type_1:1: bool,
    pub remote_atomics_en:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_atomic_fetch_add {
    pub tagged_offset: u64,
    pub remote_tagged_offset: u64,
    pub fetch_add_data_bytes: u64,
    pub stag: u32,
    pub remote_stag: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_atomic_compare_swap {
    pub tagged_offset: u64,
    pub remote_tagged_offset: u64,
    pub swap_data_bytes: u64,
    pub compare_data_bytes: u64,
    pub stag: u32,
    pub remote_stag: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_inv_local_stag {
    pub target_stag: irdma_stag,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_post_sq_info {
    pub wr_id: u64,
    pub op_type: u8,
    pub l4len: u8,
    pub signaled:1: bool,
    pub read_fence:1: bool,
    pub local_fence:1: bool,
    pub inline_data:1: bool,
    pub imm_data_valid:1: bool,
    pub report_rtt:1: bool,
    pub udp_hdr:1: bool,
    pub defer_flag:1: bool,
    pub remote_atomic_en:1: bool,
    pub imm_data: u32,
    pub stag_to_inv: u32,
    pub send: irdma_post_send,
    pub rdma_write: irdma_rdma_write,
    pub rdma_read: irdma_rdma_read,
    pub bind_window: irdma_bind_window,
    pub inv_local_stag: irdma_inv_local_stag,
    pub atomic_fetch_add: irdma_atomic_fetch_add,
    pub atomic_compare_swap: irdma_atomic_compare_swap,
    pub op: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cq_poll_info {
    pub wr_id: u64,
    pub qp_handle: irdma_qp_handle,
    pub bytes_xfered: u32,
    pub tcp_seq_num_rtt: u32,
    pub qp_id: u32,
    pub ud_src_qpn: u32,
    pub imm_data: u32,
    pub /: *mut *mut irdma_stag inv_stag; / or L_R_Key,
    pub comp_status: irdma_cmpl_status,
    pub major_err: u16,
    pub minor_err: u16,
    pub ud_vlan: u16,
    pub ud_smac: [u8; 6],
    pub op_type: u8,
    pub q_type: u8,
    pub /: *mut *mut bool stag_invalid_set:1; / or L_R_Key set,
    pub error:1: bool,
    pub solicited_event:1: bool,
    pub ipv4:1: bool,
    pub ud_vlan_valid:1: bool,
    pub ud_smac_valid:1: bool,
    pub imm_valid:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qp_err_code {
    pub flush_code: irdma_flush_opcode,
    pub event_type: irdma_qp_event_type,
}

extern "C" {
    pub fn irdma_uk_qp_post_wr(qp: *mut irdma_qp_uk);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_wqe_uk_ops {
    pub polarity): u32 num_sges, u8,
    pub data_size): *mut *mut u16 (iw_inline_data_size_to_quanta)(u32,
    pub valid): u8,
    pub op_info): *mut irdma_bind_window,
}

extern "C" {
    pub fn irdma_uk_cq_empty(cq: *mut irdma_cq_uk) -> bool;
}
extern "C" {
    pub fn irdma_uk_cq_resize(cq: *mut irdma_cq_uk, cq_base: *mut c_void, size: c_int);
}
extern "C" {
    pub fn irdma_uk_cq_set_resized_cnt(qp: *mut irdma_cq_uk, cnt: u16);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_srq_uk {
    pub srq_caps: u32,
    pub srq_base: *mut irdma_qp_quanta,
    pub uk_attrs: *mut irdma_uk_attrs,
    pub shadow_area: *mut __le64,
    pub srq_ring: irdma_ring,
    pub srq_id: u32,
    pub srq_size: u32,
    pub max_srq_frag_cnt: u32,
    pub wqe_ops: irdma_wqe_uk_ops,
    pub srwqe_polarity: u8,
    pub wqe_size: u8,
    pub wqe_size_multiplier: u8,
    pub deferred_flag: u8,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_srq_uk_init_info {
    pub srq: *mut irdma_qp_quanta,
    pub uk_attrs: *mut irdma_uk_attrs,
    pub shadow_area: *mut __le64,
    pub srq_wrid_array: *mut u64,
    pub srq_id: u32,
    pub srq_caps: u32,
    pub srq_size: u32,
    pub max_srq_frag_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_sq_uk_wr_trk_info {
    pub wrid: u64,
    pub wr_len: u32,
    pub quanta: u16,
    pub signaled: u8,
    pub reserved: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qp_quanta {
    pub elem: [__le64; IRDMA_WQE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qp_uk {
    pub sq_base: *mut irdma_qp_quanta,
    pub rq_base: *mut irdma_qp_quanta,
    pub uk_attrs: *mut irdma_uk_attrs,
    pub wqe_alloc_db: *mut u32 __iomem,
    pub sq_wrtrk_array: *mut irdma_sq_uk_wr_trk_info,
    pub rq_wrid_array: *mut u64,
    pub shadow_area: *mut __le64,
    pub sq_ring: irdma_ring,
    pub rq_ring: irdma_ring,
    pub initial_ring: irdma_ring,
    pub qp_id: u32,
    pub qp_caps: u32,
    pub sq_size: u32,
    pub rq_size: u32,
    pub max_sq_frag_cnt: u32,
    pub max_rq_frag_cnt: u32,
    pub max_inline_data: u32,
    pub wqe_ops: irdma_wqe_uk_ops,
    pub conn_wqes: u16,
    pub qp_type: u8,
    pub swqe_polarity: u8,
    pub swqe_polarity_deferred: u8,
    pub rwqe_polarity: u8,
    pub rq_wqe_size: u8,
    pub rq_wqe_size_multiplier: u8,
    pub deferred_flag:1: bool,
    pub first_sq_wq:1: bool,
    pub /: *mut *mut bool sq_flush_complete:1; / Indicates flush was seen and SQ was empty after the flush,
    pub /: *mut *mut bool rq_flush_complete:1; / Indicates flush was seen and RQ was empty after the flush,
    pub /: *mut *mut bool destroy_pending:1; / Indicates the QP is being destroyed,
    pub back_qp: *mut c_void,
    pub dbg_rq_flushed: u8,
    pub srq_uk: *mut irdma_srq_uk,
    pub sq_flush_seen: u8,
    pub rq_flush_seen: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cq_uk {
    pub cq_base: *mut irdma_cqe,
    pub cqe_alloc_db: *mut u32 __iomem,
    pub cq_ack_db: *mut u32 __iomem,
    pub shadow_area: *mut __le64,
    pub cq_id: u32,
    pub cq_size: u32,
    pub cq_ring: irdma_ring,
    pub polarity: u8,
    pub avoid_mem_cflct:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qp_uk_init_info {
    pub sq: *mut irdma_qp_quanta,
    pub rq: *mut irdma_qp_quanta,
    pub uk_attrs: *mut irdma_uk_attrs,
    pub wqe_alloc_db: *mut u32 __iomem,
    pub shadow_area: *mut __le64,
    pub sq_wrtrk_array: *mut irdma_sq_uk_wr_trk_info,
    pub rq_wrid_array: *mut u64,
    pub qp_id: u32,
    pub qp_caps: u32,
    pub sq_size: u32,
    pub rq_size: u32,
    pub max_sq_frag_cnt: u32,
    pub max_rq_frag_cnt: u32,
    pub max_inline_data: u32,
    pub sq_depth: u32,
    pub rq_depth: u32,
    pub first_sq_wq: u8,
    pub type: u8,
    pub sq_shift: u8,
    pub rq_shift: u8,
    pub abi_ver: c_int,
    pub srq_uk: *mut irdma_srq_uk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cq_uk_init_info {
    pub cqe_alloc_db: *mut u32 __iomem,
    pub cq_ack_db: *mut u32 __iomem,
    pub cq_base: *mut irdma_cqe,
    pub shadow_area: *mut __le64,
    pub cq_size: u32,
    pub cq_id: u32,
    pub avoid_mem_cflct: bool,
}

extern "C" {
    pub fn irdma_uk_clean_cq(q: *mut c_void, cq: *mut irdma_cq_uk);
}
extern "C" {
    pub fn irdma_nop(qp: *mut irdma_qp_uk, wr_id: u64, signaled: bool, post_sq: bool) -> c_int;
}
extern "C" {
    pub fn irdma_fragcnt_to_quanta_sq(frag_cnt: u32, quanta: *mut u16) -> c_int;
}
extern "C" {
    pub fn irdma_fragcnt_to_wqesize_rq(frag_cnt: u32, wqe_size: *mut u16) -> c_int;
}
extern "C" {
    pub fn irdma_clr_wqes(qp: *mut irdma_qp_uk, qp_wqe_idx: u32);
}
