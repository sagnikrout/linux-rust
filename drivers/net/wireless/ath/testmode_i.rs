//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/testmode_i.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// "API" level of the ath testmode interface. Bump it after every
// incompatible interface change.
//
pub const ATH_TESTMODE_VERSION_MAJOR: c_int = 1;
// Bump this after every _compatible_ interface change, for example
// addition of a new command or an attribute.
//
pub const ATH_TESTMODE_VERSION_MINOR: c_int = 1;
pub const ATH_TM_DATA_MAX_LEN: c_int = 5000;
pub const ATH_FTM_EVENT_MAX_BUF_LENGTH: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_tm_attr {
    __ATH_TM_ATTR_INVALID		= 0,
    ATH_TM_ATTR_CMD			= 1,
    ATH_TM_ATTR_DATA		= 2,
    ATH_TM_ATTR_WMI_CMDID		= 3,
    ATH_TM_ATTR_VERSION_MAJOR	= 4,
    ATH_TM_ATTR_VERSION_MINOR	= 5,
    ATH_TM_ATTR_WMI_OP_VERSION	= 6,

// keep last
    __ATH_TM_ATTR_AFTER_LAST,
    ATH_TM_ATTR_MAX			= __ATH_TM_ATTR_AFTER_LAST - 1,
}

// All ath testmode interface commands specified in
// ATH_TM_ATTR_CMD
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_tm_cmd {
// Returns the supported ath testmode interface version in
// ATH_TM_ATTR_VERSION. Always guaranteed to work. User space
// uses this to verify it's using the correct version of the
// testmode interface
//
    ATH_TM_CMD_GET_VERSION = 0,

// The command used to transmit a WMI command to the firmware and
// the event to receive WMI events from the firmware. Without
// struct wmi_cmd_hdr header, only the WMI payload. Command id is
// provided with ATH_TM_ATTR_WMI_CMDID and payload in
// ATH_TM_ATTR_DATA.
//
    ATH_TM_CMD_WMI = 1,

// Boots the UTF firmware, the netdev interface must be down at the
// time.
//
    ATH_TM_CMD_TESTMODE_START = 2,

// The command used to transmit a FTM WMI command to the firmware
// and the event to receive WMI events from the firmware. The data
// received only contain the payload, need to add the tlv header
// and send the cmd to firmware with command id WMI_PDEV_UTF_CMDID.
// The data payload size could be large and the driver needs to
// send segmented data to firmware.
//
    ATH_TM_CMD_WMI_FTM = 3,
}
