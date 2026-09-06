//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max98371.h
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
// max98371.h -- MAX98371 ALSA SoC Audio driver
//
// Copyright 2011-2012 Maxim Integrated Products
//
pub const MAX98371_IRQ_CLEAR1: c_uint = 0x01;
pub const MAX98371_IRQ_CLEAR2: c_uint = 0x02;
pub const MAX98371_IRQ_CLEAR3: c_uint = 0x03;
pub const MAX98371_DAI_CLK: c_uint = 0x10;
pub const MAX98371_DAI_BSEL_MASK: c_uint = 0xF;
pub const MAX98371_DAI_BSEL_32: c_int = 2;
pub const MAX98371_DAI_BSEL_48: c_int = 3;
pub const MAX98371_DAI_BSEL_64: c_int = 4;
pub const MAX98371_SPK_SR: c_uint = 0x11;
pub const MAX98371_SPK_SR_MASK: c_uint = 0xF;
pub const MAX98371_SPK_SR_32: c_int = 6;
pub const MAX98371_SPK_SR_44: c_int = 7;
pub const MAX98371_SPK_SR_48: c_int = 8;
pub const MAX98371_SPK_SR_88: c_int = 10;
pub const MAX98371_SPK_SR_96: c_int = 11;
pub const MAX98371_DAI_CHANNEL: c_uint = 0x15;
pub const MAX98371_CHANNEL_MASK: c_uint = 0x3;
pub const MAX98371_MONOMIX_SRC: c_uint = 0x18;
pub const MAX98371_MONOMIX_CFG: c_uint = 0x19;
pub const MAX98371_HPF: c_uint = 0x1C;
pub const MAX98371_MONOMIX_SRC_MASK: c_uint = 0xFF;

pub const M98371_DAI_CHANNEL_I2S: c_uint = 0x3;
pub const MAX98371_DIGITAL_GAIN: c_uint = 0x2D;
pub const MAX98371_DIGITAL_GAIN_WIDTH: c_uint = 0x7;
pub const MAX98371_GAIN: c_uint = 0x2E;
pub const MAX98371_GAIN_SHIFT: c_uint = 0x4;
pub const MAX98371_GAIN_WIDTH: c_uint = 0x4;
pub const MAX98371_DHT_MAX_WIDTH: c_int = 4;
pub const MAX98371_FMT: c_uint = 0x14;
pub const MAX98371_CHANSZ_WIDTH: c_int = 6;

pub const MAX98371_DHT: c_uint = 0x32;
pub const MAX98371_DHT_STEP: c_uint = 0x3;
pub const MAX98371_DHT_GAIN: c_uint = 0x31;
pub const MAX98371_DHT_GAIN_WIDTH: c_uint = 0x4;
pub const MAX98371_DHT_ROT_WIDTH: c_uint = 0x4;
pub const MAX98371_SPK_ENABLE: c_uint = 0x4A;
pub const MAX98371_GLOBAL_ENABLE: c_uint = 0x50;
pub const MAX98371_SOFT_RESET: c_uint = 0x51;
pub const MAX98371_VERSION: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98371_priv {
    pub regmap: *mut regmap,
}
