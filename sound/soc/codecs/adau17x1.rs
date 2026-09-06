//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/adau17x1.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adau17x1_type {
    ADAU1361,
    ADAU1761,
    ADAU1761_AS_1361,
    ADAU1381,
    ADAU1781,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adau17x1_pll {
    ADAU17X1_PLL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adau17x1_pll_src {
    ADAU17X1_PLL_SRC_MCLK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adau17x1_clk_src {
// Automatically configure PLL based on the sample rate
    ADAU17X1_CLK_SRC_PLL_AUTO,
    ADAU17X1_CLK_SRC_MCLK,
    ADAU17X1_CLK_SRC_PLL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adau {
    pub sysclk: c_uint,
    pub pll_freq: c_uint,
    pub mclk: *mut clk,
    pub clk_src: adau17x1_clk_src,
    pub type: adau17x1_type,
    pub dev): *mut *mut void (switch_mode)(struct device,
    pub dai_fmt: c_uint,
    pub pll_regs: [u8; 6],
    pub master: bool,
    pub tdm_slot: [c_uint; 2],
    pub dsp_bypass: [bool; 2],
    pub regmap: *mut regmap,
    pub sigmadsp: *mut sigmadsp,
}

extern "C" {
    pub fn adau17x1_add_widgets(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn adau17x1_add_routes(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn adau17x1_remove(dev: *mut device);
}
extern "C" {
    pub fn adau17x1_readable_register(dev: *mut device, reg: c_uint) -> bool;
}
extern "C" {
    pub fn adau17x1_volatile_register(dev: *mut device, reg: c_uint) -> bool;
}
extern "C" {
    pub fn adau17x1_precious_register(dev: *mut device, reg: c_uint) -> bool;
}
extern "C" {
    pub fn adau17x1_resume(component: *mut snd_soc_component) -> c_int;
}
pub const ADAU17X1_CLOCK_CONTROL: c_uint = 0x4000;
pub const ADAU17X1_PLL_CONTROL: c_uint = 0x4002;
pub const ADAU17X1_REC_POWER_MGMT: c_uint = 0x4009;
pub const ADAU17X1_MICBIAS: c_uint = 0x4010;
pub const ADAU17X1_SERIAL_PORT0: c_uint = 0x4015;
pub const ADAU17X1_SERIAL_PORT1: c_uint = 0x4016;
pub const ADAU17X1_CONVERTER0: c_uint = 0x4017;
pub const ADAU17X1_CONVERTER1: c_uint = 0x4018;
pub const ADAU17X1_LEFT_INPUT_DIGITAL_VOL: c_uint = 0x401a;
pub const ADAU17X1_RIGHT_INPUT_DIGITAL_VOL: c_uint = 0x401b;
pub const ADAU17X1_ADC_CONTROL: c_uint = 0x4019;
pub const ADAU17X1_PLAY_POWER_MGMT: c_uint = 0x4029;
pub const ADAU17X1_DAC_CONTROL0: c_uint = 0x402a;
pub const ADAU17X1_DAC_CONTROL1: c_uint = 0x402b;
pub const ADAU17X1_DAC_CONTROL2: c_uint = 0x402c;
pub const ADAU17X1_SERIAL_PORT_PAD: c_uint = 0x402d;
pub const ADAU17X1_CONTROL_PORT_PAD0: c_uint = 0x402f;
pub const ADAU17X1_CONTROL_PORT_PAD1: c_uint = 0x4030;
pub const ADAU17X1_DSP_SAMPLING_RATE: c_uint = 0x40eb;
pub const ADAU17X1_SERIAL_INPUT_ROUTE: c_uint = 0x40f2;
pub const ADAU17X1_SERIAL_OUTPUT_ROUTE: c_uint = 0x40f3;
pub const ADAU17X1_DSP_ENABLE: c_uint = 0x40f5;
pub const ADAU17X1_DSP_RUN: c_uint = 0x40f6;
pub const ADAU17X1_SERIAL_SAMPLING_RATE: c_uint = 0x40f8;

pub const ADAU17X1_SERIAL_PORT1_DELAY1: c_uint = 0x00;
pub const ADAU17X1_SERIAL_PORT1_DELAY0: c_uint = 0x01;
pub const ADAU17X1_SERIAL_PORT1_DELAY8: c_uint = 0x02;
pub const ADAU17X1_SERIAL_PORT1_DELAY16: c_uint = 0x03;
pub const ADAU17X1_SERIAL_PORT1_DELAY_MASK: c_uint = 0x03;
pub const ADAU17X1_CLOCK_CONTROL_INFREQ_MASK: c_uint = 0x6;

pub const ADAU17X1_CONVERTER1_ADC_PAIR_MASK: c_uint = 0x3;
pub const ADAU17X1_CONVERTER0_CONVSR_MASK: c_uint = 0x7;

