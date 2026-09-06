//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sun4i/sun8i_vi_scaler.h
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


//
// Copyright (C) 2017 Jernej Skrabec <jernej.skrabec@siol.net>
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const DE2_VI_SCALER_UNIT_BASE: c_uint = 0x20000;
pub const DE2_VI_SCALER_UNIT_SIZE: c_uint = 0x20000;
pub const DE3_VI_SCALER_UNIT_BASE: c_uint = 0x20000;
pub const DE3_VI_SCALER_UNIT_SIZE: c_uint = 0x08000;
pub const DE33_VI_SCALER_UNIT_BASE: c_uint = 0x4000;
// this two macros assumes 16 fractional bits which is standard in DRM
pub const SUN8I_VI_SCALER_SCALE_MIN: c_int = 1;

pub const SUN8I_VI_SCALER_SCALE_FRAC: c_int = 20;
pub const SUN8I_VI_SCALER_PHASE_FRAC: c_int = 20;
pub const SUN8I_VI_SCALER_COEFF_COUNT: c_int = 32;

pub const SUN50I_SCALER_VSU_SCALE_MODE_UI: c_int = 0;
pub const SUN50I_SCALER_VSU_SCALE_MODE_NORMAL: c_int = 1;
pub const SUN50I_SCALER_VSU_SCALE_MODE_ED_SCALE: c_int = 2;

extern "C" {
    pub fn sun8i_vi_scaler_enable(layer: *mut sun8i_layer, enable: bool);
}
