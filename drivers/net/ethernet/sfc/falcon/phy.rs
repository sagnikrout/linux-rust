//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/falcon/phy.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2007-2010 Solarflare Communications Inc.
//
// 10Xpress (SFX7101) PHY
//
extern "C" {
    pub fn tenxpress_set_id_led(efx: *mut ef4_nic, mode: ef4_led_mode);
}
//
// AMCC/Quake QT202x PHYs
//
// These PHYs provide various H/W control states for LEDs

// What link the LED tracks

extern "C" {
    pub fn falcon_qt202x_set_led(p: *mut ef4_nic, led: c_int, state: c_int);
}
//
// Transwitch CX4 retimer
//
pub const TXC_GPIO_DIR_INPUT: c_int = 0;
pub const TXC_GPIO_DIR_OUTPUT: c_int = 1;
extern "C" {
    pub fn falcon_txc_set_gpio_dir(efx: *mut ef4_nic, pin: c_int, dir: c_int);
}
extern "C" {
    pub fn falcon_txc_set_gpio_val(efx: *mut ef4_nic, pin: c_int, val: c_int);
}
