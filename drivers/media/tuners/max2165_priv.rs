//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/max2165_priv.h
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
// Driver for Maxim MAX2165 silicon tuner
//
// Copyright (c) 2009 David T. L. Wong <davidtlwong@gmail.com>
//
pub const REG_NDIV_INT: c_uint = 0x00;
pub const REG_NDIV_FRAC2: c_uint = 0x01;
pub const REG_NDIV_FRAC1: c_uint = 0x02;
pub const REG_NDIV_FRAC0: c_uint = 0x03;
pub const REG_TRACK_FILTER: c_uint = 0x04;
pub const REG_LNA: c_uint = 0x05;
pub const REG_PLL_CFG: c_uint = 0x06;
pub const REG_TEST: c_uint = 0x07;
pub const REG_SHUTDOWN: c_uint = 0x08;
pub const REG_VCO_CTRL: c_uint = 0x09;
pub const REG_BASEBAND_CTRL: c_uint = 0x0A;
pub const REG_DC_OFFSET_CTRL: c_uint = 0x0B;
pub const REG_DC_OFFSET_DAC: c_uint = 0x0C;
pub const REG_ROM_TABLE_ADDR: c_uint = 0x0D;
// Read Only Registers
pub const REG_ROM_TABLE_DATA: c_uint = 0x10;
pub const REG_STATUS: c_uint = 0x11;
pub const REG_AUTOTUNE: c_uint = 0x12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max2165_priv {
    pub config: *mut max2165_config,
    pub i2c: *mut i2c_adapter,
    pub frequency: u32,
    pub bandwidth: u32,
    pub tf_ntch_low_cfg: u8,
    pub tf_ntch_hi_cfg: u8,
    pub tf_balun_low_ref: u8,
    pub tf_balun_hi_ref: u8,
    pub bb_filter_7mhz_cfg: u8,
    pub bb_filter_8mhz_cfg: u8,
}
