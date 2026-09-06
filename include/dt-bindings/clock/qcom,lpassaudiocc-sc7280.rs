//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,lpassaudiocc-sc7280.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (c) 2021, The Linux Foundation. All rights reserved.
//
// LPASS_AUDIO_CC clocks
pub const LPASS_AUDIO_CC_PLL: c_int = 0;
pub const LPASS_AUDIO_CC_PLL_OUT_AUX2: c_int = 1;
pub const LPASS_AUDIO_CC_PLL_OUT_AUX2_DIV_CLK_SRC: c_int = 2;
pub const LPASS_AUDIO_CC_PLL_OUT_MAIN_DIV_CLK_SRC: c_int = 3;
pub const LPASS_AUDIO_CC_CDIV_RX_MCLK_DIV_CLK_SRC: c_int = 4;
pub const LPASS_AUDIO_CC_CODEC_MEM0_CLK: c_int = 5;
pub const LPASS_AUDIO_CC_CODEC_MEM1_CLK: c_int = 6;
pub const LPASS_AUDIO_CC_CODEC_MEM2_CLK: c_int = 7;
pub const LPASS_AUDIO_CC_CODEC_MEM_CLK: c_int = 8;
pub const LPASS_AUDIO_CC_EXT_MCLK0_CLK: c_int = 9;
pub const LPASS_AUDIO_CC_EXT_MCLK0_CLK_SRC: c_int = 10;
pub const LPASS_AUDIO_CC_EXT_MCLK1_CLK: c_int = 11;
pub const LPASS_AUDIO_CC_EXT_MCLK1_CLK_SRC: c_int = 12;
pub const LPASS_AUDIO_CC_RX_MCLK_2X_CLK: c_int = 13;
pub const LPASS_AUDIO_CC_RX_MCLK_CLK: c_int = 14;
pub const LPASS_AUDIO_CC_RX_MCLK_CLK_SRC: c_int = 15;
// LPASS AUDIO CC CSR
pub const LPASS_AUDIO_SWR_RX_CGCR: c_int = 0;
pub const LPASS_AUDIO_SWR_TX_CGCR: c_int = 1;
pub const LPASS_AUDIO_SWR_WSA_CGCR: c_int = 2;
// LPASS_AON_CC clocks
pub const LPASS_AON_CC_PLL: c_int = 0;
pub const LPASS_AON_CC_PLL_OUT_EVEN: c_int = 1;
pub const LPASS_AON_CC_PLL_OUT_MAIN_CDIV_DIV_CLK_SRC: c_int = 2;
pub const LPASS_AON_CC_PLL_OUT_ODD: c_int = 3;
pub const LPASS_AON_CC_AUDIO_HM_H_CLK: c_int = 4;
pub const LPASS_AON_CC_CDIV_TX_MCLK_DIV_CLK_SRC: c_int = 5;
pub const LPASS_AON_CC_MAIN_RCG_CLK_SRC: c_int = 6;
pub const LPASS_AON_CC_TX_MCLK_2X_CLK: c_int = 7;
pub const LPASS_AON_CC_TX_MCLK_CLK: c_int = 8;
pub const LPASS_AON_CC_TX_MCLK_RCG_CLK_SRC: c_int = 9;
pub const LPASS_AON_CC_VA_MEM0_CLK: c_int = 10;
// LPASS_AON_CC power domains
pub const LPASS_AON_CC_LPASS_AUDIO_HM_GDSC: c_int = 0;
