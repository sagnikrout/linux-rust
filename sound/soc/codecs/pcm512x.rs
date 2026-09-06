//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/pcm512x.h
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
// Driver for the PCM512x CODECs
//
// Author:	Mark Brown <broonie@kernel.org>
// Copyright 2014 Linaro Ltd
//

pub const PCM512x_VIRT_BASE: c_uint = 0x100;
pub const PCM512x_PAGE_LEN: c_uint = 0x100;

pub const PCM512x_PAGE: c_int = 0;

// Page 0, Register 1 - reset

// Page 0, Register 2 - power

pub const PCM512x_RQPD_SHIFT: c_int = 0;

pub const PCM512x_RQST_SHIFT: c_int = 4;
// Page 0, Register 3 - mute

pub const PCM512x_RQMR_SHIFT: c_int = 0;

pub const PCM512x_RQML_SHIFT: c_int = 4;
// Page 0, Register 4 - PLL

pub const PCM512x_PLLE_SHIFT: c_int = 0;

pub const PCM512x_PLCK_SHIFT: c_int = 4;
// Page 0, Register 7 - DSP

pub const PCM512x_SDSL_SHIFT: c_int = 0;

pub const PCM512x_DEMP_SHIFT: c_int = 4;
// Page 0, Register 8 - GPIO output enable

// Page 0, Register 9 - BCK, LRCLK configuration

pub const PCM512x_LRKO_SHIFT: c_int = 0;

pub const PCM512x_BCKO_SHIFT: c_int = 4;

pub const PCM512x_BCKP_SHIFT: c_int = 5;
// Page 0, Register 12 - Master mode BCK, LRCLK reset

pub const PCM512x_RLRK_SHIFT: c_int = 0;

pub const PCM512x_RBCK_SHIFT: c_int = 1;
// Page 0, Register 13 - PLL reference

pub const PCM512x_SREF_SHIFT: c_int = 4;

// Page 0, Register 14 - DAC reference

pub const PCM512x_SDAC_SHIFT: c_int = 4;

// Page 0, Register 16, 18 - GPIO source for DAC, PLL

pub const PCM512x_GREF_SHIFT: c_int = 0;

// Page 0, Register 19 - synchronize

// Page 0, Register 34 - fs speed mode

pub const PCM512x_FSSP_SHIFT: c_int = 0;

// Page 0, Register 37 - Error detection

// Page 0, Register 40 - I2S configuration

pub const PCM512x_ALEN_SHIFT: c_int = 0;

pub const PCM512x_AFMT_SHIFT: c_int = 4;

// Page 0, Register 42 - DAC routing
pub const PCM512x_AUPR_SHIFT: c_int = 0;
pub const PCM512x_AUPL_SHIFT: c_int = 4;
// Page 0, Register 59 - auto mute
pub const PCM512x_ATMR_SHIFT: c_int = 0;
pub const PCM512x_ATML_SHIFT: c_int = 4;
// Page 0, Register 63 - ramp rates
pub const PCM512x_VNDF_SHIFT: c_int = 6;
pub const PCM512x_VNDS_SHIFT: c_int = 4;
pub const PCM512x_VNUF_SHIFT: c_int = 2;
pub const PCM512x_VNUS_SHIFT: c_int = 0;
// Page 0, Register 64 - emergency ramp rates
pub const PCM512x_VEDF_SHIFT: c_int = 6;
pub const PCM512x_VEDS_SHIFT: c_int = 4;
// Page 0, Register 65 - Digital mute enables
pub const PCM512x_ACTL_SHIFT: c_int = 2;
pub const PCM512x_AMLE_SHIFT: c_int = 1;
pub const PCM512x_AMRE_SHIFT: c_int = 0;
// Page 0, Register 80-85, GPIO output selection

pub const PCM512x_GxSL_SHIFT: c_int = 0;

// Page 1, Register 2 - analog volume control
pub const PCM512x_RAGN_SHIFT: c_int = 0;
pub const PCM512x_LAGN_SHIFT: c_int = 4;
// Page 1, Register 7 - analog boost control
pub const PCM512x_AGBR_SHIFT: c_int = 0;
pub const PCM512x_AGBL_SHIFT: c_int = 4;
extern "C" {
    pub fn pcm512x_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
}
extern "C" {
    pub fn pcm512x_remove(dev: *mut device);
}
