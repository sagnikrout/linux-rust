//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/starfive-jh7100-audio.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (C) 2021 Emil Renner Berthing <kernel@esmil.dk>
//
pub const JH7100_AUDCLK_ADC_MCLK: c_int = 0;
pub const JH7100_AUDCLK_I2S1_MCLK: c_int = 1;
pub const JH7100_AUDCLK_I2SADC_APB: c_int = 2;
pub const JH7100_AUDCLK_I2SADC_BCLK: c_int = 3;
pub const JH7100_AUDCLK_I2SADC_BCLK_N: c_int = 4;
pub const JH7100_AUDCLK_I2SADC_LRCLK: c_int = 5;
pub const JH7100_AUDCLK_PDM_APB: c_int = 6;
pub const JH7100_AUDCLK_PDM_MCLK: c_int = 7;
pub const JH7100_AUDCLK_I2SVAD_APB: c_int = 8;
pub const JH7100_AUDCLK_SPDIF: c_int = 9;
pub const JH7100_AUDCLK_SPDIF_APB: c_int = 10;
pub const JH7100_AUDCLK_PWMDAC_APB: c_int = 11;
pub const JH7100_AUDCLK_DAC_MCLK: c_int = 12;
pub const JH7100_AUDCLK_I2SDAC_APB: c_int = 13;
pub const JH7100_AUDCLK_I2SDAC_BCLK: c_int = 14;
pub const JH7100_AUDCLK_I2SDAC_BCLK_N: c_int = 15;
pub const JH7100_AUDCLK_I2SDAC_LRCLK: c_int = 16;
pub const JH7100_AUDCLK_I2S1_APB: c_int = 17;
pub const JH7100_AUDCLK_I2S1_BCLK: c_int = 18;
pub const JH7100_AUDCLK_I2S1_BCLK_N: c_int = 19;
pub const JH7100_AUDCLK_I2S1_LRCLK: c_int = 20;
pub const JH7100_AUDCLK_I2SDAC16K_APB: c_int = 21;
pub const JH7100_AUDCLK_APB0_BUS: c_int = 22;
pub const JH7100_AUDCLK_DMA1P_AHB: c_int = 23;
pub const JH7100_AUDCLK_USB_APB: c_int = 24;
pub const JH7100_AUDCLK_USB_LPM: c_int = 25;
pub const JH7100_AUDCLK_USB_STB: c_int = 26;
pub const JH7100_AUDCLK_APB_EN: c_int = 27;
pub const JH7100_AUDCLK_VAD_MEM: c_int = 28;
pub const JH7100_AUDCLK_END: c_int = 29;
