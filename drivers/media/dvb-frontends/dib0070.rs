//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/dib0070.h
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
// Linux-DVB Driver for DiBcom's DiB0070 base-band RF Tuner.
//
// Copyright (C) 2005-7 DiBcom (http://www.dibcom.fr/)
//
pub const DEFAULT_DIB0070_I2C_ADDRESS: c_uint = 0x60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dib0070_wbd_gain_cfg {
    pub freq: u16,
    pub wbd_gain_val: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dib0070_config {
    pub i2c_address: u8,
// tuner pins controlled externally
    pub int): *mut *mut *mut int (reset) (struct dvb_frontend ,,
    pub int): *mut *mut *mut int (sleep) (struct dvb_frontend ,,
// offset in kHz
    pub freq_offset_khz_uhf: c_int,
    pub freq_offset_khz_vhf: c_int,
    pub /: *mut *mut u8 osc_buffer_state; / 0= normal, 1= tri-state,
    pub clock_khz: u32,
    pub /: *mut *mut *mut u8 clock_pad_drive; / (Drive + 1)  2mA,
    pub /: *mut *mut u8 invert_iq; / invert Q - in case I or Q is inverted on the board,
    pub /: *mut *mut u8 force_crystal_mode; / if == 0 -> decision is made in the driver default: <24 -> 2, >=24 -> 1,
    pub flip_chip: u8,
    pub enable_third_order_filter: u8,
    pub charge_pump: u8,
    pub wbd_gain: *const dib0070_wbd_gain_cfg,
    pub vga_filter: u8,
}

extern "C" {
    pub fn dib0070_wbd_offset(: *mut dvb_frontend) -> u16;
}
extern "C" {
    pub fn dib0070_ctrl_agc_filter(: *mut dvb_frontend, open: u8);
}
extern "C" {
    pub fn dib0070_get_rf_output(fe: *mut dvb_frontend) -> u8;
}
extern "C" {
    pub fn dib0070_set_rf_output(fe: *mut dvb_frontend, no: u8) -> c_int;
}

