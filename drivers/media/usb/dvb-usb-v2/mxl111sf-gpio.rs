//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/mxl111sf-gpio.h
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
// mxl111sf-gpio.h - driver for the MaxLinear MXL111SF
//
// Copyright (C) 2010-2014 Michael Krufky <mkrufky@linuxtv.org>
//

extern "C" {
    pub fn mxl111sf_set_gpio(state: *mut mxl111sf_state, gpio: c_int, val: c_int) -> c_int;
}
extern "C" {
    pub fn mxl111sf_init_port_expander(state: *mut mxl111sf_state) -> c_int;
}
pub const MXL111SF_GPIO_MOD_DVBT: c_int = 0;
pub const MXL111SF_GPIO_MOD_MH: c_int = 1;
pub const MXL111SF_GPIO_MOD_ATSC: c_int = 2;
extern "C" {
    pub fn mxl111sf_gpio_mode_switch(state: *mut mxl111sf_state, mode: c_uint) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl111sf_mux_config {
    PIN_MUX_DEFAULT = 0,
    PIN_MUX_TS_OUT_PARALLEL,
    PIN_MUX_TS_OUT_SERIAL,
    PIN_MUX_GPIO_MODE,
    PIN_MUX_TS_SERIAL_IN_MODE_0,
    PIN_MUX_TS_SERIAL_IN_MODE_1,
    PIN_MUX_TS_SPI_IN_MODE_0,
    PIN_MUX_TS_SPI_IN_MODE_1,
    PIN_MUX_TS_PARALLEL_IN,
    PIN_MUX_BT656_I2S_MODE,
}
