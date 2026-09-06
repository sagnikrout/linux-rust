//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/wm8776.h
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

// Macro flag: #define WM8776_H_INCLUDED
//
// the following register names are from:
// wm8776.h  --  WM8776 ASoC driver
//
// Copyright 2009 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
pub const WM8776_HPLVOL: c_uint = 0x00;
pub const WM8776_HPRVOL: c_uint = 0x01;
pub const WM8776_HPMASTER: c_uint = 0x02;
pub const WM8776_DACLVOL: c_uint = 0x03;
pub const WM8776_DACRVOL: c_uint = 0x04;
pub const WM8776_DACMASTER: c_uint = 0x05;
pub const WM8776_PHASESWAP: c_uint = 0x06;
pub const WM8776_DACCTRL1: c_uint = 0x07;
pub const WM8776_DACMUTE: c_uint = 0x08;
pub const WM8776_DACCTRL2: c_uint = 0x09;
pub const WM8776_DACIFCTRL: c_uint = 0x0a;
pub const WM8776_ADCIFCTRL: c_uint = 0x0b;
pub const WM8776_MSTRCTRL: c_uint = 0x0c;
pub const WM8776_PWRDOWN: c_uint = 0x0d;
pub const WM8776_ADCLVOL: c_uint = 0x0e;
pub const WM8776_ADCRVOL: c_uint = 0x0f;
pub const WM8776_ALCCTRL1: c_uint = 0x10;
pub const WM8776_ALCCTRL2: c_uint = 0x11;
pub const WM8776_ALCCTRL3: c_uint = 0x12;
pub const WM8776_NOISEGATE: c_uint = 0x13;
pub const WM8776_LIMITER: c_uint = 0x14;
pub const WM8776_ADCMUX: c_uint = 0x15;
pub const WM8776_OUTMUX: c_uint = 0x16;
pub const WM8776_RESET: c_uint = 0x17;
// HPLVOL/HPRVOL/HPMASTER
pub const WM8776_HPATT_MASK: c_uint = 0x07f;
pub const WM8776_HPZCEN: c_uint = 0x080;
pub const WM8776_UPDATE: c_uint = 0x100;
// DACLVOL/DACRVOL/DACMASTER
pub const WM8776_DATT_MASK: c_uint = 0x0ff;
// #define WM8776_UPDATE		0x100
// PHASESWAP
pub const WM8776_PH_MASK: c_uint = 0x003;
// DACCTRL1
pub const WM8776_DZCEN: c_uint = 0x001;
pub const WM8776_ATC: c_uint = 0x002;
pub const WM8776_IZD: c_uint = 0x004;
pub const WM8776_TOD: c_uint = 0x008;
pub const WM8776_PL_LEFT_MASK: c_uint = 0x030;
pub const WM8776_PL_LEFT_MUTE: c_uint = 0x000;
pub const WM8776_PL_LEFT_LEFT: c_uint = 0x010;
pub const WM8776_PL_LEFT_RIGHT: c_uint = 0x020;
pub const WM8776_PL_LEFT_LRMIX: c_uint = 0x030;
pub const WM8776_PL_RIGHT_MASK: c_uint = 0x0c0;
pub const WM8776_PL_RIGHT_MUTE: c_uint = 0x000;
pub const WM8776_PL_RIGHT_LEFT: c_uint = 0x040;
pub const WM8776_PL_RIGHT_RIGHT: c_uint = 0x080;
pub const WM8776_PL_RIGHT_LRMIX: c_uint = 0x0c0;
// DACMUTE
pub const WM8776_DMUTE: c_uint = 0x001;
// DACCTRL2
pub const WM8776_DEEMPH: c_uint = 0x001;
pub const WM8776_DZFM_MASK: c_uint = 0x006;
pub const WM8776_DZFM_NONE: c_uint = 0x000;
pub const WM8776_DZFM_LR: c_uint = 0x002;
pub const WM8776_DZFM_BOTH: c_uint = 0x004;
pub const WM8776_DZFM_EITHER: c_uint = 0x006;
// DACIFCTRL
pub const WM8776_DACFMT_MASK: c_uint = 0x003;
pub const WM8776_DACFMT_RJUST: c_uint = 0x000;
pub const WM8776_DACFMT_LJUST: c_uint = 0x001;
pub const WM8776_DACFMT_I2S: c_uint = 0x002;
pub const WM8776_DACFMT_DSP: c_uint = 0x003;
pub const WM8776_DACLRP: c_uint = 0x004;
pub const WM8776_DACBCP: c_uint = 0x008;
pub const WM8776_DACWL_MASK: c_uint = 0x030;
pub const WM8776_DACWL_16: c_uint = 0x000;
pub const WM8776_DACWL_20: c_uint = 0x010;
pub const WM8776_DACWL_24: c_uint = 0x020;
pub const WM8776_DACWL_32: c_uint = 0x030;
// ADCIFCTRL
pub const WM8776_ADCFMT_MASK: c_uint = 0x003;
pub const WM8776_ADCFMT_RJUST: c_uint = 0x000;
pub const WM8776_ADCFMT_LJUST: c_uint = 0x001;
pub const WM8776_ADCFMT_I2S: c_uint = 0x002;
pub const WM8776_ADCFMT_DSP: c_uint = 0x003;
pub const WM8776_ADCLRP: c_uint = 0x004;
pub const WM8776_ADCBCP: c_uint = 0x008;
pub const WM8776_ADCWL_MASK: c_uint = 0x030;
pub const WM8776_ADCWL_16: c_uint = 0x000;
pub const WM8776_ADCWL_20: c_uint = 0x010;
pub const WM8776_ADCWL_24: c_uint = 0x020;
pub const WM8776_ADCWL_32: c_uint = 0x030;
pub const WM8776_ADCMCLK: c_uint = 0x040;
pub const WM8776_ADCHPD: c_uint = 0x100;
// MSTRCTRL
pub const WM8776_ADCRATE_MASK: c_uint = 0x007;
pub const WM8776_ADCRATE_256: c_uint = 0x002;
pub const WM8776_ADCRATE_384: c_uint = 0x003;
pub const WM8776_ADCRATE_512: c_uint = 0x004;
pub const WM8776_ADCRATE_768: c_uint = 0x005;
pub const WM8776_ADCOSR: c_uint = 0x008;
pub const WM8776_DACRATE_MASK: c_uint = 0x070;
pub const WM8776_DACRATE_128: c_uint = 0x000;
pub const WM8776_DACRATE_192: c_uint = 0x010;
pub const WM8776_DACRATE_256: c_uint = 0x020;
pub const WM8776_DACRATE_384: c_uint = 0x030;
pub const WM8776_DACRATE_512: c_uint = 0x040;
pub const WM8776_DACRATE_768: c_uint = 0x050;
pub const WM8776_DACMS: c_uint = 0x080;
pub const WM8776_ADCMS: c_uint = 0x100;
// PWRDOWN
pub const WM8776_PDWN: c_uint = 0x001;
pub const WM8776_ADCPD: c_uint = 0x002;
pub const WM8776_DACPD: c_uint = 0x004;
pub const WM8776_HPPD: c_uint = 0x008;
pub const WM8776_AINPD: c_uint = 0x040;
// ADCLVOL/ADCRVOL
pub const WM8776_AGMASK: c_uint = 0x0ff;
pub const WM8776_ZCA: c_uint = 0x100;
// ALCCTRL1
pub const WM8776_LCT_MASK: c_uint = 0x00f;
pub const WM8776_MAXGAIN_MASK: c_uint = 0x070;
pub const WM8776_LCSEL_MASK: c_uint = 0x180;
pub const WM8776_LCSEL_LIMITER: c_uint = 0x000;
pub const WM8776_LCSEL_ALC_RIGHT: c_uint = 0x080;
pub const WM8776_LCSEL_ALC_LEFT: c_uint = 0x100;
pub const WM8776_LCSEL_ALC_STEREO: c_uint = 0x180;
// ALCCTRL2
pub const WM8776_HLD_MASK: c_uint = 0x00f;
pub const WM8776_ALCZC: c_uint = 0x080;
pub const WM8776_LCEN: c_uint = 0x100;
// ALCCTRL3
pub const WM8776_ATK_MASK: c_uint = 0x00f;
pub const WM8776_DCY_MASK: c_uint = 0x0f0;
// NOISEGATE
pub const WM8776_NGAT: c_uint = 0x001;
pub const WM8776_NGTH_MASK: c_uint = 0x01c;
// LIMITER
pub const WM8776_MAXATTEN_MASK: c_uint = 0x00f;
pub const WM8776_TRANWIN_MASK: c_uint = 0x070;
// ADCMUX
pub const WM8776_AMX_MASK: c_uint = 0x01f;
pub const WM8776_MUTERA: c_uint = 0x040;
pub const WM8776_MUTELA: c_uint = 0x080;
pub const WM8776_LRBOTH: c_uint = 0x100;
// OUTMUX
pub const WM8776_MX_DAC: c_uint = 0x001;
pub const WM8776_MX_AUX: c_uint = 0x002;
pub const WM8776_MX_BYPASS: c_uint = 0x004;
