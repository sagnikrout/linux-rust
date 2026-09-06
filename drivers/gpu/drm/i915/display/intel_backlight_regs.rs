//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_backlight_regs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

// Backlight control

//
// This is the most significant 15 bits of the number of backlight cycles in a
// complete cycle of the modulated backlight control.
//
// The actual value is this field multiplied by two.
//

//
// This is the number of cycles out of the backlight modulation cycle for which
// the backlight is on.
//
// This field must be no greater than the number of cycles in the complete
// backlight modulation cycle.
//

// New registers for PCH-split platforms. Safe where new bits show up, the
// register layout machtes with gen4 BLC_PWM_CTL[12].

// PCH CTL1 is totally different, all but the below bits are reserved. CTL2 is
// like the normal CTL from gen4 and earlier. Hooray for confusing naming.

// BXT backlight register definition.
pub const _BXT_BLC_PWM_CTL1: c_uint = 0xC8250;

pub const _BXT_BLC_PWM_FREQ1: c_uint = 0xC8254;
pub const _BXT_BLC_PWM_DUTY1: c_uint = 0xC8258;
pub const _BXT_BLC_PWM_CTL2: c_uint = 0xC8350;
pub const _BXT_BLC_PWM_FREQ2: c_uint = 0xC8354;
pub const _BXT_BLC_PWM_DUTY2: c_uint = 0xC8358;

// Utility pin

