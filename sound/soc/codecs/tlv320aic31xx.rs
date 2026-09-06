//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tlv320aic31xx.h
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
// ALSA SoC TLV320AIC31xx CODEC Driver Definitions
//
// Copyright (C) 2014-2017 Texas Instruments Incorporated - https://www.ti.com
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aic31xx_type {
    AIC3100	= 0,
    AIC3110 = AIC31XX_STEREO_CLASS_D_BIT,
    AIC3120 = AIC31XX_MINIDSP_BIT,
    AIC3111 = AIC31XX_STEREO_CLASS_D_BIT | AIC31XX_MINIDSP_BIT,
    DAC3100 = DAC31XX_BIT,
    DAC3101 = DAC31XX_BIT | AIC31XX_STEREO_CLASS_D_BIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aic31xx_pdata {
    pub codec_type: aic31xx_type,
    pub gpio_reset: c_uint,
    pub micbias_vg: c_int,
}

// Page 0 Registers

// Page 1 Registers

// Bits, masks, and shifts
// AIC31XX_CLKMUX

pub const AIC31XX_PLL_CLKIN_MCLK: c_uint = 0x00;
pub const AIC31XX_PLL_CLKIN_BCLK: c_uint = 0x01;
pub const AIC31XX_PLL_CLKIN_GPIO1: c_uint = 0x02;
pub const AIC31XX_PLL_CLKIN_DIN: c_uint = 0x03;

pub const AIC31XX_CODEC_CLKIN_MCLK: c_uint = 0x00;
pub const AIC31XX_CODEC_CLKIN_BCLK: c_uint = 0x01;
pub const AIC31XX_CODEC_CLKIN_GPIO1: c_uint = 0x02;
pub const AIC31XX_CODEC_CLKIN_PLL: c_uint = 0x03;
// AIC31XX_PLLPR
// AIC31XX_NDAC
// AIC31XX_MDAC
// AIC31XX_NADC
// AIC31XX_MADC
// AIC31XX_BCLKN

// AIC31XX_IFACE1

pub const AIC31XX_I2S_MODE: c_uint = 0x00;
pub const AIC31XX_DSP_MODE: c_uint = 0x01;
pub const AIC31XX_RIGHT_JUSTIFIED_MODE: c_uint = 0x02;
pub const AIC31XX_LEFT_JUSTIFIED_MODE: c_uint = 0x03;

pub const AIC31XX_WORD_LEN_16BITS: c_uint = 0x00;
pub const AIC31XX_WORD_LEN_20BITS: c_uint = 0x01;
pub const AIC31XX_WORD_LEN_24BITS: c_uint = 0x02;
pub const AIC31XX_WORD_LEN_32BITS: c_uint = 0x03;

// AIC31XX_DATA_OFFSET

// AIC31XX_IFACE2

pub const AIC31XX_DAC2BCLK: c_uint = 0x00;
pub const AIC31XX_DACMOD2BCLK: c_uint = 0x01;
pub const AIC31XX_ADC2BCLK: c_uint = 0x02;
pub const AIC31XX_ADCMOD2BCLK: c_uint = 0x03;

// AIC31XX_ADCFLAG

// AIC31XX_DACFLAG1

// AIC31XX_OFFLAG

// AIC31XX_INTRDACFLAG

// AIC31XX_INT1CTRL

// AIC31XX_GPIO1

pub const AIC31XX_GPIO1_FUNC_SHIFT: c_int = 2;
pub const AIC31XX_GPIO1_DISABLED: c_uint = 0x00;
pub const AIC31XX_GPIO1_INPUT: c_uint = 0x01;
pub const AIC31XX_GPIO1_GPI: c_uint = 0x02;
pub const AIC31XX_GPIO1_GPO: c_uint = 0x03;
pub const AIC31XX_GPIO1_CLKOUT: c_uint = 0x04;
pub const AIC31XX_GPIO1_INT1: c_uint = 0x05;
pub const AIC31XX_GPIO1_INT2: c_uint = 0x06;
pub const AIC31XX_GPIO1_ADC_WCLK: c_uint = 0x07;
pub const AIC31XX_GPIO1_SBCLK: c_uint = 0x08;
pub const AIC31XX_GPIO1_SWCLK: c_uint = 0x09;
pub const AIC31XX_GPIO1_ADC_MOD_CLK: c_uint = 0x10;
pub const AIC31XX_GPIO1_SDOUT: c_uint = 0x11;
// AIC31XX_DACMUTE

// AIC31XX_HSDETECT

pub const AIC31XX_HSD_TYPE_SHIFT: c_int = 5;
pub const AIC31XX_HSD_NONE: c_uint = 0x00;
pub const AIC31XX_HSD_HP: c_uint = 0x01;
pub const AIC31XX_HSD_HS: c_uint = 0x03;
// AIC31XX_HPDRIVER

pub const AIC31XX_HPD_OCMV_SHIFT: c_int = 3;
pub const AIC31XX_HPD_OCMV_1_35V: c_uint = 0x0;
pub const AIC31XX_HPD_OCMV_1_5V: c_uint = 0x1;
pub const AIC31XX_HPD_OCMV_1_65V: c_uint = 0x2;
pub const AIC31XX_HPD_OCMV_1_8V: c_uint = 0x3;
// AIC31XX_MICBIAS

pub const AIC31XX_MICBIAS_SHIFT: c_int = 0;
