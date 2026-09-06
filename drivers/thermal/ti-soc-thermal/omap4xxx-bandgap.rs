//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/ti-soc-thermal/omap4xxx-bandgap.h
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
// OMAP4xxx bandgap registers, bitfields and temperature definitions
//
// Copyright (C) 2013 Texas Instruments Incorporated - http://www.ti.com
// Contact:
// Eduardo Valentin <eduardo.valentin@ti.com>
//
// *** OMAP4430
//
// Below, in sequence, are the Register definitions,
// the bitfields and the temperature definitions for OMAP4430.
//
// OMAP4430 register definitions
//
// Registers are defined as offsets. The offsets are
// relative to FUSE_OPP_BGAP on 4430.
//
// OMAP4430.FUSE_OPP_BGAP
pub const OMAP4430_FUSE_OPP_BGAP: c_uint = 0x0;
// OMAP4430.TEMP_SENSOR
pub const OMAP4430_TEMP_SENSOR_CTRL_OFFSET: c_uint = 0xCC;
//
// Register and bit definitions for OMAP4430
//
// All the macros below define the required bits for
// controlling temperature on OMAP4430. Bit defines are
// grouped by register.
//
// OMAP4430.TEMP_SENSOR bits

//
// Temperature limits and thresholds for OMAP4430
//
// All the macros below are definitions for handling the
// ADC conversions and representation of temperature limits
// and thresholds for OMAP4430.
//
// ADC conversion table limits. Ignore values outside the TRM listed
// range to avoid bogus thermal shutdowns. See omap4430 TRM chapter
// "18.4.10.2.3 ADC Codes Versus Temperature".
//
pub const OMAP4430_ADC_START_VALUE: c_int = 13;
pub const OMAP4430_ADC_END_VALUE: c_int = 107;
// bandgap clock limits (no control on 4430)
pub const OMAP4430_MAX_FREQ: c_int = 32768;
pub const OMAP4430_MIN_FREQ: c_int = 32768;
//
// *** OMAP4460 *** Applicable for OMAP4470
//
// Below, in sequence, are the Register definitions,
// the bitfields and the temperature definitions for OMAP4460.
//
// OMAP4460 register definitions
//
// Registers are defined as offsets. The offsets are
// relative to FUSE_OPP_BGAP on 4460.
//
// OMAP4460.FUSE_OPP_BGAP
pub const OMAP4460_FUSE_OPP_BGAP: c_uint = 0x0;
// OMAP4460.TEMP_SENSOR
pub const OMAP4460_TEMP_SENSOR_CTRL_OFFSET: c_uint = 0xCC;
// OMAP4460.BANDGAP_CTRL
pub const OMAP4460_BGAP_CTRL_OFFSET: c_uint = 0x118;
// OMAP4460.BANDGAP_COUNTER
pub const OMAP4460_BGAP_COUNTER_OFFSET: c_uint = 0x11C;
// OMAP4460.BANDGAP_THRESHOLD
pub const OMAP4460_BGAP_THRESHOLD_OFFSET: c_uint = 0x120;
// OMAP4460.TSHUT_THRESHOLD
pub const OMAP4460_BGAP_TSHUT_OFFSET: c_uint = 0x124;
// OMAP4460.BANDGAP_STATUS
pub const OMAP4460_BGAP_STATUS_OFFSET: c_uint = 0x128;
//
// Register bitfields for OMAP4460
//
// All the macros below define the required bits for
// controlling temperature on OMAP4460. Bit defines are
// grouped by register.
//
// OMAP4460.TEMP_SENSOR bits

// OMAP4460.BANDGAP_CTRL bits

// OMAP4460.BANDGAP_COUNTER bits

// OMAP4460.BANDGAP_THRESHOLD bits

// OMAP4460.TSHUT_THRESHOLD bits

// OMAP4460.BANDGAP_STATUS bits

//
// Temperature limits and thresholds for OMAP4460
//
// All the macros below are definitions for handling the
// ADC conversions and representation of temperature limits
// and thresholds for OMAP4460.
//
// ADC conversion table limits
pub const OMAP4460_ADC_START_VALUE: c_int = 530;
pub const OMAP4460_ADC_END_VALUE: c_int = 932;
// bandgap clock limits
pub const OMAP4460_MAX_FREQ: c_int = 1500000;
pub const OMAP4460_MIN_FREQ: c_int = 1000000;
// interrupts thresholds

