//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/defs.h
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
// Copyright (c) 2015 - 2021 Intel Corporation
pub const IRDMA_FIRST_USER_QP_ID: c_int = 3;
pub const ECN_CODE_PT_VAL: c_int = 2;

pub const IRDMA_PF_FIRST_PUSH_PAGE_INDEX: c_int = 16;

pub const IRDMA_PE_DB_SIZE_4M: c_int = 1;
pub const IRDMA_PE_DB_SIZE_8M: c_int = 2;
pub const IRDMA_IRD_HW_SIZE_4_GEN3: c_int = 0;
pub const IRDMA_IRD_HW_SIZE_8_GEN3: c_int = 1;
pub const IRDMA_IRD_HW_SIZE_16_GEN3: c_int = 2;
pub const IRDMA_IRD_HW_SIZE_32_GEN3: c_int = 3;
pub const IRDMA_IRD_HW_SIZE_64_GEN3: c_int = 4;
pub const IRDMA_IRD_HW_SIZE_128_GEN3: c_int = 5;
pub const IRDMA_IRD_HW_SIZE_256_GEN3: c_int = 6;
pub const IRDMA_IRD_HW_SIZE_512_GEN3: c_int = 7;
pub const IRDMA_IRD_HW_SIZE_1024_GEN3: c_int = 8;
pub const IRDMA_IRD_HW_SIZE_2048_GEN3: c_int = 9;
pub const IRDMA_IRD_HW_SIZE_4096_GEN3: c_int = 10;
pub const IRDMA_IRD_HW_SIZE_4: c_int = 0;
pub const IRDMA_IRD_HW_SIZE_16: c_int = 1;
pub const IRDMA_IRD_HW_SIZE_64: c_int = 2;
pub const IRDMA_IRD_HW_SIZE_128: c_int = 3;
pub const IRDMA_IRD_HW_SIZE_256: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_protocol_used {
    IRDMA_ANY_PROTOCOL = 0,
    IRDMA_IWARP_PROTOCOL_ONLY = 1,
    IRDMA_ROCE_PROTOCOL_ONLY = 2,
}

pub const IRDMA_QP_STATE_INVALID: c_int = 0;
pub const IRDMA_QP_STATE_IDLE: c_int = 1;
pub const IRDMA_QP_STATE_RTS: c_int = 2;
pub const IRDMA_QP_STATE_CLOSING: c_int = 3;
pub const IRDMA_QP_STATE_SQD: c_int = 3;
pub const IRDMA_QP_STATE_RTR: c_int = 4;
pub const IRDMA_QP_STATE_TERMINATE: c_int = 5;
pub const IRDMA_QP_STATE_ERROR: c_int = 6;
pub const IRDMA_MAX_TRAFFIC_CLASS: c_int = 8;
pub const IRDMA_MAX_STATS_COUNT_GEN_1: c_int = 12;
pub const IRDMA_MAX_USER_PRIORITY: c_int = 8;
pub const IRDMA_MAX_APPS: c_int = 8;
pub const IRDMA_MAX_STATS_COUNT: c_int = 128;
pub const IRDMA_FIRST_NON_PF_STAT: c_int = 4;
pub const IRDMA_MIN_MTU_IPV4: c_int = 576;
pub const IRDMA_MIN_MTU_IPV6: c_int = 1280;
pub const IRDMA_MTU_TO_MSS_IPV4: c_int = 40;
pub const IRDMA_MTU_TO_MSS_IPV6: c_int = 60;
pub const IRDMA_DEFAULT_MTU: c_int = 1500;
pub const Q2_FPSN_OFFSET: c_int = 64;
pub const TERM_DDP_LEN_TAGGED: c_int = 14;
pub const TERM_DDP_LEN_UNTAGGED: c_int = 18;
pub const TERM_RDMA_LEN: c_int = 28;
pub const RDMA_OPCODE_M: c_uint = 0x0f;
pub const RDMA_READ_REQ_OPCODE: c_int = 1;
pub const Q2_BAD_FRAME_OFFSET: c_int = 72;
pub const CQE_MAJOR_DRV: c_uint = 0x8000;
pub const IRDMA_TERM_SENT: c_int = 1;
pub const IRDMA_TERM_RCVD: c_int = 2;
pub const IRDMA_TERM_DONE: c_int = 4;
pub const IRDMA_MAC_HLEN: c_int = 14;
pub const IRDMA_CQP_WAIT_POLL_REGS: c_int = 1;
pub const IRDMA_CQP_WAIT_POLL_CQ: c_int = 2;
pub const IRDMA_CQP_WAIT_EVENT: c_int = 3;
pub const IRDMA_AE_SOURCE_RSVD: c_uint = 0x0;
pub const IRDMA_AE_SOURCE_RQ: c_uint = 0x1;
pub const IRDMA_AE_SOURCE_RQ_0011: c_uint = 0x3;
pub const IRDMA_AE_SOURCE_CQ: c_uint = 0x2;
pub const IRDMA_AE_SOURCE_CQ_0110: c_uint = 0x6;
pub const IRDMA_AE_SOURCE_CQ_1010: c_uint = 0xa;
pub const IRDMA_AE_SOURCE_CQ_1110: c_uint = 0xe;
pub const IRDMA_AE_SOURCE_SQ: c_uint = 0x5;
pub const IRDMA_AE_SOURCE_SQ_0111: c_uint = 0x7;
pub const IRDMA_AE_SOURCE_IN_RR_WR: c_uint = 0x9;
pub const IRDMA_AE_SOURCE_IN_RR_WR_1011: c_uint = 0xb;
pub const IRDMA_AE_SOURCE_OUT_RR: c_uint = 0xd;
pub const IRDMA_AE_SOURCE_OUT_RR_1111: c_uint = 0xf;
pub const IRDMA_TCP_STATE_NON_EXISTENT: c_int = 0;
pub const IRDMA_TCP_STATE_CLOSED: c_int = 1;
pub const IRDMA_TCP_STATE_LISTEN: c_int = 2;
pub const IRDMA_STATE_SYN_SEND: c_int = 3;
pub const IRDMA_TCP_STATE_SYN_RECEIVED: c_int = 4;
pub const IRDMA_TCP_STATE_ESTABLISHED: c_int = 5;
pub const IRDMA_TCP_STATE_CLOSE_WAIT: c_int = 6;
pub const IRDMA_TCP_STATE_FIN_WAIT_1: c_int = 7;
pub const IRDMA_TCP_STATE_CLOSING: c_int = 8;
pub const IRDMA_TCP_STATE_LAST_ACK: c_int = 9;
pub const IRDMA_TCP_STATE_FIN_WAIT_2: c_int = 10;
pub const IRDMA_TCP_STATE_TIME_WAIT: c_int = 11;
pub const IRDMA_TCP_STATE_RESERVED_1: c_int = 12;
pub const IRDMA_TCP_STATE_RESERVED_2: c_int = 13;
pub const IRDMA_TCP_STATE_RESERVED_3: c_int = 14;
pub const IRDMA_TCP_STATE_RESERVED_4: c_int = 15;
pub const IRDMA_CQP_SW_SQSIZE_4: c_int = 4;
pub const IRDMA_CQP_SW_SQSIZE_2048: c_int = 2048;
pub const IRDMA_CQ_TYPE_IWARP: c_int = 1;
pub const IRDMA_CQ_TYPE_ILQ: c_int = 2;
pub const IRDMA_CQ_TYPE_IEQ: c_int = 3;
pub const IRDMA_CQ_TYPE_CQP: c_int = 4;
pub const IRDMA_DONE_COUNT: c_int = 1000;
pub const IRDMA_SLEEP_COUNT: c_int = 10;
pub const IRDMA_UPDATE_SD_BUFF_SIZE: c_int = 128;

pub const ENABLE_LOC_MEM: c_int = 63;
pub const IRDMA_ATOMICS_ALLOWED_BIT: c_int = 1;
pub const MAX_PBLE_PER_SD: c_uint = 0x40000;
pub const MAX_PBLE_SD_PER_FCN: c_uint = 0x400;
pub const MAX_MR_PER_SD: c_uint = 0x8000;
pub const MAX_MR_SD_PER_FCN: c_uint = 0x80;
pub const IRDMA_PBLE_COMMIT_OFFSET: c_int = 112;
pub const IRDMA_SCRATCH_BUF0_COMMIT_OFFSET: c_int = 192;
pub const IRDMA_SCRATCH_BUF1_COMMIT_OFFSET: c_int = 200;
pub const IRDMA_MAX_QUANTA_PER_WR: c_int = 8;
pub const IRDMA_QP_SW_MAX_WQ_QUANTA: c_int = 32768;
pub const IRDMA_QP_SW_MAX_SQ_QUANTA: c_int = 32768;
pub const IRDMA_QP_SW_MAX_RQ_QUANTA: c_int = 32768;

pub const IRDMA_SRQ_MIN_QUANTA: c_int = 8;
pub const IRDMA_SRQ_MAX_QUANTA: c_int = 262144;

pub const IRDMAQP_TERM_SEND_TERM_AND_FIN: c_int = 0;
pub const IRDMAQP_TERM_SEND_TERM_ONLY: c_int = 1;
pub const IRDMAQP_TERM_SEND_FIN_ONLY: c_int = 2;
pub const IRDMAQP_TERM_DONOT_SEND_TERM_OR_FIN: c_int = 3;
pub const IRDMA_QP_TYPE_IWARP: c_int = 1;
pub const IRDMA_QP_TYPE_UDA: c_int = 2;
pub const IRDMA_QP_TYPE_ROCE_RC: c_int = 3;
pub const IRDMA_QP_TYPE_ROCE_UD: c_int = 4;
pub const IRDMA_HW_PAGE_SIZE: c_int = 4096;
pub const IRDMA_HW_PAGE_SHIFT: c_int = 12;
pub const IRDMA_CQE_QTYPE_RQ: c_int = 0;
pub const IRDMA_CQE_QTYPE_SQ: c_int = 1;

pub const IRDMA_QP_WQE_MIN_SIZE: c_int = 32;
pub const IRDMA_QP_WQE_MAX_SIZE: c_int = 256;
pub const IRDMA_QP_WQE_MIN_QUANTA: c_int = 1;
pub const IRDMA_MAX_RQ_WQE_SHIFT_GEN1: c_int = 2;
pub const IRDMA_MAX_RQ_WQE_SHIFT_GEN2: c_int = 3;
pub const IRDMA_SQ_RSVD: c_int = 258;
pub const IRDMA_RQ_RSVD: c_int = 1;

pub const IRDMAQP_OP_RDMA_WRITE: c_uint = 0x00;
pub const IRDMAQP_OP_RDMA_READ: c_uint = 0x01;
pub const IRDMAQP_OP_RDMA_SEND: c_uint = 0x03;
pub const IRDMAQP_OP_RDMA_SEND_INV: c_uint = 0x04;
pub const IRDMAQP_OP_RDMA_SEND_SOL_EVENT: c_uint = 0x05;
pub const IRDMAQP_OP_RDMA_SEND_SOL_EVENT_INV: c_uint = 0x06;
pub const IRDMAQP_OP_BIND_MW: c_uint = 0x08;
pub const IRDMAQP_OP_FAST_REGISTER: c_uint = 0x09;
pub const IRDMAQP_OP_LOCAL_INVALIDATE: c_uint = 0x0a;
pub const IRDMAQP_OP_RDMA_READ_LOC_INV: c_uint = 0x0b;
pub const IRDMAQP_OP_NOP: c_uint = 0x0c;
pub const IRDMAQP_OP_RDMA_WRITE_SOL: c_uint = 0x0d;
pub const IRDMAQP_OP_ATOMIC_FETCH_ADD: c_uint = 0x0f;
pub const IRDMAQP_OP_ATOMIC_COMPARE_SWAP_ADD: c_uint = 0x11;
pub const IRDMAQP_OP_GEN_RTS_AE: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_cqp_op_type {
    IRDMA_OP_CEQ_DESTROY			= 1,
    IRDMA_OP_AEQ_DESTROY			= 2,
    IRDMA_OP_DELETE_ARP_CACHE_ENTRY		= 3,
    IRDMA_OP_MANAGE_APBVT_ENTRY		= 4,
    IRDMA_OP_CEQ_CREATE			= 5,
    IRDMA_OP_AEQ_CREATE			= 6,
    IRDMA_OP_MANAGE_QHASH_TABLE_ENTRY	= 7,
    IRDMA_OP_QP_MODIFY			= 8,
    IRDMA_OP_QP_UPLOAD_CONTEXT		= 9,
    IRDMA_OP_CQ_CREATE			= 10,
    IRDMA_OP_CQ_DESTROY			= 11,
    IRDMA_OP_QP_CREATE			= 12,
    IRDMA_OP_QP_DESTROY			= 13,
    IRDMA_OP_ALLOC_STAG			= 14,
    IRDMA_OP_MR_REG_NON_SHARED		= 15,
    IRDMA_OP_DEALLOC_STAG			= 16,
    IRDMA_OP_MW_ALLOC			= 17,
    IRDMA_OP_QP_FLUSH_WQES			= 18,
    IRDMA_OP_ADD_ARP_CACHE_ENTRY		= 19,
    IRDMA_OP_MANAGE_PUSH_PAGE		= 20,
    IRDMA_OP_UPDATE_PE_SDS			= 21,
    IRDMA_OP_MANAGE_HMC_PM_FUNC_TABLE	= 22,
    IRDMA_OP_SUSPEND			= 23,
    IRDMA_OP_RESUME				= 24,
    IRDMA_OP_MANAGE_VF_PBLE_BP		= 25,
    IRDMA_OP_QUERY_FPM_VAL			= 26,
    IRDMA_OP_COMMIT_FPM_VAL			= 27,
    IRDMA_OP_AH_CREATE			= 28,
    IRDMA_OP_AH_MODIFY			= 29,
    IRDMA_OP_AH_DESTROY			= 30,
    IRDMA_OP_MC_CREATE			= 31,
    IRDMA_OP_MC_DESTROY			= 32,
    IRDMA_OP_MC_MODIFY			= 33,
    IRDMA_OP_STATS_ALLOCATE			= 34,
    IRDMA_OP_STATS_FREE			= 35,
    IRDMA_OP_STATS_GATHER			= 36,
    IRDMA_OP_WS_ADD_NODE			= 37,
    IRDMA_OP_WS_MODIFY_NODE			= 38,
    IRDMA_OP_WS_DELETE_NODE			= 39,
    IRDMA_OP_WS_FAILOVER_START		= 40,
    IRDMA_OP_WS_FAILOVER_COMPLETE		= 41,
    IRDMA_OP_SET_UP_MAP			= 42,
    IRDMA_OP_GEN_AE				= 43,
    IRDMA_OP_QUERY_RDMA_FEATURES		= 44,
    IRDMA_OP_ALLOC_LOCAL_MAC_ENTRY		= 45,
    IRDMA_OP_ADD_LOCAL_MAC_ENTRY		= 46,
    IRDMA_OP_DELETE_LOCAL_MAC_ENTRY		= 47,
    IRDMA_OP_CQ_MODIFY			= 48,
    IRDMA_OP_SRQ_CREATE			= 49,
    IRDMA_OP_SRQ_MODIFY			= 50,
    IRDMA_OP_SRQ_DESTROY			= 51,

// Must be last entry
    IRDMA_MAX_CQP_OPS			= 52,
}

// CQP SQ WQES
pub const IRDMA_CQP_OP_CREATE_QP: c_int = 0;
pub const IRDMA_CQP_OP_MODIFY_QP: c_uint = 0x1;
pub const IRDMA_CQP_OP_DESTROY_QP: c_uint = 0x02;
pub const IRDMA_CQP_OP_CREATE_CQ: c_uint = 0x03;
pub const IRDMA_CQP_OP_MODIFY_CQ: c_uint = 0x04;
pub const IRDMA_CQP_OP_DESTROY_CQ: c_uint = 0x05;
pub const IRDMA_CQP_OP_CREATE_SRQ: c_uint = 0x06;
pub const IRDMA_CQP_OP_MODIFY_SRQ: c_uint = 0x07;
pub const IRDMA_CQP_OP_DESTROY_SRQ: c_uint = 0x08;
pub const IRDMA_CQP_OP_ALLOC_STAG: c_uint = 0x09;
pub const IRDMA_CQP_OP_REG_MR: c_uint = 0x0a;
pub const IRDMA_CQP_OP_QUERY_STAG: c_uint = 0x0b;
pub const IRDMA_CQP_OP_REG_SMR: c_uint = 0x0c;
pub const IRDMA_CQP_OP_DEALLOC_STAG: c_uint = 0x0d;
pub const IRDMA_CQP_OP_MANAGE_LOC_MAC_TABLE: c_uint = 0x0e;
pub const IRDMA_CQP_OP_MANAGE_ARP: c_uint = 0x0f;
pub const IRDMA_CQP_OP_MANAGE_VF_PBLE_BP: c_uint = 0x10;
pub const IRDMA_CQP_OP_MANAGE_PUSH_PAGES: c_uint = 0x11;
pub const IRDMA_CQP_OP_QUERY_RDMA_FEATURES: c_uint = 0x12;
pub const IRDMA_CQP_OP_UPLOAD_CONTEXT: c_uint = 0x13;
pub const IRDMA_CQP_OP_ALLOCATE_LOC_MAC_TABLE_ENTRY: c_uint = 0x14;
pub const IRDMA_CQP_OP_UPLOAD_CONTEXT: c_uint = 0x13;
pub const IRDMA_CQP_OP_MANAGE_HMC_PM_FUNC_TABLE: c_uint = 0x15;
pub const IRDMA_CQP_OP_CREATE_CEQ: c_uint = 0x16;
pub const IRDMA_CQP_OP_DESTROY_CEQ: c_uint = 0x18;
pub const IRDMA_CQP_OP_CREATE_AEQ: c_uint = 0x19;
pub const IRDMA_CQP_OP_DESTROY_AEQ: c_uint = 0x1b;
pub const IRDMA_CQP_OP_CREATE_ADDR_HANDLE: c_uint = 0x1c;
pub const IRDMA_CQP_OP_MODIFY_ADDR_HANDLE: c_uint = 0x1d;
pub const IRDMA_CQP_OP_DESTROY_ADDR_HANDLE: c_uint = 0x1e;
pub const IRDMA_CQP_OP_UPDATE_PE_SDS: c_uint = 0x1f;
pub const IRDMA_CQP_OP_QUERY_FPM_VAL: c_uint = 0x20;
pub const IRDMA_CQP_OP_COMMIT_FPM_VAL: c_uint = 0x21;
pub const IRDMA_CQP_OP_FLUSH_WQES: c_uint = 0x22;
// IRDMA_CQP_OP_GEN_AE is the same value as IRDMA_CQP_OP_FLUSH_WQES
pub const IRDMA_CQP_OP_GEN_AE: c_uint = 0x22;
pub const IRDMA_CQP_OP_MANAGE_APBVT: c_uint = 0x23;
pub const IRDMA_CQP_OP_NOP: c_uint = 0x24;
pub const IRDMA_CQP_OP_MANAGE_QUAD_HASH_TABLE_ENTRY: c_uint = 0x25;
pub const IRDMA_CQP_OP_CREATE_MCAST_GRP: c_uint = 0x26;
pub const IRDMA_CQP_OP_MODIFY_MCAST_GRP: c_uint = 0x27;
pub const IRDMA_CQP_OP_DESTROY_MCAST_GRP: c_uint = 0x28;
pub const IRDMA_CQP_OP_SUSPEND_QP: c_uint = 0x29;
pub const IRDMA_CQP_OP_RESUME_QP: c_uint = 0x2a;
pub const IRDMA_CQP_OP_SHMC_PAGES_ALLOCATED: c_uint = 0x2b;
pub const IRDMA_CQP_OP_WORK_SCHED_NODE: c_uint = 0x2c;
pub const IRDMA_CQP_OP_MANAGE_STATS: c_uint = 0x2d;
pub const IRDMA_CQP_OP_GATHER_STATS: c_uint = 0x2e;
pub const IRDMA_CQP_OP_UP_MAP: c_uint = 0x2f;

pub const IRDMA_MAX_STATS_24: c_uint = 0xffffffULL;
pub const IRDMA_MAX_STATS_32: c_uint = 0xffffffffULL;
pub const IRDMA_MAX_STATS_48: c_uint = 0xffffffffffffULL;
pub const IRDMA_MAX_STATS_56: c_uint = 0xffffffffffffffULL;
pub const IRDMA_MAX_STATS_64: c_uint = 0xffffffffffffffffULL;
pub const IRDMA_MAX_CQ_READ_THRESH: c_uint = 0x3FFFF;

pub const IRDMA_CQPHC_HW_MAJVER_GEN_1: c_int = 0;
pub const IRDMA_CQPHC_HW_MAJVER_GEN_2: c_int = 1;
pub const IRDMA_CQPHC_HW_MAJVER_GEN_3: c_int = 2;

// CQP and iWARP Completion Queue

// AEQE format

pub const IRDMA_AEQE_CMPL_CTXT_S: c_int = 6;

// Create/Modify/Destroy QP

pub const IRDMA_CQPSQ_QP_QPID_S: c_int = 0;

pub const IRDMA_CQPSQ_QP_OP_S: c_int = 32;

pub const IRDMA_CQPSQ_SRQ_PHYSICAL_BUFFER_ADDR_S: c_int = 8;

pub const IRDMA_CQPSQ_SRQ_DB_SHADOW_ADDR_S: c_int = 6;

// Allocate/Register/Register Shared/Deallocate Stag

pub const IRDMA_CQPSQ_STAG_IDX_S: c_int = 8;

// Manage Push Page - MPP
pub const IRDMA_INVALID_PUSH_PAGE_INDEX_GEN_1: c_uint = 0xffff;
pub const IRDMA_INVALID_PUSH_PAGE_INDEX: c_uint = 0xffffffff;

// Upload Context - UCTX

pub const IRDMA_COMMIT_FPM_BASE_S: c_int = 32;

pub const IRDMA_CQPSQ_CFPM_FW_SCRATCH_BUF_PRESENT_S: c_int = 38;

pub const IRDMA_CQPSQ_UPESD_BM_PF: c_int = 0;
pub const IRDMA_CQPSQ_UPESD_BM_CP_LM: c_int = 1;
pub const IRDMA_CQPSQ_UPESD_BM_AXF: c_int = 2;
pub const IRDMA_CQPSQ_UPESD_BM_LM: c_int = 4;

pub const IRDMA_CQPSQ_MIN_STAG_INVALID: c_uint = 0x0001;
pub const IRDMA_CQPSQ_MIN_SUSPEND_PND: c_uint = 0x0005;
pub const IRDMA_CQPSQ_MIN_DEF_CMPL: c_uint = 0x0006;
pub const IRDMA_CQPSQ_MIN_OOO_CMPL: c_uint = 0x0007;
pub const IRDMA_CQPSQ_MAJ_NO_ERROR: c_uint = 0x0000;
pub const IRDMA_CQPSQ_MAJ_OBJCACHE_ERROR: c_uint = 0xF000;
pub const IRDMA_CQPSQ_MAJ_CNTXTCACHE_ERROR: c_uint = 0xF001;
pub const IRDMA_CQPSQ_MAJ_ERROR: c_uint = 0xFFFF;

pub const IRDMA_INLINE_VALID_S: c_int = 7;

// iwarp QP RQ WQE common fields

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_qp_wqe_size {
    IRDMA_WQE_SIZE_32  = 32,
    IRDMA_WQE_SIZE_64  = 64,
    IRDMA_WQE_SIZE_96  = 96,
    IRDMA_WQE_SIZE_128 = 128,
    IRDMA_WQE_SIZE_256 = 256,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_ws_node_op {
    IRDMA_ADD_NODE = 0,
    IRDMA_MODIFY_NODE,
    IRDMA_DEL_NODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_alignment {
    IRDMA_CQP_ALIGNMENT	    = 0x200,
    IRDMA_AEQ_ALIGNMENT	    = 0x100,
    IRDMA_CEQ_ALIGNMENT	    = 0x100,
    IRDMA_CQ0_ALIGNMENT	    = 0x100,
    IRDMA_SD_BUF_ALIGNMENT      = 0x80,
    IRDMA_FEATURE_BUF_ALIGNMENT = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icrdma_protocol_used {
    ICRDMA_ANY_PROTOCOL	   = 0,
    ICRDMA_IWARP_PROTOCOL_ONLY = 1,
    ICRDMA_ROCE_PROTOCOL_ONLY  = 2,
}

//
// set_64bit_val - set 64 bit value to hw wqe
// @wqe_words: wqe addr to write
// @byte_index: index in wqe
// @val: value to write
//
// set_32bit_val - set 32 bit value to hw wqe
// @wqe_words: wqe addr to write
// @byte_index: index in wqe
// @val: value to write
//
// get_64bit_val - read 64 bit value from wqe
// @wqe_words: wqe addr
// @byte_index: index to read from
// @val: read value
//
// val = le64_to_cpu(wqe_words[byte_index >> 3]);
//
// get_32bit_val - read 32 bit value from wqe
// @wqe_words: wqe addr
// @byte_index: index to reaad from
// @val: return 32 bit value
//
// val = le32_to_cpu(wqe_words[byte_index >> 2]);
