//! Automatically rewritten from C Header to Rust Module
//! Source: net/vmw_vsock/vmci_transport_notify.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// VMware vSockets Driver
//
// Copyright (C) 2009-2013 VMware, Inc. All rights reserved.
//

// Comment this out to compare with old protocol.
pub const VSOCK_OPTIMIZATION_WAITING_NOTIFY: c_int = 1;

// Comment this out to remove flow control for "new" protocol
pub const VSOCK_OPTIMIZATION_FLOW_CONTROL: c_int = 1;

pub const VMCI_TRANSPORT_MAX_DGRAM_RESENDS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_transport_recv_notify_data {
    pub consume_head: u64,
    pub produce_tail: u64,
    pub notify_on_block: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_transport_send_notify_data {
    pub consume_head: u64,
    pub produce_tail: u64,
}

// Socket notification callbacks.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_transport_notify_ops {
    pub sk): *mut *mut void (socket_init) (struct sock,
    pub vsk): *mut *mut void (socket_destruct) (struct vsock_sock,
    pub data_ready_now): *mut bool,
    pub space_avail_now): *mut bool,
    pub pkt_processed): *mut bool,
    pub data): *mut vmci_transport_recv_notify_data,
    pub data): *mut vmci_transport_recv_notify_data,
    pub data): *mut vmci_transport_recv_notify_data,
    pub data): *mut vmci_transport_recv_notify_data,
    pub data): *mut vmci_transport_send_notify_data,
    pub data): *mut vmci_transport_send_notify_data,
    pub data): *mut vmci_transport_send_notify_data,
    pub data): *mut vmci_transport_send_notify_data,
    pub sk): *mut *mut void (process_request) (struct sock,
    pub sk): *mut *mut void (process_negotiate) (struct sock,
}
