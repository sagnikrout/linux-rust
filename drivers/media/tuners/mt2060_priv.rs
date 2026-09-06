//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/mt2060_priv.h
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
// Driver for Microtune MT2060 "Single chip dual conversion broadband tuner"
//
// Copyright (c) 2006 Olivier DANET <odanet@caramail.com>
//
// Uncomment the #define below to enable spurs checking. The results where quite unconvincing.
// #define MT2060_SPURCHECK
// This driver is based on the information available in the datasheet of the
//
pub const I2C_ADDRESS: c_uint = 0x60;
pub const REG_PART_REV: c_int = 0;
pub const REG_LO1C1: c_int = 1;
pub const REG_LO1C2: c_int = 2;
pub const REG_LO2C1: c_int = 3;
pub const REG_LO2C2: c_int = 4;
pub const REG_LO2C3: c_int = 5;
pub const REG_LO_STATUS: c_int = 6;
pub const REG_FM_FREQ: c_int = 7;
pub const REG_MISC_STAT: c_int = 8;
pub const REG_MISC_CTRL: c_int = 9;
pub const REG_RESERVED_A: c_uint = 0x0A;
pub const REG_VGAG: c_uint = 0x0B;
pub const REG_LO1B1: c_uint = 0x0C;
pub const REG_LO1B2: c_uint = 0x0D;
pub const REG_LOTO: c_uint = 0x11;
pub const PART_REV: c_uint = 0x63 // The current driver works only with PART=6 and REV=3 chips;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt2060_priv {
    pub cfg: *mut mt2060_config,
    pub i2c: *mut i2c_adapter,
    pub client: *mut i2c_client,
    pub config: mt2060_config,
    pub i2c_max_regs: u8,
    pub frequency: u32,
    pub if1_freq: u16,
    pub fmfreq: u8,
//
// Use REG_MISC_CTRL register for sleep. That drops sleep power usage
// about 0.9W (huge!). Register bit meanings are unknown, so let it be
// disabled by default to avoid possible regression. Convert driver to
// i2c model in order to enable it.
//
    pub sleep: bool,
}
