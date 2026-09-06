//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/cxgb4_dcb.h
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
// Copyright (C) 2013-2014 Chelsio Communications.  All rights reserved.
//
// Written by Anish Bhatt (anish@chelsio.com)
//

// States we can be in for a port's Data Center Bridging.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_dcb_state {
    CXGB4_DCB_STATE_START,		/* initial unknown state */
    CXGB4_DCB_STATE_HOST,		/* we're using Host DCB (if at all) */
    CXGB4_DCB_STATE_FW_INCOMPLETE,	/* using firmware DCB, incomplete */
    CXGB4_DCB_STATE_FW_ALLSYNCED,	/* using firmware DCB, all sync'ed */
}

// Data Center Bridging state input for the Finite State Machine.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_dcb_state_input {
// Input from the firmware.
//
    CXGB4_DCB_INPUT_FW_DISABLED,	/* firmware DCB disabled */
    CXGB4_DCB_INPUT_FW_ENABLED,	/* firmware DCB enabled */
    CXGB4_DCB_INPUT_FW_INCOMPLETE,	/* firmware reports incomplete DCB */
    CXGB4_DCB_INPUT_FW_ALLSYNCED,	/* firmware reports all sync'ed */

}

// Firmware DCB messages that we've received so far ...
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_dcb_fw_msgs {
    CXGB4_DCB_FW_PGID	= 0x01,
    CXGB4_DCB_FW_PGRATE	= 0x02,
    CXGB4_DCB_FW_PRIORATE	= 0x04,
    CXGB4_DCB_FW_PFC	= 0x08,
    CXGB4_DCB_FW_APP_ID	= 0x10,
}

pub const CXGB4_MAX_DCBX_APP_SUPPORTED: c_int = 8;
// Data Center Bridging support;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_dcb_info {
    pub /: *mut *mut cxgb4_dcb_state state; / DCB State Machine,
    pub /: *mut *mut cxgb4_dcb_fw_msgs msgs; / DCB Firmware messages received,
    pub /: *mut *mut unsigned int supported; / OS DCB capabilities supported,
    pub /: *mut *mut bool enabled; / OS Enabled state,
// Cached copies of DCB information sent by the firmware (in Host
// Native Endian format).
//
    pub /: *mut *mut u32 pgid; / Priority Group[0..7],
    pub /: *mut *mut u8 dcb_version; / Running DCBx version,
    pub /: *mut *mut u8 pfcen; / Priority Flow Control[0..7],
    pub /: *mut *mut u8 pg_num_tcs_supported; / max PG Traffic Classes,
    pub /: *mut *mut u8 pfc_num_tcs_supported; / max PFC Traffic Classes,
    pub /: *mut *mut u8 pgrate[8]; / Priority Group Rate[0..7],
    pub /: *mut *mut u8 priorate[8]; / Priority Rate[0..7],
    pub /: *mut *mut u8 tsa[8]; / TSA Algorithm[0..7],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_priority {
    pub /: *mut *mut u8 user_prio_map; / Priority Map bitfield,
    pub /: *mut *mut u8 sel_field; / Protocol ID interpretation,
    pub /: *mut *mut u16 protocolid; / Protocol ID,
    pub app_priority: [}; CXGB4_MAX_DCBX_APP_SUPPORTED],
}

extern "C" {
    pub fn cxgb4_dcb_state_init(: *mut net_device);
}
extern "C" {
    pub fn cxgb4_dcb_version_init(: *mut net_device);
}
extern "C" {
    pub fn cxgb4_dcb_reset(dev: *mut net_device);
}
extern "C" {
    pub fn cxgb4_dcb_state_fsm(: *mut net_device, cxgb4_dcb_state_input: enum);
}
extern "C" {
    pub fn cxgb4_dcb_handle_fw_update(: *mut adapter, : *const fw_port_cmd);
}

