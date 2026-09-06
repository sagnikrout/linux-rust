//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/mxl111sf-phy.h
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
// mxl111sf-phy.h - driver for the MaxLinear MXL111SF
//
// Copyright (C) 2010-2014 Michael Krufky <mkrufky@linuxtv.org>
//

extern "C" {
    pub fn mxl1x1sf_soft_reset(state: *mut mxl111sf_state) -> c_int;
}
extern "C" {
    pub fn mxl1x1sf_set_device_mode(state: *mut mxl111sf_state, mode: c_int) -> c_int;
}
extern "C" {
    pub fn mxl1x1sf_top_master_ctrl(state: *mut mxl111sf_state, onoff: c_int) -> c_int;
}
extern "C" {
    pub fn mxl111sf_disable_656_port(state: *mut mxl111sf_state) -> c_int;
}
extern "C" {
    pub fn mxl111sf_init_tuner_demod(state: *mut mxl111sf_state) -> c_int;
}
extern "C" {
    pub fn mxl111sf_enable_usb_output(state: *mut mxl111sf_state) -> c_int;
}
extern "C" {
    pub fn mxl111sf_init_i2s_port(state: *mut mxl111sf_state, sample_size: u8) -> c_int;
}
extern "C" {
    pub fn mxl111sf_disable_i2s_port(state: *mut mxl111sf_state) -> c_int;
}
extern "C" {
    pub fn mxl111sf_config_spi(state: *mut mxl111sf_state, onoff: c_int) -> c_int;
}
