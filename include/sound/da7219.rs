//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/da7219.h
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
// da7219.h - DA7219 ASoC Codec Driver Platform Data
//
// Copyright (c) 2015 Dialog Semiconductor
//
// Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
//
// Mic Bias
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_micbias_voltage {
    DA7219_MICBIAS_1_6V = 0,
    DA7219_MICBIAS_1_8V,
    DA7219_MICBIAS_2_0V,
    DA7219_MICBIAS_2_2V,
    DA7219_MICBIAS_2_4V,
    DA7219_MICBIAS_2_6V,
}

// Mic input type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_mic_amp_in_sel {
    DA7219_MIC_AMP_IN_SEL_DIFF = 0,
    DA7219_MIC_AMP_IN_SEL_SE_P,
    DA7219_MIC_AMP_IN_SEL_SE_N,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_dai_clks {
    DA7219_DAI_WCLK_IDX = 0,
    DA7219_DAI_BCLK_IDX,
    DA7219_DAI_NUM_CLKS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da7219_pdata {
    pub wakeup_source: bool,
    pub dai_clk_names: [*const c_char; DA7219_DAI_NUM_CLKS],
// Mic
    pub micbias_lvl: da7219_micbias_voltage,
    pub mic_amp_in_sel: da7219_mic_amp_in_sel,
// AAD
    pub aad_pdata: *mut da7219_aad_pdata,
}
