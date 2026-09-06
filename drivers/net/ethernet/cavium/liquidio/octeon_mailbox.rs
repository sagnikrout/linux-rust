//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/octeon_mailbox.h
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


//
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more details.
//
// Macros for Mail Box Communication
pub const OCTEON_MBOX_DATA_MAX: c_int = 32;
pub const OCTEON_VF_ACTIVE: c_uint = 0x1;
pub const OCTEON_VF_FLR_REQUEST: c_uint = 0x2;
pub const OCTEON_PF_CHANGED_VF_MACADDR: c_uint = 0x4;
pub const OCTEON_GET_VF_STATS: c_uint = 0x8;
// Macro for Read acknowldgement
pub const OCTEON_PFVFACK: c_uint = 0xffffffffffffffffULL;
pub const OCTEON_PFVFSIG: c_uint = 0x1122334455667788ULL;
pub const OCTEON_PFVFERR: c_uint = 0xDEADDEADDEADDEADULL;
pub const LIO_MBOX_WRITE_WAIT_CNT: c_int = 1000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octeon_mbox_cmd_status {
    OCTEON_MBOX_STATUS_SUCCESS = 0,
    OCTEON_MBOX_STATUS_FAILED = 1,
    OCTEON_MBOX_STATUS_BUSY = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octeon_mbox_message_type {
    OCTEON_MBOX_REQUEST = 0,
    OCTEON_MBOX_RESPONSE = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union octeon_mbox_message {
    pub u64: u64,
    pub 1: u16 type :,
    pub 1: u16 resp_needed :,
    pub 6: u16 cmd :,
    pub 8: u16 len :,
    pub params: [u8; 6],
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_mbox_cmd {
    pub msg: octeon_mbox_message,
    pub data: [u64; OCTEON_MBOX_DATA_MAX],
    pub q_no: u32,
    pub recv_len: u32,
    pub recv_status: u32,
    pub fn: octeon_mbox_callback_t,
    pub fn_arg: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octeon_mbox_state {
    OCTEON_MBOX_STATE_IDLE = 1,
    OCTEON_MBOX_STATE_REQUEST_RECEIVING = 2,
    OCTEON_MBOX_STATE_REQUEST_RECEIVED = 4,
    OCTEON_MBOX_STATE_RESPONSE_PENDING = 8,
    OCTEON_MBOX_STATE_RESPONSE_RECEIVING = 16,
    OCTEON_MBOX_STATE_RESPONSE_RECEIVED = 32,
    OCTEON_MBOX_STATE_ERROR = 64
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_mbox {
// A spinlock to protect access to this q_mbox.
    pub lock: spinlock_t,
    pub oct_dev: *mut octeon_device,
    pub q_no: u32,
    pub state: octeon_mbox_state,
    pub mbox_poll_wk: cavium_wk,
// SLI_MAC_PF_MBOX_INT for PF, SLI_PKT_MBOX_INT for VF.
    pub mbox_int_reg: *mut c_void,
// SLI_PKT_PF_VF_MBOX_SIG(0) for PF, SLI_PKT_PF_VF_MBOX_SIG(1) for VF.
//
    pub mbox_write_reg: *mut c_void,
// SLI_PKT_PF_VF_MBOX_SIG(1) for PF, SLI_PKT_PF_VF_MBOX_SIG(0) for VF.
//
    pub mbox_read_reg: *mut c_void,
    pub mbox_req: octeon_mbox_cmd,
    pub mbox_resp: octeon_mbox_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_vf_stats_ctx {
    pub status: core::sync::atomic::AtomicI32,
    pub stats: *mut oct_vf_stats,
}

extern "C" {
    pub fn octeon_mbox_read(mbox: *mut octeon_mbox) -> c_int;
}
extern "C" {
    pub fn octeon_mbox_process_message(mbox: *mut octeon_mbox) -> c_int;
}
extern "C" {
    pub fn octeon_mbox_cancel(oct: *mut octeon_device, q_no: c_int) -> c_int;
}
