//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max98363.h
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
// Copyright (c) 2022 Analog Devices Inc.
pub const MAX98363_R2000_SW_RESET: c_uint = 0x2000;
pub const MAX98363_R2001_INTR_RAW: c_uint = 0x2001;
pub const MAX98363_R2003_INTR_STATE: c_uint = 0x2003;
pub const MAX98363_R2005_INTR_FALG: c_uint = 0x2005;
pub const MAX98363_R2007_INTR_EN: c_uint = 0x2007;
pub const MAX98363_R2009_INTR_CLR: c_uint = 0x2009;
pub const MAX98363_R2021_ERR_MON_CTRL: c_uint = 0x2021;
pub const MAX98363_R2022_SPK_MON_THRESH: c_uint = 0x2022;
pub const MAX98363_R2023_SPK_MON_DURATION: c_uint = 0x2023;
pub const MAX98363_R2030_TONE_GEN_CFG: c_uint = 0x2030;
pub const MAX98363_R203F_TONE_GEN_EN: c_uint = 0x203F;
pub const MAX98363_R2040_AMP_VOL: c_uint = 0x2040;
pub const MAX98363_R2041_AMP_GAIN: c_uint = 0x2041;
pub const MAX98363_R2042_DSP_CFG: c_uint = 0x2042;
pub const MAX98363_R21FF_REV_ID: c_uint = 0x21FF;
// MAX98363_R2021_ERR_MON_CTRL

// MAX98363_R2042_DSP_CFG

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98363_priv {
    pub regmap: *mut regmap,
    pub slave: *mut sdw_slave,
    pub hw_init: bool,
    pub first_hw_init: bool,
}
