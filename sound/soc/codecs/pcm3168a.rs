//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/pcm3168a.h
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
// PCM3168A codec driver header
//
// Copyright (C) 2015 Imagination Technologies Ltd.
//
// Author: Damien Horsley <Damien.Horsley@imgtec.com>
//
extern "C" {
    pub fn pcm3168a_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
}
extern "C" {
    pub fn pcm3168a_remove(dev: *mut device);
}
pub const PCM3168A_RST_SMODE: c_uint = 0x40;
pub const PCM3168A_MRST_MASK: c_uint = 0x80;
pub const PCM3168A_SRST_MASK: c_uint = 0x40;
pub const PCM3168A_DAC_SRDA_SHIFT: c_int = 0;
pub const PCM3168A_DAC_SRDA_MASK: c_uint = 0x3;
pub const PCM3168A_DAC_PWR_MST_FMT: c_uint = 0x41;
pub const PCM3168A_DAC_PSMDA_SHIFT: c_int = 7;
pub const PCM3168A_DAC_PSMDA_MASK: c_uint = 0x80;
pub const PCM3168A_DAC_MSDA_SHIFT: c_int = 4;
pub const PCM3168A_DAC_MSDA_MASK: c_uint = 0x70;
pub const PCM3168A_DAC_FMT_SHIFT: c_int = 0;
pub const PCM3168A_DAC_FMT_MASK: c_uint = 0xf;
pub const PCM3168A_DAC_OP_FLT: c_uint = 0x42;
pub const PCM3168A_DAC_OPEDA_SHIFT: c_int = 4;
pub const PCM3168A_DAC_OPEDA_MASK: c_uint = 0xf0;
pub const PCM3168A_DAC_FLT_SHIFT: c_int = 0;
pub const PCM3168A_DAC_FLT_MASK: c_uint = 0xf;
pub const PCM3168A_DAC_INV: c_uint = 0x43;
pub const PCM3168A_DAC_MUTE: c_uint = 0x44;
pub const PCM3168A_DAC_ZERO: c_uint = 0x45;
pub const PCM3168A_DAC_ATT_DEMP_ZF: c_uint = 0x46;
pub const PCM3168A_DAC_ATMDDA_MASK: c_uint = 0x80;
pub const PCM3168A_DAC_ATMDDA_SHIFT: c_int = 7;
pub const PCM3168A_DAC_ATSPDA_MASK: c_uint = 0x40;
pub const PCM3168A_DAC_ATSPDA_SHIFT: c_int = 6;
pub const PCM3168A_DAC_DEMP_SHIFT: c_int = 4;
pub const PCM3168A_DAC_DEMP_MASK: c_uint = 0x30;
pub const PCM3168A_DAC_AZRO_SHIFT: c_int = 1;
pub const PCM3168A_DAC_AZRO_MASK: c_uint = 0xe;
pub const PCM3168A_DAC_ZREV_MASK: c_uint = 0x1;
pub const PCM3168A_DAC_ZREV_SHIFT: c_int = 0;
pub const PCM3168A_DAC_VOL_MASTER: c_uint = 0x47;
pub const PCM3168A_DAC_VOL_CHAN_START: c_uint = 0x48;
pub const PCM3168A_ADC_SMODE: c_uint = 0x50;
pub const PCM3168A_ADC_SRAD_SHIFT: c_int = 0;
pub const PCM3168A_ADC_SRAD_MASK: c_uint = 0x3;
pub const PCM3168A_ADC_MST_FMT: c_uint = 0x51;
pub const PCM3168A_ADC_MSAD_SHIFT: c_int = 4;
pub const PCM3168A_ADC_MSAD_MASK: c_uint = 0x70;
pub const PCM3168A_ADC_FMTAD_SHIFT: c_int = 0;
pub const PCM3168A_ADC_FMTAD_MASK: c_uint = 0x7;
pub const PCM3168A_ADC_PWR_HPFB: c_uint = 0x52;
pub const PCM3168A_ADC_PSVAD_SHIFT: c_int = 4;
pub const PCM3168A_ADC_PSVAD_MASK: c_uint = 0x70;
pub const PCM3168A_ADC_BYP_SHIFT: c_int = 0;
pub const PCM3168A_ADC_BYP_MASK: c_uint = 0x7;
pub const PCM3168A_ADC_SEAD: c_uint = 0x53;
pub const PCM3168A_ADC_INV: c_uint = 0x54;
pub const PCM3168A_ADC_MUTE: c_uint = 0x55;
pub const PCM3168A_ADC_OV: c_uint = 0x56;
pub const PCM3168A_ADC_ATT_OVF: c_uint = 0x57;
pub const PCM3168A_ADC_ATMDAD_MASK: c_uint = 0x80;
pub const PCM3168A_ADC_ATMDAD_SHIFT: c_int = 7;
pub const PCM3168A_ADC_ATSPAD_MASK: c_uint = 0x40;
pub const PCM3168A_ADC_ATSPAD_SHIFT: c_int = 6;
pub const PCM3168A_ADC_OVFP_MASK: c_uint = 0x1;
pub const PCM3168A_ADC_OVFP_SHIFT: c_int = 0;
pub const PCM3168A_ADC_VOL_MASTER: c_uint = 0x58;
pub const PCM3168A_ADC_VOL_CHAN_START: c_uint = 0x59;
