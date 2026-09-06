//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/frequency/adf4350.h
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
// ADF4350/ADF4351 SPI PLL driver
//
// Copyright 2012-2013 Analog Devices Inc.
//
// Registers
pub const ADF4350_REG0: c_int = 0;
pub const ADF4350_REG1: c_int = 1;
pub const ADF4350_REG2: c_int = 2;
pub const ADF4350_REG3: c_int = 3;
pub const ADF4350_REG4: c_int = 4;
pub const ADF4350_REG5: c_int = 5;
// REG0 Bit Definitions

// REG1 Bit Definitions

// REG2 Bit Definitions

pub const ADF4350_MUXOUT_THREESTATE: c_int = 0;
pub const ADF4350_MUXOUT_DVDD: c_int = 1;
pub const ADF4350_MUXOUT_GND: c_int = 2;
pub const ADF4350_MUXOUT_R_DIV_OUT: c_int = 3;
pub const ADF4350_MUXOUT_N_DIV_OUT: c_int = 4;
pub const ADF4350_MUXOUT_ANALOG_LOCK_DETECT: c_int = 5;
pub const ADF4350_MUXOUT_DIGITAL_LOCK_DETECT: c_int = 6;
// REG3 Bit Definitions

// REG4 Bit Definitions

// REG5 Bit Definitions

// Specifications

pub const ADF4350_MAX_MODULUS: c_int = 4095;
pub const ADF4350_MAX_R_CNT: c_int = 1023;
//
// struct adf4350_platform_data - platform specific information
// @name:		Optional device name.
// @clkin:		REFin frequency in Hz.
// @channel_spacing:	Channel spacing in Hz (influences MODULUS).
// @power_up_frequency:	Optional, If set in Hz the PLL tunes to the desired
// frequency on probe.
// @ref_div_factor:	Optional, if set the driver skips dynamic calculation
// and uses this default value instead.
// @ref_doubler_en:	Enables reference doubler.
// @ref_div2_en:	Enables reference divider.
// @r2_user_settings:	User defined settings for ADF4350/1 REGISTER_2.
// @r3_user_settings:	User defined settings for ADF4350/1 REGISTER_3.
// @r4_user_settings:	User defined settings for ADF4350/1 REGISTER_4.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf4350_platform_data {
    pub name: [c_char; 32],
    pub clkin: c_ulong,
    pub channel_spacing: c_ulong,
    pub power_up_frequency: c_ulonglong,
    pub /: *mut *mut unsigned short ref_div_factor; / 10-bit R counter,
    pub ref_doubler_en: bool,
    pub ref_div2_en: bool,
    pub r2_user_settings: unsigned,
    pub r3_user_settings: unsigned,
    pub r4_user_settings: unsigned,
}
