//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/abx500/ab8500-codec.h
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
// Copyright (C) ST-Ericsson SA 2012
//
// Author: Ola Lilja <ola.o.lilja@stericsson.com>
// for ST-Ericsson.
//
// License terms:
//
// Mic-types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amic_type {
    AMIC_TYPE_SINGLE_ENDED,
    AMIC_TYPE_DIFFERENTIAL
}

// Mic-biases
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amic_micbias {
    AMIC_MICBIAS_VAMIC1,
    AMIC_MICBIAS_VAMIC2,
    AMIC_MICBIAS_UNKNOWN
}

// Bias-voltage
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ear_cm_voltage {
    EAR_CMV_0_95V,
    EAR_CMV_1_10V,
    EAR_CMV_1_27V,
    EAR_CMV_1_58V,
    EAR_CMV_UNKNOWN
}

// Analog microphone settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amic_settings {
    pub mic1_type: amic_type,
    pub mic2_type: amic_type,
    pub mic1a_micbias: amic_micbias,
    pub mic1b_micbias: amic_micbias,
    pub mic2_micbias: amic_micbias,
}

// Platform data structure for the audio-parts of the AB8500
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ab8500_codec_platform_data {
    pub amics: amic_settings,
    pub ear_cmv: ear_cm_voltage,
}
