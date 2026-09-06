//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/es8389.h
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
// ES8389.h  --  ES8389 ALSA SoC Audio Codec
//
// Authors:
//
// Based on ES8374.h by Michael Zhang
//
// ES8389_REGISTER NAME_REG_REGISTER ADDRESS
//
pub const ES8389_RESET: c_uint = 0x00  /*reset digital,csm,clock manager etc.*/;
//
// Clock Scheme Register definition
//
pub const ES8389_MASTER_MODE: c_uint = 0x01;
pub const ES8389_MASTER_CLK: c_uint = 0x02;
pub const ES8389_CLK_OFF1: c_uint = 0x03;
pub const ES8389_CLK_DIV1: c_uint = 0x04;
pub const ES8389_CLK_MUL: c_uint = 0x05;
pub const ES8389_CLK_MUX1: c_uint = 0x06;
pub const ES8389_CLK_MUX2: c_uint = 0x07;
pub const ES8389_CLK_CTL1: c_uint = 0x08;
pub const ES8389_CLK_CTL2: c_uint = 0x09;
pub const ES8389_CLK_CTL3: c_uint = 0x0A;
pub const ES8389_SCLK_DIV: c_uint = 0x0B;
pub const ES8389_LRCK_DIV1: c_uint = 0x0C;
pub const ES8389_LRCK_DIV2: c_uint = 0x0D;
pub const ES8389_CLK_OFF2: c_uint = 0x0E;
pub const ES8389_OSC_CLK: c_uint = 0x0F;
pub const ES8389_CSM_JUMP: c_uint = 0x10;
pub const ES8389_CLK_DIV2: c_uint = 0x11;
pub const ES8389_SYSTEM12: c_uint = 0x12;
pub const ES8389_SYSTEM13: c_uint = 0x13;
pub const ES8389_SYSTEM14: c_uint = 0x14;
pub const ES8389_SYSTEM15: c_uint = 0x15;
pub const ES8389_SYSTEM16: c_uint = 0x16;
pub const ES8389_SYSTEM17: c_uint = 0x17;
pub const ES8389_SYSTEM18: c_uint = 0x18;
pub const ES8389_SYSTEM19: c_uint = 0x19;
pub const ES8389_SYSTEM1A: c_uint = 0x1A;
pub const ES8389_SYSTEM1B: c_uint = 0x1B;
pub const ES8389_SYSTEM1C: c_uint = 0x1C;
pub const ES8389_ADC_FORMAT_MUTE: c_uint = 0x20;
pub const ES8389_ADC_OSR: c_uint = 0x21;
pub const ES8389_ADC_DSP: c_uint = 0x22;
pub const ES8389_ADC_MODE: c_uint = 0x23;
pub const ES8389_ADC_HPF1: c_uint = 0x24;
pub const ES8389_ADC_HPF2: c_uint = 0x25;
pub const ES8389_OSR_VOL: c_uint = 0x26;
pub const ES8389_ADCL_VOL: c_uint = 0x27;
pub const ES8389_ADCR_VOL: c_uint = 0x28;
pub const ES8389_ALC_CTL: c_uint = 0x29;
pub const ES8389_PTDM_SLOT: c_uint = 0x2A;
pub const ES8389_ALC_ON: c_uint = 0x2B;
pub const ES8389_ALC_TARGET: c_uint = 0x2C;
pub const ES8389_ALC_GAIN: c_uint = 0x2D;
pub const ES8389_SYSTEM2E: c_uint = 0x2E;
pub const ES8389_ADC_MUTE: c_uint = 0x2F;
pub const ES8389_SYSTEM30: c_uint = 0x30;
pub const ES8389_ADC_RESET: c_uint = 0x31;
pub const ES8389_DAC_FORMAT_MUTE: c_uint = 0x40;
pub const ES8389_DAC_DSM_OSR: c_uint = 0x41;
pub const ES8389_DAC_DSP_OSR: c_uint = 0x42;
pub const ES8389_DAC_MISC: c_uint = 0x43;
pub const ES8389_DAC_MIX: c_uint = 0x44;
pub const ES8389_DAC_INV: c_uint = 0x45;
pub const ES8389_DACL_VOL: c_uint = 0x46;
pub const ES8389_DACR_VOL: c_uint = 0x47;
pub const ES8389_MIX_VOL: c_uint = 0x48;
pub const ES8389_DAC_RAMP: c_uint = 0x49;
pub const ES8389_SYSTEM4C: c_uint = 0x4C;
pub const ES8389_DAC_RESET: c_uint = 0x4D;
pub const ES8389_VMID: c_uint = 0x60;
pub const ES8389_ANA_CTL1: c_uint = 0x61;
pub const ES8389_ANA_VSEL: c_uint = 0x62;
pub const ES8389_ANA_CTL2: c_uint = 0x63;
pub const ES8389_ADC_EN: c_uint = 0x64;
pub const ES8389_HPSW: c_uint = 0x69;
pub const ES8389_LOW_POWER1: c_uint = 0x6B;
pub const ES8389_LOW_POWER2: c_uint = 0x6C;
pub const ES8389_DMIC_EN: c_uint = 0x6D;
pub const ES8389_PGA_SW: c_uint = 0x6E;
pub const ES8389_MOD_SW1: c_uint = 0x6F;
pub const ES8389_MOD_SW2: c_uint = 0x70;
pub const ES8389_MOD_SW3: c_uint = 0x71;
pub const ES8389_MIC1_GAIN: c_uint = 0x72;
pub const ES8389_MIC2_GAIN: c_uint = 0x73;
pub const ES8389_CHIP_MISC: c_uint = 0xF0;
pub const ES8389_CSM_STATE1: c_uint = 0xF1;
pub const ES8389_PULL_DOWN: c_uint = 0xF2;
pub const ES8389_ISO_CTL: c_uint = 0xF3;
pub const ES8389_CSM_STATE2: c_uint = 0xF4;
pub const ES8389_CHIP_ID0: c_uint = 0xFD;
pub const ES8389_CHIP_ID1: c_uint = 0xFE;
pub const ES8389_MAX_REGISTER: c_uint = 0xFF;

pub const ES8389_HPF_DEFAULT: c_int = 16;
pub const ES8389_HPF_OFFSET: c_int = 4;

pub const ES8389_TDM_SHIFT: c_int = 4;

pub const ES8389_MCLK_PIN: c_int = 0;
pub const ES8389_SCLK_PIN: c_int = 1;
// ES8389_FMT

pub const ES8389_DAIFMT_I2S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ES8389_supplies {
    ES8389_SUPPLY_VD = 0,
    ES8389_SUPPLY_VA,
}

pub const ES8389_3V3: c_int = 1;
pub const ES8389_1V8: c_int = 0;
