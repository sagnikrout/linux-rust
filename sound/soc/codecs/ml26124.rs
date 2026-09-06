//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/ml26124.h
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
// Copyright (C) 2011 LAPIS Semiconductor Co., Ltd.
//
// Clock Control Register
pub const ML26124_SMPLING_RATE: c_uint = 0x00;
pub const ML26124_PLLNL: c_uint = 0x02;
pub const ML26124_PLLNH: c_uint = 0x04;
pub const ML26124_PLLML: c_uint = 0x06;
pub const ML26124_PLLMH: c_uint = 0x08;
pub const ML26124_PLLDIV: c_uint = 0x0a;
pub const ML26124_CLK_EN: c_uint = 0x0c;
pub const ML26124_CLK_CTL: c_uint = 0x0e;
// System Control Register
pub const ML26124_SW_RST: c_uint = 0x10;
pub const ML26124_REC_PLYBAK_RUN: c_uint = 0x12;
pub const ML26124_MIC_TIM: c_uint = 0x14;
// Power Mnagement Register
pub const ML26124_PW_REF_PW_MNG: c_uint = 0x20;
pub const ML26124_PW_IN_PW_MNG: c_uint = 0x22;
pub const ML26124_PW_DAC_PW_MNG: c_uint = 0x24;
pub const ML26124_PW_SPAMP_PW_MNG: c_uint = 0x26;
pub const ML26124_PW_LOUT_PW_MNG: c_uint = 0x28;
pub const ML26124_PW_VOUT_PW_MNG: c_uint = 0x2a;
pub const ML26124_PW_ZCCMP_PW_MNG: c_uint = 0x2e;
// Analog Reference Control Register
pub const ML26124_PW_MICBIAS_VOL: c_uint = 0x30;
// Input/Output Amplifier Control Register
pub const ML26124_PW_MIC_IN_VOL: c_uint = 0x32;
pub const ML26124_PW_MIC_BOST_VOL: c_uint = 0x38;
pub const ML26124_PW_SPK_AMP_VOL: c_uint = 0x3a;
pub const ML26124_PW_AMP_VOL_FUNC: c_uint = 0x48;
pub const ML26124_PW_AMP_VOL_FADE: c_uint = 0x4a;
// Analog Path Control Register
pub const ML26124_SPK_AMP_OUT: c_uint = 0x54;
pub const ML26124_MIC_IF_CTL: c_uint = 0x5a;
pub const ML26124_MIC_SELECT: c_uint = 0xe8;
// Audio Interface Control Register
pub const ML26124_SAI_TRANS_CTL: c_uint = 0x60;
pub const ML26124_SAI_RCV_CTL: c_uint = 0x62;
pub const ML26124_SAI_MODE_SEL: c_uint = 0x64;
// DSP Control Register
pub const ML26124_FILTER_EN: c_uint = 0x66;
pub const ML26124_DVOL_CTL: c_uint = 0x68;
pub const ML26124_MIXER_VOL_CTL: c_uint = 0x6a;
pub const ML26124_RECORD_DIG_VOL: c_uint = 0x6c;
pub const ML26124_PLBAK_DIG_VOL: c_uint = 0x70;
pub const ML26124_DIGI_BOOST_VOL: c_uint = 0x72;
pub const ML26124_EQ_GAIN_BRAND0: c_uint = 0x74;
pub const ML26124_EQ_GAIN_BRAND1: c_uint = 0x76;
pub const ML26124_EQ_GAIN_BRAND2: c_uint = 0x78;
pub const ML26124_EQ_GAIN_BRAND3: c_uint = 0x7a;
pub const ML26124_EQ_GAIN_BRAND4: c_uint = 0x7c;
pub const ML26124_HPF2_CUTOFF: c_uint = 0x7e;
pub const ML26124_EQBRAND0_F0L: c_uint = 0x80;
pub const ML26124_EQBRAND0_F0H: c_uint = 0x82;
pub const ML26124_EQBRAND0_F1L: c_uint = 0x84;
pub const ML26124_EQBRAND0_F1H: c_uint = 0x86;
pub const ML26124_EQBRAND1_F0L: c_uint = 0x88;
pub const ML26124_EQBRAND1_F0H: c_uint = 0x8a;
pub const ML26124_EQBRAND1_F1L: c_uint = 0x8c;
pub const ML26124_EQBRAND1_F1H: c_uint = 0x8e;
pub const ML26124_EQBRAND2_F0L: c_uint = 0x90;
pub const ML26124_EQBRAND2_F0H: c_uint = 0x92;
pub const ML26124_EQBRAND2_F1L: c_uint = 0x94;
pub const ML26124_EQBRAND2_F1H: c_uint = 0x96;
pub const ML26124_EQBRAND3_F0L: c_uint = 0x98;
pub const ML26124_EQBRAND3_F0H: c_uint = 0x9a;
pub const ML26124_EQBRAND3_F1L: c_uint = 0x9c;
pub const ML26124_EQBRAND3_F1H: c_uint = 0x9e;
pub const ML26124_EQBRAND4_F0L: c_uint = 0xa0;
pub const ML26124_EQBRAND4_F0H: c_uint = 0xa2;
pub const ML26124_EQBRAND4_F1L: c_uint = 0xa4;
pub const ML26124_EQBRAND4_F1H: c_uint = 0xa6;
// ALC Control Register
pub const ML26124_ALC_MODE: c_uint = 0xb0;
pub const ML26124_ALC_ATTACK_TIM: c_uint = 0xb2;
pub const ML26124_ALC_DECAY_TIM: c_uint = 0xb4;
pub const ML26124_ALC_HOLD_TIM: c_uint = 0xb6;
pub const ML26124_ALC_TARGET_LEV: c_uint = 0xb8;
pub const ML26124_ALC_MAXMIN_GAIN: c_uint = 0xba;
pub const ML26124_NOIS_GATE_THRSH: c_uint = 0xbc;
pub const ML26124_ALC_ZERO_TIMOUT: c_uint = 0xbe;
// Playback Limiter Control Register
pub const ML26124_PL_ATTACKTIME: c_uint = 0xc0;
pub const ML26124_PL_DECAYTIME: c_uint = 0xc2;
pub const ML26124_PL_TARGETTIME: c_uint = 0xc4;
pub const ML26124_PL_MAXMIN_GAIN: c_uint = 0xc6;
pub const ML26124_PLYBAK_BOST_VOL: c_uint = 0xc8;
pub const ML26124_PL_0CROSS_TIMOUT: c_uint = 0xca;
// Video Amplifer Control Register
pub const ML26124_VIDEO_AMP_GAIN_CTL: c_uint = 0xd0;
pub const ML26124_VIDEO_AMP_SETUP1: c_uint = 0xd2;
pub const ML26124_VIDEO_AMP_CTL2: c_uint = 0xd4;
// Clock select for machine driver
pub const ML26124_USE_PLL: c_int = 0;
pub const ML26124_USE_MCLKI_256FS: c_int = 1;
pub const ML26124_USE_MCLKI_512FS: c_int = 2;
pub const ML26124_USE_MCLKI_1024FS: c_int = 3;
// Register Mask
pub const ML26124_R0_MASK: c_uint = 0xf;
pub const ML26124_R2_MASK: c_uint = 0xff;
pub const ML26124_R4_MASK: c_uint = 0x1;
pub const ML26124_R6_MASK: c_uint = 0xf;
pub const ML26124_R8_MASK: c_uint = 0x3f;
pub const ML26124_Ra_MASK: c_uint = 0x1f;
pub const ML26124_Rc_MASK: c_uint = 0x1f;
pub const ML26124_Re_MASK: c_uint = 0x7;
pub const ML26124_R10_MASK: c_uint = 0x1;
pub const ML26124_R12_MASK: c_uint = 0x17;
pub const ML26124_R14_MASK: c_uint = 0x3f;
pub const ML26124_R20_MASK: c_uint = 0x47;
pub const ML26124_R22_MASK: c_uint = 0xa;
pub const ML26124_R24_MASK: c_uint = 0x2;
pub const ML26124_R26_MASK: c_uint = 0x1f;
pub const ML26124_R28_MASK: c_uint = 0x2;
pub const ML26124_R2a_MASK: c_uint = 0x2;
pub const ML26124_R2e_MASK: c_uint = 0x2;
pub const ML26124_R30_MASK: c_uint = 0x7;
pub const ML26124_R32_MASK: c_uint = 0x3f;
pub const ML26124_R38_MASK: c_uint = 0x38;
pub const ML26124_R3a_MASK: c_uint = 0x3f;
pub const ML26124_R48_MASK: c_uint = 0x3;
pub const ML26124_R4a_MASK: c_uint = 0x7;
pub const ML26124_R54_MASK: c_uint = 0x2a;
pub const ML26124_R5a_MASK: c_uint = 0x3;
pub const ML26124_Re8_MASK: c_uint = 0x3;
pub const ML26124_R60_MASK: c_uint = 0xff;
pub const ML26124_R62_MASK: c_uint = 0xff;
pub const ML26124_R64_MASK: c_uint = 0x1;
pub const ML26124_R66_MASK: c_uint = 0xff;
pub const ML26124_R68_MASK: c_uint = 0x3b;
pub const ML26124_R6a_MASK: c_uint = 0xf3;
pub const ML26124_R6c_MASK: c_uint = 0xff;
pub const ML26124_R70_MASK: c_uint = 0xff;

pub const ML26124_BLT_ALL_ON: c_uint = 0x1f;
pub const ML26124_BLT_PREAMP_ON: c_uint = 0x13;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ml26124_regs {
    ML26124_MCLK = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ml26124_clk_in {
    ML26124_USE_PLLOUT = 0,
    ML26124_USE_MCLKI,
}
