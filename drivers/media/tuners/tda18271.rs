//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/tda18271.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda18271_std_map_item {
    pub if_freq: u16,
// EP3[4:3]
    pub agc_mode:2: c_uint,
// EP3[2:0]
    pub std:3: c_uint,
// EP4[7]
    pub fm_rfn:1: c_uint,
// EP4[4:2]
    pub if_lvl:3: c_uint,
// EB22[6:0]
    pub rfagc_top:7: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda18271_std_map {
    pub fm_radio: tda18271_std_map_item,
    pub atv_b: tda18271_std_map_item,
    pub atv_dk: tda18271_std_map_item,
    pub atv_gh: tda18271_std_map_item,
    pub atv_i: tda18271_std_map_item,
    pub atv_l: tda18271_std_map_item,
    pub atv_lc: tda18271_std_map_item,
    pub atv_mn: tda18271_std_map_item,
    pub atsc_6: tda18271_std_map_item,
    pub dvbt_6: tda18271_std_map_item,
    pub dvbt_7: tda18271_std_map_item,
    pub dvbt_8: tda18271_std_map_item,
    pub qam_6: tda18271_std_map_item,
    pub qam_7: tda18271_std_map_item,
    pub qam_8: tda18271_std_map_item,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda18271_role {
    TDA18271_MASTER = 0,
    TDA18271_SLAVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda18271_i2c_gate {
    TDA18271_GATE_AUTO = 0,
    TDA18271_GATE_ANALOG,
    TDA18271_GATE_DIGITAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda18271_output_options {
// slave tuner output & loop through & xtal oscillator always on
    TDA18271_OUTPUT_LT_XT_ON = 0,

// slave tuner output loop through off
    TDA18271_OUTPUT_LT_OFF = 1,

// xtal oscillator off
    TDA18271_OUTPUT_XT_OFF = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda18271_small_i2c {
    TDA18271_39_BYTE_CHUNK_INIT = 0,
    TDA18271_16_BYTE_CHUNK_INIT = 16,
    TDA18271_08_BYTE_CHUNK_INIT = 8,
    TDA18271_03_BYTE_CHUNK_INIT = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda18271_config {
// override default if freq / std settings (optional)
    pub std_map: *mut tda18271_std_map,
// master / slave tuner: master uses main pll, slave uses cal pll
    pub role: tda18271_role,
// use i2c gate provided by analog or digital demod
    pub gate: tda18271_i2c_gate,
// output options that can be disabled
    pub output_opt: tda18271_output_options,
// some i2c providers can't write all 39 registers at once
    pub small_i2c: tda18271_small_i2c,
// force rf tracking filter calibration on startup
    pub rf_cal_on_startup:1: c_uint,
// prevent any register access during attach(),
// delaying both IR & RF calibration until init()
// module option 'cal' overrides this delay
    pub delay_cal:1: c_uint,
// interface to saa713x / tda829x
    pub config: c_uint,
}

pub const TDA18271_CALLBACK_CMD_AGC_ENABLE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda18271_mode {
    TDA18271_ANALOG = 0,
    TDA18271_DIGITAL,
}

