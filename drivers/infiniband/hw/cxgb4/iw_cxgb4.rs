//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/cxgb4/iw_cxgb4.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_id_table {
    pub flags: u32,
    pub /: *mut *mut u32 start; / logical minimal id,
    pub /: *mut *mut u32 last; / hint for find,
    pub max: u32,
    pub lock: spinlock_t,
    pub table: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_resource {
    pub tpt_table: c4iw_id_table,
    pub qid_table: c4iw_id_table,
    pub pdid_table: c4iw_id_table,
    pub srq_table: c4iw_id_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_qid_list {
    pub entry: list_head,
    pub qid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_dev_ucontext {
    pub qpids: list_head,
    pub cqids: list_head,
    pub lock: mutex,
    pub kref: kref,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c4iw_rdev_flags {
    T4_FATAL_ERROR = (1<<0),
    T4_STATUS_PAGE_DISABLED = (1<<1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_stat {
    pub total: u64,
    pub cur: u64,
    pub max: u64,
    pub fail: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_stats {
    pub lock: mutex,
    pub qid: c4iw_stat,
    pub pd: c4iw_stat,
    pub stag: c4iw_stat,
    pub pbl: c4iw_stat,
    pub rqt: c4iw_stat,
    pub srqt: c4iw_stat,
    pub srq: c4iw_stat,
    pub ocqp: c4iw_stat,
    pub db_full: u64,
    pub db_empty: u64,
    pub db_drop: u64,
    pub db_state_transitions: u64,
    pub db_fc_interruptions: u64,
    pub tcam_full: u64,
    pub act_ofld_conn_fails: u64,
    pub pas_ofld_conn_fails: u64,
    pub neg_adv: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_hw_queue {
    pub t4_eq_status_entries: c_int,
    pub t4_max_eq_size: c_int,
    pub t4_max_iq_size: c_int,
    pub t4_max_rq_size: c_int,
    pub t4_max_sq_size: c_int,
    pub t4_max_qp_depth: c_int,
    pub t4_max_cq_depth: c_int,
    pub t4_stat_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wr_log_entry {
    pub post_host_time: ktime_t,
    pub poll_host_time: ktime_t,
    pub post_sge_ts: u64,
    pub cqe_sge_ts: u64,
    pub poll_sge_ts: u64,
    pub qid: u16,
    pub wr_id: u16,
    pub opcode: u8,
    pub valid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_rdev {
    pub resource: c4iw_resource,
    pub qpmask: u32,
    pub cqmask: u32,
    pub uctx: c4iw_dev_ucontext,
    pub pbl_pool: *mut gen_pool,
    pub rqt_pool: *mut gen_pool,
    pub ocqp_pool: *mut gen_pool,
    pub flags: u32,
    pub lldi: cxgb4_lld_info,
    pub bar2_pa: c_ulong,
    pub bar2_kva: *mut void __iomem,
    pub oc_mw_pa: c_ulong,
    pub oc_mw_kva: *mut void __iomem,
    pub stats: c4iw_stats,
    pub hw_queue: c4iw_hw_queue,
    pub status_page: *mut t4_dev_status_page,
    pub wr_log_idx: core::sync::atomic::AtomicI32,
    pub wr_log: *mut wr_log_entry,
    pub wr_log_size: c_int,
    pub free_workq: *mut workqueue_struct,
    pub rqt_compl: completion,
    pub pbl_compl: completion,
    pub rqt_kref: kref,
    pub pbl_kref: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_wr_wait {
    pub completion: completion,
    pub ret: c_int,
    pub kref: kref,
}

extern "C" {
    pub fn _c4iw_free_wr_wait(kref: *mut kref);
}
extern "C" {
    pub fn c4iw_ofld_send(rdev: *mut c4iw_rdev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn c4iw_wait_for_reply(_arg: rdev, _arg: wr_waitp, _arg: hwtid, _arg: qpid, _arg: func) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum db_state {
    NORMAL = 0,
    FLOW_CONTROL = 1,
    RECOVERY = 2,
    STOPPED = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_dev {
    pub ibdev: ib_device,
    pub rdev: c4iw_rdev,
    pub cqs: xarray,
    pub qps: xarray,
    pub mrs: xarray,
    pub db_mutex: mutex,
    pub debugfs_root: *mut dentry,
    pub db_state: db_state,
    pub hwtids: xarray,
    pub atids: xarray,
    pub stids: xarray,
    pub db_fc_list: list_head,
    pub avail_ird: u32,
    pub wait: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uld_ctx {
    pub entry: list_head,
    pub lldi: cxgb4_lld_info,
    pub dev: *mut c4iw_dev,
    pub reg_work: work_struct,
}

extern "C" {
    pub fn container_of(_arg: ibdev, c4iw_dev: struct, _arg: ibdev) -> return;
}
extern "C" {
    pub fn xa_load(_arg: &rhp->cqs, _arg: cqid) -> return;
}
extern "C" {
    pub fn xa_load(_arg: &rhp->qps, _arg: qpid) -> return;
}
extern "C" {
    pub fn min(_arg: dev->rdev.lldi.max_ordird_qp, _arg: c4iw_max_read_depth) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_pd {
    pub ibpd: ib_pd,
    pub pdid: u32,
    pub rhp: *mut c4iw_dev,
}

extern "C" {
    pub fn container_of(_arg: ibpd, c4iw_pd: struct, _arg: ibpd) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpt_attributes {
    pub len: u64,
    pub va_fbo: u64,
    pub perms: fw_ri_mem_perms,
    pub stag: u32,
    pub pdid: u32,
    pub qpid: u32,
    pub pbl_addr: u32,
    pub pbl_size: u32,
    pub state:1: u32,
    pub type:2: u32,
    pub rsvd:1: u32,
    pub remote_invaliate_disable:1: u32,
    pub zbva:1: u32,
    pub mw_bind_enable:1: u32,
    pub page_size:5: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_mr {
    pub ibmr: ib_mr,
    pub umem: *mut ib_umem,
    pub rhp: *mut c4iw_dev,
    pub dereg_skb: *mut sk_buff,
    pub kva: u64,
    pub attr: tpt_attributes,
    pub mpl: *mut u64,
    pub mpl_addr: dma_addr_t,
    pub max_mpl_len: u32,
    pub mpl_len: u32,
    pub wr_waitp: *mut c4iw_wr_wait,
}

extern "C" {
    pub fn container_of(_arg: ibmr, c4iw_mr: struct, _arg: ibmr) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_mw {
    pub ibmw: ib_mw,
    pub rhp: *mut c4iw_dev,
    pub dereg_skb: *mut sk_buff,
    pub kva: u64,
    pub attr: tpt_attributes,
    pub wr_waitp: *mut c4iw_wr_wait,
}

extern "C" {
    pub fn container_of(_arg: ibmw, c4iw_mw: struct, _arg: ibmw) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_cq {
    pub ibcq: ib_cq,
    pub rhp: *mut c4iw_dev,
    pub destroy_skb: *mut sk_buff,
    pub cq: t4_cq,
    pub lock: spinlock_t,
    pub comp_handler_lock: spinlock_t,
    pub refcnt: refcount_t,
    pub cq_rel_comp: completion,
    pub wr_waitp: *mut c4iw_wr_wait,
}

extern "C" {
    pub fn container_of(_arg: ibcq, c4iw_cq: struct, _arg: ibcq) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_mpa_attributes {
    pub initiator: u8,
    pub recv_marker_enabled: u8,
    pub xmit_marker_enabled: u8,
    pub crc_enabled: u8,
    pub enhanced_rdma_conn: u8,
    pub version: u8,
    pub p2p_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_qp_attributes {
    pub scq: u32,
    pub rcq: u32,
    pub sq_num_entries: u32,
    pub rq_num_entries: u32,
    pub sq_max_sges: u32,
    pub sq_max_sges_rdma_write: u32,
    pub rq_max_sges: u32,
    pub state: u32,
    pub enable_rdma_read: u8,
    pub enable_rdma_write: u8,
    pub enable_bind: u8,
    pub enable_mmid0_fastreg: u8,
    pub max_ord: u32,
    pub max_ird: u32,
    pub pd: u32,
    pub next_state: u32,
    pub terminate_buffer: [c_char; 52],
    pub terminate_msg_len: u32,
    pub is_terminate_local: u8,
    pub mpa_attr: c4iw_mpa_attributes,
    pub llp_stream_handle: *mut c4iw_ep,
    pub layer_etype: u8,
    pub ecode: u8,
    pub sq_db_inc: u16,
    pub rq_db_inc: u16,
    pub send_term: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_qp {
    pub ibqp: ib_qp,
    pub db_fc_entry: list_head,
    pub rhp: *mut c4iw_dev,
    pub ep: *mut c4iw_ep,
    pub attr: c4iw_qp_attributes,
    pub wq: t4_wq,
    pub lock: spinlock_t,
    pub mutex: mutex,
    pub wait: wait_queue_head_t,
    pub sq_sig_all: c_int,
    pub srq: *mut c4iw_srq,
    pub ucontext: *mut c4iw_ucontext,
    pub wr_waitp: *mut c4iw_wr_wait,
    pub qp_rel_comp: completion,
    pub qp_refcnt: refcount_t,
}

extern "C" {
    pub fn container_of(_arg: ibqp, c4iw_qp: struct, _arg: ibqp) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_srq {
    pub ibsrq: ib_srq,
    pub db_fc_entry: list_head,
    pub rhp: *mut c4iw_dev,
    pub wq: t4_srq,
    pub destroy_skb: *mut sk_buff,
    pub srq_limit: u32,
    pub pdid: u32,
    pub idx: c_int,
    pub flags: u32,
    pub /: *mut *mut spinlock_t lock; / protects srq,
    pub wr_waitp: *mut c4iw_wr_wait,
    pub armed: bool,
}

extern "C" {
    pub fn container_of(_arg: ibsrq, c4iw_srq: struct, _arg: ibsrq) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_ucontext {
    pub ibucontext: ib_ucontext,
    pub uctx: c4iw_dev_ucontext,
    pub key: u32,
    pub mmap_lock: spinlock_t,
    pub mmaps: list_head,
    pub is_32b_cqe: bool,
}

extern "C" {
    pub fn container_of(_arg: c, c4iw_ucontext: struct, _arg: ibucontext) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_mm_entry {
    pub entry: list_head,
    pub addr: u64,
    pub key: u32,
    pub vaddr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub len: unsigned,
    pub mmap_flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c4iw_qp_attr_mask {
    C4IW_QP_ATTR_NEXT_STATE = 1 << 0,
    C4IW_QP_ATTR_SQ_DB = 1<<1,
    C4IW_QP_ATTR_RQ_DB = 1<<2,
    C4IW_QP_ATTR_ENABLE_RDMA_READ = 1 << 7,
    C4IW_QP_ATTR_ENABLE_RDMA_WRITE = 1 << 8,
    C4IW_QP_ATTR_ENABLE_RDMA_BIND = 1 << 9,
    C4IW_QP_ATTR_MAX_ORD = 1 << 11,
    C4IW_QP_ATTR_MAX_IRD = 1 << 12,
    C4IW_QP_ATTR_LLP_STREAM_HANDLE = 1 << 22,
    C4IW_QP_ATTR_STREAM_MSG_BUFFER = 1 << 23,
    C4IW_QP_ATTR_MPA_ATTR = 1 << 24,
    C4IW_QP_ATTR_QP_CONTEXT_ACTIVATE = 1 << 25,
    C4IW_QP_ATTR_VALID_MODIFY = (C4IW_QP_ATTR_ENABLE_RDMA_READ |
    C4IW_QP_ATTR_ENABLE_RDMA_WRITE |
    C4IW_QP_ATTR_MAX_ORD |
    C4IW_QP_ATTR_MAX_IRD |
    C4IW_QP_ATTR_LLP_STREAM_HANDLE |
    C4IW_QP_ATTR_STREAM_MSG_BUFFER |
    C4IW_QP_ATTR_MPA_ATTR |
    C4IW_QP_ATTR_QP_CONTEXT_ACTIVATE)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c4iw_qp_state {
    C4IW_QP_STATE_IDLE,
    C4IW_QP_STATE_RTS,
    C4IW_QP_STATE_ERROR,
    C4IW_QP_STATE_TERMINATE,
    C4IW_QP_STATE_CLOSING,
    C4IW_QP_STATE_TOT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c4iw_mmid_state {
    C4IW_STAG_STATE_VALID,
    C4IW_STAG_STATE_INVALID
}

pub const MPA_MAX_PRIVATE_DATA: c_int = 256;
pub const MPA_ENHANCED_RDMA_CONN: c_uint = 0x10;
pub const MPA_REJECT: c_uint = 0x20;
pub const MPA_CRC: c_uint = 0x40;
pub const MPA_MARKERS: c_uint = 0x80;
pub const MPA_FLAGS_MASK: c_uint = 0xE0;
pub const MPA_V2_PEER2PEER_MODEL: c_uint = 0x8000;
pub const MPA_V2_ZERO_LEN_FPDU_RTR: c_uint = 0x4000;
pub const MPA_V2_RDMA_WRITE_RTR: c_uint = 0x8000;
pub const MPA_V2_RDMA_READ_RTR: c_uint = 0x4000;
pub const MPA_V2_IRD_ORD_MASK: c_uint = 0x3FFF;

extern "C" {
    pub fn _c4iw_free_ep(kref: *mut kref);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpa_message {
    pub key: [u8; 16],
    pub flags: u8,
    pub revision: u8,
    pub private_data_size: __be16,
    pub private_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpa_v2_conn_params {
    pub ird: __be16,
    pub ord: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct terminate_message {
    pub layer_etype: u8,
    pub ecode: u8,
    pub hdrct_rsvd: __be16,
    pub len_hdrs: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c4iw_layers_types {
    LAYER_RDMAP		= 0x00,
    LAYER_DDP		= 0x10,
    LAYER_MPA		= 0x20,
    RDMAP_LOCAL_CATA	= 0x00,
    RDMAP_REMOTE_PROT	= 0x01,
    RDMAP_REMOTE_OP		= 0x02,
    DDP_LOCAL_CATA		= 0x00,
    DDP_TAGGED_ERR		= 0x01,
    DDP_UNTAGGED_ERR	= 0x02,
    DDP_LLP			= 0x03
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c4iw_rdma_ecodes {
    RDMAP_INV_STAG		= 0x00,
    RDMAP_BASE_BOUNDS	= 0x01,
    RDMAP_ACC_VIOL		= 0x02,
    RDMAP_STAG_NOT_ASSOC	= 0x03,
    RDMAP_TO_WRAP		= 0x04,
    RDMAP_INV_VERS		= 0x05,
    RDMAP_INV_OPCODE	= 0x06,
    RDMAP_STREAM_CATA	= 0x07,
    RDMAP_GLOBAL_CATA	= 0x08,
    RDMAP_CANT_INV_STAG	= 0x09,
    RDMAP_UNSPECIFIED	= 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c4iw_ddp_ecodes {
    DDPT_INV_STAG		= 0x00,
    DDPT_BASE_BOUNDS	= 0x01,
    DDPT_STAG_NOT_ASSOC	= 0x02,
    DDPT_TO_WRAP		= 0x03,
    DDPT_INV_VERS		= 0x04,
    DDPU_INV_QN		= 0x01,
    DDPU_INV_MSN_NOBUF	= 0x02,
    DDPU_INV_MSN_RANGE	= 0x03,
    DDPU_INV_MO		= 0x04,
    DDPU_MSG_TOOBIG		= 0x05,
    DDPU_INV_VERS		= 0x06
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c4iw_mpa_ecodes {
    MPA_CRC_ERR		= 0x02,
    MPA_MARKER_ERR          = 0x03,
    MPA_LOCAL_CATA          = 0x05,
    MPA_INSUFF_IRD          = 0x06,
    MPA_NOMATCH_RTR         = 0x07,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c4iw_ep_state {
    IDLE = 0,
    LISTEN,
    CONNECTING,
    MPA_REQ_WAIT,
    MPA_REQ_SENT,
    MPA_REQ_RCVD,
    MPA_REP_SENT,
    FPDU_MODE,
    ABORTING,
    CLOSING,
    MORIBUND,
    DEAD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c4iw_ep_flags {
    PEER_ABORT_IN_PROGRESS	= 0,
    ABORT_REQ_IN_PROGRESS	= 1,
    RELEASE_RESOURCES	= 2,
    CLOSE_SENT		= 3,
    TIMEOUT                 = 4,
    QP_REFERENCED           = 5,
    STOP_MPA_TIMER		= 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c4iw_ep_history {
    ACT_OPEN_REQ            = 0,
    ACT_OFLD_CONN           = 1,
    ACT_OPEN_RPL            = 2,
    ACT_ESTAB               = 3,
    PASS_ACCEPT_REQ         = 4,
    PASS_ESTAB              = 5,
    ABORT_UPCALL            = 6,
    ESTAB_UPCALL            = 7,
    CLOSE_UPCALL            = 8,
    ULP_ACCEPT              = 9,
    ULP_REJECT              = 10,
    TIMEDOUT                = 11,
    PEER_ABORT              = 12,
    PEER_CLOSE              = 13,
    CONNREQ_UPCALL          = 14,
    ABORT_CONN              = 15,
    DISCONN_UPCALL          = 16,
    EP_DISC_CLOSE           = 17,
    EP_DISC_ABORT           = 18,
    CONN_RPL_UPCALL         = 19,
    ACT_RETRY_NOMEM         = 20,
    ACT_RETRY_INUSE         = 21,
    CLOSE_CON_RPL		= 22,
    EP_DISC_FAIL		= 24,
    QP_REFED		= 25,
    QP_DEREFED		= 26,
    CM_ID_REFED		= 27,
    CM_ID_DEREFED		= 28,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum conn_pre_alloc_buffers {
    CN_ABORT_REQ_BUF,
    CN_ABORT_RPL_BUF,
    CN_CLOSE_CON_REQ_BUF,
    CN_DESTROY_BUF,
    CN_FLOWC_BUF,
    CN_MAX_CON_BUF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cpl_wr_size {
    pub abrt_req: cpl_abort_req,
    pub abrt_rpl: cpl_abort_rpl,
    pub ri_req: fw_ri_wr,
    pub close_req: cpl_close_con_req,
    pub flowc_buf: [c_char; FLOWC_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_ep_common {
    pub cm_id: *mut iw_cm_id,
    pub qp: *mut c4iw_qp,
    pub dev: *mut c4iw_dev,
    pub ep_skb_list: sk_buff_head,
    pub state: c4iw_ep_state,
    pub kref: kref,
    pub mutex: mutex,
    pub local_addr: sockaddr_storage,
    pub remote_addr: sockaddr_storage,
    pub wr_waitp: *mut c4iw_wr_wait,
    pub flags: c_ulong,
    pub history: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_listen_ep {
    pub com: c4iw_ep_common,
    pub stid: c_uint,
    pub backlog: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_ep_stats {
    pub connect_neg_adv: unsigned,
    pub abort_neg_adv: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_ep {
    pub com: c4iw_ep_common,
    pub parent_ep: *mut c4iw_ep,
    pub timer: timer_list,
    pub entry: list_head,
    pub atid: c_uint,
    pub hwtid: u32,
    pub snd_seq: u32,
    pub rcv_seq: u32,
    pub l2t: *mut l2t_entry,
    pub dst: *mut dst_entry,
    pub mpa_skb: *mut sk_buff,
    pub mpa_attr: c4iw_mpa_attributes,
    pub MPA_MAX_PRIVATE_DATA]: u8 mpa_pkt[sizeof(struct mpa_message) +,
    pub mpa_pkt_len: c_uint,
    pub ird: u32,
    pub ord: u32,
    pub smac_idx: u32,
    pub tx_chan: u32,
    pub mtu: u32,
    pub mss: u16,
    pub emss: u16,
    pub plen: u16,
    pub rss_qid: u16,
    pub txq_idx: u16,
    pub ctrlq_idx: u16,
    pub tos: u8,
    pub retry_with_mpa_v1: u8,
    pub tried_with_mpa_v1: u8,
    pub retry_count: c_uint,
    pub snd_win: c_int,
    pub rcv_win: c_int,
    pub snd_wscale: u32,
    pub stats: c4iw_ep_stats,
    pub srqe_idx: u32,
    pub rx_pdu_out_cnt: u32,
    pub peer_abort_skb: *mut sk_buff,
}

extern "C" {
    pub fn c4iw_id_alloc(alloc: *mut c4iw_id_table) -> u32;
}
extern "C" {
    pub fn c4iw_id_free(alloc: *mut c4iw_id_table, obj: u32);
}
extern "C" {
    pub fn c4iw_id_table_free(alloc: *mut c4iw_id_table);
}
extern "C" {
    pub fn int(dev: *mut *mut c4iw_handler_func)(struct c4iw_dev, skb: *mut sk_buff) -> typedef;
}
extern "C" {
    pub fn c4iw_get_resource(id_table: *mut c4iw_id_table) -> u32;
}
extern "C" {
    pub fn c4iw_put_resource(id_table: *mut c4iw_id_table, entry: u32);
}
extern "C" {
    pub fn c4iw_pblpool_create(rdev: *mut c4iw_rdev) -> c_int;
}
extern "C" {
    pub fn c4iw_rqtpool_create(rdev: *mut c4iw_rdev) -> c_int;
}
extern "C" {
    pub fn c4iw_ocqp_pool_create(rdev: *mut c4iw_rdev) -> c_int;
}
extern "C" {
    pub fn c4iw_pblpool_destroy(rdev: *mut c4iw_rdev);
}
extern "C" {
    pub fn c4iw_rqtpool_destroy(rdev: *mut c4iw_rdev);
}
extern "C" {
    pub fn c4iw_ocqp_pool_destroy(rdev: *mut c4iw_rdev);
}
extern "C" {
    pub fn c4iw_destroy_resource(rscp: *mut c4iw_resource);
}
extern "C" {
    pub fn c4iw_register_device(work: *mut work_struct);
}
extern "C" {
    pub fn c4iw_unregister_device(dev: *mut c4iw_dev);
}
extern "C" {
    pub fn c4iw_cm_init() -> int __init;
}
extern "C" {
    pub fn c4iw_cm_term();
}
extern "C" {
    pub fn c4iw_poll_cq(ibcq: *mut ib_cq, num_entries: c_int, wc: *mut ib_wc) -> c_int;
}
extern "C" {
    pub fn c4iw_connect(cm_id: *mut iw_cm_id, conn_param: *mut iw_cm_conn_param) -> c_int;
}
extern "C" {
    pub fn c4iw_create_listen(cm_id: *mut iw_cm_id, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn c4iw_destroy_listen(cm_id: *mut iw_cm_id) -> c_int;
}
extern "C" {
    pub fn c4iw_accept_cr(cm_id: *mut iw_cm_id, conn_param: *mut iw_cm_conn_param) -> c_int;
}
extern "C" {
    pub fn c4iw_reject_cr(cm_id: *mut iw_cm_id, pdata: *const c_void, pdata_len: u8) -> c_int;
}
extern "C" {
    pub fn c4iw_qp_add_ref(qp: *mut ib_qp);
}
extern "C" {
    pub fn c4iw_qp_rem_ref(qp: *mut ib_qp);
}
extern "C" {
    pub fn c4iw_dealloc(ctx: *mut uld_ctx);
}
extern "C" {
    pub fn c4iw_dereg_mr(ib_mr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn c4iw_destroy_cq(ib_cq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn c4iw_cq_rem_ref(chp: *mut c4iw_cq);
}
extern "C" {
    pub fn c4iw_arm_cq(ibcq: *mut ib_cq, flags: ib_cq_notify_flags) -> c_int;
}
extern "C" {
    pub fn c4iw_destroy_srq(ib_srq: *mut ib_srq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn c4iw_destroy_qp(ib_qp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn c4iw_rqtpool_alloc(rdev: *mut c4iw_rdev, size: c_int) -> u32;
}
extern "C" {
    pub fn c4iw_rqtpool_free(rdev: *mut c4iw_rdev, addr: u32, size: c_int);
}
extern "C" {
    pub fn c4iw_pblpool_alloc(rdev: *mut c4iw_rdev, size: c_int) -> u32;
}
extern "C" {
    pub fn c4iw_pblpool_free(rdev: *mut c4iw_rdev, addr: u32, size: c_int);
}
extern "C" {
    pub fn c4iw_ocqp_pool_alloc(rdev: *mut c4iw_rdev, size: c_int) -> u32;
}
extern "C" {
    pub fn c4iw_ocqp_pool_free(rdev: *mut c4iw_rdev, addr: u32, size: c_int);
}
extern "C" {
    pub fn c4iw_flush_hw_cq(chp: *mut c4iw_cq, flush_qhp: *mut c4iw_qp);
}
extern "C" {
    pub fn c4iw_count_rcqes(cq: *mut t4_cq, wq: *mut t4_wq, count: *mut c_int);
}
extern "C" {
    pub fn c4iw_ep_disconnect(ep: *mut c4iw_ep, abrupt: c_int, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn c4iw_flush_rq(wq: *mut t4_wq, cq: *mut t4_cq, count: c_int) -> c_int;
}
extern "C" {
    pub fn c4iw_flush_sq(qhp: *mut c4iw_qp) -> c_int;
}
extern "C" {
    pub fn c4iw_ev_handler(rnicp: *mut c4iw_dev, qid: u32) -> c_int;
}
extern "C" {
    pub fn c4iw_get_cqid(rdev: *mut c4iw_rdev, uctx: *mut c4iw_dev_ucontext) -> u32;
}
extern "C" {
    pub fn c4iw_get_qpid(rdev: *mut c4iw_rdev, uctx: *mut c4iw_dev_ucontext) -> u32;
}
extern "C" {
    pub fn c4iw_ev_dispatch(dev: *mut c4iw_dev, err_cqe: *mut t4_cqe);
}
extern "C" {
    pub fn c4iw_alloc_srq_idx(rdev: *mut c4iw_rdev) -> c_int;
}
extern "C" {
    pub fn c4iw_free_srq_idx(rdev: *mut c4iw_rdev, idx: c_int);
}
extern "C" {
    pub fn c4iw_log_wr_stats(wq: *mut t4_wq, cqe: *mut t4_cqe);
}
extern "C" {
    pub fn c4iw_invalidate_mr(rhp: *mut c4iw_dev, rkey: u32);
}
extern "C" {
    pub fn c4iw_dispatch_srq_limit_reached_event(srq: *mut c4iw_srq);
}
extern "C" {
    pub fn c4iw_copy_wr_to_srq(srq: *mut t4_srq, wqe: *mut t4_recv_wr, len16: u8);
}
extern "C" {
    pub fn c4iw_flush_srqidx(qhp: *mut c4iw_qp, srqidx: u32);
}
extern "C" {
    pub fn c4iw_fill_res_mr_entry(msg: *mut sk_buff, ibmr: *mut ib_mr) -> c_int;
}
extern "C" {
    pub fn c4iw_fill_res_cq_entry(msg: *mut sk_buff, ibcq: *mut ib_cq) -> c_int;
}
extern "C" {
    pub fn c4iw_fill_res_qp_entry(msg: *mut sk_buff, ibqp: *mut ib_qp) -> c_int;
}
extern "C" {
    pub fn c4iw_fill_res_cm_id_entry(msg: *mut sk_buff, cm_id: *mut rdma_cm_id) -> c_int;
}
