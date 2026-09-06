//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/meta/fbnic/fbnic_fw_log.h
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
// Copyright (c) Meta Platforms, Inc. and affiliates.

// A 512K log buffer was chosen fairly arbitrarily

// Firmware log output is prepended with log index followed by a timestamp.
// The timestamp is similar to Zephyr's format DD:HH:MM:SS.MMM
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_fw_log_entry {
    pub list: list_head,
    pub index: u64,
    pub timestamp: u32,
    pub len: u16,
    pub __counted_by(len): char msg[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_fw_log {
    pub data_start: *mut c_void,
    pub data_end: *mut c_void,
    pub size: usize,
    pub entries: list_head,
// Spin lock for accessing or modifying entries
    pub lock: spinlock_t,
}

extern "C" {
    pub fn fbnic_fw_log_enable(fbd: *mut fbnic_dev, send_hist: bool);
}
extern "C" {
    pub fn fbnic_fw_log_disable(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_fw_log_init(fbd: *mut fbnic_dev) -> c_int;
}
extern "C" {
    pub fn fbnic_fw_log_free(fbd: *mut fbnic_dev);
}
