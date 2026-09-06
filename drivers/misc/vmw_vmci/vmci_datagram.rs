//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/vmw_vmci/vmci_datagram.h
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
// VMware VMCI Driver
//
// Copyright (C) 2012 VMware, Inc. All rights reserved.
//

pub const VMCI_MAX_DELAYED_DG_HOST_QUEUE_SIZE: c_int = 256;
//
// The struct vmci_datagram_queue_entry is a queue header for the in-kernel VMCI
// datagram queues. It is allocated in non-paged memory, as the
// content is accessed while holding a spinlock. The pending datagram
// itself may be allocated from paged memory. We shadow the size of
// the datagram in the non-paged queue entry as this size is used
// while holding the same spinlock as above.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_datagram_queue_entry {
    pub /: *mut *mut list_head list_item; / For queuing.,
    pub /: *mut *mut size_t dg_size; / Size of datagram.,
    pub /: *mut *mut *mut vmci_datagram dg; / Pending datagram.,
}

// VMCIDatagramSendRecvInfo
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_datagram_snd_rcv_info {
    pub addr: u64,
    pub len: u32,
    pub result: i32,
}

// Datagram API for non-public use.
extern "C" {
    pub fn vmci_datagram_invoke_guest_handler(dg: *mut vmci_datagram) -> c_int;
}
