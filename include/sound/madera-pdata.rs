//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/madera-pdata.h
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
// Platform data for Madera codec driver
//
// Copyright (C) 2016-2019 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

pub const MADERA_MAX_INPUT: c_int = 6;
pub const MADERA_MAX_MUXED_CHANNELS: c_int = 4;
pub const MADERA_MAX_OUTPUT: c_int = 6;
pub const MADERA_MAX_AIF: c_int = 4;
pub const MADERA_MAX_PDM_SPK: c_int = 2;
pub const MADERA_MAX_DSP: c_int = 7;
//
// struct madera_codec_pdata
//
// @max_channels_clocked: Maximum number of channels that I2S clocks will be
// generated for. Useful when clock master for systems
// where the I2S bus has multiple data lines.
// @dmic_ref:		  Indicates how the MICBIAS pins have been externally
// connected to DMICs on each input. A value of 0
// indicates MICVDD and is the default. Other values are:
// For CS47L35 one of the CS47L35_DMIC_REF_xxx values
// For all other codecs one of the MADERA_DMIC_REF_xxx
// Also see the datasheet for a description of the
// INn_DMIC_SUP field.
// @inmode:		  Mode for the ADC inputs. One of the MADERA_INMODE_xxx
// values. Two-dimensional array
// [input_number][channel number], with four slots per
// input in the order
// [n][0]=INnAL [n][1]=INnAR [n][2]=INnBL [n][3]=INnBR
// @out_mono:		  For each output set the value to TRUE to indicate that
// the output is mono. [0]=OUT1, [1]=OUT2, ...
// @pdm_fmt:		  PDM speaker data format. See the PDM_SPKn_FMT field in
// the datasheet for a description of this value.
// @pdm_mute:		  PDM mute format. See the PDM_SPKn_CTRL_1 register
// in the datasheet for a description of this value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera_codec_pdata {
    pub max_channels_clocked: [u32; MADERA_MAX_AIF],
    pub dmic_ref: [u32; MADERA_MAX_INPUT],
    pub inmode: [u32; MADERA_MAX_INPUT][MADERA_MAX_MUXED_CHANNELS],
    pub out_mono: [bool; MADERA_MAX_OUTPUT],
    pub pdm_fmt: [u32; MADERA_MAX_PDM_SPK],
    pub pdm_mute: [u32; MADERA_MAX_PDM_SPK],
}
