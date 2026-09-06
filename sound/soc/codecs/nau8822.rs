//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/nau8822.h
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
// nau8822.h  --  NAU8822 ALSA SoC Audio driver
//
// Copyright 2017 Nuvoton Technology Crop.
//
// Author: David Lin <ctlin0@nuvoton.com>
// Co-author: John Hsu <kchsu0@nuvoton.com>
// Co-author: Seven Li <wtli@nuvoton.com>
//
pub const NAU8822_REG_RESET: c_uint = 0x00;
pub const NAU8822_REG_POWER_MANAGEMENT_1: c_uint = 0x01;
pub const NAU8822_REG_POWER_MANAGEMENT_2: c_uint = 0x02;
pub const NAU8822_REG_POWER_MANAGEMENT_3: c_uint = 0x03;
pub const NAU8822_REG_AUDIO_INTERFACE: c_uint = 0x04;
pub const NAU8822_REG_COMPANDING_CONTROL: c_uint = 0x05;
pub const NAU8822_REG_CLOCKING: c_uint = 0x06;
pub const NAU8822_REG_ADDITIONAL_CONTROL: c_uint = 0x07;
pub const NAU8822_REG_GPIO_CONTROL: c_uint = 0x08;
pub const NAU8822_REG_JACK_DETECT_CONTROL_1: c_uint = 0x09;
pub const NAU8822_REG_DAC_CONTROL: c_uint = 0x0A;
pub const NAU8822_REG_LEFT_DAC_DIGITAL_VOLUME: c_uint = 0x0B;
pub const NAU8822_REG_RIGHT_DAC_DIGITAL_VOLUME: c_uint = 0x0C;
pub const NAU8822_REG_JACK_DETECT_CONTROL_2: c_uint = 0x0D;
pub const NAU8822_REG_ADC_CONTROL: c_uint = 0x0E;
pub const NAU8822_REG_LEFT_ADC_DIGITAL_VOLUME: c_uint = 0x0F;
pub const NAU8822_REG_RIGHT_ADC_DIGITAL_VOLUME: c_uint = 0x10;
pub const NAU8822_REG_EQ1: c_uint = 0x12;
pub const NAU8822_REG_EQ2: c_uint = 0x13;
pub const NAU8822_REG_EQ3: c_uint = 0x14;
pub const NAU8822_REG_EQ4: c_uint = 0x15;
pub const NAU8822_REG_EQ5: c_uint = 0x16;
pub const NAU8822_REG_DAC_LIMITER_1: c_uint = 0x18;
pub const NAU8822_REG_DAC_LIMITER_2: c_uint = 0x19;
pub const NAU8822_REG_NOTCH_FILTER_1: c_uint = 0x1B;
pub const NAU8822_REG_NOTCH_FILTER_2: c_uint = 0x1C;
pub const NAU8822_REG_NOTCH_FILTER_3: c_uint = 0x1D;
pub const NAU8822_REG_NOTCH_FILTER_4: c_uint = 0x1E;
pub const NAU8822_REG_ALC_CONTROL_1: c_uint = 0x20;
pub const NAU8822_REG_ALC_CONTROL_2: c_uint = 0x21;
pub const NAU8822_REG_ALC_CONTROL_3: c_uint = 0x22;
pub const NAU8822_REG_NOISE_GATE: c_uint = 0x23;
pub const NAU8822_REG_PLL_N: c_uint = 0x24;
pub const NAU8822_REG_PLL_K1: c_uint = 0x25;
pub const NAU8822_REG_PLL_K2: c_uint = 0x26;
pub const NAU8822_REG_PLL_K3: c_uint = 0x27;
pub const NAU8822_REG_3D_CONTROL: c_uint = 0x29;
pub const NAU8822_REG_RIGHT_SPEAKER_CONTROL: c_uint = 0x2B;
pub const NAU8822_REG_INPUT_CONTROL: c_uint = 0x2C;
pub const NAU8822_REG_LEFT_INP_PGA_CONTROL: c_uint = 0x2D;
pub const NAU8822_REG_RIGHT_INP_PGA_CONTROL: c_uint = 0x2E;
pub const NAU8822_REG_LEFT_ADC_BOOST_CONTROL: c_uint = 0x2F;
pub const NAU8822_REG_RIGHT_ADC_BOOST_CONTROL: c_uint = 0x30;
pub const NAU8822_REG_OUTPUT_CONTROL: c_uint = 0x31;
pub const NAU8822_REG_LEFT_MIXER_CONTROL: c_uint = 0x32;
pub const NAU8822_REG_RIGHT_MIXER_CONTROL: c_uint = 0x33;
pub const NAU8822_REG_LHP_VOLUME: c_uint = 0x34;
pub const NAU8822_REG_RHP_VOLUME: c_uint = 0x35;
pub const NAU8822_REG_LSPKOUT_VOLUME: c_uint = 0x36;
pub const NAU8822_REG_RSPKOUT_VOLUME: c_uint = 0x37;
pub const NAU8822_REG_AUX2_MIXER: c_uint = 0x38;
pub const NAU8822_REG_AUX1_MIXER: c_uint = 0x39;
pub const NAU8822_REG_POWER_MANAGEMENT_4: c_uint = 0x3A;
pub const NAU8822_REG_LEFT_TIME_SLOT: c_uint = 0x3B;
pub const NAU8822_REG_MISC: c_uint = 0x3C;
pub const NAU8822_REG_RIGHT_TIME_SLOT: c_uint = 0x3D;
pub const NAU8822_REG_DEVICE_REVISION: c_uint = 0x3E;
pub const NAU8822_REG_DEVICE_ID: c_uint = 0x3F;
pub const NAU8822_REG_DAC_DITHER: c_uint = 0x41;
pub const NAU8822_REG_ALC_ENHANCE_1: c_uint = 0x46;
pub const NAU8822_REG_ALC_ENHANCE_2: c_uint = 0x47;
pub const NAU8822_REG_192KHZ_SAMPLING: c_uint = 0x48;
pub const NAU8822_REG_MISC_CONTROL: c_uint = 0x49;
pub const NAU8822_REG_INPUT_TIEOFF: c_uint = 0x4A;
pub const NAU8822_REG_POWER_REDUCTION: c_uint = 0x4B;
pub const NAU8822_REG_AGC_PEAK2PEAK: c_uint = 0x4C;
pub const NAU8822_REG_AGC_PEAK_DETECT: c_uint = 0x4D;
pub const NAU8822_REG_AUTOMUTE_CONTROL: c_uint = 0x4E;
pub const NAU8822_REG_OUTPUT_TIEOFF: c_uint = 0x4F;

// NAU8822_REG_POWER_MANAGEMENT_1 (0x1)
pub const NAU8822_REFIMP_MASK: c_uint = 0x3;
pub const NAU8822_REFIMP_80K: c_uint = 0x1;
pub const NAU8822_REFIMP_300K: c_uint = 0x2;
pub const NAU8822_REFIMP_3K: c_uint = 0x3;

// NAU8822_REG_AUDIO_INTERFACE (0x4)

// NAU8822_REG_COMPANDING_CONTROL (0x5)
pub const NAU8822_ADDAP_SFT: c_int = 0;
pub const NAU8822_ADCCM_SFT: c_int = 1;
pub const NAU8822_DACCM_SFT: c_int = 3;
// NAU8822_REG_CLOCKING (0x6)
pub const NAU8822_CLKIOEN_MASK: c_uint = 0x1;
pub const NAU8822_CLK_MASTER: c_uint = 0x1;
pub const NAU8822_CLK_SLAVE: c_uint = 0x0;
pub const NAU8822_MCLKSEL_SFT: c_int = 5;

pub const NAU8822_BCLKSEL_SFT: c_int = 2;

// NAU8822_REG_ADDITIONAL_CONTROL (0x08)
pub const NAU8822_SMPLR_SFT: c_int = 1;

// NAU8822_REG_EQ1 (0x12)
pub const NAU8822_EQ1GC_SFT: c_int = 0;
pub const NAU8822_EQ1CF_SFT: c_int = 5;
pub const NAU8822_EQM_SFT: c_int = 8;
// NAU8822_REG_EQ2 (0x13)
pub const NAU8822_EQ2GC_SFT: c_int = 0;
pub const NAU8822_EQ2CF_SFT: c_int = 5;
pub const NAU8822_EQ2BW_SFT: c_int = 8;
// NAU8822_REG_EQ3 (0x14)
pub const NAU8822_EQ3GC_SFT: c_int = 0;
pub const NAU8822_EQ3CF_SFT: c_int = 5;
pub const NAU8822_EQ3BW_SFT: c_int = 8;
// NAU8822_REG_EQ4 (0x15)
pub const NAU8822_EQ4GC_SFT: c_int = 0;
pub const NAU8822_EQ4CF_SFT: c_int = 5;
pub const NAU8822_EQ4BW_SFT: c_int = 8;
// NAU8822_REG_EQ5 (0x16)
pub const NAU8822_EQ5GC_SFT: c_int = 0;
pub const NAU8822_EQ5CF_SFT: c_int = 5;
// NAU8822_REG_ALC_CONTROL_1 (0x20)
pub const NAU8822_ALCMINGAIN_SFT: c_int = 0;
pub const NAU8822_ALCMXGAIN_SFT: c_int = 3;
pub const NAU8822_ALCEN_SFT: c_int = 7;
// NAU8822_REG_ALC_CONTROL_2 (0x21)
pub const NAU8822_ALCSL_SFT: c_int = 0;
pub const NAU8822_ALCHT_SFT: c_int = 4;
// NAU8822_REG_ALC_CONTROL_3 (0x22)
pub const NAU8822_ALCATK_SFT: c_int = 0;
pub const NAU8822_ALCDCY_SFT: c_int = 4;
pub const NAU8822_ALCM_SFT: c_int = 8;
// NAU8822_REG_PLL_N (0x24)

pub const NAU8822_PLLN_MASK: c_uint = 0xF;
pub const NAU8822_PLLK1_SFT: c_int = 18;
pub const NAU8822_PLLK1_MASK: c_uint = 0x3F;
// NAU8822_REG_PLL_K2 (0x26)
pub const NAU8822_PLLK2_SFT: c_int = 9;
pub const NAU8822_PLLK2_MASK: c_uint = 0x1FF;
// NAU8822_REG_PLL_K3 (0x27)
pub const NAU8822_PLLK3_MASK: c_uint = 0x1FF;
// NAU8822_REG_RIGHT_SPEAKER_CONTROL (0x2B)
pub const NAU8822_RMIXMUT: c_uint = 0x20;
pub const NAU8822_RSUBBYP: c_uint = 0x10;
pub const NAU8822_RAUXRSUBG_SFT: c_int = 1;
pub const NAU8822_RAUXRSUBG_MASK: c_uint = 0x0E;
pub const NAU8822_RAUXSMUT: c_uint = 0x01;
// System Clock Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8822_pll {
    pub pre_factor: c_int,
    pub mclk_scaler: c_int,
    pub pll_frac: c_int,
    pub pll_int: c_int,
    pub freq_in: c_int,
    pub freq_out: c_int,
}

pub const NAU8822_NUM_SUPPLIES: c_int = 4;
// Codec Private Data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8822 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub pll: nau8822_pll,
    pub sysclk: c_int,
    pub div_id: c_int,
    pub supplies: [regulator_bulk_data; NAU8822_NUM_SUPPLIES],
}
