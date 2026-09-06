//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/scmi_imx_protocol.h
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
// SCMI Message Protocol driver NXP extension header
//
// Copyright 2024 NXP.
//

pub const SCMI_PROTOCOL_IMX_LMM: c_uint = 0x80;
pub const SCMI_PROTOCOL_IMX_BBM: c_uint = 0x81;
pub const SCMI_PROTOCOL_IMX_CPU: c_uint = 0x82;
pub const SCMI_PROTOCOL_IMX_MISC: c_uint = 0x84;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_imx_bbm_proto_ops {
    pub sec): u64,
    pub val): *mut u64,
    pub sec): bool enable, u64,
    pub state): *const *const *const int (button_get)(struct scmi_protocol_handle ph, u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_nxp_notification_events {
    SCMI_EVENT_IMX_BBM_RTC = 0x0,
    SCMI_EVENT_IMX_BBM_BUTTON = 0x1,
    SCMI_EVENT_IMX_MISC_CONTROL = 0x0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_imx_bbm_notif_report {
    pub is_rtc: bool,
    pub is_button: bool,
    pub timestamp: ktime_t,
    pub rtc_id: c_uint,
    pub rtc_evt: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_imx_misc_ctrl_notify_report {
    pub timestamp: ktime_t,
    pub ctrl_id: c_uint,
    pub flags: c_uint,
}

pub const MISC_EXT_INFO_LEN_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_imx_misc_reset_reason {
    pub valid:1: bool,
    pub orig_valid:1: bool,
    pub err_valid:1: bool,
    pub reason: u32,
    pub origin: u32,
    pub errid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_imx_misc_proto_ops {
    pub val): *mut u32 num, u32,
    pub val): *mut *mut u32 num, u32,
    pub flags): u32 ctrl_id, u32 evt_id, u32,
    pub array): *mut c_void,
    pub extinfo): *mut *mut scmi_imx_misc_reset_reason shut_r, u32,
}

// See LMM_ATTRIBUTES in imx95.rst
pub const LMM_ID_DISCOVER: c_uint = 0xFFFFFFFFU;
pub const LMM_MAX_NAME: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_imx_lmm_state {
    LMM_STATE_LM_OFF,
    LMM_STATE_LM_ON,
    LMM_STATE_LM_SUSPEND,
    LMM_STATE_LM_POWERED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_imx_lmm_info {
    pub lmid: u32,
    pub state: scmi_imx_lmm_state,
    pub errstatus: u32,
    pub name: [u8; LMM_MAX_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_imx_lmm_proto_ops {
    pub boot): bool,
    pub info): *mut scmi_imx_lmm_info,
    pub vector): u32 lmid, u32 cpuid, u32 flags, u64,
    pub flags): u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_imx_cpu_proto_ops {
    pub resume): bool boot, bool,
    pub start): bool,
    pub started): *mut bool,
}
