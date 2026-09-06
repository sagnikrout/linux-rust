//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/testmode_i.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2014,2017 Qualcomm Atheros, Inc.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// "API" level of the ath10k testmode interface. Bump it after every
// incompatible interface change.
//
pub const ATH10K_TESTMODE_VERSION_MAJOR: c_int = 1;
// Bump this after every _compatible_ interface change, for example
// addition of a new command or an attribute.
//
pub const ATH10K_TESTMODE_VERSION_MINOR: c_int = 0;
pub const ATH10K_TM_DATA_MAX_LEN: c_int = 5000;
pub const ATH_FTM_EVENT_MAX_BUF_LENGTH: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_tm_attr {
    __ATH10K_TM_ATTR_INVALID	= 0,
    ATH10K_TM_ATTR_CMD		= 1,
    ATH10K_TM_ATTR_DATA		= 2,
    ATH10K_TM_ATTR_WMI_CMDID	= 3,
    ATH10K_TM_ATTR_VERSION_MAJOR	= 4,
    ATH10K_TM_ATTR_VERSION_MINOR	= 5,
    ATH10K_TM_ATTR_WMI_OP_VERSION	= 6,

// keep last
    __ATH10K_TM_ATTR_AFTER_LAST,
    ATH10K_TM_ATTR_MAX		= __ATH10K_TM_ATTR_AFTER_LAST - 1,
}

// All ath10k testmode interface commands specified in
// ATH10K_TM_ATTR_CMD
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_tm_cmd {
// Returns the supported ath10k testmode interface version in
// ATH10K_TM_ATTR_VERSION. Always guaranteed to work. User space
// uses this to verify it's using the correct version of the
// testmode interface
//
    ATH10K_TM_CMD_GET_VERSION = 0,

// Boots the UTF firmware, the netdev interface must be down at the
// time.
//
    ATH10K_TM_CMD_UTF_START = 1,

// Shuts down the UTF firmware and puts the driver back into OFF
// state.
//
    ATH10K_TM_CMD_UTF_STOP = 2,

// The command used to transmit a WMI command to the firmware and
// the event to receive WMI events from the firmware. Without
// struct wmi_cmd_hdr header, only the WMI payload. Command id is
// provided with ATH10K_TM_ATTR_WMI_CMDID and payload in
// ATH10K_TM_ATTR_DATA.
//
    ATH10K_TM_CMD_WMI = 3,

// The command used to transmit a test command to the firmware
// and the event to receive test events from the firmware. The data
// received only contain the TLV payload, need to add the tlv header
// and send the cmd to firmware with command id WMI_PDEV_UTF_CMDID.
// The data payload size could be large and the driver needs to
// send segmented data to firmware.
//
// This legacy testmode command shares the same value as the get-version
// command. To distinguish between them, we check whether the data attribute
// is present.
//
    ATH10K_TM_CMD_TLV = ATH10K_TM_CMD_GET_VERSION,
}
