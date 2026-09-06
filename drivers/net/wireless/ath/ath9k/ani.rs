//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/ani.h
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
// Copyright (c) 2008-2011 Atheros Communications Inc.
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

// units are errors per second
pub const ATH9K_ANI_OFDM_TRIG_HIGH: c_int = 3500;
pub const ATH9K_ANI_OFDM_TRIG_HIGH_BELOW_INI: c_int = 1000;
pub const ATH9K_ANI_OFDM_TRIG_HIGH_OLD: c_int = 500;
pub const ATH9K_ANI_OFDM_TRIG_LOW: c_int = 400;
pub const ATH9K_ANI_OFDM_TRIG_LOW_ABOVE_INI: c_int = 900;
pub const ATH9K_ANI_OFDM_TRIG_LOW_OLD: c_int = 200;
pub const ATH9K_ANI_CCK_TRIG_HIGH: c_int = 600;
pub const ATH9K_ANI_CCK_TRIG_HIGH_OLD: c_int = 200;
pub const ATH9K_ANI_CCK_TRIG_LOW: c_int = 300;
pub const ATH9K_ANI_CCK_TRIG_LOW_OLD: c_int = 100;
pub const ATH9K_ANI_SPUR_IMMUNE_LVL: c_int = 3;
pub const ATH9K_ANI_FIRSTEP_LVL: c_int = 2;
pub const ATH9K_ANI_RSSI_THR_HIGH: c_int = 40;
pub const ATH9K_ANI_RSSI_THR_LOW: c_int = 7;
pub const ATH9K_ANI_PERIOD: c_int = 300;
// in ms
pub const ATH9K_ANI_POLLINTERVAL: c_int = 1000;
pub const ATH9K_SIG_FIRSTEP_SETTING_MIN: c_int = 0;
pub const ATH9K_SIG_FIRSTEP_SETTING_MAX: c_int = 20;
pub const ATH9K_SIG_SPUR_IMM_SETTING_MIN: c_int = 0;
pub const ATH9K_SIG_SPUR_IMM_SETTING_MAX: c_int = 22;
// values here are relative to the INI
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_ani_cmd {
    ATH9K_ANI_OFDM_WEAK_SIGNAL_DETECTION = 0x1,
    ATH9K_ANI_FIRSTEP_LEVEL = 0x2,
    ATH9K_ANI_SPUR_IMMUNITY_LEVEL = 0x4,
    ATH9K_ANI_MRC_CCK = 0x8,
    ATH9K_ANI_ALL = 0xfff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_mib_stats {
    pub ackrcv_bad: u32,
    pub rts_bad: u32,
    pub rts_good: u32,
    pub fcs_bad: u32,
    pub beacons: u32,
}

// INI default values for ANI registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_ani_default {
    pub m1ThreshLow: u16,
    pub m2ThreshLow: u16,
    pub m1Thresh: u16,
    pub m2Thresh: u16,
    pub m2CountThr: u16,
    pub m2CountThrLow: u16,
    pub m1ThreshLowExt: u16,
    pub m2ThreshLowExt: u16,
    pub m1ThreshExt: u16,
    pub m2ThreshExt: u16,
    pub firstep: u16,
    pub firstepLow: u16,
    pub cycpwrThr1: u16,
    pub cycpwrThr1Ext: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5416AniState {
    pub noiseImmunityLevel: u8,
    pub ofdmNoiseImmunityLevel: u8,
    pub cckNoiseImmunityLevel: u8,
    pub ofdmsTurn: bool,
    pub mrcCCK: u8,
    pub spurImmunityLevel: u8,
    pub firstepLevel: u8,
    pub ofdmWeakSigDetect: bool,
    pub listenTime: u32,
    pub ofdmPhyErrCount: u32,
    pub cckPhyErrCount: u32,
    pub iniDef: ath9k_ani_default,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5416Stats {
    pub ast_ani_spurup: u32,
    pub ast_ani_spurdown: u32,
    pub ast_ani_ofdmon: u32,
    pub ast_ani_ofdmoff: u32,
    pub ast_ani_cckhigh: u32,
    pub ast_ani_ccklow: u32,
    pub ast_ani_stepup: u32,
    pub ast_ani_stepdown: u32,
    pub ast_ani_ofdmerrs: u32,
    pub ast_ani_cckerrs: u32,
    pub ast_ani_reset: u32,
    pub ast_ani_lneg_or_lzero: u32,
    pub avgbrssi: u32,
    pub ast_mibstats: ath9k_mib_stats,
}

extern "C" {
    pub fn ath9k_enable_mib_counters(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_disable_mib_counters(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_ani_init(ah: *mut ath_hw);
}
