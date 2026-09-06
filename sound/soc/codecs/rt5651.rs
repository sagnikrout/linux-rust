//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5651.h
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
// rt5651.h  --  RT5651 ALSA SoC audio driver
//
// Copyright 2011 Realtek Microelectronics
// Author: Johnny Hsu <johnnyhsu@realtek.com>
//

// Info
pub const RT5651_RESET: c_uint = 0x00;
pub const RT5651_VERSION_ID: c_uint = 0xfd;
pub const RT5651_VENDOR_ID: c_uint = 0xfe;
pub const RT5651_DEVICE_ID: c_uint = 0xff;
// I/O - Output
pub const RT5651_HP_VOL: c_uint = 0x02;
pub const RT5651_LOUT_CTRL1: c_uint = 0x03;
pub const RT5651_LOUT_CTRL2: c_uint = 0x05;
// I/O - Input
pub const RT5651_IN1_IN2: c_uint = 0x0d;
pub const RT5651_IN3: c_uint = 0x0e;
pub const RT5651_INL1_INR1_VOL: c_uint = 0x0f;
pub const RT5651_INL2_INR2_VOL: c_uint = 0x10;
// I/O - ADC/DAC/DMIC
pub const RT5651_DAC1_DIG_VOL: c_uint = 0x19;
pub const RT5651_DAC2_DIG_VOL: c_uint = 0x1a;
pub const RT5651_DAC2_CTRL: c_uint = 0x1b;
pub const RT5651_ADC_DIG_VOL: c_uint = 0x1c;
pub const RT5651_ADC_DATA: c_uint = 0x1d;
pub const RT5651_ADC_BST_VOL: c_uint = 0x1e;
// Mixer - D-D
pub const RT5651_STO1_ADC_MIXER: c_uint = 0x27;
pub const RT5651_STO2_ADC_MIXER: c_uint = 0x28;
pub const RT5651_AD_DA_MIXER: c_uint = 0x29;
pub const RT5651_STO_DAC_MIXER: c_uint = 0x2a;
pub const RT5651_DD_MIXER: c_uint = 0x2b;
pub const RT5651_DIG_INF_DATA: c_uint = 0x2f;
// PDM
pub const RT5651_PDM_CTL: c_uint = 0x30;
pub const RT5651_PDM_I2C_CTL1: c_uint = 0x31;
pub const RT5651_PDM_I2C_CTL2: c_uint = 0x32;
pub const RT5651_PDM_I2C_DATA_W: c_uint = 0x33;
pub const RT5651_PDM_I2C_DATA_R: c_uint = 0x34;
// Mixer - ADC
pub const RT5651_REC_L1_MIXER: c_uint = 0x3b;
pub const RT5651_REC_L2_MIXER: c_uint = 0x3c;
pub const RT5651_REC_R1_MIXER: c_uint = 0x3d;
pub const RT5651_REC_R2_MIXER: c_uint = 0x3e;
// Mixer - DAC
pub const RT5651_HPO_MIXER: c_uint = 0x45;
pub const RT5651_OUT_L1_MIXER: c_uint = 0x4d;
pub const RT5651_OUT_L2_MIXER: c_uint = 0x4e;
pub const RT5651_OUT_L3_MIXER: c_uint = 0x4f;
pub const RT5651_OUT_R1_MIXER: c_uint = 0x50;
pub const RT5651_OUT_R2_MIXER: c_uint = 0x51;
pub const RT5651_OUT_R3_MIXER: c_uint = 0x52;
pub const RT5651_LOUT_MIXER: c_uint = 0x53;
// Power
pub const RT5651_PWR_DIG1: c_uint = 0x61;
pub const RT5651_PWR_DIG2: c_uint = 0x62;
pub const RT5651_PWR_ANLG1: c_uint = 0x63;
pub const RT5651_PWR_ANLG2: c_uint = 0x64;
pub const RT5651_PWR_MIXER: c_uint = 0x65;
pub const RT5651_PWR_VOL: c_uint = 0x66;
// Private Register Control
pub const RT5651_PRIV_INDEX: c_uint = 0x6a;
pub const RT5651_PRIV_DATA: c_uint = 0x6c;
// Format - ADC/DAC
pub const RT5651_I2S1_SDP: c_uint = 0x70;
pub const RT5651_I2S2_SDP: c_uint = 0x71;
pub const RT5651_ADDA_CLK1: c_uint = 0x73;
pub const RT5651_ADDA_CLK2: c_uint = 0x74;
pub const RT5651_DMIC: c_uint = 0x75;
// TDM Control
pub const RT5651_TDM_CTL_1: c_uint = 0x77;
pub const RT5651_TDM_CTL_2: c_uint = 0x78;
pub const RT5651_TDM_CTL_3: c_uint = 0x79;
// Function - Analog
pub const RT5651_GLB_CLK: c_uint = 0x80;
pub const RT5651_PLL_CTRL1: c_uint = 0x81;
pub const RT5651_PLL_CTRL2: c_uint = 0x82;
pub const RT5651_PLL_MODE_1: c_uint = 0x83;
pub const RT5651_PLL_MODE_2: c_uint = 0x84;
pub const RT5651_PLL_MODE_3: c_uint = 0x85;
pub const RT5651_PLL_MODE_4: c_uint = 0x86;
pub const RT5651_PLL_MODE_5: c_uint = 0x87;
pub const RT5651_PLL_MODE_6: c_uint = 0x89;
pub const RT5651_PLL_MODE_7: c_uint = 0x8a;
pub const RT5651_DEPOP_M1: c_uint = 0x8e;
pub const RT5651_DEPOP_M2: c_uint = 0x8f;
pub const RT5651_DEPOP_M3: c_uint = 0x90;
pub const RT5651_CHARGE_PUMP: c_uint = 0x91;
pub const RT5651_MICBIAS: c_uint = 0x93;
pub const RT5651_A_JD_CTL1: c_uint = 0x94;
// Function - Digital
pub const RT5651_EQ_CTRL1: c_uint = 0xb0;
pub const RT5651_EQ_CTRL2: c_uint = 0xb1;
pub const RT5651_ALC_1: c_uint = 0xb4;
pub const RT5651_ALC_2: c_uint = 0xb5;
pub const RT5651_ALC_3: c_uint = 0xb6;
pub const RT5651_JD_CTRL1: c_uint = 0xbb;
pub const RT5651_JD_CTRL2: c_uint = 0xbc;
pub const RT5651_IRQ_CTRL1: c_uint = 0xbd;
pub const RT5651_IRQ_CTRL2: c_uint = 0xbe;
pub const RT5651_INT_IRQ_ST: c_uint = 0xbf;
pub const RT5651_GPIO_CTRL1: c_uint = 0xc0;
pub const RT5651_GPIO_CTRL2: c_uint = 0xc1;
pub const RT5651_GPIO_CTRL3: c_uint = 0xc2;
pub const RT5651_PGM_REG_ARR1: c_uint = 0xc8;
pub const RT5651_PGM_REG_ARR2: c_uint = 0xc9;
pub const RT5651_PGM_REG_ARR3: c_uint = 0xca;
pub const RT5651_PGM_REG_ARR4: c_uint = 0xcb;
pub const RT5651_PGM_REG_ARR5: c_uint = 0xcc;
pub const RT5651_SCB_FUNC: c_uint = 0xcd;
pub const RT5651_SCB_CTRL: c_uint = 0xce;
pub const RT5651_BASE_BACK: c_uint = 0xcf;
pub const RT5651_MP3_PLUS1: c_uint = 0xd0;
pub const RT5651_MP3_PLUS2: c_uint = 0xd1;
pub const RT5651_ADJ_HPF_CTRL1: c_uint = 0xd3;
pub const RT5651_ADJ_HPF_CTRL2: c_uint = 0xd4;
pub const RT5651_HP_CALIB_AMP_DET: c_uint = 0xd6;
pub const RT5651_HP_CALIB2: c_uint = 0xd7;
pub const RT5651_SV_ZCD1: c_uint = 0xd9;
pub const RT5651_SV_ZCD2: c_uint = 0xda;
pub const RT5651_D_MISC: c_uint = 0xfa;
// Dummy Register
pub const RT5651_DUMMY2: c_uint = 0xfb;
pub const RT5651_DUMMY3: c_uint = 0xfc;
// Index of Codec Private Register definition
pub const RT5651_BIAS_CUR1: c_uint = 0x12;
pub const RT5651_BIAS_CUR3: c_uint = 0x14;
pub const RT5651_BIAS_CUR4: c_uint = 0x15;
pub const RT5651_CLSD_INT_REG1: c_uint = 0x1c;
pub const RT5651_CHPUMP_INT_REG1: c_uint = 0x24;
pub const RT5651_MAMP_INT_REG2: c_uint = 0x37;
pub const RT5651_CHOP_DAC_ADC: c_uint = 0x3d;
pub const RT5651_3D_SPK: c_uint = 0x63;
pub const RT5651_WND_1: c_uint = 0x6c;
pub const RT5651_WND_2: c_uint = 0x6d;
pub const RT5651_WND_3: c_uint = 0x6e;
pub const RT5651_WND_4: c_uint = 0x6f;
pub const RT5651_WND_5: c_uint = 0x70;
pub const RT5651_WND_8: c_uint = 0x73;
pub const RT5651_DIP_SPK_INF: c_uint = 0x75;
pub const RT5651_HP_DCC_INT1: c_uint = 0x77;
pub const RT5651_EQ_BW_LOP: c_uint = 0xa0;
pub const RT5651_EQ_GN_LOP: c_uint = 0xa1;
pub const RT5651_EQ_FC_BP1: c_uint = 0xa2;
pub const RT5651_EQ_BW_BP1: c_uint = 0xa3;
pub const RT5651_EQ_GN_BP1: c_uint = 0xa4;
pub const RT5651_EQ_FC_BP2: c_uint = 0xa5;
pub const RT5651_EQ_BW_BP2: c_uint = 0xa6;
pub const RT5651_EQ_GN_BP2: c_uint = 0xa7;
pub const RT5651_EQ_FC_BP3: c_uint = 0xa8;
pub const RT5651_EQ_BW_BP3: c_uint = 0xa9;
pub const RT5651_EQ_GN_BP3: c_uint = 0xaa;
pub const RT5651_EQ_FC_BP4: c_uint = 0xab;
pub const RT5651_EQ_BW_BP4: c_uint = 0xac;
pub const RT5651_EQ_GN_BP4: c_uint = 0xad;
pub const RT5651_EQ_FC_HIP1: c_uint = 0xae;
pub const RT5651_EQ_GN_HIP1: c_uint = 0xaf;
pub const RT5651_EQ_FC_HIP2: c_uint = 0xb0;
pub const RT5651_EQ_BW_HIP2: c_uint = 0xb1;
pub const RT5651_EQ_GN_HIP2: c_uint = 0xb2;
pub const RT5651_EQ_PRE_VOL: c_uint = 0xb3;
pub const RT5651_EQ_PST_VOL: c_uint = 0xb4;
// global definition

pub const RT5651_L_MUTE_SFT: c_int = 15;

pub const RT5651_VOL_L_SFT: c_int = 14;

pub const RT5651_R_MUTE_SFT: c_int = 7;

pub const RT5651_VOL_R_SFT: c_int = 6;

pub const RT5651_L_VOL_SFT: c_int = 8;

pub const RT5651_R_VOL_SFT: c_int = 0;
// LOUT Control 2(0x05)

// IN1 and IN2 Control (0x0d)
// IN3 and IN4 Control (0x0e)

pub const RT5651_BST_SFT1: c_int = 12;

pub const RT5651_BST_SFT2: c_int = 8;

pub const RT5651_IN_SFT1: c_int = 7;

pub const RT5651_IN_SFT2: c_int = 6;
// INL1 and INR1 Volume Control (0x0f)
// INL2 and INR2 Volume Control (0x10)

pub const RT5651_INL_SEL_SFT: c_int = 15;

pub const RT5651_INL_VOL_SFT: c_int = 8;

pub const RT5651_INR_SEL_SFT: c_int = 7;

pub const RT5651_INR_VOL_SFT: c_int = 0;
// DAC1 Digital Volume (0x19)

pub const RT5651_DAC_L1_VOL_SFT: c_int = 8;

pub const RT5651_DAC_R1_VOL_SFT: c_int = 0;
// DAC2 Digital Volume (0x1a)

pub const RT5651_DAC_L2_VOL_SFT: c_int = 8;

pub const RT5651_DAC_R2_VOL_SFT: c_int = 0;
// DAC2 Control (0x1b)

pub const RT5651_M_DAC_L2_VOL_SFT: c_int = 13;

pub const RT5651_M_DAC_R2_VOL_SFT: c_int = 12;

pub const RT5651_SEL_DAC_L2_SFT: c_int = 11;

pub const RT5651_SEL_DAC_R2_SFT: c_int = 10;
// ADC Digital Volume Control (0x1c)

pub const RT5651_ADC_L_VOL_SFT: c_int = 8;

pub const RT5651_ADC_R_VOL_SFT: c_int = 0;
// Mono ADC Digital Volume Control (0x1d)

pub const RT5651_M_MONO_ADC_L_SFT: c_int = 15;

pub const RT5651_MONO_ADC_L_VOL_SFT: c_int = 8;

pub const RT5651_M_MONO_ADC_R_SFT: c_int = 7;

pub const RT5651_MONO_ADC_R_VOL_SFT: c_int = 0;
// ADC Boost Volume Control (0x1e)

pub const RT5651_ADC_L_BST_SFT: c_int = 14;

pub const RT5651_ADC_R_BST_SFT: c_int = 12;

pub const RT5651_ADC_COMP_SFT: c_int = 10;
// Stereo ADC1 Mixer Control (0x27)

pub const RT5651_M_STO1_ADC_L1_SFT: c_int = 14;

pub const RT5651_M_STO1_ADC_L2_SFT: c_int = 13;

pub const RT5651_STO1_ADC_1_SRC_SFT: c_int = 12;

pub const RT5651_STO1_ADC_2_SRC_SFT: c_int = 11;

pub const RT5651_M_STO1_ADC_R1_SFT: c_int = 6;

pub const RT5651_M_STO1_ADC_R2_SFT: c_int = 5;
// Stereo ADC2 Mixer Control (0x28)

pub const RT5651_M_STO2_ADC_L1_SFT: c_int = 14;

pub const RT5651_M_STO2_ADC_L2_SFT: c_int = 13;

pub const RT5651_STO2_ADC_L1_SRC_SFT: c_int = 12;

pub const RT5651_STO2_ADC_L2_SRC_SFT: c_int = 11;

pub const RT5651_M_STO2_ADC_R1_SFT: c_int = 6;

pub const RT5651_M_STO2_ADC_R2_SFT: c_int = 5;

pub const RT5651_STO2_ADC_R1_SRC_SFT: c_int = 4;

pub const RT5651_STO2_ADC_R2_SRC_SFT: c_int = 3;

// ADC Mixer to DAC Mixer Control (0x29)

pub const RT5651_M_ADCMIX_L_SFT: c_int = 15;

pub const RT5651_M_IF1_DAC_L_SFT: c_int = 14;

pub const RT5651_M_ADCMIX_R_SFT: c_int = 7;

pub const RT5651_M_IF1_DAC_R_SFT: c_int = 6;
// Stereo DAC Mixer Control (0x2a)

pub const RT5651_M_DAC_L1_MIXL_SFT: c_int = 14;

pub const RT5651_DAC_L1_STO_L_VOL_SFT: c_int = 13;

pub const RT5651_M_DAC_L2_MIXL_SFT: c_int = 12;

pub const RT5651_DAC_L2_STO_L_VOL_SFT: c_int = 11;

pub const RT5651_M_DAC_R1_MIXL_SFT: c_int = 9;

pub const RT5651_DAC_R1_STO_L_VOL_SFT: c_int = 8;

pub const RT5651_M_DAC_R1_MIXR_SFT: c_int = 6;

pub const RT5651_DAC_R1_STO_R_VOL_SFT: c_int = 5;

pub const RT5651_M_DAC_R2_MIXR_SFT: c_int = 4;

pub const RT5651_DAC_R2_STO_R_VOL_SFT: c_int = 3;

pub const RT5651_M_DAC_L1_MIXR_SFT: c_int = 1;

pub const RT5651_DAC_L1_STO_R_VOL_SFT: c_int = 0;
// DD Mixer Control (0x2b)

pub const RT5651_M_STO_DD_L1_SFT: c_int = 14;

pub const RT5651_DAC_DD_L1_VOL_SFT: c_int = 13;

pub const RT5651_M_STO_DD_L2_SFT: c_int = 12;

pub const RT5651_STO_DD_L2_VOL_SFT: c_int = 11;

pub const RT5651_M_STO_DD_R2_L_SFT: c_int = 10;

pub const RT5651_STO_DD_R2_L_VOL_SFT: c_int = 9;

pub const RT5651_M_STO_DD_R1_SFT: c_int = 6;

pub const RT5651_STO_DD_R1_VOL_SFT: c_int = 5;

pub const RT5651_M_STO_DD_R2_SFT: c_int = 4;

pub const RT5651_STO_DD_R2_VOL_SFT: c_int = 3;

pub const RT5651_M_STO_DD_L2_R_SFT: c_int = 2;

pub const RT5651_STO_DD_L2_R_VOL_SFT: c_int = 1;
// Digital Mixer Control (0x2c)

pub const RT5651_M_STO_L_DAC_L_SFT: c_int = 15;

pub const RT5651_STO_L_DAC_L_VOL_SFT: c_int = 14;

pub const RT5651_M_DAC_L2_DAC_L_SFT: c_int = 13;

pub const RT5651_DAC_L2_DAC_L_VOL_SFT: c_int = 12;

pub const RT5651_M_STO_R_DAC_R_SFT: c_int = 11;

pub const RT5651_STO_R_DAC_R_VOL_SFT: c_int = 10;

pub const RT5651_M_DAC_R2_DAC_R_SFT: c_int = 9;

pub const RT5651_DAC_R2_DAC_R_VOL_SFT: c_int = 8;
// DSP Path Control 1 (0x2d)

pub const RT5651_RXDP_SRC_SFT: c_int = 15;

pub const RT5651_TXDP_SRC_SFT: c_int = 14;

// DSP Path Control 2 (0x2e)

pub const RT5651_DAC_L2_SEL_SFT: c_int = 14;

pub const RT5651_DAC_R2_SEL_SFT: c_int = 12;

pub const RT5651_IF2_ADC_L_SEL_SFT: c_int = 11;

pub const RT5651_IF2_ADC_R_SEL_SFT: c_int = 10;

pub const RT5651_RXDC_SEL_SFT: c_int = 8;

pub const RT5651_RXDP_SEL_SFT: c_int = 6;

pub const RT5651_TXDC_SEL_SFT: c_int = 4;

pub const RT5651_TXDP_SEL_SFT: c_int = 2;

// Digital Interface Data Control (0x2f)

pub const RT5651_IF2_DAC_SEL_SFT: c_int = 10;

pub const RT5651_IF2_ADC_SEL_SFT: c_int = 8;

pub const RT5651_IF2_ADC_SRC_SFT: c_int = 7;

// PDM Output Control (0x30)

pub const RT5651_PDM_L_SEL_SFT: c_int = 15;

pub const RT5651_M_PDM_L_SFT: c_int = 14;

pub const RT5651_PDM_R_SEL_SFT: c_int = 13;

pub const RT5651_M_PDM_R_SFT: c_int = 12;

pub const RT5651_PDM_BUSY_SFT: c_int = 6;

pub const RT5651_PDM_VOL_SFT: c_int = 4;

pub const RT5651_PDM_DIV_SFT: c_int = 0;
pub const RT5651_PDM_DIV_1: c_int = 0;
pub const RT5651_PDM_DIV_2: c_int = 1;
pub const RT5651_PDM_DIV_3: c_int = 2;
pub const RT5651_PDM_DIV_4: c_int = 3;
// PDM I2C/Data Control 1 (0x31)

// PDM I2C/Data Control 2 (0x32)

// REC Left Mixer Control 1 (0x3b)

pub const RT5651_G_IN_L2_RM_L_SFT: c_int = 13;

pub const RT5651_G_IN_L1_RM_L_SFT: c_int = 10;

pub const RT5651_G_BST3_RM_L_SFT: c_int = 4;

pub const RT5651_G_BST2_RM_L_SFT: c_int = 1;
// REC Left Mixer Control 2 (0x3c)

pub const RT5651_G_BST1_RM_L_SFT: c_int = 13;

pub const RT5651_G_OM_L_RM_L_SFT: c_int = 10;

pub const RT5651_M_IN2_L_RM_L_SFT: c_int = 6;

pub const RT5651_M_IN1_L_RM_L_SFT: c_int = 5;

pub const RT5651_M_BST3_RM_L_SFT: c_int = 3;

pub const RT5651_M_BST2_RM_L_SFT: c_int = 2;

pub const RT5651_M_BST1_RM_L_SFT: c_int = 1;

pub const RT5651_M_OM_L_RM_L_SFT: c_int = 0;
// REC Right Mixer Control 1 (0x3d)

pub const RT5651_G_IN2_R_RM_R_SFT: c_int = 13;

pub const RT5651_G_IN1_R_RM_R_SFT: c_int = 10;

pub const RT5651_G_BST3_RM_R_SFT: c_int = 4;

pub const RT5651_G_BST2_RM_R_SFT: c_int = 1;
// REC Right Mixer Control 2 (0x3e)

pub const RT5651_G_BST1_RM_R_SFT: c_int = 13;

pub const RT5651_G_OM_R_RM_R_SFT: c_int = 10;

pub const RT5651_M_IN2_R_RM_R_SFT: c_int = 6;

pub const RT5651_M_IN1_R_RM_R_SFT: c_int = 5;

pub const RT5651_M_BST3_RM_R_SFT: c_int = 3;

pub const RT5651_M_BST2_RM_R_SFT: c_int = 2;

pub const RT5651_M_BST1_RM_R_SFT: c_int = 1;

pub const RT5651_M_OM_R_RM_R_SFT: c_int = 0;
// HPMIX Control (0x45)

pub const RT5651_M_DAC1_HM_SFT: c_int = 14;

pub const RT5651_M_HPVOL_HM_SFT: c_int = 13;

pub const RT5651_G_HPOMIX_SFT: c_int = 12;
// SPK Left Mixer Control (0x46)

pub const RT5651_G_RM_L_SM_L_SFT: c_int = 14;

pub const RT5651_G_IN_L_SM_L_SFT: c_int = 12;

pub const RT5651_G_DAC_L1_SM_L_SFT: c_int = 10;

pub const RT5651_G_DAC_L2_SM_L_SFT: c_int = 8;

pub const RT5651_G_OM_L_SM_L_SFT: c_int = 6;

pub const RT5651_M_RM_L_SM_L_SFT: c_int = 5;

pub const RT5651_M_IN_L_SM_L_SFT: c_int = 4;

pub const RT5651_M_DAC_L1_SM_L_SFT: c_int = 3;

pub const RT5651_M_DAC_L2_SM_L_SFT: c_int = 2;

pub const RT5651_M_OM_L_SM_L_SFT: c_int = 1;
// SPK Right Mixer Control (0x47)

pub const RT5651_G_RM_R_SM_R_SFT: c_int = 14;

pub const RT5651_G_IN_R_SM_R_SFT: c_int = 12;

pub const RT5651_G_DAC_R1_SM_R_SFT: c_int = 10;

pub const RT5651_G_DAC_R2_SM_R_SFT: c_int = 8;

pub const RT5651_G_OM_R_SM_R_SFT: c_int = 6;

pub const RT5651_M_RM_R_SM_R_SFT: c_int = 5;

pub const RT5651_M_IN_R_SM_R_SFT: c_int = 4;

pub const RT5651_M_DAC_R1_SM_R_SFT: c_int = 3;

pub const RT5651_M_DAC_R2_SM_R_SFT: c_int = 2;

pub const RT5651_M_OM_R_SM_R_SFT: c_int = 1;
// SPOLMIX Control (0x48)

pub const RT5651_M_DAC_R1_SPM_L_SFT: c_int = 15;

pub const RT5651_M_DAC_L1_SPM_L_SFT: c_int = 14;

pub const RT5651_M_SV_R_SPM_L_SFT: c_int = 13;

pub const RT5651_M_SV_L_SPM_L_SFT: c_int = 12;

pub const RT5651_M_BST1_SPM_L_SFT: c_int = 11;
// SPORMIX Control (0x49)

pub const RT5651_M_DAC_R1_SPM_R_SFT: c_int = 13;

pub const RT5651_M_SV_R_SPM_R_SFT: c_int = 12;

pub const RT5651_M_BST1_SPM_R_SFT: c_int = 11;
// SPOLMIX / SPORMIX Ratio Control (0x4a)

pub const RT5651_SPO_CLSD_RATIO_SFT: c_int = 0;
// Mono Output Mixer Control (0x4c)

pub const RT5651_M_DAC_R2_MM_SFT: c_int = 15;

pub const RT5651_M_DAC_L2_MM_SFT: c_int = 14;

pub const RT5651_M_OV_R_MM_SFT: c_int = 13;

pub const RT5651_M_OV_L_MM_SFT: c_int = 12;

pub const RT5651_M_BST1_MM_SFT: c_int = 11;

pub const RT5651_G_MONOMIX_SFT: c_int = 10;
// Output Left Mixer Control 1 (0x4d)

pub const RT5651_G_BST2_OM_L_SFT: c_int = 10;

pub const RT5651_G_BST1_OM_L_SFT: c_int = 7;

pub const RT5651_G_IN1_L_OM_L_SFT: c_int = 4;

pub const RT5651_G_RM_L_OM_L_SFT: c_int = 1;
// Output Left Mixer Control 2 (0x4e)

pub const RT5651_G_DAC_L1_OM_L_SFT: c_int = 7;

pub const RT5651_G_IN2_L_OM_L_SFT: c_int = 4;
// Output Left Mixer Control 3 (0x4f)

pub const RT5651_M_IN2_L_OM_L_SFT: c_int = 9;

pub const RT5651_M_BST2_OM_L_SFT: c_int = 6;

pub const RT5651_M_BST1_OM_L_SFT: c_int = 5;

pub const RT5651_M_IN1_L_OM_L_SFT: c_int = 4;

pub const RT5651_M_RM_L_OM_L_SFT: c_int = 3;

pub const RT5651_M_DAC_L1_OM_L_SFT: c_int = 0;
// Output Right Mixer Control 1 (0x50)

pub const RT5651_G_BST2_OM_R_SFT: c_int = 10;

pub const RT5651_G_BST1_OM_R_SFT: c_int = 7;

pub const RT5651_G_IN1_R_OM_R_SFT: c_int = 4;

pub const RT5651_G_RM_R_OM_R_SFT: c_int = 1;
// Output Right Mixer Control 2 (0x51)

pub const RT5651_G_DAC_R1_OM_R_SFT: c_int = 7;

pub const RT5651_G_IN2_R_OM_R_SFT: c_int = 4;
// Output Right Mixer Control 3 (0x52)

pub const RT5651_M_IN2_R_OM_R_SFT: c_int = 9;

pub const RT5651_M_BST2_OM_R_SFT: c_int = 6;

pub const RT5651_M_BST1_OM_R_SFT: c_int = 5;

pub const RT5651_M_IN1_R_OM_R_SFT: c_int = 4;

pub const RT5651_M_RM_R_OM_R_SFT: c_int = 3;

pub const RT5651_M_DAC_R1_OM_R_SFT: c_int = 0;
// LOUT Mixer Control (0x53)

pub const RT5651_M_DAC_L1_LM_SFT: c_int = 15;

pub const RT5651_M_DAC_R1_LM_SFT: c_int = 14;

pub const RT5651_M_OV_L_LM_SFT: c_int = 13;

pub const RT5651_M_OV_R_LM_SFT: c_int = 12;

pub const RT5651_G_LOUTMIX_SFT: c_int = 11;
// Power Management for Digital 1 (0x61)

pub const RT5651_PWR_I2S1_BIT: c_int = 15;

pub const RT5651_PWR_I2S2_BIT: c_int = 14;

pub const RT5651_PWR_DAC_L1_BIT: c_int = 12;

pub const RT5651_PWR_DAC_R1_BIT: c_int = 11;

pub const RT5651_PWR_ADC_L_BIT: c_int = 2;

pub const RT5651_PWR_ADC_R_BIT: c_int = 1;
// Power Management for Digital 2 (0x62)

pub const RT5651_PWR_ADC_STO1_F_BIT: c_int = 15;

pub const RT5651_PWR_ADC_STO2_F_BIT: c_int = 14;

pub const RT5651_PWR_DAC_STO1_F_BIT: c_int = 11;

pub const RT5651_PWR_DAC_STO2_F_BIT: c_int = 10;

pub const RT5651_PWR_PDM_BIT: c_int = 9;
// Power Management for Analog 1 (0x63)

pub const RT5651_PWR_VREF1_BIT: c_int = 15;

pub const RT5651_PWR_FV1_BIT: c_int = 14;

pub const RT5651_PWR_MB_BIT: c_int = 13;

pub const RT5651_PWR_LM_BIT: c_int = 12;

pub const RT5651_PWR_BG_BIT: c_int = 11;

pub const RT5651_PWR_HP_L_BIT: c_int = 7;

pub const RT5651_PWR_HP_R_BIT: c_int = 6;

pub const RT5651_PWR_HA_BIT: c_int = 5;

pub const RT5651_PWR_VREF2_BIT: c_int = 4;

pub const RT5651_PWR_FV2_BIT: c_int = 3;

pub const RT5651_PWR_LDO_BIT: c_int = 2;

pub const RT5651_PWR_LDO_DVO_1_0V: c_int = 0;
pub const RT5651_PWR_LDO_DVO_1_1V: c_int = 1;
pub const RT5651_PWR_LDO_DVO_1_2V: c_int = 2;
pub const RT5651_PWR_LDO_DVO_1_3V: c_int = 3;
// Power Management for Analog 2 (0x64)

pub const RT5651_PWR_BST1_BIT: c_int = 15;

pub const RT5651_PWR_BST2_BIT: c_int = 14;

pub const RT5651_PWR_BST3_BIT: c_int = 13;

pub const RT5651_PWR_MB1_BIT: c_int = 11;

pub const RT5651_PWR_PLL_BIT: c_int = 9;

pub const RT5651_PWR_BST1_OP2_BIT: c_int = 5;

pub const RT5651_PWR_BST2_OP2_BIT: c_int = 4;

pub const RT5651_PWR_BST3_OP2_BIT: c_int = 3;

pub const RT5651_PWM_JD_M_BIT: c_int = 2;

pub const RT5651_PWM_JD2_BIT: c_int = 1;

pub const RT5651_PWM_JD3_BIT: c_int = 0;
// Power Management for Mixer (0x65)

pub const RT5651_PWR_OM_L_BIT: c_int = 15;

pub const RT5651_PWR_OM_R_BIT: c_int = 14;

pub const RT5651_PWR_RM_L_BIT: c_int = 11;

pub const RT5651_PWR_RM_R_BIT: c_int = 10;
// Power Management for Volume (0x66)

pub const RT5651_PWR_OV_L_BIT: c_int = 13;

pub const RT5651_PWR_OV_R_BIT: c_int = 12;

pub const RT5651_PWR_HV_L_BIT: c_int = 11;

pub const RT5651_PWR_HV_R_BIT: c_int = 10;

pub const RT5651_PWR_IN1_L_BIT: c_int = 9;

pub const RT5651_PWR_IN1_R_BIT: c_int = 8;

pub const RT5651_PWR_IN2_L_BIT: c_int = 7;

pub const RT5651_PWR_IN2_R_BIT: c_int = 6;
// I2S1/2/3 Audio Serial Data Port Control (0x70 0x71)

pub const RT5651_I2S_MS_SFT: c_int = 15;

pub const RT5651_I2S_O_CP_SFT: c_int = 10;

pub const RT5651_I2S_I_CP_SFT: c_int = 8;

pub const RT5651_I2S_BP_SFT: c_int = 7;

pub const RT5651_I2S_DL_SFT: c_int = 2;

pub const RT5651_I2S_DF_SFT: c_int = 0;

// ADC/DAC Clock Control 1 (0x73)

pub const RT5651_I2S_PD1_SFT: c_int = 12;

pub const RT5651_I2S_BCLK_MS2_SFT: c_int = 11;

pub const RT5651_I2S_PD2_SFT: c_int = 8;

pub const RT5651_DAC_OSR_SFT: c_int = 2;

pub const RT5651_ADC_OSR_SFT: c_int = 0;

// ADC/DAC Clock Control 2 (0x74)

pub const RT5651_DAHPF_EN_SFT: c_int = 11;

pub const RT5651_ADHPF_EN_SFT: c_int = 10;
// Digital Microphone Control (0x75)

pub const RT5651_DMIC_1_EN_SFT: c_int = 15;

pub const RT5651_DMIC_1L_LH_SFT: c_int = 13;

pub const RT5651_DMIC_1R_LH_SFT: c_int = 12;

pub const RT5651_DMIC_1_DP_SFT: c_int = 10;

pub const RT5651_DMIC_CLK_SFT: c_int = 5;
// TDM Control 1 (0x77)

pub const RT5651_TDM_INTEL_SEL_SFT: c_int = 15;

pub const RT5651_TDM_MODE_SEL_SFT: c_int = 14;

pub const RT5651_TDM_CH_NUM_SEL_SFT: c_int = 12;

pub const RT5651_TDM_CH_LEN_SEL_SFT: c_int = 10;

pub const RT5651_TDM_ADC_SEL_SFT: c_int = 9;

pub const RT5651_TDM_ADC_START_SEL_SFT: c_int = 8;

pub const RT5651_TDM_I2S_CH2_SEL_SFT: c_int = 6;

pub const RT5651_TDM_I2S_CH4_SEL_SFT: c_int = 4;

pub const RT5651_TDM_I2S_CH6_SEL_SFT: c_int = 2;

pub const RT5651_TDM_I2S_CH8_SEL_SFT: c_int = 0;

// TDM Control 2 (0x78)

pub const RT5651_TDM_LRCK_POL_SEL_SFT: c_int = 15;

pub const RT5651_TDM_CH_VAL_SEL_SFT: c_int = 14;

pub const RT5651_TDM_CH_VAL_SFT: c_int = 13;

pub const RT5651_TDM_LPBK_SFT: c_int = 12;

pub const RT5651_TDM_LRCK_PULSE_SEL_SFT: c_int = 11;

pub const RT5651_TDM_END_EDGE_SEL_SFT: c_int = 10;

pub const RT5651_TDM_END_EDGE_EN_SFT: c_int = 9;

pub const RT5651_TDM_TRAN_EDGE_SEL_SFT: c_int = 8;

pub const RT5651_M_TDM2_L_SFT: c_int = 7;

pub const RT5651_M_TDM2_R_SFT: c_int = 6;

pub const RT5651_M_TDM4_L_SFT: c_int = 5;

pub const RT5651_M_TDM4_R_SFT: c_int = 4;
// TDM Control 3 (0x79)

pub const RT5651_CH2_L_SEL_SFT: c_int = 12;

pub const RT5651_CH2_R_SEL_SFT: c_int = 8;

pub const RT5651_CH4_L_SEL_SFT: c_int = 4;

pub const RT5651_CH4_R_SEL_SFT: c_int = 0;

// Global Clock Control (0x80)

pub const RT5651_SCLK_SRC_SFT: c_int = 14;

pub const RT5651_PLL1_SRC_SFT: c_int = 12;

pub const RT5651_PLL1_PD_SFT: c_int = 3;

pub const RT5651_PLL_INP_MAX: c_int = 40000000;
pub const RT5651_PLL_INP_MIN: c_int = 256000;
// PLL M/N/K Code Control 1 (0x81)
pub const RT5651_PLL_N_MAX: c_uint = 0x1ff;

pub const RT5651_PLL_N_SFT: c_int = 7;
pub const RT5651_PLL_K_MAX: c_uint = 0x1f;

pub const RT5651_PLL_K_SFT: c_int = 0;
// PLL M/N/K Code Control 2 (0x82)
pub const RT5651_PLL_M_MAX: c_uint = 0xf;

pub const RT5651_PLL_M_SFT: c_int = 12;

pub const RT5651_PLL_M_BP_SFT: c_int = 11;
// PLL tracking mode 1 (0x83)

pub const RT5651_STO1_T_SFT: c_int = 15;

pub const RT5651_STO2_T_SFT: c_int = 12;

pub const RT5651_ASRC2_REF_SFT: c_int = 11;

pub const RT5651_DMIC_1_M_SFT: c_int = 9;

// PLL tracking mode 2 (0x84)

pub const RT5651_STO1_ASRC_EN_SFT: c_int = 15;

pub const RT5651_STO2_ASRC_EN_SFT: c_int = 14;

pub const RT5651_STO1_DAC_M_SFT: c_int = 13;

pub const RT5651_STO2_DAC_M_SFT: c_int = 12;

pub const RT5651_ADC_M_SFT: c_int = 11;

pub const RT5651_I2S1_R_D_SFT: c_int = 4;

pub const RT5651_I2S2_R_D_SFT: c_int = 3;

pub const RT5651_PRE_SCLK_SFT: c_int = 0;

// PLL tracking mode 3 (0x85)

pub const RT5651_I2S1_RATE_SFT: c_int = 12;

pub const RT5651_I2S2_RATE_SFT: c_int = 8;

pub const RT5651_G_ASRC_LP_SFT: c_int = 3;

pub const RT5651_ASRC_LP_F_SFT: c_int = 2;

pub const RT5651_FTK_PH_DET_SFT: c_int = 0;

// PLL tracking mode 6 (0x89)

pub const RT5651_I2S1_PD_SFT: c_int = 12;

pub const RT5651_I2S2_PD_SFT: c_int = 8;
// PLL tracking mode 7 (0x8a)

pub const RT5651_FSI1_RATE_SFT: c_int = 12;

pub const RT5651_FSI2_RATE_SFT: c_int = 8;
// HPOUT Over Current Detection (0x8b)

pub const RT5651_HP_OVCD_SFT: c_int = 10;

pub const RT5651_HP_OC_TH_SFT: c_int = 8;

// Depop Mode Control 1 (0x8e)

pub const RT5651_SMT_TRIG_SFT: c_int = 15;

pub const RT5651_HP_L_SMT_SFT: c_int = 9;

pub const RT5651_HP_R_SMT_SFT: c_int = 8;

pub const RT5651_HP_CD_PD_SFT: c_int = 7;

pub const RT5651_RSTN_SFT: c_int = 6;

pub const RT5651_RSTP_SFT: c_int = 5;

pub const RT5651_HP_CO_SFT: c_int = 4;

pub const RT5651_HP_CP_SFT: c_int = 3;

pub const RT5651_HP_SG_SFT: c_int = 2;

pub const RT5651_HP_DP_SFT: c_int = 1;

pub const RT5651_HP_CB_SFT: c_int = 0;

// Depop Mode Control 2 (0x8f)

pub const RT5651_DEPOP_SFT: c_int = 13;

pub const RT5651_RAMP_SFT: c_int = 12;

pub const RT5651_BPS_SFT: c_int = 11;

pub const RT5651_FAST_UPDN_SFT: c_int = 10;

pub const RT5651_MRES_SFT: c_int = 8;

pub const RT5651_VLO_SFT: c_int = 7;

pub const RT5651_DIG_DP_SFT: c_int = 6;

pub const RT5651_DP_TH_SFT: c_int = 4;
// Depop Mode Control 3 (0x90)

pub const RT5651_CP_SYS_SFT: c_int = 12;

pub const RT5651_CP_FQ1_SFT: c_int = 8;

pub const RT5651_CP_FQ2_SFT: c_int = 4;

pub const RT5651_CP_FQ3_SFT: c_int = 0;
pub const RT5651_CP_FQ_1_5_KHZ: c_int = 0;
pub const RT5651_CP_FQ_3_KHZ: c_int = 1;
pub const RT5651_CP_FQ_6_KHZ: c_int = 2;
pub const RT5651_CP_FQ_12_KHZ: c_int = 3;
pub const RT5651_CP_FQ_24_KHZ: c_int = 4;
pub const RT5651_CP_FQ_48_KHZ: c_int = 5;
pub const RT5651_CP_FQ_96_KHZ: c_int = 6;
pub const RT5651_CP_FQ_192_KHZ: c_int = 7;
// HPOUT charge pump (0x91)

pub const RT5651_OSW_L_SFT: c_int = 11;

pub const RT5651_OSW_R_SFT: c_int = 10;

pub const RT5651_PM_HP_SFT: c_int = 8;

pub const RT5651_IB_HP_SFT: c_int = 6;

// Micbias Control (0x93)

pub const RT5651_MIC1_BS_SFT: c_int = 15;

pub const RT5651_MIC1_CLK_SFT: c_int = 13;

pub const RT5651_MIC1_OVCD_SFT: c_int = 11;

pub const RT5651_MIC1_OVTH_SFT: c_int = 9;

pub const RT5651_PWR_MB_SFT: c_int = 5;

pub const RT5651_PWR_CLK12M_SFT: c_int = 4;

// Analog JD Control 1 (0x94)

pub const RT5651_JD2_CMP_SFT: c_int = 12;

pub const RT5651_JD_PU_SFT: c_int = 11;

pub const RT5651_JD_PD_SFT: c_int = 10;

pub const RT5651_JD_MODE_SEL_SFT: c_int = 8;

pub const RT5651_JD_M_CMP_SFT: c_int = 4;

pub const RT5651_JD_M_PU_SFT: c_int = 3;

pub const RT5651_JD_M_PD_SFT: c_int = 2;

pub const RT5651_JD_M_MODE_SEL_SFT: c_int = 0;

// Analog JD Control 2 (0x95)

pub const RT5651_JD3_CMP_SFT: c_int = 12;
// EQ Control 1 (0xb0)

pub const RT5651_EQ_SRC_SFT: c_int = 15;

pub const RT5651_EQ_UPD_BIT: c_int = 14;

pub const RT5651_EQ_CD_SFT: c_int = 13;

pub const RT5651_EQ_DITH_SFT: c_int = 8;

pub const RT5651_EQ_CD_F_BIT: c_int = 7;

pub const RT5651_EQ_STA_HP2_BIT: c_int = 6;

pub const RT5651_EQ_STA_HP1_BIT: c_int = 5;

pub const RT5651_EQ_STA_BP4_BIT: c_int = 4;

pub const RT5651_EQ_STA_BP3_BIT: c_int = 3;

pub const RT5651_EQ_STA_BP2_BIT: c_int = 2;

pub const RT5651_EQ_STA_BP1_BIT: c_int = 1;

pub const RT5651_EQ_STA_LP_BIT: c_int = 0;
// EQ Control 2 (0xb1)

pub const RT5651_EQ_HPF1_M_SFT: c_int = 8;

pub const RT5651_EQ_LPF1_M_SFT: c_int = 7;

pub const RT5651_EQ_HPF2_SFT: c_int = 6;

pub const RT5651_EQ_HPF1_SFT: c_int = 5;

pub const RT5651_EQ_BPF4_SFT: c_int = 4;

pub const RT5651_EQ_BPF3_SFT: c_int = 3;

pub const RT5651_EQ_BPF2_SFT: c_int = 2;

pub const RT5651_EQ_BPF1_SFT: c_int = 1;

pub const RT5651_EQ_LPF_SFT: c_int = 0;

// Memory Test (0xb2)

pub const RT5651_MT_SFT: c_int = 15;

// ALC Control 1 (0xb4)

pub const RT5651_ALC_P_SFT: c_int = 15;

pub const RT5651_ALC_SFT: c_int = 14;

pub const RT5651_ALC_UPD_BIT: c_int = 13;

pub const RT5651_ALC_AR_SFT: c_int = 8;

pub const RT5651_ALC_R_SFT: c_int = 5;

pub const RT5651_ALC_RC_SFT: c_int = 0;
// ALC Control 2 (0xb5)

pub const RT5651_ALC_POB_SFT: c_int = 8;

pub const RT5651_ALC_DRC_SFT: c_int = 7;

pub const RT5651_ALC_CPR_SFT: c_int = 5;

pub const RT5651_ALC_PRB_SFT: c_int = 0;
// ALC Control 3 (0xb6)

pub const RT5651_ALC_NGB_SFT: c_int = 12;

pub const RT5651_ALC_TAR_SFT: c_int = 7;

pub const RT5651_ALC_NG_SFT: c_int = 6;

pub const RT5651_ALC_NGH_SFT: c_int = 5;

pub const RT5651_ALC_NGT_SFT: c_int = 0;
// Jack Detect Control 1 (0xbb)

pub const RT5651_JD_SFT: c_int = 13;

pub const RT5651_JD_HP_SFT: c_int = 11;

pub const RT5651_JD_HP_TRG_SFT: c_int = 10;

pub const RT5651_JD_SPL_SFT: c_int = 9;

pub const RT5651_JD_SPL_TRG_SFT: c_int = 8;

pub const RT5651_JD_SPR_SFT: c_int = 7;

pub const RT5651_JD_SPR_TRG_SFT: c_int = 6;

pub const RT5651_JD_LO_SFT: c_int = 3;

pub const RT5651_JD_LO_TRG_SFT: c_int = 2;

// Jack Detect Control 2 (0xbc)

pub const RT5651_JD_TRG_SEL_SFT: c_int = 9;

pub const RT5651_JD3_IRQ_EN_SFT: c_int = 8;

pub const RT5651_JD3_EN_STKY_SFT: c_int = 7;

pub const RT5651_JD3_INV_SFT: c_int = 6;
// IRQ Control 1 (0xbd)

pub const RT5651_IRQ_JD_SFT: c_int = 15;

pub const RT5651_JD_STKY_SFT: c_int = 13;

pub const RT5651_JD_P_SFT: c_int = 11;

pub const RT5651_JD1_1_IRQ_EN_SFT: c_int = 9;

pub const RT5651_JD1_1_EN_STKY_SFT: c_int = 8;

pub const RT5651_JD1_1_INV_SFT: c_int = 7;

pub const RT5651_JD1_2_IRQ_EN_SFT: c_int = 6;

pub const RT5651_JD1_2_EN_STKY_SFT: c_int = 5;

pub const RT5651_JD1_2_INV_SFT: c_int = 4;

pub const RT5651_JD2_IRQ_EN_SFT: c_int = 3;

pub const RT5651_JD2_EN_STKY_SFT: c_int = 2;

pub const RT5651_JD2_INV_SFT: c_int = 1;
// IRQ Control 2 (0xbe)

pub const RT5651_IRQ_MB1_OC_SFT: c_int = 15;

pub const RT5651_MB1_OC_STKY_SFT: c_int = 11;

pub const RT5651_MB1_OC_P_SFT: c_int = 7;

pub const RT5651_MB1_OC_CLR_SFT: c_int = 3;

pub const RT5651_STA_GPIO8_BIT: c_int = 0;
// Internal Status and GPIO status (0xbf)

pub const RT5651_STA_JD3_BIT: c_int = 15;

pub const RT5651_STA_JD2_BIT: c_int = 14;

pub const RT5651_STA_JD1_2_BIT: c_int = 13;

pub const RT5651_STA_JD1_1_BIT: c_int = 12;

pub const RT5651_STA_GP7_BIT: c_int = 11;

pub const RT5651_STA_GP6_BIT: c_int = 10;

pub const RT5651_STA_GP5_BIT: c_int = 9;

pub const RT5651_STA_GP1_BIT: c_int = 8;

pub const RT5651_STA_GP2_BIT: c_int = 7;

pub const RT5651_STA_GP3_BIT: c_int = 6;

pub const RT5651_STA_GP4_BIT: c_int = 5;

pub const RT5651_STA_GP_JD_BIT: c_int = 4;
// GPIO Control 1 (0xc0)

pub const RT5651_GP1_PIN_SFT: c_int = 15;

pub const RT5651_GP2_PIN_SFT: c_int = 14;

pub const RT5651_GPIO_M_SFT: c_int = 9;

pub const RT5651_I2S2_SEL_SFT: c_int = 8;

pub const RT5651_GP5_PIN_SFT: c_int = 7;

pub const RT5651_GP6_PIN_SFT: c_int = 6;

pub const RT5651_GP7_PIN_SFT: c_int = 5;

pub const RT5651_GP8_PIN_SFT: c_int = 4;

pub const RT5651_GPIO_PDM_SEL_SFT: c_int = 3;

// GPIO Control 2 (0xc1)

pub const RT5651_GP5_DR_SFT: c_int = 14;

pub const RT5651_GP5_OUT_SFT: c_int = 13;

pub const RT5651_GP5_P_SFT: c_int = 12;

pub const RT5651_GP4_DR_SFT: c_int = 11;

pub const RT5651_GP4_OUT_SFT: c_int = 10;

pub const RT5651_GP4_P_SFT: c_int = 9;

pub const RT5651_GP3_DR_SFT: c_int = 8;

pub const RT5651_GP3_OUT_SFT: c_int = 7;

pub const RT5651_GP3_P_SFT: c_int = 6;

pub const RT5651_GP2_DR_SFT: c_int = 5;

pub const RT5651_GP2_OUT_SFT: c_int = 4;

pub const RT5651_GP2_P_SFT: c_int = 3;

pub const RT5651_GP1_DR_SFT: c_int = 2;

pub const RT5651_GP1_OUT_SFT: c_int = 1;

pub const RT5651_GP1_P_SFT: c_int = 0;

// GPIO Control 3 (0xc2)

pub const RT5651_GP8_DR_SFT: c_int = 8;

pub const RT5651_GP8_OUT_SFT: c_int = 7;

pub const RT5651_GP8_P_SFT: c_int = 6;

pub const RT5651_GP7_DR_SFT: c_int = 5;

pub const RT5651_GP7_OUT_SFT: c_int = 4;

pub const RT5651_GP7_P_SFT: c_int = 3;

pub const RT5651_GP6_DR_SFT: c_int = 2;

pub const RT5651_GP6_OUT_SFT: c_int = 1;

pub const RT5651_GP6_P_SFT: c_int = 0;

// Scramble Control (0xce)

pub const RT5651_SCB_SWAP_SFT: c_int = 15;

pub const RT5651_SCB_SFT: c_int = 14;

// Baseback Control (0xcf)

pub const RT5651_BB_SFT: c_int = 15;

pub const RT5651_BB_CT_SFT: c_int = 12;

pub const RT5651_M_BB_L_SFT: c_int = 9;

pub const RT5651_M_BB_R_SFT: c_int = 8;

pub const RT5651_M_BB_HPF_L_SFT: c_int = 7;

pub const RT5651_M_BB_HPF_R_SFT: c_int = 6;

pub const RT5651_G_BB_BST_SFT: c_int = 0;
// MP3 Plus Control 1 (0xd0)

pub const RT5651_M_MP3_L_SFT: c_int = 15;

pub const RT5651_M_MP3_R_SFT: c_int = 14;

pub const RT5651_M_MP3_SFT: c_int = 13;

pub const RT5651_EG_MP3_SFT: c_int = 8;

pub const RT5651_MP3_HLP_SFT: c_int = 7;

pub const RT5651_M_MP3_ORG_L_SFT: c_int = 6;

pub const RT5651_M_MP3_ORG_R_SFT: c_int = 5;
// MP3 Plus Control 2 (0xd1)

pub const RT5651_MP3_WT_SFT: c_int = 13;

pub const RT5651_OG_MP3_SFT: c_int = 8;

pub const RT5651_HG_MP3_SFT: c_int = 0;
// 3D HP Control 1 (0xd2)

pub const RT5651_3D_CF_SFT: c_int = 15;

pub const RT5651_3D_HP_SFT: c_int = 14;

pub const RT5651_3D_BT_SFT: c_int = 13;

pub const RT5651_3D_1F_MIX_SFT: c_int = 11;

pub const RT5651_3D_HP_M_SFT: c_int = 10;

pub const RT5651_M_3D_HRTF_SFT: c_int = 9;

pub const RT5651_M_3D_D2H_SFT: c_int = 8;

pub const RT5651_M_3D_D2R_SFT: c_int = 7;

pub const RT5651_M_3D_REVB_SFT: c_int = 6;
// Adjustable high pass filter control 1 (0xd3)

pub const RT5651_2ND_HPF_SFT: c_int = 15;

pub const RT5651_HPF_CF_L_SFT: c_int = 12;

pub const RT5651_HPF_CF_R_SFT: c_int = 8;

pub const RT5651_ZD_T_SFT: c_int = 6;

pub const RT5651_ZD_F_SFT: c_int = 4;

// Adjustable high pass filter control 2 (0xd4)

pub const RT5651_HPF_CF_L_NUM_SFT: c_int = 8;

pub const RT5651_HPF_CF_R_NUM_SFT: c_int = 0;
// HP calibration control and Amp detection (0xd6)

pub const RT5651_SI_DAC_SFT: c_int = 11;

pub const RT5651_DC_CAL_M_SFT: c_int = 10;

pub const RT5651_DC_CAL_SFT: c_int = 9;

pub const RT5651_HPD_RCV_SFT: c_int = 6;

pub const RT5651_HPD_PS_SFT: c_int = 5;

pub const RT5651_CAL_M_SFT: c_int = 4;

pub const RT5651_CAL_SFT: c_int = 3;

pub const RT5651_CAL_TEST_SFT: c_int = 2;

pub const RT5651_CAL_P_SFT: c_int = 0;

// Soft volume and zero cross control 1 (0xd9)

pub const RT5651_SV_SFT: c_int = 15;

pub const RT5651_OUT_SV_SFT: c_int = 13;

pub const RT5651_HP_SV_SFT: c_int = 12;

pub const RT5651_ZCD_DIG_SFT: c_int = 11;

pub const RT5651_ZCD_SFT: c_int = 10;

pub const RT5651_M_ZCD_SFT: c_int = 4;

pub const RT5651_SV_DLY_SFT: c_int = 0;
// Soft volume and zero cross control 2 (0xda)

pub const RT5651_ZCD_HP_SFT: c_int = 15;

// Digital Misc Control (0xfa)

pub const RT5651_I2S2_MS_SP_SEL: c_int = 8;

pub const RT5651_CLK_DET_EN_SFT: c_int = 3;

pub const RT5651_AMP_DET_EN_SFT: c_int = 1;

pub const RT5651_D_GATE_EN_SFT: c_int = 0;
// Codec Private Register definition
// MIC Over current threshold scale factor (0x15)

pub const RT5651_MIC_OVCD_SF_SFT: c_int = 8;

// 3D Speaker Control (0x63)

pub const RT5651_3D_SPK_SFT: c_int = 15;

pub const RT5651_3D_SPK_M_SFT: c_int = 13;

pub const RT5651_3D_SPK_CG_SFT: c_int = 8;

pub const RT5651_3D_SPK_SG_SFT: c_int = 0;
// Wind Noise Detection Control 1 (0x6c)

pub const RT5651_WND_SFT: c_int = 15;

// Wind Noise Detection Control 2 (0x6d)

pub const RT5651_WND_FC_NW_SFT: c_int = 10;

pub const RT5651_WND_FC_WK_SFT: c_int = 4;
// Wind Noise Detection Control 3 (0x6e)

pub const RT5651_HPF_FC_SFT: c_int = 6;

pub const RT5651_WND_FC_ST_SFT: c_int = 0;
// Wind Noise Detection Control 4 (0x6f)

pub const RT5651_WND_TH_LO_SFT: c_int = 0;
// Wind Noise Detection Control 5 (0x70)

pub const RT5651_WND_TH_HI_SFT: c_int = 0;
// Wind Noise Detection Control 8 (0x73)

pub const RT5651_WND_WIND_SFT: c_int = 13;

pub const RT5651_WND_STRONG_SFT: c_int = 12;
// Dipole Speaker Interface (0x75)

pub const RT5651_DP_ATT_SFT: c_int = 14;

pub const RT5651_DP_SPK_SFT: c_int = 10;

// EQ Pre Volume Control (0xb3)

pub const RT5651_EQ_PRE_VOL_SFT: c_int = 0;
// EQ Post Volume Control (0xb4)

pub const RT5651_EQ_PST_VOL_SFT: c_int = 0;
// System Clock Source
// PLL1 Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5651_pll_code {
    pub /: *mut *mut bool m_bp; / Indicates bypass m code or not.,
    pub m_code: c_int,
    pub n_code: c_int,
    pub k_code: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5651_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
// Jack and button detect data
    pub hp_jack: *mut snd_soc_jack,
    pub gpiod_hp_det: *mut gpio_desc,
    pub jack_detect_work: work_struct,
    pub bp_work: delayed_work,
    pub ovcd_irq_enabled: bool,
    pub pressed: bool,
    pub press_reported: bool,
    pub press_count: c_int,
    pub release_count: c_int,
    pub poll_count: c_int,
    pub jd_src: c_uint,
    pub jd_active_high: bool,
    pub ovcd_th: c_uint,
    pub ovcd_sf: c_uint,
    pub irq: c_int,
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: [c_int; RT5651_AIFS],
    pub bclk: [c_int; RT5651_AIFS],
    pub master: [c_int; RT5651_AIFS],
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
    pub dmic_en: c_int,
    pub hp_mute: bool,
}
