//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tlv320aic23.h
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
// ALSA SoC TLV320AIC23 codec driver
//
// Author:      Arun KS, <arunks@mistralsolutions.com>
// Copyright:   (C) 2008 Mistral Solutions Pvt Ltd
//
extern "C" {
    pub fn tlv320aic23_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
}
// Codec TLV320AIC23
pub const TLV320AIC23_LINVOL: c_uint = 0x00;
pub const TLV320AIC23_RINVOL: c_uint = 0x01;
pub const TLV320AIC23_LCHNVOL: c_uint = 0x02;
pub const TLV320AIC23_RCHNVOL: c_uint = 0x03;
pub const TLV320AIC23_ANLG: c_uint = 0x04;
pub const TLV320AIC23_DIGT: c_uint = 0x05;
pub const TLV320AIC23_PWR: c_uint = 0x06;
pub const TLV320AIC23_DIGT_FMT: c_uint = 0x07;
pub const TLV320AIC23_SRATE: c_uint = 0x08;
pub const TLV320AIC23_ACTIVE: c_uint = 0x09;
pub const TLV320AIC23_RESET: c_uint = 0x0F;
// Left (right) line input volume control register
pub const TLV320AIC23_LRS_ENABLED: c_uint = 0x0100;
pub const TLV320AIC23_LIM_MUTED: c_uint = 0x0080;
pub const TLV320AIC23_LIV_DEFAULT: c_uint = 0x0017;
pub const TLV320AIC23_LIV_MAX: c_uint = 0x001f;
pub const TLV320AIC23_LIV_MIN: c_uint = 0x0000;
// Left (right) channel headphone volume control register
pub const TLV320AIC23_LZC_ON: c_uint = 0x0080;
pub const TLV320AIC23_LHV_DEFAULT: c_uint = 0x0079;
pub const TLV320AIC23_LHV_MAX: c_uint = 0x007f;
pub const TLV320AIC23_LHV_MIN: c_uint = 0x0000;
// Analog audio path control register

pub const TLV320AIC23_STE_ENABLED: c_uint = 0x0020;
pub const TLV320AIC23_DAC_SELECTED: c_uint = 0x0010;
pub const TLV320AIC23_BYPASS_ON: c_uint = 0x0008;
pub const TLV320AIC23_INSEL_MIC: c_uint = 0x0004;
pub const TLV320AIC23_MICM_MUTED: c_uint = 0x0002;
pub const TLV320AIC23_MICB_20DB: c_uint = 0x0001;
// Digital audio path control register
pub const TLV320AIC23_DACM_MUTE: c_uint = 0x0008;
pub const TLV320AIC23_DEEMP_32K: c_uint = 0x0002;
pub const TLV320AIC23_DEEMP_44K: c_uint = 0x0004;
pub const TLV320AIC23_DEEMP_48K: c_uint = 0x0006;
pub const TLV320AIC23_ADCHP_ON: c_uint = 0x0001;
// Power control down register
pub const TLV320AIC23_DEVICE_PWR_OFF: c_uint = 0x0080;
pub const TLV320AIC23_CLK_OFF: c_uint = 0x0040;
pub const TLV320AIC23_OSC_OFF: c_uint = 0x0020;
pub const TLV320AIC23_OUT_OFF: c_uint = 0x0010;
pub const TLV320AIC23_DAC_OFF: c_uint = 0x0008;
pub const TLV320AIC23_ADC_OFF: c_uint = 0x0004;
pub const TLV320AIC23_MIC_OFF: c_uint = 0x0002;
pub const TLV320AIC23_LINE_OFF: c_uint = 0x0001;
// Digital audio interface register
pub const TLV320AIC23_MS_MASTER: c_uint = 0x0040;
pub const TLV320AIC23_LRSWAP_ON: c_uint = 0x0020;
pub const TLV320AIC23_LRP_ON: c_uint = 0x0010;
pub const TLV320AIC23_IWL_16: c_uint = 0x0000;
pub const TLV320AIC23_IWL_20: c_uint = 0x0004;
pub const TLV320AIC23_IWL_24: c_uint = 0x0008;
pub const TLV320AIC23_IWL_32: c_uint = 0x000C;
pub const TLV320AIC23_FOR_I2S: c_uint = 0x0002;
pub const TLV320AIC23_FOR_DSP: c_uint = 0x0003;
pub const TLV320AIC23_FOR_LJUST: c_uint = 0x0001;
// Sample rate control register
pub const TLV320AIC23_CLKOUT_HALF: c_uint = 0x0080;
pub const TLV320AIC23_CLKIN_HALF: c_uint = 0x0040;
pub const TLV320AIC23_BOSR_384fs: c_uint = 0x0002	/* BOSR_272fs in USB mode */;
pub const TLV320AIC23_USB_CLK_ON: c_uint = 0x0001;
pub const TLV320AIC23_SR_MASK: c_uint = 0xf;
pub const TLV320AIC23_CLKOUT_SHIFT: c_int = 7;
pub const TLV320AIC23_CLKIN_SHIFT: c_int = 6;
pub const TLV320AIC23_SR_SHIFT: c_int = 2;
pub const TLV320AIC23_BOSR_SHIFT: c_int = 1;
// Digital interface register
pub const TLV320AIC23_ACT_ON: c_uint = 0x0001;
//
// AUDIO related MACROS
//
pub const TLV320AIC23_DEFAULT_OUT_VOL: c_uint = 0x70;
pub const TLV320AIC23_DEFAULT_IN_VOLUME: c_uint = 0x10;

pub const TLV320AIC23_SIDETONE_MASK: c_uint = 0x1c0;
pub const TLV320AIC23_SIDETONE_0: c_uint = 0x100;
pub const TLV320AIC23_SIDETONE_6: c_uint = 0x000;
pub const TLV320AIC23_SIDETONE_9: c_uint = 0x040;
pub const TLV320AIC23_SIDETONE_12: c_uint = 0x080;
pub const TLV320AIC23_SIDETONE_18: c_uint = 0x0c0;
