//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5670.h
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
// rt5670.h  --  RT5670 ALSA SoC audio driver
//
// Copyright 2014 Realtek Microelectronics
// Author: Bard Liao <bardliao@realtek.com>
//
// Info
pub const RT5670_RESET: c_uint = 0x00;
pub const RT5670_VENDOR_ID: c_uint = 0xfd;
pub const RT5670_VENDOR_ID1: c_uint = 0xfe;
pub const RT5670_VENDOR_ID2: c_uint = 0xff;
// I/O - Output
pub const RT5670_HP_VOL: c_uint = 0x02;
pub const RT5670_LOUT1: c_uint = 0x03;
// I/O - Input
pub const RT5670_CJ_CTRL1: c_uint = 0x0a;
pub const RT5670_CJ_CTRL2: c_uint = 0x0b;
pub const RT5670_CJ_CTRL3: c_uint = 0x0c;
pub const RT5670_IN2: c_uint = 0x0e;
pub const RT5670_INL1_INR1_VOL: c_uint = 0x0f;
// I/O - ADC/DAC/DMIC
pub const RT5670_DAC1_DIG_VOL: c_uint = 0x19;
pub const RT5670_DAC2_DIG_VOL: c_uint = 0x1a;
pub const RT5670_DAC_CTRL: c_uint = 0x1b;
pub const RT5670_STO1_ADC_DIG_VOL: c_uint = 0x1c;
pub const RT5670_MONO_ADC_DIG_VOL: c_uint = 0x1d;
pub const RT5670_ADC_BST_VOL1: c_uint = 0x1e;
pub const RT5670_STO2_ADC_DIG_VOL: c_uint = 0x1f;
// Mixer - D-D
pub const RT5670_ADC_BST_VOL2: c_uint = 0x20;
pub const RT5670_STO2_ADC_MIXER: c_uint = 0x26;
pub const RT5670_STO1_ADC_MIXER: c_uint = 0x27;
pub const RT5670_MONO_ADC_MIXER: c_uint = 0x28;
pub const RT5670_AD_DA_MIXER: c_uint = 0x29;
pub const RT5670_STO_DAC_MIXER: c_uint = 0x2a;
pub const RT5670_DD_MIXER: c_uint = 0x2b;
pub const RT5670_DIG_MIXER: c_uint = 0x2c;
pub const RT5670_DSP_PATH1: c_uint = 0x2d;
pub const RT5670_DSP_PATH2: c_uint = 0x2e;
pub const RT5670_DIG_INF1_DATA: c_uint = 0x2f;
pub const RT5670_DIG_INF2_DATA: c_uint = 0x30;
// Mixer - PDM
pub const RT5670_PDM_OUT_CTRL: c_uint = 0x31;
pub const RT5670_PDM_DATA_CTRL1: c_uint = 0x32;
pub const RT5670_PDM1_DATA_CTRL2: c_uint = 0x33;
pub const RT5670_PDM1_DATA_CTRL3: c_uint = 0x34;
pub const RT5670_PDM1_DATA_CTRL4: c_uint = 0x35;
pub const RT5670_PDM2_DATA_CTRL2: c_uint = 0x36;
pub const RT5670_PDM2_DATA_CTRL3: c_uint = 0x37;
pub const RT5670_PDM2_DATA_CTRL4: c_uint = 0x38;
// Mixer - ADC
pub const RT5670_REC_L1_MIXER: c_uint = 0x3b;
pub const RT5670_REC_L2_MIXER: c_uint = 0x3c;
pub const RT5670_REC_R1_MIXER: c_uint = 0x3d;
pub const RT5670_REC_R2_MIXER: c_uint = 0x3e;
// Mixer - DAC
pub const RT5670_HPO_MIXER: c_uint = 0x45;
pub const RT5670_MONO_MIXER: c_uint = 0x4c;
pub const RT5670_OUT_L1_MIXER: c_uint = 0x4f;
pub const RT5670_OUT_R1_MIXER: c_uint = 0x52;
pub const RT5670_LOUT_MIXER: c_uint = 0x53;
// Power
pub const RT5670_PWR_DIG1: c_uint = 0x61;
pub const RT5670_PWR_DIG2: c_uint = 0x62;
pub const RT5670_PWR_ANLG1: c_uint = 0x63;
pub const RT5670_PWR_ANLG2: c_uint = 0x64;
pub const RT5670_PWR_MIXER: c_uint = 0x65;
pub const RT5670_PWR_VOL: c_uint = 0x66;
// Private Register Control
pub const RT5670_PRIV_INDEX: c_uint = 0x6a;
pub const RT5670_PRIV_DATA: c_uint = 0x6c;
// Format - ADC/DAC
pub const RT5670_I2S4_SDP: c_uint = 0x6f;
pub const RT5670_I2S1_SDP: c_uint = 0x70;
pub const RT5670_I2S2_SDP: c_uint = 0x71;
pub const RT5670_I2S3_SDP: c_uint = 0x72;
pub const RT5670_ADDA_CLK1: c_uint = 0x73;
pub const RT5670_ADDA_CLK2: c_uint = 0x74;
pub const RT5670_DMIC_CTRL1: c_uint = 0x75;
pub const RT5670_DMIC_CTRL2: c_uint = 0x76;
// Format - TDM Control
pub const RT5670_TDM_CTRL_1: c_uint = 0x77;
pub const RT5670_TDM_CTRL_2: c_uint = 0x78;
pub const RT5670_TDM_CTRL_3: c_uint = 0x79;
// Function - Analog
pub const RT5670_DSP_CLK: c_uint = 0x7f;
pub const RT5670_GLB_CLK: c_uint = 0x80;
pub const RT5670_PLL_CTRL1: c_uint = 0x81;
pub const RT5670_PLL_CTRL2: c_uint = 0x82;
pub const RT5670_ASRC_1: c_uint = 0x83;
pub const RT5670_ASRC_2: c_uint = 0x84;
pub const RT5670_ASRC_3: c_uint = 0x85;
pub const RT5670_ASRC_4: c_uint = 0x86;
pub const RT5670_ASRC_5: c_uint = 0x87;
pub const RT5670_ASRC_7: c_uint = 0x89;
pub const RT5670_ASRC_8: c_uint = 0x8a;
pub const RT5670_ASRC_9: c_uint = 0x8b;
pub const RT5670_ASRC_10: c_uint = 0x8c;
pub const RT5670_ASRC_11: c_uint = 0x8d;
pub const RT5670_DEPOP_M1: c_uint = 0x8e;
pub const RT5670_DEPOP_M2: c_uint = 0x8f;
pub const RT5670_DEPOP_M3: c_uint = 0x90;
pub const RT5670_CHARGE_PUMP: c_uint = 0x91;
pub const RT5670_MICBIAS: c_uint = 0x93;
pub const RT5670_A_JD_CTRL1: c_uint = 0x94;
pub const RT5670_A_JD_CTRL2: c_uint = 0x95;
pub const RT5670_ASRC_12: c_uint = 0x97;
pub const RT5670_ASRC_13: c_uint = 0x98;
pub const RT5670_ASRC_14: c_uint = 0x99;
pub const RT5670_VAD_CTRL1: c_uint = 0x9a;
pub const RT5670_VAD_CTRL2: c_uint = 0x9b;
pub const RT5670_VAD_CTRL3: c_uint = 0x9c;
pub const RT5670_VAD_CTRL4: c_uint = 0x9d;
pub const RT5670_VAD_CTRL5: c_uint = 0x9e;
// Function - Digital
pub const RT5670_ADC_EQ_CTRL1: c_uint = 0xae;
pub const RT5670_ADC_EQ_CTRL2: c_uint = 0xaf;
pub const RT5670_EQ_CTRL1: c_uint = 0xb0;
pub const RT5670_EQ_CTRL2: c_uint = 0xb1;
pub const RT5670_ALC_DRC_CTRL1: c_uint = 0xb2;
pub const RT5670_ALC_DRC_CTRL2: c_uint = 0xb3;
pub const RT5670_ALC_CTRL_1: c_uint = 0xb4;
pub const RT5670_ALC_CTRL_2: c_uint = 0xb5;
pub const RT5670_ALC_CTRL_3: c_uint = 0xb6;
pub const RT5670_ALC_CTRL_4: c_uint = 0xb7;
pub const RT5670_JD_CTRL: c_uint = 0xbb;
pub const RT5670_IRQ_CTRL1: c_uint = 0xbd;
pub const RT5670_IRQ_CTRL2: c_uint = 0xbe;
pub const RT5670_INT_IRQ_ST: c_uint = 0xbf;
pub const RT5670_GPIO_CTRL1: c_uint = 0xc0;
pub const RT5670_GPIO_CTRL2: c_uint = 0xc1;
pub const RT5670_GPIO_CTRL3: c_uint = 0xc2;
pub const RT5670_SCRABBLE_FUN: c_uint = 0xcd;
pub const RT5670_SCRABBLE_CTRL: c_uint = 0xce;
pub const RT5670_BASE_BACK: c_uint = 0xcf;
pub const RT5670_MP3_PLUS1: c_uint = 0xd0;
pub const RT5670_MP3_PLUS2: c_uint = 0xd1;
pub const RT5670_ADJ_HPF1: c_uint = 0xd3;
pub const RT5670_ADJ_HPF2: c_uint = 0xd4;
pub const RT5670_HP_CALIB_AMP_DET: c_uint = 0xd6;
pub const RT5670_SV_ZCD1: c_uint = 0xd9;
pub const RT5670_SV_ZCD2: c_uint = 0xda;
pub const RT5670_IL_CMD: c_uint = 0xdb;
pub const RT5670_IL_CMD2: c_uint = 0xdc;
pub const RT5670_IL_CMD3: c_uint = 0xdd;
pub const RT5670_DRC_HL_CTRL1: c_uint = 0xe6;
pub const RT5670_DRC_HL_CTRL2: c_uint = 0xe7;
pub const RT5670_ADC_MONO_HP_CTRL1: c_uint = 0xec;
pub const RT5670_ADC_MONO_HP_CTRL2: c_uint = 0xed;
pub const RT5670_ADC_STO2_HP_CTRL1: c_uint = 0xee;
pub const RT5670_ADC_STO2_HP_CTRL2: c_uint = 0xef;
pub const RT5670_JD_CTRL3: c_uint = 0xf8;
pub const RT5670_JD_CTRL4: c_uint = 0xf9;
// General Control
pub const RT5670_DIG_MISC: c_uint = 0xfa;
pub const RT5670_GEN_CTRL2: c_uint = 0xfb;
pub const RT5670_GEN_CTRL3: c_uint = 0xfc;
// Index of Codec Private Register definition
pub const RT5670_DIG_VOL: c_uint = 0x00;
pub const RT5670_PR_ALC_CTRL_1: c_uint = 0x01;
pub const RT5670_PR_ALC_CTRL_2: c_uint = 0x02;
pub const RT5670_PR_ALC_CTRL_3: c_uint = 0x03;
pub const RT5670_PR_ALC_CTRL_4: c_uint = 0x04;
pub const RT5670_PR_ALC_CTRL_5: c_uint = 0x05;
pub const RT5670_PR_ALC_CTRL_6: c_uint = 0x06;
pub const RT5670_BIAS_CUR1: c_uint = 0x12;
pub const RT5670_BIAS_CUR3: c_uint = 0x14;
pub const RT5670_CLSD_INT_REG1: c_uint = 0x1c;
pub const RT5670_MAMP_INT_REG2: c_uint = 0x37;
pub const RT5670_CHOP_DAC_ADC: c_uint = 0x3d;
pub const RT5670_MIXER_INT_REG: c_uint = 0x3f;
pub const RT5670_3D_SPK: c_uint = 0x63;
pub const RT5670_WND_1: c_uint = 0x6c;
pub const RT5670_WND_2: c_uint = 0x6d;
pub const RT5670_WND_3: c_uint = 0x6e;
pub const RT5670_WND_4: c_uint = 0x6f;
pub const RT5670_WND_5: c_uint = 0x70;
pub const RT5670_WND_8: c_uint = 0x73;
pub const RT5670_DIP_SPK_INF: c_uint = 0x75;
pub const RT5670_HP_DCC_INT1: c_uint = 0x77;
pub const RT5670_EQ_BW_LOP: c_uint = 0xa0;
pub const RT5670_EQ_GN_LOP: c_uint = 0xa1;
pub const RT5670_EQ_FC_BP1: c_uint = 0xa2;
pub const RT5670_EQ_BW_BP1: c_uint = 0xa3;
pub const RT5670_EQ_GN_BP1: c_uint = 0xa4;
pub const RT5670_EQ_FC_BP2: c_uint = 0xa5;
pub const RT5670_EQ_BW_BP2: c_uint = 0xa6;
pub const RT5670_EQ_GN_BP2: c_uint = 0xa7;
pub const RT5670_EQ_FC_BP3: c_uint = 0xa8;
pub const RT5670_EQ_BW_BP3: c_uint = 0xa9;
pub const RT5670_EQ_GN_BP3: c_uint = 0xaa;
pub const RT5670_EQ_FC_BP4: c_uint = 0xab;
pub const RT5670_EQ_BW_BP4: c_uint = 0xac;
pub const RT5670_EQ_GN_BP4: c_uint = 0xad;
pub const RT5670_EQ_FC_HIP1: c_uint = 0xae;
pub const RT5670_EQ_GN_HIP1: c_uint = 0xaf;
pub const RT5670_EQ_FC_HIP2: c_uint = 0xb0;
pub const RT5670_EQ_BW_HIP2: c_uint = 0xb1;
pub const RT5670_EQ_GN_HIP2: c_uint = 0xb2;
pub const RT5670_EQ_PRE_VOL: c_uint = 0xb3;
pub const RT5670_EQ_PST_VOL: c_uint = 0xb4;
// global definition

pub const RT5670_L_MUTE_SFT: c_int = 15;

pub const RT5670_R_MUTE_SFT: c_int = 7;

pub const RT5670_L_VOL_SFT: c_int = 8;

pub const RT5670_R_VOL_SFT: c_int = 0;
// SW Reset & Device ID (0x00)

// Combo Jack Control 1 (0x0a)

// Combo Jack Control 1 (0x0b)

// IN2 Control (0x0e)

pub const RT5670_BST_SFT1: c_int = 12;

pub const RT5670_BST_SFT2: c_int = 8;

pub const RT5670_IN_SFT1: c_int = 7;

pub const RT5670_IN_SFT2: c_int = 6;
// INL and INR Volume Control (0x0f)

pub const RT5670_INL_SEL_SFT: c_int = 15;

pub const RT5670_INL_VOL_SFT: c_int = 8;

pub const RT5670_INR_SEL_SFT: c_int = 7;

pub const RT5670_INR_VOL_SFT: c_int = 0;
// Sidetone Control (0x18)

pub const RT5670_ST_SEL_SFT: c_int = 9;

pub const RT5670_M_ST_DACR2_SFT: c_int = 8;

pub const RT5670_M_ST_DACL2_SFT: c_int = 7;

pub const RT5670_ST_EN_SFT: c_int = 6;
// DAC1 Digital Volume (0x19)

pub const RT5670_DAC_L1_VOL_SFT: c_int = 8;

pub const RT5670_DAC_R1_VOL_SFT: c_int = 0;
// DAC2 Digital Volume (0x1a)

pub const RT5670_DAC_L2_VOL_SFT: c_int = 8;

pub const RT5670_DAC_R2_VOL_SFT: c_int = 0;
// DAC2 Control (0x1b)

pub const RT5670_M_DAC_L2_VOL_SFT: c_int = 13;

pub const RT5670_M_DAC_R2_VOL_SFT: c_int = 12;

pub const RT5670_DAC2_L_SEL_SFT: c_int = 4;

pub const RT5670_DAC2_R_SEL_SFT: c_int = 0;
// ADC Digital Volume Control (0x1c)

pub const RT5670_ADC_L_VOL_SFT: c_int = 8;

pub const RT5670_ADC_R_VOL_SFT: c_int = 0;
// Mono ADC Digital Volume Control (0x1d)

pub const RT5670_MONO_ADC_L_VOL_SFT: c_int = 8;

pub const RT5670_MONO_ADC_R_VOL_SFT: c_int = 0;
// ADC Boost Volume Control (0x1e)

pub const RT5670_STO1_ADC_L_BST_SFT: c_int = 14;

pub const RT5670_STO1_ADC_R_BST_SFT: c_int = 12;

pub const RT5670_STO1_ADC_COMP_SFT: c_int = 10;

pub const RT5670_STO2_ADC_L_BST_SFT: c_int = 8;

pub const RT5670_STO2_ADC_R_BST_SFT: c_int = 6;

pub const RT5670_STO2_ADC_COMP_SFT: c_int = 4;
// Stereo2 ADC Mixer Control (0x26)

pub const RT5670_STO2_ADC_SRC_SFT: c_int = 15;
// Stereo ADC Mixer Control (0x26 0x27)

pub const RT5670_M_ADC_L1_SFT: c_int = 14;

pub const RT5670_M_ADC_L2_SFT: c_int = 13;

pub const RT5670_ADC_1_SRC_SFT: c_int = 12;

pub const RT5670_ADC_2_SRC_SFT: c_int = 11;

pub const RT5670_ADC_SRC_SFT: c_int = 10;

pub const RT5670_DMIC_SRC_SFT: c_int = 8;

pub const RT5670_M_ADC_R1_SFT: c_int = 6;

pub const RT5670_M_ADC_R2_SFT: c_int = 5;

pub const RT5670_DMIC3_SRC_SFT: c_int = 0;
// Mono ADC Mixer Control (0x28)

pub const RT5670_M_MONO_ADC_L1_SFT: c_int = 14;

pub const RT5670_M_MONO_ADC_L2_SFT: c_int = 13;

pub const RT5670_MONO_ADC_L1_SRC_SFT: c_int = 12;

pub const RT5670_MONO_ADC_L2_SRC_SFT: c_int = 11;

pub const RT5670_MONO_ADC_L_SRC_SFT: c_int = 10;

pub const RT5670_MONO_DMIC_L_SRC_SFT: c_int = 8;

pub const RT5670_M_MONO_ADC_R1_SFT: c_int = 6;

pub const RT5670_M_MONO_ADC_R2_SFT: c_int = 5;

pub const RT5670_MONO_ADC_R1_SRC_SFT: c_int = 4;

pub const RT5670_MONO_ADC_R2_SRC_SFT: c_int = 3;

pub const RT5670_MONO_DMIC_R_SRC_SFT: c_int = 0;
// ADC Mixer to DAC Mixer Control (0x29)

pub const RT5670_M_ADCMIX_L_SFT: c_int = 15;

pub const RT5670_M_DAC1_L_SFT: c_int = 14;

pub const RT5670_DAC1_R_SEL_SFT: c_int = 10;

pub const RT5670_DAC1_L_SEL_SFT: c_int = 8;

pub const RT5670_M_ADCMIX_R_SFT: c_int = 7;

pub const RT5670_M_DAC1_R_SFT: c_int = 6;
// Stereo DAC Mixer Control (0x2a)

pub const RT5670_M_DAC_L1_SFT: c_int = 14;

pub const RT5670_DAC_L1_STO_L_VOL_SFT: c_int = 13;

pub const RT5670_M_DAC_L2_SFT: c_int = 12;

pub const RT5670_DAC_L2_STO_L_VOL_SFT: c_int = 11;

pub const RT5670_M_DAC_R1_STO_L_SFT: c_int = 9;

pub const RT5670_DAC_R1_STO_L_VOL_SFT: c_int = 8;

pub const RT5670_M_DAC_R1_SFT: c_int = 6;

pub const RT5670_DAC_R1_STO_R_VOL_SFT: c_int = 5;

pub const RT5670_M_DAC_R2_SFT: c_int = 4;

pub const RT5670_DAC_R2_STO_R_VOL_SFT: c_int = 3;

pub const RT5670_M_DAC_L1_STO_R_SFT: c_int = 1;

pub const RT5670_DAC_L1_STO_R_VOL_SFT: c_int = 0;
// Mono DAC Mixer Control (0x2b)

pub const RT5670_M_DAC_L1_MONO_L_SFT: c_int = 14;

pub const RT5670_DAC_L1_MONO_L_VOL_SFT: c_int = 13;

pub const RT5670_M_DAC_L2_MONO_L_SFT: c_int = 12;

pub const RT5670_DAC_L2_MONO_L_VOL_SFT: c_int = 11;

pub const RT5670_M_DAC_R2_MONO_L_SFT: c_int = 10;

pub const RT5670_DAC_R2_MONO_L_VOL_SFT: c_int = 9;

pub const RT5670_M_DAC_R1_MONO_R_SFT: c_int = 6;

pub const RT5670_DAC_R1_MONO_R_VOL_SFT: c_int = 5;

pub const RT5670_M_DAC_R2_MONO_R_SFT: c_int = 4;

pub const RT5670_DAC_R2_MONO_R_VOL_SFT: c_int = 3;

pub const RT5670_M_DAC_L2_MONO_R_SFT: c_int = 2;

pub const RT5670_DAC_L2_MONO_R_VOL_SFT: c_int = 1;
// Digital Mixer Control (0x2c)

pub const RT5670_M_STO_L_DAC_L_SFT: c_int = 15;

pub const RT5670_STO_L_DAC_L_VOL_SFT: c_int = 14;

pub const RT5670_M_DAC_L2_DAC_L_SFT: c_int = 13;

pub const RT5670_DAC_L2_DAC_L_VOL_SFT: c_int = 12;

pub const RT5670_M_STO_R_DAC_R_SFT: c_int = 11;

pub const RT5670_STO_R_DAC_R_VOL_SFT: c_int = 10;

pub const RT5670_M_DAC_R2_DAC_R_SFT: c_int = 9;

pub const RT5670_DAC_R2_DAC_R_VOL_SFT: c_int = 8;

pub const RT5670_M_DAC_R2_DAC_L_SFT: c_int = 7;

pub const RT5670_DAC_R2_DAC_L_VOL_SFT: c_int = 6;

pub const RT5670_M_DAC_L2_DAC_R_SFT: c_int = 5;

pub const RT5670_DAC_L2_DAC_R_VOL_SFT: c_int = 4;
// DSP Path Control 1 (0x2d)

pub const RT5670_RXDP_SEL_SFT: c_int = 13;

pub const RT5670_RXDP_SRC_SFT: c_int = 11;

pub const RT5670_TXDP_SRC_SFT: c_int = 4;

pub const RT5670_TXDP_SLOT_SEL_SFT: c_int = 2;

pub const RT5670_DSP_UL_SFT: c_int = 1;
pub const RT5670_DSP_DL_SEL: c_uint = 0x1;
pub const RT5670_DSP_DL_SFT: c_int = 0;
// DSP Path Control 2 (0x2e)

pub const RT5670_TXDP_L_VOL_SFT: c_int = 8;

pub const RT5670_TXDP_R_VOL_SFT: c_int = 0;
// Digital Interface Data Control (0x2f)

pub const RT5670_IF1_ADC2_IN_SFT: c_int = 15;

pub const RT5670_IF2_ADC_IN_SFT: c_int = 12;

pub const RT5670_IF2_DAC_SEL_SFT: c_int = 10;

pub const RT5670_IF2_ADC_SEL_SFT: c_int = 8;
// Digital Interface Data Control (0x30)

pub const RT5670_IF4_ADC_IN_SFT: c_int = 4;
// PDM Output Control (0x31)

pub const RT5670_PDM1_L_SFT: c_int = 15;

pub const RT5670_M_PDM1_L_SFT: c_int = 14;

pub const RT5670_PDM1_R_SFT: c_int = 13;

pub const RT5670_M_PDM1_R_SFT: c_int = 12;

pub const RT5670_PDM2_L_SFT: c_int = 11;

pub const RT5670_M_PDM2_L_SFT: c_int = 10;

pub const RT5670_PDM2_R_SFT: c_int = 9;

pub const RT5670_M_PDM2_R_SFT: c_int = 8;

// REC Left Mixer Control 1 (0x3b)

pub const RT5670_G_HP_L_RM_L_SFT: c_int = 13;

pub const RT5670_G_IN_L_RM_L_SFT: c_int = 10;

pub const RT5670_G_BST4_RM_L_SFT: c_int = 7;

pub const RT5670_G_BST3_RM_L_SFT: c_int = 4;

pub const RT5670_G_BST2_RM_L_SFT: c_int = 1;
// REC Left Mixer Control 2 (0x3c)

pub const RT5670_G_BST1_RM_L_SFT: c_int = 13;

pub const RT5670_M_IN_L_RM_L_SFT: c_int = 5;

pub const RT5670_M_BST2_RM_L_SFT: c_int = 3;

pub const RT5670_M_BST1_RM_L_SFT: c_int = 1;
// REC Right Mixer Control 1 (0x3d)

pub const RT5670_G_HP_R_RM_R_SFT: c_int = 13;

pub const RT5670_G_IN_R_RM_R_SFT: c_int = 10;

pub const RT5670_G_BST4_RM_R_SFT: c_int = 7;

pub const RT5670_G_BST3_RM_R_SFT: c_int = 4;

pub const RT5670_G_BST2_RM_R_SFT: c_int = 1;
// REC Right Mixer Control 2 (0x3e)

pub const RT5670_G_BST1_RM_R_SFT: c_int = 13;

pub const RT5670_M_IN_R_RM_R_SFT: c_int = 5;

pub const RT5670_M_BST2_RM_R_SFT: c_int = 3;

pub const RT5670_M_BST1_RM_R_SFT: c_int = 1;
// HPMIX Control (0x45)

pub const RT5670_M_DAC2_HM_SFT: c_int = 15;

pub const RT5670_M_HPVOL_HM_SFT: c_int = 14;

pub const RT5670_M_DAC1_HM_SFT: c_int = 13;

pub const RT5670_G_HPOMIX_SFT: c_int = 12;

pub const RT5670_M_INR1_HMR_SFT: c_int = 3;

pub const RT5670_M_DACR1_HMR_SFT: c_int = 2;

pub const RT5670_M_INL1_HML_SFT: c_int = 1;

pub const RT5670_M_DACL1_HML_SFT: c_int = 0;
// Mono Output Mixer Control (0x4c)

pub const RT5670_M_DAC_R2_MA_SFT: c_int = 15;

pub const RT5670_M_DAC_L2_MA_SFT: c_int = 14;

pub const RT5670_M_OV_R_MM_SFT: c_int = 13;

pub const RT5670_M_OV_L_MM_SFT: c_int = 12;

pub const RT5670_G_MONOMIX_SFT: c_int = 10;

pub const RT5670_M_DAC_R2_MM_SFT: c_int = 9;

pub const RT5670_M_DAC_L2_MM_SFT: c_int = 8;

pub const RT5670_M_BST4_MM_SFT: c_int = 7;
// Output Left Mixer Control 1 (0x4d)

pub const RT5670_G_BST3_OM_L_SFT: c_int = 13;

pub const RT5670_G_BST2_OM_L_SFT: c_int = 10;

pub const RT5670_G_BST1_OM_L_SFT: c_int = 7;

pub const RT5670_G_IN_L_OM_L_SFT: c_int = 4;

pub const RT5670_G_RM_L_OM_L_SFT: c_int = 1;
// Output Left Mixer Control 2 (0x4e)

pub const RT5670_G_DAC_R2_OM_L_SFT: c_int = 13;

pub const RT5670_G_DAC_L2_OM_L_SFT: c_int = 10;

pub const RT5670_G_DAC_L1_OM_L_SFT: c_int = 7;
// Output Left Mixer Control 3 (0x4f)

pub const RT5670_M_BST1_OM_L_SFT: c_int = 5;

pub const RT5670_M_IN_L_OM_L_SFT: c_int = 4;

pub const RT5670_M_DAC_L2_OM_L_SFT: c_int = 1;

pub const RT5670_M_DAC_L1_OM_L_SFT: c_int = 0;
// Output Right Mixer Control 1 (0x50)

pub const RT5670_G_BST4_OM_R_SFT: c_int = 13;

pub const RT5670_G_BST2_OM_R_SFT: c_int = 10;

pub const RT5670_G_BST1_OM_R_SFT: c_int = 7;

pub const RT5670_G_IN_R_OM_R_SFT: c_int = 4;

pub const RT5670_G_RM_R_OM_R_SFT: c_int = 1;
// Output Right Mixer Control 2 (0x51)

pub const RT5670_G_DAC_L2_OM_R_SFT: c_int = 13;

pub const RT5670_G_DAC_R2_OM_R_SFT: c_int = 10;

pub const RT5670_G_DAC_R1_OM_R_SFT: c_int = 7;
// Output Right Mixer Control 3 (0x52)

pub const RT5670_M_BST2_OM_R_SFT: c_int = 6;

pub const RT5670_M_IN_R_OM_R_SFT: c_int = 4;

pub const RT5670_M_DAC_R2_OM_R_SFT: c_int = 1;

pub const RT5670_M_DAC_R1_OM_R_SFT: c_int = 0;
// LOUT Mixer Control (0x53)

pub const RT5670_M_DAC_L1_LM_SFT: c_int = 15;

pub const RT5670_M_DAC_R1_LM_SFT: c_int = 14;

pub const RT5670_M_OV_L_LM_SFT: c_int = 13;

pub const RT5670_M_OV_R_LM_SFT: c_int = 12;

pub const RT5670_G_LOUTMIX_SFT: c_int = 11;
// Power Management for Digital 1 (0x61)

pub const RT5670_PWR_I2S1_BIT: c_int = 15;

pub const RT5670_PWR_I2S2_BIT: c_int = 14;

pub const RT5670_PWR_DAC_L1_BIT: c_int = 12;

pub const RT5670_PWR_DAC_R1_BIT: c_int = 11;

pub const RT5670_PWR_DAC_L2_BIT: c_int = 7;

pub const RT5670_PWR_DAC_R2_BIT: c_int = 6;

pub const RT5670_PWR_ADC_L_BIT: c_int = 2;

pub const RT5670_PWR_ADC_R_BIT: c_int = 1;

pub const RT5670_PWR_CLS_D_BIT: c_int = 0;
// Power Management for Digital 2 (0x62)

pub const RT5670_PWR_ADC_S1F_BIT: c_int = 15;

pub const RT5670_PWR_ADC_MF_L_BIT: c_int = 14;

pub const RT5670_PWR_ADC_MF_R_BIT: c_int = 13;

pub const RT5670_PWR_I2S_DSP_BIT: c_int = 12;

pub const RT5670_PWR_DAC_S1F_BIT: c_int = 11;

pub const RT5670_PWR_DAC_MF_L_BIT: c_int = 10;

pub const RT5670_PWR_DAC_MF_R_BIT: c_int = 9;

pub const RT5670_PWR_ADC_S2F_BIT: c_int = 8;

pub const RT5670_PWR_PDM1_BIT: c_int = 7;

pub const RT5670_PWR_PDM2_BIT: c_int = 6;
// Power Management for Analog 1 (0x63)

pub const RT5670_PWR_VREF1_BIT: c_int = 15;

pub const RT5670_PWR_FV1_BIT: c_int = 14;

pub const RT5670_PWR_MB_BIT: c_int = 13;

pub const RT5670_PWR_LM_BIT: c_int = 12;

pub const RT5670_PWR_BG_BIT: c_int = 11;

pub const RT5670_PWR_HP_L_BIT: c_int = 7;

pub const RT5670_PWR_HP_R_BIT: c_int = 6;

pub const RT5670_PWR_HA_BIT: c_int = 5;

pub const RT5670_PWR_VREF2_BIT: c_int = 4;

pub const RT5670_PWR_FV2_BIT: c_int = 3;

pub const RT5670_LDO_SEL_SFT: c_int = 0;
// Power Management for Analog 2 (0x64)

pub const RT5670_PWR_BST1_BIT: c_int = 15;

pub const RT5670_PWR_BST2_BIT: c_int = 13;

pub const RT5670_PWR_MB1_BIT: c_int = 11;

pub const RT5670_PWR_MB2_BIT: c_int = 10;

pub const RT5670_PWR_PLL_BIT: c_int = 9;

pub const RT5670_PWR_BST1_P_BIT: c_int = 6;

pub const RT5670_PWR_BST2_P_BIT: c_int = 4;

pub const RT5670_PWR_JD1_BIT: c_int = 2;

pub const RT5670_PWR_JD_BIT: c_int = 1;
// Power Management for Mixer (0x65)

pub const RT5670_PWR_OM_L_BIT: c_int = 15;

pub const RT5670_PWR_OM_R_BIT: c_int = 14;

pub const RT5670_PWR_RM_L_BIT: c_int = 11;

pub const RT5670_PWR_RM_R_BIT: c_int = 10;
// Power Management for Volume (0x66)

pub const RT5670_PWR_HV_L_BIT: c_int = 11;

pub const RT5670_PWR_HV_R_BIT: c_int = 10;

pub const RT5670_PWR_IN_L_BIT: c_int = 9;

pub const RT5670_PWR_IN_R_BIT: c_int = 8;

pub const RT5670_PWR_MIC_DET_BIT: c_int = 5;
// I2S1/2/3 Audio Serial Data Port Control (0x70 0x71 0x72)

pub const RT5670_I2S_MS_SFT: c_int = 15;

pub const RT5670_I2S_IF_SFT: c_int = 12;

pub const RT5670_I2S_O_CP_SFT: c_int = 10;

pub const RT5670_I2S_I_CP_SFT: c_int = 8;

pub const RT5670_I2S_BP_SFT: c_int = 7;

pub const RT5670_I2S_DL_SFT: c_int = 2;

pub const RT5670_I2S_DF_SFT: c_int = 0;

// I2S2 Audio Serial Data Port Control (0x71)

pub const RT5670_I2S2_SDI_SFT: c_int = 6;

// ADC/DAC Clock Control 1 (0x73)

pub const RT5670_I2S_BCLK_MS1_SFT: c_int = 15;

pub const RT5670_I2S_PD1_SFT: c_int = 12;

pub const RT5670_I2S_BCLK_MS2_SFT: c_int = 11;

pub const RT5670_I2S_PD2_SFT: c_int = 8;

pub const RT5670_I2S_BCLK_MS3_SFT: c_int = 7;

pub const RT5670_I2S_PD3_SFT: c_int = 4;

pub const RT5670_DAC_OSR_SFT: c_int = 2;

pub const RT5670_ADC_OSR_SFT: c_int = 0;

// ADC/DAC Clock Control 2 (0x74)

pub const RT5670_DAC_L_OSR_SFT: c_int = 14;

pub const RT5670_ADC_R_OSR_SFT: c_int = 12;

pub const RT5670_DAHPF_EN_SFT: c_int = 11;

pub const RT5670_ADHPF_EN_SFT: c_int = 10;
// Digital Microphone Control (0x75)

pub const RT5670_DMIC_1_EN_SFT: c_int = 15;

pub const RT5670_DMIC_2_EN_SFT: c_int = 14;

pub const RT5670_DMIC_1L_LH_SFT: c_int = 13;

pub const RT5670_DMIC_1R_LH_SFT: c_int = 12;

pub const RT5670_DMIC_2_DP_SFT: c_int = 10;

pub const RT5670_DMIC_2L_LH_SFT: c_int = 9;

pub const RT5670_DMIC_2R_LH_SFT: c_int = 8;

pub const RT5670_DMIC_CLK_SFT: c_int = 5;

pub const RT5670_DMIC_3_EN_SFT: c_int = 4;

pub const RT5670_DMIC_1_DP_SFT: c_int = 0;

// Digital Microphone Control2 (0x76)

pub const RT5670_DMIC_3_DP_SFT: c_int = 6;

// Global Clock Control (0x80)

pub const RT5670_SCLK_SRC_SFT: c_int = 14;

pub const RT5670_PLL1_SRC_SFT: c_int = 11;

pub const RT5670_PLL1_PD_SFT: c_int = 3;

pub const RT5670_PLL_INP_MAX: c_int = 40000000;
pub const RT5670_PLL_INP_MIN: c_int = 256000;
// PLL M/N/K Code Control 1 (0x81)
pub const RT5670_PLL_N_MAX: c_uint = 0x1ff;

pub const RT5670_PLL_N_SFT: c_int = 7;
pub const RT5670_PLL_K_MAX: c_uint = 0x1f;

pub const RT5670_PLL_K_SFT: c_int = 0;
// PLL M/N/K Code Control 2 (0x82)
pub const RT5670_PLL_M_MAX: c_uint = 0xf;

pub const RT5670_PLL_M_SFT: c_int = 12;

pub const RT5670_PLL_M_BP_SFT: c_int = 11;
// ASRC Control 1 (0x83)

pub const RT5670_STO_T_SFT: c_int = 15;

pub const RT5670_M1_T_SFT: c_int = 14;

pub const RT5670_I2S2_F_SFT: c_int = 12;

pub const RT5670_DMIC_1_M_SFT: c_int = 9;

pub const RT5670_DMIC_2_M_SFT: c_int = 8;

// ASRC clock source selection (0x84, 0x85)

// ASRC Control 2 (0x84)

pub const RT5670_DA_STO_CLK_SEL_SFT: c_int = 12;

pub const RT5670_DA_MONOL_CLK_SEL_SFT: c_int = 8;

pub const RT5670_DA_MONOR_CLK_SEL_SFT: c_int = 4;

pub const RT5670_AD_STO1_CLK_SEL_SFT: c_int = 0;
// ASRC Control 3 (0x85)

pub const RT5670_UP_CLK_SEL_SFT: c_int = 12;

pub const RT5670_DOWN_CLK_SEL_SFT: c_int = 8;

pub const RT5670_AD_MONOL_CLK_SEL_SFT: c_int = 4;

pub const RT5670_AD_MONOR_CLK_SEL_SFT: c_int = 0;
// ASRC Control 4 (0x89)

pub const RT5670_I2S1_PD_SFT: c_int = 12;

pub const RT5670_I2S2_PD_SFT: c_int = 8;
// HPOUT Over Current Detection (0x8b)

pub const RT5670_HP_OVCD_SFT: c_int = 10;

pub const RT5670_HP_OC_TH_SFT: c_int = 8;

// Class D Over Current Control (0x8c)

pub const RT5670_CLSD_OC_SFT: c_int = 9;

pub const RT5670_AUTO_PD_SFT: c_int = 8;

pub const RT5670_CLSD_OC_TH_SFT: c_int = 0;
// Class D Output Control (0x8d)

pub const RT5670_CLSD_RATIO_SFT: c_int = 12;

pub const RT5670_CLSD_OM_SFT: c_int = 11;

pub const RT5670_CLSD_SCH_SFT: c_int = 10;

// Depop Mode Control 1 (0x8e)

pub const RT5670_SMT_TRIG_SFT: c_int = 15;

pub const RT5670_HP_L_SMT_SFT: c_int = 9;

pub const RT5670_HP_R_SMT_SFT: c_int = 8;

pub const RT5670_HP_CD_PD_SFT: c_int = 7;

pub const RT5670_RSTN_SFT: c_int = 6;

pub const RT5670_RSTP_SFT: c_int = 5;

pub const RT5670_HP_CO_SFT: c_int = 4;

pub const RT5670_HP_CP_SFT: c_int = 3;

pub const RT5670_HP_SG_SFT: c_int = 2;

pub const RT5670_HP_DP_SFT: c_int = 1;

pub const RT5670_HP_CB_SFT: c_int = 0;

// Depop Mode Control 2 (0x8f)

pub const RT5670_DEPOP_SFT: c_int = 13;

pub const RT5670_RAMP_SFT: c_int = 12;

pub const RT5670_BPS_SFT: c_int = 11;

pub const RT5670_FAST_UPDN_SFT: c_int = 10;

pub const RT5670_MRES_SFT: c_int = 8;

pub const RT5670_VLO_SFT: c_int = 7;

pub const RT5670_DIG_DP_SFT: c_int = 6;

pub const RT5670_DP_TH_SFT: c_int = 4;
// Depop Mode Control 3 (0x90)

pub const RT5670_CP_SYS_SFT: c_int = 12;

pub const RT5670_CP_FQ1_SFT: c_int = 8;

pub const RT5670_CP_FQ2_SFT: c_int = 4;

pub const RT5670_CP_FQ3_SFT: c_int = 0;
pub const RT5670_CP_FQ_1_5_KHZ: c_int = 0;
pub const RT5670_CP_FQ_3_KHZ: c_int = 1;
pub const RT5670_CP_FQ_6_KHZ: c_int = 2;
pub const RT5670_CP_FQ_12_KHZ: c_int = 3;
pub const RT5670_CP_FQ_24_KHZ: c_int = 4;
pub const RT5670_CP_FQ_48_KHZ: c_int = 5;
pub const RT5670_CP_FQ_96_KHZ: c_int = 6;
pub const RT5670_CP_FQ_192_KHZ: c_int = 7;
// HPOUT charge pump (0x91)

pub const RT5670_OSW_L_SFT: c_int = 11;

pub const RT5670_OSW_R_SFT: c_int = 10;

pub const RT5670_PM_HP_SFT: c_int = 8;

pub const RT5670_IB_HP_SFT: c_int = 6;

// PV detection and SPK gain control (0x92)

pub const RT5670_PVDD_DET_SFT: c_int = 15;

pub const RT5670_SPK_AG_SFT: c_int = 14;

// Micbias Control (0x93)

pub const RT5670_MIC1_BS_SFT: c_int = 15;

pub const RT5670_MIC2_BS_SFT: c_int = 14;

pub const RT5670_MIC1_CLK_SFT: c_int = 13;

pub const RT5670_MIC2_CLK_SFT: c_int = 12;

pub const RT5670_MIC1_OVCD_SFT: c_int = 11;

pub const RT5670_MIC1_OVTH_SFT: c_int = 9;

pub const RT5670_MIC2_OVCD_SFT: c_int = 8;

pub const RT5670_MIC2_OVTH_SFT: c_int = 6;

pub const RT5670_PWR_MB_SFT: c_int = 5;

pub const RT5670_PWR_CLK25M_SFT: c_int = 4;

// Analog JD Control 1 (0x94)

// VAD Control 4 (0x9d)

pub const RT5670_VAD_SEL_SFT: c_int = 8;
// EQ Control 1 (0xb0)

pub const RT5670_EQ_SRC_SFT: c_int = 15;

pub const RT5670_EQ_UPD_BIT: c_int = 14;

pub const RT5670_EQ_CD_SFT: c_int = 13;

pub const RT5670_EQ_DITH_SFT: c_int = 8;

// EQ Control 2 (0xb1)

pub const RT5670_EQ_HPF1_M_SFT: c_int = 8;

pub const RT5670_EQ_LPF1_M_SFT: c_int = 7;

pub const RT5670_EQ_HPF2_SFT: c_int = 6;

pub const RT5670_EQ_HPF1_SFT: c_int = 5;

pub const RT5670_EQ_BPF4_SFT: c_int = 4;

pub const RT5670_EQ_BPF3_SFT: c_int = 3;

pub const RT5670_EQ_BPF2_SFT: c_int = 2;

pub const RT5670_EQ_BPF1_SFT: c_int = 1;

pub const RT5670_EQ_LPF_SFT: c_int = 0;

// Memory Test (0xb2)

pub const RT5670_MT_SFT: c_int = 15;

// DRC/AGC Control 1 (0xb4)

pub const RT5670_DRC_AGC_P_SFT: c_int = 15;

pub const RT5670_DRC_AGC_SFT: c_int = 14;

pub const RT5670_DRC_AGC_UPD_BIT: c_int = 13;

pub const RT5670_DRC_AGC_AR_SFT: c_int = 8;

pub const RT5670_DRC_AGC_R_SFT: c_int = 5;

pub const RT5670_DRC_AGC_RC_SFT: c_int = 0;
// DRC/AGC Control 2 (0xb5)

pub const RT5670_DRC_AGC_POB_SFT: c_int = 8;

pub const RT5670_DRC_AGC_CP_SFT: c_int = 7;

pub const RT5670_DRC_AGC_CPR_SFT: c_int = 5;

pub const RT5670_DRC_AGC_PRB_SFT: c_int = 0;
// DRC/AGC Control 3 (0xb6)

pub const RT5670_DRC_AGC_NGB_SFT: c_int = 12;

pub const RT5670_DRC_AGC_TAR_SFT: c_int = 7;

pub const RT5670_DRC_AGC_NG_SFT: c_int = 6;

pub const RT5670_DRC_AGC_NGH_SFT: c_int = 5;

pub const RT5670_DRC_AGC_NGT_SFT: c_int = 0;
// Jack Detect Control (0xbb)

pub const RT5670_JD_SFT: c_int = 13;

pub const RT5670_JD_HP_SFT: c_int = 11;

pub const RT5670_JD_HP_TRG_SFT: c_int = 10;

pub const RT5670_JD_SPL_SFT: c_int = 9;

pub const RT5670_JD_SPL_TRG_SFT: c_int = 8;

pub const RT5670_JD_SPR_SFT: c_int = 7;

pub const RT5670_JD_SPR_TRG_SFT: c_int = 6;

pub const RT5670_JD_MO_SFT: c_int = 5;

pub const RT5670_JD_MO_TRG_SFT: c_int = 4;

pub const RT5670_JD_LO_SFT: c_int = 3;

pub const RT5670_JD_LO_TRG_SFT: c_int = 2;

pub const RT5670_JD1_IN4P_SFT: c_int = 1;

pub const RT5670_JD2_IN4N_SFT: c_int = 0;

// IRQ Control 1 (0xbd)

pub const RT5670_IRQ_JD_SFT: c_int = 15;

pub const RT5670_IRQ_OT_SFT: c_int = 14;

pub const RT5670_JD_STKY_SFT: c_int = 13;

pub const RT5670_OT_STKY_SFT: c_int = 12;

pub const RT5670_JD_P_SFT: c_int = 11;

pub const RT5670_OT_P_SFT: c_int = 10;

pub const RT5670_JD1_1_EN_SFT: c_int = 9;

// IRQ Control 2 (0xbe)

pub const RT5670_IRQ_MB1_OC_SFT: c_int = 15;

pub const RT5670_IRQ_MB2_OC_SFT: c_int = 14;

pub const RT5670_MB1_OC_STKY_SFT: c_int = 11;

pub const RT5670_MB2_OC_STKY_SFT: c_int = 10;

pub const RT5670_MB1_OC_P_SFT: c_int = 7;

pub const RT5670_MB2_OC_P_SFT: c_int = 6;

pub const RT5670_MB1_OC_CLR_SFT: c_int = 3;

pub const RT5670_MB2_OC_CLR_SFT: c_int = 2;
// GPIO Control 1 (0xc0)

pub const RT5670_GP1_PIN_SFT: c_int = 15;

pub const RT5670_GP2_PIN_SFT: c_int = 14;

pub const RT5670_GP3_PIN_SFT: c_int = 12;

pub const RT5670_GP4_PIN_SFT: c_int = 11;

pub const RT5670_DP_SIG_SFT: c_int = 10;

pub const RT5670_GPIO_M_SFT: c_int = 9;

pub const RT5670_I2S2_PIN_SFT: c_int = 8;

pub const RT5670_GP5_PIN_SFT: c_int = 7;

pub const RT5670_GP6_PIN_SFT: c_int = 6;

pub const RT5670_GP7_PIN_SFT: c_int = 4;

pub const RT5670_GP8_PIN_SFT: c_int = 3;

pub const RT5670_GP9_PIN_SFT: c_int = 2;

pub const RT5670_GP10_PIN_SFT: c_int = 0;

// GPIO Control 2 (0xc1)

pub const RT5670_GP4_PF_SFT: c_int = 11;

pub const RT5670_GP4_OUT_SFT: c_int = 10;

pub const RT5670_GP4_P_SFT: c_int = 9;

pub const RT5670_GP3_PF_SFT: c_int = 8;

pub const RT5670_GP3_OUT_SFT: c_int = 7;

pub const RT5670_GP3_P_SFT: c_int = 6;

pub const RT5670_GP2_PF_SFT: c_int = 5;

pub const RT5670_GP2_OUT_SFT: c_int = 4;

pub const RT5670_GP2_P_SFT: c_int = 3;

pub const RT5670_GP1_PF_SFT: c_int = 2;

pub const RT5670_GP1_OUT_SFT: c_int = 1;

pub const RT5670_GP1_P_SFT: c_int = 0;

// Scramble Function (0xcd)

pub const RT5670_SCB_KEY_SFT: c_int = 0;
// Scramble Control (0xce)

pub const RT5670_SCB_SWAP_SFT: c_int = 15;

pub const RT5670_SCB_SFT: c_int = 14;

// Baseback Control (0xcf)

pub const RT5670_BB_SFT: c_int = 15;

pub const RT5670_BB_CT_SFT: c_int = 12;

pub const RT5670_M_BB_L_SFT: c_int = 9;

pub const RT5670_M_BB_R_SFT: c_int = 8;

pub const RT5670_M_BB_HPF_L_SFT: c_int = 7;

pub const RT5670_M_BB_HPF_R_SFT: c_int = 6;

pub const RT5670_G_BB_BST_SFT: c_int = 0;
// MP3 Plus Control 1 (0xd0)

pub const RT5670_M_MP3_L_SFT: c_int = 15;

pub const RT5670_M_MP3_R_SFT: c_int = 14;

pub const RT5670_M_MP3_SFT: c_int = 13;

pub const RT5670_EG_MP3_SFT: c_int = 8;

pub const RT5670_MP3_HLP_SFT: c_int = 7;

pub const RT5670_M_MP3_ORG_L_SFT: c_int = 6;

pub const RT5670_M_MP3_ORG_R_SFT: c_int = 5;
// MP3 Plus Control 2 (0xd1)

pub const RT5670_MP3_WT_SFT: c_int = 13;

pub const RT5670_OG_MP3_SFT: c_int = 8;

pub const RT5670_HG_MP3_SFT: c_int = 0;
// 3D HP Control 1 (0xd2)

pub const RT5670_3D_CF_SFT: c_int = 15;

pub const RT5670_3D_HP_SFT: c_int = 14;

pub const RT5670_3D_BT_SFT: c_int = 13;

pub const RT5670_3D_1F_MIX_SFT: c_int = 11;

pub const RT5670_3D_HP_M_SFT: c_int = 10;

pub const RT5670_M_3D_HRTF_SFT: c_int = 9;

pub const RT5670_M_3D_D2H_SFT: c_int = 8;

pub const RT5670_M_3D_D2R_SFT: c_int = 7;

pub const RT5670_M_3D_REVB_SFT: c_int = 6;
// Adjustable high pass filter control 1 (0xd3)

pub const RT5670_2ND_HPF_SFT: c_int = 15;

pub const RT5670_HPF_CF_L_SFT: c_int = 12;

pub const RT5670_1ST_HPF_SFT: c_int = 11;

pub const RT5670_HPF_CF_R_SFT: c_int = 8;

pub const RT5670_ZD_T_SFT: c_int = 6;

pub const RT5670_ZD_F_SFT: c_int = 4;

// HP calibration control and Amp detection (0xd6)

pub const RT5670_SI_DAC_SFT: c_int = 11;

pub const RT5670_DC_CAL_M_SFT: c_int = 10;

pub const RT5670_DC_CAL_SFT: c_int = 9;

pub const RT5670_HPD_RCV_SFT: c_int = 6;

pub const RT5670_HPD_PS_SFT: c_int = 5;

pub const RT5670_CAL_M_SFT: c_int = 4;

pub const RT5670_CAL_SFT: c_int = 3;

pub const RT5670_CAL_TEST_SFT: c_int = 2;

pub const RT5670_CAL_P_SFT: c_int = 0;

// Soft volume and zero cross control 1 (0xd9)

pub const RT5670_SV_SFT: c_int = 15;

pub const RT5670_SPO_SV_SFT: c_int = 14;

pub const RT5670_OUT_SV_SFT: c_int = 13;

pub const RT5670_HP_SV_SFT: c_int = 12;

pub const RT5670_ZCD_DIG_SFT: c_int = 11;

pub const RT5670_ZCD_SFT: c_int = 10;

pub const RT5670_M_ZCD_SFT: c_int = 4;

pub const RT5670_SV_DLY_SFT: c_int = 0;
// Soft volume and zero cross control 2 (0xda)

pub const RT5670_ZCD_HP_SFT: c_int = 15;

// General Control 3 (0xfc)

// Codec Private Register definition
// 3D Speaker Control (0x63)

pub const RT5670_3D_SPK_SFT: c_int = 15;

pub const RT5670_3D_SPK_M_SFT: c_int = 13;

pub const RT5670_3D_SPK_CG_SFT: c_int = 8;

pub const RT5670_3D_SPK_SG_SFT: c_int = 0;
// Wind Noise Detection Control 1 (0x6c)

pub const RT5670_WND_SFT: c_int = 15;

// Wind Noise Detection Control 2 (0x6d)

pub const RT5670_WND_FC_NW_SFT: c_int = 10;

pub const RT5670_WND_FC_WK_SFT: c_int = 4;
// Wind Noise Detection Control 3 (0x6e)

pub const RT5670_HPF_FC_SFT: c_int = 6;

pub const RT5670_WND_FC_ST_SFT: c_int = 0;
// Wind Noise Detection Control 4 (0x6f)

pub const RT5670_WND_TH_LO_SFT: c_int = 0;
// Wind Noise Detection Control 5 (0x70)

pub const RT5670_WND_TH_HI_SFT: c_int = 0;
// Wind Noise Detection Control 8 (0x73)

pub const RT5670_WND_WIND_SFT: c_int = 13;

pub const RT5670_WND_STRONG_SFT: c_int = 12;
// Dipole Speaker Interface (0x75)

pub const RT5670_DP_ATT_SFT: c_int = 14;

pub const RT5670_DP_SPK_SFT: c_int = 10;

// EQ Pre Volume Control (0xb3)

pub const RT5670_EQ_PRE_VOL_SFT: c_int = 0;
// EQ Post Volume Control (0xb4)

pub const RT5670_EQ_PST_VOL_SFT: c_int = 0;
// Jack Detect Control 3 (0xf8)

// Digital Misc Control (0xfa)

pub const RT5670_IF1_ADC1_IN1_SFT: c_int = 12;

pub const RT5670_IF1_ADC1_IN2_SFT: c_int = 11;

pub const RT5670_IF1_ADC2_IN1_SFT: c_int = 10;

// General Control2 (0xfb)

// System Clock Source
// PLL1 Source
// filter mask
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5670_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub jack: *mut snd_soc_jack,
    pub hp_gpio: snd_soc_jack_gpio,
    pub jd_mode: c_int,
    pub in2_diff: bool,
    pub gpio1_is_irq: bool,
    pub gpio1_is_ext_spk_en: bool,
    pub dmic_en: bool,
    pub dmic1_data_pin: c_uint,
// 0 = GPIO6; 1 = IN2P; 3 = GPIO7
    pub dmic2_data_pin: c_uint,
// 0 = GPIO8; 1 = IN3N;
    pub dmic3_data_pin: c_uint,
// 0 = GPIO9; 1 = GPIO10; 2 = GPIO5
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: [c_int; RT5670_AIFS],
    pub bclk: [c_int; RT5670_AIFS],
    pub master: [c_int; RT5670_AIFS],
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
    pub /: *mut *mut int dsp_sw; / expected parameter setting,
    pub dsp_rate: c_int,
    pub jack_type: c_int,
    pub jack_type_saved: c_int,
    pub dac1_mixl_dac1_switch: bool,
    pub dac1_mixr_dac1_switch: bool,
    pub dac1_playback_switch_l: bool,
    pub dac1_playback_switch_r: bool,
}

extern "C" {
    pub fn rt5670_jack_suspend(component: *mut snd_soc_component);
}
extern "C" {
    pub fn rt5670_jack_resume(component: *mut snd_soc_component);
}
