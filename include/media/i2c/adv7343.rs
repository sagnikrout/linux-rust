//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/adv7343.h
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
// ADV7343 header file
//
// Copyright (C) 2009 Texas Instruments Incorporated - http://www.ti.com
//

//
// struct adv7343_power_mode - power mode configuration.
// @sleep_mode: on enable the current consumption is reduced to micro ampere
// level. All DACs and the internal PLL circuit are disabled.
// Registers can be read from and written in sleep mode.
// @pll_control: PLL and oversampling control. This control allows internal
// PLL 1 circuit to be powered down and the oversampling to be
// switched off.
// @dac: array to configure power on/off DAC's 1..6
//
// Power mode register (Register 0x0), for more info refer REGISTER MAP ACCESS
// section of datasheet[1], table 17 page no 30.
//
// [1] http://www.analog.com/static/imported-files/data_sheets/ADV7342_7343.pdf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7343_power_mode {
    pub sleep_mode: bool,
    pub pll_control: bool,
    pub dac: [u32; 6],
}

//
// struct adv7343_sd_config - SD Only Output Configuration.
// @sd_dac_out: array configuring SD DAC Outputs 1 and 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7343_sd_config {
// SD only Output Configuration
    pub sd_dac_out: [u32; 2],
}

//
// struct adv7343_platform_data - Platform data values and access functions.
// @mode_config: Configuration for power mode.
// @sd_config: SD Only Configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7343_platform_data {
    pub mode_config: adv7343_power_mode,
    pub sd_config: adv7343_sd_config,
}
