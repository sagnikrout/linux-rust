//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/greybus/arpc.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Copyright(c) 2016 Google Inc. All rights reserved.
// Copyright(c) 2016 Linaro Ltd. All rights reserved.
//
// APBridgeA RPC (ARPC)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arpc_result {
    ARPC_SUCCESS		= 0x00,
    ARPC_NO_MEMORY		= 0x01,
    ARPC_INVALID		= 0x02,
    ARPC_TIMEOUT		= 0x03,
    ARPC_UNKNOWN_ERROR	= 0xff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpc_request_message {
    pub /: *mut *mut __le16 id; / RPC unique id,
    pub /: *mut *mut __le16 size; / Size in bytes of header + payload,
    pub /: *mut *mut __u8 type; / RPC type,
    pub /: *mut *mut __u8 data[]; / ARPC data,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpc_response_message {
    pub /: *mut *mut __le16 id; / RPC unique id,
    pub /: *mut *mut __u8 result; / Result of RPC,
    pub __packed: },
// ARPC requests
pub const ARPC_TYPE_CPORT_CONNECTED: c_uint = 0x01;
pub const ARPC_TYPE_CPORT_QUIESCE: c_uint = 0x02;
pub const ARPC_TYPE_CPORT_CLEAR: c_uint = 0x03;
pub const ARPC_TYPE_CPORT_FLUSH: c_uint = 0x04;
pub const ARPC_TYPE_CPORT_SHUTDOWN: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpc_cport_connected_req {
    pub cport_id: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpc_cport_quiesce_req {
    pub cport_id: __le16,
    pub peer_space: __le16,
    pub timeout: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpc_cport_clear_req {
    pub cport_id: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpc_cport_flush_req {
    pub cport_id: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpc_cport_shutdown_req {
    pub cport_id: __le16,
    pub timeout: __le16,
    pub phase: __u8,
    pub __packed: },
