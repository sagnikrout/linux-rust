//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/tda8290.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda8290_lna {
    TDA8290_LNA_OFF = 0,
    TDA8290_LNA_GP0_HIGH_ON = 1,
    TDA8290_LNA_GP0_HIGH_OFF = 2,
    TDA8290_LNA_ON_BRIDGE = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda829x_config {
    pub lna_cfg: tda8290_lna,
    pub probe_tuner:1: c_uint,
pub const TDA829X_PROBE_TUNER: c_int = 0;
pub const TDA829X_DONT_PROBE: c_int = 1;
    pub no_i2c_gate:1: c_uint,
    pub tda18271_std_map: *mut tda18271_std_map,
}

extern "C" {
    pub fn tda829x_probe(i2c_adap: *mut i2c_adapter, i2c_addr: u8) -> c_int;
}

