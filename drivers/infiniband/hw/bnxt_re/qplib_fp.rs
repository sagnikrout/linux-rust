//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bnxt_re/qplib_fp.h
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
// Description: Fast Path Operators (header)
//

// Few helper structures temporarily defined here
// should get rid of these when roce_hsi.h is updated
// in original code base
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_ud_ext_hdr {
    pub dst_qp: __le32,
    pub avid: __le32,
    pub rsvd: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_raw_ext_hdr {
    pub cfa_meta: __le32,
    pub rsvd0: __le32,
    pub rsvd1: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_rdma_ext_hdr {
    pub remote_va: __le64,
    pub remote_key: __le32,
    pub rsvd: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_atomic_ext_hdr {
    pub swap_data: __le64,
    pub cmp_data: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_fr_pmr_ext_hdr {
    pub pblptr: __le64,
    pub va: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_bind_ext_hdr {
    pub va: __le64,
    pub length_lo: __le32,
    pub length_hi: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_ext_hdr {
    pub rsvd1: __le64,
    pub rsvd2: __le64,
}

// Helper structures end
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_srq {
    pub pd: *mut bnxt_qplib_pd,
    pub dpi: *mut bnxt_qplib_dpi,
    pub dbinfo: bnxt_qplib_db_info,
    pub srq_handle: u64,
    pub id: u32,
    pub wqe_size: u16,
    pub max_wqe: u32,
    pub max_sge: u32,
    pub threshold: u32,
    pub arm_req: bool,
    pub cq: *mut bnxt_qplib_cq,
    pub hwq: bnxt_qplib_hwq,
    pub swq: *mut bnxt_qplib_swq,
    pub start_idx: c_int,
    pub last_idx: c_int,
    pub sg_info: bnxt_qplib_sg_info,
    pub eventq_hw_ring_id: u16,
    pub /: *mut *mut spinlock_t lock; / protect SRQE link list,
    pub toggle: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_sge {
    pub addr: u64,
    pub lkey: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_swq {
    pub wr_id: u64,
    pub next_idx: c_int,
    pub type: u8,
    pub flags: u8,
    pub start_psn: u32,
    pub next_psn: u32,
    pub slot_idx: u32,
    pub slots: u8,
    pub psn_search: *mut sq_psn_search,
    pub psn_ext: *mut sq_psn_search_ext,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_swqe {
// General
pub const BNXT_QPLIB_FENCE_WRID: c_uint = 0x46454E43	/* "FENC" */;
    pub wr_id: u64,
    pub reqs_type: u8,
    pub type: u8,
pub const BNXT_QPLIB_SWQE_TYPE_SEND: c_int = 0;
pub const BNXT_QPLIB_SWQE_TYPE_SEND_WITH_IMM: c_int = 1;
pub const BNXT_QPLIB_SWQE_TYPE_SEND_WITH_INV: c_int = 2;
pub const BNXT_QPLIB_SWQE_TYPE_RDMA_WRITE: c_int = 4;
pub const BNXT_QPLIB_SWQE_TYPE_RDMA_WRITE_WITH_IMM: c_int = 5;
pub const BNXT_QPLIB_SWQE_TYPE_RDMA_READ: c_int = 6;
pub const BNXT_QPLIB_SWQE_TYPE_ATOMIC_CMP_AND_SWP: c_int = 8;
pub const BNXT_QPLIB_SWQE_TYPE_ATOMIC_FETCH_AND_ADD: c_int = 11;
pub const BNXT_QPLIB_SWQE_TYPE_LOCAL_INV: c_int = 12;
pub const BNXT_QPLIB_SWQE_TYPE_FAST_REG_MR: c_int = 13;
pub const BNXT_QPLIB_SWQE_TYPE_REG_MR: c_int = 13;
pub const BNXT_QPLIB_SWQE_TYPE_BIND_MW: c_int = 14;
pub const BNXT_QPLIB_SWQE_TYPE_RECV: c_int = 128;
pub const BNXT_QPLIB_SWQE_TYPE_RECV_RDMA_IMM: c_int = 129;
    pub flags: u8,
    pub sg_list: [bnxt_qplib_sge; BNXT_VAR_MAX_SGE],
    pub num_sge: c_int,
// Max inline data is 96 bytes
    pub inline_len: u32,
pub const BNXT_QPLIB_SWQE_MAX_INLINE_LENGTH: c_int = 96;
    pub inline_data: [u8; BNXT_QPLIB_SWQE_MAX_INLINE_LENGTH],
// Send, with imm, inval key
    pub imm_data: u32,
    pub inv_key: u32,
}

// Send Raw Ethernet and QP1
// RDMA write, with imm, read
// Atomic cmp/swap, fetch/add
// Local Invalidate
// FR-PMR
pub const BNXT_QPLIB_SWQE_PAGE_SIZE_4K: c_int = 0;
pub const BNXT_QPLIB_SWQE_PAGE_SIZE_8K: c_int = 1;
pub const BNXT_QPLIB_SWQE_PAGE_SIZE_64K: c_int = 4;
pub const BNXT_QPLIB_SWQE_PAGE_SIZE_256K: c_int = 6;
pub const BNXT_QPLIB_SWQE_PAGE_SIZE_1M: c_int = 8;
pub const BNXT_QPLIB_SWQE_PAGE_SIZE_2M: c_int = 9;
pub const BNXT_QPLIB_SWQE_PAGE_SIZE_4M: c_int = 10;
pub const BNXT_QPLIB_SWQE_PAGE_SIZE_1G: c_int = 18;
pub const PAGE_SHIFT_4K: c_int = 12;
// Bind

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_q {
    pub hwq: bnxt_qplib_hwq,
    pub swq: *mut bnxt_qplib_swq,
    pub dbinfo: bnxt_qplib_db_info,
    pub sg_info: bnxt_qplib_sg_info,
    pub max_wqe: u32,
    pub max_sw_wqe: u32,
    pub wqe_size: u16,
    pub q_full_delta: u16,
    pub max_sge: u16,
    pub psn: u32,
    pub condition: bool,
    pub single: bool,
    pub send_phantom: bool,
    pub phantom_wqe_cnt: u32,
    pub phantom_cqe_cnt: u32,
    pub next_cq_cons: u32,
    pub flushed: bool,
    pub swq_start: u32,
    pub swq_last: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_qp {
    pub pd: *mut bnxt_qplib_pd,
    pub dpi: *mut bnxt_qplib_dpi,
    pub cctx: *mut bnxt_qplib_chip_ctx,
    pub qp_handle: u64,
pub const BNXT_QPLIB_QP_ID_INVALID: c_uint = 0xFFFFFFFF;
    pub id: u32,
    pub type: u8,
    pub sig_type: u8,
    pub wqe_mode: u8,
    pub state: u8,
    pub cur_qp_state: u8,
    pub is_user: u8,
    pub modify_flags: u64,
    pub ext_modify_flags: u32,
    pub max_inline_data: u32,
    pub mtu: u32,
    pub path_mtu: u8,
    pub en_sqd_async_notify: bool,
    pub pkey_index: u16,
    pub qkey: u32,
    pub dest_qp_id: u32,
    pub access: u8,
    pub timeout: u8,
    pub retry_cnt: u8,
    pub rnr_retry: u8,
    pub wqe_cnt: u64,
    pub min_rnr_timer: u32,
    pub max_rd_atomic: u32,
    pub max_dest_rd_atomic: u32,
    pub dest_qpn: u32,
    pub smac: [u8; 6],
    pub vlan_id: u16,
    pub port_id: u16,
    pub udp_sport: u16,
    pub nw_type: u8,
    pub ah: bnxt_qplib_ah,

// SQ
    pub sq: bnxt_qplib_q,
// RQ
    pub rq: bnxt_qplib_q,
// SRQ
    pub srq: *mut bnxt_qplib_srq,
// CQ
    pub scq: *mut bnxt_qplib_cq,
    pub rcq: *mut bnxt_qplib_cq,
// IRRQ and ORRQ
    pub irrq: bnxt_qplib_hwq,
    pub orrq: bnxt_qplib_hwq,
// Header buffer for QP1
    pub sq_hdr_buf_size: c_int,
    pub rq_hdr_buf_size: c_int,
//
// Buffer space for ETH(14), IP or GRH(40), UDP header(8)
// and ib_bth + ib_deth (20).
// Max required is 82 when RoCE V2 is enabled
//
pub const BNXT_QPLIB_MAX_QP1_SQ_HDR_SIZE_V2: c_int = 86;
// Ethernet header	=  14
// ib_grh		=  40 (provided by MAD)
// ib_bth + ib_deth	=  20
// MAD			= 256 (provided by MAD)
// iCRC			=   4
pub const BNXT_QPLIB_MAX_QP1_RQ_ETH_HDR_SIZE: c_int = 14;
pub const BNXT_QPLIB_MAX_QP1_RQ_HDR_SIZE_V2: c_int = 512;
pub const BNXT_QPLIB_MAX_GRH_HDR_SIZE_IPV4: c_int = 20;
pub const BNXT_QPLIB_MAX_GRH_HDR_SIZE_IPV6: c_int = 40;
pub const BNXT_QPLIB_MAX_QP1_RQ_BDETH_HDR_SIZE: c_int = 20;
    pub sq_hdr_buf: *mut c_void,
    pub sq_hdr_buf_map: dma_addr_t,
    pub rq_hdr_buf: *mut c_void,
    pub rq_hdr_buf_map: dma_addr_t,
    pub sq_flush: list_head,
    pub rq_flush: list_head,
    pub msn: u32,
    pub msn_tbl_sz: u32,
    pub psn_sz: u32,
    pub is_host_msn_tbl: bool,
    pub tos_dscp: u8,
    pub ugid_index: u32,
    pub dev_cap_flags: u16,
    pub rate_limit: u32,
    pub shaper_allocation_status: u8,
}

pub const BNXT_RE_MAX_MSG_SIZE: c_uint = 0x80000000;
pub const BNXT_RE_INVAL_MSG_SIZE: c_uint = 0xFFFFFFFF;

pub const ROCE_CQE_CMP_V: c_int = 0;

// False full is possible, retrying post-send makes sense
// CQ coalescing parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_cq_coal_param {
    pub buf_maxtime: u16,
    pub normal_maxbuf: u8,
    pub during_maxbuf: u8,
    pub en_ring_idle_mode: u8,
    pub enable: u8,
}

pub const BNXT_QPLIB_CQ_COAL_DEF_BUF_MAXTIME: c_uint = 0x1;
pub const BNXT_QPLIB_CQ_COAL_DEF_NORMAL_MAXBUF_P7: c_uint = 0x8;
pub const BNXT_QPLIB_CQ_COAL_DEF_DURING_MAXBUF_P7: c_uint = 0x8;
pub const BNXT_QPLIB_CQ_COAL_DEF_NORMAL_MAXBUF_P5: c_uint = 0x1;
pub const BNXT_QPLIB_CQ_COAL_DEF_DURING_MAXBUF_P5: c_uint = 0x1;
pub const BNXT_QPLIB_CQ_COAL_DEF_EN_RING_IDLE_MODE: c_uint = 0x1;
pub const BNXT_QPLIB_CQ_COAL_MAX_BUF_MAXTIME: c_uint = 0x1bf;
pub const BNXT_QPLIB_CQ_COAL_MAX_NORMAL_MAXBUF: c_uint = 0x1f;
pub const BNXT_QPLIB_CQ_COAL_MAX_DURING_MAXBUF: c_uint = 0x1f;
pub const BNXT_QPLIB_CQ_COAL_MAX_EN_RING_IDLE_MODE: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_cqe {
    pub status: u8,
    pub type: u8,
    pub opcode: u8,
    pub length: u32,
    pub cfa_meta: u16,
    pub wr_id: u64,
    pub immdata: u32,
    pub invrkey: u32,
}

pub const BNXT_QPLIB_QUEUE_START_PERIOD: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_cq {
    pub dpi: *mut bnxt_qplib_dpi,
    pub dbinfo: bnxt_qplib_db_info,
    pub max_wqe: u32,
    pub id: u32,
    pub count: u16,
    pub period: u16,
    pub hwq: bnxt_qplib_hwq,
    pub resize_hwq: bnxt_qplib_hwq,
    pub cnq_hw_ring_id: u32,
    pub nq: *mut bnxt_qplib_nq,
    pub resize_in_progress: bool,
    pub sg_info: bnxt_qplib_sg_info,
    pub cq_handle: u64,
    pub toggle: u8,
pub const CQ_RESIZE_WAIT_TIME_MS: c_int = 500;
    pub flags: c_ulong,
pub const CQ_FLAGS_RESIZE_IN_PROG: c_int = 1;
    pub waitq: wait_queue_head_t,
    pub rqf_head: list_head sqf_head,,
    pub arm_state: core::sync::atomic::AtomicI32,
    pub /: *mut *mut spinlock_t compl_lock; / synch CQ handlers,
// Locking Notes:
// QP can move to error state from modify_qp, async error event or error
// CQE as part of poll_cq. When QP is moved to error state, it gets added
// to two flush lists, one each for SQ and RQ.
// Each flush list is protected by qplib_cq->flush_lock. Both scq and rcq
// flush_locks should be acquired when QP is moved to error. The control path
// operations(modify_qp and async error events) are synchronized with poll_cq
// using upper level CQ locks (bnxt_re_cq->cq_lock) of both SCQ and RCQ.
// The qplib_cq->flush_lock is required to synchronize two instances of poll_cq
// of the same QP while manipulating the flush list.
//
    pub /: *mut *mut spinlock_t flush_lock; / QP flush management,
    pub cnq_events: u16,
    pub coalescing: *mut bnxt_qplib_cq_coal_param,
}

pub const NQ_CONS_PCI_BAR_REGION: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_nq_db {
    pub reg: bnxt_qplib_reg_desc,
    pub dbinfo: bnxt_qplib_db_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_nq {
    pub pdev: *mut pci_dev,
    pub res: *mut bnxt_qplib_res,
    pub name: *mut c_char,
    pub hwq: bnxt_qplib_hwq,
    pub nq_db: bnxt_qplib_nq_db,
    pub ring_id: u16,
    pub msix_vec: c_int,
    pub mask: cpumask_t,
    pub nq_tasklet: tasklet_struct,
    pub requested: bool,
    pub budget: c_int,
    pub load: u32,
    pub cqn_handler: cqn_handler_t,
    pub srqn_handler: srqn_handler_t,
    pub cqn_wq: *mut workqueue_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_nq_work {
    pub work: work_struct,
    pub nq: *mut bnxt_qplib_nq,
    pub cq: *mut bnxt_qplib_cq,
}

extern "C" {
    pub fn bnxt_qplib_nq_stop_irq(nq: *mut bnxt_qplib_nq, kill: bool);
}
extern "C" {
    pub fn bnxt_qplib_disable_nq(nq: *mut bnxt_qplib_nq);
}
extern "C" {
    pub fn bnxt_qplib_create_qp1(res: *mut bnxt_qplib_res, qp: *mut bnxt_qplib_qp) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_create_qp(res: *mut bnxt_qplib_res, qp: *mut bnxt_qplib_qp) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_modify_qp(res: *mut bnxt_qplib_res, qp: *mut bnxt_qplib_qp) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_query_qp(res: *mut bnxt_qplib_res, qp: *mut bnxt_qplib_qp) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_destroy_qp(res: *mut bnxt_qplib_res, qp: *mut bnxt_qplib_qp) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_clean_qp(qp: *mut bnxt_qplib_qp);
}
extern "C" {
    pub fn bnxt_qplib_get_rq_prod_index(qp: *mut bnxt_qplib_qp) -> u32;
}
extern "C" {
    pub fn bnxt_qplib_post_send_db(qp: *mut bnxt_qplib_qp);
}
extern "C" {
    pub fn bnxt_qplib_post_recv_db(qp: *mut bnxt_qplib_qp);
}
extern "C" {
    pub fn bnxt_qplib_create_cq(res: *mut bnxt_qplib_res, cq: *mut bnxt_qplib_cq) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_destroy_cq(res: *mut bnxt_qplib_res, cq: *mut bnxt_qplib_cq) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_is_cq_empty(cq: *mut bnxt_qplib_cq) -> bool;
}
extern "C" {
    pub fn bnxt_qplib_req_notify_cq(cq: *mut bnxt_qplib_cq, arm_type: u32);
}
extern "C" {
    pub fn bnxt_qplib_free_nq(nq: *mut bnxt_qplib_nq);
}
extern "C" {
    pub fn bnxt_qplib_alloc_nq(res: *mut bnxt_qplib_res, nq: *mut bnxt_qplib_nq) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_add_flush_qp(qp: *mut bnxt_qplib_qp);
}
extern "C" {
    pub fn bnxt_qplib_flush_cqn_wq(qp: *mut bnxt_qplib_qp);
}
extern "C" {
    pub fn bnxt_re_synchronize_nq(nq: *mut bnxt_qplib_nq);
}
// swq_idx = idx;
extern "C" {
    pub fn sizeof(sq_sge: struct) -> return;
}
// Queue depth is the number of slots.
// For variable WQE mode, need to align the slots to 256
// For Cu/Wh delta = 128, stride = 16, wqe_bytes = 128
// For Gen-p5 B/C mode delta = 0, stride = 16, wqe_bytes = 128.
// For Gen-p5 delta = 0, stride = 16, 32 <= wqe_bytes <= 512.
// when 8916 is disabled.
//
// MSN table update inlin
