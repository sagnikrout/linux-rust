//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,lpasscorecc-sc7180.h
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
// Copyright (c) 2020, The Linux Foundation. All rights reserved.
//
// LPASS_CORE_CC clocks
pub const LPASS_LPAAUDIO_DIG_PLL: c_int = 0;
pub const LPASS_LPAAUDIO_DIG_PLL_OUT_ODD: c_int = 1;
pub const CORE_CLK_SRC: c_int = 2;
pub const EXT_MCLK0_CLK_SRC: c_int = 3;
pub const LPAIF_PRI_CLK_SRC: c_int = 4;
pub const LPAIF_SEC_CLK_SRC: c_int = 5;
pub const LPASS_AUDIO_CORE_CORE_CLK: c_int = 6;
pub const LPASS_AUDIO_CORE_EXT_MCLK0_CLK: c_int = 7;
pub const LPASS_AUDIO_CORE_LPAIF_PRI_IBIT_CLK: c_int = 8;
pub const LPASS_AUDIO_CORE_LPAIF_SEC_IBIT_CLK: c_int = 9;
pub const LPASS_AUDIO_CORE_SYSNOC_MPORT_CORE_CLK: c_int = 10;
// LPASS Core power domains
pub const LPASS_CORE_HM_GDSCR: c_int = 0;
// LPASS Audio power domains
pub const LPASS_AUDIO_HM_GDSCR: c_int = 0;
pub const LPASS_PDC_HM_GDSCR: c_int = 1;
