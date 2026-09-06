//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sysctrl_mailbox_types.h
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
// Copyright © 2026 Intel Corporation
//

//
// enum xe_sysctrl_group - System Controller command groups
//
// @XE_SYSCTRL_GROUP_GFSP: GFSP group
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_sysctrl_group {
    XE_SYSCTRL_GROUP_GFSP			= 0x01,
}

//
// enum xe_sysctrl_gfsp_cmd - Commands supported by GFSP group
//
// @XE_SYSCTRL_CMD_GET_SOC_ERROR: Retrieve basic error information
// @XE_SYSCTRL_CMD_GET_COUNTER: Get error counter value
// @XE_SYSCTRL_CMD_CLEAR_COUNTER: Clear error counter value
// @XE_SYSCTRL_CMD_GET_PENDING_EVENT: Retrieve pending event
// @XE_SYSCTRL_CMD_GET_HEALTH: Retrieve gpu health
// @XE_SYSCTRL_CMD_SET_HEALTH: Set gpu health
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_sysctrl_gfsp_cmd {
    XE_SYSCTRL_CMD_GET_SOC_ERROR		= 0x01,
    XE_SYSCTRL_CMD_GET_COUNTER		= 0x03,
    XE_SYSCTRL_CMD_CLEAR_COUNTER		= 0x04,
    XE_SYSCTRL_CMD_GET_PENDING_EVENT	= 0x07,
    XE_SYSCTRL_CMD_GET_HEALTH		= 0x0B,
    XE_SYSCTRL_CMD_SET_HEALTH		= 0x0C,
}

//
// struct xe_sysctrl_mailbox_command - System Controller mailbox command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sysctrl_mailbox_command {
// @header: Application message header containing command information
    pub header: xe_sysctrl_app_msg_hdr,
// @data_in: Pointer to input payload data (can be NULL if no input data)
    pub data_in: *mut c_void,
// @data_in_len: Size of input payload in bytes (0 if no input data)
    pub data_in_len: usize,
// @data_out: Pointer to output buffer for response data (can be NULL if no response)
    pub data_out: *mut c_void,
// @data_out_len: Size of output buffer in bytes (0 if no response expected)
    pub data_out_len: usize,
}

// Modify as needed
pub const XE_SYSCTRL_FLOOD_LIMIT: c_int = 16;
pub const XE_SYSCTRL_MB_FRAME_SIZE: c_int = 16;
pub const XE_SYSCTRL_MB_MAX_FRAMES: c_int = 64;

pub const XE_SYSCTRL_MB_DEFAULT_TIMEOUT_MS: c_int = 500;
