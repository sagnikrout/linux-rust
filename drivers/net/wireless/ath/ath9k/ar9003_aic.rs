//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/ar9003_aic.h
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
// Copyright (c) 2015 Qualcomm Atheros Inc.
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
pub const ATH_AIC_MAX_COM_ATT_DB_TABLE: c_int = 6;
pub const ATH_AIC_MAX_AIC_LIN_TABLE: c_int = 69;
pub const ATH_AIC_MIN_ROT_DIR_ATT_DB: c_int = 0;
pub const ATH_AIC_MIN_ROT_QUAD_ATT_DB: c_int = 0;
pub const ATH_AIC_MAX_ROT_DIR_ATT_DB: c_int = 37;
pub const ATH_AIC_MAX_ROT_QUAD_ATT_DB: c_int = 37;
pub const ATH_AIC_SRAM_AUTO_INCREMENT: c_uint = 0x80000000;
pub const ATH_AIC_SRAM_GAIN_TABLE_OFFSET: c_uint = 0x280;
pub const ATH_AIC_SRAM_CAL_OFFSET: c_uint = 0x140;
pub const ATH_AIC_SRAM_OFFSET: c_uint = 0x00;
pub const ATH_AIC_MEAS_MAG_THRESH: c_int = 20;
pub const ATH_AIC_BT_JUPITER_CTRL: c_uint = 0x66820;
pub const ATH_AIC_BT_AIC_ENABLE: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aic_cal_state {
    AIC_CAL_STATE_IDLE = 0,
    AIC_CAL_STATE_STARTED,
    AIC_CAL_STATE_DONE,
    AIC_CAL_STATE_ERROR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_aic_sram_info {
    pub valid:1: bool,
    pub vga_quad_sign:1: bool,
    pub vga_dir_sign:1: bool,
    pub rot_quad_att_db: u8,
    pub rot_dir_att_db: u8,
    pub com_att_6db: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_aic_out_info {
    pub dir_path_gain_lin: i16,
    pub quad_path_gain_lin: i16,
}

extern "C" {
    pub fn ar9003_aic_calibration(ah: *mut ath_hw) -> u8;
}
extern "C" {
    pub fn ar9003_aic_start_normal(ah: *mut ath_hw) -> u8;
}
extern "C" {
    pub fn ar9003_aic_cal_reset(ah: *mut ath_hw) -> u8;
}
extern "C" {
    pub fn ar9003_aic_calibration_single(ah: *mut ath_hw) -> u8;
}
