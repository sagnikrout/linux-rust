//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/ac100.h
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
// Functions and registers to access AC100 codec / RTC combo IC.
//
// Copyright (C) 2016 Chen-Yu Tsai
//
// Chen-Yu Tsai <wens@csie.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ac100_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

// Audio codec related registers
pub const AC100_CHIP_AUDIO_RST: c_uint = 0x00;
pub const AC100_PLL_CTRL1: c_uint = 0x01;
pub const AC100_PLL_CTRL2: c_uint = 0x02;
pub const AC100_SYSCLK_CTRL: c_uint = 0x03;
pub const AC100_MOD_CLK_ENA: c_uint = 0x04;
pub const AC100_MOD_RST_CTRL: c_uint = 0x05;
pub const AC100_I2S_SR_CTRL: c_uint = 0x06;
// I2S1 interface
pub const AC100_I2S1_CLK_CTRL: c_uint = 0x10;
pub const AC100_I2S1_SND_OUT_CTRL: c_uint = 0x11;
pub const AC100_I2S1_SND_IN_CTRL: c_uint = 0x12;
pub const AC100_I2S1_MXR_SRC: c_uint = 0x13;
pub const AC100_I2S1_VOL_CTRL1: c_uint = 0x14;
pub const AC100_I2S1_VOL_CTRL2: c_uint = 0x15;
pub const AC100_I2S1_VOL_CTRL3: c_uint = 0x16;
pub const AC100_I2S1_VOL_CTRL4: c_uint = 0x17;
pub const AC100_I2S1_MXR_GAIN: c_uint = 0x18;
// I2S2 interface
pub const AC100_I2S2_CLK_CTRL: c_uint = 0x20;
pub const AC100_I2S2_SND_OUT_CTRL: c_uint = 0x21;
pub const AC100_I2S2_SND_IN_CTRL: c_uint = 0x22;
pub const AC100_I2S2_MXR_SRC: c_uint = 0x23;
pub const AC100_I2S2_VOL_CTRL1: c_uint = 0x24;
pub const AC100_I2S2_VOL_CTRL2: c_uint = 0x25;
pub const AC100_I2S2_VOL_CTRL3: c_uint = 0x26;
pub const AC100_I2S2_VOL_CTRL4: c_uint = 0x27;
pub const AC100_I2S2_MXR_GAIN: c_uint = 0x28;
// I2S3 interface
pub const AC100_I2S3_CLK_CTRL: c_uint = 0x30;
pub const AC100_I2S3_SND_OUT_CTRL: c_uint = 0x31;
pub const AC100_I2S3_SND_IN_CTRL: c_uint = 0x32;
pub const AC100_I2S3_SIG_PATH_CTRL: c_uint = 0x33;
// ADC digital controls
pub const AC100_ADC_DIG_CTRL: c_uint = 0x40;
pub const AC100_ADC_VOL_CTRL: c_uint = 0x41;
// HMIC plug sensing / key detection
pub const AC100_HMIC_CTRL1: c_uint = 0x44;
pub const AC100_HMIC_CTRL2: c_uint = 0x45;
pub const AC100_HMIC_STATUS: c_uint = 0x46;
// DAC digital controls
pub const AC100_DAC_DIG_CTRL: c_uint = 0x48;
pub const AC100_DAC_VOL_CTRL: c_uint = 0x49;
pub const AC100_DAC_MXR_SRC: c_uint = 0x4c;
pub const AC100_DAC_MXR_GAIN: c_uint = 0x4d;
// Analog controls
pub const AC100_ADC_APC_CTRL: c_uint = 0x50;
pub const AC100_ADC_SRC: c_uint = 0x51;
pub const AC100_ADC_SRC_BST_CTRL: c_uint = 0x52;
pub const AC100_OUT_MXR_DAC_A_CTRL: c_uint = 0x53;
pub const AC100_OUT_MXR_SRC: c_uint = 0x54;
pub const AC100_OUT_MXR_SRC_BST: c_uint = 0x55;
pub const AC100_HPOUT_CTRL: c_uint = 0x56;
pub const AC100_ERPOUT_CTRL: c_uint = 0x57;
pub const AC100_SPKOUT_CTRL: c_uint = 0x58;
pub const AC100_LINEOUT_CTRL: c_uint = 0x59;
// ADC digital audio processing (high pass filter & auto gain control
pub const AC100_ADC_DAP_L_STA: c_uint = 0x80;
pub const AC100_ADC_DAP_R_STA: c_uint = 0x81;
pub const AC100_ADC_DAP_L_CTRL: c_uint = 0x82;
pub const AC100_ADC_DAP_R_CTRL: c_uint = 0x83;
pub const AC100_ADC_DAP_L_T_L: c_uint = 0x84 /* Left Target Level */;
pub const AC100_ADC_DAP_R_T_L: c_uint = 0x85 /* Right Target Level */;
pub const AC100_ADC_DAP_L_H_A_C: c_uint = 0x86 /* Left High Avg. Coef */;
pub const AC100_ADC_DAP_L_L_A_C: c_uint = 0x87 /* Left Low Avg. Coef */;
pub const AC100_ADC_DAP_R_H_A_C: c_uint = 0x88 /* Right High Avg. Coef */;
pub const AC100_ADC_DAP_R_L_A_C: c_uint = 0x89 /* Right Low Avg. Coef */;
pub const AC100_ADC_DAP_L_D_T: c_uint = 0x8a /* Left Decay Time */;
pub const AC100_ADC_DAP_L_A_T: c_uint = 0x8b /* Left Attack Time */;
pub const AC100_ADC_DAP_R_D_T: c_uint = 0x8c /* Right Decay Time */;
pub const AC100_ADC_DAP_R_A_T: c_uint = 0x8d /* Right Attack Time */;
pub const AC100_ADC_DAP_N_TH: c_uint = 0x8e /* Noise Threshold */;
pub const AC100_ADC_DAP_L_H_N_A_C: c_uint = 0x8f /* Left High Noise Avg. Coef */;
pub const AC100_ADC_DAP_L_L_N_A_C: c_uint = 0x90 /* Left Low Noise Avg. Coef */;
pub const AC100_ADC_DAP_R_H_N_A_C: c_uint = 0x91 /* Right High Noise Avg. Coef */;
pub const AC100_ADC_DAP_R_L_N_A_C: c_uint = 0x92 /* Right Low Noise Avg. Coef */;
pub const AC100_ADC_DAP_H_HPF_C: c_uint = 0x93 /* High High-Pass-Filter Coef */;
pub const AC100_ADC_DAP_L_HPF_C: c_uint = 0x94 /* Low High-Pass-Filter Coef */;
pub const AC100_ADC_DAP_OPT: c_uint = 0x95 /* AGC Optimum */;
// DAC digital audio processing (high pass filter & dynamic range control)
pub const AC100_DAC_DAP_CTRL: c_uint = 0xa0;
pub const AC100_DAC_DAP_H_HPF_C: c_uint = 0xa1 /* High High-Pass-Filter Coef */;
pub const AC100_DAC_DAP_L_HPF_C: c_uint = 0xa2 /* Low High-Pass-Filter Coef */;
pub const AC100_DAC_DAP_L_H_E_A_C: c_uint = 0xa3 /* Left High Energy Avg Coef */;
pub const AC100_DAC_DAP_L_L_E_A_C: c_uint = 0xa4 /* Left Low Energy Avg Coef */;
pub const AC100_DAC_DAP_R_H_E_A_C: c_uint = 0xa5 /* Right High Energy Avg Coef */;
pub const AC100_DAC_DAP_R_L_E_A_C: c_uint = 0xa6 /* Right Low Energy Avg Coef */;
pub const AC100_DAC_DAP_H_G_D_T_C: c_uint = 0xa7 /* High Gain Delay Time Coef */;
pub const AC100_DAC_DAP_L_G_D_T_C: c_uint = 0xa8 /* Low Gain Delay Time Coef */;
pub const AC100_DAC_DAP_H_G_A_T_C: c_uint = 0xa9 /* High Gain Attack Time Coef */;
pub const AC100_DAC_DAP_L_G_A_T_C: c_uint = 0xaa /* Low Gain Attack Time Coef */;
pub const AC100_DAC_DAP_H_E_TH: c_uint = 0xab /* High Energy Threshold */;
pub const AC100_DAC_DAP_L_E_TH: c_uint = 0xac /* Low Energy Threshold */;
pub const AC100_DAC_DAP_H_G_K: c_uint = 0xad /* High Gain K parameter */;
pub const AC100_DAC_DAP_L_G_K: c_uint = 0xae /* Low Gain K parameter */;
pub const AC100_DAC_DAP_H_G_OFF: c_uint = 0xaf /* High Gain offset */;
pub const AC100_DAC_DAP_L_G_OFF: c_uint = 0xb0 /* Low Gain offset */;
pub const AC100_DAC_DAP_OPT: c_uint = 0xb1 /* DRC optimum */;
// Digital audio processing enable
pub const AC100_ADC_DAP_ENA: c_uint = 0xb4;
pub const AC100_DAC_DAP_ENA: c_uint = 0xb5;
// SRC control
pub const AC100_SRC1_CTRL1: c_uint = 0xb8;
pub const AC100_SRC1_CTRL2: c_uint = 0xb9;
pub const AC100_SRC1_CTRL3: c_uint = 0xba;
pub const AC100_SRC1_CTRL4: c_uint = 0xbb;
pub const AC100_SRC2_CTRL1: c_uint = 0xbc;
pub const AC100_SRC2_CTRL2: c_uint = 0xbd;
pub const AC100_SRC2_CTRL3: c_uint = 0xbe;
pub const AC100_SRC2_CTRL4: c_uint = 0xbf;
// RTC clk control
pub const AC100_CLK32K_ANALOG_CTRL: c_uint = 0xc0;
pub const AC100_CLKOUT_CTRL1: c_uint = 0xc1;
pub const AC100_CLKOUT_CTRL2: c_uint = 0xc2;
pub const AC100_CLKOUT_CTRL3: c_uint = 0xc3;
// RTC module
pub const AC100_RTC_RST: c_uint = 0xc6;
pub const AC100_RTC_CTRL: c_uint = 0xc7;
pub const AC100_RTC_SEC: c_uint = 0xc8 /* second */;
pub const AC100_RTC_MIN: c_uint = 0xc9 /* minute */;
pub const AC100_RTC_HOU: c_uint = 0xca /* hour */;
pub const AC100_RTC_WEE: c_uint = 0xcb /* weekday */;
pub const AC100_RTC_DAY: c_uint = 0xcc /* day */;
pub const AC100_RTC_MON: c_uint = 0xcd /* month */;
pub const AC100_RTC_YEA: c_uint = 0xce /* year */;
pub const AC100_RTC_UPD: c_uint = 0xcf /* update trigger */;
// RTC alarm
pub const AC100_ALM_INT_ENA: c_uint = 0xd0;
pub const AC100_ALM_INT_STA: c_uint = 0xd1;
pub const AC100_ALM_SEC: c_uint = 0xd8;
pub const AC100_ALM_MIN: c_uint = 0xd9;
pub const AC100_ALM_HOU: c_uint = 0xda;
pub const AC100_ALM_WEE: c_uint = 0xdb;
pub const AC100_ALM_DAY: c_uint = 0xdc;
pub const AC100_ALM_MON: c_uint = 0xdd;
pub const AC100_ALM_YEA: c_uint = 0xde;
pub const AC100_ALM_UPD: c_uint = 0xdf;
// RTC general purpose register 0 ~ 15

