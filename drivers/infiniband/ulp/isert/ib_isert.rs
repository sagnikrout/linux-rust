//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/ulp/isert/ib_isert.h
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

// Constant PDU lengths calculations

// QP settings
// Maximal bounds on received asynchronous PDUs

// NOOP_OUT(2), TEXT(1),
// SCSI_TMFUNC(2), LOGOUT(1)
//

//
// RX size is default of 8k plus headers, but data needs to align to
// 512 boundary, so use 1024 to have the extra space for alignment.
//

// Minimum I/O size is 512KB
pub const ISCSI_ISER_MIN_SG_TABLESIZE: c_int = 128;
// Maximum support is 16MB I/O size
pub const ISCSI_ISER_MAX_SG_TABLESIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isert_desc_type {
    ISCSI_TX_CONTROL,
    ISCSI_TX_DATAIN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iser_conn_state {
    ISER_CONN_INIT,
    ISER_CONN_UP,
    ISER_CONN_BOUND,
    ISER_CONN_FULL_FEATURE,
    ISER_CONN_TERMINATING,
    ISER_CONN_DOWN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_rx_desc {
    pub buf: [c_char; ISER_RX_SIZE],
    pub dma_addr: u64,
    pub rx_sg: ib_sge,
    pub rx_cqe: ib_cqe,
    pub in_use: bool,
}

extern "C" {
    pub fn container_of(_arg: cqe, iser_rx_desc: struct, _arg: rx_cqe) -> return;
}
extern "C" {
    pub fn isert_get_iser_hdr(iser_ctrl: desc) + sizeof(struct) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_tx_desc {
    pub iser_header: iser_ctrl,
    pub iscsi_header: iscsi_hdr,
    pub type: isert_desc_type,
    pub dma_addr: u64,
    pub tx_sg: [ib_sge; 2],
    pub tx_cqe: ib_cqe,
    pub num_sge: c_int,
    pub send_wr: ib_send_wr,
    pub __packed: },
    pub tx_cqe): return container_of(cqe, struct iser_tx_desc,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isert_cmd {
    pub read_stag: u32,
    pub write_stag: u32,
    pub read_va: u64,
    pub write_va: u64,
    pub inv_rkey: u32,
    pub pdu_buf_dma: u64,
    pub pdu_buf_len: u32,
    pub conn: *mut isert_conn,
    pub iscsit_cmd: *mut iscsit_cmd,
    pub tx_desc: iser_tx_desc,
    pub rx_desc: *mut iser_rx_desc,
    pub rw: rdma_rw_ctx,
    pub comp_work: work_struct,
    pub sg: scatterlist,
    pub ctx_init_done: bool,
}

extern "C" {
    pub fn container_of(_arg: desc, isert_cmd: struct, _arg: tx_desc) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isert_conn {
    pub state: iser_conn_state,
    pub responder_resources: u32,
    pub initiator_depth: u32,
    pub pi_support: bool,
    pub login_desc: *mut iser_rx_desc,
    pub login_rsp_buf: *mut c_char,
    pub login_req_len: c_int,
    pub login_rsp_dma: u64,
    pub rx_descs: *mut iser_rx_desc,
    pub rx_wr: [ib_recv_wr; ISERT_QP_MAX_RECV_DTOS],
    pub conn: *mut iscsit_conn,
    pub node: list_head,
    pub login_comp: completion,
    pub login_req_comp: completion,
    pub login_tx_desc: iser_tx_desc,
    pub login_rsp_pending: bool,
    pub cm_id: *mut rdma_cm_id,
    pub qp: *mut ib_qp,
    pub cq: *mut ib_cq,
    pub cq_size: u32,
    pub device: *mut isert_device,
    pub mutex: mutex,
    pub kref: kref,
    pub release_work: work_struct,
    pub logout_posted: bool,
    pub snd_w_inv: bool,
    pub rem_wait: wait_queue_head_t,
    pub dev_removed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isert_device {
    pub pi_capable: bool,
    pub refcount: c_int,
    pub ib_device: *mut ib_device,
    pub pd: *mut ib_pd,
    pub comps: *mut isert_comp,
    pub comps_used: c_int,
    pub dev_node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isert_np {
    pub np: *mut iscsi_np,
    pub sem: semaphore,
    pub cm_id: *mut rdma_cm_id,
    pub mutex: mutex,
    pub accepted: list_head,
    pub pending: list_head,
}
