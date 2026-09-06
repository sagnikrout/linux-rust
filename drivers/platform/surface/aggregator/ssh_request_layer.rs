//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/surface/aggregator/ssh_request_layer.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// SSH request transport layer.
//
// Copyright (C) 2019-2022 Maximilian Luz <luzmaximilian@gmail.com>
//

//
// enum ssh_rtl_state_flags - State-flags for &struct ssh_rtl.
//
// @SSH_RTL_SF_SHUTDOWN_BIT:
// Indicates that the request transport layer has been shut down or is
// being shut down and should not accept any new requests.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssh_rtl_state_flags {
    SSH_RTL_SF_SHUTDOWN_BIT,
}

//
// struct ssh_rtl_ops - Callback operations for request transport layer.
// @handle_event: Function called when a SSH event has been received. The
// specified function takes the request layer, received command
// struct, and corresponding payload as arguments. If the event
// has no payload, the payload span is empty (not %NULL).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssh_rtl_ops {
    pub data): *const ssam_span,
}

//
// struct ssh_rtl - SSH request transport layer.
// @ptl:           Underlying packet transport layer.
// @state:         State(-flags) of the transport layer.
// @queue:         Request submission queue.
// @queue.lock:    Lock for modifying the request submission queue.
// @queue.head:    List-head of the request submission queue.
// @pending:       Set/list of pending requests.
// @pending.lock:  Lock for modifying the request set.
// @pending.head:  List-head of the pending set/list.
// @pending.count: Number of currently pending requests.
// @tx:            Transmitter subsystem.
// @tx.work:       Transmitter work item.
// @rtx_timeout:   Retransmission timeout subsystem.
// @rtx_timeout.lock:    Lock for modifying the retransmission timeout reaper.
// @rtx_timeout.timeout: Timeout interval for retransmission.
// @rtx_timeout.expires: Time specifying when the reaper work is next scheduled.
// @rtx_timeout.reaper:  Work performing timeout checks and subsequent actions.
// @ops:           Request layer operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssh_rtl {
    pub ptl: ssh_ptl,
    pub state: c_ulong,
    pub lock: spinlock_t,
    pub head: list_head,
    pub queue: },
    pub lock: spinlock_t,
    pub head: list_head,
    pub count: core::sync::atomic::AtomicI32,
    pub pending: },
    pub work: work_struct,
    pub tx: },
    pub lock: spinlock_t,
    pub timeout: ktime_t,
    pub expires: ktime_t,
    pub reaper: delayed_work,
    pub rtx_timeout: },
    pub ops: ssh_rtl_ops,
}

//
// ssh_rtl_get_device() - Get device associated with request transport layer.
// @rtl: The request transport layer.
//
// Return: Returns the device on which the given request transport layer
// builds upon.
//
extern "C" {
    pub fn ssh_ptl_get_device(_arg: &rtl->ptl) -> return;
}
//
// ssh_request_rtl() - Get request transport layer associated with request.
// @rqst: The request to get the request transport layer reference for.
//
// Return: Returns the &struct ssh_rtl associated with the given SSH request.
//
extern "C" {
    pub fn ssh_rtl_submit(rtl: *mut ssh_rtl, rqst: *mut ssh_request) -> c_int;
}
extern "C" {
    pub fn ssh_rtl_cancel(rqst: *mut ssh_request, pending: bool) -> bool;
}
extern "C" {
    pub fn ssh_rtl_start(rtl: *mut ssh_rtl) -> c_int;
}
extern "C" {
    pub fn ssh_rtl_flush(rtl: *mut ssh_rtl, timeout: c_ulong) -> c_int;
}
extern "C" {
    pub fn ssh_rtl_shutdown(rtl: *mut ssh_rtl);
}
extern "C" {
    pub fn ssh_rtl_destroy(rtl: *mut ssh_rtl);
}
