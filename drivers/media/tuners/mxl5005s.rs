//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/mxl5005s.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl5005s_config {
// 7 bit i2c address
    pub i2c_address: u8,
pub const IF_FREQ_4570000HZ: c_int = 4570000;
pub const IF_FREQ_4571429HZ: c_int = 4571429;
pub const IF_FREQ_5380000HZ: c_int = 5380000;
pub const IF_FREQ_36000000HZ: c_int = 36000000;
pub const IF_FREQ_36125000HZ: c_int = 36125000;
pub const IF_FREQ_36166667HZ: c_int = 36166667;
pub const IF_FREQ_44000000HZ: c_int = 44000000;
    pub if_freq: u32,
pub const CRYSTAL_FREQ_4000000HZ: c_int = 4000000;
pub const CRYSTAL_FREQ_16000000HZ: c_int = 16000000;
pub const CRYSTAL_FREQ_25000000HZ: c_int = 25000000;
pub const CRYSTAL_FREQ_28800000HZ: c_int = 28800000;
    pub xtal_freq: u32,
pub const MXL_DUAL_AGC: c_int = 0;
pub const MXL_SINGLE_AGC: c_int = 1;
    pub agc_mode: u8,
pub const MXL_TF_DEFAULT: c_int = 0;
pub const MXL_TF_OFF: c_int = 1;
pub const MXL_TF_C: c_int = 2;
pub const MXL_TF_C_H: c_int = 3;
pub const MXL_TF_D: c_int = 4;
pub const MXL_TF_D_L: c_int = 5;
pub const MXL_TF_E: c_int = 6;
pub const MXL_TF_F: c_int = 7;
pub const MXL_TF_E_2: c_int = 8;
pub const MXL_TF_E_NA: c_int = 9;
pub const MXL_TF_G: c_int = 10;
    pub tracking_filter: u8,
pub const MXL_RSSI_DISABLE: c_int = 0;
pub const MXL_RSSI_ENABLE: c_int = 1;
    pub rssi_enable: u8,
pub const MXL_CAP_SEL_DISABLE: c_int = 0;
pub const MXL_CAP_SEL_ENABLE: c_int = 1;
    pub cap_select: u8,
pub const MXL_DIV_OUT_1: c_int = 0;
pub const MXL_DIV_OUT_4: c_int = 1;
    pub div_out: u8,
pub const MXL_CLOCK_OUT_DISABLE: c_int = 0;
pub const MXL_CLOCK_OUT_ENABLE: c_int = 1;
    pub clock_out: u8,
pub const MXL5005S_IF_OUTPUT_LOAD_200_OHM: c_int = 200;
pub const MXL5005S_IF_OUTPUT_LOAD_300_OHM: c_int = 300;
    pub output_load: u32,
pub const MXL5005S_TOP_5P5: c_int = 55;
pub const MXL5005S_TOP_7P2: c_int = 72;
pub const MXL5005S_TOP_9P2: c_int = 92;
pub const MXL5005S_TOP_11P0: c_int = 110;
pub const MXL5005S_TOP_12P9: c_int = 129;
pub const MXL5005S_TOP_14P7: c_int = 147;
pub const MXL5005S_TOP_16P8: c_int = 168;
pub const MXL5005S_TOP_19P4: c_int = 194;
pub const MXL5005S_TOP_21P2: c_int = 212;
pub const MXL5005S_TOP_23P2: c_int = 232;
pub const MXL5005S_TOP_25P2: c_int = 252;
pub const MXL5005S_TOP_27P1: c_int = 271;
pub const MXL5005S_TOP_29P2: c_int = 292;
pub const MXL5005S_TOP_31P7: c_int = 317;
pub const MXL5005S_TOP_34P9: c_int = 349;
    pub top: u32,
pub const MXL_ANALOG_MODE: c_int = 0;
pub const MXL_DIGITAL_MODE: c_int = 1;
    pub mod_mode: u8,
pub const MXL_ZERO_IF: c_int = 0;
pub const MXL_LOW_IF: c_int = 1;
    pub if_mode: u8,
// Some boards need to override the built-in logic for determining
    pub qam_gain: u8,
// Stuff I don't know what to do with
    pub AgcMasterByte: u8,
}

