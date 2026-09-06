//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5660.h
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
// rt5660.h  --  RT5660 ALSA SoC audio driver
//
// Copyright 2016 Realtek Semiconductor Corp.
// Author: Oder Chiou <oder_chiou@realtek.com>
//

// Info
pub const RT5660_RESET: c_uint = 0x00;
pub const RT5660_VENDOR_ID: c_uint = 0xfd;
pub const RT5660_VENDOR_ID1: c_uint = 0xfe;
pub const RT5660_VENDOR_ID2: c_uint = 0xff;
// I/O - Output
pub const RT5660_SPK_VOL: c_uint = 0x01;
pub const RT5660_LOUT_VOL: c_uint = 0x02;
// I/O - Input
pub const RT5660_IN1_IN2: c_uint = 0x0d;
pub const RT5660_IN3_IN4: c_uint = 0x0e;
// I/O - ADC/DAC/DMIC
pub const RT5660_DAC1_DIG_VOL: c_uint = 0x19;
pub const RT5660_STO1_ADC_DIG_VOL: c_uint = 0x1c;
pub const RT5660_ADC_BST_VOL1: c_uint = 0x1e;
// Mixer - D-D
pub const RT5660_STO1_ADC_MIXER: c_uint = 0x27;
pub const RT5660_AD_DA_MIXER: c_uint = 0x29;
pub const RT5660_STO_DAC_MIXER: c_uint = 0x2a;
pub const RT5660_DIG_INF1_DATA: c_uint = 0x2f;
// Mixer - ADC
pub const RT5660_REC_L1_MIXER: c_uint = 0x3b;
pub const RT5660_REC_L2_MIXER: c_uint = 0x3c;
pub const RT5660_REC_R1_MIXER: c_uint = 0x3d;
pub const RT5660_REC_R2_MIXER: c_uint = 0x3e;
// Mixer - DAC
pub const RT5660_LOUT_MIXER: c_uint = 0x45;
pub const RT5660_SPK_MIXER: c_uint = 0x46;
pub const RT5660_SPO_MIXER: c_uint = 0x48;
pub const RT5660_SPO_CLSD_RATIO: c_uint = 0x4a;
pub const RT5660_OUT_L_GAIN1: c_uint = 0x4d;
pub const RT5660_OUT_L_GAIN2: c_uint = 0x4e;
pub const RT5660_OUT_L1_MIXER: c_uint = 0x4f;
pub const RT5660_OUT_R_GAIN1: c_uint = 0x50;
pub const RT5660_OUT_R_GAIN2: c_uint = 0x51;
pub const RT5660_OUT_R1_MIXER: c_uint = 0x52;
// Power
pub const RT5660_PWR_DIG1: c_uint = 0x61;
pub const RT5660_PWR_DIG2: c_uint = 0x62;
pub const RT5660_PWR_ANLG1: c_uint = 0x63;
pub const RT5660_PWR_ANLG2: c_uint = 0x64;
pub const RT5660_PWR_MIXER: c_uint = 0x65;
pub const RT5660_PWR_VOL: c_uint = 0x66;
// Private Register Control
pub const RT5660_PRIV_INDEX: c_uint = 0x6a;
pub const RT5660_PRIV_DATA: c_uint = 0x6c;
// Format - ADC/DAC
pub const RT5660_I2S1_SDP: c_uint = 0x70;
pub const RT5660_ADDA_CLK1: c_uint = 0x73;
pub const RT5660_ADDA_CLK2: c_uint = 0x74;
pub const RT5660_DMIC_CTRL1: c_uint = 0x75;
// Function - Analog
pub const RT5660_GLB_CLK: c_uint = 0x80;
pub const RT5660_PLL_CTRL1: c_uint = 0x81;
pub const RT5660_PLL_CTRL2: c_uint = 0x82;
pub const RT5660_CLSD_AMP_OC_CTRL: c_uint = 0x8c;
pub const RT5660_CLSD_AMP_CTRL: c_uint = 0x8d;
pub const RT5660_LOUT_AMP_CTRL: c_uint = 0x8e;
pub const RT5660_SPK_AMP_SPKVDD: c_uint = 0x92;
pub const RT5660_MICBIAS: c_uint = 0x93;
pub const RT5660_CLSD_OUT_CTRL1: c_uint = 0xa1;
pub const RT5660_CLSD_OUT_CTRL2: c_uint = 0xa2;
pub const RT5660_DIPOLE_MIC_CTRL1: c_uint = 0xa3;
pub const RT5660_DIPOLE_MIC_CTRL2: c_uint = 0xa4;
pub const RT5660_DIPOLE_MIC_CTRL3: c_uint = 0xa5;
pub const RT5660_DIPOLE_MIC_CTRL4: c_uint = 0xa6;
pub const RT5660_DIPOLE_MIC_CTRL5: c_uint = 0xa7;
pub const RT5660_DIPOLE_MIC_CTRL6: c_uint = 0xa8;
pub const RT5660_DIPOLE_MIC_CTRL7: c_uint = 0xa9;
pub const RT5660_DIPOLE_MIC_CTRL8: c_uint = 0xaa;
pub const RT5660_DIPOLE_MIC_CTRL9: c_uint = 0xab;
pub const RT5660_DIPOLE_MIC_CTRL10: c_uint = 0xac;
pub const RT5660_DIPOLE_MIC_CTRL11: c_uint = 0xad;
pub const RT5660_DIPOLE_MIC_CTRL12: c_uint = 0xae;
// Function - Digital
pub const RT5660_EQ_CTRL1: c_uint = 0xb0;
pub const RT5660_EQ_CTRL2: c_uint = 0xb1;
pub const RT5660_DRC_AGC_CTRL1: c_uint = 0xb3;
pub const RT5660_DRC_AGC_CTRL2: c_uint = 0xb4;
pub const RT5660_DRC_AGC_CTRL3: c_uint = 0xb5;
pub const RT5660_DRC_AGC_CTRL4: c_uint = 0xb6;
pub const RT5660_DRC_AGC_CTRL5: c_uint = 0xb7;
pub const RT5660_JD_CTRL: c_uint = 0xbb;
pub const RT5660_IRQ_CTRL1: c_uint = 0xbd;
pub const RT5660_IRQ_CTRL2: c_uint = 0xbe;
pub const RT5660_INT_IRQ_ST: c_uint = 0xbf;
pub const RT5660_GPIO_CTRL1: c_uint = 0xc0;
pub const RT5660_GPIO_CTRL2: c_uint = 0xc2;
pub const RT5660_WIND_FILTER_CTRL1: c_uint = 0xd3;
pub const RT5660_SV_ZCD1: c_uint = 0xd9;
pub const RT5660_SV_ZCD2: c_uint = 0xda;
pub const RT5660_DRC1_LM_CTRL1: c_uint = 0xe0;
pub const RT5660_DRC1_LM_CTRL2: c_uint = 0xe1;
pub const RT5660_DRC2_LM_CTRL1: c_uint = 0xe2;
pub const RT5660_DRC2_LM_CTRL2: c_uint = 0xe3;
pub const RT5660_MULTI_DRC_CTRL: c_uint = 0xe4;
pub const RT5660_DRC2_CTRL1: c_uint = 0xe5;
pub const RT5660_DRC2_CTRL2: c_uint = 0xe6;
pub const RT5660_DRC2_CTRL3: c_uint = 0xe7;
pub const RT5660_DRC2_CTRL4: c_uint = 0xe8;
pub const RT5660_DRC2_CTRL5: c_uint = 0xe9;
pub const RT5660_ALC_PGA_CTRL1: c_uint = 0xea;
pub const RT5660_ALC_PGA_CTRL2: c_uint = 0xeb;
pub const RT5660_ALC_PGA_CTRL3: c_uint = 0xec;
pub const RT5660_ALC_PGA_CTRL4: c_uint = 0xed;
pub const RT5660_ALC_PGA_CTRL5: c_uint = 0xee;
pub const RT5660_ALC_PGA_CTRL6: c_uint = 0xef;
pub const RT5660_ALC_PGA_CTRL7: c_uint = 0xf0;
// General Control
pub const RT5660_GEN_CTRL1: c_uint = 0xfa;
pub const RT5660_GEN_CTRL2: c_uint = 0xfb;
pub const RT5660_GEN_CTRL3: c_uint = 0xfc;
// Index of Codec Private Register definition
pub const RT5660_CHOP_DAC_ADC: c_uint = 0x3d;
// Global Definition

pub const RT5660_L_MUTE_SFT: c_int = 15;

pub const RT5660_VOL_L_SFT: c_int = 14;

pub const RT5660_R_MUTE_SFT: c_int = 7;

pub const RT5660_VOL_R_SFT: c_int = 6;

pub const RT5660_L_VOL_SFT: c_int = 8;

pub const RT5660_R_VOL_SFT: c_int = 0;
// IN1 and IN2 Control (0x0d)

pub const RT5660_IN_SFT1: c_int = 15;

pub const RT5660_BST_SFT1: c_int = 8;

pub const RT5660_IN_SFT2: c_int = 7;

pub const RT5660_BST_SFT2: c_int = 0;
// IN3 and IN4 Control (0x0e)

pub const RT5660_IN_SFT3: c_int = 15;

pub const RT5660_BST_SFT3: c_int = 8;

pub const RT5660_IN_SFT4: c_int = 7;

pub const RT5660_BST_SFT4: c_int = 0;
// DAC1 Digital Volume (0x19)

pub const RT5660_DAC_L1_VOL_SFT: c_int = 9;

pub const RT5660_DAC_R1_VOL_SFT: c_int = 1;
// ADC Digital Volume Control (0x1c)

pub const RT5660_ADC_L_VOL_SFT: c_int = 9;

pub const RT5660_ADC_R_VOL_SFT: c_int = 1;
// ADC Boost Volume Control (0x1e)

pub const RT5660_STO1_ADC_L_BST_SFT: c_int = 14;

pub const RT5660_STO1_ADC_R_BST_SFT: c_int = 12;
// Stereo ADC Mixer Control (0x27)

pub const RT5660_M_ADC_L1_SFT: c_int = 14;

pub const RT5660_M_ADC_L2_SFT: c_int = 13;

pub const RT5660_M_ADC_R1_SFT: c_int = 6;

pub const RT5660_M_ADC_R2_SFT: c_int = 5;
// ADC Mixer to DAC Mixer Control (0x29)

pub const RT5660_M_ADCMIX_L_SFT: c_int = 15;

pub const RT5660_M_DAC1_L_SFT: c_int = 14;

pub const RT5660_M_ADCMIX_R_SFT: c_int = 7;

pub const RT5660_M_DAC1_R_SFT: c_int = 6;
// Stereo DAC Mixer Control (0x2a)

pub const RT5660_M_DAC_L1_SFT: c_int = 14;

pub const RT5660_DAC_L1_STO_L_VOL_SFT: c_int = 13;

pub const RT5660_M_DAC_R1_STO_L_SFT: c_int = 9;

pub const RT5660_DAC_R1_STO_L_VOL_SFT: c_int = 8;

pub const RT5660_M_DAC_R1_SFT: c_int = 6;

pub const RT5660_DAC_R1_STO_R_VOL_SFT: c_int = 5;

pub const RT5660_M_DAC_L1_STO_R_SFT: c_int = 1;

pub const RT5660_DAC_L1_STO_R_VOL_SFT: c_int = 0;
// Digital Interface Data Control (0x2f)

pub const RT5660_IF1_DAC_IN_SFT: c_int = 14;

pub const RT5660_IF1_ADC_IN_SFT: c_int = 12;
// REC Left Mixer Control 1 (0x3b)

pub const RT5660_G_BST3_RM_L_SFT: c_int = 4;

pub const RT5660_G_BST2_RM_L_SFT: c_int = 1;
// REC Left Mixer Control 2 (0x3c)

pub const RT5660_G_BST1_RM_L_SFT: c_int = 13;

pub const RT5660_G_OM_L_RM_L_SFT: c_int = 10;

pub const RT5660_M_BST3_RM_L_SFT: c_int = 3;

pub const RT5660_M_BST2_RM_L_SFT: c_int = 2;

pub const RT5660_M_BST1_RM_L_SFT: c_int = 1;

pub const RT5660_M_OM_L_RM_L_SFT: c_int = 0;
// REC Right Mixer Control 1 (0x3d)

pub const RT5660_G_BST3_RM_R_SFT: c_int = 4;

pub const RT5660_G_BST2_RM_R_SFT: c_int = 1;
// REC Right Mixer Control 2 (0x3e)

pub const RT5660_G_BST1_RM_R_SFT: c_int = 13;

pub const RT5660_G_OM_R_RM_R_SFT: c_int = 10;

pub const RT5660_M_BST3_RM_R_SFT: c_int = 3;

pub const RT5660_M_BST2_RM_R_SFT: c_int = 2;

pub const RT5660_M_BST1_RM_R_SFT: c_int = 1;

pub const RT5660_M_OM_R_RM_R_SFT: c_int = 0;
// LOUTMIX Control (0x45)

pub const RT5660_M_DAC1_LM_SFT: c_int = 14;

pub const RT5660_M_LOVOL_LM_SFT: c_int = 13;
// SPK Mixer Control (0x46)

pub const RT5660_G_BST3_SM_SFT: c_int = 14;

pub const RT5660_G_BST1_SM_SFT: c_int = 12;

pub const RT5660_G_DACl_SM_SFT: c_int = 10;

pub const RT5660_G_DACR_SM_SFT: c_int = 8;

pub const RT5660_G_OM_L_SM_SFT: c_int = 6;

pub const RT5660_M_DACR_SM_SFT: c_int = 5;

pub const RT5660_M_BST1_SM_SFT: c_int = 4;

pub const RT5660_M_BST3_SM_SFT: c_int = 3;

pub const RT5660_M_DACL_SM_SFT: c_int = 2;

pub const RT5660_M_OM_L_SM_SFT: c_int = 1;
// SPOMIX Control (0x48)

pub const RT5660_M_DAC_R_SPM_SFT: c_int = 14;

pub const RT5660_M_DAC_L_SPM_SFT: c_int = 13;

pub const RT5660_M_SV_SPM_SFT: c_int = 12;

pub const RT5660_M_BST1_SPM_SFT: c_int = 11;
// Output Left Mixer Control 1 (0x4d)

pub const RT5660_G_BST3_OM_L_SFT: c_int = 13;

pub const RT5660_G_BST2_OM_L_SFT: c_int = 10;

pub const RT5660_G_BST1_OM_L_SFT: c_int = 7;

pub const RT5660_G_RM_L_OM_L_SFT: c_int = 1;
// Output Left Mixer Control 2 (0x4e)

pub const RT5660_G_DAC_R1_OM_L_SFT: c_int = 10;

pub const RT5660_G_DAC_L1_OM_L_SFT: c_int = 7;
// Output Left Mixer Control 3 (0x4f)

pub const RT5660_M_BST3_OM_L_SFT: c_int = 5;

pub const RT5660_M_BST2_OM_L_SFT: c_int = 4;

pub const RT5660_M_BST1_OM_L_SFT: c_int = 3;

pub const RT5660_M_RM_L_OM_L_SFT: c_int = 2;

pub const RT5660_M_DAC_R_OM_L_SFT: c_int = 1;

pub const RT5660_M_DAC_L_OM_L_SFT: c_int = 0;
// Output Right Mixer Control 1 (0x50)

pub const RT5660_G_BST2_OM_R_SFT: c_int = 10;

pub const RT5660_G_BST1_OM_R_SFT: c_int = 7;

pub const RT5660_G_RM_R_OM_R_SFT: c_int = 1;
// Output Right Mixer Control 2 (0x51)

pub const RT5660_G_DAC_L_OM_R_SFT: c_int = 10;

pub const RT5660_G_DAC_R_OM_R_SFT: c_int = 7;
// Output Right Mixer Control 3 (0x52)

pub const RT5660_M_BST2_OM_R_SFT: c_int = 4;

pub const RT5660_M_BST1_OM_R_SFT: c_int = 3;

pub const RT5660_M_RM_R_OM_R_SFT: c_int = 2;

pub const RT5660_M_DAC_L_OM_R_SFT: c_int = 1;

pub const RT5660_M_DAC_R_OM_R_SFT: c_int = 0;
// Power Management for Digital 1 (0x61)

pub const RT5660_PWR_I2S1_BIT: c_int = 15;

pub const RT5660_PWR_DAC_L1_BIT: c_int = 12;

pub const RT5660_PWR_DAC_R1_BIT: c_int = 11;

pub const RT5660_PWR_ADC_L_BIT: c_int = 2;

pub const RT5660_PWR_ADC_R_BIT: c_int = 1;

pub const RT5660_PWR_CLS_D_BIT: c_int = 0;
// Power Management for Digital 2 (0x62)

pub const RT5660_PWR_ADC_S1F_BIT: c_int = 15;

pub const RT5660_PWR_DAC_S1F_BIT: c_int = 11;
// Power Management for Analog 1 (0x63)

pub const RT5660_PWR_VREF1_BIT: c_int = 15;

pub const RT5660_PWR_FV1_BIT: c_int = 14;

pub const RT5660_PWR_MB_BIT: c_int = 13;

pub const RT5660_PWR_BG_BIT: c_int = 11;

pub const RT5660_PWR_HP_L_BIT: c_int = 7;

pub const RT5660_PWR_HP_R_BIT: c_int = 6;

pub const RT5660_PWR_HA_BIT: c_int = 5;

pub const RT5660_PWR_VREF2_BIT: c_int = 4;

pub const RT5660_PWR_FV2_BIT: c_int = 3;

pub const RT5660_PWR_LDO2_BIT: c_int = 2;
// Power Management for Analog 2 (0x64)

pub const RT5660_PWR_BST1_BIT: c_int = 15;

pub const RT5660_PWR_BST2_BIT: c_int = 14;

pub const RT5660_PWR_BST3_BIT: c_int = 13;

pub const RT5660_PWR_MB1_BIT: c_int = 11;

pub const RT5660_PWR_MB2_BIT: c_int = 10;

pub const RT5660_PWR_PLL_BIT: c_int = 9;
// Power Management for Mixer (0x65)

pub const RT5660_PWR_OM_L_BIT: c_int = 15;

pub const RT5660_PWR_OM_R_BIT: c_int = 14;

pub const RT5660_PWR_SM_BIT: c_int = 13;

pub const RT5660_PWR_RM_L_BIT: c_int = 11;

pub const RT5660_PWR_RM_R_BIT: c_int = 10;
// Power Management for Volume (0x66)

pub const RT5660_PWR_SV_BIT: c_int = 15;

pub const RT5660_PWR_LV_L_BIT: c_int = 11;

pub const RT5660_PWR_LV_R_BIT: c_int = 10;
// I2S1 Audio Serial Data Port Control (0x70)

pub const RT5660_I2S_MS_SFT: c_int = 15;

pub const RT5660_I2S_O_CP_SFT: c_int = 10;

pub const RT5660_I2S_I_CP_SFT: c_int = 8;

pub const RT5660_I2S_BP_SFT: c_int = 7;

pub const RT5660_I2S_DL_SFT: c_int = 2;

pub const RT5660_I2S_DF_SFT: c_int = 0;

// ADC/DAC Clock Control 1 (0x73)

pub const RT5660_I2S_BCLK_MS1_SFT: c_int = 15;

pub const RT5660_I2S_PD1_SFT: c_int = 12;

pub const RT5660_DAC_OSR_SFT: c_int = 2;

pub const RT5660_ADC_OSR_SFT: c_int = 0;

// ADC/DAC Clock Control 2 (0x74)

pub const RT5660_RESET_ADF_SFT: c_int = 13;

pub const RT5660_RESET_DAF_SFT: c_int = 12;

pub const RT5660_DAHPF_EN_SFT: c_int = 11;

pub const RT5660_ADHPF_EN_SFT: c_int = 10;
// Digital Microphone Control (0x75)

pub const RT5660_DMIC_1_EN_SFT: c_int = 15;

pub const RT5660_DMIC_1L_LH_SFT: c_int = 13;

pub const RT5660_DMIC_1R_LH_SFT: c_int = 12;

pub const RT5660_SEL_DMIC_DATA_SFT: c_int = 11;

pub const RT5660_DMIC_CLK_SFT: c_int = 5;
// Global Clock Control (0x80)

pub const RT5660_SCLK_SRC_SFT: c_int = 14;

pub const RT5660_PLL1_SRC_SFT: c_int = 12;

pub const RT5660_PLL1_PD_SFT: c_int = 3;

pub const RT5660_PLL_INP_MAX: c_int = 40000000;
pub const RT5660_PLL_INP_MIN: c_int = 256000;
// PLL M/N/K Code Control 1 (0x81)
pub const RT5660_PLL_N_MAX: c_uint = 0x1ff;

pub const RT5660_PLL_N_SFT: c_int = 7;
pub const RT5660_PLL_K_MAX: c_uint = 0x1f;

pub const RT5660_PLL_K_SFT: c_int = 0;
// PLL M/N/K Code Control 2 (0x82)
pub const RT5660_PLL_M_MAX: c_uint = 0xf;

pub const RT5660_PLL_M_SFT: c_int = 12;

pub const RT5660_PLL_M_BP_SFT: c_int = 11;
// Class D Over Current Control (0x8c)

pub const RT5660_CLSD_OC_SFT: c_int = 9;

pub const RT5660_AUTO_PD_SFT: c_int = 8;

pub const RT5660_CLSD_OC_TH_SFT: c_int = 0;
// Class D Output Control (0x8d)

pub const RT5660_CLSD_RATIO_SFT: c_int = 12;
// Lout Amp Control 1 (0x8e)

pub const RT5660_LOUT_CO_SFT: c_int = 4;

pub const RT5660_LOUT_CB_SFT: c_int = 0;

// SPKVDD detection control (0x92)

pub const RT5660_SPKVDD_DET_SFT: c_int = 15;

pub const RT5660_SPK_AG_SFT: c_int = 14;

// Micbias Control (0x93)

pub const RT5660_MIC1_BS_SFT: c_int = 15;

pub const RT5660_MIC2_BS_SFT: c_int = 14;

pub const RT5660_MIC1_OVCD_SFT: c_int = 11;

pub const RT5660_MIC1_OVTH_SFT: c_int = 9;

pub const RT5660_MIC2_OVCD_SFT: c_int = 8;

pub const RT5660_MIC2_OVTH_SFT: c_int = 6;

pub const RT5660_PWR_CLK25M_SFT: c_int = 4;

// EQ Control 1 (0xb0)

pub const RT5660_EQ_SRC_SFT: c_int = 15;

pub const RT5660_EQ_UPD_BIT: c_int = 14;
// Jack Detect Control (0xbb)

pub const RT5660_JD_SFT: c_int = 14;

pub const RT5660_JD_LOUT_SFT: c_int = 11;

pub const RT5660_JD_LOUT_TRG_SFT: c_int = 10;

pub const RT5660_JD_SPO_SFT: c_int = 9;

pub const RT5660_JD_SPO_TRG_SFT: c_int = 8;

// IRQ Control 1 (0xbd)

pub const RT5660_IRQ_JD_SFT: c_int = 15;

pub const RT5660_IRQ_OT_SFT: c_int = 14;

pub const RT5660_JD_STKY_SFT: c_int = 13;

pub const RT5660_OT_STKY_SFT: c_int = 12;

pub const RT5660_JD_P_SFT: c_int = 11;

pub const RT5660_OT_P_SFT: c_int = 10;

// IRQ Control 2 (0xbe)

pub const RT5660_IRQ_MB1_OC_SFT: c_int = 15;

pub const RT5660_IRQ_MB2_OC_SFT: c_int = 14;

pub const RT5660_MB1_OC_STKY_SFT: c_int = 11;

pub const RT5660_MB2_OC_STKY_SFT: c_int = 10;

pub const RT5660_MB1_OC_P_SFT: c_int = 7;

pub const RT5660_MB2_OC_P_SFT: c_int = 6;

pub const RT5660_MB1_OC_CLR_SFT: c_int = 3;

pub const RT5660_MB2_OC_CLR_SFT: c_int = 2;
// GPIO Control 1 (0xc0)

pub const RT5660_GP2_PIN_SFT: c_int = 14;

pub const RT5660_GP1_PIN_SFT: c_int = 12;

pub const RT5660_GPIO_M_SFT: c_int = 9;

// GPIO Control 3 (0xc2)

pub const RT5660_GP2_PF_SFT: c_int = 5;

pub const RT5660_GP2_OUT_SFT: c_int = 4;

pub const RT5660_GP2_P_SFT: c_int = 3;

pub const RT5660_GP1_PF_SFT: c_int = 2;

pub const RT5660_GP1_OUT_SFT: c_int = 1;

pub const RT5660_GP1_P_SFT: c_int = 0;

// Soft volume and zero cross control 1 (0xd9)

pub const RT5660_SV_SFT: c_int = 15;

pub const RT5660_SPO_SV_SFT: c_int = 14;

pub const RT5660_OUT_SV_SFT: c_int = 12;

pub const RT5660_ZCD_DIG_SFT: c_int = 11;

pub const RT5660_ZCD_SFT: c_int = 10;

pub const RT5660_SV_DLY_SFT: c_int = 0;
// Soft volume and zero cross control 2 (0xda)

pub const RT5660_ZCD_SPO_SFT: c_int = 15;

pub const RT5660_ZCD_OMR_SFT: c_int = 8;

pub const RT5660_ZCD_OML_SFT: c_int = 7;

pub const RT5660_ZCD_SPM_SFT: c_int = 6;

pub const RT5660_ZCD_RMR_SFT: c_int = 5;

pub const RT5660_ZCD_RML_SFT: c_int = 4;

// General Control 1 (0xfa)

pub const RT5660_PWR_VREF_HP_SFT: c_int = 11;

pub const RT5660_DIG_GATE_CTRL_SFT: c_int = 0;
// System Clock Source
pub const RT5660_SCLK_S_MCLK: c_int = 0;
pub const RT5660_SCLK_S_PLL1: c_int = 1;
pub const RT5660_SCLK_S_RCCLK: c_int = 2;
// PLL1 Source
pub const RT5660_PLL1_S_MCLK: c_int = 0;
pub const RT5660_PLL1_S_BCLK: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5660_priv {
    pub component: *mut snd_soc_component,
    pub pdata: rt5660_platform_data,
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: [c_int; RT5660_AIFS],
    pub bclk: [c_int; RT5660_AIFS],
    pub master: [c_int; RT5660_AIFS],
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
}
