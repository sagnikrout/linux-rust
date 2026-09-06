//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5663.h
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
// rt5663.h  --  RT5663 ALSA SoC audio driver
//
// Copyright 2016 Realtek Microelectronics
// Author: Jack Yu <jack.yu@realtek.com>
//

// Info
pub const RT5663_RESET: c_uint = 0x0000;
pub const RT5663_VENDOR_ID: c_uint = 0x00fd;
pub const RT5663_VENDOR_ID_1: c_uint = 0x00fe;
pub const RT5663_VENDOR_ID_2: c_uint = 0x00ff;
pub const RT5663_LOUT_CTRL: c_uint = 0x0001;
pub const RT5663_HP_AMP_2: c_uint = 0x0003;
pub const RT5663_MONO_OUT: c_uint = 0x0004;
pub const RT5663_MONO_GAIN: c_uint = 0x0007;
pub const RT5663_AEC_BST: c_uint = 0x000b;
pub const RT5663_IN1_IN2: c_uint = 0x000c;
pub const RT5663_IN3_IN4: c_uint = 0x000d;
pub const RT5663_INL1_INR1: c_uint = 0x000f;
pub const RT5663_CBJ_TYPE_2: c_uint = 0x0011;
pub const RT5663_CBJ_TYPE_3: c_uint = 0x0012;
pub const RT5663_CBJ_TYPE_4: c_uint = 0x0013;
pub const RT5663_CBJ_TYPE_5: c_uint = 0x0014;
pub const RT5663_CBJ_TYPE_8: c_uint = 0x0017;
// I/O - ADC/DAC/DMIC
pub const RT5663_DAC3_DIG_VOL: c_uint = 0x001a;
pub const RT5663_DAC3_CTRL: c_uint = 0x001b;
pub const RT5663_MONO_ADC_DIG_VOL: c_uint = 0x001d;
pub const RT5663_STO2_ADC_DIG_VOL: c_uint = 0x001e;
pub const RT5663_MONO_ADC_BST_GAIN: c_uint = 0x0020;
pub const RT5663_STO2_ADC_BST_GAIN: c_uint = 0x0021;
pub const RT5663_SIDETONE_CTRL: c_uint = 0x0024;
// Mixer - D-D
pub const RT5663_MONO1_ADC_MIXER: c_uint = 0x0027;
pub const RT5663_STO2_ADC_MIXER: c_uint = 0x0028;
pub const RT5663_MONO_DAC_MIXER: c_uint = 0x002b;
pub const RT5663_DAC2_SRC_CTRL: c_uint = 0x002e;
pub const RT5663_IF_3_4_DATA_CTL: c_uint = 0x002f;
pub const RT5663_IF_5_DATA_CTL: c_uint = 0x0030;
pub const RT5663_PDM_OUT_CTL: c_uint = 0x0031;
pub const RT5663_PDM_I2C_DATA_CTL1: c_uint = 0x0032;
pub const RT5663_PDM_I2C_DATA_CTL2: c_uint = 0x0033;
pub const RT5663_PDM_I2C_DATA_CTL3: c_uint = 0x0034;
pub const RT5663_PDM_I2C_DATA_CTL4: c_uint = 0x0035;
// Mixer - Analog
pub const RT5663_RECMIX1_NEW: c_uint = 0x003a;
pub const RT5663_RECMIX1L_0: c_uint = 0x003b;
pub const RT5663_RECMIX1L: c_uint = 0x003c;
pub const RT5663_RECMIX1R_0: c_uint = 0x003d;
pub const RT5663_RECMIX1R: c_uint = 0x003e;
pub const RT5663_RECMIX2_NEW: c_uint = 0x003f;
pub const RT5663_RECMIX2_L_2: c_uint = 0x0041;
pub const RT5663_RECMIX2_R: c_uint = 0x0042;
pub const RT5663_RECMIX2_R_2: c_uint = 0x0043;
pub const RT5663_CALIB_REC_LR: c_uint = 0x0044;
pub const RT5663_ALC_BK_GAIN: c_uint = 0x0049;
pub const RT5663_MONOMIX_GAIN: c_uint = 0x004a;
pub const RT5663_MONOMIX_IN_GAIN: c_uint = 0x004b;
pub const RT5663_OUT_MIXL_GAIN: c_uint = 0x004d;
pub const RT5663_OUT_LMIX_IN_GAIN: c_uint = 0x004e;
pub const RT5663_OUT_RMIX_IN_GAIN: c_uint = 0x004f;
pub const RT5663_OUT_RMIX_IN_GAIN1: c_uint = 0x0050;
pub const RT5663_LOUT_MIXER_CTRL: c_uint = 0x0052;
// Power
pub const RT5663_PWR_VOL: c_uint = 0x0067;
pub const RT5663_ADCDAC_RST: c_uint = 0x006d;
// Format - ADC/DAC
pub const RT5663_I2S34_SDP: c_uint = 0x0071;
pub const RT5663_I2S5_SDP: c_uint = 0x0072;
// Function - Analog
pub const RT5663_ASRC_3: c_uint = 0x0085;
pub const RT5663_ASRC_6: c_uint = 0x0088;
pub const RT5663_ASRC_7: c_uint = 0x0089;
pub const RT5663_PLL_TRK_13: c_uint = 0x0099;
pub const RT5663_I2S_M_CLK_CTL: c_uint = 0x00a0;
pub const RT5663_FDIV_I2S34_M_CLK: c_uint = 0x00a1;
pub const RT5663_FDIV_I2S34_M_CLK2: c_uint = 0x00a2;
pub const RT5663_FDIV_I2S5_M_CLK: c_uint = 0x00a3;
pub const RT5663_FDIV_I2S5_M_CLK2: c_uint = 0x00a4;
// Function - Digital
pub const RT5663_V2_IRQ_4: c_uint = 0x00b9;
pub const RT5663_GPIO_3: c_uint = 0x00c2;
pub const RT5663_GPIO_4: c_uint = 0x00c3;
pub const RT5663_GPIO_STA2: c_uint = 0x00c4;
pub const RT5663_HP_AMP_DET1: c_uint = 0x00d0;
pub const RT5663_HP_AMP_DET2: c_uint = 0x00d1;
pub const RT5663_HP_AMP_DET3: c_uint = 0x00d2;
pub const RT5663_MID_BD_HP_AMP: c_uint = 0x00d3;
pub const RT5663_LOW_BD_HP_AMP: c_uint = 0x00d4;
pub const RT5663_SOF_VOL_ZC2: c_uint = 0x00da;
pub const RT5663_ADC_STO2_ADJ1: c_uint = 0x00ee;
pub const RT5663_ADC_STO2_ADJ2: c_uint = 0x00ef;
// General Control
pub const RT5663_A_JD_CTRL: c_uint = 0x00f0;
pub const RT5663_JD1_TRES_CTRL: c_uint = 0x00f1;
pub const RT5663_JD2_TRES_CTRL: c_uint = 0x00f2;
pub const RT5663_V2_JD_CTRL2: c_uint = 0x00f7;
pub const RT5663_DUM_REG_2: c_uint = 0x00fb;
pub const RT5663_DUM_REG_3: c_uint = 0x00fc;
pub const RT5663_DACADC_DIG_VOL2: c_uint = 0x0101;
pub const RT5663_DIG_IN_PIN2: c_uint = 0x0133;
pub const RT5663_PAD_DRV_CTL1: c_uint = 0x0136;
pub const RT5663_SOF_RAM_DEPOP: c_uint = 0x0138;
pub const RT5663_VOL_TEST: c_uint = 0x013f;
pub const RT5663_MONO_DYNA_1: c_uint = 0x0170;
pub const RT5663_MONO_DYNA_2: c_uint = 0x0171;
pub const RT5663_MONO_DYNA_3: c_uint = 0x0172;
pub const RT5663_MONO_DYNA_4: c_uint = 0x0173;
pub const RT5663_MONO_DYNA_5: c_uint = 0x0174;
pub const RT5663_MONO_DYNA_6: c_uint = 0x0175;
pub const RT5663_STO1_SIL_DET: c_uint = 0x0190;
pub const RT5663_MONOL_SIL_DET: c_uint = 0x0191;
pub const RT5663_MONOR_SIL_DET: c_uint = 0x0192;
pub const RT5663_STO2_DAC_SIL: c_uint = 0x0193;
pub const RT5663_PWR_SAV_CTL1: c_uint = 0x0194;
pub const RT5663_PWR_SAV_CTL2: c_uint = 0x0195;
pub const RT5663_PWR_SAV_CTL3: c_uint = 0x0196;
pub const RT5663_PWR_SAV_CTL4: c_uint = 0x0197;
pub const RT5663_PWR_SAV_CTL5: c_uint = 0x0198;
pub const RT5663_PWR_SAV_CTL6: c_uint = 0x0199;
pub const RT5663_MONO_AMP_CAL1: c_uint = 0x01a0;
pub const RT5663_MONO_AMP_CAL2: c_uint = 0x01a1;
pub const RT5663_MONO_AMP_CAL3: c_uint = 0x01a2;
pub const RT5663_MONO_AMP_CAL4: c_uint = 0x01a3;
pub const RT5663_MONO_AMP_CAL5: c_uint = 0x01a4;
pub const RT5663_MONO_AMP_CAL6: c_uint = 0x01a5;
pub const RT5663_MONO_AMP_CAL7: c_uint = 0x01a6;
pub const RT5663_MONO_AMP_CAL_ST1: c_uint = 0x01a7;
pub const RT5663_MONO_AMP_CAL_ST2: c_uint = 0x01a8;
pub const RT5663_MONO_AMP_CAL_ST3: c_uint = 0x01a9;
pub const RT5663_MONO_AMP_CAL_ST4: c_uint = 0x01aa;
pub const RT5663_MONO_AMP_CAL_ST5: c_uint = 0x01ab;
pub const RT5663_V2_HP_IMP_SEN_13: c_uint = 0x01b9;
pub const RT5663_V2_HP_IMP_SEN_14: c_uint = 0x01ba;
pub const RT5663_V2_HP_IMP_SEN_6: c_uint = 0x01bb;
pub const RT5663_V2_HP_IMP_SEN_7: c_uint = 0x01bc;
pub const RT5663_V2_HP_IMP_SEN_8: c_uint = 0x01bd;
pub const RT5663_V2_HP_IMP_SEN_9: c_uint = 0x01be;
pub const RT5663_V2_HP_IMP_SEN_10: c_uint = 0x01bf;
pub const RT5663_HP_LOGIC_3: c_uint = 0x01dc;
pub const RT5663_HP_CALIB_ST10: c_uint = 0x01f3;
pub const RT5663_HP_CALIB_ST11: c_uint = 0x01f4;
pub const RT5663_PRO_REG_TBL_4: c_uint = 0x0203;
pub const RT5663_PRO_REG_TBL_5: c_uint = 0x0204;
pub const RT5663_PRO_REG_TBL_6: c_uint = 0x0205;
pub const RT5663_PRO_REG_TBL_7: c_uint = 0x0206;
pub const RT5663_PRO_REG_TBL_8: c_uint = 0x0207;
pub const RT5663_PRO_REG_TBL_9: c_uint = 0x0208;
pub const RT5663_SAR_ADC_INL_1: c_uint = 0x0210;
pub const RT5663_SAR_ADC_INL_2: c_uint = 0x0211;
pub const RT5663_SAR_ADC_INL_3: c_uint = 0x0212;
pub const RT5663_SAR_ADC_INL_4: c_uint = 0x0213;
pub const RT5663_SAR_ADC_INL_5: c_uint = 0x0214;
pub const RT5663_SAR_ADC_INL_6: c_uint = 0x0215;
pub const RT5663_SAR_ADC_INL_7: c_uint = 0x0216;
pub const RT5663_SAR_ADC_INL_8: c_uint = 0x0217;
pub const RT5663_SAR_ADC_INL_9: c_uint = 0x0218;
pub const RT5663_SAR_ADC_INL_10: c_uint = 0x0219;
pub const RT5663_SAR_ADC_INL_11: c_uint = 0x021a;
pub const RT5663_SAR_ADC_INL_12: c_uint = 0x021b;
pub const RT5663_DRC_CTRL_1: c_uint = 0x02ff;
pub const RT5663_DRC1_CTRL_2: c_uint = 0x0301;
pub const RT5663_DRC1_CTRL_3: c_uint = 0x0302;
pub const RT5663_DRC1_CTRL_4: c_uint = 0x0303;
pub const RT5663_DRC1_CTRL_5: c_uint = 0x0304;
pub const RT5663_DRC1_CTRL_6: c_uint = 0x0305;
pub const RT5663_DRC1_HD_CTRL_1: c_uint = 0x0306;
pub const RT5663_DRC1_HD_CTRL_2: c_uint = 0x0307;
pub const RT5663_DRC1_PRI_REG_1: c_uint = 0x0310;
pub const RT5663_DRC1_PRI_REG_2: c_uint = 0x0311;
pub const RT5663_DRC1_PRI_REG_3: c_uint = 0x0312;
pub const RT5663_DRC1_PRI_REG_4: c_uint = 0x0313;
pub const RT5663_DRC1_PRI_REG_5: c_uint = 0x0314;
pub const RT5663_DRC1_PRI_REG_6: c_uint = 0x0315;
pub const RT5663_DRC1_PRI_REG_7: c_uint = 0x0316;
pub const RT5663_DRC1_PRI_REG_8: c_uint = 0x0317;
pub const RT5663_ALC_PGA_CTL_1: c_uint = 0x0330;
pub const RT5663_ALC_PGA_CTL_2: c_uint = 0x0331;
pub const RT5663_ALC_PGA_CTL_3: c_uint = 0x0332;
pub const RT5663_ALC_PGA_CTL_4: c_uint = 0x0333;
pub const RT5663_ALC_PGA_CTL_5: c_uint = 0x0334;
pub const RT5663_ALC_PGA_CTL_6: c_uint = 0x0335;
pub const RT5663_ALC_PGA_CTL_7: c_uint = 0x0336;
pub const RT5663_ALC_PGA_CTL_8: c_uint = 0x0337;
pub const RT5663_ALC_PGA_REG_1: c_uint = 0x0338;
pub const RT5663_ALC_PGA_REG_2: c_uint = 0x0339;
pub const RT5663_ALC_PGA_REG_3: c_uint = 0x033a;
pub const RT5663_ADC_EQ_RECOV_1: c_uint = 0x03c0;
pub const RT5663_ADC_EQ_RECOV_2: c_uint = 0x03c1;
pub const RT5663_ADC_EQ_RECOV_3: c_uint = 0x03c2;
pub const RT5663_ADC_EQ_RECOV_4: c_uint = 0x03c3;
pub const RT5663_ADC_EQ_RECOV_5: c_uint = 0x03c4;
pub const RT5663_ADC_EQ_RECOV_6: c_uint = 0x03c5;
pub const RT5663_ADC_EQ_RECOV_7: c_uint = 0x03c6;
pub const RT5663_ADC_EQ_RECOV_8: c_uint = 0x03c7;
pub const RT5663_ADC_EQ_RECOV_9: c_uint = 0x03c8;
pub const RT5663_ADC_EQ_RECOV_10: c_uint = 0x03c9;
pub const RT5663_ADC_EQ_RECOV_11: c_uint = 0x03ca;
pub const RT5663_ADC_EQ_RECOV_12: c_uint = 0x03cb;
pub const RT5663_ADC_EQ_RECOV_13: c_uint = 0x03cc;
pub const RT5663_VID_HIDDEN: c_uint = 0x03fe;
pub const RT5663_VID_CUSTOMER: c_uint = 0x03ff;
pub const RT5663_SCAN_MODE: c_uint = 0x07f0;
pub const RT5663_I2C_BYPA: c_uint = 0x07fa;
// Headphone Amp Control 2 (0x0003)

pub const RT5663_EN_DAC_HPO_SHIFT: c_int = 14;

// Headphone Amp L/R Analog Gain and Digital NG2 Gain Control (0x0005 0x0006)

pub const RT5663_GAIN_HP_SHIFT: c_int = 8;
// AEC BST Control (0x000b)

pub const RT5663_GAIN_CBJ_SHIFT: c_int = 8;
// IN1 Control / MIC GND REF (0x000c)

pub const RT5663_IN1_DF_SHIFT: c_int = 15;
// Combo Jack and Type Detection Control 1 (0x0010)

pub const RT5663_CBJ_DET_SHIFT: c_int = 15;

pub const RT5663_DET_TYPE_SHIFT: c_int = 12;

pub const RT5663_VREF_BIAS_SHIFT: c_int = 6;

// REC Left Mixer Control 2 (0x003c)

pub const RT5663_RECMIX1L_BST1_CBJ_SHIFT: c_int = 7;

pub const RT5663_RECMIX1L_BST2_SHIFT: c_int = 4;
// REC Right Mixer Control 2 (0x003e)

pub const RT5663_RECMIX1R_BST2_SHIFT: c_int = 4;
// DAC1 Digital Volume (0x0019)

pub const RT5663_DAC_L1_VOL_SHIFT: c_int = 8;

pub const RT5663_DAC_R1_VOL_SHIFT: c_int = 0;
// ADC Digital Volume Control (0x001c)

pub const RT5663_ADC_L_MUTE_SHIFT: c_int = 15;

pub const RT5663_ADC_L_VOL_SHIFT: c_int = 8;

pub const RT5663_ADC_R_MUTE_SHIFT: c_int = 7;

pub const RT5663_ADC_R_VOL_SHIFT: c_int = 0;
// Stereo ADC Mixer Control (0x0026)

pub const RT5663_M_STO1_ADC_L1_SHIFT: c_int = 15;

pub const RT5663_M_STO1_ADC_L2_SHIFT: c_int = 14;

pub const RT5663_STO1_ADC_L1_SRC_SHIFT: c_int = 13;

pub const RT5663_STO1_ADC_L2_SRC_SHIFT: c_int = 12;

pub const RT5663_STO1_ADC_L_SRC_SHIFT: c_int = 10;

pub const RT5663_M_STO1_ADC_R1_SHIFT: c_int = 7;

pub const RT5663_M_STO1_ADC_R2_SHIFT: c_int = 6;

pub const RT5663_STO1_ADC_R1_SRC_SHIFT: c_int = 5;

pub const RT5663_STO1_ADC_R2_SRC_SHIFT: c_int = 4;

pub const RT5663_STO1_ADC_R_SRC_SHIFT: c_int = 2;
// ADC Mixer to DAC Mixer Control (0x0029)

pub const RT5663_M_ADCMIX_L_SHIFT: c_int = 15;

pub const RT5663_M_DAC1_L_SHIFT: c_int = 14;

pub const RT5663_M_ADCMIX_R_SHIFT: c_int = 7;

pub const RT5663_M_DAC1_R_SHIFT: c_int = 6;
// Stereo DAC Mixer Control (0x002a)

pub const RT5663_M_DAC_L1_STO_L_SHIFT: c_int = 15;

pub const RT5663_M_DAC_R1_STO_L_SHIFT: c_int = 13;

pub const RT5663_M_DAC_L1_STO_R_SHIFT: c_int = 7;

pub const RT5663_M_DAC_R1_STO_R_SHIFT: c_int = 5;
// Power Management for Digital 1 (0x0061)

pub const RT5663_PWR_I2S1_SHIFT: c_int = 15;

pub const RT5663_PWR_DAC_L1_SHIFT: c_int = 11;

pub const RT5663_PWR_DAC_R1_SHIFT: c_int = 10;

pub const RT5663_PWR_LDO_DACREF_SHIFT: c_int = 8;

pub const RT5663_PWR_LDO_SHIFT: c_int = 8;

pub const RT5663_PWR_ADC_L1_SHIFT: c_int = 4;

pub const RT5663_PWR_ADC_R1_SHIFT: c_int = 3;
// Power Management for Digital 2 (0x0062)

pub const RT5663_PWR_ADC_S1F_SHIFT: c_int = 15;

pub const RT5663_PWR_DAC_S1F_SHIFT: c_int = 10;
// Power Management for Analog 1 (0x0063)

pub const RT5663_PWR_VREF1_SHIFT: c_int = 15;

pub const RT5663_PWR_FV1_SHIFT: c_int = 14;

pub const RT5663_PWR_VREF2_SHIFT: c_int = 13;

pub const RT5663_PWR_FV2_SHIFT: c_int = 12;

pub const RT5663_PWR_MB_SHIFT: c_int = 9;

pub const RT5663_AMP_HP_SHIFT: c_int = 2;

pub const RT5663_LDO1_DVO_SHIFT: c_int = 0;

// Power Management for Analog 2 (0x0064)

pub const RT5663_PWR_BST1_SHIFT: c_int = 15;

pub const RT5663_PWR_BST2_SHIFT: c_int = 14;

pub const RT5663_PWR_MB1_SHIFT: c_int = 11;

pub const RT5663_PWR_MB2_SHIFT: c_int = 10;

pub const RT5663_PWR_BST2_OP_SHIFT: c_int = 6;

pub const RT5663_PWR_JD1_SHIFT: c_int = 3;

pub const RT5663_PWR_JD2_SHIFT: c_int = 2;

pub const RT5663_PWR_RECMIX1_SHIFT: c_int = 1;

pub const RT5663_PWR_RECMIX2_SHIFT: c_int = 0;
// Power Management for Analog 3 (0x0065)

pub const RT5663_PWR_CBJ_SHIFT: c_int = 9;

pub const RT5663_PWR_PLL_SHIFT: c_int = 6;

pub const RT5663_PWR_LDO2_SHIFT: c_int = 2;
// Power Management for Volume (0x0067)

pub const RT5663_V2_PWR_MIC_DET_SHIFT: c_int = 5;
// MCLK and System Clock Detection Control (0x006b)

pub const RT5663_EN_ANA_CLK_DET_SHIFT: c_int = 15;

pub const RT5663_PWR_CLK_DET_SHIFT: c_int = 0;

// I2S1 Audio Serial Data Port Control (0x0070)

pub const RT5663_I2S_MS_SHIFT: c_int = 15;

pub const RT5663_I2S_BP_SHIFT: c_int = 8;

pub const RT5663_I2S_DL_SHIFT: c_int = 4;

pub const RT5663_I2S_DF_SHIFT: c_int = 0;

// ADC/DAC Clock Control 1 (0x0073)

pub const RT5663_I2S_PD1_SHIFT: c_int = 12;

pub const RT5663_M_I2S_DIV_SHIFT: c_int = 8;

pub const RT5663_DAC_OSR_SHIFT: c_int = 2;

pub const RT5663_ADC_OSR_SHIFT: c_int = 0;

// TDM1 control 1 (0x0078)

pub const RT5663_TDM_MODE_SHIFT: c_int = 15;

pub const RT5663_TDM_IN_CH_SHIFT: c_int = 10;

pub const RT5663_TDM_OUT_CH_SHIFT: c_int = 8;

pub const RT5663_TDM_IN_LEN_SHIFT: c_int = 6;

pub const RT5663_TDM_OUT_LEN_SHIFT: c_int = 4;

// Global Clock Control (0x0080)

pub const RT5663_SCLK_SRC_SHIFT: c_int = 14;

pub const RT5663_PLL1_SRC_SHIFT: c_int = 11;

pub const RT5663_V2_PLL1_SRC_SHIFT: c_int = 8;

pub const RT5663_PLL1_PD_SHIFT: c_int = 4;
pub const RT5663_PLL_INP_MAX: c_int = 40000000;
pub const RT5663_PLL_INP_MIN: c_int = 256000;
// PLL M/N/K Code Control 1 (0x0081)
pub const RT5663_PLL_N_MAX: c_uint = 0x001ff;

pub const RT5663_PLL_N_SHIFT: c_int = 7;
pub const RT5663_PLL_K_MAX: c_uint = 0x001f;

pub const RT5663_PLL_K_SHIFT: c_int = 0;
// PLL M/N/K Code Control 2 (0x0082)
pub const RT5663_PLL_M_MAX: c_uint = 0x00f;

pub const RT5663_PLL_M_SHIFT: c_int = 12;

pub const RT5663_PLL_M_BP_SHIFT: c_int = 11;
// PLL tracking mode 1 (0x0083)

pub const RT5663_V2_I2S1_ASRC_SHIFT: c_int = 13;

pub const RT5663_V2_DAC_STO1_ASRC_SHIFT: c_int = 12;

pub const RT5663_V2_ADC_STO1_ASRC_SHIFT: c_int = 4;
// PLL tracking mode 2 (0x0084)

pub const RT5663_DA_STO1_TRACK_SHIFT: c_int = 12;

// PLL tracking mode 3 (0x0085)

pub const RT5663_V2_AD_STO1_TRACK_SHIFT: c_int = 12;

// HPOUT Charge pump control 1 (0x0091)

pub const RT5663_OSW_HP_L_SHIFT: c_int = 11;

pub const RT5663_OSW_HP_R_SHIFT: c_int = 10;

pub const RT5663_SEL_PM_HP_SHIFT: c_int = 8;

pub const RT5663_OVCD_HP_SHIFT: c_int = 2;

// RC Clock Control (0x0094)

pub const RT5663_DIG_25M_CLK_SHIFT: c_int = 9;

pub const RT5663_DIG_1M_CLK_SHIFT: c_int = 8;

// Auto Turn On 1M RC CLK (0x009f)

pub const RT5663_IRQ_POW_SAV_SHIFT: c_int = 15;

pub const RT5663_IRQ_POW_SAV_JD1_SHIFT: c_int = 14;

pub const RT5663_IRQ_MANUAL_SHIFT: c_int = 8;

// IRQ Control 1 (0x00b6)

pub const RT5663_EN_CB_JD_SHIFT: c_int = 3;

// IRQ Control 3 (0x00b8)

pub const RT5663_V2_EN_IRQ_INLINE_SHIFT: c_int = 6;

// GPIO Control 1 (0x00c0)

pub const RT5663_GP1_PIN_SHIFT: c_int = 15;

// GPIO Control 2 (0x00c1)

pub const RT5663_GP4_PIN_CONF_SHIFT: c_int = 5;

// GPIO Control 2 (0x00c2)

pub const RT5663_GP8_PIN_CONF_SHIFT: c_int = 13;

// 4 Buttons Inline Command Function 1 (0x00df)

pub const RT5663_4BTN_CLK_DEB_SHIFT: c_int = 2;

// Inline Command Function 6 (0x00e0)

pub const RT5663_EN_4BTN_INL_SHIFT: c_int = 15;

pub const RT5663_RESET_4BTN_INL_SHIFT: c_int = 14;

// Digital Misc Control (0x00fa)
pub const RT5663_DIG_GATE_CTRL_MASK: c_uint = 0x1;

pub const RT5663_DIG_GATE_CTRL_DIS: c_uint = 0x0;
pub const RT5663_DIG_GATE_CTRL_EN: c_uint = 0x1;
// Chopper and Clock control for DAC L (0x013a)

pub const RT5663_CKXEN_DAC1_SHIFT: c_int = 13;

pub const RT5663_CKGEN_DAC1_SHIFT: c_int = 12;
// Chopper and Clock control for ADC (0x013b)

pub const RT5663_CKXEN_ADCC_SHIFT: c_int = 13;

pub const RT5663_CKGEN_ADCC_SHIFT: c_int = 12;
// HP Behavior Logic Control 2 (0x01db)

pub const RT5663_HP_SIG_SRC1_SHIFT: c_int = 0;

// RT5663 specific register
pub const RT5663_HP_OUT_EN: c_uint = 0x0002;
pub const RT5663_HP_LCH_DRE: c_uint = 0x0005;
pub const RT5663_HP_RCH_DRE: c_uint = 0x0006;
pub const RT5663_CALIB_BST: c_uint = 0x000a;
pub const RT5663_RECMIX: c_uint = 0x0010;
pub const RT5663_SIL_DET_CTL: c_uint = 0x0015;
pub const RT5663_PWR_SAV_SILDET: c_uint = 0x0016;
pub const RT5663_SIDETONE_CTL: c_uint = 0x0018;
pub const RT5663_STO1_DAC_DIG_VOL: c_uint = 0x0019;
pub const RT5663_STO1_ADC_DIG_VOL: c_uint = 0x001c;
pub const RT5663_STO1_BOOST: c_uint = 0x001f;
pub const RT5663_HP_IMP_GAIN_1: c_uint = 0x0022;
pub const RT5663_HP_IMP_GAIN_2: c_uint = 0x0023;
pub const RT5663_STO1_ADC_MIXER: c_uint = 0x0026;
pub const RT5663_AD_DA_MIXER: c_uint = 0x0029;
pub const RT5663_STO_DAC_MIXER: c_uint = 0x002a;
pub const RT5663_DIG_SIDE_MIXER: c_uint = 0x002c;
pub const RT5663_BYPASS_STO_DAC: c_uint = 0x002d;
pub const RT5663_CALIB_REC_MIX: c_uint = 0x0040;
pub const RT5663_PWR_DIG_1: c_uint = 0x0061;
pub const RT5663_PWR_DIG_2: c_uint = 0x0062;
pub const RT5663_PWR_ANLG_1: c_uint = 0x0063;
pub const RT5663_PWR_ANLG_2: c_uint = 0x0064;
pub const RT5663_PWR_ANLG_3: c_uint = 0x0065;
pub const RT5663_PWR_MIXER: c_uint = 0x0066;
pub const RT5663_SIG_CLK_DET: c_uint = 0x006b;
pub const RT5663_PRE_DIV_GATING_1: c_uint = 0x006e;
pub const RT5663_PRE_DIV_GATING_2: c_uint = 0x006f;
pub const RT5663_I2S1_SDP: c_uint = 0x0070;
pub const RT5663_ADDA_CLK_1: c_uint = 0x0073;
pub const RT5663_ADDA_RST: c_uint = 0x0074;
pub const RT5663_FRAC_DIV_1: c_uint = 0x0075;
pub const RT5663_FRAC_DIV_2: c_uint = 0x0076;
pub const RT5663_TDM_1: c_uint = 0x0077;
pub const RT5663_TDM_2: c_uint = 0x0078;
pub const RT5663_TDM_3: c_uint = 0x0079;
pub const RT5663_TDM_4: c_uint = 0x007a;
pub const RT5663_TDM_5: c_uint = 0x007b;
pub const RT5663_TDM_6: c_uint = 0x007c;
pub const RT5663_TDM_7: c_uint = 0x007d;
pub const RT5663_TDM_8: c_uint = 0x007e;
pub const RT5663_TDM_9: c_uint = 0x007f;
pub const RT5663_GLB_CLK: c_uint = 0x0080;
pub const RT5663_PLL_1: c_uint = 0x0081;
pub const RT5663_PLL_2: c_uint = 0x0082;
pub const RT5663_ASRC_1: c_uint = 0x0083;
pub const RT5663_ASRC_2: c_uint = 0x0084;
pub const RT5663_ASRC_4: c_uint = 0x0086;
pub const RT5663_DUMMY_REG: c_uint = 0x0087;
pub const RT5663_ASRC_8: c_uint = 0x008a;
pub const RT5663_ASRC_9: c_uint = 0x008b;
pub const RT5663_ASRC_11: c_uint = 0x008c;
pub const RT5663_DEPOP_1: c_uint = 0x008e;
pub const RT5663_DEPOP_2: c_uint = 0x008f;
pub const RT5663_DEPOP_3: c_uint = 0x0090;
pub const RT5663_HP_CHARGE_PUMP_1: c_uint = 0x0091;
pub const RT5663_HP_CHARGE_PUMP_2: c_uint = 0x0092;
pub const RT5663_MICBIAS_1: c_uint = 0x0093;
pub const RT5663_RC_CLK: c_uint = 0x0094;
pub const RT5663_ASRC_11_2: c_uint = 0x0097;
pub const RT5663_DUMMY_REG_2: c_uint = 0x0098;
pub const RT5663_REC_PATH_GAIN: c_uint = 0x009a;
pub const RT5663_AUTO_1MRC_CLK: c_uint = 0x009f;
pub const RT5663_ADC_EQ_1: c_uint = 0x00ae;
pub const RT5663_ADC_EQ_2: c_uint = 0x00af;
pub const RT5663_IRQ_1: c_uint = 0x00b6;
pub const RT5663_IRQ_2: c_uint = 0x00b7;
pub const RT5663_IRQ_3: c_uint = 0x00b8;
pub const RT5663_IRQ_4: c_uint = 0x00ba;
pub const RT5663_IRQ_5: c_uint = 0x00bb;
pub const RT5663_INT_ST_1: c_uint = 0x00be;
pub const RT5663_INT_ST_2: c_uint = 0x00bf;
pub const RT5663_GPIO_1: c_uint = 0x00c0;
pub const RT5663_GPIO_2: c_uint = 0x00c1;
pub const RT5663_GPIO_STA1: c_uint = 0x00c5;
pub const RT5663_SIN_GEN_1: c_uint = 0x00cb;
pub const RT5663_SIN_GEN_2: c_uint = 0x00cc;
pub const RT5663_SIN_GEN_3: c_uint = 0x00cd;
pub const RT5663_SOF_VOL_ZC1: c_uint = 0x00d9;
pub const RT5663_IL_CMD_1: c_uint = 0x00db;
pub const RT5663_IL_CMD_2: c_uint = 0x00dc;
pub const RT5663_IL_CMD_3: c_uint = 0x00dd;
pub const RT5663_IL_CMD_4: c_uint = 0x00de;
pub const RT5663_IL_CMD_5: c_uint = 0x00df;
pub const RT5663_IL_CMD_6: c_uint = 0x00e0;
pub const RT5663_IL_CMD_7: c_uint = 0x00e1;
pub const RT5663_IL_CMD_8: c_uint = 0x00e2;
pub const RT5663_IL_CMD_PWRSAV1: c_uint = 0x00e4;
pub const RT5663_IL_CMD_PWRSAV2: c_uint = 0x00e5;
pub const RT5663_EM_JACK_TYPE_1: c_uint = 0x00e6;
pub const RT5663_EM_JACK_TYPE_2: c_uint = 0x00e7;
pub const RT5663_EM_JACK_TYPE_3: c_uint = 0x00e8;
pub const RT5663_EM_JACK_TYPE_4: c_uint = 0x00e9;
pub const RT5663_EM_JACK_TYPE_5: c_uint = 0x00ea;
pub const RT5663_EM_JACK_TYPE_6: c_uint = 0x00eb;
pub const RT5663_STO1_HPF_ADJ1: c_uint = 0x00ec;
pub const RT5663_STO1_HPF_ADJ2: c_uint = 0x00ed;
pub const RT5663_FAST_OFF_MICBIAS: c_uint = 0x00f4;
pub const RT5663_JD_CTRL1: c_uint = 0x00f6;
pub const RT5663_JD_CTRL2: c_uint = 0x00f8;
pub const RT5663_DIG_MISC: c_uint = 0x00fa;
pub const RT5663_DIG_VOL_ZCD: c_uint = 0x0100;
pub const RT5663_ANA_BIAS_CUR_1: c_uint = 0x0108;
pub const RT5663_ANA_BIAS_CUR_2: c_uint = 0x0109;
pub const RT5663_ANA_BIAS_CUR_3: c_uint = 0x010a;
pub const RT5663_ANA_BIAS_CUR_4: c_uint = 0x010b;
pub const RT5663_ANA_BIAS_CUR_5: c_uint = 0x010c;
pub const RT5663_ANA_BIAS_CUR_6: c_uint = 0x010d;
pub const RT5663_BIAS_CUR_5: c_uint = 0x010e;
pub const RT5663_BIAS_CUR_6: c_uint = 0x010f;
pub const RT5663_BIAS_CUR_7: c_uint = 0x0110;
pub const RT5663_BIAS_CUR_8: c_uint = 0x0111;
pub const RT5663_DACREF_LDO: c_uint = 0x0112;
pub const RT5663_DUMMY_REG_3: c_uint = 0x0113;
pub const RT5663_BIAS_CUR_9: c_uint = 0x0114;
pub const RT5663_DUMMY_REG_4: c_uint = 0x0116;
pub const RT5663_VREFADJ_OP: c_uint = 0x0117;
pub const RT5663_VREF_RECMIX: c_uint = 0x0118;
pub const RT5663_CHARGE_PUMP_1: c_uint = 0x0125;
pub const RT5663_CHARGE_PUMP_1_2: c_uint = 0x0126;
pub const RT5663_CHARGE_PUMP_1_3: c_uint = 0x0127;
pub const RT5663_CHARGE_PUMP_2: c_uint = 0x0128;
pub const RT5663_DIG_IN_PIN1: c_uint = 0x0132;
pub const RT5663_PAD_DRV_CTL: c_uint = 0x0137;
pub const RT5663_PLL_INT_REG: c_uint = 0x0139;
pub const RT5663_CHOP_DAC_L: c_uint = 0x013a;
pub const RT5663_CHOP_ADC: c_uint = 0x013b;
pub const RT5663_CALIB_ADC: c_uint = 0x013c;
pub const RT5663_CHOP_DAC_R: c_uint = 0x013d;
pub const RT5663_DUMMY_CTL_DACLR: c_uint = 0x013e;
pub const RT5663_DUMMY_REG_5: c_uint = 0x0140;
pub const RT5663_SOFT_RAMP: c_uint = 0x0141;
pub const RT5663_TEST_MODE_1: c_uint = 0x0144;
pub const RT5663_TEST_MODE_2: c_uint = 0x0145;
pub const RT5663_TEST_MODE_3: c_uint = 0x0146;
pub const RT5663_TEST_MODE_4: c_uint = 0x0147;
pub const RT5663_TEST_MODE_5: c_uint = 0x0148;
pub const RT5663_STO_DRE_1: c_uint = 0x0160;
pub const RT5663_STO_DRE_2: c_uint = 0x0161;
pub const RT5663_STO_DRE_3: c_uint = 0x0162;
pub const RT5663_STO_DRE_4: c_uint = 0x0163;
pub const RT5663_STO_DRE_5: c_uint = 0x0164;
pub const RT5663_STO_DRE_6: c_uint = 0x0165;
pub const RT5663_STO_DRE_7: c_uint = 0x0166;
pub const RT5663_STO_DRE_8: c_uint = 0x0167;
pub const RT5663_STO_DRE_9: c_uint = 0x0168;
pub const RT5663_STO_DRE_10: c_uint = 0x0169;
pub const RT5663_MIC_DECRO_1: c_uint = 0x0180;
pub const RT5663_MIC_DECRO_2: c_uint = 0x0181;
pub const RT5663_MIC_DECRO_3: c_uint = 0x0182;
pub const RT5663_MIC_DECRO_4: c_uint = 0x0183;
pub const RT5663_MIC_DECRO_5: c_uint = 0x0184;
pub const RT5663_MIC_DECRO_6: c_uint = 0x0185;
pub const RT5663_HP_DECRO_1: c_uint = 0x01b0;
pub const RT5663_HP_DECRO_2: c_uint = 0x01b1;
pub const RT5663_HP_DECRO_3: c_uint = 0x01b2;
pub const RT5663_HP_DECRO_4: c_uint = 0x01b3;
pub const RT5663_HP_DECOUP: c_uint = 0x01b4;
pub const RT5663_HP_IMP_SEN_MAP8: c_uint = 0x01b5;
pub const RT5663_HP_IMP_SEN_MAP9: c_uint = 0x01b6;
pub const RT5663_HP_IMP_SEN_MAP10: c_uint = 0x01b7;
pub const RT5663_HP_IMP_SEN_MAP11: c_uint = 0x01b8;
pub const RT5663_HP_IMP_SEN_1: c_uint = 0x01c0;
pub const RT5663_HP_IMP_SEN_2: c_uint = 0x01c1;
pub const RT5663_HP_IMP_SEN_3: c_uint = 0x01c2;
pub const RT5663_HP_IMP_SEN_4: c_uint = 0x01c3;
pub const RT5663_HP_IMP_SEN_5: c_uint = 0x01c4;
pub const RT5663_HP_IMP_SEN_6: c_uint = 0x01c5;
pub const RT5663_HP_IMP_SEN_7: c_uint = 0x01c6;
pub const RT5663_HP_IMP_SEN_8: c_uint = 0x01c7;
pub const RT5663_HP_IMP_SEN_9: c_uint = 0x01c8;
pub const RT5663_HP_IMP_SEN_10: c_uint = 0x01c9;
pub const RT5663_HP_IMP_SEN_11: c_uint = 0x01ca;
pub const RT5663_HP_IMP_SEN_12: c_uint = 0x01cb;
pub const RT5663_HP_IMP_SEN_13: c_uint = 0x01cc;
pub const RT5663_HP_IMP_SEN_14: c_uint = 0x01cd;
pub const RT5663_HP_IMP_SEN_15: c_uint = 0x01ce;
pub const RT5663_HP_IMP_SEN_16: c_uint = 0x01cf;
pub const RT5663_HP_IMP_SEN_17: c_uint = 0x01d0;
pub const RT5663_HP_IMP_SEN_18: c_uint = 0x01d1;
pub const RT5663_HP_IMP_SEN_19: c_uint = 0x01d2;
pub const RT5663_HP_IMPSEN_DIG5: c_uint = 0x01d3;
pub const RT5663_HP_IMPSEN_MAP1: c_uint = 0x01d4;
pub const RT5663_HP_IMPSEN_MAP2: c_uint = 0x01d5;
pub const RT5663_HP_IMPSEN_MAP3: c_uint = 0x01d6;
pub const RT5663_HP_IMPSEN_MAP4: c_uint = 0x01d7;
pub const RT5663_HP_IMPSEN_MAP5: c_uint = 0x01d8;
pub const RT5663_HP_IMPSEN_MAP7: c_uint = 0x01d9;
pub const RT5663_HP_LOGIC_1: c_uint = 0x01da;
pub const RT5663_HP_LOGIC_2: c_uint = 0x01db;
pub const RT5663_HP_CALIB_1: c_uint = 0x01dd;
pub const RT5663_HP_CALIB_1_1: c_uint = 0x01de;
pub const RT5663_HP_CALIB_2: c_uint = 0x01df;
pub const RT5663_HP_CALIB_3: c_uint = 0x01e0;
pub const RT5663_HP_CALIB_4: c_uint = 0x01e1;
pub const RT5663_HP_CALIB_5: c_uint = 0x01e2;
pub const RT5663_HP_CALIB_5_1: c_uint = 0x01e3;
pub const RT5663_HP_CALIB_6: c_uint = 0x01e4;
pub const RT5663_HP_CALIB_7: c_uint = 0x01e5;
pub const RT5663_HP_CALIB_9: c_uint = 0x01e6;
pub const RT5663_HP_CALIB_10: c_uint = 0x01e7;
pub const RT5663_HP_CALIB_11: c_uint = 0x01e8;
pub const RT5663_HP_CALIB_ST1: c_uint = 0x01ea;
pub const RT5663_HP_CALIB_ST2: c_uint = 0x01eb;
pub const RT5663_HP_CALIB_ST3: c_uint = 0x01ec;
pub const RT5663_HP_CALIB_ST4: c_uint = 0x01ed;
pub const RT5663_HP_CALIB_ST5: c_uint = 0x01ee;
pub const RT5663_HP_CALIB_ST6: c_uint = 0x01ef;
pub const RT5663_HP_CALIB_ST7: c_uint = 0x01f0;
pub const RT5663_HP_CALIB_ST8: c_uint = 0x01f1;
pub const RT5663_HP_CALIB_ST9: c_uint = 0x01f2;
pub const RT5663_HP_AMP_DET: c_uint = 0x0200;
pub const RT5663_DUMMY_REG_6: c_uint = 0x0201;
pub const RT5663_HP_BIAS: c_uint = 0x0202;
pub const RT5663_CBJ_1: c_uint = 0x0250;
pub const RT5663_CBJ_2: c_uint = 0x0251;
pub const RT5663_CBJ_3: c_uint = 0x0252;
pub const RT5663_DUMMY_1: c_uint = 0x02fa;
pub const RT5663_DUMMY_2: c_uint = 0x02fb;
pub const RT5663_DUMMY_3: c_uint = 0x02fc;
pub const RT5663_ANA_JD: c_uint = 0x0300;
pub const RT5663_ADC_LCH_LPF1_A1: c_uint = 0x03d0;
pub const RT5663_ADC_RCH_LPF1_A1: c_uint = 0x03d1;
pub const RT5663_ADC_LCH_LPF1_H0: c_uint = 0x03d2;
pub const RT5663_ADC_RCH_LPF1_H0: c_uint = 0x03d3;
pub const RT5663_ADC_LCH_BPF1_A1: c_uint = 0x03d4;
pub const RT5663_ADC_RCH_BPF1_A1: c_uint = 0x03d5;
pub const RT5663_ADC_LCH_BPF1_A2: c_uint = 0x03d6;
pub const RT5663_ADC_RCH_BPF1_A2: c_uint = 0x03d7;
pub const RT5663_ADC_LCH_BPF1_H0: c_uint = 0x03d8;
pub const RT5663_ADC_RCH_BPF1_H0: c_uint = 0x03d9;
pub const RT5663_ADC_LCH_BPF2_A1: c_uint = 0x03da;
pub const RT5663_ADC_RCH_BPF2_A1: c_uint = 0x03db;
pub const RT5663_ADC_LCH_BPF2_A2: c_uint = 0x03dc;
pub const RT5663_ADC_RCH_BPF2_A2: c_uint = 0x03dd;
pub const RT5663_ADC_LCH_BPF2_H0: c_uint = 0x03de;
pub const RT5663_ADC_RCH_BPF2_H0: c_uint = 0x03df;
pub const RT5663_ADC_LCH_BPF3_A1: c_uint = 0x03e0;
pub const RT5663_ADC_RCH_BPF3_A1: c_uint = 0x03e1;
pub const RT5663_ADC_LCH_BPF3_A2: c_uint = 0x03e2;
pub const RT5663_ADC_RCH_BPF3_A2: c_uint = 0x03e3;
pub const RT5663_ADC_LCH_BPF3_H0: c_uint = 0x03e4;
pub const RT5663_ADC_RCH_BPF3_H0: c_uint = 0x03e5;
pub const RT5663_ADC_LCH_BPF4_A1: c_uint = 0x03e6;
pub const RT5663_ADC_RCH_BPF4_A1: c_uint = 0x03e7;
pub const RT5663_ADC_LCH_BPF4_A2: c_uint = 0x03e8;
pub const RT5663_ADC_RCH_BPF4_A2: c_uint = 0x03e9;
pub const RT5663_ADC_LCH_BPF4_H0: c_uint = 0x03ea;
pub const RT5663_ADC_RCH_BPF4_H0: c_uint = 0x03eb;
pub const RT5663_ADC_LCH_HPF1_A1: c_uint = 0x03ec;
pub const RT5663_ADC_RCH_HPF1_A1: c_uint = 0x03ed;
pub const RT5663_ADC_LCH_HPF1_H0: c_uint = 0x03ee;
pub const RT5663_ADC_RCH_HPF1_H0: c_uint = 0x03ef;
pub const RT5663_ADC_EQ_PRE_VOL_L: c_uint = 0x03f0;
pub const RT5663_ADC_EQ_PRE_VOL_R: c_uint = 0x03f1;
pub const RT5663_ADC_EQ_POST_VOL_L: c_uint = 0x03f2;
pub const RT5663_ADC_EQ_POST_VOL_R: c_uint = 0x03f3;
// RECMIX Control (0x0010)

pub const RT5663_RECMIX1_BST1_SHIFT: c_int = 0;

// Bypass Stereo1 DAC Mixer Control (0x002d)

pub const RT5663_DACL1_SRC_SHIFT: c_int = 3;

pub const RT5663_DACR1_SRC_SHIFT: c_int = 2;
// TDM control 2 (0x0078)

pub const RT5663_DATA_SWAP_ADCDAT1_SHIFT: c_int = 14;

// TDM control 5 (0x007b)

pub const RT5663_TDM_LENGTN_SHIFT: c_int = 0;

// PLL tracking mode 1 (0x0083)

pub const RT5663_I2S1_ASRC_SHIFT: c_int = 11;

pub const RT5663_DAC_STO1_ASRC_SHIFT: c_int = 10;

pub const RT5663_ADC_STO1_ASRC_SHIFT: c_int = 3;
// PLL tracking mode 2 (0x0084)

pub const RT5663_DA_STO1_TRACK_SHIFT: c_int = 12;

pub const RT5663_AD_STO1_TRACK_SHIFT: c_int = 0;

// HPOUT Charge pump control 1 (0x0091)

pub const RT5663_SI_HP_SHIFT: c_int = 12;

// GPIO Control 2 (0x00b6)

pub const RT5663_GP1_PIN_CONF_SHIFT: c_int = 2;

// GPIO Control 2 (0x00b7)

pub const RT5663_EN_IRQ_INLINE_SHIFT: c_int = 3;

// GPIO Control 1 (0x00c0)

pub const RT5663_GPIO1_TYPE_SHIFT: c_int = 15;

// IRQ Control 1 (0x00c1)

pub const RT5663_EN_IRQ_JD1_SHIFT: c_int = 6;

pub const RT5663_SEL_GPIO1_SHIFT: c_int = 6;

// Inline Command Function 2 (0x00dc)

pub const RT5663_PWR_MIC_DET_SHIFT: c_int = 0;

// Embeeded Jack and Type Detection Control 1 (0x00e6)

pub const RT5663_CBJ_DET_SHIFT: c_int = 15;

pub const RT5663_EXT_JD_SHIFT: c_int = 11;

pub const RT5663_POL_EXT_JD_SHIFT: c_int = 10;

pub const RT5663_EM_JD_SHIFT: c_int = 7;

// DACREF LDO Control (0x0112)

pub const RT5663_PWR_LDO_DACREFL_SHIFT: c_int = 9;

pub const RT5663_PWR_LDO_DACREFR_SHIFT: c_int = 1;
// Stereo Dynamic Range Enhancement Control 9 (0x0168, 0x0169)

pub const RT5663_DRE_GAIN_HP_SHIFT: c_int = 0;
// Combo Jack Control (0x0250)

pub const RT5663_INBUF_CBJ_BST1_SHIFT: c_int = 11;

pub const RT5663_CBJ_SENSE_BST1_SHIFT: c_int = 10;

// Combo Jack Control (0x0251)

pub const RT5663_GAIN_BST1_SHIFT: c_int = 0;
// Dummy register 1 (0x02fa)

pub const RT5663_EMB_CLK_SHIFT: c_int = 9;

pub const RT5663_HPA_CPL_BIAS_SHIFT: c_int = 6;

pub const RT5663_HPA_CPR_BIAS_SHIFT: c_int = 3;

pub const RT5663_DUMMY_BIAS_SHIFT: c_int = 0;

// System Clock Source
// PLL1 Source
// asrc clock source
// filter mask
