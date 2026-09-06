//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/vendor_cmd.h
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
// This file is part of wlcore
//
// Copyright (C) 2014 Texas Instruments. All rights reserved.
//

extern "C" {
    pub fn wlcore_set_vendor_commands(wiphy: *mut wiphy);
}

pub const TI_OUI: c_uint = 0x080028;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlcore_vendor_commands {
    WLCORE_VENDOR_CMD_SMART_CONFIG_START,
    WLCORE_VENDOR_CMD_SMART_CONFIG_STOP,
    WLCORE_VENDOR_CMD_SMART_CONFIG_SET_GROUP_KEY,

    NUM_WLCORE_VENDOR_CMD,
    MAX_WLCORE_VENDOR_CMD = NUM_WLCORE_VENDOR_CMD - 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlcore_vendor_attributes {
    WLCORE_VENDOR_ATTR_FREQ,
    WLCORE_VENDOR_ATTR_PSK,
    WLCORE_VENDOR_ATTR_SSID,
    WLCORE_VENDOR_ATTR_GROUP_ID,
    WLCORE_VENDOR_ATTR_GROUP_KEY,

    NUM_WLCORE_VENDOR_ATTR,
    MAX_WLCORE_VENDOR_ATTR = NUM_WLCORE_VENDOR_ATTR - 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlcore_vendor_events {
    WLCORE_VENDOR_EVENT_SC_SYNC,
    WLCORE_VENDOR_EVENT_SC_DECODE,
}
