//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/ti-soc-thermal/omap5xxx-bandgap.h
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
// OMAP5xxx bandgap registers, bitfields and temperature definitions
//
// Copyright (C) 2013 Texas Instruments Incorporated - http://www.ti.com
// Contact:
// Eduardo Valentin <eduardo.valentin@ti.com>
//
// *** OMAP5430
//
// Below, in sequence, are the Register definitions,
// the bitfields and the temperature definitions for OMAP5430.
//
// OMAP5430 register definitions
//
// Registers are defined as offsets. The offsets are
// relative to FUSE_OPP_BGAP_GPU on 5430.
//
// Register below are grouped by domain (not necessarily in offset order)
//
// OMAP5430.GPU register offsets
pub const OMAP5430_FUSE_OPP_BGAP_GPU: c_uint = 0x0;
pub const OMAP5430_TEMP_SENSOR_GPU_OFFSET: c_uint = 0x150;
pub const OMAP5430_BGAP_THRESHOLD_GPU_OFFSET: c_uint = 0x1A8;
pub const OMAP5430_BGAP_TSHUT_GPU_OFFSET: c_uint = 0x1B4;
pub const OMAP5430_BGAP_DTEMP_GPU_1_OFFSET: c_uint = 0x1F8;
pub const OMAP5430_BGAP_DTEMP_GPU_2_OFFSET: c_uint = 0x1FC;
// OMAP5430.MPU register offsets
pub const OMAP5430_FUSE_OPP_BGAP_MPU: c_uint = 0x4;
pub const OMAP5430_TEMP_SENSOR_MPU_OFFSET: c_uint = 0x14C;
pub const OMAP5430_BGAP_THRESHOLD_MPU_OFFSET: c_uint = 0x1A4;
pub const OMAP5430_BGAP_TSHUT_MPU_OFFSET: c_uint = 0x1B0;
pub const OMAP5430_BGAP_DTEMP_MPU_1_OFFSET: c_uint = 0x1E4;
pub const OMAP5430_BGAP_DTEMP_MPU_2_OFFSET: c_uint = 0x1E8;
// OMAP5430.MPU register offsets
pub const OMAP5430_FUSE_OPP_BGAP_CORE: c_uint = 0x8;
pub const OMAP5430_TEMP_SENSOR_CORE_OFFSET: c_uint = 0x154;
pub const OMAP5430_BGAP_THRESHOLD_CORE_OFFSET: c_uint = 0x1AC;
pub const OMAP5430_BGAP_TSHUT_CORE_OFFSET: c_uint = 0x1B8;
pub const OMAP5430_BGAP_DTEMP_CORE_1_OFFSET: c_uint = 0x20C;
pub const OMAP5430_BGAP_DTEMP_CORE_2_OFFSET: c_uint = 0x210;
// OMAP5430.common register offsets
pub const OMAP5430_BGAP_CTRL_OFFSET: c_uint = 0x1A0;
pub const OMAP5430_BGAP_STATUS_OFFSET: c_uint = 0x1C8;
//
// Register bitfields for OMAP5430
//
// All the macros below define the required bits for
// controlling temperature on OMAP5430. Bit defines are
// grouped by register.
//
// OMAP5430.TEMP_SENSOR

// OMAP5430.BANDGAP_CTRL

// OMAP5430.BANDGAP_COUNTER

// OMAP5430.BANDGAP_THRESHOLD

// OMAP5430.TSHUT_THRESHOLD

// OMAP5430.BANDGAP_STATUS

//
// Temperature limits and thresholds for OMAP5430
//
// All the macros below are definitions for handling the
// ADC conversions and representation of temperature limits
// and thresholds for OMAP5430. Definitions are grouped
// by temperature domain.
//
// OMAP5430.common temperature definitions
// ADC conversion table limits
pub const OMAP5430_ADC_START_VALUE: c_int = 540;
pub const OMAP5430_ADC_END_VALUE: c_int = 945;
// OMAP5430.GPU temperature definitions
// bandgap clock limits
pub const OMAP5430_GPU_MAX_FREQ: c_int = 1500000;
pub const OMAP5430_GPU_MIN_FREQ: c_int = 1000000;
// interrupts thresholds
pub const OMAP5430_GPU_TSHUT_HOT: c_int = 915;
pub const OMAP5430_GPU_TSHUT_COLD: c_int = 900;
pub const OMAP5430_GPU_T_HOT: c_int = 800;
pub const OMAP5430_GPU_T_COLD: c_int = 795;
// OMAP5430.MPU temperature definitions
// bandgap clock limits
pub const OMAP5430_MPU_MAX_FREQ: c_int = 1500000;
pub const OMAP5430_MPU_MIN_FREQ: c_int = 1000000;
// interrupts thresholds
pub const OMAP5430_MPU_TSHUT_HOT: c_int = 915;
pub const OMAP5430_MPU_TSHUT_COLD: c_int = 900;
pub const OMAP5430_MPU_T_HOT: c_int = 800;
pub const OMAP5430_MPU_T_COLD: c_int = 795;
// OMAP5430.CORE temperature definitions
// bandgap clock limits
pub const OMAP5430_CORE_MAX_FREQ: c_int = 1500000;
pub const OMAP5430_CORE_MIN_FREQ: c_int = 1000000;
// interrupts thresholds
pub const OMAP5430_CORE_TSHUT_HOT: c_int = 915;
pub const OMAP5430_CORE_TSHUT_COLD: c_int = 900;
pub const OMAP5430_CORE_T_HOT: c_int = 800;
pub const OMAP5430_CORE_T_COLD: c_int = 795;
