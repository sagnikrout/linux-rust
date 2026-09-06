//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/iscsi_tcp.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// iSCSI Initiator TCP Transport
// Copyright (C) 2004 Dmitry Yusupov
// Copyright (C) 2004 Alex Aizman
// Copyright (C) 2005 - 2006 Mike Christie
// Copyright (C) 2006 Red Hat, Inc.  All rights reserved.
// maintained by open-iscsi@googlegroups.com
//
// See the file COPYING included with this distribution for more details.
//

// Socket connection send helper
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_sw_tcp_send {
    pub hdr: *mut iscsi_hdr,
    pub segment: iscsi_segment,
    pub data_segment: iscsi_segment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_sw_tcp_conn {
    pub sock: *mut socket,
// Taken when accessing the sock from the netlink/sysfs interface
    pub sock_lock: mutex,
    pub recvwork: work_struct,
    pub queue_recv: bool,
    pub out: iscsi_sw_tcp_send,
// old values for socket callbacks
    pub ): *mut *mut void (old_data_ready)(struct sock,
    pub ): *mut *mut void (old_state_change)(struct sock,
    pub ): *mut *mut void (old_write_space)(struct sock,
// data and header digests
    pub /: *mut *mut u32 tx_crc; / CRC32C (Tx),
    pub /: *mut *mut u32 rx_crc; / CRC32C (Rx),
// MIB custom statistics
    pub sendpage_failures_cnt: u32,
    pub discontiguous_hdr_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_sw_tcp_host {
    pub session: *mut iscsi_session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_sw_tcp_hdrbuf {
    pub hdrbuf: iscsi_hdr,
}
