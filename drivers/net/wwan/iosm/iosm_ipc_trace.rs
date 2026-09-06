//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_trace.h
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
// Copyright (C) 2020-2021 Intel Corporation.
//

//
// enum trace_ctrl_mode - State of trace channel
// @TRACE_DISABLE:	mode for disable trace
// @TRACE_ENABLE:	mode for enable trace
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trace_ctrl_mode {
    TRACE_DISABLE = 0,
    TRACE_ENABLE,
}

//
// struct iosm_trace - Struct for trace interface
// @ipc_rchan:		Pointer to relay channel
// @ctrl_file:		Pointer to trace control file
// @ipc_imem:		Imem instance
// @dev:		Pointer to device struct
// @channel:		Channel instance
// @chl_id:		Channel Identifier
// @trc_mutex:		Mutex used for read and write mode
// @mode:		Mode for enable and disable trace
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_trace {
    pub ipc_rchan: *mut rchan,
    pub ctrl_file: *mut dentry,
    pub ipc_imem: *mut iosm_imem,
    pub dev: *mut device,
    pub channel: *mut ipc_mem_channel,
    pub chl_id: ipc_channel_id,
    pub /: *mut *mut mutex trc_mutex; / Mutex used for read and write mode,
    pub mode: trace_ctrl_mode,
}

extern "C" {
    pub fn ipc_trace_deinit(ipc_trace: *mut iosm_trace);
}
extern "C" {
    pub fn ipc_trace_port_rx(ipc_imem: *mut iosm_imem, skb: *mut sk_buff);
}

