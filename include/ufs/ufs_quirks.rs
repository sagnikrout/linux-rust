//! Automatically rewritten from C Header to Rust Module
//! Source: include/ufs/ufs_quirks.h
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
// Copyright (c) 2014-2016, The Linux Foundation. All rights reserved.
//
// return true if s1 is a prefix of s2

pub const UFS_ANY_VENDOR: c_uint = 0xFFFF;

pub const UFS_VENDOR_MICRON: c_uint = 0x12C;
pub const UFS_VENDOR_SAMSUNG: c_uint = 0x1CE;
pub const UFS_VENDOR_SKHYNIX: c_uint = 0x1AD;
pub const UFS_VENDOR_TOSHIBA: c_uint = 0x198;
pub const UFS_VENDOR_WDC: c_uint = 0x145;
//
// ufs_dev_quirk - ufs device quirk info
// @card: ufs card details
// @quirk: device quirk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_dev_quirk {
    pub wmanufacturerid: u16,
    pub model: *const u8,
    pub quirk: c_uint,
}

//
// Some vendor's UFS device sends back to back NACs for the DL data frames
// causing the host controller to raise the DFES error status. Sometimes
// such UFS devices send back to back NAC without waiting for new
// retransmitted DL frame from the host and in such cases it might be possible
// the Host UniPro goes into bad state without raising the DFES error
// interrupt. If this happens then all the pending commands would timeout
// only after respective SW command (which is generally too large).
//
// We can workaround such device behaviour like this:
// - As soon as SW sees the DL NAC error, it should schedule the error handler
// - Error handler would sleep for 50ms to see if there are any fatal errors
// raised by UFS controller.
// - If there are fatal errors then SW does normal error recovery.
// - If there are no fatal errors then SW sends the NOP command to device
// to check if link is alive.
// - If NOP command times out, SW does normal error recovery
// - If NOP command succeed, skip the error handling.
//
// If DL NAC error is seen multiple times with some vendor's UFS devices then
// enable this quirk to initiate quick error recovery and also silence related
// error logs to reduce spamming of kernel logs.
//

//
// Few Toshiba UFS device models advertise RX_MIN_ACTIVATETIME_CAPABILITY as
// 600us which may not be enough for reliable hibern8 exit hardware sequence
// from UFS device.
// To workaround this issue, host should set its PA_TACTIVATE time to 1ms even
// if device advertises RX_MIN_ACTIVATETIME_CAPABILITY less than 1ms.
//

//
// It seems some UFS devices may keep drawing more than sleep current
// (atleast for 500us) from UFS rails (especially from VCCQ rail).
// To avoid this situation, add 2ms delay before putting these UFS
// rails in LPM mode.
//

//
// Some UFS devices require host PA_TACTIVATE to be lower than device
// PA_TACTIVATE, enabling this quirk ensure this.
//

//
// The max. value PA_SaveConfigTime is 250 (10us) but this is not enough for
// some vendors.
// Gear switch from PWM to HS may fail even with this max. PA_SaveConfigTime.
// Gear switch can be issued by host controller as an error recovery and any
// software delay will not help on this case so we need to increase
// PA_SaveConfigTime to >32us as per vendor recommendation.
//

//
// Some UFS devices require VS_DebugSaveConfigTime is 0x10,
// enabling this quirk ensure this.
//

//
// Some pre-3.1 UFS devices can support extended features by upgrading
// the firmware. Enable this quirk to make UFS core driver probe and enable
// supported features on such devices.
//

//
// Some ufs devices may need more time to be in hibern8 before exiting.
// Enable this quirk to give it an additional 100us.
//

// Some UFS 4 devices do not support the qTimestamp attribute

