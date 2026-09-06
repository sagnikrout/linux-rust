//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5575.h
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
// rt5575.h  --  ALC5575 ALSA SoC audio driver
//
// Copyright(c) 2025 Realtek Semiconductor Corp.
//
pub const RT5575_DEVICE_ID: c_uint = 0x10ec5575;
pub const RT5575_DSP_MAPPING: c_uint = 0x18000000;
pub const RT5575_BOOT: c_uint = 0x8004;
pub const RT5575_ID: c_uint = 0x8008;
pub const RT5575_ID_1: c_uint = 0x800c;
pub const RT5575_MIXL_VOL: c_uint = 0x8a14;
pub const RT5575_MIXR_VOL: c_uint = 0x8a18;
pub const RT5575_PROMPT_VOL: c_uint = 0x8a84;
pub const RT5575_SPK01_VOL: c_uint = 0x8a88;
pub const RT5575_SPK23_VOL: c_uint = 0x8a8c;
pub const RT5575_MIC1_VOL: c_uint = 0x8a98;
pub const RT5575_MIC2_VOL: c_uint = 0x8a9c;
pub const RT5575_WNC_CTRL: c_uint = 0x80ec;
pub const RT5575_MODE_CTRL: c_uint = 0x80f0;
pub const RT5575_I2S_RATE_CTRL: c_uint = 0x80f4;
pub const RT5575_SLEEP_CTRL: c_uint = 0x80f8;
pub const RT5575_ALG_BYPASS_CTRL: c_uint = 0x80fc;
pub const RT5575_PINMUX_CTRL_2: c_uint = 0x81a4;
pub const RT5575_GPIO_CTRL_1: c_uint = 0x8208;
pub const RT5575_DSP_BUS_CTRL: c_uint = 0x880c;
pub const RT5575_SW_INT: c_uint = 0x0018;
pub const RT5575_DSP_BOOT_ERR: c_uint = 0x8e14;
pub const RT5575_DSP_READY: c_uint = 0x8e24;
pub const RT5575_DSP_CMD_ADDR: c_uint = 0x8e28;
pub const RT5575_EFUSE_DATA_2: c_uint = 0xc638;
pub const RT5575_EFUSE_DATA_3: c_uint = 0xc63c;
pub const RT5575_EFUSE_PID: c_uint = 0xc660;
pub const RT5575_BOOT_MASK: c_uint = 0x3;
pub const RT5575_BOOT_SPI: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5575_priv {
    pub i2c: *mut i2c_client,
    pub component: *mut snd_soc_component,
    pub regmap: *mut *mut regmap dsp_regmap,,
}
