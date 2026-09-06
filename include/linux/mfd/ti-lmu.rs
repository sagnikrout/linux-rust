//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/ti-lmu.h
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
// TI LMU (Lighting Management Unit) Devices
//
// Copyright 2017 Texas Instruments
//
// Author: Milo Kim <milo.kim@ti.com>
//

// Notifier event
pub const LMU_EVENT_MONITOR_DONE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ti_lmu_id {
    LM3631,
    LM3632,
    LM3633,
    LM3695,
    LM36274,
    LMU_MAX_ID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ti_lmu_max_current {
    LMU_IMAX_5mA,
    LMU_IMAX_6mA,
    LMU_IMAX_7mA = 0x03,
    LMU_IMAX_8mA,
    LMU_IMAX_9mA,
    LMU_IMAX_10mA = 0x07,
    LMU_IMAX_11mA,
    LMU_IMAX_12mA,
    LMU_IMAX_13mA,
    LMU_IMAX_14mA,
    LMU_IMAX_15mA = 0x0D,
    LMU_IMAX_16mA,
    LMU_IMAX_17mA,
    LMU_IMAX_18mA,
    LMU_IMAX_19mA,
    LMU_IMAX_20mA = 0x13,
    LMU_IMAX_21mA,
    LMU_IMAX_22mA,
    LMU_IMAX_23mA = 0x17,
    LMU_IMAX_24mA,
    LMU_IMAX_25mA,
    LMU_IMAX_26mA,
    LMU_IMAX_27mA = 0x1C,
    LMU_IMAX_28mA,
    LMU_IMAX_29mA,
    LMU_IMAX_30mA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm363x_regulator_id {
    LM3631_BOOST,		/* Boost output */
    LM3631_LDO_CONT,	/* Display panel controller */
    LM3631_LDO_OREF,	/* Gamma reference */
    LM3631_LDO_POS,		/* Positive display bias output */
    LM3631_LDO_NEG,		/* Negative display bias output */
    LM3632_BOOST,		/* Boost output */
    LM3632_LDO_POS,		/* Positive display bias output */
    LM3632_LDO_NEG,		/* Negative display bias output */
    LM36274_BOOST,		/* Boost output */
    LM36274_LDO_POS,	/* Positive display bias output */
    LM36274_LDO_NEG,	/* Negative display bias output */
}

//
// struct ti_lmu
//
// @dev:	Parent device pointer
// @regmap:	Used for i2c communcation on accessing registers
// @en_gpio:	GPIO for HWEN pin [Optional]
// @notifier:	Notifier for reporting hwmon event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_lmu {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub en_gpio: *mut gpio_desc,
    pub notifier: blocking_notifier_head,
}
