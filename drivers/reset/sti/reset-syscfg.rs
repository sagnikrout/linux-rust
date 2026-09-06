//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/reset/sti/reset-syscfg.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2013 STMicroelectronics (R&D) Limited
// Author: Stephen Gallimore <stephen.gallimore@st.com>
//

//
// Reset channel description for a system configuration register based
// reset controller.
//
// @compatible: Compatible string of the syscon regmap containing this
// channel's control and ack (status) bits.
// @reset: Regmap field description of the channel's reset bit.
// @ack: Regmap field description of the channel's acknowledge bit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscfg_reset_channel_data {
    pub compatible: *const c_char,
    pub reset: reg_field,
    pub ack: reg_field,
}

//
// Description of a system configuration register based reset controller.
//
// @wait_for_ack: The controller will wait for reset assert and de-assert to
// be "ack'd" in a channel's ack field.
// @active_low: Are the resets in this controller active low, i.e. clearing
// the reset bit puts the hardware into reset.
// @nr_channels: The number of reset channels in this controller.
// @channels: An array of reset channel descriptions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscfg_reset_controller_data {
    pub wait_for_ack: bool,
    pub active_low: bool,
    pub nr_channels: c_int,
    pub channels: *const syscfg_reset_channel_data,
}

//
// syscfg_reset_probe(): platform device probe function used by syscfg
// reset controller drivers. This registers a reset
// controller configured by the OF match data for
// the compatible device which should be of type
// "struct syscfg_reset_controller_data".
//
// @pdev: platform device
//
extern "C" {
    pub fn syscfg_reset_probe(pdev: *mut platform_device) -> c_int;
}
