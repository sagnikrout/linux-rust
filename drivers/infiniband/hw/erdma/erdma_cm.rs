//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/erdma/erdma_cm.h
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
// Authors: Cheng Xu <chengyou@linux.alibaba.com>
// Kai Shen <kaishen@linux.alibaba.com>
// Copyright (c) 2020-2022, Alibaba Group.
// Authors: Bernard Metzler <bmt@zurich.ibm.com>
// Greg Joyce <greg@opengridcomputing.com>
// Copyright (c) 2008-2019, IBM Corporation
// Copyright (c) 2017, Open Grid Computing, Inc.

// iWarp MPA protocol defs
pub const MPA_REVISION_EXT_1: c_int = 129;

pub const MPA_KEY_SIZE: c_int = 16;
pub const MPA_DEFAULT_HDR_LEN: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpa_rr_params {
    pub bits: __be16,
    pub pd_len: __be16,
}

//
// MPA request/response Hdr bits & fields
//
// MPA request/reply header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpa_rr {
    pub key: [u8; 16],
    pub params: mpa_rr_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_mpa_ext {
    pub cookie: __be32,
    pub bits: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_mpa_info {
    pub /: *mut *mut mpa_rr hdr; / peer mpa hdr in host byte order,
    pub ext_data: erdma_mpa_ext,
    pub pdata: *mut c_char,
    pub bytes_rcvd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_sk_upcalls {
    pub sk): *mut *mut void (sk_state_change)(struct sock,
    pub bytes): *mut *mut *mut void (sk_data_ready)(struct sock sk, int,
    pub sk): *mut *mut void (sk_error_report)(struct sock,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_cep_state {
    ERDMA_EPSTATE_IDLE = 1,
    ERDMA_EPSTATE_LISTENING,
    ERDMA_EPSTATE_CONNECTING,
    ERDMA_EPSTATE_AWAIT_MPAREQ,
    ERDMA_EPSTATE_RECVD_MPAREQ,
    ERDMA_EPSTATE_AWAIT_MPAREP,
    ERDMA_EPSTATE_RDMA_MODE,
    ERDMA_EPSTATE_CLOSED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cep {
    pub cm_id: *mut iw_cm_id,
    pub dev: *mut erdma_dev,
    pub devq: list_head,
    pub lock: spinlock_t,
    pub ref: kref,
    pub in_use: c_int,
    pub waitq: wait_queue_head_t,
    pub state: erdma_cep_state,
    pub listenq: list_head,
    pub listen_cep: *mut erdma_cep,
    pub qp: *mut erdma_qp,
    pub sock: *mut socket,
    pub mpa_timer: *mut erdma_cm_work,
    pub work_freelist: list_head,
    pub mpa: erdma_mpa_info,
    pub ord: c_int,
    pub ird: c_int,
    pub pd_len: c_int,
// hold user's private data.
    pub private_data: *mut c_void,
// Saved upcalls of socket llp.sock
    pub sk): *mut *mut void (sk_state_change)(struct sock,
    pub sk): *mut *mut void (sk_data_ready)(struct sock,
    pub sk): *mut *mut void (sk_error_report)(struct sock,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_work_type {
    ERDMA_CM_WORK_ACCEPT = 1,
    ERDMA_CM_WORK_READ_MPAHDR,
    ERDMA_CM_WORK_CLOSE_LLP, /* close socket */
    ERDMA_CM_WORK_PEER_CLOSE, /* socket indicated peer close */
    ERDMA_CM_WORK_MPATIMEOUT,
    ERDMA_CM_WORK_CONNECTED,
    ERDMA_CM_WORK_CONNECTTIMEOUT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cm_work {
    pub work: delayed_work,
    pub list: list_head,
    pub type: erdma_work_type,
    pub cep: *mut erdma_cep,
}

extern "C" {
    pub fn erdma_connect(id: *mut iw_cm_id, param: *mut iw_cm_conn_param) -> c_int;
}
extern "C" {
    pub fn erdma_accept(id: *mut iw_cm_id, param: *mut iw_cm_conn_param) -> c_int;
}
extern "C" {
    pub fn erdma_reject(id: *mut iw_cm_id, pdata: *const c_void, plen: u8) -> c_int;
}
extern "C" {
    pub fn erdma_create_listen(id: *mut iw_cm_id, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn erdma_destroy_listen(id: *mut iw_cm_id) -> c_int;
}
extern "C" {
    pub fn erdma_cep_get(ceq: *mut erdma_cep);
}
extern "C" {
    pub fn erdma_cep_put(ceq: *mut erdma_cep);
}
extern "C" {
    pub fn erdma_cm_queue_work(ceq: *mut erdma_cep, type: erdma_work_type) -> c_int;
}
extern "C" {
    pub fn erdma_cm_init() -> c_int;
}
extern "C" {
    pub fn erdma_cm_exit();
}

