//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,shikra-audiocorecc.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// AUDIO_CORE_CC clocks
pub const AUDIO_CORE_CC_DIG_PLL_OUT_AUX: c_int = 0;
pub const AUDIO_CORE_CC_DIG_PLL_OUT_AUX2: c_int = 1;
pub const AUDIO_CORE_CC_DIG_PLL: c_int = 2;
pub const AUDIO_CORE_CC_AIF_IF0_CLK_SRC: c_int = 3;
pub const AUDIO_CORE_CC_AIF_IF0_EBIT_CLK: c_int = 4;
pub const AUDIO_CORE_CC_AIF_IF0_IBIT_CLK: c_int = 5;
pub const AUDIO_CORE_CC_AIF_IF1_CLK_SRC: c_int = 6;
pub const AUDIO_CORE_CC_AIF_IF1_EBIT_CLK: c_int = 7;
pub const AUDIO_CORE_CC_AIF_IF1_IBIT_CLK: c_int = 8;
pub const AUDIO_CORE_CC_AIF_IF2_CLK_SRC: c_int = 9;
pub const AUDIO_CORE_CC_AIF_IF2_EBIT_CLK: c_int = 10;
pub const AUDIO_CORE_CC_AIF_IF2_IBIT_CLK: c_int = 11;
pub const AUDIO_CORE_CC_AIF_IF3_CLK_SRC: c_int = 12;
pub const AUDIO_CORE_CC_AIF_IF3_EBIT_CLK: c_int = 13;
pub const AUDIO_CORE_CC_AIF_IF3_IBIT_CLK: c_int = 14;
pub const AUDIO_CORE_CC_AUD_DMA_CLK: c_int = 15;
pub const AUDIO_CORE_CC_AUD_DMA_CLK_SRC: c_int = 16;
pub const AUDIO_CORE_CC_AUD_DMA_MEM_CLK: c_int = 17;
pub const AUDIO_CORE_CC_BUS_CLK: c_int = 18;
pub const AUDIO_CORE_CC_BUS_CLK_SRC: c_int = 19;
pub const AUDIO_CORE_CC_CDIV_TX_MCLK_DIV_CLK_SRC: c_int = 20;
pub const AUDIO_CORE_CC_EXT_MCLKA_CLK_SRC: c_int = 21;
pub const AUDIO_CORE_CC_EXT_MCLKA_OUT_CLK: c_int = 22;
pub const AUDIO_CORE_CC_EXT_MCLKB_CLK_SRC: c_int = 23;
pub const AUDIO_CORE_CC_EXT_MCLKB_OUT_CLK: c_int = 24;
pub const AUDIO_CORE_CC_IM_SLEEP_CLK: c_int = 25;
pub const AUDIO_CORE_CC_LPAIF_PCMOE_CLK: c_int = 26;
pub const AUDIO_CORE_CC_LPAIF_PCMOE_CLK_SRC: c_int = 27;
pub const AUDIO_CORE_CC_RX_MCLK_2X_CLK: c_int = 28;
pub const AUDIO_CORE_CC_RX_MCLK_CLK: c_int = 29;
pub const AUDIO_CORE_CC_SAMPLING_CLK: c_int = 30;
pub const AUDIO_CORE_CC_TX_MCLK_2X_CLK: c_int = 31;
pub const AUDIO_CORE_CC_TX_MCLK_CLK: c_int = 32;
pub const AUDIO_CORE_CC_TX_MCLK_RCG_CLK_SRC: c_int = 33;
// AUDIO_CORE_CSR resets
pub const AUDIO_CORE_CSR_RX_SWR_CGCR: c_int = 0;
pub const AUDIO_CORE_CSR_TX_SWR_CGCR: c_int = 1;
