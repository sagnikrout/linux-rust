//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/meson-a1-power.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
// Copyright (c) 2019 Amlogic, Inc.
// Author: Jianxin Pan <jianxin.pan@amlogic.com>
//
pub const PWRC_DSPA_ID: c_int = 8;
pub const PWRC_DSPB_ID: c_int = 9;
pub const PWRC_UART_ID: c_int = 10;
pub const PWRC_DMC_ID: c_int = 11;
pub const PWRC_I2C_ID: c_int = 12;
pub const PWRC_PSRAM_ID: c_int = 13;
pub const PWRC_ACODEC_ID: c_int = 14;
pub const PWRC_AUDIO_ID: c_int = 15;
pub const PWRC_OTP_ID: c_int = 16;
pub const PWRC_DMA_ID: c_int = 17;
pub const PWRC_SD_EMMC_ID: c_int = 18;
pub const PWRC_RAMA_ID: c_int = 19;
pub const PWRC_RAMB_ID: c_int = 20;
pub const PWRC_IR_ID: c_int = 21;
pub const PWRC_SPICC_ID: c_int = 22;
pub const PWRC_SPIFC_ID: c_int = 23;
pub const PWRC_USB_ID: c_int = 24;
pub const PWRC_NIC_ID: c_int = 25;
pub const PWRC_PDMIN_ID: c_int = 26;
pub const PWRC_RSA_ID: c_int = 27;
pub const PWRC_MAX_ID: c_int = 28;
