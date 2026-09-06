//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panel/panel-samsung-s6e63m0.h
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


// SPDX-License-Identifier: GPL-2.0
// Manufacturer Command Set
pub const MCS_ELVSS_ON: c_uint = 0xb1;
pub const MCS_TEMP_SWIRE: c_uint = 0xb2;
pub const MCS_PENTILE_1: c_uint = 0xb3;
pub const MCS_PENTILE_2: c_uint = 0xb4;
pub const MCS_GAMMA_DELTA_Y_RED: c_uint = 0xb5;
pub const MCS_GAMMA_DELTA_X_RED: c_uint = 0xb6;
pub const MCS_GAMMA_DELTA_Y_GREEN: c_uint = 0xb7;
pub const MCS_GAMMA_DELTA_X_GREEN: c_uint = 0xb8;
pub const MCS_GAMMA_DELTA_Y_BLUE: c_uint = 0xb9;
pub const MCS_GAMMA_DELTA_X_BLUE: c_uint = 0xba;
pub const MCS_MIECTL1: c_uint = 0xc0;
pub const MCS_BCMODE: c_uint = 0xc1;
pub const MCS_ERROR_CHECK: c_uint = 0xd5;
pub const MCS_READ_ID1: c_uint = 0xda;
pub const MCS_READ_ID2: c_uint = 0xdb;
pub const MCS_READ_ID3: c_uint = 0xdc;
pub const MCS_LEVEL_2_KEY: c_uint = 0xf0;
pub const MCS_MTP_KEY: c_uint = 0xf1;
pub const MCS_DISCTL: c_uint = 0xf2;
pub const MCS_SRCCTL: c_uint = 0xf6;
pub const MCS_IFCTL: c_uint = 0xf7;
pub const MCS_PANELCTL: c_uint = 0xf8;
pub const MCS_PGAMMACTL: c_uint = 0xfa;
extern "C" {
    pub fn s6e63m0_remove(dev: *mut device);
}
