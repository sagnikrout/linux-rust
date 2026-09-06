//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/nau8821.h
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
// NAU88L21 ALSA SoC audio driver
//
// Copyright 2021 Nuvoton Technology Corp.
// Author: John Hsu <kchsu0@nuvoton.com>
// Co-author: Seven Lee <wtli@nuvoton.com>
//
pub const NAU8821_R00_RESET: c_uint = 0x00;
pub const NAU8821_R01_ENA_CTRL: c_uint = 0x01;
pub const NAU8821_R03_CLK_DIVIDER: c_uint = 0x03;
pub const NAU8821_R04_FLL1: c_uint = 0x04;
pub const NAU8821_R05_FLL2: c_uint = 0x05;
pub const NAU8821_R06_FLL3: c_uint = 0x06;
pub const NAU8821_R07_FLL4: c_uint = 0x07;
pub const NAU8821_R08_FLL5: c_uint = 0x08;
pub const NAU8821_R09_FLL6: c_uint = 0x09;
pub const NAU8821_R0A_FLL7: c_uint = 0x0a;
pub const NAU8821_R0B_FLL8: c_uint = 0x0b;
pub const NAU8821_R0D_JACK_DET_CTRL: c_uint = 0x0d;
pub const NAU8821_R0F_INTERRUPT_MASK: c_uint = 0x0f;
pub const NAU8821_R10_IRQ_STATUS: c_uint = 0x10;
pub const NAU8821_R11_INT_CLR_KEY_STATUS: c_uint = 0x11;
pub const NAU8821_R12_INTERRUPT_DIS_CTRL: c_uint = 0x12;
pub const NAU8821_R13_DMIC_CTRL: c_uint = 0x13;
pub const NAU8821_R1A_GPIO12_CTRL: c_uint = 0x1a;
pub const NAU8821_R1B_TDM_CTRL: c_uint = 0x1b;
pub const NAU8821_R1C_I2S_PCM_CTRL1: c_uint = 0x1c;
pub const NAU8821_R1D_I2S_PCM_CTRL2: c_uint = 0x1d;
pub const NAU8821_R1E_LEFT_TIME_SLOT: c_uint = 0x1e;
pub const NAU8821_R1F_RIGHT_TIME_SLOT: c_uint = 0x1f;
pub const NAU8821_R21_BIQ0_COF1: c_uint = 0x21;
pub const NAU8821_R22_BIQ0_COF2: c_uint = 0x22;
pub const NAU8821_R23_BIQ0_COF3: c_uint = 0x23;
pub const NAU8821_R24_BIQ0_COF4: c_uint = 0x24;
pub const NAU8821_R25_BIQ0_COF5: c_uint = 0x25;
pub const NAU8821_R26_BIQ0_COF6: c_uint = 0x26;
pub const NAU8821_R27_BIQ0_COF7: c_uint = 0x27;
pub const NAU8821_R28_BIQ0_COF8: c_uint = 0x28;
pub const NAU8821_R29_BIQ0_COF9: c_uint = 0x29;
pub const NAU8821_R2A_BIQ0_COF10: c_uint = 0x2a;
pub const NAU8821_R2B_ADC_RATE: c_uint = 0x2b;
pub const NAU8821_R2C_DAC_CTRL1: c_uint = 0x2c;
pub const NAU8821_R2D_DAC_CTRL2: c_uint = 0x2d;
pub const NAU8821_R2F_DAC_DGAIN_CTRL: c_uint = 0x2f;
pub const NAU8821_R30_ADC_DGAIN_CTRL: c_uint = 0x30;
pub const NAU8821_R31_MUTE_CTRL: c_uint = 0x31;
pub const NAU8821_R32_HSVOL_CTRL: c_uint = 0x32;
pub const NAU8821_R34_DACR_CTRL: c_uint = 0x34;
pub const NAU8821_R35_ADC_DGAIN_CTRL1: c_uint = 0x35;
pub const NAU8821_R36_ADC_DRC_KNEE_IP12: c_uint = 0x36;
pub const NAU8821_R37_ADC_DRC_KNEE_IP34: c_uint = 0x37;
pub const NAU8821_R38_ADC_DRC_SLOPES: c_uint = 0x38;
pub const NAU8821_R39_ADC_DRC_ATKDCY: c_uint = 0x39;
pub const NAU8821_R3A_DAC_DRC_KNEE_IP12: c_uint = 0x3a;
pub const NAU8821_R3B_DAC_DRC_KNEE_IP34: c_uint = 0x3b;
pub const NAU8821_R3C_DAC_DRC_SLOPES: c_uint = 0x3c;
pub const NAU8821_R3D_DAC_DRC_ATKDCY: c_uint = 0x3d;
pub const NAU8821_R41_BIQ1_COF1: c_uint = 0x41;
pub const NAU8821_R42_BIQ1_COF2: c_uint = 0x42;
pub const NAU8821_R43_BIQ1_COF3: c_uint = 0x43;
pub const NAU8821_R44_BIQ1_COF4: c_uint = 0x44;
pub const NAU8821_R45_BIQ1_COF5: c_uint = 0x45;
pub const NAU8821_R46_BIQ1_COF6: c_uint = 0x46;
pub const NAU8821_R47_BIQ1_COF7: c_uint = 0x47;
pub const NAU8821_R48_BIQ1_COF8: c_uint = 0x48;
pub const NAU8821_R49_BIQ1_COF9: c_uint = 0x49;
pub const NAU8821_R4A_BIQ1_COF10: c_uint = 0x4a;
pub const NAU8821_R4B_CLASSG_CTRL: c_uint = 0x4b;
pub const NAU8821_R4C_IMM_MODE_CTRL: c_uint = 0x4c;
pub const NAU8821_R4D_IMM_RMS_L: c_uint = 0x4d;
pub const NAU8821_R4E_FUSE_CTRL2: c_uint = 0x4e;
pub const NAU8821_R4F_FUSE_CTRL3: c_uint = 0x4f;
pub const NAU8821_R51_FUSE_CTRL1: c_uint = 0x51;
pub const NAU8821_R53_OTPDOUT_1: c_uint = 0x53;
pub const NAU8821_R54_OTPDOUT_2: c_uint = 0x54;
pub const NAU8821_R55_MISC_CTRL: c_uint = 0x55;
pub const NAU8821_R58_I2C_DEVICE_ID: c_uint = 0x58;
pub const NAU8821_R59_SARDOUT_RAM_STATUS: c_uint = 0x59;
pub const NAU8821_R5A_SOFTWARE_RST: c_uint = 0x5a;
pub const NAU8821_R66_BIAS_ADJ: c_uint = 0x66;
pub const NAU8821_R68_TRIM_SETTINGS: c_uint = 0x68;
pub const NAU8821_R69_ANALOG_CONTROL_1: c_uint = 0x69;
pub const NAU8821_R6A_ANALOG_CONTROL_2: c_uint = 0x6a;
pub const NAU8821_R6B_PGA_MUTE: c_uint = 0x6b;
pub const NAU8821_R71_ANALOG_ADC_1: c_uint = 0x71;
pub const NAU8821_R72_ANALOG_ADC_2: c_uint = 0x72;
pub const NAU8821_R73_RDAC: c_uint = 0x73;
pub const NAU8821_R74_MIC_BIAS: c_uint = 0x74;
pub const NAU8821_R76_BOOST: c_uint = 0x76;
pub const NAU8821_R77_FEPGA: c_uint = 0x77;
pub const NAU8821_R7E_PGA_GAIN: c_uint = 0x7e;
pub const NAU8821_R7F_POWER_UP_CONTROL: c_uint = 0x7f;
pub const NAU8821_R80_CHARGE_PUMP: c_uint = 0x80;
pub const NAU8821_R81_CHARGE_PUMP_INPUT_READ: c_uint = 0x81;
pub const NAU8821_R82_GENERAL_STATUS: c_uint = 0x82;

// 16-bit control register address, and 16-bits control register data
pub const NAU8821_REG_ADDR_LEN: c_int = 16;
pub const NAU8821_REG_DATA_LEN: c_int = 16;
// ENA_CTRL (0x01)
pub const NAU8821_CLK_DAC_INV_SFT: c_int = 14;

pub const NAU8821_EN_DACR_SFT: c_int = 11;

pub const NAU8821_EN_DACL_SFT: c_int = 10;

pub const NAU8821_EN_ADCR_SFT: c_int = 9;

pub const NAU8821_EN_ADCL_SFT: c_int = 8;

pub const NAU8821_EN_ADC_CLK_SFT: c_int = 7;

pub const NAU8821_EN_DAC_CLK_SFT: c_int = 6;

pub const NAU8821_EN_I2S_CLK_SFT: c_int = 4;

pub const NAU8821_EN_DRC_CLK_SFT: c_int = 0;

// CLK_DIVIDER (0x03)
pub const NAU8821_CLK_SRC_SFT: c_int = 15;

pub const NAU8821_CLK_CODEC_SRC_SFT: c_int = 13;

pub const NAU8821_CLK_ADC_SRC_SFT: c_int = 6;

pub const NAU8821_CLK_DAC_SRC_SFT: c_int = 4;

pub const NAU8821_CLK_MCLK_SRC_MASK: c_uint = 0xf;
// FLL1 (0x04)
pub const NAU8821_ICTRL_LATCH_SFT: c_int = 10;

pub const NAU8821_FLL_RATIO_MASK: c_uint = 0x7f;
// FLL3 (0x06)
pub const NAU8821_GAIN_ERR_SFT: c_int = 12;

pub const NAU8821_FLL_CLK_SRC_SFT: c_int = 10;

pub const NAU8821_FLL_INTEGER_MASK: c_uint = 0x3ff;
// FLL4 (0x07)
pub const NAU8821_HIGHBW_EN_SFT: c_int = 15;

pub const NAU8821_FLL_REF_DIV_SFT: c_int = 10;

// FLL5 (0x08)

pub const NAU8821_FLL_CLK_SW_SFT: c_int = 13;

pub const NAU8821_FLL_FTR_SW_SFT: c_int = 12;

// FLL6 (0x09)

// FLL7 (0x0a)
pub const NAU8821_FLL_FRACH_MASK: c_uint = 0xff;
// FLL8 (0x0b)
pub const NAU8821_FLL_FRACL_MASK: c_uint = 0xffff;
// JACK_DET_CTRL (0x0d)
// 0 - open, 1 - short to GND
pub const NAU8821_SPKR_DWN1R_SFT: c_int = 15;

pub const NAU8821_SPKR_DWN1L_SFT: c_int = 14;

pub const NAU8821_JACK_INSERT_DEBOUNCE_SFT: c_int = 5;

pub const NAU8821_JACK_EJECT_DEBOUNCE_SFT: c_int = 2;

// INTERRUPT_MASK (0x0f)

pub const NAU8821_IRQ_INSERT_EN: c_uint = 0x1;
// IRQ_STATUS (0x10)

pub const NAU8821_KEY_IRQ_SFT: c_int = 6;

pub const NAU8821_JACK_INSERT_IRQ_MASK: c_uint = 0x3;
pub const NAU8821_JACK_INSERT_DETECTED: c_uint = 0x1;
// INTERRUPT_DIS_CTRL (0x12)

pub const NAU8821_IRQ_INSERT_DIS: c_uint = 0x1;
// DMIC_CTRL (0x13)
pub const NAU8821_DMIC_DS_SFT: c_int = 7;

pub const NAU8821_DMIC_SRC_SFT: c_int = 1;

pub const NAU8821_DMIC_EN_SFT: c_int = 0;
pub const NAU8821_DMIC_SLEW_SFT: c_int = 8;

// GPIO12_CTRL (0x1a)

// TDM_CTRL (0x1b)
pub const NAU8821_TDM_EN_SFT: c_int = 15;

pub const NAU8821_ADCPHS_SFT: c_int = 13;
pub const NAU8821_DACL_CH_SFT: c_int = 7;

pub const NAU8821_DACR_CH_SFT: c_int = 4;

pub const NAU8821_ADCL_CH_SFT: c_int = 2;

pub const NAU8821_ADCR_CH_SFT: c_int = 0;
pub const NAU8821_ADCR_CH_MASK: c_uint = 0x3;
// I2S_PCM_CTRL1 (0x1c)
pub const NAU8821_I2S_BP_SFT: c_int = 7;

pub const NAU8821_I2S_PCMB_SFT: c_int = 6;

pub const NAU8821_I2S_DL_SFT: c_int = 2;

pub const NAU8821_I2S_DF_MASK: c_uint = 0x3;
pub const NAU8821_I2S_DF_PCM_AB: c_uint = 0x3;
pub const NAU8821_I2S_DF_I2S: c_uint = 0x2;
pub const NAU8821_I2S_DF_LEFT: c_uint = 0x1;
pub const NAU8821_I2S_DF_RIGTH: c_uint = 0x0;
// I2S_PCM_CTRL2 (0x1d)
pub const NAU8821_I2S_TRISTATE_SFT: c_int = 15;

pub const NAU8821_I2S_LRC_DIV_SFT: c_int = 12;

pub const NAU8821_I2S_MS_SFT: c_int = 3;

pub const NAU8821_I2S_BLK_DIV_MASK: c_uint = 0x7;
// LEFT_TIME_SLOT (0x1e)
pub const NAU8821_TSLOT_L_OFFSET_MASK: c_uint = 0x3ff;

// RIGHT_TIME_SLOT (0x1f)
pub const NAU8821_TSLOT_R_OFFSET_MASK: c_uint = 0x3ff;
// BIQ0_COF10 (0x2a)
pub const NAU8821_BIQ0_ADC_EN_SFT: c_int = 3;

// ADC_RATE (0x2b)
pub const NAU8821_ADC_SYNC_DOWN_SFT: c_int = 0;
pub const NAU8821_ADC_SYNC_DOWN_MASK: c_uint = 0x3;
pub const NAU8821_ADC_SYNC_DOWN_256: c_uint = 0x3;
pub const NAU8821_ADC_SYNC_DOWN_128: c_uint = 0x2;
pub const NAU8821_ADC_SYNC_DOWN_64: c_uint = 0x1;
pub const NAU8821_ADC_SYNC_DOWN_32: c_uint = 0x0;
pub const NAU8821_ADC_L_SRC_SFT: c_int = 15;

pub const NAU8821_ADC_R_SRC_SFT: c_int = 14;

// DAC_CTRL1 (0x2c)
pub const NAU8821_DAC_OVERSAMPLE_SFT: c_int = 0;
pub const NAU8821_DAC_OVERSAMPLE_MASK: c_uint = 0x7;
pub const NAU8821_DAC_OVERSAMPLE_32: c_uint = 0x4;
pub const NAU8821_DAC_OVERSAMPLE_128: c_uint = 0x2;
pub const NAU8821_DAC_OVERSAMPLE_256: c_uint = 0x1;
pub const NAU8821_DAC_OVERSAMPLE_64: c_uint = 0x0;
// DAC_DGAIN_CTRL (0x2f)
pub const NAU8821_DAC1_TO_DAC0_ST_SFT: c_int = 8;

pub const NAU8821_DAC0_TO_DAC1_ST_SFT: c_int = 0;
pub const NAU8821_DAC0_TO_DAC1_ST_MASK: c_uint = 0xff;
// MUTE_CTRL (0x31)

// HSVOL_CTRL (0x32)

pub const NAU8821_HPL_VOL_SFT: c_int = 4;

pub const NAU8821_HPR_VOL_SFT: c_int = 0;

// DACR_CTRL (0x34)
pub const NAU8821_DACR_CH_VOL_SFT: c_int = 8;

pub const NAU8821_DACL_CH_VOL_SFT: c_int = 0;
pub const NAU8821_DACL_CH_VOL_MASK: c_uint = 0xff;
// ADC_DGAIN_CTRL1 (0x35)
pub const NAU8821_ADCR_CH_VOL_SFT: c_int = 8;

pub const NAU8821_ADCL_CH_VOL_SFT: c_int = 0;
pub const NAU8821_ADCL_CH_VOL_MASK: c_uint = 0xff;
// ADC_DRC_KNEE_IP12 (0x36)
pub const NAU8821_DRC_ENA_ADC_SFT: c_int = 15;

// ADC_DRC_KNEE_IP34 (0x37)
pub const NAU8821_DRC_KNEE4_IP_ADC_SFT: c_int = 8;

pub const NAU8821_DRC_KNEE3_IP_ADC_SFT: c_int = 0;
pub const NAU8821_DRC_KNEE3_IP_ADC_MASK: c_uint = 0xff;
// ADC_DRC_SLOPES (0x38)
pub const NAU8821_DRC_NG_SLP_ADC_SFT: c_int = 12;
pub const NAU8821_DRC_EXP_SLP_ADC_SFT: c_int = 9;
pub const NAU8821_DRC_CMP2_SLP_ADC_SFT: c_int = 6;
pub const NAU8821_DRC_CMP1_SLP_ADC_SFT: c_int = 3;
pub const NAU8821_DRC_LMT_SLP_ADC_SFT: c_int = 0;
// ADC_DRC_ATKDCY (0x39)
pub const NAU8821_DRC_PK_COEF1_ADC_SFT: c_int = 12;
pub const NAU8821_DRC_PK_COEF2_ADC_SFT: c_int = 8;
pub const NAU8821_DRC_ATK_ADC_SFT: c_int = 4;
pub const NAU8821_DRC_DCY_ADC_SFT: c_int = 0;
// BIQ1_COF10 (0x4a)
pub const NAU8821_BIQ1_DAC_EN_SFT: c_int = 3;

// CLASSG_CTRL (0x4b)
pub const NAU8821_CLASSG_TIMER_SFT: c_int = 8;

pub const NAU8821_CLASSG_RDAC_EN_SFT: c_int = 2;

pub const NAU8821_CLASSG_LDAC_EN_SFT: c_int = 1;

pub const NAU8821_CLASSG_EN_SFT: c_int = 0;
pub const NAU8821_CLASSG_EN: c_uint = 0x1;
// IMM_MODE_CTRL (0x4c)
pub const NAU8821_IMM_THD_SFT: c_int = 8;

pub const NAU8821_IMM_GEN_VOL_SFT: c_int = 6;

pub const NAU8821_IMM_CYC_SFT: c_int = 4;

pub const NAU8821_IMM_DAC_SRC_MASK: c_uint = 0x3;
// I2C_DEVICE_ID (0x58)

pub const NAU8821_SOFTWARE_ID_MASK: c_uint = 0x3;
// BIAS_ADJ (0x66)

pub const NAU8821_BIAS_TESTDAC_SFT: c_int = 8;

pub const NAU8821_BIAS_VMID_SEL_SFT: c_int = 4;

// ANALOG_CONTROL_1 (0x69)
pub const NAU8821_JD_POL_SFT: c_int = 2;

pub const NAU8821_JD_OUT_POL_SFT: c_int = 1;

pub const NAU8821_JD_EN_SFT: c_int = 0;
pub const NAU8821_JD_EN: c_uint = 0x1;
// ANALOG_CONTROL_2 (0x6a)

pub const NAU8821_DAC_CAPACITOR_LSB: c_uint = 0x1;
// MUTE_MIC_L_N (0x6b)
pub const NAU8821_MUTE_MICNL_SFT: c_int = 5;

pub const NAU8821_MUTE_MICNR_SFT: c_int = 4;

pub const NAU8821_MUTE_MICRP_SFT: c_int = 2;

// ANALOG_ADC_1 (0x71)
pub const NAU8821_MICDET_EN_SFT: c_int = 0;
pub const NAU8821_MICDET_MASK: c_uint = 0x1;
pub const NAU8821_MICDET_DIS: c_uint = 0x1;
pub const NAU8821_MICDET_EN: c_uint = 0x0;
// ANALOG_ADC_2 (0x72)
pub const NAU8821_ADC_VREFSEL_SFT: c_int = 8;

pub const NAU8821_POWERUP_ADCL_SFT: c_int = 6;

pub const NAU8821_POWERUP_ADCR_SFT: c_int = 4;

// RDAC (0x73)
pub const NAU8821_DACR_EN_SFT: c_int = 13;

pub const NAU8821_DACL_EN_SFT: c_int = 12;

pub const NAU8821_DACR_CLK_EN_SFT: c_int = 9;

pub const NAU8821_DACL_CLK_EN_SFT: c_int = 8;

pub const NAU8821_DAC_CLK_DELAY_SFT: c_int = 4;

pub const NAU8821_DAC_VREF_SFT: c_int = 2;

// MIC_BIAS (0x74)

pub const NAU8821_MICBIAS_LOWNOISE_SFT: c_int = 10;

pub const NAU8821_MICBIAS_POWERUP_SFT: c_int = 8;

pub const NAU8821_MICBIAS_VOLTAGE_SFT: c_int = 0;
pub const NAU8821_MICBIAS_VOLTAGE_MASK: c_uint = 0x7;
// BOOST (0x76)

pub const NAU8821_HP_BOOST_DISCHRG_SFT: c_int = 11;

pub const NAU8821_HP_BOOST_DIS_SFT: c_int = 9;

// FEPGA (0x77)
pub const NAU8821_ACDC_CTRL_SFT: c_int = 14;

pub const NAU8821_FEPGA_MODEL_SFT: c_int = 4;

pub const NAU8821_FEPGA_MODER_SFT: c_int = 0;
pub const NAU8821_FEPGA_MODER_MASK: c_uint = 0xf;
pub const NAU8821_FEPGA_MODER_AAF: c_uint = 0x1;
pub const NAU8821_FEPGA_MODER_DIS: c_uint = 0x2;
pub const NAU8821_FEPGA_MODER_IMP12K: c_uint = 0x8;
// PGA_GAIN (0x7e)
pub const NAU8821_PGA_GAIN_L_SFT: c_int = 8;

pub const NAU8821_PGA_GAIN_R_SFT: c_int = 0;
pub const NAU8821_PGA_GAIN_R_MASK: c_uint = 0x3f;
// POWER_UP_CONTROL (0x7f)
pub const NAU8821_PUP_PGA_L_SFT: c_int = 15;

pub const NAU8821_PUP_PGA_R_SFT: c_int = 14;

pub const NAU8821_PUP_INTEG_R_SFT: c_int = 5;

pub const NAU8821_PUP_INTEG_L_SFT: c_int = 4;

pub const NAU8821_PUP_DRV_INSTG_R_SFT: c_int = 3;

pub const NAU8821_PUP_DRV_INSTG_L_SFT: c_int = 2;

pub const NAU8821_PUP_MAIN_DRV_R_SFT: c_int = 1;

pub const NAU8821_PUP_MAIN_DRV_L_SFT: c_int = 0;
pub const NAU8821_PUP_MAIN_DRV_L: c_uint = 0x1;
// CHARGE_PUMP (0x80)

pub const NAU8821_POWER_DOWN_DACR_SFT: c_int = 9;

pub const NAU8821_POWER_DOWN_DACL_SFT: c_int = 8;

pub const NAU8821_CHANRGE_PUMP_EN_SFT: c_int = 5;

// GENERAL_STATUS (0x82)
pub const NAU8821_GPIO2_IN_SFT: c_int = 1;

// System Clock Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8821 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub dapm: *mut snd_soc_dapm_context,
    pub jack: *mut snd_soc_jack,
    pub jdet_work: delayed_work,
    pub jdet_active: bool,
    pub irq: c_int,
    pub clk_id: c_int,
    pub micbias_voltage: c_int,
    pub vref_impedance: c_int,
    pub jkdet_enable: bool,
    pub jkdet_pull_enable: bool,
    pub jkdet_pull_up: bool,
    pub left_input_single_end: bool,
    pub jkdet_polarity: c_int,
    pub jack_insert_debounce: c_int,
    pub jack_eject_debounce: c_int,
    pub fs: c_int,
    pub dmic_clk_threshold: c_int,
    pub dmic_slew_rate: c_int,
    pub key_enable: c_int,
    pub adc_delay: c_int,
}
