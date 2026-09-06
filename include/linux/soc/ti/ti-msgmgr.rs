//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/ti/ti-msgmgr.h
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
// Texas Instruments' Message Manager
//
// Copyright (C) 2015-2022 Texas Instruments Incorporated - https://www.ti.com
// Nishanth Menon
//
// struct ti_msgmgr_message - Message Manager structure
// @len: Length of data in the Buffer
// @buf: Buffer pointer
// @chan_rx: Expected channel for response, must be provided to use polled rx
// @timeout_rx_ms: Timeout value to use if polling for response
//
// This is the structure for data used in mbox_send_message
// the length of data buffer used depends on the SoC integration
// parameters - each message may be 64, 128 bytes long depending
// on SoC. Client is supposed to be aware of this.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_msgmgr_message {
    pub len: usize,
    pub buf: *mut u8,
    pub chan_rx: *mut mbox_chan,
    pub timeout_rx_ms: c_int,
}
