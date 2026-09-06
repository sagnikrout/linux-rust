//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/cxgb4/t4.h
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
// Copyright (c) 2009-2010 Chelsio, Inc. All rights reserved.
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

pub const T4_MAX_NUM_PD: c_int = 65536;

pub const T4_PAGESIZE_MASK: c_uint = 0xffff000  /* 4KB-128MB */;
pub const T4_STAG_UNSET: c_uint = 0xffffffff;
pub const T4_FW_MAJ: c_int = 0;
pub const PCIE_MA_SYNC_A: c_uint = 0x30b4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_status_page {
    pub /: *mut *mut __be32 rsvd1; / flit 0 - hw owns,
    pub rsvd2: __be16,
    pub qid: __be16,
    pub cidx: __be16,
    pub pidx: __be16,
    pub /: *mut *mut u8 qp_err; / flit 1 - sw owns,
    pub db_off: u8,
    pub pad: [u8; 2],
    pub host_wq_pidx: u16,
    pub host_cidx: u16,
    pub host_pidx: u16,
    pub pad2: u16,
    pub srqidx: u32,
}

pub const T4_RQT_ENTRY_SHIFT: c_int = 6;

pub const T4_EQ_ENTRY_SIZE: c_int = 64;
pub const T4_SQ_NUM_SLOTS: c_int = 5;

pub const T4_MAX_FR_DSGL: c_int = 1024;

pub const T4_RQ_NUM_SLOTS: c_int = 2;

pub const T4_MAX_RECV_SGE: c_int = 4;
pub const T4_WRITE_CMPL_MAX_SGL: c_int = 4;
pub const T4_WRITE_CMPL_MAX_CQE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub union t4_wr {
    pub res: fw_ri_res_wr,
    pub ri: fw_ri_wr,
    pub write: fw_ri_rdma_write_wr,
    pub send: fw_ri_send_wr,
    pub read: fw_ri_rdma_read_wr,
    pub bind: fw_ri_bind_mw_wr,
    pub fr: fw_ri_fr_nsmr_wr,
    pub fr_tpte: fw_ri_fr_nsmr_tpte_wr,
    pub inv: fw_ri_inv_lstag_wr,
    pub write_cmpl: fw_ri_rdma_write_cmpl_wr,
    pub status: t4_status_page,
    pub T4_SQ_NUM_SLOTS]: *mut *mut __be64 flits[T4_EQ_ENTRY_SIZE / sizeof(__be64),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union t4_recv_wr {
    pub recv: fw_ri_recv_wr,
    pub status: t4_status_page,
    pub T4_RQ_NUM_SLOTS]: *mut *mut __be64 flits[T4_EQ_ENTRY_SIZE / sizeof(__be64),
}

// CQE/AE status codes
pub const T4_ERR_SUCCESS: c_uint = 0x0;
pub const T4_ERR_STAG: c_uint = 0x1	/* STAG invalid: either the */;
// STAG is offlimt, being 0,
// or STAG_key mismatch
pub const T4_ERR_PDID: c_uint = 0x2	/* PDID mismatch */;
pub const T4_ERR_QPID: c_uint = 0x3	/* QPID mismatch */;
pub const T4_ERR_ACCESS: c_uint = 0x4	/* Invalid access right */;
pub const T4_ERR_WRAP: c_uint = 0x5	/* Wrap error */;
pub const T4_ERR_BOUND: c_uint = 0x6	/* base and bounds voilation */;
pub const T4_ERR_INVALIDATE_SHARED_MR: c_uint = 0x7	/* attempt to invalidate a  */;
// shared memory region
pub const T4_ERR_INVALIDATE_MR_WITH_MW_BOUND: c_uint = 0x8	/* attempt to invalidate a  */;
// shared memory region
pub const T4_ERR_ECC: c_uint = 0x9	/* ECC error detected */;
pub const T4_ERR_ECC_PSTAG: c_uint = 0xA	/* ECC error detected when  */;
// reading PSTAG for a MW
// Invalidate
pub const T4_ERR_PBL_ADDR_BOUND: c_uint = 0xB	/* pbl addr out of bounds:  */;
// software error
pub const T4_ERR_SWFLUSH: c_uint = 0xC	/* SW FLUSHED */;
pub const T4_ERR_CRC: c_uint = 0x10 /* CRC error */;
pub const T4_ERR_MARKER: c_uint = 0x11 /* Marker error */;
pub const T4_ERR_PDU_LEN_ERR: c_uint = 0x12 /* invalid PDU length */;
pub const T4_ERR_OUT_OF_RQE: c_uint = 0x13 /* out of RQE */;
pub const T4_ERR_DDP_VERSION: c_uint = 0x14 /* wrong DDP version */;
pub const T4_ERR_RDMA_VERSION: c_uint = 0x15 /* wrong RDMA version */;
pub const T4_ERR_OPCODE: c_uint = 0x16 /* invalid rdma opcode */;
pub const T4_ERR_DDP_QUEUE_NUM: c_uint = 0x17 /* invalid ddp queue number */;
pub const T4_ERR_MSN: c_uint = 0x18 /* MSN error */;
pub const T4_ERR_TBIT: c_uint = 0x19 /* tag bit not set correctly */;
pub const T4_ERR_MO: c_uint = 0x1A /* MO not 0 for TERMINATE  */;
// or READ_REQ
pub const T4_ERR_MSN_GAP: c_uint = 0x1B;
pub const T4_ERR_MSN_RANGE: c_uint = 0x1C;
pub const T4_ERR_IRD_OVERFLOW: c_uint = 0x1D;
pub const T4_ERR_RQE_ADDR_BOUND: c_uint = 0x1E /* RQE addr out of bounds:  */;
// software error
pub const T4_ERR_INTERNAL_ERR: c_uint = 0x1F /* internal error (opcode  */;
// mismatch)
//
// CQE defs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_cqe {
    pub header: __be32,
    pub len: __be32,
    pub stag: __be32,
    pub msn: __be32,
    pub rcqe: },
    pub stag: __be32,
    pub nada2: u16,
    pub cidx: u16,
    pub scqe: },
    pub wrid_hi: __be32,
    pub wrid_low: __be32,
    pub gen: },
    pub stag: __be32,
    pub msn: __be32,
    pub reserved: __be32,
    pub abs_rqe_idx: __be32,
    pub srcqe: },
    pub mo: __be32,
    pub msn: __be32,
//
// Use union for immediate data to be consistent with
// stack's 32 bit data and iWARP spec's 64 bit data.
//
    pub imm_data32: __be32,
    pub reserved: u32,
    pub ib_imm_data: },
    pub imm_data64: __be64,
    pub iw_imm_data: },
    pub imm_data_rcqe: },
    pub drain_cookie: u64,
    pub flits: [__be64; 3],
    pub u: },
    pub reserved: [__be64; 3],
    pub bits_type_ts: __be64,
}

// macros for flit 0 of the cqe
pub const CQE_QPID_S: c_int = 12;
pub const CQE_QPID_M: c_uint = 0xFFFFF;

pub const CQE_SWCQE_S: c_int = 11;
pub const CQE_SWCQE_M: c_uint = 0x1;

pub const CQE_DRAIN_S: c_int = 10;
pub const CQE_DRAIN_M: c_uint = 0x1;

pub const CQE_STATUS_S: c_int = 5;
pub const CQE_STATUS_M: c_uint = 0x1F;

pub const CQE_TYPE_S: c_int = 4;
pub const CQE_TYPE_M: c_uint = 0x1;

pub const CQE_OPCODE_S: c_int = 0;
pub const CQE_OPCODE_M: c_uint = 0xF;

// used for RQ completion processing

// used for SQ completion processing

// generic accessor macros

// macros for flit 3 of the cqe
pub const CQE_GENBIT_S: c_int = 63;
pub const CQE_GENBIT_M: c_uint = 0x1;

pub const CQE_OVFBIT_S: c_int = 62;
pub const CQE_OVFBIT_M: c_uint = 0x1;

pub const CQE_IQTYPE_S: c_int = 60;
pub const CQE_IQTYPE_M: c_uint = 0x3;

pub const CQE_TS_M: c_uint = 0x0fffffffffffffffULL;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_swsqe {
    pub wr_id: u64,
    pub cqe: t4_cqe,
    pub read_len: c_int,
    pub opcode: c_int,
    pub complete: c_int,
    pub signaled: c_int,
    pub idx: u16,
    pub flushed: c_int,
    pub host_time: ktime_t,
    pub sge_ts: u64,
}

extern "C" {
    pub fn pgprot_writecombine(_arg: prot) -> return;
}

extern "C" {
    pub fn pgprot_noncached(_arg: prot) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_sq {
    pub queue: *mut t4_wr,
    pub dma_addr: dma_addr_t,
    pub phys_addr: c_ulong,
    pub sw_sq: *mut t4_swsqe,
    pub oldest_read: *mut t4_swsqe,
    pub bar2_va: *mut void __iomem,
    pub bar2_pa: u64,
    pub memsize: usize,
    pub bar2_qid: u32,
    pub qid: u32,
    pub in_use: u16,
    pub size: u16,
    pub cidx: u16,
    pub pidx: u16,
    pub wq_pidx: u16,
    pub wq_pidx_inc: u16,
    pub flags: u16,
    pub flush_cidx: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_swrqe {
    pub wr_id: u64,
    pub host_time: ktime_t,
    pub sge_ts: u64,
    pub valid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_rq {
    pub queue: *mut t4_recv_wr,
    pub dma_addr: dma_addr_t,
    pub sw_rq: *mut t4_swrqe,
    pub bar2_va: *mut void __iomem,
    pub bar2_pa: u64,
    pub memsize: usize,
    pub bar2_qid: u32,
    pub qid: u32,
    pub msn: u32,
    pub rqt_hwaddr: u32,
    pub rqt_size: u16,
    pub in_use: u16,
    pub size: u16,
    pub cidx: u16,
    pub pidx: u16,
    pub wq_pidx: u16,
    pub wq_pidx_inc: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_wq {
    pub sq: t4_sq,
    pub rq: t4_rq,
    pub db: *mut void __iomem,
    pub rdev: *mut c4iw_rdev,
    pub flushed: c_int,
    pub qp_errp: *mut u8,
    pub srqidxp: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_srq_pending_wr {
    pub wr_id: u64,
    pub wqe: t4_recv_wr,
    pub len16: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_srq {
    pub queue: *mut t4_recv_wr,
    pub dma_addr: dma_addr_t,
    pub sw_rq: *mut t4_swrqe,
    pub bar2_va: *mut void __iomem,
    pub bar2_pa: u64,
    pub memsize: usize,
    pub bar2_qid: u32,
    pub qid: u32,
    pub msn: u32,
    pub rqt_hwaddr: u32,
    pub rqt_abs_idx: u32,
    pub rqt_size: u16,
    pub size: u16,
    pub cidx: u16,
    pub pidx: u16,
    pub wq_pidx: u16,
    pub wq_pidx_inc: u16,
    pub in_use: u16,
    pub pending_wrs: *mut t4_srq_pending_wr,
    pub pending_cidx: u16,
    pub pending_pidx: u16,
    pub pending_in_use: u16,
    pub ooo_count: u16,
}

// This function copies 64 byte coalesced work request to memory
// mapped BAR2 space. For coalesced WRs, the SGE fetches data
// from the FIFO instead of from Host.
//
// Flush host queue memory writes.
// Flush user doorbell area writes.
// Flush host queue memory writes.
// Flush user doorbell area writes.
// Flush host queue memory writes.
// Flush user doorbell area writes.
// wq->srqidxp = srqidx;
// wq->qp_errp = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t4_cq_flags {
    CQ_ARMED	= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_cq {
    pub queue: *mut t4_cqe,
    pub dma_addr: dma_addr_t,
    pub sw_queue: *mut t4_cqe,
    pub gts: *mut void __iomem,
    pub bar2_va: *mut void __iomem,
    pub bar2_pa: u64,
    pub bar2_qid: u32,
    pub rdev: *mut c4iw_rdev,
    pub memsize: usize,
    pub bits_type_ts: __be64,
    pub cqid: u32,
    pub qid_mask: u32,
    pub vector: c_int,
    pub /: *mut *mut u16 size; / including status page,
    pub cidx: u16,
    pub sw_pidx: u16,
    pub sw_cidx: u16,
    pub sw_in_use: u16,
    pub cidx_inc: u16,
    pub gen: u8,
    pub error: u8,
    pub qp_errp: *mut u8,
    pub flags: c_ulong,
}

extern "C" {
    pub fn test_and_clear_bit(_arg: CQ_ARMED, _arg: &cq->flags) -> return;
}
// Ensure CQE is flushed to memory
// cqe = &cq->queue[cq->cidx];
// cqe = &cq->sw_queue[cq->sw_cidx];
// cq->qp_errp = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_dev_status_page {
    pub db_off: u8,
    pub write_cmpl_supported: u8,
    pub pad2: u16,
    pub pad3: u32,
    pub qp_start: u64,
    pub qp_size: u64,
    pub cq_start: u64,
    pub cq_size: u64,
}
