//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/nau8540.h
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
// NAU85L40 ALSA SoC audio driver
//
// Copyright 2016 Nuvoton Technology Corp.
// Author: John Hsu <KCHSU0@nuvoton.com>
//
pub const NAU8540_REG_SW_RESET: c_uint = 0x00;
pub const NAU8540_REG_POWER_MANAGEMENT: c_uint = 0x01;
pub const NAU8540_REG_CLOCK_CTRL: c_uint = 0x02;
pub const NAU8540_REG_CLOCK_SRC: c_uint = 0x03;
pub const NAU8540_REG_FLL1: c_uint = 0x04;
pub const NAU8540_REG_FLL2: c_uint = 0x05;
pub const NAU8540_REG_FLL3: c_uint = 0x06;
pub const NAU8540_REG_FLL4: c_uint = 0x07;
pub const NAU8540_REG_FLL5: c_uint = 0x08;
pub const NAU8540_REG_FLL6: c_uint = 0x09;
pub const NAU8540_REG_FLL_VCO_RSV: c_uint = 0x0A;
pub const NAU8540_REG_PCM_CTRL0: c_uint = 0x10;
pub const NAU8540_REG_PCM_CTRL1: c_uint = 0x11;
pub const NAU8540_REG_PCM_CTRL2: c_uint = 0x12;
pub const NAU8540_REG_PCM_CTRL3: c_uint = 0x13;
pub const NAU8540_REG_PCM_CTRL4: c_uint = 0x14;
pub const NAU8540_REG_ALC_CONTROL_1: c_uint = 0x20;
pub const NAU8540_REG_ALC_CONTROL_2: c_uint = 0x21;
pub const NAU8540_REG_ALC_CONTROL_3: c_uint = 0x22;
pub const NAU8540_REG_ALC_CONTROL_4: c_uint = 0x23;
pub const NAU8540_REG_ALC_CONTROL_5: c_uint = 0x24;
pub const NAU8540_REG_ALC_GAIN_CH12: c_uint = 0x2D;
pub const NAU8540_REG_ALC_GAIN_CH34: c_uint = 0x2E;
pub const NAU8540_REG_ALC_STATUS: c_uint = 0x2F;
pub const NAU8540_REG_NOTCH_FIL1_CH1: c_uint = 0x30;
pub const NAU8540_REG_NOTCH_FIL2_CH1: c_uint = 0x31;
pub const NAU8540_REG_NOTCH_FIL1_CH2: c_uint = 0x32;
pub const NAU8540_REG_NOTCH_FIL2_CH2: c_uint = 0x33;
pub const NAU8540_REG_NOTCH_FIL1_CH3: c_uint = 0x34;
pub const NAU8540_REG_NOTCH_FIL2_CH3: c_uint = 0x35;
pub const NAU8540_REG_NOTCH_FIL1_CH4: c_uint = 0x36;
pub const NAU8540_REG_NOTCH_FIL2_CH4: c_uint = 0x37;
pub const NAU8540_REG_HPF_FILTER_CH12: c_uint = 0x38;
pub const NAU8540_REG_HPF_FILTER_CH34: c_uint = 0x39;
pub const NAU8540_REG_ADC_SAMPLE_RATE: c_uint = 0x3A;
pub const NAU8540_REG_DIGITAL_GAIN_CH1: c_uint = 0x40;
pub const NAU8540_REG_DIGITAL_GAIN_CH2: c_uint = 0x41;
pub const NAU8540_REG_DIGITAL_GAIN_CH3: c_uint = 0x42;
pub const NAU8540_REG_DIGITAL_GAIN_CH4: c_uint = 0x43;
pub const NAU8540_REG_DIGITAL_MUX: c_uint = 0x44;
pub const NAU8540_REG_P2P_CH1: c_uint = 0x48;
pub const NAU8540_REG_P2P_CH2: c_uint = 0x49;
pub const NAU8540_REG_P2P_CH3: c_uint = 0x4A;
pub const NAU8540_REG_P2P_CH4: c_uint = 0x4B;
pub const NAU8540_REG_PEAK_CH1: c_uint = 0x4C;
pub const NAU8540_REG_PEAK_CH2: c_uint = 0x4D;
pub const NAU8540_REG_PEAK_CH3: c_uint = 0x4E;
pub const NAU8540_REG_PEAK_CH4: c_uint = 0x4F;
pub const NAU8540_REG_GPIO_CTRL: c_uint = 0x50;
pub const NAU8540_REG_MISC_CTRL: c_uint = 0x51;
pub const NAU8540_REG_I2C_CTRL: c_uint = 0x52;
pub const NAU8540_REG_I2C_DEVICE_ID: c_uint = 0x58;
pub const NAU8540_REG_RST: c_uint = 0x5A;
pub const NAU8540_REG_VMID_CTRL: c_uint = 0x60;
pub const NAU8540_REG_MUTE: c_uint = 0x61;
pub const NAU8540_REG_ANALOG_ADC1: c_uint = 0x64;
pub const NAU8540_REG_ANALOG_ADC2: c_uint = 0x65;
pub const NAU8540_REG_ANALOG_PWR: c_uint = 0x66;
pub const NAU8540_REG_MIC_BIAS: c_uint = 0x67;
pub const NAU8540_REG_REFERENCE: c_uint = 0x68;
pub const NAU8540_REG_FEPGA1: c_uint = 0x69;
pub const NAU8540_REG_FEPGA2: c_uint = 0x6A;
pub const NAU8540_REG_FEPGA3: c_uint = 0x6B;
pub const NAU8540_REG_FEPGA4: c_uint = 0x6C;
pub const NAU8540_REG_PWR: c_uint = 0x6D;

// POWER_MANAGEMENT (0x01)
pub const NAU8540_ADC_ALL_EN: c_uint = 0xf;

pub const NAU8540_ADC1_EN: c_uint = 0x1;
// CLOCK_CTRL (0x02)

// CLOCK_SRC (0x03)
pub const NAU8540_CLK_SRC_SFT: c_int = 15;

pub const NAU8540_CLK_ADC_SRC_SFT: c_int = 6;

pub const NAU8540_CLK_MCLK_SRC_MASK: c_uint = 0xf;
// FLL1 (0x04)
pub const NAU8540_ICTRL_LATCH_SFT: c_int = 10;

pub const NAU8540_FLL_RATIO_MASK: c_uint = 0x7f;
// FLL3 (0x06)
pub const NAU8540_GAIN_ERR_SFT: c_int = 12;

pub const NAU8540_FLL_CLK_SRC_SFT: c_int = 10;

pub const NAU8540_FLL_INTEGER_MASK: c_uint = 0x3ff;
// FLL4 (0x07)
pub const NAU8540_FLL_REF_DIV_SFT: c_int = 10;

// FLL5 (0x08)

// FLL6 (0x9)

// PCM_CTRL0 (0x10)
pub const NAU8540_I2S_BP_SFT: c_int = 7;

pub const NAU8540_I2S_PCMB_SFT: c_int = 6;

pub const NAU8540_I2S_DL_SFT: c_int = 2;

pub const NAU8540_I2S_DF_MASK: c_uint = 0x3;
pub const NAU8540_I2S_DF_RIGTH: c_int = 0;
pub const NAU8540_I2S_DF_LEFT: c_uint = 0x1;
pub const NAU8540_I2S_DF_I2S: c_uint = 0x2;
pub const NAU8540_I2S_DF_PCM_AB: c_uint = 0x3;
// PCM_CTRL1 (0x11)

pub const NAU8540_I2S_LRC_DIV_SFT: c_int = 12;

pub const NAU8540_I2S_MS_SFT: c_int = 3;

pub const NAU8540_I2S_BLK_DIV_MASK: c_uint = 0x7;
// PCM_CTRL1 (0x12)

pub const NAU8540_I2S_TSLOT_L_MASK: c_uint = 0x3ff;
// PCM_CTRL4 (0x14)

pub const NAU8540_TDM_TX_MASK: c_uint = 0xf;
// ALC_CONTROL_3 (0x22)

// ADC_SAMPLE_RATE (0x3A)

pub const NAU8540_ADC_OSR_MASK: c_uint = 0x3;
pub const NAU8540_ADC_OSR_256: c_uint = 0x3;
pub const NAU8540_ADC_OSR_128: c_uint = 0x2;
pub const NAU8540_ADC_OSR_64: c_uint = 0x1;
pub const NAU8540_ADC_OSR_32: c_uint = 0x0;
// VMID_CTRL (0x60)

pub const NAU8540_VMID_SEL_SFT: c_int = 4;

// MUTE (0x61)
pub const NAU8540_PGA_CH1_MUTE: c_uint = 0x1;
pub const NAU8540_PGA_CH2_MUTE: c_uint = 0x2;
pub const NAU8540_PGA_CH3_MUTE: c_uint = 0x4;
pub const NAU8540_PGA_CH4_MUTE: c_uint = 0x8;
pub const NAU8540_PGA_CH_ALL_MUTE: c_uint = 0xf;
// MIC_BIAS (0x67)

// REFERENCE (0x68)

// FEPGA1 (0x69)
pub const NAU8540_FEPGA1_MODCH2_SHT_SFT: c_int = 7;

pub const NAU8540_FEPGA1_MODCH1_SHT_SFT: c_int = 3;

// FEPGA2 (0x6A)
pub const NAU8540_FEPGA2_MODCH4_SHT_SFT: c_int = 7;

pub const NAU8540_FEPGA2_MODCH3_SHT_SFT: c_int = 3;

pub const NAU8540_ACDC_CTL_SFT: c_int = 8;

// System Clock Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8540 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8540_fll {
    pub mclk_src: c_int,
    pub ratio: c_int,
    pub fll_frac: c_int,
    pub fll_int: c_int,
    pub clk_ref_div: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8540_fll_attr {
    pub param: c_uint,
    pub val: c_uint,
}

// over sampling rate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8540_osr_attr {
    pub osr: c_uint,
    pub clk_src: c_uint,
}
