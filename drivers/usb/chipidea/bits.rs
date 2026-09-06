//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/chipidea/bits.h
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


// SPDX-License-Identifier: GPL-2.0
//
// bits.h - register bits of the ChipIdea USB IP core
//
// Copyright (C) 2008 Chipidea - MIPS Technologies, Inc. All rights reserved.
//
// Author: David Lopo
//

//
// ID
// For 1.x revision, bit24 - bit31 are reserved
// For 2.x revision, bit25 - bit28 are 0x2
//

// SBUSCFG
pub const AHBBRST_MASK: c_uint = 0x7;
// HCCPARAMS

// DCCPARAMS

// TESTMODE

// USBCMD

// USBSTS & USBINTR

// DEVICEADDR

// TTCTRL

// Set non-zero value for internal TT Hub address representation

// BURSTSIZE
pub const RX_BURST_MASK: c_uint = 0xff;
pub const TX_BURST_MASK: c_uint = 0xff00;
// PORTSC

// PTS and PTW for non lpm version only

// DEVLC

// Encoding for DEVLC_PTS and PORTSC_PTS
pub const PTS_UTMI: c_int = 0;
pub const PTS_ULPI: c_int = 2;
pub const PTS_SERIAL: c_int = 3;
pub const PTS_HSIC: c_int = 4;
// OTGSC

// USBMODE

// ENDPTCTRL

