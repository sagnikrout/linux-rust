//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/ts2020.h
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
pub struct ts2020_config {
    pub tuner_address: u8,
    pub frequency_div: u32,
//
// RF loop-through
//
    pub loop_through:1: bool,
//
// clock output
//
pub const TS2020_CLK_OUT_DISABLED: c_int = 0;
pub const TS2020_CLK_OUT_ENABLED: c_int = 1;
pub const TS2020_CLK_OUT_ENABLED_XTALOUT: c_int = 2;
    pub clk_out:2: u8,
//
// clock output divider
// 1 - 31
//
    pub clk_out_div:5: u8,
// Set to true to suppress stat polling
    pub dont_poll:1: bool,
//
// pointer to DVB frontend
//
    pub fe: *mut dvb_frontend,
//
// driver private, do not set value
//
    pub attach_in_use:1: u8,
// Operation to be called by the ts2020 driver to get the value of the
// AGC PWM tuner input as theoretically output by the demodulator.
//
    pub _agc_pwm): *mut *mut *mut int (get_agc_pwm)(struct dvb_frontend fe, u8,
}

// Do not add new ts2020_attach() users! Use I2C bindings instead.

