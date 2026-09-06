//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/dac/ad5624r.h
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
// AD5624R SPI DAC driver
//
// Copyright 2010-2011 Analog Devices Inc.
//
pub const AD5624R_DAC_CHANNELS: c_int = 4;
pub const AD5624R_ADDR_DAC0: c_uint = 0x0;
pub const AD5624R_ADDR_DAC1: c_uint = 0x1;
pub const AD5624R_ADDR_DAC2: c_uint = 0x2;
pub const AD5624R_ADDR_DAC3: c_uint = 0x3;
pub const AD5624R_ADDR_ALL_DAC: c_uint = 0x7;
pub const AD5624R_CMD_WRITE_INPUT_N: c_uint = 0x0;
pub const AD5624R_CMD_UPDATE_DAC_N: c_uint = 0x1;
pub const AD5624R_CMD_WRITE_INPUT_N_UPDATE_ALL: c_uint = 0x2;
pub const AD5624R_CMD_WRITE_INPUT_N_UPDATE_N: c_uint = 0x3;
pub const AD5624R_CMD_POWERDOWN_DAC: c_uint = 0x4;
pub const AD5624R_CMD_RESET: c_uint = 0x5;
pub const AD5624R_CMD_LDAC_SETUP: c_uint = 0x6;
pub const AD5624R_CMD_INTERNAL_REFER_SETUP: c_uint = 0x7;
pub const AD5624R_LDAC_PWRDN_NONE: c_uint = 0x0;
pub const AD5624R_LDAC_PWRDN_1K: c_uint = 0x1;
pub const AD5624R_LDAC_PWRDN_100K: c_uint = 0x2;
pub const AD5624R_LDAC_PWRDN_3STATE: c_uint = 0x3;
//
// struct ad5624r_chip_info - chip specific information
// @channels:		channel spec for the DAC
// @int_vref_mv:	AD5620/40/60: the internal reference voltage
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5624r_chip_info {
    pub channels: *const iio_chan_spec,
    pub int_vref_mv: u16,
}

//
// struct ad5624r_state - driver instance specific data
// @us:			spi_device
// @chip_info:		chip model specific constants, available modes etc
// @vref_mv:		actual reference voltage used
// @pwr_down_mask	power down mask
// @pwr_down_mode	current power down mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5624r_state {
    pub us: *mut spi_device,
    pub chip_info: *const ad5624r_chip_info,
    pub vref_mv: c_ushort,
    pub pwr_down_mask: unsigned,
    pub pwr_down_mode: unsigned,
}

//
// ad5624r_supported_device_ids:
// The AD5624/44/64 parts are available in different
// fixed internal reference voltage options.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad5624r_supported_device_ids {
    ID_AD5624R3,
    ID_AD5644R3,
    ID_AD5664R3,
    ID_AD5624R5,
    ID_AD5644R5,
    ID_AD5664R5,
}
