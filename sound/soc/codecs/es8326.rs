//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/es8326.h
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
// es8326.h -- es8326 ALSA SoC audio driver
// Copyright Everest Semiconductor Co.,Ltd
//
// Authors: David Yang <yangxiaohua@everest-semi.com>
//
// ES8326 register space
pub const ES8326_RESET: c_uint = 0x00;
pub const ES8326_CLK_CTL: c_uint = 0x01;
pub const ES8326_CLK_INV: c_uint = 0x02;
pub const ES8326_CLK_RESAMPLE: c_uint = 0x03;
pub const ES8326_CLK_DIV1: c_uint = 0x04;
pub const ES8326_CLK_DIV2: c_uint = 0x05;
pub const ES8326_CLK_DLL: c_uint = 0x06;
pub const ES8326_CLK_MUX: c_uint = 0x07;
pub const ES8326_CLK_ADC_SEL: c_uint = 0x08;
pub const ES8326_CLK_DAC_SEL: c_uint = 0x09;
pub const ES8326_CLK_ADC_OSR: c_uint = 0x0a;
pub const ES8326_CLK_DAC_OSR: c_uint = 0x0b;
pub const ES8326_CLK_DIV_CPC: c_uint = 0x0c;
pub const ES8326_CLK_DIV_BCLK: c_uint = 0x0d;
pub const ES8326_CLK_TRI: c_uint = 0x0e;
pub const ES8326_CLK_DIV_LRCK: c_uint = 0x0f;
pub const ES8326_CLK_VMIDS1: c_uint = 0x10;
pub const ES8326_CLK_VMIDS2: c_uint = 0x11;
pub const ES8326_CLK_CAL_TIME: c_uint = 0x12;
pub const ES8326_FMT: c_uint = 0x13;
pub const ES8326_DAC_MUTE: c_uint = 0x14;
pub const ES8326_ADC_MUTE: c_uint = 0x15;
pub const ES8326_ANA_PDN: c_uint = 0x16;
pub const ES8326_PGA_PDN: c_uint = 0x17;
pub const ES8326_VMIDSEL: c_uint = 0x18;
pub const ES8326_ANA_LP: c_uint = 0x19;
pub const ES8326_ANA_DMS: c_uint = 0x1a;
pub const ES8326_ANA_MICBIAS: c_uint = 0x1b;
pub const ES8326_ANA_VSEL: c_uint = 0x1c;
pub const ES8326_SYS_BIAS: c_uint = 0x1d;
pub const ES8326_BIAS_SW1: c_uint = 0x1e;
pub const ES8326_BIAS_SW2: c_uint = 0x1f;
pub const ES8326_BIAS_SW3: c_uint = 0x20;
pub const ES8326_BIAS_SW4: c_uint = 0x21;
pub const ES8326_VMIDLOW: c_uint = 0x22;
pub const ES8326_PGAGAIN: c_uint = 0x23;
pub const ES8326_HP_DRIVER: c_uint = 0x24;
pub const ES8326_DAC2HPMIX: c_uint = 0x25;
pub const ES8326_HP_VOL: c_uint = 0x26;
pub const ES8326_HP_CAL: c_uint = 0x27;
pub const ES8326_HP_DRIVER_REF: c_uint = 0x28;
pub const ES8326_ADC_SCALE: c_uint = 0x29;
pub const ES8326_ADC1_SRC: c_uint = 0x2a;
pub const ES8326_ADC2_SRC: c_uint = 0x2b;
pub const ES8326_ADC1_VOL: c_uint = 0x2c;
pub const ES8326_ADC2_VOL: c_uint = 0x2d;
pub const ES8326_ADC_RAMPRATE: c_uint = 0x2e;
pub const ES8326_ADC_DRE: c_uint = 0x2f;
pub const ES8326_ADC_DRE_GAIN: c_uint = 0x30;
pub const ES8326_ADC_DRE_GATE: c_uint = 0x31;
pub const ES8326_ALC_RECOVERY: c_uint = 0x32;
pub const ES8326_ALC_LEVEL: c_uint = 0x33;
pub const ES8326_ADC_HPFS1: c_uint = 0x34;
pub const ES8326_ADC_HPFS2: c_uint = 0x35;
pub const ES8326_ADC_EQ: c_uint = 0x36;
pub const ES8326_HP_OFFSET_CAL: c_uint = 0x4A;
pub const ES8326_HPL_OFFSET_INI: c_uint = 0x4B;
pub const ES8326_HPR_OFFSET_INI: c_uint = 0x4C;
pub const ES8326_DAC_DSM: c_uint = 0x4D;
pub const ES8326_DAC_RAMPRATE: c_uint = 0x4E;
pub const ES8326_DAC_VPPSCALE: c_uint = 0x4F;
pub const ES8326_DACL_VOL: c_uint = 0x50;
pub const ES8326_DRC_RECOVERY: c_uint = 0x53;
pub const ES8326_DRC_WINSIZE: c_uint = 0x54;
pub const ES8326_DAC_CROSSTALK: c_uint = 0x55;
pub const ES8326_HPJACK_TIMER: c_uint = 0x56;
pub const ES8326_HPDET_TYPE: c_uint = 0x57;
pub const ES8326_INT_SOURCE: c_uint = 0x58;
pub const ES8326_INTOUT_IO: c_uint = 0x59;
pub const ES8326_SDINOUT1_IO: c_uint = 0x5A;
pub const ES8326_SDINOUT23_IO: c_uint = 0x5B;
pub const ES8326_JACK_PULSE: c_uint = 0x5C;
pub const ES8326_DACR_VOL: c_uint = 0xF4;
pub const ES8326_SPKL_VOL: c_uint = 0xF5;
pub const ES8326_SPKR_VOL: c_uint = 0xF6;
pub const ES8326_HP_MISC: c_uint = 0xF7;
pub const ES8326_CTIA_OMTP_STA: c_uint = 0xF8;
pub const ES8326_PULLUP_CTL: c_uint = 0xF9;
pub const ES8326_CSM_I2C_STA: c_uint = 0xFA;
pub const ES8326_HPDET_STA: c_uint = 0xFB;
pub const ES8326_CSM_MUTE_STA: c_uint = 0xFC;
pub const ES8326_CHIP_ID1: c_uint = 0xFD;
pub const ES8326_CHIP_ID2: c_uint = 0xFE;
pub const ES8326_CHIP_VERSION: c_uint = 0xFF;
// ES8326_RESET

// ES8326_CLK_CTL

// ES8326_CLK_INV

// ES8326_FMT

pub const ES8326_DAIFMT_I2S: c_int = 0;

// ES8326_PGAGAIN

// ES8326_HP_CAL
pub const ES8326_HP_OFF: c_int = 0;

// ES8326_ADC1_SRC
pub const ES8326_ADC1_SHIFT: c_int = 0;
pub const ES8326_ADC2_SHIFT: c_int = 4;
pub const ES8326_ADC_SRC_ANA: c_int = 0;
pub const ES8326_ADC_SRC_ANA_INV_SW0: c_int = 1;
pub const ES8326_ADC_SRC_ANA_INV_SW1: c_int = 2;
pub const ES8326_ADC_SRC_DMIC_MCLK: c_int = 3;
pub const ES8326_ADC_SRC_DMIC_SDIN2: c_int = 4;
pub const ES8326_ADC_SRC_DMIC_SDIN2_INV: c_int = 5;
pub const ES8326_ADC_SRC_DMIC_SDIN3: c_int = 6;
pub const ES8326_ADC_SRC_DMIC_SDIN3_INV: c_int = 7;

// ES8326_ADC2_SRC
pub const ES8326_ADC3_SHIFT: c_int = 0;
pub const ES8326_ADC4_SHIFT: c_int = 3;
// ES8326_HPDET_TYPE

// ES8326_INT_SOURCE

// ES8326_SDINOUT1_IO

pub const ES8326_SDINOUT1_SHIFT: c_int = 4;
// ES8326_SDINOUT23_IO
pub const ES8326_SDINOUT2_SHIFT: c_int = 4;
pub const ES8326_SDINOUT3_SHIFT: c_int = 0;
// ES8326_HPDET_STA

// ES8326_CHIP_VERSION 0xFF

