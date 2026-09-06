//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_mgmt.h
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
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_recv_msg {
// Preallocated buffer of size MAX_PF_MGMT_BUF_SIZE that accumulates
// receive message, segment-by-segment.
//
    pub msg: *mut c_void,
// Message id for which segments are accumulated.
    pub msg_id: u8,
// Sequence id of last received segment of current message.
    pub seq_id: u8,
    pub msg_len: u16,
    pub async_mgmt_to_pf: c_int,
    pub mod: mgmt_mod_type,
    pub cmd: u16,
    pub recv_done: completion,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum comm_pf_to_mgmt_event_state {
    COMM_SEND_EVENT_UNINIT,
    COMM_SEND_EVENT_START,
    COMM_SEND_EVENT_SUCCESS,
    COMM_SEND_EVENT_TIMEOUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_msg_pf_to_mgmt {
    pub hwdev: *mut hinic3_hwdev,
    pub workq: *mut workqueue_struct,
    pub mgmt_ack_buf: *mut c_void,
    pub recv_msg_from_mgmt: hinic3_recv_msg,
    pub recv_resp_msg_from_mgmt: hinic3_recv_msg,
    pub async_msg_id: u16,
    pub sync_msg_id: u16,
    pub async_msg_cb_data: [*mut c_void; MGMT_MOD_HW_MAX],
// synchronizes message send with message receives via event queue
    pub sync_event_lock: spinlock_t,
    pub event_flag: comm_pf_to_mgmt_event_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_msg_handle_work {
    pub work: work_struct,
    pub pf_to_mgmt: *mut hinic3_msg_pf_to_mgmt,
    pub msg: *mut c_void,
    pub msg_len: u16,
    pub mod: mgmt_mod_type,
    pub cmd: u16,
    pub msg_id: u16,
    pub async_mgmt_to_pf: c_int,
}

extern "C" {
    pub fn hinic3_pf_to_mgmt_init(hwdev: *mut hinic3_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic3_pf_to_mgmt_free(hwdev: *mut hinic3_hwdev);
}
extern "C" {
    pub fn hinic3_flush_mgmt_workq(hwdev: *mut hinic3_hwdev);
}
