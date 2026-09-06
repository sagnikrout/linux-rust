//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/dib0090.h
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
// Linux-DVB Driver for DiBcom's DiB0090 base-band RF Tuner.
//
// Copyright (C) 2005-7 DiBcom (http://www.dibcom.fr/)
//
pub const DEFAULT_DIB0090_I2C_ADDRESS: c_uint = 0x60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dib0090_io_config {
    pub clock_khz: u32,
    pub pll_bypass:1: u8,
    pub pll_range:1: u8,
    pub pll_prediv:6: u8,
    pub pll_loopdiv:6: u8,
    pub /: *mut *mut u8 adc_clock_ratio; / valid is 8, 7 ,6,
    pub pll_int_loop_filt: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dib0090_wbd_slope {
    pub /: *mut *mut u16 max_freq; / for every frequency less than or equal to that field: this information is correct,
    pub slope_cold: u16,
    pub offset_cold: u16,
    pub slope_hot: u16,
    pub offset_hot: u16,
    pub wbd_gain: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dib0090_low_if_offset_table {
    pub std: c_int,
    pub RF_freq: u32,
    pub offset_khz: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dib0090_config {
    pub io: dib0090_io_config,
    pub int): *mut *mut *mut int (reset) (struct dvb_frontend ,,
    pub int): *mut *mut *mut int (sleep) (struct dvb_frontend ,,
// offset in kHz
    pub freq_offset_khz_uhf: c_int,
    pub freq_offset_khz_vhf: c_int,
    pub ): *mut *mut int (get_adc_power) (struct dvb_frontend,
    pub /: *mut *mut u8 clkouttobamse:1; / activate or deactivate clock output,
    pub analog_output: u8,
    pub i2c_address: u8,
// add drives and other things if necessary
    pub wbd_vhf_offset: u16,
    pub wbd_cband_offset: u16,
    pub use_pwm_agc: u8,
    pub clkoutdrive: u8,
    pub ls_cfg_pad_drv: u8,
    pub data_tx_drv: u8,
    pub in_soc: u8,
    pub low_if: *const dib0090_low_if_offset_table,
    pub fref_clock_ratio: u8,
    pub force_cband_input: u16,
    pub wbd: *mut dib0090_wbd_slope,
    pub is_dib7090e: u8,
    pub force_crystal_mode: u8,
}

extern "C" {
    pub fn dib0090_dcc_freq(fe: *mut dvb_frontend, fast: u8);
}
extern "C" {
    pub fn dib0090_pwm_gain_reset(fe: *mut dvb_frontend);
}
extern "C" {
    pub fn dib0090_get_wbd_target(tuner: *mut dvb_frontend) -> u16;
}
extern "C" {
    pub fn dib0090_get_wbd_offset(fe: *mut dvb_frontend) -> u16;
}
extern "C" {
    pub fn dib0090_gain_control(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn dib0090_get_tune_state(fe: *mut dvb_frontend) -> frontend_tune_state;
}
extern "C" {
    pub fn dib0090_set_tune_state(fe: *mut dvb_frontend, tune_state: frontend_tune_state) -> c_int;
}
extern "C" {
    pub fn dib0090_get_current_gain(fe: *mut dvb_frontend, rf: *mut *mut u16, bb: *mut *mut u16, rf_gain_limit: *mut *mut u16, rflt: *mut *mut u16);
}
extern "C" {
    pub fn dib0090_set_dc_servo(fe: *mut dvb_frontend, DC_servo_cutoff: u8);
}
extern "C" {
    pub fn dib0090_set_switch(fe: *mut dvb_frontend, sw1: u8, sw2: u8, sw3: u8) -> c_int;
}
extern "C" {
    pub fn dib0090_set_vga(fe: *mut dvb_frontend, onoff: u8) -> c_int;
}

