//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/tda10048.h
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
pub struct tda10048_config {
// the demodulator's i2c address
    pub demod_address: u8,
// serial/parallel output
pub const TDA10048_PARALLEL_OUTPUT: c_int = 0;
pub const TDA10048_SERIAL_OUTPUT: c_int = 1;
    pub output_mode: u8,
pub const TDA10048_BULKWRITE_200: c_int = 200;
pub const TDA10048_BULKWRITE_50: c_int = 50;
    pub fwbulkwritelen: u8,
// Spectral Inversion
pub const TDA10048_INVERSION_OFF: c_int = 0;
pub const TDA10048_INVERSION_ON: c_int = 1;
    pub inversion: u8,
pub const TDA10048_IF_3300: c_int = 3300;
pub const TDA10048_IF_3500: c_int = 3500;
pub const TDA10048_IF_3800: c_int = 3800;
pub const TDA10048_IF_4000: c_int = 4000;
pub const TDA10048_IF_4300: c_int = 4300;
pub const TDA10048_IF_4500: c_int = 4500;
pub const TDA10048_IF_4750: c_int = 4750;
pub const TDA10048_IF_5000: c_int = 5000;
pub const TDA10048_IF_36130: c_int = 36130;
    pub dtv6_if_freq_khz: u16,
    pub dtv7_if_freq_khz: u16,
    pub dtv8_if_freq_khz: u16,
pub const TDA10048_CLK_4000: c_int = 4000;
pub const TDA10048_CLK_16000: c_int = 16000;
    pub clk_freq_khz: u16,
// Disable I2C gate access
    pub disable_gate_access: u8,
    pub no_firmware: bool,
    pub set_pll: bool,
    pub pll_m: u8,
    pub pll_p: u8,
    pub pll_n: u8,
}

