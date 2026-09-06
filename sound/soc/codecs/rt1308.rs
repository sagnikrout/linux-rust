//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt1308.h
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
// rt1308.h  --  RT1308 ALSA SoC amplifier component driver
//
// Copyright 2019 Realtek Semiconductor Corp.
// Author: Derek Fang <derek.fang@realtek.com>
//
pub const RT1308_DEVICE_ID_NUM: c_uint = 0x10ec1300;
pub const RT1308_RESET: c_uint = 0x00;
pub const RT1308_RESET_N: c_uint = 0x01;
pub const RT1308_CLK_GATING: c_uint = 0x02;
pub const RT1308_PLL_1: c_uint = 0x03;
pub const RT1308_PLL_2: c_uint = 0x04;
pub const RT1308_PLL_INT: c_uint = 0x05;
pub const RT1308_CLK_1: c_uint = 0x06;
pub const RT1308_DATA_PATH: c_uint = 0x07;
pub const RT1308_CLK_2: c_uint = 0x08;
pub const RT1308_SIL_DET: c_uint = 0x09;
pub const RT1308_CLK_DET: c_uint = 0x0a;
pub const RT1308_DC_DET: c_uint = 0x0b;
pub const RT1308_DC_DET_THRES: c_uint = 0x0c;
pub const RT1308_DAC_SET: c_uint = 0x10;
pub const RT1308_SRC_SET: c_uint = 0x11;
pub const RT1308_DAC_BUF: c_uint = 0x12;
pub const RT1308_ADC_SET: c_uint = 0x13;
pub const RT1308_ADC_SET_INT: c_uint = 0x14;
pub const RT1308_I2S_SET_1: c_uint = 0x15;
pub const RT1308_I2S_SET_2: c_uint = 0x16;
pub const RT1308_I2C_I2S_SDW_SET: c_uint = 0x17;
pub const RT1308_SDW_REG_RW: c_uint = 0x18;
pub const RT1308_SDW_REG_RDATA: c_uint = 0x19;
pub const RT1308_IV_SENSE: c_uint = 0x1a;
pub const RT1308_I2S_TX_DAC_SET: c_uint = 0x1b;
pub const RT1308_AD_FILTER_SET: c_uint = 0x1c;
pub const RT1308_DC_CAL_1: c_uint = 0x20;
pub const RT1308_DC_CAL_2: c_uint = 0x21;
pub const RT1308_DC_CAL_L_OFFSET: c_uint = 0x22;
pub const RT1308_DC_CAL_R_OFFSET: c_uint = 0x23;
pub const RT1308_PVDD_OFFSET_CTL: c_uint = 0x24;
pub const RT1308_PVDD_OFFSET_L: c_uint = 0x25;
pub const RT1308_PVDD_OFFSET_R: c_uint = 0x26;
pub const RT1308_PVDD_OFFSET_PBTL: c_uint = 0x27;
pub const RT1308_PVDD_OFFSET_PVDD: c_uint = 0x28;
pub const RT1308_CAL_OFFSET_DAC_PBTL: c_uint = 0x29;
pub const RT1308_CAL_OFFSET_DAC_L: c_uint = 0x2a;
pub const RT1308_CAL_OFFSET_DAC_R: c_uint = 0x2b;
pub const RT1308_CAL_OFFSET_PWM_L: c_uint = 0x2c;
pub const RT1308_CAL_OFFSET_PWM_R: c_uint = 0x2d;
pub const RT1308_CAL_PWM_VOS_ADC_L: c_uint = 0x2e;
pub const RT1308_CAL_PWM_VOS_ADC_R: c_uint = 0x2f;
pub const RT1308_CLASS_D_SET_1: c_uint = 0x30;
pub const RT1308_CLASS_D_SET_2: c_uint = 0x31;
pub const RT1308_POWER: c_uint = 0x32;
pub const RT1308_LDO: c_uint = 0x33;
pub const RT1308_VREF: c_uint = 0x34;
pub const RT1308_MBIAS: c_uint = 0x35;
pub const RT1308_POWER_STATUS: c_uint = 0x36;
pub const RT1308_POWER_INT: c_uint = 0x37;
pub const RT1308_SINE_TONE_GEN_1: c_uint = 0x50;
pub const RT1308_SINE_TONE_GEN_2: c_uint = 0x51;
pub const RT1308_BQ_SET: c_uint = 0x54;
pub const RT1308_BQ_PARA_UPDATE: c_uint = 0x55;
pub const RT1308_BQ_PRE_VOL_L: c_uint = 0x56;
pub const RT1308_BQ_PRE_VOL_R: c_uint = 0x57;
pub const RT1308_BQ_POST_VOL_L: c_uint = 0x58;
pub const RT1308_BQ_POST_VOL_R: c_uint = 0x59;
pub const RT1308_BQ1_L_H0: c_uint = 0x5b;
pub const RT1308_BQ1_L_B1: c_uint = 0x5c;
pub const RT1308_BQ1_L_B2: c_uint = 0x5d;
pub const RT1308_BQ1_L_A1: c_uint = 0x5e;
pub const RT1308_BQ1_L_A2: c_uint = 0x5f;
pub const RT1308_BQ1_R_H0: c_uint = 0x60;
pub const RT1308_BQ1_R_B1: c_uint = 0x61;
pub const RT1308_BQ1_R_B2: c_uint = 0x62;
pub const RT1308_BQ1_R_A1: c_uint = 0x63;
pub const RT1308_BQ1_R_A2: c_uint = 0x64;
pub const RT1308_BQ2_L_H0: c_uint = 0x65;
pub const RT1308_BQ2_L_B1: c_uint = 0x66;
pub const RT1308_BQ2_L_B2: c_uint = 0x67;
pub const RT1308_BQ2_L_A1: c_uint = 0x68;
pub const RT1308_BQ2_L_A2: c_uint = 0x69;
pub const RT1308_BQ2_R_H0: c_uint = 0x6a;
pub const RT1308_BQ2_R_B1: c_uint = 0x6b;
pub const RT1308_BQ2_R_B2: c_uint = 0x6c;
pub const RT1308_BQ2_R_A1: c_uint = 0x6d;
pub const RT1308_BQ2_R_A2: c_uint = 0x6e;
pub const RT1308_VEN_DEV_ID: c_uint = 0x70;
pub const RT1308_VERSION_ID: c_uint = 0x71;
pub const RT1308_SPK_BOUND: c_uint = 0x72;
pub const RT1308_BQ1_EQ_L_1: c_uint = 0x73;
pub const RT1308_BQ1_EQ_L_2: c_uint = 0x74;
pub const RT1308_BQ1_EQ_L_3: c_uint = 0x75;
pub const RT1308_BQ1_EQ_R_1: c_uint = 0x76;
pub const RT1308_BQ1_EQ_R_2: c_uint = 0x77;
pub const RT1308_BQ1_EQ_R_3: c_uint = 0x78;
pub const RT1308_BQ2_EQ_L_1: c_uint = 0x79;
pub const RT1308_BQ2_EQ_L_2: c_uint = 0x7a;
pub const RT1308_BQ2_EQ_L_3: c_uint = 0x7b;
pub const RT1308_BQ2_EQ_R_1: c_uint = 0x7c;
pub const RT1308_BQ2_EQ_R_2: c_uint = 0x7d;
pub const RT1308_BQ2_EQ_R_3: c_uint = 0x7e;
pub const RT1308_EFUSE_1: c_uint = 0x7f;
pub const RT1308_EFUSE_2: c_uint = 0x80;
pub const RT1308_EFUSE_PROG_PVDD_L: c_uint = 0x81;
pub const RT1308_EFUSE_PROG_PVDD_R: c_uint = 0x82;
pub const RT1308_EFUSE_PROG_R0_L: c_uint = 0x83;
pub const RT1308_EFUSE_PROG_R0_R: c_uint = 0x84;
pub const RT1308_EFUSE_PROG_DEV: c_uint = 0x85;
pub const RT1308_EFUSE_READ_PVDD_L: c_uint = 0x86;
pub const RT1308_EFUSE_READ_PVDD_R: c_uint = 0x87;
pub const RT1308_EFUSE_READ_PVDD_PTBL: c_uint = 0x88;
pub const RT1308_EFUSE_READ_DEV: c_uint = 0x89;
pub const RT1308_EFUSE_READ_R0: c_uint = 0x8a;
pub const RT1308_EFUSE_READ_ADC_L: c_uint = 0x8b;
pub const RT1308_EFUSE_READ_ADC_R: c_uint = 0x8c;
pub const RT1308_EFUSE_READ_ADC_PBTL: c_uint = 0x8d;
pub const RT1308_EFUSE_RESERVE: c_uint = 0x8e;
pub const RT1308_PADS_1: c_uint = 0x90;
pub const RT1308_PADS_2: c_uint = 0x91;
pub const RT1308_TEST_MODE: c_uint = 0xa0;
pub const RT1308_TEST_1: c_uint = 0xa1;
pub const RT1308_TEST_2: c_uint = 0xa2;
pub const RT1308_TEST_3: c_uint = 0xa3;
pub const RT1308_TEST_4: c_uint = 0xa4;
pub const RT1308_EFUSE_DATA_0_MSB: c_uint = 0xb0;
pub const RT1308_EFUSE_DATA_0_LSB: c_uint = 0xb1;
pub const RT1308_EFUSE_DATA_1_MSB: c_uint = 0xb2;
pub const RT1308_EFUSE_DATA_1_LSB: c_uint = 0xb3;
pub const RT1308_EFUSE_DATA_2_MSB: c_uint = 0xb4;
pub const RT1308_EFUSE_DATA_2_LSB: c_uint = 0xb5;
pub const RT1308_EFUSE_DATA_3_MSB: c_uint = 0xb6;
pub const RT1308_EFUSE_DATA_3_LSB: c_uint = 0xb7;
pub const RT1308_EFUSE_DATA_TEST_MSB: c_uint = 0xb8;
pub const RT1308_EFUSE_DATA_TEST_LSB: c_uint = 0xb9;
pub const RT1308_EFUSE_STATUS_1: c_uint = 0xba;
pub const RT1308_EFUSE_STATUS_2: c_uint = 0xbb;
pub const RT1308_TCON_1: c_uint = 0xc0;
pub const RT1308_TCON_2: c_uint = 0xc1;
pub const RT1308_DUMMY_REG: c_uint = 0xf0;
pub const RT1308_MAX_REG: c_uint = 0xff;
// PLL1 M/N/K Code-1 (0x03)
pub const RT1308_PLL1_K_SFT: c_int = 24;

pub const RT1308_PLL1_M_BYPASS_SFT: c_int = 23;

pub const RT1308_PLL1_M_SFT: c_int = 16;

pub const RT1308_PLL1_N_SFT: c_int = 8;
// CLOCK-1 (0x06)

pub const RT1308_DIV_FS_SYS_SFT: c_int = 28;

pub const RT1308_SEL_FS_SYS_SFT: c_int = 24;

// CLOCK-2 (0x08)

pub const RT1308_DIV_PRE_PLL_SFT: c_int = 28;

pub const RT1308_SEL_PLL_SRC_SFT: c_int = 24;

// Clock Detect (0x0a)

pub const RT1308_MCLK_DET_EN_SFT: c_int = 25;

pub const RT1308_BCLK_DET_EN_SFT: c_int = 24;

// DAC Setting (0x10)
pub const RT1308_DVOL_MUTE_R_EN_SFT: c_int = 7;
pub const RT1308_DVOL_MUTE_L_EN_SFT: c_int = 6;
// I2S Setting-1 (0x15)

pub const RT1308_I2S_DF_SEL_SFT: c_int = 12;

pub const RT1308_I2S_DL_RX_SEL_SFT: c_int = 4;

pub const RT1308_I2S_DL_TX_SEL_SFT: c_int = 0;

// I2S Setting-2 (0x16)

pub const RT1308_I2S_DL_SEL_SFT: c_int = 24;

pub const RT1308_I2S_BCLK_SFT: c_int = 14;

// Power Control-1 (0x32)

pub const RT1308_POW_MBIAS20U_BIT: c_int = 31;

pub const RT1308_POW_ALDO_BIT: c_int = 30;

pub const RT1308_POW_DBG_BIT: c_int = 29;

pub const RT1308_POW_DACL_BIT: c_int = 28;

pub const RT1308_POW_DAC1_BIT: c_int = 27;

pub const RT1308_POW_CLK25M_BIT: c_int = 26;

pub const RT1308_POW_ADC_R_BIT: c_int = 25;

pub const RT1308_POW_ADC_L_BIT: c_int = 24;

pub const RT1308_POW_DLDO_BIT: c_int = 21;

pub const RT1308_POW_VREF_BIT: c_int = 20;

pub const RT1308_POW_MIXER_R_BIT: c_int = 18;

pub const RT1308_POW_MIXER_L_BIT: c_int = 17;

pub const RT1308_POW_MBIAS4U_BIT: c_int = 16;

pub const RT1308_POW_PLL2_LDO_EN_BIT: c_int = 12;

pub const RT1308_POW_PLL2B_EN_BIT: c_int = 11;

pub const RT1308_POW_PLL2F_EN_BIT: c_int = 10;

pub const RT1308_POW_PLL2F2_EN_BIT: c_int = 9;

pub const RT1308_POW_PLL2B2_EN_BIT: c_int = 8;
// Power Control-2 (0x36)

// System Clock Source
// PLL Source
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt1308_hw_ver {
    RT1308_VER_C = 2,
    RT1308_VER_D
}
