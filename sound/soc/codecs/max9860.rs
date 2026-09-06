//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max9860.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Driver for the MAX9860 Mono Audio Voice Codec
//
// Author: Peter Rosin <peda@axentia.s>
// Copyright 2016 Axentia Technologies
//
pub const MAX9860_INTRSTATUS: c_uint = 0x00;
pub const MAX9860_MICREADBACK: c_uint = 0x01;
pub const MAX9860_INTEN: c_uint = 0x02;
pub const MAX9860_SYSCLK: c_uint = 0x03;
pub const MAX9860_AUDIOCLKHIGH: c_uint = 0x04;
pub const MAX9860_AUDIOCLKLOW: c_uint = 0x05;
pub const MAX9860_IFC1A: c_uint = 0x06;
pub const MAX9860_IFC1B: c_uint = 0x07;
pub const MAX9860_VOICEFLTR: c_uint = 0x08;
pub const MAX9860_DACATTN: c_uint = 0x09;
pub const MAX9860_ADCLEVEL: c_uint = 0x0a;
pub const MAX9860_DACGAIN: c_uint = 0x0b;
pub const MAX9860_MICGAIN: c_uint = 0x0c;
pub const MAX9860_RESERVED: c_uint = 0x0d;
pub const MAX9860_MICADC: c_uint = 0x0e;
pub const MAX9860_NOISEGATE: c_uint = 0x0f;
pub const MAX9860_PWRMAN: c_uint = 0x10;
pub const MAX9860_REVISION: c_uint = 0xff;
pub const MAX9860_MAX_REGISTER: c_uint = 0xff;
// INTRSTATUS
pub const MAX9860_CLD: c_uint = 0x80;
pub const MAX9860_SLD: c_uint = 0x40;
pub const MAX9860_ULK: c_uint = 0x20;
// MICREADBACK
pub const MAX9860_NG: c_uint = 0xe0;
pub const MAX9860_AGC: c_uint = 0x1f;
// INTEN
pub const MAX9860_ICLD: c_uint = 0x80;
pub const MAX9860_ISLD: c_uint = 0x40;
pub const MAX9860_IULK: c_uint = 0x20;
// SYSCLK
pub const MAX9860_PSCLK: c_uint = 0x30;
pub const MAX9860_PSCLK_OFF: c_uint = 0x00;
pub const MAX9860_PSCLK_SHIFT: c_int = 4;
pub const MAX9860_FREQ: c_uint = 0x06;
pub const MAX9860_FREQ_NORMAL: c_uint = 0x00;
pub const MAX9860_FREQ_12MHZ: c_uint = 0x02;
pub const MAX9860_FREQ_13MHZ: c_uint = 0x04;
pub const MAX9860_FREQ_19_2MHZ: c_uint = 0x06;
pub const MAX9860_16KHZ: c_uint = 0x01;
// AUDIOCLKHIGH
pub const MAX9860_PLL: c_uint = 0x80;
pub const MAX9860_NHI: c_uint = 0x7f;
// AUDIOCLKLOW
pub const MAX9860_NLO: c_uint = 0xff;
// IFC1A
pub const MAX9860_MASTER: c_uint = 0x80;
pub const MAX9860_WCI: c_uint = 0x40;
pub const MAX9860_DBCI: c_uint = 0x20;
pub const MAX9860_DDLY: c_uint = 0x10;
pub const MAX9860_HIZ: c_uint = 0x08;
pub const MAX9860_TDM: c_uint = 0x04;
// IFC1B
pub const MAX9860_ABCI: c_uint = 0x20;
pub const MAX9860_ADLY: c_uint = 0x10;
pub const MAX9860_ST: c_uint = 0x08;
pub const MAX9860_BSEL: c_uint = 0x07;
pub const MAX9860_BSEL_OFF: c_uint = 0x00;
pub const MAX9860_BSEL_64X: c_uint = 0x01;
pub const MAX9860_BSEL_48X: c_uint = 0x02;
pub const MAX9860_BSEL_PCLK_2: c_uint = 0x04;
pub const MAX9860_BSEL_PCLK_4: c_uint = 0x05;
pub const MAX9860_BSEL_PCLK_8: c_uint = 0x06;
pub const MAX9860_BSEL_PCLK_16: c_uint = 0x07;
// VOICEFLTR
pub const MAX9860_AVFLT: c_uint = 0xf0;
pub const MAX9860_AVFLT_SHIFT: c_int = 4;
pub const MAX9860_AVFLT_COUNT: c_int = 6;
pub const MAX9860_DVFLT: c_uint = 0x0f;
pub const MAX9860_DVFLT_SHIFT: c_int = 0;
pub const MAX9860_DVFLT_COUNT: c_int = 6;
// DACATTN
pub const MAX9860_DVA: c_uint = 0xfe;
pub const MAX9860_DVA_SHIFT: c_int = 1;
pub const MAX9860_DVA_MUTE: c_uint = 0x5e;
// ADCLEVEL
pub const MAX9860_ADCRL: c_uint = 0xf0;
pub const MAX9860_ADCRL_SHIFT: c_int = 4;
pub const MAX9860_ADCLL: c_uint = 0x0f;
pub const MAX9860_ADCLL_SHIFT: c_int = 0;
pub const MAX9860_ADCxL_MIN: c_int = 15;
// DACGAIN
pub const MAX9860_DVG: c_uint = 0x60;
pub const MAX9860_DVG_SHIFT: c_int = 5;
pub const MAX9860_DVG_MAX: c_int = 3;
pub const MAX9860_DVST: c_uint = 0x1f;
pub const MAX9860_DVST_SHIFT: c_int = 0;
pub const MAX9860_DVST_MIN: c_int = 31;
// MICGAIN
pub const MAX9860_PAM: c_uint = 0x60;
pub const MAX9860_PAM_SHIFT: c_int = 5;
pub const MAX9860_PAM_MAX: c_int = 3;
pub const MAX9860_PGAM: c_uint = 0x1f;
pub const MAX9860_PGAM_SHIFT: c_int = 0;
pub const MAX9860_PGAM_MIN: c_int = 20;
// MICADC
pub const MAX9860_AGCSRC: c_uint = 0x80;
pub const MAX9860_AGCSRC_SHIFT: c_int = 7;
pub const MAX9860_AGCSRC_COUNT: c_int = 2;
pub const MAX9860_AGCRLS: c_uint = 0x70;
pub const MAX9860_AGCRLS_SHIFT: c_int = 4;
pub const MAX9860_AGCRLS_COUNT: c_int = 8;
pub const MAX9860_AGCATK: c_uint = 0x0c;
pub const MAX9860_AGCATK_SHIFT: c_int = 2;
pub const MAX9860_AGCATK_COUNT: c_int = 4;
pub const MAX9860_AGCHLD: c_uint = 0x03;
pub const MAX9860_AGCHLD_OFF: c_uint = 0x00;
pub const MAX9860_AGCHLD_SHIFT: c_int = 0;
pub const MAX9860_AGCHLD_COUNT: c_int = 4;
// NOISEGATE
pub const MAX9860_ANTH: c_uint = 0xf0;
pub const MAX9860_ANTH_SHIFT: c_int = 4;
pub const MAX9860_ANTH_MAX: c_int = 15;
pub const MAX9860_AGCTH: c_uint = 0x0f;
pub const MAX9860_AGCTH_SHIFT: c_int = 0;
pub const MAX9860_AGCTH_MIN: c_int = 15;
// PWRMAN
pub const MAX9860_SHDN: c_uint = 0x80;
pub const MAX9860_DACEN: c_uint = 0x08;
pub const MAX9860_DACEN_SHIFT: c_int = 3;
pub const MAX9860_ADCLEN: c_uint = 0x02;
pub const MAX9860_ADCLEN_SHIFT: c_int = 1;
pub const MAX9860_ADCREN: c_uint = 0x01;
pub const MAX9860_ADCREN_SHIFT: c_int = 0;
