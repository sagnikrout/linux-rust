//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/rxe/rxe_verbs.h
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
//
// Copyright (c) 2016 Mellanox Technologies Ltd. All rights reserved.
// Copyright (c) 2015 System Fabric Works, Inc. All rights reserved.
//

// Return >0 if psn_a > psn_b
// 0 if psn_a == psn_b
// <0 if psn_a < psn_b
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_ucontext {
    pub ibuc: ib_ucontext,
    pub elem: rxe_pool_elem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_pd {
    pub ibpd: ib_pd,
    pub elem: rxe_pool_elem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_ah {
    pub ibah: ib_ah,
    pub elem: rxe_pool_elem,
    pub av: rxe_av,
    pub is_user: bool,
    pub ah_num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_cqe {
    pub ibwc: ib_wc,
    pub uibwc: ib_uverbs_wc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_cq {
    pub ibcq: ib_cq,
    pub elem: rxe_pool_elem,
    pub queue: *mut rxe_queue,
    pub cq_lock: spinlock_t,
    pub notify: u8,
    pub is_user: bool,
    pub num_wq: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wqe_state {
    wqe_state_posted,
    wqe_state_processing,
    wqe_state_pending,
    wqe_state_done,
    wqe_state_error,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_sq {
    pub max_wr: c_int,
    pub max_sge: c_int,
    pub max_inline: c_int,
    pub /: *mut *mut spinlock_t sq_lock; / guard queue,
    pub queue: *mut rxe_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_rq {
    pub max_wr: c_int,
    pub max_sge: c_int,
    pub /: *mut *mut spinlock_t producer_lock; / guard queue producer,
    pub /: *mut *mut spinlock_t consumer_lock; / guard queue consumer,
    pub queue: *mut rxe_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_srq {
    pub ibsrq: ib_srq,
    pub elem: rxe_pool_elem,
    pub pd: *mut rxe_pd,
    pub rq: rxe_rq,
    pub srq_num: u32,
    pub limit: c_int,
    pub error: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_req_info {
    pub wqe_index: c_int,
    pub psn: u32,
    pub opcode: c_int,
    pub rd_atomic: core::sync::atomic::AtomicI32,
    pub wait_fence: c_int,
    pub need_rd_atomic: c_int,
    pub wait_psn: c_int,
    pub need_retry: c_int,
    pub wait_for_rnr_timer: c_int,
    pub noack_pkts: c_int,
    pub again: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_comp_info {
    pub psn: u32,
    pub opcode: c_int,
    pub timeout: c_int,
    pub timeout_retry: c_int,
    pub started_retry: c_int,
    pub retry_cnt: u32,
    pub rnr_retry: u32,
}

// responder states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resp_states {
    RESPST_NONE,
    RESPST_GET_REQ,
    RESPST_CHK_PSN,
    RESPST_CHK_OP_SEQ,
    RESPST_CHK_OP_VALID,
    RESPST_CHK_RESOURCE,
    RESPST_CHK_LENGTH,
    RESPST_CHK_RKEY,
    RESPST_EXECUTE,
    RESPST_READ_REPLY,
    RESPST_ATOMIC_REPLY,
    RESPST_ATOMIC_WRITE_REPLY,
    RESPST_PROCESS_FLUSH,
    RESPST_COMPLETE,
    RESPST_ACKNOWLEDGE,
    RESPST_CLEANUP,
    RESPST_DUPLICATE_REQUEST,
    RESPST_ERR_MALFORMED_WQE,
    RESPST_ERR_UNSUPPORTED_OPCODE,
    RESPST_ERR_MISALIGNED_ATOMIC,
    RESPST_ERR_PSN_OUT_OF_SEQ,
    RESPST_ERR_MISSING_OPCODE_FIRST,
    RESPST_ERR_MISSING_OPCODE_LAST_C,
    RESPST_ERR_MISSING_OPCODE_LAST_D1E,
    RESPST_ERR_TOO_MANY_RDMA_ATM_REQ,
    RESPST_ERR_RNR,
    RESPST_ERR_RKEY_VIOLATION_EVENT,
    RESPST_ERR_RKEY_VIOLATION,
    RESPST_ERR_INVALIDATE_RKEY,
    RESPST_ERR_LENGTH,
    RESPST_ERR_CQ_OVERFLOW,
    RESPST_ERROR,
    RESPST_DONE,
    RESPST_EXIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdatm_res_state {
    rdatm_res_state_next,
    rdatm_res_state_new,
    rdatm_res_state_replay,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resp_res {
    pub type: c_int,
    pub replay: c_int,
    pub first_psn: u32,
    pub last_psn: u32,
    pub cur_psn: u32,
    pub state: rdatm_res_state,
    pub orig_val: u64,
    pub atomic: },
    pub va_org: u64,
    pub rkey: u32,
    pub length: u32,
    pub va: u64,
    pub resid: u32,
    pub read: },
    pub length: u32,
    pub va: u64,
    pub type: u8,
    pub level: u8,
    pub flush: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_resp_info {
    pub msn: u32,
    pub psn: u32,
    pub ack_psn: u32,
    pub opcode: c_int,
    pub drop_msg: c_int,
    pub goto_error: c_int,
    pub sent_psn_nak: c_int,
    pub status: ib_wc_status,
    pub aeth_syndrome: u8,
// Receive only
    pub wqe: *mut rxe_recv_wqe,
// RDMA read / atomic only
    pub va: u64,
    pub offset: u64,
    pub mr: *mut rxe_mr,
    pub resid: u32,
    pub rkey: u32,
    pub length: u32,
// SRQ only
    pub wqe: rxe_recv_wqe,
    pub sge: [ib_sge; RXE_MAX_SGE],
    pub srq_wqe: },
// Responder resources. It's a circular list where the oldest
// resource is dropped first.
//
    pub resources: *mut resp_res,
    pub res_head: c_uint,
    pub res_tail: c_uint,
    pub res: *mut resp_res,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_qp {
    pub ibqp: ib_qp,
    pub elem: rxe_pool_elem,
    pub attr: ib_qp_attr,
    pub valid: c_uint,
    pub mtu: c_uint,
    pub is_user: bool,
    pub pd: *mut rxe_pd,
    pub srq: *mut rxe_srq,
    pub scq: *mut rxe_cq,
    pub rcq: *mut rxe_cq,
    pub sq_sig_type: ib_sig_type,
    pub sq: rxe_sq,
    pub rq: rxe_rq,
    pub sk: *mut socket,
    pub dst_cookie: u32,
    pub src_port: u16,
    pub pri_av: rxe_av,
    pub alt_av: rxe_av,
    pub mcg_num: core::sync::atomic::AtomicI32,
    pub req_pkts: sk_buff_head,
    pub resp_pkts: sk_buff_head,
    pub send_task: rxe_task,
    pub recv_task: rxe_task,
    pub req: rxe_req_info,
    pub comp: rxe_comp_info,
    pub resp: rxe_resp_info,
    pub ssn: core::sync::atomic::AtomicI32,
    pub skb_out: core::sync::atomic::AtomicI32,
    pub need_req_skb: c_int,
// Timer for retranmitting packet when ACKs have been lost. RC
// only. The requester sets it when it is not already
// started. The responder resets it whenever an ack is
// received.
//
    pub retrans_timer: timer_list,
    pub qp_timeout_jiffies: u64,
// Timer for handling RNR NAKS.
    pub rnr_nak_timer: timer_list,
    pub /: *mut *mut spinlock_t state_lock; / guard requester and completer,
    pub cleanup_work: execute_work,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxe_mr_state {
    RXE_MR_STATE_INVALID,
    RXE_MR_STATE_FREE,
    RXE_MR_STATE_VALID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxe_mr_copy_dir {
    RXE_TO_MR_OBJ,
    RXE_FROM_MR_OBJ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxe_mr_lookup_type {
    RXE_LOOKUP_LOCAL,
    RXE_LOOKUP_REMOTE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxe_rereg {
    RXE_MR_REREG_SUPPORTED	= IB_MR_REREG_PD
    | IB_MR_REREG_ACCESS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_mr_page {
    pub page: *mut page,
    pub /: *mut *mut unsigned int offset; / offset in system page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_mr {
    pub elem: rxe_pool_elem,
    pub ibmr: ib_mr,
    pub umem: *mut ib_umem,
    pub lkey: u32,
    pub rkey: u32,
    pub state: rxe_mr_state,
    pub access: c_int,
    pub num_mw: core::sync::atomic::AtomicI32,
    pub page_shift: c_uint,
    pub page_mask: u64,
// size of page_info when mr allocated
    pub num_buf: u32,
// real size of page_info
    pub max_allowed_buf: u32,
    pub nbuf: u32,
    pub page_info: *mut rxe_mr_page,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxe_mw_state {
    RXE_MW_STATE_INVALID	= RXE_MR_STATE_INVALID,
    RXE_MW_STATE_FREE	= RXE_MR_STATE_FREE,
    RXE_MW_STATE_VALID	= RXE_MR_STATE_VALID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_mw {
    pub ibmw: ib_mw,
    pub elem: rxe_pool_elem,
    pub lock: spinlock_t,
    pub state: rxe_mw_state,
    pub /: *mut *mut *mut rxe_qp qp; / Type 2 only,
    pub mr: *mut rxe_mr,
    pub rkey: u32,
    pub access: c_int,
    pub addr: u64,
    pub length: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_mcg {
    pub node: rb_node,
    pub ref_cnt: kref,
    pub rxe: *mut rxe_dev,
    pub qp_list: list_head,
    pub mgid: ib_gid,
    pub qp_num: core::sync::atomic::AtomicI32,
    pub qkey: u32,
    pub pkey: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_mca {
    pub qp_list: list_head,
    pub qp: *mut rxe_qp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_port {
    pub attr: ib_port_attr,
    pub port_guid: __be64,
    pub subnet_prefix: __be64,
    pub /: *mut *mut spinlock_t port_lock; / guard port,
    pub mtu_cap: c_uint,
// special QPs
    pub qp_gsi_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_dev {
    pub ib_dev: ib_device,
    pub attr: ib_device_attr,
    pub max_ucontext: c_int,
    pub max_inline_data: c_int,
    pub usdev_lock: mutex,
    pub raw_gid: [c_char; ETH_ALEN],
    pub uc_pool: rxe_pool,
    pub pd_pool: rxe_pool,
    pub ah_pool: rxe_pool,
    pub srq_pool: rxe_pool,
    pub qp_pool: rxe_pool,
    pub cq_pool: rxe_pool,
    pub mr_pool: rxe_pool,
    pub mw_pool: rxe_pool,
// multicast support
    pub mcg_lock: spinlock_t,
    pub mcg_tree: rb_root,
    pub mcg_num: core::sync::atomic::AtomicI32,
    pub mcg_attach: core::sync::atomic::AtomicI32,
    pub /: *mut *mut spinlock_t pending_lock; / guard pending_mmaps,
    pub pending_mmaps: list_head,
    pub /: *mut *mut spinlock_t mmap_offset_lock; / guard mmap_offset,
    pub mmap_offset: u64,
    pub stats_counters: [core::sync::atomic::AtomicI64; RXE_NUM_OF_COUNTERS],
    pub port: rxe_port,
}

extern "C" {
    pub fn to_rpd(_arg: ah->ibah.pd) -> return;
}
extern "C" {
    pub fn to_rpd(_arg: mr->ibmr.pd) -> return;
}
extern "C" {
    pub fn to_rpd(_arg: mw->ibmw.pd) -> return;
}
