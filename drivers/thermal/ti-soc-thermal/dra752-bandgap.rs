//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/ti-soc-thermal/dra752-bandgap.h
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
// DRA752 bandgap registers, bitfields and temperature definitions
//
// Copyright (C) 2013 Texas Instruments Incorporated - http://www.ti.com
// Contact:
// Eduardo Valentin <eduardo.valentin@ti.com>
// Tero Kristo <t-kristo@ti.com>
//
// This is an auto generated file.
//
// *** DRA752
//
// Below, in sequence, are the Register definitions,
// the bitfields and the temperature definitions for DRA752.
//
// DRA752 register definitions
//
// Registers are defined as offsets. The offsets are
// relative to FUSE_OPP_BGAP_GPU on DRA752.
// DRA752_BANDGAP_BASE		0x4a0021e0
//
// Register below are grouped by domain (not necessarily in offset order)
//
// DRA752.common register offsets
pub const DRA752_BANDGAP_CTRL_1_OFFSET: c_uint = 0x1a0;
pub const DRA752_BANDGAP_STATUS_1_OFFSET: c_uint = 0x1c8;
pub const DRA752_BANDGAP_CTRL_2_OFFSET: c_uint = 0x39c;
pub const DRA752_BANDGAP_STATUS_2_OFFSET: c_uint = 0x3b8;
// DRA752.core register offsets
pub const DRA752_STD_FUSE_OPP_BGAP_CORE_OFFSET: c_uint = 0x8;
pub const DRA752_TEMP_SENSOR_CORE_OFFSET: c_uint = 0x154;
pub const DRA752_BANDGAP_THRESHOLD_CORE_OFFSET: c_uint = 0x1ac;
pub const DRA752_DTEMP_CORE_1_OFFSET: c_uint = 0x20c;
pub const DRA752_DTEMP_CORE_2_OFFSET: c_uint = 0x210;
// DRA752.iva register offsets
pub const DRA752_STD_FUSE_OPP_BGAP_IVA_OFFSET: c_uint = 0x388;
pub const DRA752_TEMP_SENSOR_IVA_OFFSET: c_uint = 0x398;
pub const DRA752_BANDGAP_THRESHOLD_IVA_OFFSET: c_uint = 0x3a4;
pub const DRA752_DTEMP_IVA_1_OFFSET: c_uint = 0x3d4;
pub const DRA752_DTEMP_IVA_2_OFFSET: c_uint = 0x3d8;
// DRA752.mpu register offsets
pub const DRA752_STD_FUSE_OPP_BGAP_MPU_OFFSET: c_uint = 0x4;
pub const DRA752_TEMP_SENSOR_MPU_OFFSET: c_uint = 0x14c;
pub const DRA752_BANDGAP_THRESHOLD_MPU_OFFSET: c_uint = 0x1a4;
pub const DRA752_DTEMP_MPU_1_OFFSET: c_uint = 0x1e4;
pub const DRA752_DTEMP_MPU_2_OFFSET: c_uint = 0x1e8;
// DRA752.dspeve register offsets
pub const DRA752_STD_FUSE_OPP_BGAP_DSPEVE_OFFSET: c_uint = 0x384;
pub const DRA752_TEMP_SENSOR_DSPEVE_OFFSET: c_uint = 0x394;
pub const DRA752_BANDGAP_THRESHOLD_DSPEVE_OFFSET: c_uint = 0x3a0;
pub const DRA752_DTEMP_DSPEVE_1_OFFSET: c_uint = 0x3c0;
pub const DRA752_DTEMP_DSPEVE_2_OFFSET: c_uint = 0x3c4;
// DRA752.gpu register offsets
pub const DRA752_STD_FUSE_OPP_BGAP_GPU_OFFSET: c_uint = 0x0;
pub const DRA752_TEMP_SENSOR_GPU_OFFSET: c_uint = 0x150;
pub const DRA752_BANDGAP_THRESHOLD_GPU_OFFSET: c_uint = 0x1a8;
pub const DRA752_DTEMP_GPU_1_OFFSET: c_uint = 0x1f8;
pub const DRA752_DTEMP_GPU_2_OFFSET: c_uint = 0x1fc;
//
// Register bitfields for DRA752
//
// All the macros below define the required bits for
// controlling temperature on DRA752. Bit defines are
// grouped by register.
//
// DRA752.BANDGAP_STATUS_1

// DRA752.BANDGAP_CTRL_2

// DRA752.BANDGAP_STATUS_2

// DRA752.BANDGAP_CTRL_1

// DRA752.TEMP_SENSOR

// DRA752.BANDGAP_THRESHOLD

//
// Temperature limits and thresholds for DRA752
//
// All the macros below are definitions for handling the
// ADC conversions and representation of temperature limits
// and thresholds for DRA752. Definitions are grouped
// by temperature domain.
//
// DRA752.common temperature definitions
// ADC conversion table limits
pub const DRA752_ADC_START_VALUE: c_int = 540;
pub const DRA752_ADC_END_VALUE: c_int = 945;
// DRA752.GPU temperature definitions
// bandgap clock limits
pub const DRA752_GPU_MAX_FREQ: c_int = 1500000;
pub const DRA752_GPU_MIN_FREQ: c_int = 1000000;
// interrupts thresholds
pub const DRA752_GPU_T_HOT: c_int = 800;
pub const DRA752_GPU_T_COLD: c_int = 795;
// DRA752.MPU temperature definitions
// bandgap clock limits
pub const DRA752_MPU_MAX_FREQ: c_int = 1500000;
pub const DRA752_MPU_MIN_FREQ: c_int = 1000000;
// interrupts thresholds
pub const DRA752_MPU_T_HOT: c_int = 800;
pub const DRA752_MPU_T_COLD: c_int = 795;
// DRA752.CORE temperature definitions
// bandgap clock limits
pub const DRA752_CORE_MAX_FREQ: c_int = 1500000;
pub const DRA752_CORE_MIN_FREQ: c_int = 1000000;
// interrupts thresholds
pub const DRA752_CORE_T_HOT: c_int = 800;
pub const DRA752_CORE_T_COLD: c_int = 795;
// DRA752.DSPEVE temperature definitions
// bandgap clock limits
pub const DRA752_DSPEVE_MAX_FREQ: c_int = 1500000;
pub const DRA752_DSPEVE_MIN_FREQ: c_int = 1000000;
// interrupts thresholds
pub const DRA752_DSPEVE_T_HOT: c_int = 800;
pub const DRA752_DSPEVE_T_COLD: c_int = 795;
// DRA752.IVA temperature definitions
// bandgap clock limits
pub const DRA752_IVA_MAX_FREQ: c_int = 1500000;
pub const DRA752_IVA_MIN_FREQ: c_int = 1000000;
// interrupts thresholds
pub const DRA752_IVA_T_HOT: c_int = 800;
pub const DRA752_IVA_T_COLD: c_int = 795;
