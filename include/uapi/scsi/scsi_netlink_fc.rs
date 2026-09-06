//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/scsi/scsi_netlink_fc.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// FC Transport Netlink Interface
//
// Copyright (C) 2006   James Smart, Emulex Corporation
//

//
// This file intended to be included by both kernel and user space
//
// FC Transport Message Types
//
// kernel -> user
pub const FC_NL_ASYNC_EVENT: c_uint = 0x0100;
// user -> kernel
// none
//
// Message Structures :
//
// macro to round up message lengths to 8byte boundary

//
// FC Transport Broadcast Event Message :
// FC_NL_ASYNC_EVENT
//
// Note: if Vendor Unique message, event_data_flex will be start of
// vendor unique payload, and the length of the payload is
// per event_datalen
//
// Note: When specifying vendor_id, be sure to read the Vendor Type and ID
// formatting requirements specified in scsi_netlink.h
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_nl_event {
    pub /: *mut *mut scsi_nl_hdr snlh; / must be 1st element !,
    pub seconds: __u64,
    pub vendor_id: __u64,
    pub host_no: __u16,
    pub event_datalen: __u16,
    pub event_num: __u32,
    pub event_code: __u32,
    pub event_data: __u32,
    pub event_data_flex): __DECLARE_FLEX_ARRAY(__u8,,
}
