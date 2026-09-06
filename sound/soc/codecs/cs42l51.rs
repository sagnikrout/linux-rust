//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs42l51.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// cs42l51.h
//
// ASoC Driver for Cirrus Logic CS42L51 codecs
//
// Copyright (c) 2010 Arnaud Patard <apatard@mandriva.com>
//
extern "C" {
    pub fn cs42l51_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
}
extern "C" {
    pub fn cs42l51_remove(dev: *mut device);
}
extern "C" {
    pub fn cs42l51_suspend(dev: *mut device) -> int __maybe_unused;
}
extern "C" {
    pub fn cs42l51_resume(dev: *mut device) -> int __maybe_unused;
}
pub const CS42L51_CHIP_ID: c_uint = 0x1B;
pub const CS42L51_CHIP_REV_A: c_uint = 0x00;
pub const CS42L51_CHIP_REV_B: c_uint = 0x01;
pub const CS42L51_CHIP_REV_MASK: c_uint = 0x07;
pub const CS42L51_CHIP_REV_ID: c_uint = 0x01;

pub const CS42L51_POWER_CTL1: c_uint = 0x02;

pub const CS42L51_MIC_POWER_CTL: c_uint = 0x03;

pub const CS42L51_QSM_MODE: c_int = 3;
pub const CS42L51_HSM_MODE: c_int = 2;
pub const CS42L51_SSM_MODE: c_int = 1;
pub const CS42L51_DSM_MODE: c_int = 0;

pub const CS42L51_INTF_CTL: c_uint = 0x04;

pub const CS42L51_DAC_DIF_LJ24: c_uint = 0x00;
pub const CS42L51_DAC_DIF_I2S: c_uint = 0x01;
pub const CS42L51_DAC_DIF_RJ24: c_uint = 0x02;
pub const CS42L51_DAC_DIF_RJ20: c_uint = 0x03;
pub const CS42L51_DAC_DIF_RJ18: c_uint = 0x04;
pub const CS42L51_DAC_DIF_RJ16: c_uint = 0x05;

pub const CS42L51_MIC_CTL: c_uint = 0x05;

pub const CS42L51_ADC_CTL: c_uint = 0x06;

pub const CS42L51_ADC_INPUT: c_uint = 0x07;

pub const CS42L51_DAC_OUT_CTL: c_uint = 0x08;

pub const CS42L51_DAC_CTL: c_uint = 0x09;

pub const CS42L51_ALC_PGA_CTL: c_uint = 0x0A;
pub const CS42L51_ALC_PGB_CTL: c_uint = 0x0B;

pub const CS42L51_ADCA_ATT: c_uint = 0x0C;
pub const CS42L51_ADCB_ATT: c_uint = 0x0D;
pub const CS42L51_ADCA_VOL: c_uint = 0x0E;
pub const CS42L51_ADCB_VOL: c_uint = 0x0F;
pub const CS42L51_PCMA_VOL: c_uint = 0x10;
pub const CS42L51_PCMB_VOL: c_uint = 0x11;

pub const CS42L51_BEEP_FREQ: c_uint = 0x12;
pub const CS42L51_BEEP_VOL: c_uint = 0x13;
pub const CS42L51_BEEP_CONF: c_uint = 0x14;
pub const CS42L51_TONE_CTL: c_uint = 0x15;

pub const CS42L51_AOUTA_VOL: c_uint = 0x16;
pub const CS42L51_AOUTB_VOL: c_uint = 0x17;
pub const CS42L51_PCM_MIXER: c_uint = 0x18;
pub const CS42L51_LIMIT_THRES_DIS: c_uint = 0x19;
pub const CS42L51_LIMIT_REL: c_uint = 0x1A;
pub const CS42L51_LIMIT_ATT: c_uint = 0x1B;
pub const CS42L51_ALC_EN: c_uint = 0x1C;
pub const CS42L51_ALC_REL: c_uint = 0x1D;
pub const CS42L51_ALC_THRES: c_uint = 0x1E;
pub const CS42L51_NOISE_CONF: c_uint = 0x1F;
pub const CS42L51_STATUS: c_uint = 0x20;

pub const CS42L51_CHARGE_FREQ: c_uint = 0x21;
pub const CS42L51_FIRSTREG: c_uint = 0x01;
//
// Hack: with register 0x21, it makes 33 registers. Looks like someone in the
// i2c layer doesn't like i2c smbus block read of 33 regs. Workaround by using
// 32 regs
//
pub const CS42L51_LASTREG: c_uint = 0x20;

