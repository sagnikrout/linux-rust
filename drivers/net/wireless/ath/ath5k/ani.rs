//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath5k/ani.h
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
// Copyright (C) 2010 Bruno Randolf <br1@einfach.org>
//
// Permission to use, copy, modify, and/or distribute this software for any
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

// these thresholds are relative to the ATH5K_ANI_LISTEN_PERIOD
pub const ATH5K_ANI_LISTEN_PERIOD: c_int = 100;
pub const ATH5K_ANI_OFDM_TRIG_HIGH: c_int = 500;
pub const ATH5K_ANI_OFDM_TRIG_LOW: c_int = 200;
pub const ATH5K_ANI_CCK_TRIG_HIGH: c_int = 200;
pub const ATH5K_ANI_CCK_TRIG_LOW: c_int = 100;
// average beacon RSSI thresholds
pub const ATH5K_ANI_RSSI_THR_HIGH: c_int = 40;
pub const ATH5K_ANI_RSSI_THR_LOW: c_int = 7;
// maximum available levels
pub const ATH5K_ANI_MAX_FIRSTEP_LVL: c_int = 2;
pub const ATH5K_ANI_MAX_NOISE_IMM_LVL: c_int = 1;
//
// enum ath5k_ani_mode - mode for ANI / noise sensitivity
//
// @ATH5K_ANI_MODE_OFF: Turn ANI off. This can be useful to just stop the ANI
// algorithm after it has been on auto mode.
// @ATH5K_ANI_MODE_MANUAL_LOW: Manually set all immunity parameters to low,
// maximizing sensitivity. ANI will not run.
// @ATH5K_ANI_MODE_MANUAL_HIGH: Manually set all immunity parameters to high,
// minimizing sensitivity. ANI will not run.
// @ATH5K_ANI_MODE_AUTO: Automatically control immunity parameters based on the
// amount of OFDM and CCK frame errors (default).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_ani_mode {
    ATH5K_ANI_MODE_OFF		= 0,
    ATH5K_ANI_MODE_MANUAL_LOW	= 1,
    ATH5K_ANI_MODE_MANUAL_HIGH	= 2,
    ATH5K_ANI_MODE_AUTO		= 3
}

//
// struct ath5k_ani_state - ANI state and associated counters
// @ani_mode: One of enum ath5k_ani_mode
// @noise_imm_level: Noise immunity level
// @spur_level: Spur immunity level
// @firstep_level: FIRstep level
// @ofdm_weak_sig: OFDM weak signal detection state (on/off)
// @cck_weak_sig: CCK weak signal detection state (on/off)
// @max_spur_level: Max spur immunity level (chip specific)
// @listen_time: Listen time
// @ofdm_errors: OFDM timing error count
// @cck_errors: CCK timing error count
// @last_cc: The &struct ath_cycle_counters (for stats)
// @last_listen: Listen time from previous run (for stats)
// @last_ofdm_errors: OFDM timing error count from previous run (for tats)
// @last_cck_errors: CCK timing error count from previous run (for stats)
// @sum_ofdm_errors: Sum of OFDM timing errors (for stats)
// @sum_cck_errors: Sum of all CCK timing errors (for stats)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_ani_state {
    pub ani_mode: ath5k_ani_mode,
// state
    pub noise_imm_level: c_int,
    pub spur_level: c_int,
    pub firstep_level: c_int,
    pub ofdm_weak_sig: bool,
    pub cck_weak_sig: bool,
    pub max_spur_level: c_int,
// used by the algorithm
    pub listen_time: c_uint,
    pub ofdm_errors: c_uint,
    pub cck_errors: c_uint,
// debug/statistics only: numbers from last ANI calibration
    pub last_cc: ath_cycle_counters,
    pub last_listen: c_uint,
    pub last_ofdm_errors: c_uint,
    pub last_cck_errors: c_uint,
    pub sum_ofdm_errors: c_uint,
    pub sum_cck_errors: c_uint,
}

extern "C" {
    pub fn ath5k_ani_init(ah: *mut ath5k_hw, mode: ath5k_ani_mode);
}
extern "C" {
    pub fn ath5k_ani_mib_intr(ah: *mut ath5k_hw);
}
extern "C" {
    pub fn ath5k_ani_calibration(ah: *mut ath5k_hw);
}
// for manual control
extern "C" {
    pub fn ath5k_ani_set_noise_immunity_level(ah: *mut ath5k_hw, level: c_int);
}
extern "C" {
    pub fn ath5k_ani_set_spur_immunity_level(ah: *mut ath5k_hw, level: c_int);
}
extern "C" {
    pub fn ath5k_ani_set_firstep_level(ah: *mut ath5k_hw, level: c_int);
}
extern "C" {
    pub fn ath5k_ani_set_ofdm_weak_signal_detection(ah: *mut ath5k_hw, on: bool);
}
extern "C" {
    pub fn ath5k_ani_set_cck_weak_signal_detection(ah: *mut ath5k_hw, on: bool);
}
extern "C" {
    pub fn ath5k_ani_print_counters(ah: *mut ath5k_hw);
}
