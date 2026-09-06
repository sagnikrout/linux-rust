//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/calib.h
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

pub const AR_PHY_CCA_FILTERWINDOW_LENGTH: c_int = 5;
// Internal noise floor can vary by about 6db depending on the frequency
pub const ATH9K_NF_CAL_NOISE_THRESH: c_int = 6;
pub const NUM_NF_READINGS: c_int = 6;
pub const ATH9K_NF_CAL_HIST_MAX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5416IniArray {
    pub ia_array: *mut u32,
    pub ia_rows: u32,
    pub ia_columns: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_cal_state {
    CAL_INACTIVE,
    CAL_WAITING,
    CAL_RUNNING,
    CAL_DONE
}

pub const MIN_CAL_SAMPLES: c_int = 1;
pub const MAX_CAL_SAMPLES: c_int = 64;
pub const INIT_LOG_COUNT: c_int = 5;
pub const PER_MIN_LOG_COUNT: c_int = 2;
pub const PER_MAX_LOG_COUNT: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_percal_data {
    pub calType: u32,
    pub calNumSamples: u32,
    pub calCountMax: u32,
    pub ): *mut *mut void (calCollect) (struct ath_hw,
    pub u8): *mut *mut *mut void (calPostProc) (struct ath_hw ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_cal_list {
    pub calData: *const ath9k_percal_data,
    pub calState: ath9k_cal_state,
    pub calNext: *mut ath9k_cal_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_nfcal_hist {
    pub nfCalBuffer: [i16; ATH9K_NF_CAL_HIST_MAX],
    pub currIndex: u8,
    pub privNF: i16,
    pub invalidNFcount: u8,
}

pub const MAX_PACAL_SKIPCOUNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_pacal_info {
    pub /: *mut *mut int32_t prev_offset; / Previous value of PA offset value,
    pub /: *mut *mut int8_t max_skipcount; / Max No. of times PACAL can be skipped,
    pub /: *mut *mut int8_t skipcount; / No. of times the PACAL to be skipped,
}

extern "C" {
    pub fn ath9k_hw_reset_calvalid(ah: *mut ath_hw) -> bool;
}
extern "C" {
    pub fn ath9k_hw_start_nfcal(ah: *mut ath_hw, update: bool);
}
extern "C" {
    pub fn ath9k_hw_loadnf(ah: *mut ath_hw, chan: *mut ath9k_channel) -> c_int;
}
extern "C" {
    pub fn ath9k_hw_getnf(ah: *mut ath_hw, chan: *mut ath9k_channel) -> bool;
}
extern "C" {
    pub fn ath9k_hw_bstuck_nfcal(ah: *mut ath_hw);
}
