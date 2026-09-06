//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/tea5767.h
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
pub enum tea5767_xtal {
    TEA5767_LOW_LO_32768    = 0,
    TEA5767_HIGH_LO_32768   = 1,
    TEA5767_LOW_LO_13MHz    = 2,
    TEA5767_HIGH_LO_13MHz   = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tea5767_ctrl {
    pub port1:1: c_uint,
    pub port2:1: c_uint,
    pub high_cut:1: c_uint,
    pub st_noise:1: c_uint,
    pub soft_mute:1: c_uint,
    pub japan_band:1: c_uint,
    pub deemph_75:1: c_uint,
    pub pllref:1: c_uint,
    pub xtal_freq: tea5767_xtal,
}

extern "C" {
    pub fn tea5767_autodetection(i2c_adap: *mut *mut i2c_adapter, i2c_addr: u8) -> c_int;
}

