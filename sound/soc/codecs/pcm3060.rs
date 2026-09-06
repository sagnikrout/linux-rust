//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/pcm3060.h
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
// PCM3060 codec driver
//
// Copyright (C) 2018 Kirill Marinushkin <k.marinushkin@gmail.com>
//

pub const PCM3060_DAI_ID_DAC: c_int = 0;
pub const PCM3060_DAI_ID_ADC: c_int = 1;
pub const PCM3060_DAI_IDS_NUM: c_int = 2;
// ADC and DAC can be clocked from separate or same sources CLK1 and CLK2

pub const PCM3060_CLK1: c_int = 1;
pub const PCM3060_CLK2: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm3060_priv_dai {
    pub is_provider: bool,
    pub sclk_freq: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm3060_priv {
    pub regmap: *mut regmap,
    pub dai: [pcm3060_priv_dai; PCM3060_DAI_IDS_NUM],
    pub 1: u8 out_se:,
}

extern "C" {
    pub fn pcm3060_probe(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pcm3060_remove(dev: *mut device) -> c_int;
}
// registers
pub const PCM3060_REG64: c_uint = 0x40;
pub const PCM3060_REG_MRST: c_uint = 0x80;
pub const PCM3060_REG_SRST: c_uint = 0x40;
pub const PCM3060_REG_ADPSV: c_uint = 0x20;
pub const PCM3060_REG_SHIFT_ADPSV: c_uint = 0x05;
pub const PCM3060_REG_DAPSV: c_uint = 0x10;
pub const PCM3060_REG_SHIFT_DAPSV: c_uint = 0x04;
pub const PCM3060_REG_SE: c_uint = 0x01;
pub const PCM3060_REG65: c_uint = 0x41;
pub const PCM3060_REG66: c_uint = 0x42;
pub const PCM3060_REG_AT2_MIN: c_uint = 0x36;
pub const PCM3060_REG_AT2_MAX: c_uint = 0xFF;
pub const PCM3060_REG67: c_uint = 0x43;
pub const PCM3060_REG72: c_uint = 0x48;
pub const PCM3060_REG_CSEL: c_uint = 0x80;
pub const PCM3060_REG_MASK_MS: c_uint = 0x70;
pub const PCM3060_REG_MS_S: c_uint = 0x00;

pub const PCM3060_REG_MASK_FMT: c_uint = 0x03;
pub const PCM3060_REG_FMT_I2S: c_uint = 0x00;
pub const PCM3060_REG_FMT_LJ: c_uint = 0x01;
pub const PCM3060_REG_FMT_RJ: c_uint = 0x02;
pub const PCM3060_REG68: c_uint = 0x44;
pub const PCM3060_REG_OVER: c_uint = 0x40;
pub const PCM3060_REG_DREV2: c_uint = 0x04;
pub const PCM3060_REG_SHIFT_MUT21: c_uint = 0x00;
pub const PCM3060_REG_SHIFT_MUT22: c_uint = 0x01;
pub const PCM3060_REG69: c_uint = 0x45;
pub const PCM3060_REG_FLT: c_uint = 0x80;
pub const PCM3060_REG_MASK_DMF: c_uint = 0x60;
pub const PCM3060_REG_DMC: c_uint = 0x10;
pub const PCM3060_REG_ZREV: c_uint = 0x02;
pub const PCM3060_REG_AZRO: c_uint = 0x01;
pub const PCM3060_REG70: c_uint = 0x46;
pub const PCM3060_REG71: c_uint = 0x47;
pub const PCM3060_REG_AT1_MIN: c_uint = 0x0E;
pub const PCM3060_REG_AT1_MAX: c_uint = 0xFF;
pub const PCM3060_REG73: c_uint = 0x49;
pub const PCM3060_REG_ZCDD: c_uint = 0x10;
pub const PCM3060_REG_BYP: c_uint = 0x08;
pub const PCM3060_REG_DREV1: c_uint = 0x04;
pub const PCM3060_REG_SHIFT_MUT11: c_uint = 0x00;
pub const PCM3060_REG_SHIFT_MUT12: c_uint = 0x01;
