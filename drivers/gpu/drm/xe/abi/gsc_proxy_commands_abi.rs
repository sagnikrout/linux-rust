//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/gsc_proxy_commands_abi.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

// Heci client ID for proxy commands
pub const HECI_MEADDRESS_PROXY: c_int = 10;
// FW-defined proxy header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gsc_proxy_header {
//
// hdr:
// Bits 0-7: type of the proxy message (see enum xe_gsc_proxy_type)
// Bits 8-15: rsvd
// Bits 16-31: length in bytes of the payload following the proxy header
//
    pub hdr: u32,

    pub /: *mut *mut u32 source; / Source of the Proxy message,
    pub /: *mut *mut u32 destination; / Destination of the Proxy message,
pub const GSC_PROXY_ADDRESSING_KMD: c_uint = 0x10000;
pub const GSC_PROXY_ADDRESSING_GSC: c_uint = 0x20000;
pub const GSC_PROXY_ADDRESSING_CSME: c_uint = 0x30000;
    pub /: *mut *mut u32 status; / Command status,
    pub __packed: },
// FW-defined proxy types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_gsc_proxy_type {
    GSC_PROXY_MSG_TYPE_PROXY_INVALID = 0,
    GSC_PROXY_MSG_TYPE_PROXY_QUERY = 1,
    GSC_PROXY_MSG_TYPE_PROXY_PAYLOAD = 2,
    GSC_PROXY_MSG_TYPE_PROXY_END = 3,
    GSC_PROXY_MSG_TYPE_PROXY_NOTIFICATION = 4,
}
