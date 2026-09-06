//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath5k/rfgain.h
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
// RF Gain optimization
//
// Copyright (c) 2004-2009 Reyk Floeter <reyk@openbsd.org>
// Copyright (c) 2006-2009 Nick Kossifidis <mickflemm@gmail.com>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// struct ath5k_ini_rfgain - RF Gain table
// @rfg_register: RF Gain register address
// @rfg_value: Register value for 5 and 2GHz
//
// Mode-specific RF Gain table (64bytes) for RF5111/5112
// (RF5110 only comes with AR5210 and only supports a/turbo a mode so initial
// RF Gain values are included in AR5K_AR5210_INI)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_ini_rfgain {
    pub rfg_register: u16,
    pub /: *mut *mut u32 rfg_value[2]; / [freq (see below)],
}

// Initial RF Gain settings for RF5111
// 5GHz	2GHz
// Initial RF Gain settings for RF5112
// 5GHz	2GHz
// Initial RF Gain settings for RF2413
// Initial RF Gain settings for AR2316
// Initial RF Gain settings for RF5413
// 5GHz	2GHz
// Initial RF Gain settings for RF2425
pub const AR5K_GAIN_CRN_FIX_BITS_5111: c_int = 4;
pub const AR5K_GAIN_CRN_FIX_BITS_5112: c_int = 7;

pub const AR5K_GAIN_DYN_ADJUST_HI_MARGIN: c_int = 15;
pub const AR5K_GAIN_DYN_ADJUST_LO_MARGIN: c_int = 20;
pub const AR5K_GAIN_CCK_PROBE_CORR: c_int = 5;
pub const AR5K_GAIN_CCK_OFDM_GAIN_DELTA: c_int = 15;
pub const AR5K_GAIN_STEP_COUNT: c_int = 10;
// Check if our current measurement is inside our
// current variable attenuation window

//
// struct ath5k_gain_opt_step - An RF gain optimization step
// @gos_param: Set of parameters
// @gos_gain: Gain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_gain_opt_step {
    pub gos_param: [i8; AR5K_GAIN_CRN_MAX_FIX_BITS],
    pub gos_gain: i8,
}

//
// struct ath5k_gain_opt - RF Gain optimization ladder
// @go_default: The default step
// @go_steps_count: How many optimization steps
// @go_step: Array of &struct ath5k_gain_opt_step
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_gain_opt {
    pub go_default: u8,
    pub go_steps_count: u8,
    pub go_step: [ath5k_gain_opt_step; AR5K_GAIN_STEP_COUNT],
}

//
// RF5111
// Parameters on gos_param:
// 1) Tx clip PHY register
// 2) PWD 90 RF register
// 3) PWD 84 RF register
// 4) RFGainSel RF register
//
// RF5112
// Parameters on gos_param:
// 1) Mixgain ovr RF register
// 2) PWD 138 RF register
// 3) PWD 137 RF register
// 4) PWD 136 RF register
// 5) PWD 132 RF register
// 6) PWD 131 RF register
// 7) PWD 130 RF register
//
