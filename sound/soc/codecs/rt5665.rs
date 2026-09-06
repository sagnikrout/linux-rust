//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5665.h
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
// rt5665.h  --  RT5665/RT5658 ALSA SoC audio driver
//
// Copyright 2016 Realtek Microelectronics
// Author: Bard Liao <bardliao@realtek.com>
//

pub const DEVICE_ID: c_uint = 0x6451;
// Info
pub const RT5665_RESET: c_uint = 0x0000;
pub const RT5665_VENDOR_ID: c_uint = 0x00fd;
pub const RT5665_VENDOR_ID_1: c_uint = 0x00fe;
pub const RT5665_DEVICE_ID: c_uint = 0x00ff;
// I/O - Output
pub const RT5665_LOUT: c_uint = 0x0001;
pub const RT5665_HP_CTRL_1: c_uint = 0x0002;
pub const RT5665_HP_CTRL_2: c_uint = 0x0003;
pub const RT5665_MONO_OUT: c_uint = 0x0004;
pub const RT5665_HPL_GAIN: c_uint = 0x0005;
pub const RT5665_HPR_GAIN: c_uint = 0x0006;
pub const RT5665_MONO_GAIN: c_uint = 0x0007;
// I/O - Input
pub const RT5665_CAL_BST_CTRL: c_uint = 0x000a;
pub const RT5665_CBJ_BST_CTRL: c_uint = 0x000b;
pub const RT5665_IN1_IN2: c_uint = 0x000c;
pub const RT5665_IN3_IN4: c_uint = 0x000d;
pub const RT5665_INL1_INR1_VOL: c_uint = 0x000f;
// I/O - Speaker
pub const RT5665_EJD_CTRL_1: c_uint = 0x0010;
pub const RT5665_EJD_CTRL_2: c_uint = 0x0011;
pub const RT5665_EJD_CTRL_3: c_uint = 0x0012;
pub const RT5665_EJD_CTRL_4: c_uint = 0x0013;
pub const RT5665_EJD_CTRL_5: c_uint = 0x0014;
pub const RT5665_EJD_CTRL_6: c_uint = 0x0015;
pub const RT5665_EJD_CTRL_7: c_uint = 0x0016;
// I/O - ADC/DAC/DMIC
pub const RT5665_DAC2_CTRL: c_uint = 0x0017;
pub const RT5665_DAC2_DIG_VOL: c_uint = 0x0018;
pub const RT5665_DAC1_DIG_VOL: c_uint = 0x0019;
pub const RT5665_DAC3_DIG_VOL: c_uint = 0x001a;
pub const RT5665_DAC3_CTRL: c_uint = 0x001b;
pub const RT5665_STO1_ADC_DIG_VOL: c_uint = 0x001c;
pub const RT5665_MONO_ADC_DIG_VOL: c_uint = 0x001d;
pub const RT5665_STO2_ADC_DIG_VOL: c_uint = 0x001e;
pub const RT5665_STO1_ADC_BOOST: c_uint = 0x001f;
pub const RT5665_MONO_ADC_BOOST: c_uint = 0x0020;
pub const RT5665_STO2_ADC_BOOST: c_uint = 0x0021;
pub const RT5665_HP_IMP_GAIN_1: c_uint = 0x0022;
pub const RT5665_HP_IMP_GAIN_2: c_uint = 0x0023;
// Mixer - D-D
pub const RT5665_STO1_ADC_MIXER: c_uint = 0x0026;
pub const RT5665_MONO_ADC_MIXER: c_uint = 0x0027;
pub const RT5665_STO2_ADC_MIXER: c_uint = 0x0028;
pub const RT5665_AD_DA_MIXER: c_uint = 0x0029;
pub const RT5665_STO1_DAC_MIXER: c_uint = 0x002a;
pub const RT5665_MONO_DAC_MIXER: c_uint = 0x002b;
pub const RT5665_STO2_DAC_MIXER: c_uint = 0x002c;
pub const RT5665_A_DAC1_MUX: c_uint = 0x002d;
pub const RT5665_A_DAC2_MUX: c_uint = 0x002e;
pub const RT5665_DIG_INF2_DATA: c_uint = 0x002f;
pub const RT5665_DIG_INF3_DATA: c_uint = 0x0030;
// Mixer - PDM
pub const RT5665_PDM_OUT_CTRL: c_uint = 0x0031;
pub const RT5665_PDM_DATA_CTRL_1: c_uint = 0x0032;
pub const RT5665_PDM_DATA_CTRL_2: c_uint = 0x0033;
pub const RT5665_PDM_DATA_CTRL_3: c_uint = 0x0034;
pub const RT5665_PDM_DATA_CTRL_4: c_uint = 0x0035;
// Mixer - ADC
pub const RT5665_REC1_GAIN: c_uint = 0x003a;
pub const RT5665_REC1_L1_MIXER: c_uint = 0x003b;
pub const RT5665_REC1_L2_MIXER: c_uint = 0x003c;
pub const RT5665_REC1_R1_MIXER: c_uint = 0x003d;
pub const RT5665_REC1_R2_MIXER: c_uint = 0x003e;
pub const RT5665_REC2_GAIN: c_uint = 0x003f;
pub const RT5665_REC2_L1_MIXER: c_uint = 0x0040;
pub const RT5665_REC2_L2_MIXER: c_uint = 0x0041;
pub const RT5665_REC2_R1_MIXER: c_uint = 0x0042;
pub const RT5665_REC2_R2_MIXER: c_uint = 0x0043;
pub const RT5665_CAL_REC: c_uint = 0x0044;
// Mixer - DAC
pub const RT5665_ALC_BACK_GAIN: c_uint = 0x0049;
pub const RT5665_MONOMIX_GAIN: c_uint = 0x004a;
pub const RT5665_MONOMIX_IN_GAIN: c_uint = 0x004b;
pub const RT5665_OUT_L_GAIN: c_uint = 0x004d;
pub const RT5665_OUT_L_MIXER: c_uint = 0x004e;
pub const RT5665_OUT_R_GAIN: c_uint = 0x004f;
pub const RT5665_OUT_R_MIXER: c_uint = 0x0050;
pub const RT5665_LOUT_MIXER: c_uint = 0x0052;
// Power
pub const RT5665_PWR_DIG_1: c_uint = 0x0061;
pub const RT5665_PWR_DIG_2: c_uint = 0x0062;
pub const RT5665_PWR_ANLG_1: c_uint = 0x0063;
pub const RT5665_PWR_ANLG_2: c_uint = 0x0064;
pub const RT5665_PWR_ANLG_3: c_uint = 0x0065;
pub const RT5665_PWR_MIXER: c_uint = 0x0066;
pub const RT5665_PWR_VOL: c_uint = 0x0067;
// Clock Detect
pub const RT5665_CLK_DET: c_uint = 0x006b;
// Filter
pub const RT5665_HPF_CTRL1: c_uint = 0x006d;
// DMIC
pub const RT5665_DMIC_CTRL_1: c_uint = 0x006e;
pub const RT5665_DMIC_CTRL_2: c_uint = 0x006f;
// Format - ADC/DAC
pub const RT5665_I2S1_SDP: c_uint = 0x0070;
pub const RT5665_I2S2_SDP: c_uint = 0x0071;
pub const RT5665_I2S3_SDP: c_uint = 0x0072;
pub const RT5665_ADDA_CLK_1: c_uint = 0x0073;
pub const RT5665_ADDA_CLK_2: c_uint = 0x0074;
pub const RT5665_I2S1_F_DIV_CTRL_1: c_uint = 0x0075;
pub const RT5665_I2S1_F_DIV_CTRL_2: c_uint = 0x0076;
// Format - TDM Control
pub const RT5665_TDM_CTRL_1: c_uint = 0x0078;
pub const RT5665_TDM_CTRL_2: c_uint = 0x0079;
pub const RT5665_TDM_CTRL_3: c_uint = 0x007a;
pub const RT5665_TDM_CTRL_4: c_uint = 0x007b;
pub const RT5665_TDM_CTRL_5: c_uint = 0x007c;
pub const RT5665_TDM_CTRL_6: c_uint = 0x007d;
pub const RT5665_TDM_CTRL_7: c_uint = 0x007e;
pub const RT5665_TDM_CTRL_8: c_uint = 0x007f;
// Function - Analog
pub const RT5665_GLB_CLK: c_uint = 0x0080;
pub const RT5665_PLL_CTRL_1: c_uint = 0x0081;
pub const RT5665_PLL_CTRL_2: c_uint = 0x0082;
pub const RT5665_ASRC_1: c_uint = 0x0083;
pub const RT5665_ASRC_2: c_uint = 0x0084;
pub const RT5665_ASRC_3: c_uint = 0x0085;
pub const RT5665_ASRC_4: c_uint = 0x0086;
pub const RT5665_ASRC_5: c_uint = 0x0087;
pub const RT5665_ASRC_6: c_uint = 0x0088;
pub const RT5665_ASRC_7: c_uint = 0x0089;
pub const RT5665_ASRC_8: c_uint = 0x008a;
pub const RT5665_ASRC_9: c_uint = 0x008b;
pub const RT5665_ASRC_10: c_uint = 0x008c;
pub const RT5665_DEPOP_1: c_uint = 0x008e;
pub const RT5665_DEPOP_2: c_uint = 0x008f;
pub const RT5665_HP_CHARGE_PUMP_1: c_uint = 0x0091;
pub const RT5665_HP_CHARGE_PUMP_2: c_uint = 0x0092;
pub const RT5665_MICBIAS_1: c_uint = 0x0093;
pub const RT5665_MICBIAS_2: c_uint = 0x0094;
pub const RT5665_ASRC_12: c_uint = 0x0098;
pub const RT5665_ASRC_13: c_uint = 0x0099;
pub const RT5665_ASRC_14: c_uint = 0x009a;
pub const RT5665_RC_CLK_CTRL: c_uint = 0x009f;
pub const RT5665_I2S_M_CLK_CTRL_1: c_uint = 0x00a0;
pub const RT5665_I2S2_F_DIV_CTRL_1: c_uint = 0x00a1;
pub const RT5665_I2S2_F_DIV_CTRL_2: c_uint = 0x00a2;
pub const RT5665_I2S3_F_DIV_CTRL_1: c_uint = 0x00a3;
pub const RT5665_I2S3_F_DIV_CTRL_2: c_uint = 0x00a4;
// Function - Digital
pub const RT5665_EQ_CTRL_1: c_uint = 0x00ae;
pub const RT5665_EQ_CTRL_2: c_uint = 0x00af;
pub const RT5665_IRQ_CTRL_1: c_uint = 0x00b6;
pub const RT5665_IRQ_CTRL_2: c_uint = 0x00b7;
pub const RT5665_IRQ_CTRL_3: c_uint = 0x00b8;
pub const RT5665_IRQ_CTRL_4: c_uint = 0x00b9;
pub const RT5665_IRQ_CTRL_5: c_uint = 0x00ba;
pub const RT5665_IRQ_CTRL_6: c_uint = 0x00bb;
pub const RT5665_INT_ST_1: c_uint = 0x00be;
pub const RT5665_GPIO_CTRL_1: c_uint = 0x00c0;
pub const RT5665_GPIO_CTRL_2: c_uint = 0x00c1;
pub const RT5665_GPIO_CTRL_3: c_uint = 0x00c2;
pub const RT5665_GPIO_CTRL_4: c_uint = 0x00c3;
pub const RT5665_GPIO_STA: c_uint = 0x00c4;
pub const RT5665_HP_AMP_DET_CTRL_1: c_uint = 0x00d0;
pub const RT5665_HP_AMP_DET_CTRL_2: c_uint = 0x00d1;
pub const RT5665_MID_HP_AMP_DET: c_uint = 0x00d3;
pub const RT5665_LOW_HP_AMP_DET: c_uint = 0x00d4;
pub const RT5665_SV_ZCD_1: c_uint = 0x00d9;
pub const RT5665_SV_ZCD_2: c_uint = 0x00da;
pub const RT5665_IL_CMD_1: c_uint = 0x00db;
pub const RT5665_IL_CMD_2: c_uint = 0x00dc;
pub const RT5665_IL_CMD_3: c_uint = 0x00dd;
pub const RT5665_IL_CMD_4: c_uint = 0x00de;
pub const RT5665_4BTN_IL_CMD_1: c_uint = 0x00df;
pub const RT5665_4BTN_IL_CMD_2: c_uint = 0x00e0;
pub const RT5665_4BTN_IL_CMD_3: c_uint = 0x00e1;
pub const RT5665_PSV_IL_CMD_1: c_uint = 0x00e2;
pub const RT5665_ADC_STO1_HP_CTRL_1: c_uint = 0x00ea;
pub const RT5665_ADC_STO1_HP_CTRL_2: c_uint = 0x00eb;
pub const RT5665_ADC_MONO_HP_CTRL_1: c_uint = 0x00ec;
pub const RT5665_ADC_MONO_HP_CTRL_2: c_uint = 0x00ed;
pub const RT5665_ADC_STO2_HP_CTRL_1: c_uint = 0x00ee;
pub const RT5665_ADC_STO2_HP_CTRL_2: c_uint = 0x00ef;
pub const RT5665_AJD1_CTRL: c_uint = 0x00f0;
pub const RT5665_JD1_THD: c_uint = 0x00f1;
pub const RT5665_JD2_THD: c_uint = 0x00f2;
pub const RT5665_JD_CTRL_1: c_uint = 0x00f6;
pub const RT5665_JD_CTRL_2: c_uint = 0x00f7;
pub const RT5665_JD_CTRL_3: c_uint = 0x00f8;
// General Control
pub const RT5665_DIG_MISC: c_uint = 0x00fa;
pub const RT5665_DUMMY_2: c_uint = 0x00fb;
pub const RT5665_DUMMY_3: c_uint = 0x00fc;
pub const RT5665_DAC_ADC_DIG_VOL1: c_uint = 0x0100;
pub const RT5665_DAC_ADC_DIG_VOL2: c_uint = 0x0101;
pub const RT5665_BIAS_CUR_CTRL_1: c_uint = 0x010a;
pub const RT5665_BIAS_CUR_CTRL_2: c_uint = 0x010b;
pub const RT5665_BIAS_CUR_CTRL_3: c_uint = 0x010c;
pub const RT5665_BIAS_CUR_CTRL_4: c_uint = 0x010d;
pub const RT5665_BIAS_CUR_CTRL_5: c_uint = 0x010e;
pub const RT5665_BIAS_CUR_CTRL_6: c_uint = 0x010f;
pub const RT5665_BIAS_CUR_CTRL_7: c_uint = 0x0110;
pub const RT5665_BIAS_CUR_CTRL_8: c_uint = 0x0111;
pub const RT5665_BIAS_CUR_CTRL_9: c_uint = 0x0112;
pub const RT5665_BIAS_CUR_CTRL_10: c_uint = 0x0113;
pub const RT5665_VREF_REC_OP_FB_CAP_CTRL: c_uint = 0x0117;
pub const RT5665_CHARGE_PUMP_1: c_uint = 0x0125;
pub const RT5665_DIG_IN_CTRL_1: c_uint = 0x0132;
pub const RT5665_DIG_IN_CTRL_2: c_uint = 0x0133;
pub const RT5665_PAD_DRIVING_CTRL: c_uint = 0x0137;
pub const RT5665_SOFT_RAMP_DEPOP: c_uint = 0x0138;
pub const RT5665_PLL: c_uint = 0x0139;
pub const RT5665_CHOP_DAC: c_uint = 0x013a;
pub const RT5665_CHOP_ADC: c_uint = 0x013b;
pub const RT5665_CALIB_ADC_CTRL: c_uint = 0x013c;
pub const RT5665_VOL_TEST: c_uint = 0x013f;
pub const RT5665_TEST_MODE_CTRL_1: c_uint = 0x0145;
pub const RT5665_TEST_MODE_CTRL_2: c_uint = 0x0146;
pub const RT5665_TEST_MODE_CTRL_3: c_uint = 0x0147;
pub const RT5665_TEST_MODE_CTRL_4: c_uint = 0x0148;
pub const RT5665_BASSBACK_CTRL: c_uint = 0x0150;
pub const RT5665_STO_NG2_CTRL_1: c_uint = 0x0160;
pub const RT5665_STO_NG2_CTRL_2: c_uint = 0x0161;
pub const RT5665_STO_NG2_CTRL_3: c_uint = 0x0162;
pub const RT5665_STO_NG2_CTRL_4: c_uint = 0x0163;
pub const RT5665_STO_NG2_CTRL_5: c_uint = 0x0164;
pub const RT5665_STO_NG2_CTRL_6: c_uint = 0x0165;
pub const RT5665_STO_NG2_CTRL_7: c_uint = 0x0166;
pub const RT5665_STO_NG2_CTRL_8: c_uint = 0x0167;
pub const RT5665_MONO_NG2_CTRL_1: c_uint = 0x0170;
pub const RT5665_MONO_NG2_CTRL_2: c_uint = 0x0171;
pub const RT5665_MONO_NG2_CTRL_3: c_uint = 0x0172;
pub const RT5665_MONO_NG2_CTRL_4: c_uint = 0x0173;
pub const RT5665_MONO_NG2_CTRL_5: c_uint = 0x0174;
pub const RT5665_MONO_NG2_CTRL_6: c_uint = 0x0175;
pub const RT5665_STO1_DAC_SIL_DET: c_uint = 0x0190;
pub const RT5665_MONOL_DAC_SIL_DET: c_uint = 0x0191;
pub const RT5665_MONOR_DAC_SIL_DET: c_uint = 0x0192;
pub const RT5665_STO2_DAC_SIL_DET: c_uint = 0x0193;
pub const RT5665_SIL_PSV_CTRL1: c_uint = 0x0194;
pub const RT5665_SIL_PSV_CTRL2: c_uint = 0x0195;
pub const RT5665_SIL_PSV_CTRL3: c_uint = 0x0196;
pub const RT5665_SIL_PSV_CTRL4: c_uint = 0x0197;
pub const RT5665_SIL_PSV_CTRL5: c_uint = 0x0198;
pub const RT5665_SIL_PSV_CTRL6: c_uint = 0x0199;
pub const RT5665_MONO_AMP_CALIB_CTRL_1: c_uint = 0x01a0;
pub const RT5665_MONO_AMP_CALIB_CTRL_2: c_uint = 0x01a1;
pub const RT5665_MONO_AMP_CALIB_CTRL_3: c_uint = 0x01a2;
pub const RT5665_MONO_AMP_CALIB_CTRL_4: c_uint = 0x01a3;
pub const RT5665_MONO_AMP_CALIB_CTRL_5: c_uint = 0x01a4;
pub const RT5665_MONO_AMP_CALIB_CTRL_6: c_uint = 0x01a5;
pub const RT5665_MONO_AMP_CALIB_CTRL_7: c_uint = 0x01a6;
pub const RT5665_MONO_AMP_CALIB_STA1: c_uint = 0x01a7;
pub const RT5665_MONO_AMP_CALIB_STA2: c_uint = 0x01a8;
pub const RT5665_MONO_AMP_CALIB_STA3: c_uint = 0x01a9;
pub const RT5665_MONO_AMP_CALIB_STA4: c_uint = 0x01aa;
pub const RT5665_MONO_AMP_CALIB_STA6: c_uint = 0x01ab;
pub const RT5665_HP_IMP_SENS_CTRL_01: c_uint = 0x01b5;
pub const RT5665_HP_IMP_SENS_CTRL_02: c_uint = 0x01b6;
pub const RT5665_HP_IMP_SENS_CTRL_03: c_uint = 0x01b7;
pub const RT5665_HP_IMP_SENS_CTRL_04: c_uint = 0x01b8;
pub const RT5665_HP_IMP_SENS_CTRL_05: c_uint = 0x01b9;
pub const RT5665_HP_IMP_SENS_CTRL_06: c_uint = 0x01ba;
pub const RT5665_HP_IMP_SENS_CTRL_07: c_uint = 0x01bb;
pub const RT5665_HP_IMP_SENS_CTRL_08: c_uint = 0x01bc;
pub const RT5665_HP_IMP_SENS_CTRL_09: c_uint = 0x01bd;
pub const RT5665_HP_IMP_SENS_CTRL_10: c_uint = 0x01be;
pub const RT5665_HP_IMP_SENS_CTRL_11: c_uint = 0x01bf;
pub const RT5665_HP_IMP_SENS_CTRL_12: c_uint = 0x01c0;
pub const RT5665_HP_IMP_SENS_CTRL_13: c_uint = 0x01c1;
pub const RT5665_HP_IMP_SENS_CTRL_14: c_uint = 0x01c2;
pub const RT5665_HP_IMP_SENS_CTRL_15: c_uint = 0x01c3;
pub const RT5665_HP_IMP_SENS_CTRL_16: c_uint = 0x01c4;
pub const RT5665_HP_IMP_SENS_CTRL_17: c_uint = 0x01c5;
pub const RT5665_HP_IMP_SENS_CTRL_18: c_uint = 0x01c6;
pub const RT5665_HP_IMP_SENS_CTRL_19: c_uint = 0x01c7;
pub const RT5665_HP_IMP_SENS_CTRL_20: c_uint = 0x01c8;
pub const RT5665_HP_IMP_SENS_CTRL_21: c_uint = 0x01c9;
pub const RT5665_HP_IMP_SENS_CTRL_22: c_uint = 0x01ca;
pub const RT5665_HP_IMP_SENS_CTRL_23: c_uint = 0x01cb;
pub const RT5665_HP_IMP_SENS_CTRL_24: c_uint = 0x01cc;
pub const RT5665_HP_IMP_SENS_CTRL_25: c_uint = 0x01cd;
pub const RT5665_HP_IMP_SENS_CTRL_26: c_uint = 0x01ce;
pub const RT5665_HP_IMP_SENS_CTRL_27: c_uint = 0x01cf;
pub const RT5665_HP_IMP_SENS_CTRL_28: c_uint = 0x01d0;
pub const RT5665_HP_IMP_SENS_CTRL_29: c_uint = 0x01d1;
pub const RT5665_HP_IMP_SENS_CTRL_30: c_uint = 0x01d2;
pub const RT5665_HP_IMP_SENS_CTRL_31: c_uint = 0x01d3;
pub const RT5665_HP_IMP_SENS_CTRL_32: c_uint = 0x01d4;
pub const RT5665_HP_IMP_SENS_CTRL_33: c_uint = 0x01d5;
pub const RT5665_HP_IMP_SENS_CTRL_34: c_uint = 0x01d6;
pub const RT5665_HP_LOGIC_CTRL_1: c_uint = 0x01da;
pub const RT5665_HP_LOGIC_CTRL_2: c_uint = 0x01db;
pub const RT5665_HP_LOGIC_CTRL_3: c_uint = 0x01dc;
pub const RT5665_HP_CALIB_CTRL_1: c_uint = 0x01de;
pub const RT5665_HP_CALIB_CTRL_2: c_uint = 0x01df;
pub const RT5665_HP_CALIB_CTRL_3: c_uint = 0x01e0;
pub const RT5665_HP_CALIB_CTRL_4: c_uint = 0x01e1;
pub const RT5665_HP_CALIB_CTRL_5: c_uint = 0x01e2;
pub const RT5665_HP_CALIB_CTRL_6: c_uint = 0x01e3;
pub const RT5665_HP_CALIB_CTRL_7: c_uint = 0x01e4;
pub const RT5665_HP_CALIB_CTRL_9: c_uint = 0x01e6;
pub const RT5665_HP_CALIB_CTRL_10: c_uint = 0x01e7;
pub const RT5665_HP_CALIB_CTRL_11: c_uint = 0x01e8;
pub const RT5665_HP_CALIB_STA_1: c_uint = 0x01ea;
pub const RT5665_HP_CALIB_STA_2: c_uint = 0x01eb;
pub const RT5665_HP_CALIB_STA_3: c_uint = 0x01ec;
pub const RT5665_HP_CALIB_STA_4: c_uint = 0x01ed;
pub const RT5665_HP_CALIB_STA_5: c_uint = 0x01ee;
pub const RT5665_HP_CALIB_STA_6: c_uint = 0x01ef;
pub const RT5665_HP_CALIB_STA_7: c_uint = 0x01f0;
pub const RT5665_HP_CALIB_STA_8: c_uint = 0x01f1;
pub const RT5665_HP_CALIB_STA_9: c_uint = 0x01f2;
pub const RT5665_HP_CALIB_STA_10: c_uint = 0x01f3;
pub const RT5665_HP_CALIB_STA_11: c_uint = 0x01f4;
pub const RT5665_PGM_TAB_CTRL1: c_uint = 0x0200;
pub const RT5665_PGM_TAB_CTRL2: c_uint = 0x0201;
pub const RT5665_PGM_TAB_CTRL3: c_uint = 0x0202;
pub const RT5665_PGM_TAB_CTRL4: c_uint = 0x0203;
pub const RT5665_PGM_TAB_CTRL5: c_uint = 0x0204;
pub const RT5665_PGM_TAB_CTRL6: c_uint = 0x0205;
pub const RT5665_PGM_TAB_CTRL7: c_uint = 0x0206;
pub const RT5665_PGM_TAB_CTRL8: c_uint = 0x0207;
pub const RT5665_PGM_TAB_CTRL9: c_uint = 0x0208;
pub const RT5665_SAR_IL_CMD_1: c_uint = 0x0210;
pub const RT5665_SAR_IL_CMD_2: c_uint = 0x0211;
pub const RT5665_SAR_IL_CMD_3: c_uint = 0x0212;
pub const RT5665_SAR_IL_CMD_4: c_uint = 0x0213;
pub const RT5665_SAR_IL_CMD_5: c_uint = 0x0214;
pub const RT5665_SAR_IL_CMD_6: c_uint = 0x0215;
pub const RT5665_SAR_IL_CMD_7: c_uint = 0x0216;
pub const RT5665_SAR_IL_CMD_8: c_uint = 0x0217;
pub const RT5665_SAR_IL_CMD_9: c_uint = 0x0218;
pub const RT5665_SAR_IL_CMD_10: c_uint = 0x0219;
pub const RT5665_SAR_IL_CMD_11: c_uint = 0x021a;
pub const RT5665_SAR_IL_CMD_12: c_uint = 0x021b;
pub const RT5665_DRC1_CTRL_0: c_uint = 0x02ff;
pub const RT5665_DRC1_CTRL_1: c_uint = 0x0300;
pub const RT5665_DRC1_CTRL_2: c_uint = 0x0301;
pub const RT5665_DRC1_CTRL_3: c_uint = 0x0302;
pub const RT5665_DRC1_CTRL_4: c_uint = 0x0303;
pub const RT5665_DRC1_CTRL_5: c_uint = 0x0304;
pub const RT5665_DRC1_CTRL_6: c_uint = 0x0305;
pub const RT5665_DRC1_HARD_LMT_CTRL_1: c_uint = 0x0306;
pub const RT5665_DRC1_HARD_LMT_CTRL_2: c_uint = 0x0307;
pub const RT5665_DRC1_PRIV_1: c_uint = 0x0310;
pub const RT5665_DRC1_PRIV_2: c_uint = 0x0311;
pub const RT5665_DRC1_PRIV_3: c_uint = 0x0312;
pub const RT5665_DRC1_PRIV_4: c_uint = 0x0313;
pub const RT5665_DRC1_PRIV_5: c_uint = 0x0314;
pub const RT5665_DRC1_PRIV_6: c_uint = 0x0315;
pub const RT5665_DRC1_PRIV_7: c_uint = 0x0316;
pub const RT5665_DRC1_PRIV_8: c_uint = 0x0317;
pub const RT5665_ALC_PGA_CTRL_1: c_uint = 0x0330;
pub const RT5665_ALC_PGA_CTRL_2: c_uint = 0x0331;
pub const RT5665_ALC_PGA_CTRL_3: c_uint = 0x0332;
pub const RT5665_ALC_PGA_CTRL_4: c_uint = 0x0333;
pub const RT5665_ALC_PGA_CTRL_5: c_uint = 0x0334;
pub const RT5665_ALC_PGA_CTRL_6: c_uint = 0x0335;
pub const RT5665_ALC_PGA_CTRL_7: c_uint = 0x0336;
pub const RT5665_ALC_PGA_CTRL_8: c_uint = 0x0337;
pub const RT5665_ALC_PGA_STA_1: c_uint = 0x0338;
pub const RT5665_ALC_PGA_STA_2: c_uint = 0x0339;
pub const RT5665_ALC_PGA_STA_3: c_uint = 0x033a;
pub const RT5665_EQ_AUTO_RCV_CTRL1: c_uint = 0x03c0;
pub const RT5665_EQ_AUTO_RCV_CTRL2: c_uint = 0x03c1;
pub const RT5665_EQ_AUTO_RCV_CTRL3: c_uint = 0x03c2;
pub const RT5665_EQ_AUTO_RCV_CTRL4: c_uint = 0x03c3;
pub const RT5665_EQ_AUTO_RCV_CTRL5: c_uint = 0x03c4;
pub const RT5665_EQ_AUTO_RCV_CTRL6: c_uint = 0x03c5;
pub const RT5665_EQ_AUTO_RCV_CTRL7: c_uint = 0x03c6;
pub const RT5665_EQ_AUTO_RCV_CTRL8: c_uint = 0x03c7;
pub const RT5665_EQ_AUTO_RCV_CTRL9: c_uint = 0x03c8;
pub const RT5665_EQ_AUTO_RCV_CTRL10: c_uint = 0x03c9;
pub const RT5665_EQ_AUTO_RCV_CTRL11: c_uint = 0x03ca;
pub const RT5665_EQ_AUTO_RCV_CTRL12: c_uint = 0x03cb;
pub const RT5665_EQ_AUTO_RCV_CTRL13: c_uint = 0x03cc;
pub const RT5665_ADC_L_EQ_LPF1_A1: c_uint = 0x03d0;
pub const RT5665_R_EQ_LPF1_A1: c_uint = 0x03d1;
pub const RT5665_L_EQ_LPF1_H0: c_uint = 0x03d2;
pub const RT5665_R_EQ_LPF1_H0: c_uint = 0x03d3;
pub const RT5665_L_EQ_BPF1_A1: c_uint = 0x03d4;
pub const RT5665_R_EQ_BPF1_A1: c_uint = 0x03d5;
pub const RT5665_L_EQ_BPF1_A2: c_uint = 0x03d6;
pub const RT5665_R_EQ_BPF1_A2: c_uint = 0x03d7;
pub const RT5665_L_EQ_BPF1_H0: c_uint = 0x03d8;
pub const RT5665_R_EQ_BPF1_H0: c_uint = 0x03d9;
pub const RT5665_L_EQ_BPF2_A1: c_uint = 0x03da;
pub const RT5665_R_EQ_BPF2_A1: c_uint = 0x03db;
pub const RT5665_L_EQ_BPF2_A2: c_uint = 0x03dc;
pub const RT5665_R_EQ_BPF2_A2: c_uint = 0x03dd;
pub const RT5665_L_EQ_BPF2_H0: c_uint = 0x03de;
pub const RT5665_R_EQ_BPF2_H0: c_uint = 0x03df;
pub const RT5665_L_EQ_BPF3_A1: c_uint = 0x03e0;
pub const RT5665_R_EQ_BPF3_A1: c_uint = 0x03e1;
pub const RT5665_L_EQ_BPF3_A2: c_uint = 0x03e2;
pub const RT5665_R_EQ_BPF3_A2: c_uint = 0x03e3;
pub const RT5665_L_EQ_BPF3_H0: c_uint = 0x03e4;
pub const RT5665_R_EQ_BPF3_H0: c_uint = 0x03e5;
pub const RT5665_L_EQ_BPF4_A1: c_uint = 0x03e6;
pub const RT5665_R_EQ_BPF4_A1: c_uint = 0x03e7;
pub const RT5665_L_EQ_BPF4_A2: c_uint = 0x03e8;
pub const RT5665_R_EQ_BPF4_A2: c_uint = 0x03e9;
pub const RT5665_L_EQ_BPF4_H0: c_uint = 0x03ea;
pub const RT5665_R_EQ_BPF4_H0: c_uint = 0x03eb;
pub const RT5665_L_EQ_HPF1_A1: c_uint = 0x03ec;
pub const RT5665_R_EQ_HPF1_A1: c_uint = 0x03ed;
pub const RT5665_L_EQ_HPF1_H0: c_uint = 0x03ee;
pub const RT5665_R_EQ_HPF1_H0: c_uint = 0x03ef;
pub const RT5665_L_EQ_PRE_VOL: c_uint = 0x03f0;
pub const RT5665_R_EQ_PRE_VOL: c_uint = 0x03f1;
pub const RT5665_L_EQ_POST_VOL: c_uint = 0x03f2;
pub const RT5665_R_EQ_POST_VOL: c_uint = 0x03f3;
pub const RT5665_SCAN_MODE_CTRL: c_uint = 0x07f0;
pub const RT5665_I2C_MODE: c_uint = 0x07fa;
// global definition

pub const RT5665_L_MUTE_SFT: c_int = 15;

pub const RT5665_VOL_L_SFT: c_int = 14;

pub const RT5665_R_MUTE_SFT: c_int = 7;

pub const RT5665_VOL_R_SFT: c_int = 6;

pub const RT5665_L_VOL_SFT: c_int = 8;

pub const RT5665_R_VOL_SFT: c_int = 0;
// Headphone Amp L/R Analog Gain and Digital NG2 Gain Control (0x0005 0x0006)

pub const RT5665_G_HP_SFT: c_int = 8;

pub const RT5665_G_STO_DA_SFT: c_int = 0;
// CBJ Control (0x000b)

pub const RT5665_BST_CBJ_SFT: c_int = 8;
// IN1/IN2 Control (0x000c)

pub const RT5665_IN1_DF: c_int = 15;

pub const RT5665_BST1_SFT: c_int = 8;

pub const RT5665_IN2_DF: c_int = 7;

pub const RT5665_BST2_SFT: c_int = 0;
// IN3/IN4 Control (0x000d)

pub const RT5665_IN3_DF: c_int = 15;

pub const RT5665_BST3_SFT: c_int = 8;

pub const RT5665_IN4_DF: c_int = 7;

pub const RT5665_BST4_SFT: c_int = 0;
// INL and INR Volume Control (0x000f)

pub const RT5665_INL_VOL_SFT: c_int = 8;

pub const RT5665_INR_VOL_SFT: c_int = 0;
// Embeeded Jack and Type Detection Control 1 (0x0010)

pub const RT5665_EMB_JD_EN_SFT: c_int = 15;

pub const RT5665_JD_MODE_SFT: c_int = 13;

// Embeeded Jack and Type Detection Control 2 (0x0011)

pub const RT5665_EXT_JD_SRC_SFT: c_int = 4;

// Combo Jack and Type Detection Control 4 (0x0013)

// Slience Detection Control (0x0015)

// DAC2 Control (0x0017)

pub const RT5665_M_DAC2_L_VOL_SFT: c_int = 13;

pub const RT5665_M_DAC2_R_VOL_SFT: c_int = 12;

pub const RT5665_DAC_L2_SEL_SFT: c_int = 4;

pub const RT5665_DAC_R2_SEL_SFT: c_int = 0;
// Sidetone Control (0x0018)

pub const RT5665_ST_SEL_SFT: c_int = 9;

pub const RT5665_ST_EN_SFT: c_int = 6;
// DAC1 Digital Volume (0x0019)

pub const RT5665_DAC_L1_VOL_SFT: c_int = 8;

pub const RT5665_DAC_R1_VOL_SFT: c_int = 0;
// DAC2 Digital Volume (0x001a)

pub const RT5665_DAC_L2_VOL_SFT: c_int = 8;

pub const RT5665_DAC_R2_VOL_SFT: c_int = 0;
// DAC3 Control (0x001b)

pub const RT5665_M_DAC3_L_VOL_SFT: c_int = 13;

pub const RT5665_M_DAC3_R_VOL_SFT: c_int = 12;

pub const RT5665_DAC_L3_SEL_SFT: c_int = 4;

pub const RT5665_DAC_R3_SEL_SFT: c_int = 0;
// ADC Digital Volume Control (0x001c)

pub const RT5665_ADC_L_VOL_SFT: c_int = 8;

pub const RT5665_ADC_R_VOL_SFT: c_int = 0;
// Mono ADC Digital Volume Control (0x001d)

pub const RT5665_MONO_ADC_L_VOL_SFT: c_int = 8;

pub const RT5665_MONO_ADC_R_VOL_SFT: c_int = 0;
// Stereo1 ADC Boost Gain Control (0x001f)

pub const RT5665_STO1_ADC_L_BST_SFT: c_int = 14;

pub const RT5665_STO1_ADC_R_BST_SFT: c_int = 12;
// Mono ADC Boost Gain Control (0x0020)

pub const RT5665_MONO_ADC_L_BST_SFT: c_int = 14;

pub const RT5665_MONO_ADC_R_BST_SFT: c_int = 12;
// Stereo1 ADC Boost Gain Control (0x001f)

pub const RT5665_STO2_ADC_L_BST_SFT: c_int = 14;

pub const RT5665_STO2_ADC_R_BST_SFT: c_int = 12;
// Stereo1 ADC Mixer Control (0x0026)

pub const RT5665_M_STO1_ADC_L1_SFT: c_int = 15;

pub const RT5665_M_STO1_ADC_L2_SFT: c_int = 14;

pub const RT5665_STO1_ADC1L_SRC_SFT: c_int = 13;

pub const RT5665_STO1_ADC2L_SRC_SFT: c_int = 12;

pub const RT5665_STO1_ADCL_SRC_SFT: c_int = 10;

pub const RT5665_STO1_DD_L_SRC_SFT: c_int = 9;

pub const RT5665_STO1_DMIC_SRC_SFT: c_int = 8;

pub const RT5665_M_STO1_ADC_R1_SFT: c_int = 7;

pub const RT5665_M_STO1_ADC_R2_SFT: c_int = 6;

pub const RT5665_STO1_ADC1R_SRC_SFT: c_int = 5;

pub const RT5665_STO1_ADC2R_SRC_SFT: c_int = 4;

pub const RT5665_STO1_ADCR_SRC_SFT: c_int = 2;

pub const RT5665_STO1_DD_R_SRC_SFT: c_int = 0;
// Mono1 ADC Mixer control (0x0027)

pub const RT5665_M_MONO_ADC_L1_SFT: c_int = 15;

pub const RT5665_M_MONO_ADC_L2_SFT: c_int = 14;

pub const RT5665_MONO_ADC_L1_SRC_SFT: c_int = 13;

pub const RT5665_MONO_ADC_L2_SRC_SFT: c_int = 12;

pub const RT5665_MONO_ADC_L_SRC_SFT: c_int = 10;

pub const RT5665_MONO_DD_L_SRC_SFT: c_int = 9;

pub const RT5665_MONO_DMIC_L_SRC_SFT: c_int = 8;

pub const RT5665_M_MONO_ADC_R1_SFT: c_int = 7;

pub const RT5665_M_MONO_ADC_R2_SFT: c_int = 6;

pub const RT5665_MONO_ADC_R1_SRC_SFT: c_int = 5;

pub const RT5665_MONO_ADC_R2_SRC_SFT: c_int = 4;

pub const RT5665_MONO_ADC_R_SRC_SFT: c_int = 2;

pub const RT5665_MONO_DD_R_SRC_SFT: c_int = 1;
pub const RT5665_MONO_DMIC_R_SRC_MASK: c_uint = 0x1;
pub const RT5665_MONO_DMIC_R_SRC_SFT: c_int = 0;
// Stereo2 ADC Mixer Control (0x0028)

pub const RT5665_M_STO2_ADC_L1_SFT: c_int = 15;

pub const RT5665_M_STO2_ADC_L2_SFT: c_int = 14;

pub const RT5665_STO2_ADC1L_SRC_SFT: c_int = 13;

pub const RT5665_STO2_ADC2L_SRC_SFT: c_int = 12;

pub const RT5665_STO2_ADCL_SRC_SFT: c_int = 10;

pub const RT5665_STO2_DD_L_SRC_SFT: c_int = 9;

pub const RT5665_STO2_DMIC_SRC_SFT: c_int = 8;

pub const RT5665_M_STO2_ADC_R1_SFT: c_int = 7;

pub const RT5665_M_STO2_ADC_R2_SFT: c_int = 6;

pub const RT5665_STO2_ADC1R_SRC_SFT: c_int = 5;

pub const RT5665_STO2_ADC2R_SRC_SFT: c_int = 4;

pub const RT5665_STO2_ADCR_SRC_SFT: c_int = 2;

pub const RT5665_STO2_DD_R_SRC_SFT: c_int = 1;
// ADC Mixer to DAC Mixer Control (0x0029)

pub const RT5665_M_ADCMIX_L_SFT: c_int = 15;

pub const RT5665_M_DAC1_L_SFT: c_int = 14;

pub const RT5665_DAC1_R_SEL_SFT: c_int = 10;

pub const RT5665_DAC1_L_SEL_SFT: c_int = 8;

pub const RT5665_M_ADCMIX_R_SFT: c_int = 7;

pub const RT5665_M_DAC1_R_SFT: c_int = 6;
// Stereo1 DAC Mixer Control (0x002a)

pub const RT5665_M_DAC_L1_STO_L_SFT: c_int = 15;

pub const RT5665_G_DAC_L1_STO_L_SFT: c_int = 14;

pub const RT5665_M_DAC_R1_STO_L_SFT: c_int = 13;

pub const RT5665_G_DAC_R1_STO_L_SFT: c_int = 12;

pub const RT5665_M_DAC_L2_STO_L_SFT: c_int = 11;

pub const RT5665_G_DAC_L2_STO_L_SFT: c_int = 10;

pub const RT5665_M_DAC_R2_STO_L_SFT: c_int = 9;

pub const RT5665_G_DAC_R2_STO_L_SFT: c_int = 8;

pub const RT5665_M_DAC_L1_STO_R_SFT: c_int = 7;

pub const RT5665_G_DAC_L1_STO_R_SFT: c_int = 6;

pub const RT5665_M_DAC_R1_STO_R_SFT: c_int = 5;

pub const RT5665_G_DAC_R1_STO_R_SFT: c_int = 4;

pub const RT5665_M_DAC_L2_STO_R_SFT: c_int = 3;

pub const RT5665_G_DAC_L2_STO_R_SFT: c_int = 2;

pub const RT5665_M_DAC_R2_STO_R_SFT: c_int = 1;

pub const RT5665_G_DAC_R2_STO_R_SFT: c_int = 0;
// Mono DAC Mixer Control (0x002b)

pub const RT5665_M_DAC_L1_MONO_L_SFT: c_int = 15;

pub const RT5665_G_DAC_L1_MONO_L_SFT: c_int = 14;

pub const RT5665_M_DAC_R1_MONO_L_SFT: c_int = 13;

pub const RT5665_G_DAC_R1_MONO_L_SFT: c_int = 12;

pub const RT5665_M_DAC_L2_MONO_L_SFT: c_int = 11;

pub const RT5665_G_DAC_L2_MONO_L_SFT: c_int = 10;

pub const RT5665_M_DAC_R2_MONO_L_SFT: c_int = 9;

pub const RT5665_G_DAC_R2_MONO_L_SFT: c_int = 8;

pub const RT5665_M_DAC_L1_MONO_R_SFT: c_int = 7;

pub const RT5665_G_DAC_L1_MONO_R_SFT: c_int = 6;

pub const RT5665_M_DAC_R1_MONO_R_SFT: c_int = 5;

pub const RT5665_G_DAC_R1_MONO_R_SFT: c_int = 4;

pub const RT5665_M_DAC_L2_MONO_R_SFT: c_int = 3;

pub const RT5665_G_DAC_L2_MONO_R_SFT: c_int = 2;

pub const RT5665_M_DAC_R2_MONO_R_SFT: c_int = 1;

pub const RT5665_G_DAC_R2_MONO_R_SFT: c_int = 0;
// Stereo2 DAC Mixer Control (0x002c)

pub const RT5665_M_DAC_L1_STO2_L_SFT: c_int = 15;

pub const RT5665_G_DAC_L1_STO2_L_SFT: c_int = 14;

pub const RT5665_M_DAC_L2_STO2_L_SFT: c_int = 13;

pub const RT5665_G_DAC_L2_STO2_L_SFT: c_int = 12;

pub const RT5665_M_DAC_L3_STO2_L_SFT: c_int = 11;

pub const RT5665_G_DAC_L3_STO2_L_SFT: c_int = 10;

pub const RT5665_M_ST_DAC_L1_SFT: c_int = 9;

pub const RT5665_M_ST_DAC_R1_SFT: c_int = 8;

pub const RT5665_M_DAC_R1_STO2_R_SFT: c_int = 7;

pub const RT5665_G_DAC_R1_STO2_R_SFT: c_int = 6;

pub const RT5665_M_DAC_R2_STO2_R_SFT: c_int = 5;

pub const RT5665_G_DAC_R2_STO2_R_SFT: c_int = 4;

pub const RT5665_M_DAC_R3_STO2_R_SFT: c_int = 3;

pub const RT5665_G_DAC_R3_STO2_R_SFT: c_int = 2;
// Analog DAC1 Input Source Control (0x002d)

pub const RT5665_DAC_MIX_L_SFT: c_int = 12;

pub const RT5665_DAC_MIX_R_SFT: c_int = 8;

pub const RT5665_A_DACL1_SFT: c_int = 4;

pub const RT5665_A_DACR1_SFT: c_int = 0;
// Analog DAC Input Source Control (0x002e)

pub const RT5665_A_DACL2_SFT: c_int = 4;

pub const RT5665_A_DACR2_SFT: c_int = 0;
// Digital Interface Data Control (0x002f)

pub const RT5665_IF2_1_ADC_IN_SFT: c_int = 12;

pub const RT5665_IF2_1_DAC_SEL_SFT: c_int = 10;

pub const RT5665_IF2_1_ADC_SEL_SFT: c_int = 8;

pub const RT5665_IF2_2_ADC_IN_SFT: c_int = 4;

pub const RT5665_IF2_2_DAC_SEL_SFT: c_int = 2;

pub const RT5665_IF2_2_ADC_SEL_SFT: c_int = 0;
// Digital Interface Data Control (0x0030)

pub const RT5665_IF3_ADC_IN_SFT: c_int = 4;

pub const RT5665_IF3_DAC_SEL_SFT: c_int = 2;

pub const RT5665_IF3_ADC_SEL_SFT: c_int = 0;
// PDM Output Control (0x0031)

pub const RT5665_M_PDM1_L_SFT: c_int = 14;

pub const RT5665_M_PDM1_R_SFT: c_int = 12;

pub const RT5665_PDM1_L_SFT: c_int = 10;

pub const RT5665_PDM1_R_SFT: c_int = 8;

// S/PDIF Output Control (0x0036)

pub const RT5665_SPDIF_SEL_SFT: c_int = 0;
// REC Left Mixer Control 2 (0x003c)

pub const RT5665_M_CBJ_RM1_L_SFT: c_int = 7;

pub const RT5665_M_BST1_RM1_L_SFT: c_int = 5;

pub const RT5665_M_BST2_RM1_L_SFT: c_int = 4;

pub const RT5665_M_BST3_RM1_L_SFT: c_int = 3;

pub const RT5665_M_BST4_RM1_L_SFT: c_int = 2;

pub const RT5665_M_INL_RM1_L_SFT: c_int = 1;

pub const RT5665_M_INR_RM1_L_SFT: c_int = 0;
// REC Right Mixer Control 2 (0x003e)

pub const RT5665_M_AEC_REF_RM1_R_SFT: c_int = 7;

pub const RT5665_M_BST1_RM1_R_SFT: c_int = 5;

pub const RT5665_M_BST2_RM1_R_SFT: c_int = 4;

pub const RT5665_M_BST3_RM1_R_SFT: c_int = 3;

pub const RT5665_M_BST4_RM1_R_SFT: c_int = 2;

pub const RT5665_M_INR_RM1_R_SFT: c_int = 1;

pub const RT5665_M_MONOVOL_RM1_R_SFT: c_int = 0;
// REC Mixer 2 Left Control 2 (0x0041)

pub const RT5665_M_CBJ_RM2_L_SFT: c_int = 7;

pub const RT5665_M_BST1_RM2_L_SFT: c_int = 5;

pub const RT5665_M_BST2_RM2_L_SFT: c_int = 4;

pub const RT5665_M_BST3_RM2_L_SFT: c_int = 3;

pub const RT5665_M_BST4_RM2_L_SFT: c_int = 2;

pub const RT5665_M_INL_RM2_L_SFT: c_int = 1;

pub const RT5665_M_INR_RM2_L_SFT: c_int = 0;
// REC Mixer 2 Right Control 2 (0x0043)

pub const RT5665_M_MONOVOL_RM2_R_SFT: c_int = 7;

pub const RT5665_M_BST1_RM2_R_SFT: c_int = 5;

pub const RT5665_M_BST2_RM2_R_SFT: c_int = 4;

pub const RT5665_M_BST3_RM2_R_SFT: c_int = 3;

pub const RT5665_M_BST4_RM2_R_SFT: c_int = 2;

pub const RT5665_M_INL_RM2_R_SFT: c_int = 1;

pub const RT5665_M_INR_RM2_R_SFT: c_int = 0;
// SPK Left Mixer Control (0x0046)

pub const RT5665_M_BST3_SM_L_SFT: c_int = 4;

pub const RT5665_M_IN_R_SM_L_SFT: c_int = 3;

pub const RT5665_M_IN_L_SM_L_SFT: c_int = 2;

pub const RT5665_M_BST1_SM_L_SFT: c_int = 1;

pub const RT5665_M_DAC_L2_SM_L_SFT: c_int = 0;
// SPK Right Mixer Control (0x0047)

pub const RT5665_M_BST3_SM_R_SFT: c_int = 4;

pub const RT5665_M_IN_R_SM_R_SFT: c_int = 3;

pub const RT5665_M_IN_L_SM_R_SFT: c_int = 2;

pub const RT5665_M_BST4_SM_R_SFT: c_int = 1;

pub const RT5665_M_DAC_R2_SM_R_SFT: c_int = 0;
// SPO Amp Input and Gain Control (0x0048)

pub const RT5665_M_DAC_L2_SPKOMIX_SFT: c_int = 13;

pub const RT5665_M_SPKVOLL_SPKOMIX_SFT: c_int = 12;

pub const RT5665_M_DAC_R2_SPKOMIX_SFT: c_int = 9;

pub const RT5665_M_SPKVOLR_SPKOMIX_SFT: c_int = 8;
// MONOMIX Input and Gain Control (0x004b)

pub const RT5665_G_MONOVOL_MA_SFT: c_int = 10;

pub const RT5665_M_MONOVOL_MA_SFT: c_int = 9;

pub const RT5665_M_DAC_L2_MA_SFT: c_int = 8;

pub const RT5665_M_BST3_MM_SFT: c_int = 4;

pub const RT5665_M_BST2_MM_SFT: c_int = 3;

pub const RT5665_M_BST1_MM_SFT: c_int = 2;

pub const RT5665_M_RECMIC2L_MM_SFT: c_int = 1;

pub const RT5665_M_DAC_L2_MM_SFT: c_int = 0;
// Output Left Mixer Control 1 (0x004d)

pub const RT5665_G_BST3_OM_L_SFT: c_int = 12;

pub const RT5665_G_BST2_OM_L_SFT: c_int = 9;

pub const RT5665_G_BST1_OM_L_SFT: c_int = 6;

pub const RT5665_G_IN_L_OM_L_SFT: c_int = 3;

pub const RT5665_G_DAC_L2_OM_L_SFT: c_int = 0;
// Output Left Mixer Input Control (0x004e)

pub const RT5665_M_BST3_OM_L_SFT: c_int = 4;

pub const RT5665_M_BST2_OM_L_SFT: c_int = 3;

pub const RT5665_M_BST1_OM_L_SFT: c_int = 2;

pub const RT5665_M_IN_L_OM_L_SFT: c_int = 1;

pub const RT5665_M_DAC_L2_OM_L_SFT: c_int = 0;
// Output Right Mixer Input Control (0x0050)

pub const RT5665_M_BST4_OM_R_SFT: c_int = 4;

pub const RT5665_M_BST3_OM_R_SFT: c_int = 3;

pub const RT5665_M_BST2_OM_R_SFT: c_int = 2;

pub const RT5665_M_IN_R_OM_R_SFT: c_int = 1;

pub const RT5665_M_DAC_R2_OM_R_SFT: c_int = 0;
// LOUT Mixer Control (0x0052)

pub const RT5665_M_DAC_L2_LM_SFT: c_int = 15;

pub const RT5665_M_DAC_R2_LM_SFT: c_int = 14;

pub const RT5665_M_OV_L_LM_SFT: c_int = 13;

pub const RT5665_M_OV_R_LM_SFT: c_int = 12;
pub const RT5665_LOUT_BST_SFT: c_int = 11;

pub const RT5665_LOUT_DF_SFT: c_int = 11;
// Power Management for Digital 1 (0x0061)

pub const RT5665_PWR_I2S1_1_BIT: c_int = 15;

pub const RT5665_PWR_I2S1_2_BIT: c_int = 14;

pub const RT5665_PWR_I2S2_1_BIT: c_int = 13;

pub const RT5665_PWR_I2S2_2_BIT: c_int = 12;

pub const RT5665_PWR_DAC_L1_BIT: c_int = 11;

pub const RT5665_PWR_DAC_R1_BIT: c_int = 10;

pub const RT5665_PWR_I2S3_BIT: c_int = 9;

pub const RT5665_PWR_LDO_BIT: c_int = 8;

pub const RT5665_PWR_DAC_L2_BIT: c_int = 7;

pub const RT5665_PWR_DAC_R2_BIT: c_int = 6;

pub const RT5665_PWR_ADC_L1_BIT: c_int = 4;

pub const RT5665_PWR_ADC_R1_BIT: c_int = 3;

pub const RT5665_PWR_ADC_L2_BIT: c_int = 2;

pub const RT5665_PWR_ADC_R2_BIT: c_int = 1;
// Power Management for Digital 2 (0x0062)

pub const RT5665_PWR_ADC_S1F_BIT: c_int = 15;

pub const RT5665_PWR_ADC_S2F_BIT: c_int = 14;

pub const RT5665_PWR_ADC_MF_L_BIT: c_int = 13;

pub const RT5665_PWR_ADC_MF_R_BIT: c_int = 12;

pub const RT5665_PWR_DAC_S2F_BIT: c_int = 11;

pub const RT5665_PWR_DAC_S1F_BIT: c_int = 10;

pub const RT5665_PWR_DAC_MF_L_BIT: c_int = 9;

pub const RT5665_PWR_DAC_MF_R_BIT: c_int = 8;

pub const RT5665_PWR_PDM1_BIT: c_int = 7;
// Power Management for Analog 1 (0x0063)

pub const RT5665_PWR_VREF1_BIT: c_int = 15;

pub const RT5665_PWR_FV1_BIT: c_int = 14;

pub const RT5665_PWR_VREF2_BIT: c_int = 13;

pub const RT5665_PWR_FV2_BIT: c_int = 12;

pub const RT5665_PWR_VREF3_BIT: c_int = 11;

pub const RT5665_PWR_FV3_BIT: c_int = 10;

pub const RT5665_PWR_MB_BIT: c_int = 9;

pub const RT5665_PWR_LM_BIT: c_int = 8;

pub const RT5665_PWR_BG_BIT: c_int = 7;

pub const RT5665_PWR_MA_BIT: c_int = 6;

pub const RT5665_PWR_HA_L_BIT: c_int = 5;

pub const RT5665_PWR_HA_R_BIT: c_int = 4;

// Power Management for Analog 2 (0x0064)

pub const RT5665_PWR_BST1_BIT: c_int = 15;

pub const RT5665_PWR_BST2_BIT: c_int = 14;

pub const RT5665_PWR_BST3_BIT: c_int = 13;

pub const RT5665_PWR_BST4_BIT: c_int = 12;

pub const RT5665_PWR_MB1_BIT: c_int = 11;

pub const RT5665_PWR_MB2_BIT: c_int = 10;

pub const RT5665_PWR_MB3_BIT: c_int = 9;

pub const RT5665_PWR_BST1_P_BIT: c_int = 7;

pub const RT5665_PWR_BST2_P_BIT: c_int = 6;

pub const RT5665_PWR_BST3_P_BIT: c_int = 5;

pub const RT5665_PWR_BST4_P_BIT: c_int = 4;

pub const RT5665_PWR_JD1_BIT: c_int = 3;

pub const RT5665_PWR_JD2_BIT: c_int = 2;

pub const RT5665_PWR_RM1_L_BIT: c_int = 1;

pub const RT5665_PWR_RM1_R_BIT: c_int = 0;
// Power Management for Analog 3 (0x0065)

pub const RT5665_PWR_CBJ_BIT: c_int = 9;

pub const RT5665_PWR_BST_L_BIT: c_int = 8;

pub const RT5665_PWR_BST_R_BIT: c_int = 7;

pub const RT5665_PWR_PLL_BIT: c_int = 6;

pub const RT5665_PWR_LDO2_BIT: c_int = 2;

pub const RT5665_PWR_SVD_BIT: c_int = 1;
// Power Management for Mixer (0x0066)

pub const RT5665_PWR_RM2_L_BIT: c_int = 15;

pub const RT5665_PWR_RM2_R_BIT: c_int = 14;

pub const RT5665_PWR_OM_L_BIT: c_int = 13;

pub const RT5665_PWR_OM_R_BIT: c_int = 12;

pub const RT5665_PWR_MM_BIT: c_int = 11;

pub const RT5665_PWR_AEC_REF_BIT: c_int = 6;

pub const RT5665_PWR_STO1_DAC_L_BIT: c_int = 5;

pub const RT5665_PWR_STO1_DAC_R_BIT: c_int = 4;

pub const RT5665_PWR_MONO_DAC_L_BIT: c_int = 3;

pub const RT5665_PWR_MONO_DAC_R_BIT: c_int = 2;

pub const RT5665_PWR_STO2_DAC_L_BIT: c_int = 1;

pub const RT5665_PWR_STO2_DAC_R_BIT: c_int = 0;
// Power Management for Volume (0x0067)

pub const RT5665_PWR_OV_L_BIT: c_int = 13;

pub const RT5665_PWR_OV_R_BIT: c_int = 12;

pub const RT5665_PWR_IN_L_BIT: c_int = 9;

pub const RT5665_PWR_IN_R_BIT: c_int = 8;

pub const RT5665_PWR_MV_BIT: c_int = 7;

pub const RT5665_PWR_MIC_DET_BIT: c_int = 5;
// (0x006b)
pub const RT5665_SYS_CLK_DET: c_int = 15;
pub const RT5665_HP_CLK_DET: c_int = 14;
pub const RT5665_MONO_CLK_DET: c_int = 13;
pub const RT5665_LOUT_CLK_DET: c_int = 12;
pub const RT5665_POW_CLK_DET: c_int = 0;
// Digital Microphone Control 1 (0x006e)

pub const RT5665_DMIC_1_EN_SFT: c_int = 15;

pub const RT5665_DMIC_2_EN_SFT: c_int = 14;

pub const RT5665_DMIC_2_DP_SFT: c_int = 9;

pub const RT5665_DMIC_CLK_SFT: c_int = 5;

pub const RT5665_DMIC_1_DP_SFT: c_int = 1;

// Digital Microphone Control 1 (0x006f)

pub const RT5665_DMIC_2L_LH_SFT: c_int = 3;

pub const RT5665_DMIC_2R_LH_SFT: c_int = 2;

pub const RT5665_DMIC_1L_LH_SFT: c_int = 1;

pub const RT5665_DMIC_1R_LH_SFT: c_int = 0;

// I2S1/2/3 Audio Serial Data Port Control (0x0070 0x0071 0x0072)

pub const RT5665_I2S_MS_SFT: c_int = 15;

pub const RT5665_I2S_PIN_CFG_SFT: c_int = 14;

pub const RT5665_I2S_CLK_SEL_SFT: c_int = 11;

pub const RT5665_I2S_BP_SFT: c_int = 8;

pub const RT5665_I2S_DL_SFT: c_int = 4;

pub const RT5665_I2S_DF_SFT: c_int = 0;

// ADC/DAC Clock Control 1 (0x0073)

pub const RT5665_I2S_PD1_SFT: c_int = 12;

pub const RT5665_I2S_M_PD2_SFT: c_int = 8;

pub const RT5665_I2S_CLK_SRC_SFT: c_int = 4;

pub const RT5665_DAC_OSR_SFT: c_int = 2;

pub const RT5665_ADC_OSR_SFT: c_int = 0;

// ADC/DAC Clock Control 2 (0x0074)

pub const RT5665_I2S_BCLK_MS2_SFT: c_int = 15;

pub const RT5665_I2S_PD2_SFT: c_int = 12;

pub const RT5665_I2S_BCLK_MS3_SFT: c_int = 11;

pub const RT5665_I2S_PD3_SFT: c_int = 8;

pub const RT5665_I2S_PD4_SFT: c_int = 4;

// TDM control 1 (0x0078)

// TDM control 2 (0x0079)
pub const RT5665_I2S1_1_DS_ADC_SLOT01_SFT: c_int = 14;
pub const RT5665_I2S1_1_DS_ADC_SLOT23_SFT: c_int = 12;
pub const RT5665_I2S1_1_DS_ADC_SLOT45_SFT: c_int = 10;
pub const RT5665_I2S1_1_DS_ADC_SLOT67_SFT: c_int = 8;
pub const RT5665_I2S1_2_DS_ADC_SLOT01_SFT: c_int = 6;
pub const RT5665_I2S1_2_DS_ADC_SLOT23_SFT: c_int = 4;
pub const RT5665_I2S1_2_DS_ADC_SLOT45_SFT: c_int = 2;
pub const RT5665_I2S1_2_DS_ADC_SLOT67_SFT: c_int = 0;
// TDM control 3/4 (0x007a) (0x007b)
pub const RT5665_IF1_ADC1_SEL_SFT: c_int = 10;
pub const RT5665_IF1_ADC2_SEL_SFT: c_int = 9;
pub const RT5665_IF1_ADC3_SEL_SFT: c_int = 8;
pub const RT5665_IF1_ADC4_SEL_SFT: c_int = 7;
pub const RT5665_TDM_ADC_SEL_SFT: c_int = 0;

// Global Clock Control (0x0080)

pub const RT5665_SCLK_SRC_SFT: c_int = 14;

pub const RT5665_PLL1_SRC_SFT: c_int = 8;

pub const RT5665_PLL1_PD_SFT: c_int = 4;
pub const RT5665_PLL_INP_MAX: c_int = 40000000;
pub const RT5665_PLL_INP_MIN: c_int = 256000;
// PLL M/N/K Code Control 1 (0x0081)
pub const RT5665_PLL_N_MAX: c_uint = 0x001ff;

pub const RT5665_PLL_N_SFT: c_int = 7;
pub const RT5665_PLL_K_MAX: c_uint = 0x001f;

pub const RT5665_PLL_K_SFT: c_int = 0;
// PLL M/N/K Code Control 2 (0x0082)
pub const RT5665_PLL_M_MAX: c_uint = 0x00f;

pub const RT5665_PLL_M_SFT: c_int = 12;

pub const RT5665_PLL_M_BP_SFT: c_int = 11;

pub const RT5665_PLL_K_BP_SFT: c_int = 10;
// PLL tracking mode 1 (0x0083)

pub const RT5665_I2S3_ASRC_SFT: c_int = 15;

pub const RT5665_I2S2_ASRC_SFT: c_int = 14;

pub const RT5665_I2S1_ASRC_SFT: c_int = 13;

pub const RT5665_DAC_STO1_ASRC_SFT: c_int = 12;

pub const RT5665_DAC_STO2_ASRC_SFT: c_int = 11;

pub const RT5665_DAC_MONO_L_ASRC_SFT: c_int = 10;

pub const RT5665_DAC_MONO_R_ASRC_SFT: c_int = 9;

pub const RT5665_DMIC_STO1_ASRC_SFT: c_int = 8;

pub const RT5665_DMIC_STO2_ASRC_SFT: c_int = 7;

pub const RT5665_DMIC_MONO_L_ASRC_SFT: c_int = 6;

pub const RT5665_DMIC_MONO_R_ASRC_SFT: c_int = 5;

pub const RT5665_ADC_STO1_ASRC_SFT: c_int = 4;

pub const RT5665_ADC_STO2_ASRC_SFT: c_int = 3;

pub const RT5665_ADC_MONO_L_ASRC_SFT: c_int = 2;

pub const RT5665_ADC_MONO_R_ASRC_SFT: c_int = 1;
// PLL tracking mode 2 (0x0084)

pub const RT5665_DA_STO1_CLK_SEL_SFT: c_int = 12;

pub const RT5665_DA_STO2_CLK_SEL_SFT: c_int = 8;

pub const RT5665_DA_MONOL_CLK_SEL_SFT: c_int = 4;

pub const RT5665_DA_MONOR_CLK_SEL_SFT: c_int = 0;
// PLL tracking mode 3 (0x0085)

pub const RT5665_AD_STO1_CLK_SEL_SFT: c_int = 12;

pub const RT5665_AD_STO2_CLK_SEL_SFT: c_int = 8;

pub const RT5665_AD_MONOL_CLK_SEL_SFT: c_int = 4;

pub const RT5665_AD_MONOR_CLK_SEL_SFT: c_int = 0;
// ASRC Control 4 (0x0086)

pub const RT5665_I2S1_RATE_SFT: c_int = 12;

pub const RT5665_I2S2_RATE_SFT: c_int = 8;

pub const RT5665_I2S3_RATE_SFT: c_int = 4;
// Depop Mode Control 1 (0x008e)

// Depop Mode Control 2 (0x8f)

pub const RT5665_DEPOP_SFT: c_int = 13;

pub const RT5665_RAMP_SFT: c_int = 12;

pub const RT5665_BPS_SFT: c_int = 11;

pub const RT5665_FAST_UPDN_SFT: c_int = 10;

pub const RT5665_MRES_SFT: c_int = 8;

pub const RT5665_VLO_SFT: c_int = 7;

pub const RT5665_DIG_DP_SFT: c_int = 6;

pub const RT5665_DP_TH_SFT: c_int = 4;
// Depop Mode Control 3 (0x90)

pub const RT5665_CP_SYS_SFT: c_int = 12;

pub const RT5665_CP_FQ1_SFT: c_int = 8;

pub const RT5665_CP_FQ2_SFT: c_int = 4;

pub const RT5665_CP_FQ3_SFT: c_int = 0;
pub const RT5665_CP_FQ_1_5_KHZ: c_int = 0;
pub const RT5665_CP_FQ_3_KHZ: c_int = 1;
pub const RT5665_CP_FQ_6_KHZ: c_int = 2;
pub const RT5665_CP_FQ_12_KHZ: c_int = 3;
pub const RT5665_CP_FQ_24_KHZ: c_int = 4;
pub const RT5665_CP_FQ_48_KHZ: c_int = 5;
pub const RT5665_CP_FQ_96_KHZ: c_int = 6;
pub const RT5665_CP_FQ_192_KHZ: c_int = 7;
// HPOUT charge pump 1 (0x0091)

pub const RT5665_OSW_L_SFT: c_int = 11;

pub const RT5665_OSW_R_SFT: c_int = 10;

pub const RT5665_PM_HP_SFT: c_int = 8;

pub const RT5665_IB_HP_SFT: c_int = 6;

// PV detection and SPK gain control (0x92)

pub const RT5665_PVDD_DET_SFT: c_int = 15;

pub const RT5665_SPK_AG_SFT: c_int = 14;

// Micbias Control1 (0x93)

pub const RT5665_MIC1_BS_SFT: c_int = 15;

pub const RT5665_MIC2_BS_SFT: c_int = 14;

pub const RT5665_MIC1_CLK_SFT: c_int = 13;

pub const RT5665_MIC2_CLK_SFT: c_int = 12;

pub const RT5665_MIC1_OVCD_SFT: c_int = 11;

pub const RT5665_MIC1_OVTH_SFT: c_int = 9;

pub const RT5665_MIC2_OVCD_SFT: c_int = 8;

pub const RT5665_MIC2_OVTH_SFT: c_int = 6;

pub const RT5665_PWR_MB_SFT: c_int = 5;

// Micbias Control2 (0x94)

pub const RT5665_PWR_CLK25M_SFT: c_int = 9;

pub const RT5665_PWR_CLK1M_SFT: c_int = 8;

// I2S Master Mode Clock Control 1 (0x00a0)

pub const RT5665_I2S2_SRC_SFT: c_int = 12;

pub const RT5665_I2S2_M_PD_SFT: c_int = 8;

pub const RT5665_I2S3_SRC_SFT: c_int = 4;

pub const RT5665_I2S3_M_PD_SFT: c_int = 0;
// EQ Control 1 (0x00b0)

pub const RT5665_EQ_UPD_BIT: c_int = 14;

pub const RT5665_EQ_CD_SFT: c_int = 13;

pub const RT5665_EQ_DITH_SFT: c_int = 8;

// IRQ Control 1 (0x00b7)

pub const RT5665_JD1_1_EN_SFT: c_int = 15;

pub const RT5665_JD1_2_EN_SFT: c_int = 12;

// IRQ Control 2 (0x00b8)

// IRQ Control 5 (0x00ba)

pub const RT5665_IRQ_JD_EN_SFT: c_int = 3;
// GPIO Control 1 (0x00c0)

pub const RT5665_GP1_PIN_SFT: c_int = 15;

pub const RT5665_GP2_PIN_SFT: c_int = 13;

pub const RT5665_GP3_PIN_SFT: c_int = 11;

pub const RT5665_GP4_PIN_SFT: c_int = 9;

pub const RT5665_GP5_PIN_SFT: c_int = 7;

pub const RT5665_GP6_PIN_SFT: c_int = 5;

pub const RT5665_GP7_PIN_SFT: c_int = 3;

pub const RT5665_GP8_PIN_SFT: c_int = 1;

// GPIO Control 2 (0x00c1)

pub const RT5665_GP9_PIN_SFT: c_int = 14;

pub const RT5665_GP10_PIN_SFT: c_int = 12;

// GPIO Control 3 (0x00c2)

// Soft volume and zero cross control 1 (0x00d9)

pub const RT5665_SV_SFT: c_int = 15;

pub const RT5665_OUT_SV_SFT: c_int = 13;

pub const RT5665_HP_SV_SFT: c_int = 12;

pub const RT5665_ZCD_DIG_SFT: c_int = 11;

pub const RT5665_ZCD_SFT: c_int = 10;

pub const RT5665_SV_DLY_SFT: c_int = 0;
// Soft volume and zero cross control 2 (0x00da)

pub const RT5665_ZCD_HP_SFT: c_int = 15;

// 4 Button Inline Command Control 2 (0x00e0)

// Analog JD Control 1 (0x00f0)

// Jack Detect Control 3 (0x00f8)

// Digital Misc Control (0x00fa)

pub const RT5665_DIG_GATE_CTRL: c_uint = 0x1;

// Chopper and Clock control for ADC (0x011c)

pub const RT5665_M_RF_DIG_SFT: c_int = 12;

// Chopper and Clock control for DAC (0x013a)

pub const RT5665_CKXEN_DAC1_SFT: c_int = 13;

pub const RT5665_CKGEN_DAC1_SFT: c_int = 12;

pub const RT5665_CKXEN_DAC2_SFT: c_int = 5;

pub const RT5665_CKGEN_DAC2_SFT: c_int = 4;
// Chopper and Clock control for ADC (0x013b)

pub const RT5665_CKXEN_ADC1_SFT: c_int = 13;

pub const RT5665_CKGEN_ADC1_SFT: c_int = 12;

pub const RT5665_CKXEN_ADC2_SFT: c_int = 5;

pub const RT5665_CKGEN_ADC2_SFT: c_int = 4;
// Volume test (0x013f)

// Test Mode Control 1 (0x0145)

pub const RT5665_AD2DA_LB_SFT: c_int = 9;
// Stereo Noise Gate Control 1 (0x0160)

// Stereo1 DAC Silence Detection Control (0x0190)

// SAR ADC Inline Command Control 1 (0x0210)

// System Clock Source
// PLL1 Source
// filter mask
