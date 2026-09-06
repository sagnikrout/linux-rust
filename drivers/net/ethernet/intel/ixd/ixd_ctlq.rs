//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixd/ixd_ctlq.h
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
// Copyright (C) 2025 Intel Corporation

pub const IXD_CTLQ_TIMEOUT: c_int = 2000;
//
// struct ixd_ctlq_req - Standard virtchnl request description
// @opcode: protocol opcode, only virtchnl2 is needed for now
// @send_size: required length of the send buffer
// @send_buff_init: function to initialize the allocated send buffer
// @recv_process: function to handle the CP response
// @ctx: additional context for callbacks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixd_ctlq_req {
    pub opcode: virtchnl2_op,
    pub send_size: usize,
    pub ctx): *mut c_void,
    pub ctx): *mut size_t recv_size, void,
    pub ctx: *mut c_void,
}

extern "C" {
    pub fn ixd_ctlq_clean_sq(adapter: *mut ixd_adapter, force: bool);
}
extern "C" {
    pub fn ixd_ctlq_rx_task(work: *mut work_struct);
}
