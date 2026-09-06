//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/siw/siw_cm.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Authors: Bernard Metzler <bmt@zurich.ibm.com>
// Greg Joyce <greg@opengridcomputing.com>
// Copyright (c) 2008-2019, IBM Corporation
// Copyright (c) 2017, Open Grid Computing, Inc.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_cep_state {
    SIW_EPSTATE_IDLE = 1,
    SIW_EPSTATE_LISTENING,
    SIW_EPSTATE_CONNECTING,
    SIW_EPSTATE_AWAIT_MPAREQ,
    SIW_EPSTATE_RECVD_MPAREQ,
    SIW_EPSTATE_AWAIT_MPAREP,
    SIW_EPSTATE_RDMA_MODE,
    SIW_EPSTATE_CLOSED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_mpa_info {
    pub /: *mut *mut mpa_rr hdr; / peer mpa hdr in host byte order,
    pub v2_ctrl: mpa_v2_data,
    pub v2_ctrl_req: mpa_v2_data,
    pub pdata: *mut c_char,
    pub bytes_rcvd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_cep {
    pub cm_id: *mut iw_cm_id,
    pub sdev: *mut siw_device,
    pub devq: list_head,
    pub lock: spinlock_t,
    pub ref: kref,
    pub in_use: c_int,
    pub waitq: wait_queue_head_t,
    pub state: siw_cep_state,
    pub listenq: list_head,
    pub listen_cep: *mut siw_cep,
    pub qp: *mut siw_qp,
    pub sock: *mut socket,
    pub mpa_timer: *mut siw_cm_work,
    pub work_freelist: list_head,
    pub mpa: siw_mpa_info,
    pub ord: c_int,
    pub ird: c_int,
    pub enhanced_rdma_conn_est: bool,
// Saved upcalls of socket
    pub sk): *mut *mut void (sk_state_change)(struct sock,
    pub sk): *mut *mut void (sk_data_ready)(struct sock,
    pub sk): *mut *mut void (sk_write_space)(struct sock,
    pub sk): *mut *mut void (sk_error_report)(struct sock,
}

//
// Connection initiator waits 10 seconds to receive an
// MPA reply after sending out MPA request. Reponder waits for
// 5 seconds for MPA request to arrive if new TCP connection
// was set up.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_work_type {
    SIW_CM_WORK_ACCEPT = 1,
    SIW_CM_WORK_READ_MPAHDR,
    SIW_CM_WORK_CLOSE_LLP, /* close socket */
    SIW_CM_WORK_PEER_CLOSE, /* socket indicated peer close */
    SIW_CM_WORK_MPATIMEOUT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_cm_work {
    pub work: delayed_work,
    pub list: list_head,
    pub type: siw_work_type,
    pub cep: *mut siw_cep,
}

extern "C" {
    pub fn kernel_recvmsg(_arg: sock, _arg: &msg, _arg: &iov, _arg: 1, _arg: size, _arg: flags) -> return;
}
extern "C" {
    pub fn siw_connect(id: *mut iw_cm_id, parm: *mut iw_cm_conn_param) -> c_int;
}
extern "C" {
    pub fn siw_accept(id: *mut iw_cm_id, param: *mut iw_cm_conn_param) -> c_int;
}
extern "C" {
    pub fn siw_reject(id: *mut iw_cm_id, data: *const c_void, len: u8) -> c_int;
}
extern "C" {
    pub fn siw_create_listen(id: *mut iw_cm_id, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn siw_destroy_listen(id: *mut iw_cm_id) -> c_int;
}
extern "C" {
    pub fn siw_cep_get(cep: *mut siw_cep);
}
extern "C" {
    pub fn siw_cep_put(cep: *mut siw_cep);
}
extern "C" {
    pub fn siw_cm_queue_work(cep: *mut siw_cep, type: siw_work_type) -> c_int;
}
extern "C" {
    pub fn siw_cm_init() -> c_int;
}
extern "C" {
    pub fn siw_cm_exit();
}
//
// TCP socket interface
//

