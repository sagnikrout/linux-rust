//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max98088.h
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
// max98088.h -- MAX98088 ALSA SoC Audio driver
//
// Copyright 2010 Maxim Integrated Products
//
// MAX98088 Registers Definition
//
pub const M98088_REG_00_IRQ_STATUS: c_uint = 0x00;
pub const M98088_REG_01_MIC_STATUS: c_uint = 0x01;
pub const M98088_REG_02_JACK_STATUS: c_uint = 0x02;
pub const M98088_REG_03_BATTERY_VOLTAGE: c_uint = 0x03;
pub const M98088_REG_0F_IRQ_ENABLE: c_uint = 0x0F;
pub const M98088_REG_10_SYS_CLK: c_uint = 0x10;
pub const M98088_REG_11_DAI1_CLKMODE: c_uint = 0x11;
pub const M98088_REG_12_DAI1_CLKCFG_HI: c_uint = 0x12;
pub const M98088_REG_13_DAI1_CLKCFG_LO: c_uint = 0x13;
pub const M98088_REG_14_DAI1_FORMAT: c_uint = 0x14;
pub const M98088_REG_15_DAI1_CLOCK: c_uint = 0x15;
pub const M98088_REG_16_DAI1_IOCFG: c_uint = 0x16;
pub const M98088_REG_17_DAI1_TDM: c_uint = 0x17;
pub const M98088_REG_18_DAI1_FILTERS: c_uint = 0x18;
pub const M98088_REG_19_DAI2_CLKMODE: c_uint = 0x19;
pub const M98088_REG_1A_DAI2_CLKCFG_HI: c_uint = 0x1A;
pub const M98088_REG_1B_DAI2_CLKCFG_LO: c_uint = 0x1B;
pub const M98088_REG_1C_DAI2_FORMAT: c_uint = 0x1C;
pub const M98088_REG_1D_DAI2_CLOCK: c_uint = 0x1D;
pub const M98088_REG_1E_DAI2_IOCFG: c_uint = 0x1E;
pub const M98088_REG_1F_DAI2_TDM: c_uint = 0x1F;
pub const M98088_REG_20_DAI2_FILTERS: c_uint = 0x20;
pub const M98088_REG_21_SRC: c_uint = 0x21;
pub const M98088_REG_22_MIX_DAC: c_uint = 0x22;
pub const M98088_REG_23_MIX_ADC_LEFT: c_uint = 0x23;
pub const M98088_REG_24_MIX_ADC_RIGHT: c_uint = 0x24;
pub const M98088_REG_25_MIX_HP_LEFT: c_uint = 0x25;
pub const M98088_REG_26_MIX_HP_RIGHT: c_uint = 0x26;
pub const M98088_REG_27_MIX_HP_CNTL: c_uint = 0x27;
pub const M98088_REG_28_MIX_REC_LEFT: c_uint = 0x28;
pub const M98088_REG_29_MIX_REC_RIGHT: c_uint = 0x29;
pub const M98088_REG_2A_MIC_REC_CNTL: c_uint = 0x2A;
pub const M98088_REG_2B_MIX_SPK_LEFT: c_uint = 0x2B;
pub const M98088_REG_2C_MIX_SPK_RIGHT: c_uint = 0x2C;
pub const M98088_REG_2D_MIX_SPK_CNTL: c_uint = 0x2D;
pub const M98088_REG_2E_LVL_SIDETONE: c_uint = 0x2E;
pub const M98088_REG_2F_LVL_DAI1_PLAY: c_uint = 0x2F;
pub const M98088_REG_30_LVL_DAI1_PLAY_EQ: c_uint = 0x30;
pub const M98088_REG_31_LVL_DAI2_PLAY: c_uint = 0x31;
pub const M98088_REG_32_LVL_DAI2_PLAY_EQ: c_uint = 0x32;
pub const M98088_REG_33_LVL_ADC_L: c_uint = 0x33;
pub const M98088_REG_34_LVL_ADC_R: c_uint = 0x34;
pub const M98088_REG_35_LVL_MIC1: c_uint = 0x35;
pub const M98088_REG_36_LVL_MIC2: c_uint = 0x36;
pub const M98088_REG_37_LVL_INA: c_uint = 0x37;
pub const M98088_REG_38_LVL_INB: c_uint = 0x38;
pub const M98088_REG_39_LVL_HP_L: c_uint = 0x39;
pub const M98088_REG_3A_LVL_HP_R: c_uint = 0x3A;
pub const M98088_REG_3B_LVL_REC_L: c_uint = 0x3B;
pub const M98088_REG_3C_LVL_REC_R: c_uint = 0x3C;
pub const M98088_REG_3D_LVL_SPK_L: c_uint = 0x3D;
pub const M98088_REG_3E_LVL_SPK_R: c_uint = 0x3E;
pub const M98088_REG_3F_MICAGC_CFG: c_uint = 0x3F;
pub const M98088_REG_40_MICAGC_THRESH: c_uint = 0x40;
pub const M98088_REG_41_SPKDHP: c_uint = 0x41;
pub const M98088_REG_42_SPKDHP_THRESH: c_uint = 0x42;
pub const M98088_REG_43_SPKALC_COMP: c_uint = 0x43;
pub const M98088_REG_44_PWRLMT_CFG: c_uint = 0x44;
pub const M98088_REG_45_PWRLMT_TIME: c_uint = 0x45;
pub const M98088_REG_46_THDLMT_CFG: c_uint = 0x46;
pub const M98088_REG_47_CFG_AUDIO_IN: c_uint = 0x47;
pub const M98088_REG_48_CFG_MIC: c_uint = 0x48;
pub const M98088_REG_49_CFG_LEVEL: c_uint = 0x49;
pub const M98088_REG_4A_CFG_BYPASS: c_uint = 0x4A;
pub const M98088_REG_4B_CFG_JACKDET: c_uint = 0x4B;
pub const M98088_REG_4C_PWR_EN_IN: c_uint = 0x4C;
pub const M98088_REG_4D_PWR_EN_OUT: c_uint = 0x4D;
pub const M98088_REG_4E_BIAS_CNTL: c_uint = 0x4E;
pub const M98088_REG_4F_DAC_BIAS1: c_uint = 0x4F;
pub const M98088_REG_50_DAC_BIAS2: c_uint = 0x50;
pub const M98088_REG_51_PWR_SYS: c_uint = 0x51;
pub const M98088_REG_52_DAI1_EQ_BASE: c_uint = 0x52;
pub const M98088_REG_84_DAI2_EQ_BASE: c_uint = 0x84;
pub const M98088_REG_B6_DAI1_BIQUAD_BASE: c_uint = 0xB6;
pub const M98088_REG_C0_DAI2_BIQUAD_BASE: c_uint = 0xC0;
pub const M98088_REG_FF_REV_ID: c_uint = 0xFF;

// MAX98088 Registers Bit Fields
// M98088_REG_11_DAI1_CLKMODE, M98088_REG_19_DAI2_CLKMODE
pub const M98088_CLKMODE_MASK: c_uint = 0xFF;
// M98088_REG_14_DAI1_FORMAT, M98088_REG_1C_DAI2_FORMAT

// M98088_REG_15_DAI1_CLOCK, M98088_REG_1D_DAI2_CLOCK

// M98088_REG_16_DAI1_IOCFG, M98088_REG_1E_DAI2_IOCFG

// M98088_REG_18_DAI1_FILTERS, M98088_REG_20_DAI2_FILTERS

// M98088_REG_22_MIX_DAC

// M98088_REG_2A_MIC_REC_CNTL

// M98088_REG_2D_MIX_SPK_CNTL

pub const M98088_MIX_SPKR_GAIN_SHIFT: c_int = 2;

pub const M98088_MIX_SPKL_GAIN_SHIFT: c_int = 0;
// M98088_REG_2F_LVL_DAI1_PLAY, M98088_REG_31_LVL_DAI2_PLAY

pub const M98088_DAI_ATTENUATION_SHIFT: c_int = 0;
// M98088_REG_35_LVL_MIC1, M98088_REG_36_LVL_MIC2

pub const M98088_MICPRE_SHIFT: c_int = 5;
// M98088_REG_3A_LVL_HP_R

// M98088_REG_3C_LVL_REC_R

// M98088_REG_3E_LVL_SPK_R

// M98088_REG_48_CFG_MIC

// M98088_REG_49_CFG_LEVEL

// M98088_REG_4C_PWR_EN_IN

// M98088_REG_4D_PWR_EN_OUT

// M98088_REG_51_PWR_SYS

// Line inputs
pub const LINE_INA: c_int = 0;
pub const LINE_INB: c_int = 1;
pub const M98088_COEFS_PER_BAND: c_int = 5;

