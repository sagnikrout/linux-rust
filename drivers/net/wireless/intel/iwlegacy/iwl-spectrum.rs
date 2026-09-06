//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlegacy/iwl-spectrum.h
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
// Copyright(c) 2003 - 2011 Intel Corporation. All rights reserved.
//
// Portions of this file are derived from the ieee80211 subsystem header files.
//
// Contact Information:
// Intel Linux Wireless <ilw@linux.intel.com>
// Intel Corporation, 5200 N.E. Elam Young Parkway, Hillsboro, OR 97124-6497
//

// Macro flag: #define __il_spectrum_h__
// Bits 5-7 are reserved
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_basic_report {
    pub channel: u8,
    pub start_time: __le64,
    pub duration: __le16,
    pub map: u8,
    pub __packed: },
// Bit 0 is reserved
// Bits 4-7 are reserved
}

// 3-255 reserved
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_measurement_params {
    pub channel: u8,
    pub start_time: __le64,
    pub duration: __le16,
    pub __packed: },
