//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/apple/rtkit-internal.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple RTKit IPC library
// Copyright (C) The Asahi Linux Contributors
//

pub const APPLE_RTKIT_APP_ENDPOINT_START: c_uint = 0x20;
pub const APPLE_RTKIT_MAX_ENDPOINTS: c_uint = 0x100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_rtkit {
    pub cookie: *mut c_void,
    pub ops: *const apple_rtkit_ops,
    pub dev: *mut device,
    pub mbox: *mut apple_mbox,
    pub epmap_completion: completion,
    pub iop_pwr_ack_completion: completion,
    pub ap_pwr_ack_completion: completion,
    pub boot_result: c_int,
    pub version: c_int,
    pub iop_power_state: c_uint,
    pub ap_power_state: c_uint,
    pub crashed: bool,
    pub APPLE_RTKIT_MAX_ENDPOINTS): DECLARE_BITMAP(endpoints,,
    pub ioreport_buffer: apple_rtkit_shmem,
    pub crashlog_buffer: apple_rtkit_shmem,
    pub oslog_buffer: apple_rtkit_shmem,
    pub syslog_buffer: apple_rtkit_shmem,
    pub syslog_msg_buffer: *mut c_char,
    pub syslog_n_entries: usize,
    pub syslog_msg_size: usize,
    pub wq: *mut workqueue_struct,
}

extern "C" {
    pub fn apple_rtkit_crashlog_dump(rtk: *mut apple_rtkit, bfr: *mut u8, size: usize);
}
