//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/tid_rdma.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Copyright(c) 2018 Intel Corporation.
//

// Add a convenience helper

pub const TID_RDMA_SEGMENT_SHIFT: c_int = 18;
//
// Bit definitions for priv->s_flags.
// These bit flags overload the bit flags defined for the QP's s_flags.
// Due to the fact that these bit fields are used only for the QP priv
// s_flags, there are no collisions.
//
// HFI1_S_TID_WAIT_INTERLCK - QP is waiting for requester interlock
// HFI1_R_TID_WAIT_INTERLCK - QP is waiting for responder interlock
//

// BIT(1) reserved for RVT_S_BUSY.

// BIT(3) reserved for RVT_S_RESP_PENDING.
// BIT(4) reserved for RVT_S_ACK_PENDING.

// BIT(7) - BIT(15) reserved for RVT_S_WAIT_*.
// BIT(16) reserved for RVT_S_SEND_ONE

// BIT(18) reserved for RVT_S_ECN.

// BIT(26) reserved for HFI1_S_WAIT_HALT
// BIT(27) reserved for HFI1_S_WAIT_TID_RESP
// BIT(28) reserved for HFI1_S_WAIT_TID_SPACE
//
// Unlike regular IB RDMA VERBS, which do not require an entry
// in the s_ack_queue, TID RDMA WRITE requests do because they
// generate responses.
// Therefore, the s_ack_queue needs to be extended by a certain
// amount. The key point is that the queue needs to be extended
// without letting the "user" know so they user doesn't end up
// using these extra entries.
//
pub const HFI1_TID_RDMA_WRITE_CNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rdma_params {
    pub rcu_head: rcu_head,
    pub qp: u32,
    pub max_len: u32,
    pub jkey: u16,
    pub max_read: u8,
    pub max_write: u8,
    pub timeout: u8,
    pub urg: u8,
    pub version: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rdma_qp_params {
    pub trigger_work: work_struct,
    pub local: tid_rdma_params,
    pub remote: *mut tid_rdma_params __rcu,
}

// Track state for each hardware flow
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_flow_state {
    pub generation: u32,
    pub psn: u32,
    pub index: u8,
    pub last_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tid_rdma_req_state {
    TID_REQUEST_INACTIVE = 0,
    TID_REQUEST_INIT,
    TID_REQUEST_INIT_RESEND,
    TID_REQUEST_ACTIVE,
    TID_REQUEST_RESEND,
    TID_REQUEST_RESEND_ACTIVE,
    TID_REQUEST_QUEUED,
    TID_REQUEST_SYNC,
    TID_REQUEST_RNR_NAK,
    TID_REQUEST_COMPLETE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rdma_request {
    pub qp: *mut rvt_qp,
    pub rcd: *mut hfi1_ctxtdata,
    pub swqe: *mut rvt_swqe,
    pub ack: *mut rvt_ack_entry,
    pub e: },
    pub /: *mut *mut *mut tid_rdma_flow flows; / array of tid flows,
    pub /: *mut *mut rvt_sge_state ss; / SGE state for TID RDMA requests,
    pub /: *mut *mut u16 n_flows; / size of the flow buffer window,
    pub /: *mut *mut u16 setup_head; / flow index we are setting up,
    pub /: *mut *mut u16 clear_tail; / flow index we are clearing,
    pub /: *mut *mut u16 flow_idx; / flow index most recently set up,
    pub acked_tail: u16,
    pub seg_len: u32,
    pub total_len: u32,
    pub /: *mut *mut u32 r_ack_psn; / next expected ack PSN,
    pub /: *mut *mut u32 r_flow_psn; / IB PSN of next segment start,
    pub /: *mut *mut u32 r_last_acked; / IB PSN of last ACK'ed packet,
    pub /: *mut *mut u32 s_next_psn; / IB PSN of next segment start for read,
    pub /: *mut *mut u32 total_segs; / segments required to complete a request,
    pub /: *mut *mut u32 cur_seg; / index of current segment,
    pub /: *mut *mut u32 comp_seg; / index of last completed segment,
    pub /: *mut *mut u32 ack_seg; / index of last ack'ed segment,
    pub /: *mut *mut u32 alloc_seg; / index of next segment to be allocated,
    pub /: *mut *mut u32 isge; / index of "current" sge,
    pub /: *mut *mut u32 ack_pending; / num acks pending for this request,
    pub state: tid_rdma_req_state,
}

//
// When header suppression is used, PSNs associated with a "flow" are
// relevant (and not the PSNs maintained by verbs). Track per-flow
// PSNs here for a TID RDMA segment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_state {
    pub flags: u32,
    pub /: *mut *mut u32 resp_ib_psn; / The IB PSN of the response for this flow,
    pub /: *mut *mut u32 generation; / generation of flow,
    pub /: *mut *mut u32 spsn; / starting PSN in TID space,
    pub /: *mut *mut u32 lpsn; / last PSN in TID space,
    pub /: *mut *mut u32 r_next_psn; / next PSN to be received (in TID space),
// For tid rdma read
    pub /: *mut *mut u32 ib_spsn; / starting PSN in Verbs space,
    pub /: *mut *mut u32 ib_lpsn; / last PSn in Verbs space,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rdma_pageset {
    pub /: *mut *mut dma_addr_t addr : 48; / Only needed for the first page,
    pub 8: u8 idx:,
    pub 7: u8 count :,
    pub 1: u8 mapped:,
}

//
// kern_tid_node - used for managing TID's in TID groups
//
// @grp_idx: rcd relative index to tid_group
// @map: grp->map captured prior to programming this TID group in HW
// @cnt: Only @cnt of available group entries are actually programmed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_tid_node {
    pub grp: *mut tid_group,
    pub map: u8,
    pub cnt: u8,
}

// Overall info for a TID RDMA segment
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rdma_flow {
//
// While a TID RDMA segment is being transferred, it uses a QP number
// from the "KDETH section of QP numbers" (which is different from the
// QP number that originated the request). Bits 11-15 of these QP
// numbers identify the "TID flow" for the segment.
//
    pub flow_state: flow_state,
    pub req: *mut tid_rdma_request,
    pub tid_qpn: u32,
    pub tid_offset: u32,
    pub length: u32,
    pub sent: u32,
    pub tnode_cnt: u8,
    pub tidcnt: u8,
    pub tid_idx: u8,
    pub idx: u8,
    pub npagesets: u8,
    pub npkts: u8,
    pub pkt: u8,
    pub resync_npkts: u8,
    pub tnode: [kern_tid_node; TID_RDMA_MAX_PAGES],
    pub pagesets: [tid_rdma_pageset; TID_RDMA_MAX_PAGES],
    pub tid_entry: [u32; TID_RDMA_MAX_PAGES],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tid_rnr_nak_state {
    TID_RNR_NAK_INIT = 0,
    TID_RNR_NAK_SEND,
    TID_RNR_NAK_SENT,
}

extern "C" {
    pub fn tid_rdma_conn_req(qp: *mut rvt_qp, data: *mut u64) -> bool;
}
extern "C" {
    pub fn tid_rdma_conn_reply(qp: *mut rvt_qp, data: u64) -> bool;
}
extern "C" {
    pub fn tid_rdma_conn_resp(qp: *mut rvt_qp, data: *mut u64) -> bool;
}
extern "C" {
    pub fn tid_rdma_conn_error(qp: *mut rvt_qp);
}
extern "C" {
    pub fn tid_rdma_opfn_init(qp: *mut rvt_qp, p: *mut tid_rdma_params);
}
extern "C" {
    pub fn hfi1_kern_exp_rcv_init(rcd: *mut hfi1_ctxtdata, reinit: c_int) -> c_int;
}
extern "C" {
    pub fn hfi1_kern_exp_rcv_clear(req: *mut tid_rdma_request) -> c_int;
}
extern "C" {
    pub fn hfi1_kern_exp_rcv_clear_all(req: *mut tid_rdma_request);
}
extern "C" {
    pub fn __trdma_clean_swqe(qp: *mut rvt_qp, wqe: *mut rvt_swqe);
}
//
// trdma_clean_swqe - clean flows for swqe if large send queue
// @qp: the qp
// @wqe: the send wqe
//
extern "C" {
    pub fn hfi1_kern_read_tid_flow_free(qp: *mut rvt_qp);
}
extern "C" {
    pub fn hfi1_qp_priv_tid_free(rdi: *mut rvt_dev_info, qp: *mut rvt_qp);
}
extern "C" {
    pub fn hfi1_tid_rdma_flush_wait(qp: *mut rvt_qp);
}
extern "C" {
    pub fn hfi1_kern_setup_hw_flow(rcd: *mut hfi1_ctxtdata, qp: *mut rvt_qp) -> c_int;
}
extern "C" {
    pub fn hfi1_kern_clear_hw_flow(rcd: *mut hfi1_ctxtdata, qp: *mut rvt_qp);
}
extern "C" {
    pub fn hfi1_kern_init_ctxt_generations(rcd: *mut hfi1_ctxtdata);
}
extern "C" {
    pub fn hfi1_rc_rcv_tid_rdma_read_req(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_rc_rcv_tid_rdma_read_resp(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_qp_kern_exp_rcv_clear_all(qp: *mut rvt_qp);
}
extern "C" {
    pub fn hfi1_tid_rdma_wqe_interlock(qp: *mut rvt_qp, wqe: *mut rvt_swqe) -> bool;
}
extern "C" {
    pub fn setup_tid_rdma_wqe(qp: *mut rvt_qp, wqe: *mut rvt_swqe);
}
extern "C" {
    pub fn hfi1_rc_rcv_tid_rdma_write_req(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_del_tid_reap_timer(qp: *mut rvt_qp);
}
extern "C" {
    pub fn hfi1_rc_rcv_tid_rdma_write_resp(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_rc_rcv_tid_rdma_write_data(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_rc_rcv_tid_rdma_ack(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_add_tid_retry_timer(qp: *mut rvt_qp);
}
extern "C" {
    pub fn hfi1_del_tid_retry_timer(qp: *mut rvt_qp);
}
extern "C" {
    pub fn hfi1_rc_rcv_tid_rdma_resync(packet: *mut hfi1_packet);
}
extern "C" {
    pub fn hfi1_make_tid_rdma_pkt(qp: *mut rvt_qp, ps: *mut hfi1_pkt_state) -> c_int;
}
extern "C" {
    pub fn _hfi1_do_tid_send(work: *mut work_struct);
}
extern "C" {
    pub fn hfi1_schedule_tid_send(qp: *mut rvt_qp) -> bool;
}
extern "C" {
    pub fn hfi1_tid_rdma_ack_interlock(qp: *mut rvt_qp, e: *mut rvt_ack_entry) -> bool;
}
