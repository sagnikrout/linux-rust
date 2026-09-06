//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5640.h
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
// rt5640.h  --  RT5640 ALSA SoC audio driver
//
// Copyright 2011 Realtek Microelectronics
// Author: Johnny Hsu <johnnyhsu@realtek.com>
//

// Info
pub const RT5640_RESET: c_uint = 0x00;
pub const RT5640_VENDOR_ID: c_uint = 0xfd;
pub const RT5640_VENDOR_ID1: c_uint = 0xfe;
pub const RT5640_VENDOR_ID2: c_uint = 0xff;
// I/O - Output
pub const RT5640_SPK_VOL: c_uint = 0x01;
pub const RT5640_HP_VOL: c_uint = 0x02;
pub const RT5640_OUTPUT: c_uint = 0x03;
pub const RT5640_MONO_OUT: c_uint = 0x04;
// I/O - Input
pub const RT5640_IN1_IN2: c_uint = 0x0d;
pub const RT5640_IN3_IN4: c_uint = 0x0e;
pub const RT5640_INL_INR_VOL: c_uint = 0x0f;
// I/O - ADC/DAC/DMIC
pub const RT5640_DAC1_DIG_VOL: c_uint = 0x19;
pub const RT5640_DAC2_DIG_VOL: c_uint = 0x1a;
pub const RT5640_DAC2_CTRL: c_uint = 0x1b;
pub const RT5640_ADC_DIG_VOL: c_uint = 0x1c;
pub const RT5640_ADC_DATA: c_uint = 0x1d;
pub const RT5640_ADC_BST_VOL: c_uint = 0x1e;
// Mixer - D-D
pub const RT5640_STO_ADC_MIXER: c_uint = 0x27;
pub const RT5640_MONO_ADC_MIXER: c_uint = 0x28;
pub const RT5640_AD_DA_MIXER: c_uint = 0x29;
pub const RT5640_STO_DAC_MIXER: c_uint = 0x2a;
pub const RT5640_MONO_DAC_MIXER: c_uint = 0x2b;
pub const RT5640_DIG_MIXER: c_uint = 0x2c;
pub const RT5640_DSP_PATH1: c_uint = 0x2d;
pub const RT5640_DSP_PATH2: c_uint = 0x2e;
pub const RT5640_DIG_INF_DATA: c_uint = 0x2f;
// Mixer - ADC
pub const RT5640_REC_L1_MIXER: c_uint = 0x3b;
pub const RT5640_REC_L2_MIXER: c_uint = 0x3c;
pub const RT5640_REC_R1_MIXER: c_uint = 0x3d;
pub const RT5640_REC_R2_MIXER: c_uint = 0x3e;
// Mixer - DAC
pub const RT5640_HPO_MIXER: c_uint = 0x45;
pub const RT5640_SPK_L_MIXER: c_uint = 0x46;
pub const RT5640_SPK_R_MIXER: c_uint = 0x47;
pub const RT5640_SPO_L_MIXER: c_uint = 0x48;
pub const RT5640_SPO_R_MIXER: c_uint = 0x49;
pub const RT5640_SPO_CLSD_RATIO: c_uint = 0x4a;
pub const RT5640_MONO_MIXER: c_uint = 0x4c;
pub const RT5640_OUT_L1_MIXER: c_uint = 0x4d;
pub const RT5640_OUT_L2_MIXER: c_uint = 0x4e;
pub const RT5640_OUT_L3_MIXER: c_uint = 0x4f;
pub const RT5640_OUT_R1_MIXER: c_uint = 0x50;
pub const RT5640_OUT_R2_MIXER: c_uint = 0x51;
pub const RT5640_OUT_R3_MIXER: c_uint = 0x52;
pub const RT5640_LOUT_MIXER: c_uint = 0x53;
// Power
pub const RT5640_PWR_DIG1: c_uint = 0x61;
pub const RT5640_PWR_DIG2: c_uint = 0x62;
pub const RT5640_PWR_ANLG1: c_uint = 0x63;
pub const RT5640_PWR_ANLG2: c_uint = 0x64;
pub const RT5640_PWR_MIXER: c_uint = 0x65;
pub const RT5640_PWR_VOL: c_uint = 0x66;
// Private Register Control
pub const RT5640_PRIV_INDEX: c_uint = 0x6a;
pub const RT5640_PRIV_DATA: c_uint = 0x6c;
// Format - ADC/DAC
pub const RT5640_I2S1_SDP: c_uint = 0x70;
pub const RT5640_I2S2_SDP: c_uint = 0x71;
pub const RT5640_ADDA_CLK1: c_uint = 0x73;
pub const RT5640_ADDA_CLK2: c_uint = 0x74;
pub const RT5640_DMIC: c_uint = 0x75;
// Function - Analog
pub const RT5640_GLB_CLK: c_uint = 0x80;
pub const RT5640_PLL_CTRL1: c_uint = 0x81;
pub const RT5640_PLL_CTRL2: c_uint = 0x82;
pub const RT5640_ASRC_1: c_uint = 0x83;
pub const RT5640_ASRC_2: c_uint = 0x84;
pub const RT5640_ASRC_3: c_uint = 0x85;
pub const RT5640_ASRC_4: c_uint = 0x89;
pub const RT5640_ASRC_5: c_uint = 0x8a;
pub const RT5640_HP_OVCD: c_uint = 0x8b;
pub const RT5640_CLS_D_OVCD: c_uint = 0x8c;
pub const RT5640_CLS_D_OUT: c_uint = 0x8d;
pub const RT5640_DEPOP_M1: c_uint = 0x8e;
pub const RT5640_DEPOP_M2: c_uint = 0x8f;
pub const RT5640_DEPOP_M3: c_uint = 0x90;
pub const RT5640_CHARGE_PUMP: c_uint = 0x91;
pub const RT5640_PV_DET_SPK_G: c_uint = 0x92;
pub const RT5640_MICBIAS: c_uint = 0x93;
// Function - Digital
pub const RT5640_EQ_CTRL1: c_uint = 0xb0;
pub const RT5640_EQ_CTRL2: c_uint = 0xb1;
pub const RT5640_WIND_FILTER: c_uint = 0xb2;
pub const RT5640_DRC_AGC_1: c_uint = 0xb4;
pub const RT5640_DRC_AGC_2: c_uint = 0xb5;
pub const RT5640_DRC_AGC_3: c_uint = 0xb6;
pub const RT5640_SVOL_ZC: c_uint = 0xb7;
pub const RT5640_ANC_CTRL1: c_uint = 0xb8;
pub const RT5640_ANC_CTRL2: c_uint = 0xb9;
pub const RT5640_ANC_CTRL3: c_uint = 0xba;
pub const RT5640_JD_CTRL: c_uint = 0xbb;
pub const RT5640_ANC_JD: c_uint = 0xbc;
pub const RT5640_IRQ_CTRL1: c_uint = 0xbd;
pub const RT5640_IRQ_CTRL2: c_uint = 0xbe;
pub const RT5640_INT_IRQ_ST: c_uint = 0xbf;
pub const RT5640_GPIO_CTRL1: c_uint = 0xc0;
pub const RT5640_GPIO_CTRL2: c_uint = 0xc1;
pub const RT5640_GPIO_CTRL3: c_uint = 0xc2;
pub const RT5640_DSP_CTRL1: c_uint = 0xc4;
pub const RT5640_DSP_CTRL2: c_uint = 0xc5;
pub const RT5640_DSP_CTRL3: c_uint = 0xc6;
pub const RT5640_DSP_CTRL4: c_uint = 0xc7;
pub const RT5640_PGM_REG_ARR1: c_uint = 0xc8;
pub const RT5640_PGM_REG_ARR2: c_uint = 0xc9;
pub const RT5640_PGM_REG_ARR3: c_uint = 0xca;
pub const RT5640_PGM_REG_ARR4: c_uint = 0xcb;
pub const RT5640_PGM_REG_ARR5: c_uint = 0xcc;
pub const RT5640_SCB_FUNC: c_uint = 0xcd;
pub const RT5640_SCB_CTRL: c_uint = 0xce;
pub const RT5640_BASE_BACK: c_uint = 0xcf;
pub const RT5640_MP3_PLUS1: c_uint = 0xd0;
pub const RT5640_MP3_PLUS2: c_uint = 0xd1;
pub const RT5640_3D_HP: c_uint = 0xd2;
pub const RT5640_ADJ_HPF: c_uint = 0xd3;
pub const RT5640_HP_CALIB_AMP_DET: c_uint = 0xd6;
pub const RT5640_HP_CALIB2: c_uint = 0xd7;
pub const RT5640_SV_ZCD1: c_uint = 0xd9;
pub const RT5640_SV_ZCD2: c_uint = 0xda;
// Dummy Register
pub const RT5640_GCTL1: c_uint = 0xfa;
pub const RT5640_GCTL2: c_uint = 0xfb;
pub const RT5640_DUMMY3: c_uint = 0xfc;
// Index of Codec Private Register definition
pub const RT5640_BIAS_CUR4: c_uint = 0x15;
pub const RT5640_CHPUMP_INT_REG1: c_uint = 0x24;
pub const RT5640_MAMP_INT_REG2: c_uint = 0x37;
pub const RT5640_3D_SPK: c_uint = 0x63;
pub const RT5640_WND_1: c_uint = 0x6c;
pub const RT5640_WND_2: c_uint = 0x6d;
pub const RT5640_WND_3: c_uint = 0x6e;
pub const RT5640_WND_4: c_uint = 0x6f;
pub const RT5640_WND_5: c_uint = 0x70;
pub const RT5640_WND_8: c_uint = 0x73;
pub const RT5640_DIP_SPK_INF: c_uint = 0x75;
pub const RT5640_HP_DCC_INT1: c_uint = 0x77;
pub const RT5640_EQ_BW_LOP: c_uint = 0xa0;
pub const RT5640_EQ_GN_LOP: c_uint = 0xa1;
pub const RT5640_EQ_FC_BP1: c_uint = 0xa2;
pub const RT5640_EQ_BW_BP1: c_uint = 0xa3;
pub const RT5640_EQ_GN_BP1: c_uint = 0xa4;
pub const RT5640_EQ_FC_BP2: c_uint = 0xa5;
pub const RT5640_EQ_BW_BP2: c_uint = 0xa6;
pub const RT5640_EQ_GN_BP2: c_uint = 0xa7;
pub const RT5640_EQ_FC_BP3: c_uint = 0xa8;
pub const RT5640_EQ_BW_BP3: c_uint = 0xa9;
pub const RT5640_EQ_GN_BP3: c_uint = 0xaa;
pub const RT5640_EQ_FC_BP4: c_uint = 0xab;
pub const RT5640_EQ_BW_BP4: c_uint = 0xac;
pub const RT5640_EQ_GN_BP4: c_uint = 0xad;
pub const RT5640_EQ_FC_HIP1: c_uint = 0xae;
pub const RT5640_EQ_GN_HIP1: c_uint = 0xaf;
pub const RT5640_EQ_FC_HIP2: c_uint = 0xb0;
pub const RT5640_EQ_BW_HIP2: c_uint = 0xb1;
pub const RT5640_EQ_GN_HIP2: c_uint = 0xb2;
pub const RT5640_EQ_PRE_VOL: c_uint = 0xb3;
pub const RT5640_EQ_PST_VOL: c_uint = 0xb4;
// global definition

pub const RT5640_L_MUTE_SFT: c_int = 15;

pub const RT5640_VOL_L_SFT: c_int = 14;

pub const RT5640_R_MUTE_SFT: c_int = 7;

pub const RT5640_VOL_R_SFT: c_int = 6;

pub const RT5640_L_VOL_SFT: c_int = 8;

pub const RT5640_R_VOL_SFT: c_int = 0;
// SW Reset & Device ID (0x00)

// IN1 and IN2 Control (0x0d)
// IN3 and IN4 Control (0x0e)
pub const RT5640_BST_SFT1: c_int = 12;
pub const RT5640_BST_SFT2: c_int = 8;

pub const RT5640_IN_SFT1: c_int = 7;

pub const RT5640_IN_SFT2: c_int = 6;
// INL and INR Volume Control (0x0f)

pub const RT5640_INL_SEL_SFT: c_int = 15;

pub const RT5640_INL_VOL_SFT: c_int = 8;

pub const RT5640_INR_SEL_SFT: c_int = 7;

pub const RT5640_INR_VOL_SFT: c_int = 0;
// DAC1 Digital Volume (0x19)

pub const RT5640_DAC_L1_VOL_SFT: c_int = 8;

pub const RT5640_DAC_R1_VOL_SFT: c_int = 0;
// DAC2 Digital Volume (0x1a)

pub const RT5640_DAC_L2_VOL_SFT: c_int = 8;

pub const RT5640_DAC_R2_VOL_SFT: c_int = 0;
// DAC2 Control (0x1b)

pub const RT5640_M_DAC_L2_VOL_SFT: c_int = 13;

pub const RT5640_M_DAC_R2_VOL_SFT: c_int = 12;
// ADC Digital Volume Control (0x1c)

pub const RT5640_ADC_L_VOL_SFT: c_int = 8;

pub const RT5640_ADC_R_VOL_SFT: c_int = 0;
// Mono ADC Digital Volume Control (0x1d)

pub const RT5640_MONO_ADC_L_VOL_SFT: c_int = 8;

pub const RT5640_MONO_ADC_R_VOL_SFT: c_int = 0;
// ADC Boost Volume Control (0x1e)

pub const RT5640_ADC_L_BST_SFT: c_int = 14;

pub const RT5640_ADC_R_BST_SFT: c_int = 12;

pub const RT5640_ADC_COMP_SFT: c_int = 10;
// Stereo ADC Mixer Control (0x27)

pub const RT5640_M_ADC_L1_SFT: c_int = 14;

pub const RT5640_M_ADC_L2_SFT: c_int = 13;

pub const RT5640_ADC_1_SRC_SFT: c_int = 12;

pub const RT5640_ADC_2_SRC_SFT: c_int = 10;

pub const RT5640_M_ADC_R1_SFT: c_int = 6;

pub const RT5640_M_ADC_R2_SFT: c_int = 5;
// Mono ADC Mixer Control (0x28)

pub const RT5640_M_MONO_ADC_L1_SFT: c_int = 14;

pub const RT5640_M_MONO_ADC_L2_SFT: c_int = 13;

pub const RT5640_MONO_ADC_L1_SRC_SFT: c_int = 12;

pub const RT5640_MONO_ADC_L2_SRC_SFT: c_int = 10;

pub const RT5640_M_MONO_ADC_R1_SFT: c_int = 6;

pub const RT5640_M_MONO_ADC_R2_SFT: c_int = 5;

pub const RT5640_MONO_ADC_R1_SRC_SFT: c_int = 4;

pub const RT5640_MONO_ADC_R2_SRC_SFT: c_int = 2;

// ADC Mixer to DAC Mixer Control (0x29)

pub const RT5640_M_ADCMIX_L_SFT: c_int = 15;

pub const RT5640_M_IF1_DAC_L_SFT: c_int = 14;

pub const RT5640_M_ADCMIX_R_SFT: c_int = 7;

pub const RT5640_M_IF1_DAC_R_SFT: c_int = 6;
// Stereo DAC Mixer Control (0x2a)

pub const RT5640_M_DAC_L1_SFT: c_int = 14;

pub const RT5640_DAC_L1_STO_L_VOL_SFT: c_int = 13;

pub const RT5640_M_DAC_L2_SFT: c_int = 12;

pub const RT5640_DAC_L2_STO_L_VOL_SFT: c_int = 11;

pub const RT5640_M_ANC_DAC_L_SFT: c_int = 10;

pub const RT5640_M_DAC_R1_SFT: c_int = 6;

pub const RT5640_DAC_R1_STO_R_VOL_SFT: c_int = 5;

pub const RT5640_M_DAC_R2_SFT: c_int = 4;

pub const RT5640_DAC_R2_STO_R_VOL_SFT: c_int = 3;

pub const RT5640_M_ANC_DAC_R_SFT: c_int = 2;
// Mono DAC Mixer Control (0x2b)

pub const RT5640_M_DAC_L1_MONO_L_SFT: c_int = 14;

pub const RT5640_DAC_L1_MONO_L_VOL_SFT: c_int = 13;

pub const RT5640_M_DAC_L2_MONO_L_SFT: c_int = 12;

pub const RT5640_DAC_L2_MONO_L_VOL_SFT: c_int = 11;

pub const RT5640_M_DAC_R2_MONO_L_SFT: c_int = 10;

pub const RT5640_DAC_R2_MONO_L_VOL_SFT: c_int = 9;

pub const RT5640_M_DAC_R1_MONO_R_SFT: c_int = 6;

pub const RT5640_DAC_R1_MONO_R_VOL_SFT: c_int = 5;

pub const RT5640_M_DAC_R2_MONO_R_SFT: c_int = 4;

pub const RT5640_DAC_R2_MONO_R_VOL_SFT: c_int = 3;

pub const RT5640_M_DAC_L2_MONO_R_SFT: c_int = 2;

pub const RT5640_DAC_L2_MONO_R_VOL_SFT: c_int = 1;
// Digital Mixer Control (0x2c)

pub const RT5640_M_STO_L_DAC_L_SFT: c_int = 15;

pub const RT5640_STO_L_DAC_L_VOL_SFT: c_int = 14;

pub const RT5640_M_DAC_L2_DAC_L_SFT: c_int = 13;

pub const RT5640_DAC_L2_DAC_L_VOL_SFT: c_int = 12;

pub const RT5640_M_STO_R_DAC_R_SFT: c_int = 11;

pub const RT5640_STO_R_DAC_R_VOL_SFT: c_int = 10;

pub const RT5640_M_DAC_R2_DAC_R_SFT: c_int = 9;

pub const RT5640_DAC_R2_DAC_R_VOL_SFT: c_int = 8;
// DSP Path Control 1 (0x2d)

pub const RT5640_RXDP_SRC_SFT: c_int = 15;

pub const RT5640_TXDP_SRC_SFT: c_int = 14;

// DSP Path Control 2 (0x2e)

pub const RT5640_DAC_L2_SEL_SFT: c_int = 14;

pub const RT5640_DAC_R2_SEL_SFT: c_int = 12;

pub const RT5640_IF2_ADC_L_SEL_SFT: c_int = 11;

pub const RT5640_IF2_ADC_R_SEL_SFT: c_int = 10;

pub const RT5640_RXDC_SEL_SFT: c_int = 8;

pub const RT5640_RXDP_SEL_SFT: c_int = 6;

pub const RT5640_TXDC_SEL_SFT: c_int = 4;

pub const RT5640_TXDP_SEL_SFT: c_int = 2;

// Digital Interface Data Control (0x2f)

pub const RT5640_IF1_DAC_SEL_SFT: c_int = 14;

pub const RT5640_IF1_ADC_SEL_SFT: c_int = 12;

pub const RT5640_IF2_DAC_SEL_SFT: c_int = 10;

pub const RT5640_IF2_ADC_SEL_SFT: c_int = 8;

pub const RT5640_IF3_DAC_SEL_SFT: c_int = 6;

pub const RT5640_IF3_ADC_SEL_SFT: c_int = 4;

// REC Left Mixer Control 1 (0x3b)

pub const RT5640_G_HP_L_RM_L_SFT: c_int = 13;

pub const RT5640_G_IN_L_RM_L_SFT: c_int = 10;

pub const RT5640_G_BST4_RM_L_SFT: c_int = 7;

pub const RT5640_G_BST3_RM_L_SFT: c_int = 4;

pub const RT5640_G_BST2_RM_L_SFT: c_int = 1;
// REC Left Mixer Control 2 (0x3c)

pub const RT5640_G_BST1_RM_L_SFT: c_int = 13;

pub const RT5640_G_OM_L_RM_L_SFT: c_int = 10;

pub const RT5640_M_HP_L_RM_L_SFT: c_int = 6;

pub const RT5640_M_IN_L_RM_L_SFT: c_int = 5;

pub const RT5640_M_BST4_RM_L_SFT: c_int = 4;

pub const RT5640_M_BST3_RM_L_SFT: c_int = 3;

pub const RT5640_M_BST2_RM_L_SFT: c_int = 2;

pub const RT5640_M_BST1_RM_L_SFT: c_int = 1;

pub const RT5640_M_OM_L_RM_L_SFT: c_int = 0;
// REC Right Mixer Control 1 (0x3d)

pub const RT5640_G_HP_R_RM_R_SFT: c_int = 13;

pub const RT5640_G_IN_R_RM_R_SFT: c_int = 10;

pub const RT5640_G_BST4_RM_R_SFT: c_int = 7;

pub const RT5640_G_BST3_RM_R_SFT: c_int = 4;

pub const RT5640_G_BST2_RM_R_SFT: c_int = 1;
// REC Right Mixer Control 2 (0x3e)

pub const RT5640_G_BST1_RM_R_SFT: c_int = 13;

pub const RT5640_G_OM_R_RM_R_SFT: c_int = 10;

pub const RT5640_M_HP_R_RM_R_SFT: c_int = 6;

pub const RT5640_M_IN_R_RM_R_SFT: c_int = 5;

pub const RT5640_M_BST4_RM_R_SFT: c_int = 4;

pub const RT5640_M_BST3_RM_R_SFT: c_int = 3;

pub const RT5640_M_BST2_RM_R_SFT: c_int = 2;

pub const RT5640_M_BST1_RM_R_SFT: c_int = 1;

pub const RT5640_M_OM_R_RM_R_SFT: c_int = 0;
// HPMIX Control (0x45)

pub const RT5640_M_DAC2_HM_SFT: c_int = 15;

pub const RT5640_M_DAC1_HM_SFT: c_int = 14;

pub const RT5640_M_HPVOL_HM_SFT: c_int = 13;

pub const RT5640_G_HPOMIX_SFT: c_int = 12;
// SPK Left Mixer Control (0x46)

pub const RT5640_G_RM_L_SM_L_SFT: c_int = 14;

pub const RT5640_G_IN_L_SM_L_SFT: c_int = 12;

pub const RT5640_G_DAC_L1_SM_L_SFT: c_int = 10;

pub const RT5640_G_DAC_L2_SM_L_SFT: c_int = 8;

pub const RT5640_G_OM_L_SM_L_SFT: c_int = 6;

pub const RT5640_M_RM_L_SM_L_SFT: c_int = 5;

pub const RT5640_M_IN_L_SM_L_SFT: c_int = 4;

pub const RT5640_M_DAC_L1_SM_L_SFT: c_int = 3;

pub const RT5640_M_DAC_L2_SM_L_SFT: c_int = 2;

pub const RT5640_M_OM_L_SM_L_SFT: c_int = 1;
// SPK Right Mixer Control (0x47)

pub const RT5640_G_RM_R_SM_R_SFT: c_int = 14;

pub const RT5640_G_IN_R_SM_R_SFT: c_int = 12;

pub const RT5640_G_DAC_R1_SM_R_SFT: c_int = 10;

pub const RT5640_G_DAC_R2_SM_R_SFT: c_int = 8;

pub const RT5640_G_OM_R_SM_R_SFT: c_int = 6;

pub const RT5640_M_RM_R_SM_R_SFT: c_int = 5;

pub const RT5640_M_IN_R_SM_R_SFT: c_int = 4;

pub const RT5640_M_DAC_R1_SM_R_SFT: c_int = 3;

pub const RT5640_M_DAC_R2_SM_R_SFT: c_int = 2;

pub const RT5640_M_OM_R_SM_R_SFT: c_int = 1;
// SPOLMIX Control (0x48)

pub const RT5640_M_DAC_R1_SPM_L_SFT: c_int = 15;

pub const RT5640_M_DAC_L1_SPM_L_SFT: c_int = 14;

pub const RT5640_M_SV_R_SPM_L_SFT: c_int = 13;

pub const RT5640_M_SV_L_SPM_L_SFT: c_int = 12;

pub const RT5640_M_BST1_SPM_L_SFT: c_int = 11;
// SPORMIX Control (0x49)

pub const RT5640_M_DAC_R1_SPM_R_SFT: c_int = 13;

pub const RT5640_M_SV_R_SPM_R_SFT: c_int = 12;

pub const RT5640_M_BST1_SPM_R_SFT: c_int = 11;
// SPOLMIX / SPORMIX Ratio Control (0x4a)

pub const RT5640_SPO_CLSD_RATIO_SFT: c_int = 0;
// Mono Output Mixer Control (0x4c)

pub const RT5640_M_DAC_R2_MM_SFT: c_int = 15;

pub const RT5640_M_DAC_L2_MM_SFT: c_int = 14;

pub const RT5640_M_OV_R_MM_SFT: c_int = 13;

pub const RT5640_M_OV_L_MM_SFT: c_int = 12;

pub const RT5640_M_BST1_MM_SFT: c_int = 11;

pub const RT5640_G_MONOMIX_SFT: c_int = 10;
// Output Left Mixer Control 1 (0x4d)

pub const RT5640_G_BST3_OM_L_SFT: c_int = 13;

pub const RT5640_G_BST2_OM_L_SFT: c_int = 10;

pub const RT5640_G_BST1_OM_L_SFT: c_int = 7;

pub const RT5640_G_IN_L_OM_L_SFT: c_int = 4;

pub const RT5640_G_RM_L_OM_L_SFT: c_int = 1;
// Output Left Mixer Control 2 (0x4e)

pub const RT5640_G_DAC_R2_OM_L_SFT: c_int = 13;

pub const RT5640_G_DAC_L2_OM_L_SFT: c_int = 10;

pub const RT5640_G_DAC_L1_OM_L_SFT: c_int = 7;
// Output Left Mixer Control 3 (0x4f)

pub const RT5640_M_SM_L_OM_L_SFT: c_int = 8;

pub const RT5640_M_BST3_OM_L_SFT: c_int = 7;

pub const RT5640_M_BST2_OM_L_SFT: c_int = 6;

pub const RT5640_M_BST1_OM_L_SFT: c_int = 5;

pub const RT5640_M_IN_L_OM_L_SFT: c_int = 4;

pub const RT5640_M_RM_L_OM_L_SFT: c_int = 3;

pub const RT5640_M_DAC_R2_OM_L_SFT: c_int = 2;

pub const RT5640_M_DAC_L2_OM_L_SFT: c_int = 1;

pub const RT5640_M_DAC_L1_OM_L_SFT: c_int = 0;
// Output Right Mixer Control 1 (0x50)

pub const RT5640_G_BST4_OM_R_SFT: c_int = 13;

pub const RT5640_G_BST2_OM_R_SFT: c_int = 10;

pub const RT5640_G_BST1_OM_R_SFT: c_int = 7;

pub const RT5640_G_IN_R_OM_R_SFT: c_int = 4;

pub const RT5640_G_RM_R_OM_R_SFT: c_int = 1;
// Output Right Mixer Control 2 (0x51)

pub const RT5640_G_DAC_L2_OM_R_SFT: c_int = 13;

pub const RT5640_G_DAC_R2_OM_R_SFT: c_int = 10;

pub const RT5640_G_DAC_R1_OM_R_SFT: c_int = 7;
// Output Right Mixer Control 3 (0x52)

pub const RT5640_M_SM_L_OM_R_SFT: c_int = 8;

pub const RT5640_M_BST4_OM_R_SFT: c_int = 7;

pub const RT5640_M_BST2_OM_R_SFT: c_int = 6;

pub const RT5640_M_BST1_OM_R_SFT: c_int = 5;

pub const RT5640_M_IN_R_OM_R_SFT: c_int = 4;

pub const RT5640_M_RM_R_OM_R_SFT: c_int = 3;

pub const RT5640_M_DAC_L2_OM_R_SFT: c_int = 2;

pub const RT5640_M_DAC_R2_OM_R_SFT: c_int = 1;

pub const RT5640_M_DAC_R1_OM_R_SFT: c_int = 0;
// LOUT Mixer Control (0x53)

pub const RT5640_M_DAC_L1_LM_SFT: c_int = 15;

pub const RT5640_M_DAC_R1_LM_SFT: c_int = 14;

pub const RT5640_M_OV_L_LM_SFT: c_int = 13;

pub const RT5640_M_OV_R_LM_SFT: c_int = 12;

pub const RT5640_G_LOUTMIX_SFT: c_int = 11;
// Power Management for Digital 1 (0x61)

pub const RT5640_PWR_I2S1_BIT: c_int = 15;

pub const RT5640_PWR_I2S2_BIT: c_int = 14;

pub const RT5640_PWR_DAC_L1_BIT: c_int = 12;

pub const RT5640_PWR_DAC_R1_BIT: c_int = 11;

pub const RT5640_PWR_DAC_L2_BIT: c_int = 7;

pub const RT5640_PWR_DAC_R2_BIT: c_int = 6;

pub const RT5640_PWR_ADC_L_BIT: c_int = 2;

pub const RT5640_PWR_ADC_R_BIT: c_int = 1;

pub const RT5640_PWR_CLS_D_BIT: c_int = 0;
// Power Management for Digital 2 (0x62)

pub const RT5640_PWR_ADC_SF_BIT: c_int = 15;

pub const RT5640_PWR_ADC_MF_L_BIT: c_int = 14;

pub const RT5640_PWR_ADC_MF_R_BIT: c_int = 13;

pub const RT5640_PWR_I2S_DSP_BIT: c_int = 12;
// Power Management for Analog 1 (0x63)

pub const RT5640_PWR_VREF1_BIT: c_int = 15;

pub const RT5640_PWR_FV1_BIT: c_int = 14;

pub const RT5640_PWR_MB_BIT: c_int = 13;

pub const RT5640_PWR_LM_BIT: c_int = 12;

pub const RT5640_PWR_BG_BIT: c_int = 11;

pub const RT5640_PWR_MM_BIT: c_int = 10;

pub const RT5640_PWR_MA_BIT: c_int = 8;

pub const RT5640_PWR_HP_L_BIT: c_int = 7;

pub const RT5640_PWR_HP_R_BIT: c_int = 6;

pub const RT5640_PWR_HA_BIT: c_int = 5;

pub const RT5640_PWR_VREF2_BIT: c_int = 4;

pub const RT5640_PWR_FV2_BIT: c_int = 3;

pub const RT5640_PWR_LDO2_BIT: c_int = 2;
// Power Management for Analog 2 (0x64)

pub const RT5640_PWR_BST1_BIT: c_int = 15;

pub const RT5640_PWR_BST2_BIT: c_int = 14;

pub const RT5640_PWR_BST3_BIT: c_int = 13;

pub const RT5640_PWR_BST4_BIT: c_int = 12;

pub const RT5640_PWR_MB1_BIT: c_int = 11;

pub const RT5640_PWR_PLL_BIT: c_int = 9;
// Power Management for Mixer (0x65)

pub const RT5640_PWR_OM_L_BIT: c_int = 15;

pub const RT5640_PWR_OM_R_BIT: c_int = 14;

pub const RT5640_PWR_SM_L_BIT: c_int = 13;

pub const RT5640_PWR_SM_R_BIT: c_int = 12;

pub const RT5640_PWR_RM_L_BIT: c_int = 11;

pub const RT5640_PWR_RM_R_BIT: c_int = 10;
// Power Management for Volume (0x66)

pub const RT5640_PWR_SV_L_BIT: c_int = 15;

pub const RT5640_PWR_SV_R_BIT: c_int = 14;

pub const RT5640_PWR_OV_L_BIT: c_int = 13;

pub const RT5640_PWR_OV_R_BIT: c_int = 12;

pub const RT5640_PWR_HV_L_BIT: c_int = 11;

pub const RT5640_PWR_HV_R_BIT: c_int = 10;

pub const RT5640_PWR_IN_L_BIT: c_int = 9;

pub const RT5640_PWR_IN_R_BIT: c_int = 8;
// I2S1/2/3 Audio Serial Data Port Control (0x70 0x71 0x72)

pub const RT5640_I2S_MS_SFT: c_int = 15;

pub const RT5640_I2S_IF_SFT: c_int = 12;

pub const RT5640_I2S_O_CP_SFT: c_int = 10;

pub const RT5640_I2S_I_CP_SFT: c_int = 8;

pub const RT5640_I2S_BP_SFT: c_int = 7;

pub const RT5640_I2S_DL_SFT: c_int = 2;

pub const RT5640_I2S_DF_SFT: c_int = 0;

// I2S2 Audio Serial Data Port Control (0x71)

pub const RT5640_I2S2_SDI_SFT: c_int = 6;

// ADC/DAC Clock Control 1 (0x73)

pub const RT5640_I2S_BCLK_MS1_SFT: c_int = 15;

pub const RT5640_I2S_PD1_SFT: c_int = 12;

pub const RT5640_I2S_BCLK_MS2_SFT: c_int = 11;

pub const RT5640_I2S_PD2_SFT: c_int = 8;

pub const RT5640_I2S_BCLK_MS3_SFT: c_int = 7;

pub const RT5640_I2S_PD3_SFT: c_int = 4;

pub const RT5640_DAC_OSR_SFT: c_int = 2;

pub const RT5640_ADC_OSR_SFT: c_int = 0;

// ADC/DAC Clock Control 2 (0x74)

pub const RT5640_DAC_L_OSR_SFT: c_int = 14;

pub const RT5640_ADC_R_OSR_SFT: c_int = 12;

pub const RT5640_DAHPF_EN_SFT: c_int = 11;

pub const RT5640_ADHPF_EN_SFT: c_int = 10;
// Digital Microphone Control (0x75)

pub const RT5640_DMIC_1_EN_SFT: c_int = 15;

pub const RT5640_DMIC_2_EN_SFT: c_int = 14;

pub const RT5640_DMIC_1L_LH_SFT: c_int = 13;

pub const RT5640_DMIC_1R_LH_SFT: c_int = 12;

pub const RT5640_DMIC_1_DP_SFT: c_int = 11;

pub const RT5640_DMIC_2_DP_SFT: c_int = 10;

pub const RT5640_DMIC_2L_LH_SFT: c_int = 9;

pub const RT5640_DMIC_2R_LH_SFT: c_int = 8;

pub const RT5640_DMIC_CLK_SFT: c_int = 5;
// Global Clock Control (0x80)

pub const RT5640_SCLK_SRC_SFT: c_int = 14;

pub const RT5640_PLL1_SRC_SFT: c_int = 12;

pub const RT5640_PLL1_PD_SFT: c_int = 3;

pub const RT5640_PLL_INP_MAX: c_int = 40000000;
pub const RT5640_PLL_INP_MIN: c_int = 256000;
// PLL M/N/K Code Control 1 (0x81)
pub const RT5640_PLL_N_MAX: c_uint = 0x1ff;

pub const RT5640_PLL_N_SFT: c_int = 7;
pub const RT5640_PLL_K_MAX: c_uint = 0x1f;

pub const RT5640_PLL_K_SFT: c_int = 0;
// PLL M/N/K Code Control 2 (0x82)
pub const RT5640_PLL_M_MAX: c_uint = 0xf;

pub const RT5640_PLL_M_SFT: c_int = 12;

pub const RT5640_PLL_M_BP_SFT: c_int = 11;
// ASRC Control 1 (0x83)

pub const RT5640_STO_T_SFT: c_int = 15;

pub const RT5640_M1_T_SFT: c_int = 14;

pub const RT5640_I2S2_F_SFT: c_int = 12;

pub const RT5640_DMIC_1_M_SFT: c_int = 9;

pub const RT5640_DMIC_2_M_SFT: c_int = 8;

// ASRC clock source selection (0x84)

// ASRC Control 2 (0x84)

pub const RT5640_MDA_L_M_SFT: c_int = 15;

pub const RT5640_MDA_R_M_SFT: c_int = 14;

pub const RT5640_MAD_L_M_SFT: c_int = 13;

pub const RT5640_MAD_R_M_SFT: c_int = 12;

pub const RT5640_ADC_M_SFT: c_int = 11;

pub const RT5640_STO_DAC_M_SFT: c_int = 5;

pub const RT5640_I2S1_R_D_SFT: c_int = 4;

pub const RT5640_I2S2_R_D_SFT: c_int = 3;

pub const RT5640_PRE_SCLK_SFT: c_int = 0;

// ASRC Control 3 (0x85)

pub const RT5640_I2S1_RATE_SFT: c_int = 12;

pub const RT5640_I2S2_RATE_SFT: c_int = 8;
// ASRC Control 4 (0x89)

pub const RT5640_I2S1_PD_SFT: c_int = 12;

pub const RT5640_I2S2_PD_SFT: c_int = 8;
// HPOUT Over Current Detection (0x8b)

pub const RT5640_HP_OVCD_SFT: c_int = 10;

pub const RT5640_HP_OC_TH_SFT: c_int = 8;

// Class D Over Current Control (0x8c)

pub const RT5640_CLSD_OC_SFT: c_int = 9;

pub const RT5640_AUTO_PD_SFT: c_int = 8;

pub const RT5640_CLSD_OC_TH_SFT: c_int = 0;
// Class D Output Control (0x8d)

pub const RT5640_CLSD_RATIO_SFT: c_int = 12;

pub const RT5640_CLSD_OM_SFT: c_int = 11;

pub const RT5640_CLSD_SCH_SFT: c_int = 10;

// Depop Mode Control 1 (0x8e)

pub const RT5640_SMT_TRIG_SFT: c_int = 15;

pub const RT5640_HP_L_SMT_SFT: c_int = 9;

pub const RT5640_HP_R_SMT_SFT: c_int = 8;

pub const RT5640_HP_CD_PD_SFT: c_int = 7;

pub const RT5640_RSTN_SFT: c_int = 6;

pub const RT5640_RSTP_SFT: c_int = 5;

pub const RT5640_HP_CO_SFT: c_int = 4;

pub const RT5640_HP_CP_SFT: c_int = 3;

pub const RT5640_HP_SG_SFT: c_int = 2;

pub const RT5640_HP_DP_SFT: c_int = 1;

pub const RT5640_HP_CB_SFT: c_int = 0;

// Depop Mode Control 2 (0x8f)

pub const RT5640_DEPOP_SFT: c_int = 13;

pub const RT5640_RAMP_SFT: c_int = 12;

pub const RT5640_BPS_SFT: c_int = 11;

pub const RT5640_FAST_UPDN_SFT: c_int = 10;

pub const RT5640_MRES_SFT: c_int = 8;

pub const RT5640_VLO_SFT: c_int = 7;

pub const RT5640_DIG_DP_SFT: c_int = 6;

pub const RT5640_DP_TH_SFT: c_int = 4;
// Depop Mode Control 3 (0x90)

pub const RT5640_CP_SYS_SFT: c_int = 12;

pub const RT5640_CP_FQ1_SFT: c_int = 8;

pub const RT5640_CP_FQ2_SFT: c_int = 4;

pub const RT5640_CP_FQ3_SFT: c_int = 0;
pub const RT5640_CP_FQ_1_5_KHZ: c_int = 0;
pub const RT5640_CP_FQ_3_KHZ: c_int = 1;
pub const RT5640_CP_FQ_6_KHZ: c_int = 2;
pub const RT5640_CP_FQ_12_KHZ: c_int = 3;
pub const RT5640_CP_FQ_24_KHZ: c_int = 4;
pub const RT5640_CP_FQ_48_KHZ: c_int = 5;
pub const RT5640_CP_FQ_96_KHZ: c_int = 6;
pub const RT5640_CP_FQ_192_KHZ: c_int = 7;
// HPOUT charge pump (0x91)

pub const RT5640_OSW_L_SFT: c_int = 11;

pub const RT5640_OSW_R_SFT: c_int = 10;

pub const RT5640_PM_HP_SFT: c_int = 8;

pub const RT5640_IB_HP_SFT: c_int = 6;

// PV detection and SPK gain control (0x92)

pub const RT5640_PVDD_DET_SFT: c_int = 15;

pub const RT5640_SPK_AG_SFT: c_int = 14;

// Micbias Control (0x93)

pub const RT5640_MIC1_BS_SFT: c_int = 15;

pub const RT5640_MIC2_BS_SFT: c_int = 14;

pub const RT5640_MIC1_CLK_SFT: c_int = 13;

pub const RT5640_MIC2_CLK_SFT: c_int = 12;

pub const RT5640_MIC1_OVCD_SFT: c_int = 11;

pub const RT5640_MIC1_OVTH_SFT: c_int = 9;

pub const RT5640_MIC2_OVCD_SFT: c_int = 8;

pub const RT5640_MIC2_OVTH_SFT: c_int = 6;

pub const RT5640_PWR_MB_SFT: c_int = 5;

pub const RT5640_PWR_CLK25M_SFT: c_int = 4;

// EQ Control 1 (0xb0)

pub const RT5640_EQ_SRC_SFT: c_int = 15;

pub const RT5640_EQ_UPD_BIT: c_int = 14;

pub const RT5640_EQ_CD_SFT: c_int = 13;

pub const RT5640_EQ_DITH_SFT: c_int = 8;

// EQ Control 2 (0xb1)

pub const RT5640_EQ_HPF1_M_SFT: c_int = 8;

pub const RT5640_EQ_LPF1_M_SFT: c_int = 7;

pub const RT5640_EQ_HPF2_SFT: c_int = 6;

pub const RT5640_EQ_HPF1_SFT: c_int = 5;

pub const RT5640_EQ_BPF4_SFT: c_int = 4;

pub const RT5640_EQ_BPF3_SFT: c_int = 3;

pub const RT5640_EQ_BPF2_SFT: c_int = 2;

pub const RT5640_EQ_BPF1_SFT: c_int = 1;

pub const RT5640_EQ_LPF_SFT: c_int = 0;

// Memory Test (0xb2)

pub const RT5640_MT_SFT: c_int = 15;

// DRC/AGC Control 1 (0xb4)

pub const RT5640_DRC_AGC_P_SFT: c_int = 15;

pub const RT5640_DRC_AGC_SFT: c_int = 14;

pub const RT5640_DRC_AGC_UPD_BIT: c_int = 13;

pub const RT5640_DRC_AGC_AR_SFT: c_int = 8;

pub const RT5640_DRC_AGC_R_SFT: c_int = 5;

pub const RT5640_DRC_AGC_RC_SFT: c_int = 0;
// DRC/AGC Control 2 (0xb5)

pub const RT5640_DRC_AGC_POB_SFT: c_int = 8;

pub const RT5640_DRC_AGC_CP_SFT: c_int = 7;

pub const RT5640_DRC_AGC_CPR_SFT: c_int = 5;

pub const RT5640_DRC_AGC_PRB_SFT: c_int = 0;
// DRC/AGC Control 3 (0xb6)

pub const RT5640_DRC_AGC_NGB_SFT: c_int = 12;

pub const RT5640_DRC_AGC_TAR_SFT: c_int = 7;

pub const RT5640_DRC_AGC_NG_SFT: c_int = 6;

pub const RT5640_DRC_AGC_NGH_SFT: c_int = 5;

pub const RT5640_DRC_AGC_NGT_SFT: c_int = 0;
// ANC Control 1 (0xb8)

pub const RT5640_ANC_M_SFT: c_int = 15;

pub const RT5640_ANC_SFT: c_int = 14;

pub const RT5640_ANC_MD_SFT: c_int = 12;

pub const RT5640_ANC_SN_SFT: c_int = 11;

pub const RT5640_ANC_CLK_SFT: c_int = 10;

pub const RT5640_ANC_ZCD_SFT: c_int = 8;

pub const RT5640_ANC_CS_SFT: c_int = 7;

pub const RT5640_ANC_SW_SFT: c_int = 6;

pub const RT5640_ANC_CO_L_SFT: c_int = 0;
// ANC Control 2 (0xb6)

pub const RT5640_ANC_FG_R_SFT: c_int = 12;

pub const RT5640_ANC_FG_L_SFT: c_int = 8;

pub const RT5640_ANC_CG_R_SFT: c_int = 4;

pub const RT5640_ANC_CG_L_SFT: c_int = 0;
// ANC Control 3 (0xb6)

pub const RT5640_ANC_CD_SFT: c_int = 6;

pub const RT5640_ANC_CO_R_SFT: c_int = 0;
// Jack Detect Control (0xbb)

pub const RT5640_JD_SFT: c_int = 13;

pub const RT5640_JD_HP_SFT: c_int = 11;

pub const RT5640_JD_HP_TRG_SFT: c_int = 10;

pub const RT5640_JD_SPL_SFT: c_int = 9;

pub const RT5640_JD_SPL_TRG_SFT: c_int = 8;

pub const RT5640_JD_SPR_SFT: c_int = 7;

pub const RT5640_JD_SPR_TRG_SFT: c_int = 6;

pub const RT5640_JD_MO_SFT: c_int = 5;

pub const RT5640_JD_MO_TRG_SFT: c_int = 4;

pub const RT5640_JD_LO_SFT: c_int = 3;

pub const RT5640_JD_LO_TRG_SFT: c_int = 2;

pub const RT5640_JD1_IN4P_SFT: c_int = 1;

pub const RT5640_JD2_IN4N_SFT: c_int = 0;

// Jack detect for ANC (0xbc)

pub const RT5640_ANC_DET_SFT: c_int = 4;

pub const RT5640_AD_TRG_SFT: c_int = 3;

pub const RT5640_ANCM_DET_SFT: c_int = 4;

pub const RT5640_AMD_TRG_SFT: c_int = 3;

// IRQ Control 1 (0xbd)

pub const RT5640_IRQ_JD_SFT: c_int = 15;

pub const RT5640_IRQ_OT_SFT: c_int = 14;

pub const RT5640_JD_STKY_SFT: c_int = 13;

pub const RT5640_OT_STKY_SFT: c_int = 12;

pub const RT5640_JD_P_SFT: c_int = 11;

pub const RT5640_OT_P_SFT: c_int = 10;

// IRQ Control 2 (0xbe)

pub const RT5640_IRQ_MB1_OC_SFT: c_int = 15;

pub const RT5640_IRQ_MB2_OC_SFT: c_int = 14;

pub const RT5640_MB1_OC_STKY_SFT: c_int = 11;

pub const RT5640_MB2_OC_STKY_SFT: c_int = 10;

pub const RT5640_MB1_OC_P_SFT: c_int = 7;

pub const RT5640_MB2_OC_P_SFT: c_int = 6;

pub const RT5640_MB1_OC_STATUS_SFT: c_int = 3;

pub const RT5640_MB2_OC_STATUS_SFT: c_int = 2;
// GPIO and Internal Status (0xbf)

// GPIO Control 1 (0xc0)

pub const RT5640_GP1_PIN_SFT: c_int = 15;

pub const RT5640_GP2_PIN_SFT: c_int = 14;

pub const RT5640_GP3_PIN_SFT: c_int = 12;

pub const RT5640_GP4_PIN_SFT: c_int = 11;

pub const RT5640_DP_SIG_SFT: c_int = 10;

pub const RT5640_GPIO_M_SFT: c_int = 9;

// GPIO Control 3 (0xc2)

pub const RT5640_GP4_PF_SFT: c_int = 11;

pub const RT5640_GP4_OUT_SFT: c_int = 10;

pub const RT5640_GP4_P_SFT: c_int = 9;

pub const RT5640_GP3_PF_SFT: c_int = 8;

pub const RT5640_GP3_OUT_SFT: c_int = 7;

pub const RT5640_GP3_P_SFT: c_int = 6;

pub const RT5640_GP2_PF_SFT: c_int = 5;

pub const RT5640_GP2_OUT_SFT: c_int = 4;

pub const RT5640_GP2_P_SFT: c_int = 3;

pub const RT5640_GP1_PF_SFT: c_int = 2;

pub const RT5640_GP1_OUT_SFT: c_int = 1;

pub const RT5640_GP1_P_SFT: c_int = 0;

// FM34-500 Register Control 1 (0xc4)
pub const RT5640_DSP_ADD_SFT: c_int = 0;
// FM34-500 Register Control 2 (0xc5)
pub const RT5640_DSP_DAT_SFT: c_int = 0;
// FM34-500 Register Control 3 (0xc6)

pub const RT5640_DSP_BUSY_BIT: c_int = 15;

pub const RT5640_DSP_DS_SFT: c_int = 14;

pub const RT5640_DSP_CLK_SFT: c_int = 12;

pub const RT5640_DSP_PD_PIN_SFT: c_int = 11;

pub const RT5640_DSP_RST_PIN_SFT: c_int = 10;

pub const RT5640_DSP_R_EN_BIT: c_int = 9;

pub const RT5640_DSP_W_EN_BIT: c_int = 8;

pub const RT5640_DSP_CMD_SFT: c_int = 0;

// Programmable Register Array Control 1 (0xc8)

pub const RT5640_REG_SEQ_SFT: c_int = 12;

pub const RT5640_SEQ1_ST_SFT: c_int = 11;

pub const RT5640_SEQ2_ST_SFT: c_int = 10;

pub const RT5640_REG_LV_SFT: c_int = 9;

pub const RT5640_SEQ_2_PT_BIT: c_int = 8;

pub const RT5640_REG_IDX_SFT: c_int = 0;
// Programmable Register Array Control 2 (0xc9)

pub const RT5640_REG_DAT_SFT: c_int = 0;
// Programmable Register Array Control 3 (0xca)

pub const RT5640_SEQ_DLY_SFT: c_int = 8;

pub const RT5640_PROG_SFT: c_int = 7;

pub const RT5640_SEQ1_PT_RUN_BIT: c_int = 6;

pub const RT5640_SEQ2_PT_RUN_BIT: c_int = 5;
// Programmable Register Array Control 4 (0xcb)

pub const RT5640_SEQ1_START_SFT: c_int = 8;

pub const RT5640_SEQ1_END_SFT: c_int = 0;
// Programmable Register Array Control 5 (0xcc)

pub const RT5640_SEQ2_START_SFT: c_int = 8;

pub const RT5640_SEQ2_END_SFT: c_int = 0;
// Scramble Function (0xcd)

pub const RT5640_SCB_KEY_SFT: c_int = 0;
// Scramble Control (0xce)

pub const RT5640_SCB_SWAP_SFT: c_int = 15;

pub const RT5640_SCB_SFT: c_int = 14;

// Baseback Control (0xcf)

pub const RT5640_BB_SFT: c_int = 15;

pub const RT5640_BB_CT_SFT: c_int = 12;

pub const RT5640_M_BB_L_SFT: c_int = 9;

pub const RT5640_M_BB_R_SFT: c_int = 8;

pub const RT5640_M_BB_HPF_L_SFT: c_int = 7;

pub const RT5640_M_BB_HPF_R_SFT: c_int = 6;

pub const RT5640_G_BB_BST_SFT: c_int = 0;
// MP3 Plus Control 1 (0xd0)

pub const RT5640_M_MP3_L_SFT: c_int = 15;

pub const RT5640_M_MP3_R_SFT: c_int = 14;

pub const RT5640_M_MP3_SFT: c_int = 13;

pub const RT5640_EG_MP3_SFT: c_int = 8;

pub const RT5640_MP3_HLP_SFT: c_int = 7;

pub const RT5640_M_MP3_ORG_L_SFT: c_int = 6;

pub const RT5640_M_MP3_ORG_R_SFT: c_int = 5;
// MP3 Plus Control 2 (0xd1)

pub const RT5640_MP3_WT_SFT: c_int = 13;

pub const RT5640_OG_MP3_SFT: c_int = 8;

pub const RT5640_HG_MP3_SFT: c_int = 0;
// 3D HP Control 1 (0xd2)

pub const RT5640_3D_CF_SFT: c_int = 15;

pub const RT5640_3D_HP_SFT: c_int = 14;

pub const RT5640_3D_BT_SFT: c_int = 13;

pub const RT5640_3D_1F_MIX_SFT: c_int = 11;

pub const RT5640_3D_HP_M_SFT: c_int = 10;

pub const RT5640_M_3D_HRTF_SFT: c_int = 9;

pub const RT5640_M_3D_D2H_SFT: c_int = 8;

pub const RT5640_M_3D_D2R_SFT: c_int = 7;

pub const RT5640_M_3D_REVB_SFT: c_int = 6;
// Adjustable high pass filter control 1 (0xd3)

pub const RT5640_2ND_HPF_SFT: c_int = 15;

pub const RT5640_HPF_CF_L_SFT: c_int = 12;

pub const RT5640_1ST_HPF_SFT: c_int = 11;

pub const RT5640_HPF_CF_R_SFT: c_int = 8;

pub const RT5640_ZD_T_SFT: c_int = 6;

pub const RT5640_ZD_F_SFT: c_int = 4;

// HP calibration control and Amp detection (0xd6)

pub const RT5640_SI_DAC_SFT: c_int = 11;

pub const RT5640_DC_CAL_M_SFT: c_int = 10;

pub const RT5640_DC_CAL_SFT: c_int = 9;

pub const RT5640_HPD_RCV_SFT: c_int = 6;

pub const RT5640_HPD_PS_SFT: c_int = 5;

pub const RT5640_CAL_M_SFT: c_int = 4;

pub const RT5640_CAL_SFT: c_int = 3;

pub const RT5640_CAL_TEST_SFT: c_int = 2;

pub const RT5640_CAL_P_SFT: c_int = 0;

// Soft volume and zero cross control 1 (0xd9)

pub const RT5640_SV_SFT: c_int = 15;

pub const RT5640_SPO_SV_SFT: c_int = 14;

pub const RT5640_OUT_SV_SFT: c_int = 13;

pub const RT5640_HP_SV_SFT: c_int = 12;

pub const RT5640_ZCD_DIG_SFT: c_int = 11;

pub const RT5640_ZCD_SFT: c_int = 10;

pub const RT5640_M_ZCD_SFT: c_int = 4;

pub const RT5640_SV_DLY_SFT: c_int = 0;
// Soft volume and zero cross control 2 (0xda)

pub const RT5640_ZCD_HP_SFT: c_int = 15;

// General Control 1 (0xfa)

pub const RT5640_EN_LOUT_DF_SFT: c_int = 14;

pub const RT5640_M_MONO_ADC_L_SFT: c_int = 13;

pub const RT5640_M_MONO_ADC_R_SFT: c_int = 12;

// General Control 2 (0xfb)

pub const RT5640_IRQ_JD2_SFT: c_int = 12;

pub const RT5640_JD2_P_SFT: c_int = 10;

pub const RT5640_JD2_SFT: c_int = 8;

// Codec Private Register definition
// MIC Over current threshold scale factor (0x15)

pub const RT5640_MIC_OVCD_SF_SFT: c_int = 8;

// 3D Speaker Control (0x63)

pub const RT5640_3D_SPK_SFT: c_int = 15;

pub const RT5640_3D_SPK_M_SFT: c_int = 13;

pub const RT5640_3D_SPK_CG_SFT: c_int = 8;

pub const RT5640_3D_SPK_SG_SFT: c_int = 0;
// Wind Noise Detection Control 1 (0x6c)

pub const RT5640_WND_SFT: c_int = 15;

// Wind Noise Detection Control 2 (0x6d)

pub const RT5640_WND_FC_NW_SFT: c_int = 10;

pub const RT5640_WND_FC_WK_SFT: c_int = 4;
// Wind Noise Detection Control 3 (0x6e)

pub const RT5640_HPF_FC_SFT: c_int = 6;

pub const RT5640_WND_FC_ST_SFT: c_int = 0;
// Wind Noise Detection Control 4 (0x6f)

pub const RT5640_WND_TH_LO_SFT: c_int = 0;
// Wind Noise Detection Control 5 (0x70)

pub const RT5640_WND_TH_HI_SFT: c_int = 0;
// Wind Noise Detection Control 8 (0x73)

pub const RT5640_WND_WIND_SFT: c_int = 13;

pub const RT5640_WND_STRONG_SFT: c_int = 12;
// Dipole Speaker Interface (0x75)

pub const RT5640_DP_ATT_SFT: c_int = 14;

pub const RT5640_DP_SPK_SFT: c_int = 10;

// EQ Pre Volume Control (0xb3)

pub const RT5640_EQ_PRE_VOL_SFT: c_int = 0;
// EQ Post Volume Control (0xb4)

pub const RT5640_EQ_PST_VOL_SFT: c_int = 0;

// System Clock Source
pub const RT5640_SCLK_S_MCLK: c_int = 0;
pub const RT5640_SCLK_S_PLL1: c_int = 1;
pub const RT5640_SCLK_S_PLL1_TK: c_int = 2;
pub const RT5640_SCLK_S_RCCLK: c_int = 3;
// PLL1 Source
pub const RT5640_PLL1_S_MCLK: c_int = 0;
pub const RT5640_PLL1_S_BCLK1: c_int = 1;
pub const RT5640_PLL1_S_BCLK2: c_int = 2;
pub const RT5640_PLL1_S_BCLK3: c_int = 3;
// filter mask
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5640_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub /: *mut *mut *mut gpio_desc ldo1_en; / GPIO for LDO1_EN,
    pub irq: c_int,
    pub jd_gpio_irq: c_int,
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: [c_int; RT5640_AIFS],
    pub bclk: [c_int; RT5640_AIFS],
    pub master: [c_int; RT5640_AIFS],
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
    pub hp_mute: bool,
    pub asrc_en: bool,
    pub irq_requested: bool,
    pub jd_gpio_irq_requested: bool,
// Jack and button detect data
    pub ovcd_irq_enabled: bool,
    pub pressed: bool,
    pub press_reported: bool,
    pub press_count: c_int,
    pub release_count: c_int,
    pub poll_count: c_int,
    pub bp_work: delayed_work,
    pub jack_work: delayed_work,
    pub jack: *mut snd_soc_jack,
    pub jd_gpio: *mut gpio_desc,
    pub jd_src: c_uint,
    pub jd_inverted: bool,
    pub ovcd_th: c_uint,
    pub ovcd_sf: c_uint,
    pub use_platform_clock: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5640_set_jack_data {
    pub codec_irq_override: c_int,
    pub jd_gpio: *mut gpio_desc,
    pub use_platform_clock: bool,
}

extern "C" {
    pub fn rt5640_set_ovcd_params(component: *mut snd_soc_component);
}
extern "C" {
    pub fn rt5640_enable_micbias1_for_ovcd(component: *mut snd_soc_component);
}
extern "C" {
    pub fn rt5640_disable_micbias1_for_ovcd(component: *mut snd_soc_component);
}
extern "C" {
    pub fn rt5640_detect_headset(component: *mut snd_soc_component, hp_det_gpio: *mut gpio_desc) -> c_int;
}
