//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/da7213.h
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
// da7213.h - DA7213 ASoC Codec Driver Platform Data
//
// Copyright (c) 2013 Dialog Semiconductor
//
// Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7213_micbias_voltage {
    DA7213_MICBIAS_1_6V = 0,
    DA7213_MICBIAS_2_2V = 1,
    DA7213_MICBIAS_2_5V = 2,
    DA7213_MICBIAS_3_0V = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7213_dmic_data_sel {
    DA7213_DMIC_DATA_LRISE_RFALL = 0,
    DA7213_DMIC_DATA_LFALL_RRISE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7213_dmic_samplephase {
    DA7213_DMIC_SAMPLE_ON_CLKEDGE = 0,
    DA7213_DMIC_SAMPLE_BETWEEN_CLKEDGE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7213_dmic_clk_rate {
    DA7213_DMIC_CLK_3_0MHZ = 0,
    DA7213_DMIC_CLK_1_5MHZ = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da7213_platform_data {
// Mic Bias voltage
    pub micbias1_lvl: da7213_micbias_voltage,
    pub micbias2_lvl: da7213_micbias_voltage,
// DMIC config
    pub dmic_data_sel: da7213_dmic_data_sel,
    pub dmic_samplephase: da7213_dmic_samplephase,
    pub dmic_clk_rate: da7213_dmic_clk_rate,
}
