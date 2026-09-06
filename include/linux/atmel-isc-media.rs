//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/atmel-isc-media.h
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
// Copyright (c) 2019 Microchip Technology Inc. and its subsidiaries
//
// Author: Eugen Hristev <eugen.hristev@microchip.com>
//
// There are 8 controls available:
// 4 gain controls, sliders, for each of the BAYER components: R, B, GR, GB.
// These gains are multipliers for each component, in format unsigned 0:4:9 with
// a default value of 512 (1.0 multiplier).
// 4 offset controls, sliders, for each of the BAYER components: R, B, GR, GB.
// These offsets are added/substracted from each component, in format signed
// 1:12:0 with a default value of 0 (+/- 0)
//
// To expose this to userspace, added 8 custom controls, in an auto cluster.
//
// To summarize the functionality:
// The auto cluster switch is the auto white balance control, and it works
// like this:
// AWB == 1: autowhitebalance is on, the do_white_balance button is inactive,
// the gains/offsets are inactive, but volatile and readable.
// Thus, the results of the whitebalance algorithm are available to userspace to
// read at any time.
// AWB == 0: autowhitebalance is off, cluster is in manual mode, user can
// configure the gain/offsets directly.
// More than that, if the do_white_balance button is
// pressed, the driver will perform one-time-adjustment, (preferably with color
// checker card) and the userspace can read again the new values.
//
// With this feature, the userspace can save the coefficients and reinstall them
// for example after reboot or reprobing the driver.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atmel_isc_ctrl_id {
// Red component gain control
    ISC_CID_R_GAIN = (V4L2_CID_USER_ATMEL_ISC_BASE + 0),
// Blue component gain control
    ISC_CID_B_GAIN,
// Green Red component gain control
    ISC_CID_GR_GAIN,
// Green Blue gain control
    ISC_CID_GB_GAIN,
// Red component offset control
    ISC_CID_R_OFFSET,
// Blue component offset control
    ISC_CID_B_OFFSET,
// Green Red component offset control
    ISC_CID_GR_OFFSET,
// Green Blue component offset control
    ISC_CID_GB_OFFSET,
}
