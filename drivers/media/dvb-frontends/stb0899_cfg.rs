//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stb0899_cfg.h
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
pub const STB0899_DVBS2_ESNO_AVE: c_int = 3;
pub const STB0899_DVBS2_ESNO_QUANT: c_int = 32;
pub const STB0899_DVBS2_AVFRAMES_COARSE: c_int = 10;
pub const STB0899_DVBS2_AVFRAMES_FINE: c_int = 20;
pub const STB0899_DVBS2_MISS_THRESHOLD: c_int = 6;
pub const STB0899_DVBS2_UWP_THRESHOLD_ACQ: c_int = 1125;
pub const STB0899_DVBS2_UWP_THRESHOLD_TRACK: c_int = 758;
pub const STB0899_DVBS2_UWP_THRESHOLD_SOF: c_int = 1350;
pub const STB0899_DVBS2_SOF_SEARCH_TIMEOUT: c_int = 1664100;
pub const STB0899_DVBS2_BTR_NCO_BITS: c_int = 28;
pub const STB0899_DVBS2_BTR_GAIN_SHIFT_OFFSET: c_int = 15;
pub const STB0899_DVBS2_CRL_NCO_BITS: c_int = 30;
pub const STB0899_DVBS2_LDPC_MAX_ITER: c_int = 70;
