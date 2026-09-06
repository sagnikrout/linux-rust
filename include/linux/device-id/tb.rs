//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/tb.h
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

pub type kernel_ulong_t = c_ulong;

pub const TBSVC_MATCH_PROTOCOL_KEY: c_uint = 0x0001;
pub const TBSVC_MATCH_PROTOCOL_ID: c_uint = 0x0002;
pub const TBSVC_MATCH_PROTOCOL_VERSION: c_uint = 0x0004;
pub const TBSVC_MATCH_PROTOCOL_REVISION: c_uint = 0x0008;
//
// struct tb_service_id - Thunderbolt service identifiers
// @match_flags: Flags used to match the structure
// @protocol_key: Protocol key the service supports
// @protocol_id: Protocol id the service supports
// @protocol_version: Version of the protocol
// @protocol_revision: Revision of the protocol software
// @driver_data: Driver specific data
//
// Thunderbolt XDomain services are exposed as devices where each device
// carries the protocol information the service supports. Thunderbolt
// XDomain service drivers match against that information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_service_id {
    pub match_flags: __u32,
    pub 1]: char protocol_key[8 +,
    pub protocol_id: __u32,
    pub protocol_version: __u32,
    pub protocol_revision: __u32,
    pub driver_data: kernel_ulong_t,
}
