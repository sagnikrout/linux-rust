//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5677.h
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
// rt5677.h  --  RT5677 ALSA SoC audio driver
//
// Copyright 2013 Realtek Semiconductor Corp.
// Author: Oder Chiou <oder_chiou@realtek.com>
//

// Info
pub const RT5677_RESET: c_uint = 0x00;
pub const RT5677_VENDOR_ID: c_uint = 0xfd;
pub const RT5677_VENDOR_ID1: c_uint = 0xfe;
pub const RT5677_VENDOR_ID2: c_uint = 0xff;
// I/O - Output
pub const RT5677_LOUT1: c_uint = 0x01;
// I/O - Input
pub const RT5677_IN1: c_uint = 0x03;
pub const RT5677_MICBIAS: c_uint = 0x04;
// I/O - SLIMBus
pub const RT5677_SLIMBUS_PARAM: c_uint = 0x07;
pub const RT5677_SLIMBUS_RX: c_uint = 0x08;
pub const RT5677_SLIMBUS_CTRL: c_uint = 0x09;
// I/O
pub const RT5677_SIDETONE_CTRL: c_uint = 0x13;
// I/O - ADC/DAC
pub const RT5677_ANA_DAC1_2_3_SRC: c_uint = 0x15;
pub const RT5677_IF_DSP_DAC3_4_MIXER: c_uint = 0x16;
pub const RT5677_DAC4_DIG_VOL: c_uint = 0x17;
pub const RT5677_DAC3_DIG_VOL: c_uint = 0x18;
pub const RT5677_DAC1_DIG_VOL: c_uint = 0x19;
pub const RT5677_DAC2_DIG_VOL: c_uint = 0x1a;
pub const RT5677_IF_DSP_DAC2_MIXER: c_uint = 0x1b;
pub const RT5677_STO1_ADC_DIG_VOL: c_uint = 0x1c;
pub const RT5677_MONO_ADC_DIG_VOL: c_uint = 0x1d;
pub const RT5677_STO1_2_ADC_BST: c_uint = 0x1e;
pub const RT5677_STO2_ADC_DIG_VOL: c_uint = 0x1f;
// Mixer - D-D
pub const RT5677_ADC_BST_CTRL2: c_uint = 0x20;
pub const RT5677_STO3_4_ADC_BST: c_uint = 0x21;
pub const RT5677_STO3_ADC_DIG_VOL: c_uint = 0x22;
pub const RT5677_STO4_ADC_DIG_VOL: c_uint = 0x23;
pub const RT5677_STO4_ADC_MIXER: c_uint = 0x24;
pub const RT5677_STO3_ADC_MIXER: c_uint = 0x25;
pub const RT5677_STO2_ADC_MIXER: c_uint = 0x26;
pub const RT5677_STO1_ADC_MIXER: c_uint = 0x27;
pub const RT5677_MONO_ADC_MIXER: c_uint = 0x28;
pub const RT5677_ADC_IF_DSP_DAC1_MIXER: c_uint = 0x29;
pub const RT5677_STO1_DAC_MIXER: c_uint = 0x2a;
pub const RT5677_MONO_DAC_MIXER: c_uint = 0x2b;
pub const RT5677_DD1_MIXER: c_uint = 0x2c;
pub const RT5677_DD2_MIXER: c_uint = 0x2d;
pub const RT5677_IF3_DATA: c_uint = 0x2f;
pub const RT5677_IF4_DATA: c_uint = 0x30;
// Mixer - PDM
pub const RT5677_PDM_OUT_CTRL: c_uint = 0x31;
pub const RT5677_PDM_DATA_CTRL1: c_uint = 0x32;
pub const RT5677_PDM_DATA_CTRL2: c_uint = 0x33;
pub const RT5677_PDM1_DATA_CTRL2: c_uint = 0x34;
pub const RT5677_PDM1_DATA_CTRL3: c_uint = 0x35;
pub const RT5677_PDM1_DATA_CTRL4: c_uint = 0x36;
pub const RT5677_PDM2_DATA_CTRL2: c_uint = 0x37;
pub const RT5677_PDM2_DATA_CTRL3: c_uint = 0x38;
pub const RT5677_PDM2_DATA_CTRL4: c_uint = 0x39;
// TDM
pub const RT5677_TDM1_CTRL1: c_uint = 0x3b;
pub const RT5677_TDM1_CTRL2: c_uint = 0x3c;
pub const RT5677_TDM1_CTRL3: c_uint = 0x3d;
pub const RT5677_TDM1_CTRL4: c_uint = 0x3e;
pub const RT5677_TDM1_CTRL5: c_uint = 0x3f;
pub const RT5677_TDM2_CTRL1: c_uint = 0x40;
pub const RT5677_TDM2_CTRL2: c_uint = 0x41;
pub const RT5677_TDM2_CTRL3: c_uint = 0x42;
pub const RT5677_TDM2_CTRL4: c_uint = 0x43;
pub const RT5677_TDM2_CTRL5: c_uint = 0x44;
// I2C_MASTER_CTRL
pub const RT5677_I2C_MASTER_CTRL1: c_uint = 0x47;
pub const RT5677_I2C_MASTER_CTRL2: c_uint = 0x48;
pub const RT5677_I2C_MASTER_CTRL3: c_uint = 0x49;
pub const RT5677_I2C_MASTER_CTRL4: c_uint = 0x4a;
pub const RT5677_I2C_MASTER_CTRL5: c_uint = 0x4b;
pub const RT5677_I2C_MASTER_CTRL6: c_uint = 0x4c;
pub const RT5677_I2C_MASTER_CTRL7: c_uint = 0x4d;
pub const RT5677_I2C_MASTER_CTRL8: c_uint = 0x4e;
// DMIC
pub const RT5677_DMIC_CTRL1: c_uint = 0x50;
pub const RT5677_DMIC_CTRL2: c_uint = 0x51;
// Haptic Generator
pub const RT5677_HAP_GENE_CTRL1: c_uint = 0x56;
pub const RT5677_HAP_GENE_CTRL2: c_uint = 0x57;
pub const RT5677_HAP_GENE_CTRL3: c_uint = 0x58;
pub const RT5677_HAP_GENE_CTRL4: c_uint = 0x59;
pub const RT5677_HAP_GENE_CTRL5: c_uint = 0x5a;
pub const RT5677_HAP_GENE_CTRL6: c_uint = 0x5b;
pub const RT5677_HAP_GENE_CTRL7: c_uint = 0x5c;
pub const RT5677_HAP_GENE_CTRL8: c_uint = 0x5d;
pub const RT5677_HAP_GENE_CTRL9: c_uint = 0x5e;
pub const RT5677_HAP_GENE_CTRL10: c_uint = 0x5f;
// Power
pub const RT5677_PWR_DIG1: c_uint = 0x61;
pub const RT5677_PWR_DIG2: c_uint = 0x62;
pub const RT5677_PWR_ANLG1: c_uint = 0x63;
pub const RT5677_PWR_ANLG2: c_uint = 0x64;
pub const RT5677_PWR_DSP1: c_uint = 0x65;
pub const RT5677_PWR_DSP_ST: c_uint = 0x66;
pub const RT5677_PWR_DSP2: c_uint = 0x67;
pub const RT5677_ADC_DAC_HPF_CTRL1: c_uint = 0x68;
// Private Register Control
pub const RT5677_PRIV_INDEX: c_uint = 0x6a;
pub const RT5677_PRIV_DATA: c_uint = 0x6c;
// Format - ADC/DAC
pub const RT5677_I2S4_SDP: c_uint = 0x6f;
pub const RT5677_I2S1_SDP: c_uint = 0x70;
pub const RT5677_I2S2_SDP: c_uint = 0x71;
pub const RT5677_I2S3_SDP: c_uint = 0x72;
pub const RT5677_CLK_TREE_CTRL1: c_uint = 0x73;
pub const RT5677_CLK_TREE_CTRL2: c_uint = 0x74;
pub const RT5677_CLK_TREE_CTRL3: c_uint = 0x75;
// Function - Analog
pub const RT5677_PLL1_CTRL1: c_uint = 0x7a;
pub const RT5677_PLL1_CTRL2: c_uint = 0x7b;
pub const RT5677_PLL2_CTRL1: c_uint = 0x7c;
pub const RT5677_PLL2_CTRL2: c_uint = 0x7d;
pub const RT5677_GLB_CLK1: c_uint = 0x80;
pub const RT5677_GLB_CLK2: c_uint = 0x81;
pub const RT5677_ASRC_1: c_uint = 0x83;
pub const RT5677_ASRC_2: c_uint = 0x84;
pub const RT5677_ASRC_3: c_uint = 0x85;
pub const RT5677_ASRC_4: c_uint = 0x86;
pub const RT5677_ASRC_5: c_uint = 0x87;
pub const RT5677_ASRC_6: c_uint = 0x88;
pub const RT5677_ASRC_7: c_uint = 0x89;
pub const RT5677_ASRC_8: c_uint = 0x8a;
pub const RT5677_ASRC_9: c_uint = 0x8b;
pub const RT5677_ASRC_10: c_uint = 0x8c;
pub const RT5677_ASRC_11: c_uint = 0x8d;
pub const RT5677_ASRC_12: c_uint = 0x8e;
pub const RT5677_ASRC_13: c_uint = 0x8f;
pub const RT5677_ASRC_14: c_uint = 0x90;
pub const RT5677_ASRC_15: c_uint = 0x91;
pub const RT5677_ASRC_16: c_uint = 0x92;
pub const RT5677_ASRC_17: c_uint = 0x93;
pub const RT5677_ASRC_18: c_uint = 0x94;
pub const RT5677_ASRC_19: c_uint = 0x95;
pub const RT5677_ASRC_20: c_uint = 0x97;
pub const RT5677_ASRC_21: c_uint = 0x98;
pub const RT5677_ASRC_22: c_uint = 0x99;
pub const RT5677_ASRC_23: c_uint = 0x9a;
pub const RT5677_VAD_CTRL1: c_uint = 0x9c;
pub const RT5677_VAD_CTRL2: c_uint = 0x9d;
pub const RT5677_VAD_CTRL3: c_uint = 0x9e;
pub const RT5677_VAD_CTRL4: c_uint = 0x9f;
pub const RT5677_VAD_CTRL5: c_uint = 0xa0;
// Function - Digital
pub const RT5677_DSP_INB_CTRL1: c_uint = 0xa3;
pub const RT5677_DSP_INB_CTRL2: c_uint = 0xa4;
pub const RT5677_DSP_IN_OUTB_CTRL: c_uint = 0xa5;
pub const RT5677_DSP_OUTB0_1_DIG_VOL: c_uint = 0xa6;
pub const RT5677_DSP_OUTB2_3_DIG_VOL: c_uint = 0xa7;
pub const RT5677_DSP_OUTB4_5_DIG_VOL: c_uint = 0xa8;
pub const RT5677_DSP_OUTB6_7_DIG_VOL: c_uint = 0xa9;
pub const RT5677_ADC_EQ_CTRL1: c_uint = 0xae;
pub const RT5677_ADC_EQ_CTRL2: c_uint = 0xaf;
pub const RT5677_EQ_CTRL1: c_uint = 0xb0;
pub const RT5677_EQ_CTRL2: c_uint = 0xb1;
pub const RT5677_EQ_CTRL3: c_uint = 0xb2;
pub const RT5677_SOFT_VOL_ZERO_CROSS1: c_uint = 0xb3;
pub const RT5677_JD_CTRL1: c_uint = 0xb5;
pub const RT5677_JD_CTRL2: c_uint = 0xb6;
pub const RT5677_JD_CTRL3: c_uint = 0xb8;
pub const RT5677_IRQ_CTRL1: c_uint = 0xbd;
pub const RT5677_IRQ_CTRL2: c_uint = 0xbe;
pub const RT5677_GPIO_ST: c_uint = 0xbf;
pub const RT5677_GPIO_CTRL1: c_uint = 0xc0;
pub const RT5677_GPIO_CTRL2: c_uint = 0xc1;
pub const RT5677_GPIO_CTRL3: c_uint = 0xc2;
pub const RT5677_STO1_ADC_HI_FILTER1: c_uint = 0xc5;
pub const RT5677_STO1_ADC_HI_FILTER2: c_uint = 0xc6;
pub const RT5677_MONO_ADC_HI_FILTER1: c_uint = 0xc7;
pub const RT5677_MONO_ADC_HI_FILTER2: c_uint = 0xc8;
pub const RT5677_STO2_ADC_HI_FILTER1: c_uint = 0xc9;
pub const RT5677_STO2_ADC_HI_FILTER2: c_uint = 0xca;
pub const RT5677_STO3_ADC_HI_FILTER1: c_uint = 0xcb;
pub const RT5677_STO3_ADC_HI_FILTER2: c_uint = 0xcc;
pub const RT5677_STO4_ADC_HI_FILTER1: c_uint = 0xcd;
pub const RT5677_STO4_ADC_HI_FILTER2: c_uint = 0xce;
pub const RT5677_MB_DRC_CTRL1: c_uint = 0xd0;
pub const RT5677_DRC1_CTRL1: c_uint = 0xd2;
pub const RT5677_DRC1_CTRL2: c_uint = 0xd3;
pub const RT5677_DRC1_CTRL3: c_uint = 0xd4;
pub const RT5677_DRC1_CTRL4: c_uint = 0xd5;
pub const RT5677_DRC1_CTRL5: c_uint = 0xd6;
pub const RT5677_DRC1_CTRL6: c_uint = 0xd7;
pub const RT5677_DRC2_CTRL1: c_uint = 0xd8;
pub const RT5677_DRC2_CTRL2: c_uint = 0xd9;
pub const RT5677_DRC2_CTRL3: c_uint = 0xda;
pub const RT5677_DRC2_CTRL4: c_uint = 0xdb;
pub const RT5677_DRC2_CTRL5: c_uint = 0xdc;
pub const RT5677_DRC2_CTRL6: c_uint = 0xdd;
pub const RT5677_DRC1_HL_CTRL1: c_uint = 0xde;
pub const RT5677_DRC1_HL_CTRL2: c_uint = 0xdf;
pub const RT5677_DRC2_HL_CTRL1: c_uint = 0xe0;
pub const RT5677_DRC2_HL_CTRL2: c_uint = 0xe1;
pub const RT5677_DSP_INB1_SRC_CTRL1: c_uint = 0xe3;
pub const RT5677_DSP_INB1_SRC_CTRL2: c_uint = 0xe4;
pub const RT5677_DSP_INB1_SRC_CTRL3: c_uint = 0xe5;
pub const RT5677_DSP_INB1_SRC_CTRL4: c_uint = 0xe6;
pub const RT5677_DSP_INB2_SRC_CTRL1: c_uint = 0xe7;
pub const RT5677_DSP_INB2_SRC_CTRL2: c_uint = 0xe8;
pub const RT5677_DSP_INB2_SRC_CTRL3: c_uint = 0xe9;
pub const RT5677_DSP_INB2_SRC_CTRL4: c_uint = 0xea;
pub const RT5677_DSP_INB3_SRC_CTRL1: c_uint = 0xeb;
pub const RT5677_DSP_INB3_SRC_CTRL2: c_uint = 0xec;
pub const RT5677_DSP_INB3_SRC_CTRL3: c_uint = 0xed;
pub const RT5677_DSP_INB3_SRC_CTRL4: c_uint = 0xee;
pub const RT5677_DSP_OUTB1_SRC_CTRL1: c_uint = 0xef;
pub const RT5677_DSP_OUTB1_SRC_CTRL2: c_uint = 0xf0;
pub const RT5677_DSP_OUTB1_SRC_CTRL3: c_uint = 0xf1;
pub const RT5677_DSP_OUTB1_SRC_CTRL4: c_uint = 0xf2;
pub const RT5677_DSP_OUTB2_SRC_CTRL1: c_uint = 0xf3;
pub const RT5677_DSP_OUTB2_SRC_CTRL2: c_uint = 0xf4;
pub const RT5677_DSP_OUTB2_SRC_CTRL3: c_uint = 0xf5;
pub const RT5677_DSP_OUTB2_SRC_CTRL4: c_uint = 0xf6;
// Virtual DSP Mixer Control
pub const RT5677_DSP_OUTB_0123_MIXER_CTRL: c_uint = 0xf7;
pub const RT5677_DSP_OUTB_45_MIXER_CTRL: c_uint = 0xf8;
pub const RT5677_DSP_OUTB_67_MIXER_CTRL: c_uint = 0xf9;
// General Control
pub const RT5677_DIG_MISC: c_uint = 0xfa;
pub const RT5677_GEN_CTRL1: c_uint = 0xfb;
pub const RT5677_GEN_CTRL2: c_uint = 0xfc;
// DSP Mode I2C Control
pub const RT5677_DSP_I2C_OP_CODE: c_uint = 0x00;
pub const RT5677_DSP_I2C_ADDR_LSB: c_uint = 0x01;
pub const RT5677_DSP_I2C_ADDR_MSB: c_uint = 0x02;
pub const RT5677_DSP_I2C_DATA_LSB: c_uint = 0x03;
pub const RT5677_DSP_I2C_DATA_MSB: c_uint = 0x04;
// Index of Codec Private Register definition
pub const RT5677_PR_DRC1_CTRL_1: c_uint = 0x01;
pub const RT5677_PR_DRC1_CTRL_2: c_uint = 0x02;
pub const RT5677_PR_DRC1_CTRL_3: c_uint = 0x03;
pub const RT5677_PR_DRC1_CTRL_4: c_uint = 0x04;
pub const RT5677_PR_DRC1_CTRL_5: c_uint = 0x05;
pub const RT5677_PR_DRC1_CTRL_6: c_uint = 0x06;
pub const RT5677_PR_DRC1_CTRL_7: c_uint = 0x07;
pub const RT5677_PR_DRC2_CTRL_1: c_uint = 0x08;
pub const RT5677_PR_DRC2_CTRL_2: c_uint = 0x09;
pub const RT5677_PR_DRC2_CTRL_3: c_uint = 0x0a;
pub const RT5677_PR_DRC2_CTRL_4: c_uint = 0x0b;
pub const RT5677_PR_DRC2_CTRL_5: c_uint = 0x0c;
pub const RT5677_PR_DRC2_CTRL_6: c_uint = 0x0d;
pub const RT5677_PR_DRC2_CTRL_7: c_uint = 0x0e;
pub const RT5677_BIAS_CUR1: c_uint = 0x10;
pub const RT5677_BIAS_CUR2: c_uint = 0x12;
pub const RT5677_BIAS_CUR3: c_uint = 0x13;
pub const RT5677_BIAS_CUR4: c_uint = 0x14;
pub const RT5677_BIAS_CUR5: c_uint = 0x15;
pub const RT5677_VREF_LOUT_CTRL: c_uint = 0x17;
pub const RT5677_DIG_VOL_CTRL1: c_uint = 0x1a;
pub const RT5677_DIG_VOL_CTRL2: c_uint = 0x1b;
pub const RT5677_ANA_ADC_GAIN_CTRL: c_uint = 0x1e;
pub const RT5677_VAD_SRAM_TEST1: c_uint = 0x20;
pub const RT5677_VAD_SRAM_TEST2: c_uint = 0x21;
pub const RT5677_VAD_SRAM_TEST3: c_uint = 0x22;
pub const RT5677_VAD_SRAM_TEST4: c_uint = 0x23;
pub const RT5677_PAD_DRV_CTRL: c_uint = 0x26;
pub const RT5677_DIG_IN_PIN_ST_CTRL1: c_uint = 0x29;
pub const RT5677_DIG_IN_PIN_ST_CTRL2: c_uint = 0x2a;
pub const RT5677_DIG_IN_PIN_ST_CTRL3: c_uint = 0x2b;
pub const RT5677_PLL1_INT: c_uint = 0x38;
pub const RT5677_PLL2_INT: c_uint = 0x39;
pub const RT5677_TEST_CTRL1: c_uint = 0x3a;
pub const RT5677_TEST_CTRL2: c_uint = 0x3b;
pub const RT5677_TEST_CTRL3: c_uint = 0x3c;
pub const RT5677_CHOP_DAC_ADC: c_uint = 0x3d;
pub const RT5677_SOFT_DEPOP_DAC_CLK_CTRL: c_uint = 0x3e;
pub const RT5677_CROSS_OVER_FILTER1: c_uint = 0x90;
pub const RT5677_CROSS_OVER_FILTER2: c_uint = 0x91;
pub const RT5677_CROSS_OVER_FILTER3: c_uint = 0x92;
pub const RT5677_CROSS_OVER_FILTER4: c_uint = 0x93;
pub const RT5677_CROSS_OVER_FILTER5: c_uint = 0x94;
pub const RT5677_CROSS_OVER_FILTER6: c_uint = 0x95;
pub const RT5677_CROSS_OVER_FILTER7: c_uint = 0x96;
pub const RT5677_CROSS_OVER_FILTER8: c_uint = 0x97;
pub const RT5677_CROSS_OVER_FILTER9: c_uint = 0x98;
pub const RT5677_CROSS_OVER_FILTER10: c_uint = 0x99;
// global definition

pub const RT5677_L_MUTE_SFT: c_int = 15;

pub const RT5677_VOL_L_SFT: c_int = 14;

pub const RT5677_R_MUTE_SFT: c_int = 7;

pub const RT5677_VOL_R_SFT: c_int = 6;

pub const RT5677_L_VOL_SFT: c_int = 9;

pub const RT5677_R_VOL_SFT: c_int = 1;
// LOUT1 Control (0x01)

// IN1 Control (0x03)

pub const RT5677_BST_SFT1: c_int = 12;

pub const RT5677_BST_SFT2: c_int = 8;

pub const RT5677_IN_DF1_SFT: c_int = 7;

pub const RT5677_IN_DF2_SFT: c_int = 6;
// Micbias Control (0x04)

pub const RT5677_MICBIAS1_OVTH_SFT: c_int = 9;

// SLIMbus Parameter (0x07)
// SLIMbus Rx (0x08)

pub const RT5677_SLB_ADC4_SFT: c_int = 6;

pub const RT5677_SLB_ADC3_SFT: c_int = 4;

pub const RT5677_SLB_ADC2_SFT: c_int = 2;

pub const RT5677_SLB_ADC1_SFT: c_int = 0;
// SLIMBus control (0x09)
// Sidetone Control (0x13)

pub const RT5677_ST_HPF_SEL_SFT: c_int = 13;

pub const RT5677_ST_HPF_PATH_SFT: c_int = 12;

pub const RT5677_ST_SEL_SFT: c_int = 9;

pub const RT5677_ST_EN_SFT: c_int = 6;

pub const RT5677_ST_GAIN_SFT: c_int = 5;

pub const RT5677_ST_VOL_SFT: c_int = 0;
// Analog DAC1/2/3 Source Control (0x15)

pub const RT5677_ANA_DAC3_SRC_SEL_SFT: c_int = 4;

pub const RT5677_ANA_DAC1_2_SRC_SEL_SFT: c_int = 0;
// IF/DSP to DAC3/4 Mixer Control (0x16)

pub const RT5677_M_DAC4_L_VOL_SFT: c_int = 15;

pub const RT5677_SEL_DAC4_L_SRC_SFT: c_int = 12;

pub const RT5677_M_DAC4_R_VOL_SFT: c_int = 11;

pub const RT5677_SEL_DAC4_R_SRC_SFT: c_int = 8;

pub const RT5677_M_DAC3_L_VOL_SFT: c_int = 7;

pub const RT5677_SEL_DAC3_L_SRC_SFT: c_int = 4;

pub const RT5677_M_DAC3_R_VOL_SFT: c_int = 3;

pub const RT5677_SEL_DAC3_R_SRC_SFT: c_int = 0;
// DAC4 Digital Volume (0x17)

pub const RT5677_DAC4_L_VOL_SFT: c_int = 8;

pub const RT5677_DAC4_R_VOL_SFT: c_int = 0;
// DAC3 Digital Volume (0x18)

pub const RT5677_DAC3_L_VOL_SFT: c_int = 8;

pub const RT5677_DAC3_R_VOL_SFT: c_int = 0;
// DAC1 Digital Volume (0x19)

pub const RT5677_DAC1_L_VOL_SFT: c_int = 8;

pub const RT5677_DAC1_R_VOL_SFT: c_int = 0;
// DAC2 Digital Volume (0x1a)

pub const RT5677_DAC2_L_VOL_SFT: c_int = 8;

pub const RT5677_DAC2_R_VOL_SFT: c_int = 0;
// IF/DSP to DAC2 Mixer Control (0x1b)

pub const RT5677_M_DAC2_L_VOL_SFT: c_int = 7;

pub const RT5677_SEL_DAC2_L_SRC_SFT: c_int = 4;

pub const RT5677_M_DAC2_R_VOL_SFT: c_int = 3;

pub const RT5677_SEL_DAC2_R_SRC_SFT: c_int = 0;
// Stereo1 ADC Digital Volume Control (0x1c)

pub const RT5677_STO1_ADC_L_VOL_SFT: c_int = 9;

pub const RT5677_STO1_ADC_R_VOL_SFT: c_int = 1;
// Mono ADC Digital Volume Control (0x1d)

pub const RT5677_MONO_ADC_L_VOL_SFT: c_int = 9;

pub const RT5677_MONO_ADC_R_VOL_SFT: c_int = 1;
// Stereo 1/2 ADC Boost Gain Control (0x1e)

pub const RT5677_STO1_ADC_L_BST_SFT: c_int = 14;

pub const RT5677_STO1_ADC_R_BST_SFT: c_int = 12;

pub const RT5677_STO1_ADC_COMP_SFT: c_int = 10;

pub const RT5677_STO2_ADC_L_BST_SFT: c_int = 8;

pub const RT5677_STO2_ADC_R_BST_SFT: c_int = 6;

pub const RT5677_STO2_ADC_COMP_SFT: c_int = 4;
// Stereo2 ADC Digital Volume Control (0x1f)

pub const RT5677_STO2_ADC_L_VOL_SFT: c_int = 8;

pub const RT5677_STO2_ADC_R_VOL_SFT: c_int = 0;
// ADC Boost Gain Control 2 (0x20)

pub const RT5677_MONO_ADC_L_BST_SFT: c_int = 14;

pub const RT5677_MONO_ADC_R_BST_SFT: c_int = 12;

pub const RT5677_MONO_ADC_COMP_SFT: c_int = 10;
// Stereo 3/4 ADC Boost Gain Control (0x21)

pub const RT5677_STO3_ADC_L_BST_SFT: c_int = 14;

pub const RT5677_STO3_ADC_R_BST_SFT: c_int = 12;

pub const RT5677_STO3_ADC_COMP_SFT: c_int = 10;

pub const RT5677_STO4_ADC_L_BST_SFT: c_int = 8;

pub const RT5677_STO4_ADC_R_BST_SFT: c_int = 6;

pub const RT5677_STO4_ADC_COMP_SFT: c_int = 4;
// Stereo3 ADC Digital Volume Control (0x22)

pub const RT5677_STO3_ADC_L_VOL_SFT: c_int = 8;

pub const RT5677_STO3_ADC_R_VOL_SFT: c_int = 0;
// Stereo4 ADC Digital Volume Control (0x23)

pub const RT5677_STO4_ADC_L_VOL_SFT: c_int = 8;

pub const RT5677_STO4_ADC_R_VOL_SFT: c_int = 0;
// Stereo4 ADC Mixer control (0x24)

pub const RT5677_M_STO4_ADC_L2_SFT: c_int = 15;

pub const RT5677_M_STO4_ADC_L1_SFT: c_int = 14;

pub const RT5677_SEL_STO4_ADC1_SFT: c_int = 12;

pub const RT5677_SEL_STO4_ADC2_SFT: c_int = 10;

pub const RT5677_SEL_STO4_DMIC_SFT: c_int = 8;

pub const RT5677_M_STO4_ADC_R1_SFT: c_int = 7;

pub const RT5677_M_STO4_ADC_R2_SFT: c_int = 6;
// Stereo3 ADC Mixer control (0x25)

pub const RT5677_M_STO3_ADC_L2_SFT: c_int = 15;

pub const RT5677_M_STO3_ADC_L1_SFT: c_int = 14;

pub const RT5677_SEL_STO3_ADC1_SFT: c_int = 12;

pub const RT5677_SEL_STO3_ADC2_SFT: c_int = 10;

pub const RT5677_SEL_STO3_DMIC_SFT: c_int = 8;

pub const RT5677_M_STO3_ADC_R1_SFT: c_int = 7;

pub const RT5677_M_STO3_ADC_R2_SFT: c_int = 6;
// Stereo2 ADC Mixer Control (0x26)

pub const RT5677_M_STO2_ADC_L2_SFT: c_int = 15;

pub const RT5677_M_STO2_ADC_L1_SFT: c_int = 14;

pub const RT5677_SEL_STO2_ADC1_SFT: c_int = 12;

pub const RT5677_SEL_STO2_ADC2_SFT: c_int = 10;

pub const RT5677_SEL_STO2_DMIC_SFT: c_int = 8;

pub const RT5677_M_STO2_ADC_R1_SFT: c_int = 7;

pub const RT5677_M_STO2_ADC_R2_SFT: c_int = 6;

pub const RT5677_SEL_STO2_LR_MIX_SFT: c_int = 0;

// Stereo1 ADC Mixer control (0x27)

pub const RT5677_M_STO1_ADC_L2_SFT: c_int = 15;

pub const RT5677_M_STO1_ADC_L1_SFT: c_int = 14;

pub const RT5677_SEL_STO1_ADC1_SFT: c_int = 12;

pub const RT5677_SEL_STO1_ADC2_SFT: c_int = 10;

pub const RT5677_SEL_STO1_DMIC_SFT: c_int = 8;

pub const RT5677_M_STO1_ADC_R1_SFT: c_int = 7;

pub const RT5677_M_STO1_ADC_R2_SFT: c_int = 6;
// Mono ADC Mixer control (0x28)

pub const RT5677_M_MONO_ADC_L2_SFT: c_int = 15;

pub const RT5677_M_MONO_ADC_L1_SFT: c_int = 14;

pub const RT5677_SEL_MONO_ADC_L1_SFT: c_int = 12;

pub const RT5677_SEL_MONO_ADC_L2_SFT: c_int = 10;

pub const RT5677_SEL_MONO_DMIC_L_SFT: c_int = 8;

pub const RT5677_M_MONO_ADC_R1_SFT: c_int = 7;

pub const RT5677_M_MONO_ADC_R2_SFT: c_int = 6;

pub const RT5677_SEL_MONO_ADC_R1_SFT: c_int = 4;

pub const RT5677_SEL_MONO_ADC_R2_SFT: c_int = 2;

pub const RT5677_SEL_MONO_DMIC_R_SFT: c_int = 0;
// ADC/IF/DSP to DAC1 Mixer control (0x29)

pub const RT5677_M_ADDA_MIXER1_L_SFT: c_int = 15;

pub const RT5677_M_DAC1_L_SFT: c_int = 14;

pub const RT5677_DAC1_L_SEL_SFT: c_int = 8;

pub const RT5677_M_ADDA_MIXER1_R_SFT: c_int = 7;

pub const RT5677_M_DAC1_R_SFT: c_int = 6;

pub const RT5677_ADDA1_SEL_SFT: c_int = 0;
// Stereo1 DAC Mixer L/R Control (0x2a)

pub const RT5677_M_ST_DAC1_L_SFT: c_int = 15;

pub const RT5677_M_DAC1_L_STO_L_SFT: c_int = 13;

pub const RT5677_DAC1_L_STO_L_VOL_SFT: c_int = 12;

pub const RT5677_M_DAC2_L_STO_L_SFT: c_int = 11;

pub const RT5677_DAC2_L_STO_L_VOL_SFT: c_int = 10;

pub const RT5677_M_DAC1_R_STO_L_SFT: c_int = 9;

pub const RT5677_DAC1_R_STO_L_VOL_SFT: c_int = 8;

pub const RT5677_M_ST_DAC1_R_SFT: c_int = 7;

pub const RT5677_M_DAC1_R_STO_R_SFT: c_int = 5;

pub const RT5677_DAC1_R_STO_R_VOL_SFT: c_int = 4;

pub const RT5677_M_DAC2_R_STO_R_SFT: c_int = 3;

pub const RT5677_DAC2_R_STO_R_VOL_SFT: c_int = 2;

pub const RT5677_M_DAC1_L_STO_R_SFT: c_int = 1;

pub const RT5677_DAC1_L_STO_R_VOL_SFT: c_int = 0;
// Mono DAC Mixer L/R Control (0x2b)

pub const RT5677_M_ST_DAC2_L_SFT: c_int = 15;

pub const RT5677_M_DAC2_L_MONO_L_SFT: c_int = 13;

pub const RT5677_DAC2_L_MONO_L_VOL_SFT: c_int = 12;

pub const RT5677_M_DAC2_R_MONO_L_SFT: c_int = 11;

pub const RT5677_DAC2_R_MONO_L_VOL_SFT: c_int = 10;

pub const RT5677_M_DAC1_L_MONO_L_SFT: c_int = 9;

pub const RT5677_DAC1_L_MONO_L_VOL_SFT: c_int = 8;

pub const RT5677_M_ST_DAC2_R_SFT: c_int = 7;

pub const RT5677_M_DAC2_R_MONO_R_SFT: c_int = 5;

pub const RT5677_DAC2_R_MONO_R_VOL_SFT: c_int = 4;

pub const RT5677_M_DAC1_R_MONO_R_SFT: c_int = 3;

pub const RT5677_DAC1_R_MONO_R_VOL_SFT: c_int = 2;

pub const RT5677_M_DAC2_L_MONO_R_SFT: c_int = 1;

pub const RT5677_DAC2_L_MONO_R_VOL_SFT: c_int = 0;
// DD Mixer 1 Control (0x2c)

pub const RT5677_M_STO_L_DD1_L_SFT: c_int = 15;

pub const RT5677_STO_L_DD1_L_VOL_SFT: c_int = 14;

pub const RT5677_M_MONO_L_DD1_L_SFT: c_int = 13;

pub const RT5677_MONO_L_DD1_L_VOL_SFT: c_int = 12;

pub const RT5677_M_DAC3_L_DD1_L_SFT: c_int = 11;

pub const RT5677_DAC3_L_DD1_L_VOL_SFT: c_int = 10;

pub const RT5677_M_DAC3_R_DD1_L_SFT: c_int = 9;

pub const RT5677_DAC3_R_DD1_L_VOL_SFT: c_int = 8;

pub const RT5677_M_STO_R_DD1_R_SFT: c_int = 7;

pub const RT5677_STO_R_DD1_R_VOL_SFT: c_int = 6;

pub const RT5677_M_MONO_R_DD1_R_SFT: c_int = 5;

pub const RT5677_MONO_R_DD1_R_VOL_SFT: c_int = 4;

pub const RT5677_M_DAC3_R_DD1_R_SFT: c_int = 3;

pub const RT5677_DAC3_R_DD1_R_VOL_SFT: c_int = 2;

pub const RT5677_M_DAC3_L_DD1_R_SFT: c_int = 1;

pub const RT5677_DAC3_L_DD1_R_VOL_SFT: c_int = 0;
// DD Mixer 2 Control (0x2d)

pub const RT5677_M_STO_L_DD2_L_SFT: c_int = 15;

pub const RT5677_STO_L_DD2_L_VOL_SFT: c_int = 14;

pub const RT5677_M_MONO_L_DD2_L_SFT: c_int = 13;

pub const RT5677_MONO_L_DD2_L_VOL_SFT: c_int = 12;

pub const RT5677_M_DAC4_L_DD2_L_SFT: c_int = 11;

pub const RT5677_DAC4_L_DD2_L_VOL_SFT: c_int = 10;

pub const RT5677_M_DAC4_R_DD2_L_SFT: c_int = 9;

pub const RT5677_DAC4_R_DD2_L_VOL_SFT: c_int = 8;

pub const RT5677_M_STO_R_DD2_R_SFT: c_int = 7;

pub const RT5677_STO_R_DD2_R_VOL_SFT: c_int = 6;

pub const RT5677_M_MONO_R_DD2_R_SFT: c_int = 5;

pub const RT5677_MONO_R_DD2_R_VOL_SFT: c_int = 4;

pub const RT5677_M_DAC4_R_DD2_R_SFT: c_int = 3;

pub const RT5677_DAC4_R_DD2_R_VOL_SFT: c_int = 2;

pub const RT5677_M_DAC4_L_DD2_R_SFT: c_int = 1;

pub const RT5677_DAC4_L_DD2_R_VOL_SFT: c_int = 0;
// IF3 data control (0x2f)

pub const RT5677_IF3_DAC_SEL_SFT: c_int = 6;

pub const RT5677_IF3_ADC_SEL_SFT: c_int = 4;

pub const RT5677_IF3_ADC_IN_SFT: c_int = 0;
// IF4 data control (0x30)

pub const RT5677_IF4_ADC_IN_SFT: c_int = 4;

pub const RT5677_IF4_DAC_SEL_SFT: c_int = 2;

pub const RT5677_IF4_ADC_SEL_SFT: c_int = 0;
// PDM Output Control (0x31)

pub const RT5677_M_PDM1_L_SFT: c_int = 15;

pub const RT5677_SEL_PDM1_L_SFT: c_int = 12;

pub const RT5677_M_PDM1_R_SFT: c_int = 11;

pub const RT5677_SEL_PDM1_R_SFT: c_int = 8;

pub const RT5677_M_PDM2_L_SFT: c_int = 7;

pub const RT5677_SEL_PDM2_L_SFT: c_int = 4;

pub const RT5677_M_PDM2_R_SFT: c_int = 3;

pub const RT5677_SEL_PDM2_R_SFT: c_int = 0;
// PDM I2C / Data Control 1 (0x32)

// PDM I2C / Data Control 2 (0x33)

// TDM1 control 1 (0x3b)

pub const RT5677_IF1_ADC_MODE_SFT: c_int = 12;

pub const RT5677_IF1_ADC1_SWAP_SFT: c_int = 6;

pub const RT5677_IF1_ADC2_SWAP_SFT: c_int = 4;

pub const RT5677_IF1_ADC3_SWAP_SFT: c_int = 2;

pub const RT5677_IF1_ADC4_SWAP_SFT: c_int = 0;
// TDM1 control 2 (0x3c)

pub const RT5677_IF1_ADC4_SFT: c_int = 10;

pub const RT5677_IF1_ADC3_SFT: c_int = 8;

pub const RT5677_IF1_ADC2_SFT: c_int = 6;

pub const RT5677_IF1_ADC1_SFT: c_int = 4;

pub const RT5677_IF1_ADC_CTRL_SFT: c_int = 0;
// TDM1 control 4 (0x3e)

pub const RT5677_IF1_DAC0_SFT: c_int = 12;

pub const RT5677_IF1_DAC1_SFT: c_int = 8;

pub const RT5677_IF1_DAC2_SFT: c_int = 4;

pub const RT5677_IF1_DAC3_SFT: c_int = 0;
// TDM1 control 5 (0x3f)

pub const RT5677_IF1_DAC4_SFT: c_int = 12;

pub const RT5677_IF1_DAC5_SFT: c_int = 8;

pub const RT5677_IF1_DAC6_SFT: c_int = 4;

pub const RT5677_IF1_DAC7_SFT: c_int = 0;
// TDM2 control 1 (0x40)

pub const RT5677_IF2_ADC_MODE_SFT: c_int = 12;

pub const RT5677_IF2_ADC1_SWAP_SFT: c_int = 6;

pub const RT5677_IF2_ADC2_SWAP_SFT: c_int = 4;

pub const RT5677_IF2_ADC3_SWAP_SFT: c_int = 2;

pub const RT5677_IF2_ADC4_SWAP_SFT: c_int = 0;
// TDM2 control 2 (0x41)

pub const RT5677_IF2_ADC4_SFT: c_int = 10;

pub const RT5677_IF2_ADC3_SFT: c_int = 8;

pub const RT5677_IF2_ADC2_SFT: c_int = 6;

pub const RT5677_IF2_ADC1_SFT: c_int = 4;

pub const RT5677_IF2_ADC_CTRL_SFT: c_int = 0;
// TDM2 control 4 (0x43)

pub const RT5677_IF2_DAC0_SFT: c_int = 12;

pub const RT5677_IF2_DAC1_SFT: c_int = 8;

pub const RT5677_IF2_DAC2_SFT: c_int = 4;

pub const RT5677_IF2_DAC3_SFT: c_int = 0;
// TDM2 control 5 (0x44)

pub const RT5677_IF2_DAC4_SFT: c_int = 12;

pub const RT5677_IF2_DAC5_SFT: c_int = 8;

pub const RT5677_IF2_DAC6_SFT: c_int = 4;

pub const RT5677_IF2_DAC7_SFT: c_int = 0;
// Digital Microphone Control 1 (0x50)

pub const RT5677_DMIC_1_EN_SFT: c_int = 15;

pub const RT5677_DMIC_2_EN_SFT: c_int = 14;

pub const RT5677_DMIC_L_STO1_LH_SFT: c_int = 13;

pub const RT5677_DMIC_R_STO1_LH_SFT: c_int = 12;

pub const RT5677_DMIC_L_STO3_LH_SFT: c_int = 11;

pub const RT5677_DMIC_R_STO3_LH_SFT: c_int = 10;

pub const RT5677_DMIC_L_STO2_LH_SFT: c_int = 9;

pub const RT5677_DMIC_R_STO2_LH_SFT: c_int = 8;

pub const RT5677_DMIC_CLK_SFT: c_int = 5;

pub const RT5677_DMIC_3_EN_SFT: c_int = 4;

pub const RT5677_DMIC_R_MONO_LH_SFT: c_int = 2;

pub const RT5677_DMIC_L_STO4_LH_SFT: c_int = 1;

pub const RT5677_DMIC_R_STO4_LH_SFT: c_int = 0;

// Digital Microphone Control 2 (0x51)

pub const RT5677_DMIC_4_EN_SFT: c_int = 15;

pub const RT5677_DMIC_4L_LH_SFT: c_int = 7;

pub const RT5677_DMIC_4R_LH_SFT: c_int = 6;

pub const RT5677_DMIC_3L_LH_SFT: c_int = 5;

pub const RT5677_DMIC_3R_LH_SFT: c_int = 4;

pub const RT5677_DMIC_2L_LH_SFT: c_int = 3;

pub const RT5677_DMIC_2R_LH_SFT: c_int = 2;

pub const RT5677_DMIC_1L_LH_SFT: c_int = 1;

pub const RT5677_DMIC_1R_LH_SFT: c_int = 0;

// Power Management for Digital 1 (0x61)

pub const RT5677_PWR_I2S1_BIT: c_int = 15;

pub const RT5677_PWR_I2S2_BIT: c_int = 14;

pub const RT5677_PWR_I2S3_BIT: c_int = 13;

pub const RT5677_PWR_DAC1_BIT: c_int = 12;

pub const RT5677_PWR_DAC2_BIT: c_int = 11;

pub const RT5677_PWR_I2S4_BIT: c_int = 10;

pub const RT5677_PWR_SLB_BIT: c_int = 9;

pub const RT5677_PWR_DAC3_BIT: c_int = 7;

pub const RT5677_PWR_ADCFED2_BIT: c_int = 4;

pub const RT5677_PWR_ADCFED1_BIT: c_int = 3;

pub const RT5677_PWR_ADC_L_BIT: c_int = 2;

pub const RT5677_PWR_ADC_R_BIT: c_int = 1;

pub const RT5677_PWR_I2C_MASTER_BIT: c_int = 0;
// Power Management for Digital 2 (0x62)

pub const RT5677_PWR_ADC_S1F_BIT: c_int = 15;

pub const RT5677_PWR_ADC_MF_L_BIT: c_int = 14;

pub const RT5677_PWR_ADC_MF_R_BIT: c_int = 13;

pub const RT5677_PWR_DAC_S1F_BIT: c_int = 12;

pub const RT5677_PWR_DAC_M2F_L_BIT: c_int = 11;

pub const RT5677_PWR_DAC_M2F_R_BIT: c_int = 10;

pub const RT5677_PWR_DAC_M3F_L_BIT: c_int = 9;

pub const RT5677_PWR_DAC_M3F_R_BIT: c_int = 8;

pub const RT5677_PWR_DAC_M4F_L_BIT: c_int = 7;

pub const RT5677_PWR_DAC_M4F_R_BIT: c_int = 6;

pub const RT5677_PWR_ADC_S2F_BIT: c_int = 5;

pub const RT5677_PWR_ADC_S3F_BIT: c_int = 4;

pub const RT5677_PWR_ADC_S4F_BIT: c_int = 3;

pub const RT5677_PWR_PDM1_BIT: c_int = 2;

pub const RT5677_PWR_PDM2_BIT: c_int = 1;
// Power Management for Analog 1 (0x63)

pub const RT5677_PWR_VREF1_BIT: c_int = 15;

pub const RT5677_PWR_FV1_BIT: c_int = 14;

pub const RT5677_PWR_MB_BIT: c_int = 13;

pub const RT5677_PWR_LO1_BIT: c_int = 12;

pub const RT5677_PWR_BG_BIT: c_int = 11;

pub const RT5677_PWR_LO2_BIT: c_int = 10;

pub const RT5677_PWR_LO3_BIT: c_int = 9;

pub const RT5677_PWR_VREF2_BIT: c_int = 8;

pub const RT5677_PWR_FV2_BIT: c_int = 7;

pub const RT5677_LDO2_SEL_SFT: c_int = 4;

pub const RT5677_LDO1_SEL_SFT: c_int = 0;
// Power Management for Analog 2 (0x64)

pub const RT5677_PWR_BST1_BIT: c_int = 15;

pub const RT5677_PWR_BST2_BIT: c_int = 14;

pub const RT5677_PWR_CLK_MB1_BIT: c_int = 13;

pub const RT5677_PWR_SLIM_BIT: c_int = 12;

pub const RT5677_PWR_MB1_BIT: c_int = 11;

pub const RT5677_PWR_PP_MB1_BIT: c_int = 10;

pub const RT5677_PWR_PLL1_BIT: c_int = 9;

pub const RT5677_PWR_PLL2_BIT: c_int = 8;

pub const RT5677_PWR_CORE_BIT: c_int = 7;

pub const RT5677_PWR_CLK_MB_BIT: c_int = 6;

pub const RT5677_PWR_BST1_P_BIT: c_int = 5;

pub const RT5677_PWR_BST2_P_BIT: c_int = 4;

pub const RT5677_PWR_IPTV_BIT: c_int = 3;

pub const RT5677_PWR_25M_CLK_BIT: c_int = 1;

pub const RT5677_PWR_LDO1_BIT: c_int = 0;
// Power Management for DSP (0x65)

pub const RT5677_PWR_SR7_BIT: c_int = 10;

pub const RT5677_PWR_SR6_BIT: c_int = 9;

pub const RT5677_PWR_SR5_BIT: c_int = 8;

pub const RT5677_PWR_SR4_BIT: c_int = 7;

pub const RT5677_PWR_SR3_BIT: c_int = 6;

pub const RT5677_PWR_SR2_BIT: c_int = 5;

pub const RT5677_PWR_SR1_BIT: c_int = 4;

pub const RT5677_PWR_SR0_BIT: c_int = 3;

pub const RT5677_PWR_MLT_BIT: c_int = 2;

pub const RT5677_PWR_DSP_BIT: c_int = 1;

pub const RT5677_PWR_DSP_CPU_BIT: c_int = 0;
// Power Status for DSP (0x66)

pub const RT5677_PWR_SR7_RDY_BIT: c_int = 9;

pub const RT5677_PWR_SR6_RDY_BIT: c_int = 8;

pub const RT5677_PWR_SR5_RDY_BIT: c_int = 7;

pub const RT5677_PWR_SR4_RDY_BIT: c_int = 6;

pub const RT5677_PWR_SR3_RDY_BIT: c_int = 5;

pub const RT5677_PWR_SR2_RDY_BIT: c_int = 4;

pub const RT5677_PWR_SR1_RDY_BIT: c_int = 3;

pub const RT5677_PWR_SR0_RDY_BIT: c_int = 2;

pub const RT5677_PWR_MLT_RDY_BIT: c_int = 1;

pub const RT5677_PWR_DSP_RDY_BIT: c_int = 0;
// Power Management for DSP (0x67)

pub const RT5677_PWR_SLIM_ISO_BIT: c_int = 11;

pub const RT5677_PWR_CORE_ISO_BIT: c_int = 10;

pub const RT5677_PWR_DSP_ISO_BIT: c_int = 9;

pub const RT5677_PWR_SR7_ISO_BIT: c_int = 8;

pub const RT5677_PWR_SR6_ISO_BIT: c_int = 7;

pub const RT5677_PWR_SR5_ISO_BIT: c_int = 6;

pub const RT5677_PWR_SR4_ISO_BIT: c_int = 5;

pub const RT5677_PWR_SR3_ISO_BIT: c_int = 4;

pub const RT5677_PWR_SR2_ISO_BIT: c_int = 3;

pub const RT5677_PWR_SR1_ISO_BIT: c_int = 2;

pub const RT5677_PWR_SR0_ISO_BIT: c_int = 1;

pub const RT5677_PWR_MLT_ISO_BIT: c_int = 0;
// I2S1/2/3/4 Audio Serial Data Port Control (0x6f 0x70 0x71 0x72)

pub const RT5677_I2S_MS_SFT: c_int = 15;

pub const RT5677_I2S_O_CP_SFT: c_int = 10;

pub const RT5677_I2S_I_CP_SFT: c_int = 8;

pub const RT5677_I2S_BP_SFT: c_int = 7;

pub const RT5677_I2S_DL_SFT: c_int = 2;

pub const RT5677_I2S_DF_SFT: c_int = 0;

// Clock Tree Control 1 (0x73)

pub const RT5677_I2S_PD1_SFT: c_int = 12;

pub const RT5677_I2S_BCLK_MS2_SFT: c_int = 11;

pub const RT5677_I2S_PD2_SFT: c_int = 8;

pub const RT5677_I2S_BCLK_MS3_SFT: c_int = 7;

pub const RT5677_I2S_PD3_SFT: c_int = 4;

pub const RT5677_I2S_BCLK_MS4_SFT: c_int = 3;

pub const RT5677_I2S_PD4_SFT: c_int = 0;

// Clock Tree Control 2 (0x74)

pub const RT5677_I2S_PD5_SFT: c_int = 12;

pub const RT5677_I2S_PD6_SFT: c_int = 8;

pub const RT5677_I2S_PD7_SFT: c_int = 4;

pub const RT5677_I2S_PD8_SFT: c_int = 0;

// Clock Tree Control 3 (0x75)

pub const RT5677_DSP_ASRC_O_SFT: c_int = 6;

pub const RT5677_DSP_ASRC_I_SFT: c_int = 4;

pub const RT5677_DSP_BUS_PD_SFT: c_int = 0;

pub const RT5677_PLL_INP_MAX: c_int = 40000000;
pub const RT5677_PLL_INP_MIN: c_int = 2048000;
// PLL M/N/K Code Control 1 (0x7a 0x7c)
pub const RT5677_PLL_N_MAX: c_uint = 0x1ff;

pub const RT5677_PLL_N_SFT: c_int = 7;

pub const RT5677_PLL_K_BP_SFT: c_int = 5;
pub const RT5677_PLL_K_MAX: c_uint = 0x1f;

pub const RT5677_PLL_K_SFT: c_int = 0;
// PLL M/N/K Code Control 2 (0x7b 0x7d)
pub const RT5677_PLL_M_MAX: c_uint = 0xf;

pub const RT5677_PLL_M_SFT: c_int = 12;

pub const RT5677_PLL_M_BP_SFT: c_int = 11;

pub const RT5677_PLL_UPDATE_PLL1_SFT: c_int = 1;
// Global Clock Control 1 (0x80)

pub const RT5677_SCLK_SRC_SFT: c_int = 14;

pub const RT5677_PLL1_SRC_SFT: c_int = 11;

pub const RT5677_MCLK_SRC_SFT: c_int = 10;

pub const RT5677_PLL1_PD_SFT: c_int = 8;

pub const RT5677_DAC_OSR_SFT: c_int = 6;

pub const RT5677_ADC_OSR_SFT: c_int = 4;

// Global Clock Control 2 (0x81)

pub const RT5677_PLL2_PR_SRC_SFT: c_int = 15;

pub const RT5677_PLL2_SRC_SFT: c_int = 12;

pub const RT5677_DSP_ASRC_O_SRC_SFT: c_int = 10;

pub const RT5677_DSP_ASRC_I_SRC_SFT: c_int = 8;

pub const RT5677_DSP_CLK_SRC_SFT: c_int = 7;

// ASRC Control 3 (0x85)

pub const RT5677_DA_STO_CLK_SEL_SFT: c_int = 12;

pub const RT5677_DA_MONO2L_CLK_SEL_SFT: c_int = 4;

pub const RT5677_DA_MONO2R_CLK_SEL_SFT: c_int = 0;
// ASRC Control 4 (0x86)

pub const RT5677_DA_MONO3L_CLK_SEL_SFT: c_int = 12;

pub const RT5677_DA_MONO3R_CLK_SEL_SFT: c_int = 8;

pub const RT5677_DA_MONO4L_CLK_SEL_SFT: c_int = 4;

pub const RT5677_DA_MONO4R_CLK_SEL_SFT: c_int = 0;
// ASRC Control 5 (0x87)

pub const RT5677_AD_STO1_CLK_SEL_SFT: c_int = 12;

pub const RT5677_AD_STO2_CLK_SEL_SFT: c_int = 8;

pub const RT5677_AD_STO3_CLK_SEL_SFT: c_int = 4;

pub const RT5677_AD_STO4_CLK_SEL_SFT: c_int = 0;
// ASRC Control 6 (0x88)

pub const RT5677_AD_MONOL_CLK_SEL_SFT: c_int = 12;

pub const RT5677_AD_MONOR_CLK_SEL_SFT: c_int = 8;
// ASRC Control 7 (0x89)

pub const RT5677_DSP_OB_0_3_CLK_SEL_SFT: c_int = 12;

pub const RT5677_DSP_OB_4_7_CLK_SEL_SFT: c_int = 8;
// ASRC Control 8 (0x8a)

pub const RT5677_I2S1_CLK_SEL_SFT: c_int = 12;

pub const RT5677_I2S2_CLK_SEL_SFT: c_int = 8;

pub const RT5677_I2S3_CLK_SEL_SFT: c_int = 4;

pub const RT5677_I2S4_CLK_SEL_SFT: c_int = 0;
// VAD Function Control 1 (0x9c)

pub const RT5677_VAD_MIN_DUR_SFT: c_int = 13;

pub const RT5677_VAD_ADPCM_BYPASS_BIT: c_int = 10;

pub const RT5677_VAD_FG2ENC_BIT: c_int = 9;

pub const RT5677_VAD_BUF_OW_BIT: c_int = 8;

pub const RT5677_VAD_CLR_FLAG_BIT: c_int = 7;

pub const RT5677_VAD_BUF_POP_BIT: c_int = 6;

pub const RT5677_VAD_BUF_PUSH_BIT: c_int = 5;

pub const RT5677_VAD_DET_ENABLE_BIT: c_int = 4;

pub const RT5677_VAD_FUNC_ENABLE_BIT: c_int = 3;

pub const RT5677_VAD_FUNC_RESET_BIT: c_int = 2;
// VAD Function Control 4 (0x9f)

pub const RT5677_VAD_OUT_SRC_RATE_SFT: c_int = 11;

pub const RT5677_VAD_OUT_SRC_SFT: c_int = 10;

pub const RT5677_VAD_SRC_SFT: c_int = 8;

pub const RT5677_VAD_LV_DIFF_SFT: c_int = 0;
// DSP InBound Control (0xa3)

pub const RT5677_IB01_SRC_SFT: c_int = 12;

pub const RT5677_IB23_SRC_SFT: c_int = 8;

pub const RT5677_IB45_SRC_SFT: c_int = 4;

pub const RT5677_IB6_SRC_SFT: c_int = 0;
// DSP InBound Control (0xa4)

pub const RT5677_IB7_SRC_SFT: c_int = 12;

pub const RT5677_IB8_SRC_SFT: c_int = 8;

pub const RT5677_IB9_SRC_SFT: c_int = 4;
// DSP In/OutBound Control (0xa5)

pub const RT5677_SEL_SRC_OB23_SFT: c_int = 4;

pub const RT5677_SEL_SRC_OB01_SFT: c_int = 3;

pub const RT5677_SEL_SRC_IB45_SFT: c_int = 2;

pub const RT5677_SEL_SRC_IB23_SFT: c_int = 1;

pub const RT5677_SEL_SRC_IB01_SFT: c_int = 0;
// Jack Detect Control 1 (0xb5)

pub const RT5677_SEL_GPIO_JD1_SFT: c_int = 14;

pub const RT5677_SEL_GPIO_JD2_SFT: c_int = 12;

pub const RT5677_SEL_GPIO_JD3_SFT: c_int = 10;
// IRQ Control 1 (0xbd)

pub const RT5677_STA_GPIO_JD1_SFT: c_int = 15;

pub const RT5677_EN_IRQ_GPIO_JD1_SFT: c_int = 14;

pub const RT5677_EN_GPIO_JD1_STICKY_SFT: c_int = 13;

pub const RT5677_INV_GPIO_JD1_SFT: c_int = 12;

pub const RT5677_STA_GPIO_JD2_SFT: c_int = 11;

pub const RT5677_EN_IRQ_GPIO_JD2_SFT: c_int = 10;

pub const RT5677_EN_GPIO_JD2_STICKY_SFT: c_int = 9;

pub const RT5677_INV_GPIO_JD2_SFT: c_int = 8;

pub const RT5677_STA_MICBIAS1_OVCD_SFT: c_int = 7;

pub const RT5677_EN_IRQ_MICBIAS1_OVCD_SFT: c_int = 6;

pub const RT5677_EN_MICBIAS1_OVCD_STICKY_SFT: c_int = 5;

pub const RT5677_INV_MICBIAS1_OVCD_SFT: c_int = 4;

pub const RT5677_STA_GPIO_JD3_SFT: c_int = 3;

pub const RT5677_EN_IRQ_GPIO_JD3_SFT: c_int = 2;

pub const RT5677_EN_GPIO_JD3_STICKY_SFT: c_int = 1;

pub const RT5677_INV_GPIO_JD3_SFT: c_int = 0;
// GPIO status (0xbf)

pub const RT5677_GPIO6_STATUS_SFT: c_int = 5;

pub const RT5677_GPIO5_STATUS_SFT: c_int = 4;

pub const RT5677_GPIO4_STATUS_SFT: c_int = 3;

pub const RT5677_GPIO3_STATUS_SFT: c_int = 2;

pub const RT5677_GPIO2_STATUS_SFT: c_int = 1;

pub const RT5677_GPIO1_STATUS_SFT: c_int = 0;
// GPIO Control 1 (0xc0)

pub const RT5677_GPIO1_PIN_SFT: c_int = 15;

pub const RT5677_IPTV_MODE_SFT: c_int = 14;

pub const RT5677_FUNC_MODE_SFT: c_int = 13;

// GPIO Control 2 (0xc1) & 3 (0xc2) common bits

pub const RT5677_GPIOx_DIR_SFT: c_int = 2;

pub const RT5677_GPIOx_OUT_SFT: c_int = 1;

pub const RT5677_GPIOx_P_SFT: c_int = 0;

// General Control (0xfa)

// Virtual DSP Mixer Control (0xf7 0xf8 0xf9)

pub const RT5677_DSP_IB_01_H_SFT: c_int = 15;

pub const RT5677_DSP_IB_23_H_SFT: c_int = 14;

pub const RT5677_DSP_IB_45_H_SFT: c_int = 13;

pub const RT5677_DSP_IB_6_H_SFT: c_int = 12;

pub const RT5677_DSP_IB_7_H_SFT: c_int = 11;

pub const RT5677_DSP_IB_8_H_SFT: c_int = 10;

pub const RT5677_DSP_IB_9_H_SFT: c_int = 9;

pub const RT5677_DSP_IB_01_L_SFT: c_int = 7;

pub const RT5677_DSP_IB_23_L_SFT: c_int = 6;

pub const RT5677_DSP_IB_45_L_SFT: c_int = 5;

pub const RT5677_DSP_IB_6_L_SFT: c_int = 4;

pub const RT5677_DSP_IB_7_L_SFT: c_int = 3;

pub const RT5677_DSP_IB_8_L_SFT: c_int = 2;

pub const RT5677_DSP_IB_9_L_SFT: c_int = 1;
// General Control2 (0xfc)

// System Clock Source
// PLL1 Source
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt5677_type {
    RT5677 = 1,
    RT5676 = 2,
}

// ASRC clock source selection
// filter mask
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt5677_dmic2_clk {
    RT5677_DMIC_CLK1 = 0,
    RT5677_DMIC_CLK2 = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5677_platform_data {
// IN1/IN2/LOUT1/LOUT2/LOUT3 can optionally be differential
    pub in1_diff: bool,
    pub in2_diff: bool,
    pub lout1_diff: bool,
    pub lout2_diff: bool,
    pub lout3_diff: bool,
// DMIC2 clock source selection
    pub dmic2_clk_pin: rt5677_dmic2_clk,
// configures GPIO, 0 - floating, 1 - pulldown, 2 - pullup
    pub gpio_config: [u8; 6],
// jd1 can select 0 ~ 3 as OFF, GPIO1, GPIO2 and GPIO3 respectively
    pub jd1_gpio: c_uint,
// jd2 and jd3 can select 0 ~ 3 as
    pub jd2_gpio: c_uint,
    pub jd3_gpio: c_uint,
// Set MICBIAS1 VDD 1v8 or 3v3
    pub micbias1_vdd_3v3: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5677_priv {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
    pub pdata: rt5677_platform_data,
    pub regmap_physical: *mut *mut regmap regmap,,
    pub fw2: *const *const firmware fw1,,
    pub dsp_pri_lock: mutex dsp_cmd_lock,,
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: [c_int; RT5677_AIFS],
    pub bclk: [c_int; RT5677_AIFS],
    pub master: [c_int; RT5677_AIFS],
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
    pub /: *mut *mut *mut gpio_desc pow_ldo2; / POW_LDO2 pin,
    pub /: *mut *mut *mut gpio_desc reset_pin; / RESET pin,
    pub type: rt5677_type,

    pub gpio_chip: gpio_chip,

    pub /: *mut *mut bool dsp_vad_en_request; / DSP VAD enable/disable request,
    pub /: *mut *mut bool dsp_vad_en; / dsp_work parameter,
    pub is_dsp_mode: bool,
    pub is_vref_slow: bool,
    pub dsp_work: delayed_work,
// Interrupt handling
    pub domain: *mut irq_domain,
    pub irq_lock: mutex,
    pub irq_en: c_uint,
    pub resume_irq_check: delayed_work,
    pub irq: c_int,
    pub on): *mut *mut *mut int (set_dsp_vad)(struct snd_soc_component component, bool,
}
