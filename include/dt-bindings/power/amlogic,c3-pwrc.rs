//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/amlogic,c3-pwrc.h
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
// Copyright (c) 2023 Amlogic, Inc.
// Author: hongyu chen1 <hongyu.chen1@amlogic.com>
//
pub const PWRC_C3_NNA_ID: c_int = 0;
pub const PWRC_C3_AUDIO_ID: c_int = 1;
pub const PWRC_C3_RESV_SEC_ID: c_int = 2;
pub const PWRC_C3_SDIOA_ID: c_int = 3;
pub const PWRC_C3_EMMC_ID: c_int = 4;
pub const PWRC_C3_USB_COMB_ID: c_int = 5;
pub const PWRC_C3_SDCARD_ID: c_int = 6;
pub const PWRC_C3_ETH_ID: c_int = 7;
pub const PWRC_C3_RESV0_ID: c_int = 8;
pub const PWRC_C3_GE2D_ID: c_int = 9;
pub const PWRC_C3_CVE_ID: c_int = 10;
pub const PWRC_C3_GDC_WRAP_ID: c_int = 11;
pub const PWRC_C3_ISP_TOP_ID: c_int = 12;
pub const PWRC_C3_MIPI_ISP_WRAP_ID: c_int = 13;
pub const PWRC_C3_VCODEC_ID: c_int = 14;
