//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/dib3000mc.h
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
// Driver for DiBcom DiB3000MC/P-demodulator.
//
// Copyright (C) 2004-6 DiBcom (http://www.dibcom.fr/)
// Copyright (C) 2004-5 Patrick Boettcher (patrick.boettcher@posteo.de)
//
// This code is partially based on the previous dib3000mc.c .
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dib3000mc_config {
    pub agc: *mut dibx000_agc_config,
    pub phase_noise_mode: u8,
    pub impulse_noise_mode: u8,
    pub pwm3_inversion: u8,
    pub use_pwm3: u8,
    pub pwm3_value: u16,
    pub max_time: u16,
    pub ln_adc_level: u16,
    pub :1: u8 agc_command1,
    pub :1: u8 agc_command2,
    pub mobile_mode: u8,
    pub output_mpeg2_in_188_bytes: u8,
}

pub const DEFAULT_DIB3000MC_I2C_ADDRESS: c_int = 16;
pub const DEFAULT_DIB3000P_I2C_ADDRESS: c_int = 24;

extern "C" {
    pub fn dib3000mc_pid_control(fe: *mut dvb_frontend, index: c_int, pid: c_int, onoff: c_int) -> c_int;
}
extern "C" {
    pub fn dib3000mc_pid_parse(fe: *mut dvb_frontend, onoff: c_int) -> c_int;
}
extern "C" {
    pub fn dib3000mc_set_config(: *mut dvb_frontend, : *mut dib3000mc_config);
}
