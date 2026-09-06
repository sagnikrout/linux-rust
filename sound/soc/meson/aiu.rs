//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/meson/aiu.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aiu_clk_ids {
    PCLK = 0,
    AOCLK,
    MCLK,
    MIXER
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aiu_interface {
    pub clks: *mut clk_bulk_data,
    pub clk_num: c_uint,
    pub irq: c_int,
    pub iface: gx_iface,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aiu_platform_data {
    pub has_acodec: bool,
    pub has_clk_ctrl_more_i2s_div: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aiu {
    pub spdif_mclk: *mut clk,
    pub i2s: aiu_interface,
    pub spdif: aiu_interface,
    pub platform: *const aiu_platform_data,
}

extern "C" {
    pub fn aiu_hdmi_ctrl_register_component(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn aiu_acodec_ctrl_register_component(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn aiu_fifo_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn aiu_fifo_spdif_dai_probe(dai: *mut snd_soc_dai) -> c_int;
}
pub const AIU_IEC958_BPF: c_uint = 0x000;
pub const AIU_958_MISC: c_uint = 0x010;
pub const AIU_IEC958_DCU_FF_CTRL: c_uint = 0x01c;
pub const AIU_958_CHSTAT_L0: c_uint = 0x020;
pub const AIU_958_CHSTAT_L1: c_uint = 0x024;
pub const AIU_958_CTRL: c_uint = 0x028;
pub const AIU_I2S_SOURCE_DESC: c_uint = 0x034;
pub const AIU_I2S_DAC_CFG: c_uint = 0x040;
pub const AIU_I2S_SYNC: c_uint = 0x044;
pub const AIU_I2S_MISC: c_uint = 0x048;
pub const AIU_RST_SOFT: c_uint = 0x054;
pub const AIU_CLK_CTRL: c_uint = 0x058;
pub const AIU_CLK_CTRL_MORE: c_uint = 0x064;
pub const AIU_CODEC_DAC_LRCLK_CTRL: c_uint = 0x0a0;
pub const AIU_HDMI_CLK_DATA_CTRL: c_uint = 0x0a8;
pub const AIU_ACODEC_CTRL: c_uint = 0x0b0;
pub const AIU_958_CHSTAT_R0: c_uint = 0x0c0;
pub const AIU_958_CHSTAT_R1: c_uint = 0x0c4;
pub const AIU_MEM_I2S_START: c_uint = 0x180;
pub const AIU_MEM_I2S_MASKS: c_uint = 0x18c;
pub const AIU_MEM_I2S_CONTROL: c_uint = 0x190;
pub const AIU_MEM_IEC958_START: c_uint = 0x194;
pub const AIU_MEM_IEC958_CONTROL: c_uint = 0x1a4;
pub const AIU_MEM_I2S_BUF_CNTL: c_uint = 0x1d8;
pub const AIU_MEM_IEC958_BUF_CNTL: c_uint = 0x1fc;
