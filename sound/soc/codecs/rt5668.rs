//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5668.h
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
// rt5668.h  --  RT5668/RT5658 ALSA SoC audio driver
//
// Copyright 2018 Realtek Microelectronics
// Author: Bard Liao <bardliao@realtek.com>
//

pub const DEVICE_ID: c_uint = 0x6530;
// Info
pub const RT5668_RESET: c_uint = 0x0000;
pub const RT5668_VERSION_ID: c_uint = 0x00fd;
pub const RT5668_VENDOR_ID: c_uint = 0x00fe;
pub const RT5668_DEVICE_ID: c_uint = 0x00ff;
// I/O - Output
pub const RT5668_HP_CTRL_1: c_uint = 0x0002;
pub const RT5668_HP_CTRL_2: c_uint = 0x0003;
pub const RT5668_HPL_GAIN: c_uint = 0x0005;
pub const RT5668_HPR_GAIN: c_uint = 0x0006;
pub const RT5668_I2C_CTRL: c_uint = 0x0008;
// I/O - Input
pub const RT5668_CBJ_BST_CTRL: c_uint = 0x000b;
pub const RT5668_CBJ_CTRL_1: c_uint = 0x0010;
pub const RT5668_CBJ_CTRL_2: c_uint = 0x0011;
pub const RT5668_CBJ_CTRL_3: c_uint = 0x0012;
pub const RT5668_CBJ_CTRL_4: c_uint = 0x0013;
pub const RT5668_CBJ_CTRL_5: c_uint = 0x0014;
pub const RT5668_CBJ_CTRL_6: c_uint = 0x0015;
pub const RT5668_CBJ_CTRL_7: c_uint = 0x0016;
// I/O - ADC/DAC/DMIC
pub const RT5668_DAC1_DIG_VOL: c_uint = 0x0019;
pub const RT5668_STO1_ADC_DIG_VOL: c_uint = 0x001c;
pub const RT5668_STO1_ADC_BOOST: c_uint = 0x001f;
pub const RT5668_HP_IMP_GAIN_1: c_uint = 0x0022;
pub const RT5668_HP_IMP_GAIN_2: c_uint = 0x0023;
// Mixer - D-D
pub const RT5668_SIDETONE_CTRL: c_uint = 0x0024;
pub const RT5668_STO1_ADC_MIXER: c_uint = 0x0026;
pub const RT5668_AD_DA_MIXER: c_uint = 0x0029;
pub const RT5668_STO1_DAC_MIXER: c_uint = 0x002a;
pub const RT5668_A_DAC1_MUX: c_uint = 0x002b;
pub const RT5668_DIG_INF2_DATA: c_uint = 0x0030;
// Mixer - ADC
pub const RT5668_REC_MIXER: c_uint = 0x003c;
pub const RT5668_CAL_REC: c_uint = 0x0044;
pub const RT5668_ALC_BACK_GAIN: c_uint = 0x0049;
// Power
pub const RT5668_PWR_DIG_1: c_uint = 0x0061;
pub const RT5668_PWR_DIG_2: c_uint = 0x0062;
pub const RT5668_PWR_ANLG_1: c_uint = 0x0063;
pub const RT5668_PWR_ANLG_2: c_uint = 0x0064;
pub const RT5668_PWR_ANLG_3: c_uint = 0x0065;
pub const RT5668_PWR_MIXER: c_uint = 0x0066;
pub const RT5668_PWR_VOL: c_uint = 0x0067;
// Clock Detect
pub const RT5668_CLK_DET: c_uint = 0x006b;
// Filter Auto Reset
pub const RT5668_RESET_LPF_CTRL: c_uint = 0x006c;
pub const RT5668_RESET_HPF_CTRL: c_uint = 0x006d;
// DMIC
pub const RT5668_DMIC_CTRL_1: c_uint = 0x006e;
// Format - ADC/DAC
pub const RT5668_I2S1_SDP: c_uint = 0x0070;
pub const RT5668_I2S2_SDP: c_uint = 0x0071;
pub const RT5668_ADDA_CLK_1: c_uint = 0x0073;
pub const RT5668_ADDA_CLK_2: c_uint = 0x0074;
pub const RT5668_I2S1_F_DIV_CTRL_1: c_uint = 0x0075;
pub const RT5668_I2S1_F_DIV_CTRL_2: c_uint = 0x0076;
// Format - TDM Control
pub const RT5668_TDM_CTRL: c_uint = 0x0079;
pub const RT5668_TDM_ADDA_CTRL_1: c_uint = 0x007a;
pub const RT5668_TDM_ADDA_CTRL_2: c_uint = 0x007b;
pub const RT5668_DATA_SEL_CTRL_1: c_uint = 0x007c;
pub const RT5668_TDM_TCON_CTRL: c_uint = 0x007e;
// Function - Analog
pub const RT5668_GLB_CLK: c_uint = 0x0080;
pub const RT5668_PLL_CTRL_1: c_uint = 0x0081;
pub const RT5668_PLL_CTRL_2: c_uint = 0x0082;
pub const RT5668_PLL_TRACK_1: c_uint = 0x0083;
pub const RT5668_PLL_TRACK_2: c_uint = 0x0084;
pub const RT5668_PLL_TRACK_3: c_uint = 0x0085;
pub const RT5668_PLL_TRACK_4: c_uint = 0x0086;
pub const RT5668_PLL_TRACK_5: c_uint = 0x0087;
pub const RT5668_PLL_TRACK_6: c_uint = 0x0088;
pub const RT5668_PLL_TRACK_11: c_uint = 0x008c;
pub const RT5668_SDW_REF_CLK: c_uint = 0x008d;
pub const RT5668_DEPOP_1: c_uint = 0x008e;
pub const RT5668_DEPOP_2: c_uint = 0x008f;
pub const RT5668_HP_CHARGE_PUMP_1: c_uint = 0x0091;
pub const RT5668_HP_CHARGE_PUMP_2: c_uint = 0x0092;
pub const RT5668_MICBIAS_1: c_uint = 0x0093;
pub const RT5668_MICBIAS_2: c_uint = 0x0094;
pub const RT5668_PLL_TRACK_12: c_uint = 0x0098;
pub const RT5668_PLL_TRACK_14: c_uint = 0x009a;
pub const RT5668_PLL2_CTRL_1: c_uint = 0x009b;
pub const RT5668_PLL2_CTRL_2: c_uint = 0x009c;
pub const RT5668_PLL2_CTRL_3: c_uint = 0x009d;
pub const RT5668_PLL2_CTRL_4: c_uint = 0x009e;
pub const RT5668_RC_CLK_CTRL: c_uint = 0x009f;
pub const RT5668_I2S_M_CLK_CTRL_1: c_uint = 0x00a0;
pub const RT5668_I2S2_F_DIV_CTRL_1: c_uint = 0x00a3;
pub const RT5668_I2S2_F_DIV_CTRL_2: c_uint = 0x00a4;
// Function - Digital
pub const RT5668_EQ_CTRL_1: c_uint = 0x00ae;
pub const RT5668_EQ_CTRL_2: c_uint = 0x00af;
pub const RT5668_IRQ_CTRL_1: c_uint = 0x00b6;
pub const RT5668_IRQ_CTRL_2: c_uint = 0x00b7;
pub const RT5668_IRQ_CTRL_3: c_uint = 0x00b8;
pub const RT5668_IRQ_CTRL_4: c_uint = 0x00b9;
pub const RT5668_INT_ST_1: c_uint = 0x00be;
pub const RT5668_GPIO_CTRL_1: c_uint = 0x00c0;
pub const RT5668_GPIO_CTRL_2: c_uint = 0x00c1;
pub const RT5668_GPIO_CTRL_3: c_uint = 0x00c2;
pub const RT5668_HP_AMP_DET_CTRL_1: c_uint = 0x00d0;
pub const RT5668_HP_AMP_DET_CTRL_2: c_uint = 0x00d1;
pub const RT5668_MID_HP_AMP_DET: c_uint = 0x00d2;
pub const RT5668_LOW_HP_AMP_DET: c_uint = 0x00d3;
pub const RT5668_DELAY_BUF_CTRL: c_uint = 0x00d4;
pub const RT5668_SV_ZCD_1: c_uint = 0x00d9;
pub const RT5668_SV_ZCD_2: c_uint = 0x00da;
pub const RT5668_IL_CMD_1: c_uint = 0x00db;
pub const RT5668_IL_CMD_2: c_uint = 0x00dc;
pub const RT5668_IL_CMD_3: c_uint = 0x00dd;
pub const RT5668_IL_CMD_4: c_uint = 0x00de;
pub const RT5668_IL_CMD_5: c_uint = 0x00df;
pub const RT5668_IL_CMD_6: c_uint = 0x00e0;
pub const RT5668_4BTN_IL_CMD_1: c_uint = 0x00e2;
pub const RT5668_4BTN_IL_CMD_2: c_uint = 0x00e3;
pub const RT5668_4BTN_IL_CMD_3: c_uint = 0x00e4;
pub const RT5668_4BTN_IL_CMD_4: c_uint = 0x00e5;
pub const RT5668_4BTN_IL_CMD_5: c_uint = 0x00e6;
pub const RT5668_4BTN_IL_CMD_6: c_uint = 0x00e7;
pub const RT5668_4BTN_IL_CMD_7: c_uint = 0x00e8;
pub const RT5668_ADC_STO1_HP_CTRL_1: c_uint = 0x00ea;
pub const RT5668_ADC_STO1_HP_CTRL_2: c_uint = 0x00eb;
pub const RT5668_AJD1_CTRL: c_uint = 0x00f0;
pub const RT5668_JD1_THD: c_uint = 0x00f1;
pub const RT5668_JD2_THD: c_uint = 0x00f2;
pub const RT5668_JD_CTRL_1: c_uint = 0x00f6;
// General Control
pub const RT5668_DUMMY_1: c_uint = 0x00fa;
pub const RT5668_DUMMY_2: c_uint = 0x00fb;
pub const RT5668_DUMMY_3: c_uint = 0x00fc;
pub const RT5668_DAC_ADC_DIG_VOL1: c_uint = 0x0100;
pub const RT5668_BIAS_CUR_CTRL_2: c_uint = 0x010b;
pub const RT5668_BIAS_CUR_CTRL_3: c_uint = 0x010c;
pub const RT5668_BIAS_CUR_CTRL_4: c_uint = 0x010d;
pub const RT5668_BIAS_CUR_CTRL_5: c_uint = 0x010e;
pub const RT5668_BIAS_CUR_CTRL_6: c_uint = 0x010f;
pub const RT5668_BIAS_CUR_CTRL_7: c_uint = 0x0110;
pub const RT5668_BIAS_CUR_CTRL_8: c_uint = 0x0111;
pub const RT5668_BIAS_CUR_CTRL_9: c_uint = 0x0112;
pub const RT5668_BIAS_CUR_CTRL_10: c_uint = 0x0113;
pub const RT5668_VREF_REC_OP_FB_CAP_CTRL: c_uint = 0x0117;
pub const RT5668_CHARGE_PUMP_1: c_uint = 0x0125;
pub const RT5668_DIG_IN_CTRL_1: c_uint = 0x0132;
pub const RT5668_PAD_DRIVING_CTRL: c_uint = 0x0136;
pub const RT5668_SOFT_RAMP_DEPOP: c_uint = 0x0138;
pub const RT5668_CHOP_DAC: c_uint = 0x013a;
pub const RT5668_CHOP_ADC: c_uint = 0x013b;
pub const RT5668_CALIB_ADC_CTRL: c_uint = 0x013c;
pub const RT5668_VOL_TEST: c_uint = 0x013f;
pub const RT5668_SPKVDD_DET_STA: c_uint = 0x0142;
pub const RT5668_TEST_MODE_CTRL_1: c_uint = 0x0145;
pub const RT5668_TEST_MODE_CTRL_2: c_uint = 0x0146;
pub const RT5668_TEST_MODE_CTRL_3: c_uint = 0x0147;
pub const RT5668_TEST_MODE_CTRL_4: c_uint = 0x0148;
pub const RT5668_TEST_MODE_CTRL_5: c_uint = 0x0149;
pub const RT5668_PLL1_INTERNAL: c_uint = 0x0150;
pub const RT5668_PLL2_INTERNAL: c_uint = 0x0151;
pub const RT5668_STO_NG2_CTRL_1: c_uint = 0x0160;
pub const RT5668_STO_NG2_CTRL_2: c_uint = 0x0161;
pub const RT5668_STO_NG2_CTRL_3: c_uint = 0x0162;
pub const RT5668_STO_NG2_CTRL_4: c_uint = 0x0163;
pub const RT5668_STO_NG2_CTRL_5: c_uint = 0x0164;
pub const RT5668_STO_NG2_CTRL_6: c_uint = 0x0165;
pub const RT5668_STO_NG2_CTRL_7: c_uint = 0x0166;
pub const RT5668_STO_NG2_CTRL_8: c_uint = 0x0167;
pub const RT5668_STO_NG2_CTRL_9: c_uint = 0x0168;
pub const RT5668_STO_NG2_CTRL_10: c_uint = 0x0169;
pub const RT5668_STO1_DAC_SIL_DET: c_uint = 0x0190;
pub const RT5668_SIL_PSV_CTRL1: c_uint = 0x0194;
pub const RT5668_SIL_PSV_CTRL2: c_uint = 0x0195;
pub const RT5668_SIL_PSV_CTRL3: c_uint = 0x0197;
pub const RT5668_SIL_PSV_CTRL4: c_uint = 0x0198;
pub const RT5668_SIL_PSV_CTRL5: c_uint = 0x0199;
pub const RT5668_HP_IMP_SENS_CTRL_01: c_uint = 0x01af;
pub const RT5668_HP_IMP_SENS_CTRL_02: c_uint = 0x01b0;
pub const RT5668_HP_IMP_SENS_CTRL_03: c_uint = 0x01b1;
pub const RT5668_HP_IMP_SENS_CTRL_04: c_uint = 0x01b2;
pub const RT5668_HP_IMP_SENS_CTRL_05: c_uint = 0x01b3;
pub const RT5668_HP_IMP_SENS_CTRL_06: c_uint = 0x01b4;
pub const RT5668_HP_IMP_SENS_CTRL_07: c_uint = 0x01b5;
pub const RT5668_HP_IMP_SENS_CTRL_08: c_uint = 0x01b6;
pub const RT5668_HP_IMP_SENS_CTRL_09: c_uint = 0x01b7;
pub const RT5668_HP_IMP_SENS_CTRL_10: c_uint = 0x01b8;
pub const RT5668_HP_IMP_SENS_CTRL_11: c_uint = 0x01b9;
pub const RT5668_HP_IMP_SENS_CTRL_12: c_uint = 0x01ba;
pub const RT5668_HP_IMP_SENS_CTRL_13: c_uint = 0x01bb;
pub const RT5668_HP_IMP_SENS_CTRL_14: c_uint = 0x01bc;
pub const RT5668_HP_IMP_SENS_CTRL_15: c_uint = 0x01bd;
pub const RT5668_HP_IMP_SENS_CTRL_16: c_uint = 0x01be;
pub const RT5668_HP_IMP_SENS_CTRL_17: c_uint = 0x01bf;
pub const RT5668_HP_IMP_SENS_CTRL_18: c_uint = 0x01c0;
pub const RT5668_HP_IMP_SENS_CTRL_19: c_uint = 0x01c1;
pub const RT5668_HP_IMP_SENS_CTRL_20: c_uint = 0x01c2;
pub const RT5668_HP_IMP_SENS_CTRL_21: c_uint = 0x01c3;
pub const RT5668_HP_IMP_SENS_CTRL_22: c_uint = 0x01c4;
pub const RT5668_HP_IMP_SENS_CTRL_23: c_uint = 0x01c5;
pub const RT5668_HP_IMP_SENS_CTRL_24: c_uint = 0x01c6;
pub const RT5668_HP_IMP_SENS_CTRL_25: c_uint = 0x01c7;
pub const RT5668_HP_IMP_SENS_CTRL_26: c_uint = 0x01c8;
pub const RT5668_HP_IMP_SENS_CTRL_27: c_uint = 0x01c9;
pub const RT5668_HP_IMP_SENS_CTRL_28: c_uint = 0x01ca;
pub const RT5668_HP_IMP_SENS_CTRL_29: c_uint = 0x01cb;
pub const RT5668_HP_IMP_SENS_CTRL_30: c_uint = 0x01cc;
pub const RT5668_HP_IMP_SENS_CTRL_31: c_uint = 0x01cd;
pub const RT5668_HP_IMP_SENS_CTRL_32: c_uint = 0x01ce;
pub const RT5668_HP_IMP_SENS_CTRL_33: c_uint = 0x01cf;
pub const RT5668_HP_IMP_SENS_CTRL_34: c_uint = 0x01d0;
pub const RT5668_HP_IMP_SENS_CTRL_35: c_uint = 0x01d1;
pub const RT5668_HP_IMP_SENS_CTRL_36: c_uint = 0x01d2;
pub const RT5668_HP_IMP_SENS_CTRL_37: c_uint = 0x01d3;
pub const RT5668_HP_IMP_SENS_CTRL_38: c_uint = 0x01d4;
pub const RT5668_HP_IMP_SENS_CTRL_39: c_uint = 0x01d5;
pub const RT5668_HP_IMP_SENS_CTRL_40: c_uint = 0x01d6;
pub const RT5668_HP_IMP_SENS_CTRL_41: c_uint = 0x01d7;
pub const RT5668_HP_IMP_SENS_CTRL_42: c_uint = 0x01d8;
pub const RT5668_HP_IMP_SENS_CTRL_43: c_uint = 0x01d9;
pub const RT5668_HP_LOGIC_CTRL_1: c_uint = 0x01da;
pub const RT5668_HP_LOGIC_CTRL_2: c_uint = 0x01db;
pub const RT5668_HP_LOGIC_CTRL_3: c_uint = 0x01dc;
pub const RT5668_HP_CALIB_CTRL_1: c_uint = 0x01de;
pub const RT5668_HP_CALIB_CTRL_2: c_uint = 0x01df;
pub const RT5668_HP_CALIB_CTRL_3: c_uint = 0x01e0;
pub const RT5668_HP_CALIB_CTRL_4: c_uint = 0x01e1;
pub const RT5668_HP_CALIB_CTRL_5: c_uint = 0x01e2;
pub const RT5668_HP_CALIB_CTRL_6: c_uint = 0x01e3;
pub const RT5668_HP_CALIB_CTRL_7: c_uint = 0x01e4;
pub const RT5668_HP_CALIB_CTRL_9: c_uint = 0x01e6;
pub const RT5668_HP_CALIB_CTRL_10: c_uint = 0x01e7;
pub const RT5668_HP_CALIB_CTRL_11: c_uint = 0x01e8;
pub const RT5668_HP_CALIB_STA_1: c_uint = 0x01ea;
pub const RT5668_HP_CALIB_STA_2: c_uint = 0x01eb;
pub const RT5668_HP_CALIB_STA_3: c_uint = 0x01ec;
pub const RT5668_HP_CALIB_STA_4: c_uint = 0x01ed;
pub const RT5668_HP_CALIB_STA_5: c_uint = 0x01ee;
pub const RT5668_HP_CALIB_STA_6: c_uint = 0x01ef;
pub const RT5668_HP_CALIB_STA_7: c_uint = 0x01f0;
pub const RT5668_HP_CALIB_STA_8: c_uint = 0x01f1;
pub const RT5668_HP_CALIB_STA_9: c_uint = 0x01f2;
pub const RT5668_HP_CALIB_STA_10: c_uint = 0x01f3;
pub const RT5668_HP_CALIB_STA_11: c_uint = 0x01f4;
pub const RT5668_SAR_IL_CMD_1: c_uint = 0x0210;
pub const RT5668_SAR_IL_CMD_2: c_uint = 0x0211;
pub const RT5668_SAR_IL_CMD_3: c_uint = 0x0212;
pub const RT5668_SAR_IL_CMD_4: c_uint = 0x0213;
pub const RT5668_SAR_IL_CMD_5: c_uint = 0x0214;
pub const RT5668_SAR_IL_CMD_6: c_uint = 0x0215;
pub const RT5668_SAR_IL_CMD_7: c_uint = 0x0216;
pub const RT5668_SAR_IL_CMD_8: c_uint = 0x0217;
pub const RT5668_SAR_IL_CMD_9: c_uint = 0x0218;
pub const RT5668_SAR_IL_CMD_10: c_uint = 0x0219;
pub const RT5668_SAR_IL_CMD_11: c_uint = 0x021a;
pub const RT5668_SAR_IL_CMD_12: c_uint = 0x021b;
pub const RT5668_SAR_IL_CMD_13: c_uint = 0x021c;
pub const RT5668_EFUSE_CTRL_1: c_uint = 0x0250;
pub const RT5668_EFUSE_CTRL_2: c_uint = 0x0251;
pub const RT5668_EFUSE_CTRL_3: c_uint = 0x0252;
pub const RT5668_EFUSE_CTRL_4: c_uint = 0x0253;
pub const RT5668_EFUSE_CTRL_5: c_uint = 0x0254;
pub const RT5668_EFUSE_CTRL_6: c_uint = 0x0255;
pub const RT5668_EFUSE_CTRL_7: c_uint = 0x0256;
pub const RT5668_EFUSE_CTRL_8: c_uint = 0x0257;
pub const RT5668_EFUSE_CTRL_9: c_uint = 0x0258;
pub const RT5668_EFUSE_CTRL_10: c_uint = 0x0259;
pub const RT5668_EFUSE_CTRL_11: c_uint = 0x025a;
pub const RT5668_JD_TOP_VC_VTRL: c_uint = 0x0270;
pub const RT5668_DRC1_CTRL_0: c_uint = 0x02ff;
pub const RT5668_DRC1_CTRL_1: c_uint = 0x0300;
pub const RT5668_DRC1_CTRL_2: c_uint = 0x0301;
pub const RT5668_DRC1_CTRL_3: c_uint = 0x0302;
pub const RT5668_DRC1_CTRL_4: c_uint = 0x0303;
pub const RT5668_DRC1_CTRL_5: c_uint = 0x0304;
pub const RT5668_DRC1_CTRL_6: c_uint = 0x0305;
pub const RT5668_DRC1_HARD_LMT_CTRL_1: c_uint = 0x0306;
pub const RT5668_DRC1_HARD_LMT_CTRL_2: c_uint = 0x0307;
pub const RT5668_DRC1_PRIV_1: c_uint = 0x0310;
pub const RT5668_DRC1_PRIV_2: c_uint = 0x0311;
pub const RT5668_DRC1_PRIV_3: c_uint = 0x0312;
pub const RT5668_DRC1_PRIV_4: c_uint = 0x0313;
pub const RT5668_DRC1_PRIV_5: c_uint = 0x0314;
pub const RT5668_DRC1_PRIV_6: c_uint = 0x0315;
pub const RT5668_DRC1_PRIV_7: c_uint = 0x0316;
pub const RT5668_DRC1_PRIV_8: c_uint = 0x0317;
pub const RT5668_EQ_AUTO_RCV_CTRL1: c_uint = 0x03c0;
pub const RT5668_EQ_AUTO_RCV_CTRL2: c_uint = 0x03c1;
pub const RT5668_EQ_AUTO_RCV_CTRL3: c_uint = 0x03c2;
pub const RT5668_EQ_AUTO_RCV_CTRL4: c_uint = 0x03c3;
pub const RT5668_EQ_AUTO_RCV_CTRL5: c_uint = 0x03c4;
pub const RT5668_EQ_AUTO_RCV_CTRL6: c_uint = 0x03c5;
pub const RT5668_EQ_AUTO_RCV_CTRL7: c_uint = 0x03c6;
pub const RT5668_EQ_AUTO_RCV_CTRL8: c_uint = 0x03c7;
pub const RT5668_EQ_AUTO_RCV_CTRL9: c_uint = 0x03c8;
pub const RT5668_EQ_AUTO_RCV_CTRL10: c_uint = 0x03c9;
pub const RT5668_EQ_AUTO_RCV_CTRL11: c_uint = 0x03ca;
pub const RT5668_EQ_AUTO_RCV_CTRL12: c_uint = 0x03cb;
pub const RT5668_EQ_AUTO_RCV_CTRL13: c_uint = 0x03cc;
pub const RT5668_ADC_L_EQ_LPF1_A1: c_uint = 0x03d0;
pub const RT5668_R_EQ_LPF1_A1: c_uint = 0x03d1;
pub const RT5668_L_EQ_LPF1_H0: c_uint = 0x03d2;
pub const RT5668_R_EQ_LPF1_H0: c_uint = 0x03d3;
pub const RT5668_L_EQ_BPF1_A1: c_uint = 0x03d4;
pub const RT5668_R_EQ_BPF1_A1: c_uint = 0x03d5;
pub const RT5668_L_EQ_BPF1_A2: c_uint = 0x03d6;
pub const RT5668_R_EQ_BPF1_A2: c_uint = 0x03d7;
pub const RT5668_L_EQ_BPF1_H0: c_uint = 0x03d8;
pub const RT5668_R_EQ_BPF1_H0: c_uint = 0x03d9;
pub const RT5668_L_EQ_BPF2_A1: c_uint = 0x03da;
pub const RT5668_R_EQ_BPF2_A1: c_uint = 0x03db;
pub const RT5668_L_EQ_BPF2_A2: c_uint = 0x03dc;
pub const RT5668_R_EQ_BPF2_A2: c_uint = 0x03dd;
pub const RT5668_L_EQ_BPF2_H0: c_uint = 0x03de;
pub const RT5668_R_EQ_BPF2_H0: c_uint = 0x03df;
pub const RT5668_L_EQ_BPF3_A1: c_uint = 0x03e0;
pub const RT5668_R_EQ_BPF3_A1: c_uint = 0x03e1;
pub const RT5668_L_EQ_BPF3_A2: c_uint = 0x03e2;
pub const RT5668_R_EQ_BPF3_A2: c_uint = 0x03e3;
pub const RT5668_L_EQ_BPF3_H0: c_uint = 0x03e4;
pub const RT5668_R_EQ_BPF3_H0: c_uint = 0x03e5;
pub const RT5668_L_EQ_BPF4_A1: c_uint = 0x03e6;
pub const RT5668_R_EQ_BPF4_A1: c_uint = 0x03e7;
pub const RT5668_L_EQ_BPF4_A2: c_uint = 0x03e8;
pub const RT5668_R_EQ_BPF4_A2: c_uint = 0x03e9;
pub const RT5668_L_EQ_BPF4_H0: c_uint = 0x03ea;
pub const RT5668_R_EQ_BPF4_H0: c_uint = 0x03eb;
pub const RT5668_L_EQ_HPF1_A1: c_uint = 0x03ec;
pub const RT5668_R_EQ_HPF1_A1: c_uint = 0x03ed;
pub const RT5668_L_EQ_HPF1_H0: c_uint = 0x03ee;
pub const RT5668_R_EQ_HPF1_H0: c_uint = 0x03ef;
pub const RT5668_L_EQ_PRE_VOL: c_uint = 0x03f0;
pub const RT5668_R_EQ_PRE_VOL: c_uint = 0x03f1;
pub const RT5668_L_EQ_POST_VOL: c_uint = 0x03f2;
pub const RT5668_R_EQ_POST_VOL: c_uint = 0x03f3;
pub const RT5668_I2C_MODE: c_uint = 0xffff;
// global definition

pub const RT5668_L_MUTE_SFT: c_int = 15;

pub const RT5668_VOL_L_SFT: c_int = 14;

pub const RT5668_R_MUTE_SFT: c_int = 7;

pub const RT5668_VOL_R_SFT: c_int = 6;

pub const RT5668_L_VOL_SFT: c_int = 8;

pub const RT5668_R_VOL_SFT: c_int = 0;
// Headphone Amp L/R Analog Gain and Digital NG2 Gain Control (0x0005 0x0006)

pub const RT5668_G_HP_SFT: c_int = 8;

pub const RT5668_G_STO_DA_SFT: c_int = 0;
// CBJ Control (0x000b)

pub const RT5668_BST_CBJ_SFT: c_int = 8;
// Embeeded Jack and Type Detection Control 1 (0x0010)

pub const RT5668_EMB_JD_EN_SFT: c_int = 15;

pub const RT5668_JD_MODE_SFT: c_int = 13;

pub const RT5668_DET_TYPE_SFT: c_int = 12;

// Embeeded Jack and Type Detection Control 2 (0x0011)

pub const RT5668_EXT_JD_SRC_SFT: c_int = 4;

// Combo Jack and Type Detection Control 3 (0x0012)

// Combo Jack and Type Detection Control 4 (0x0013)

// DAC1 Digital Volume (0x0019)

pub const RT5668_DAC_L1_VOL_SFT: c_int = 8;

pub const RT5668_DAC_R1_VOL_SFT: c_int = 0;
// ADC Digital Volume Control (0x001c)

pub const RT5668_ADC_L_VOL_SFT: c_int = 8;

pub const RT5668_ADC_R_VOL_SFT: c_int = 0;
// Stereo1 ADC Boost Gain Control (0x001f)

pub const RT5668_STO1_ADC_L_BST_SFT: c_int = 14;

pub const RT5668_STO1_ADC_R_BST_SFT: c_int = 12;
// Sidetone Control (0x0024)

pub const RT5668_ST_SRC_SFT: c_int = 8;

pub const RT5668_ST_EN_SFT: c_int = 6;
// Stereo1 ADC Mixer Control (0x0026)

pub const RT5668_M_STO1_ADC_L1_SFT: c_int = 15;

pub const RT5668_M_STO1_ADC_L2_SFT: c_int = 14;

pub const RT5668_STO1_ADC1L_SRC_SFT: c_int = 13;

pub const RT5668_STO1_ADC2L_SRC_SFT: c_int = 12;

pub const RT5668_STO1_ADCL_SRC_SFT: c_int = 10;

pub const RT5668_STO1_DD_L_SRC_SFT: c_int = 9;

pub const RT5668_STO1_DMIC_SRC_SFT: c_int = 8;

pub const RT5668_M_STO1_ADC_R1_SFT: c_int = 7;

pub const RT5668_M_STO1_ADC_R2_SFT: c_int = 6;

pub const RT5668_STO1_ADC1R_SRC_SFT: c_int = 5;

pub const RT5668_STO1_ADC2R_SRC_SFT: c_int = 4;

pub const RT5668_STO1_ADCR_SRC_SFT: c_int = 2;
// ADC Mixer to DAC Mixer Control (0x0029)

pub const RT5668_M_ADCMIX_L_SFT: c_int = 15;

pub const RT5668_M_DAC1_L_SFT: c_int = 14;

pub const RT5668_DAC1_R_SEL_SFT: c_int = 10;

pub const RT5668_DAC1_L_SEL_SFT: c_int = 8;

pub const RT5668_M_ADCMIX_R_SFT: c_int = 7;

pub const RT5668_M_DAC1_R_SFT: c_int = 6;
// Stereo1 DAC Mixer Control (0x002a)

pub const RT5668_M_DAC_L1_STO_L_SFT: c_int = 15;

pub const RT5668_G_DAC_L1_STO_L_SFT: c_int = 14;

pub const RT5668_M_DAC_R1_STO_L_SFT: c_int = 13;

pub const RT5668_G_DAC_R1_STO_L_SFT: c_int = 12;

pub const RT5668_M_DAC_L1_STO_R_SFT: c_int = 7;

pub const RT5668_G_DAC_L1_STO_R_SFT: c_int = 6;

pub const RT5668_M_DAC_R1_STO_R_SFT: c_int = 5;

pub const RT5668_G_DAC_R1_STO_R_SFT: c_int = 4;
// Analog DAC1 Input Source Control (0x002b)

pub const RT5668_M_ST_STO_L_SFT: c_int = 9;

pub const RT5668_M_ST_STO_R_SFT: c_int = 8;

pub const RT5668_A_DACL1_SFT: c_int = 4;

pub const RT5668_A_DACR1_SFT: c_int = 0;
// Digital Interface Data Control (0x0030)

pub const RT5668_IF2_ADC_SEL_SFT: c_int = 0;
// REC Left Mixer Control 2 (0x003c)

pub const RT5668_G_CBJ_RM1_L_SFT: c_int = 10;

pub const RT5668_M_CBJ_RM1_L_SFT: c_int = 7;
// Power Management for Digital 1 (0x0061)

pub const RT5668_PWR_I2S1_BIT: c_int = 15;

pub const RT5668_PWR_I2S2_BIT: c_int = 14;

pub const RT5668_PWR_DAC_L1_BIT: c_int = 11;

pub const RT5668_PWR_DAC_R1_BIT: c_int = 10;

pub const RT5668_PWR_LDO_BIT: c_int = 8;

pub const RT5668_PWR_ADC_L1_BIT: c_int = 4;

pub const RT5668_PWR_ADC_R1_BIT: c_int = 3;

pub const RT5668_DIG_GATE_CTRL_SFT: c_int = 0;
// Power Management for Digital 2 (0x0062)

pub const RT5668_PWR_ADC_S1F_BIT: c_int = 15;

pub const RT5668_PWR_DAC_S1F_BIT: c_int = 10;
// Power Management for Analog 1 (0x0063)

pub const RT5668_PWR_VREF1_BIT: c_int = 15;

pub const RT5668_PWR_FV1_BIT: c_int = 14;

pub const RT5668_PWR_VREF2_BIT: c_int = 13;

pub const RT5668_PWR_FV2_BIT: c_int = 12;

pub const RT5668_PWR_MB_BIT: c_int = 9;

pub const RT5668_PWR_BG_BIT: c_int = 7;

pub const RT5668_PWR_MA_BIT: c_int = 6;

pub const RT5668_PWR_HA_L_BIT: c_int = 1;

pub const RT5668_PWR_HA_R_BIT: c_int = 0;
// Power Management for Analog 2 (0x0064)

pub const RT5668_PWR_MB1_BIT: c_int = 11;

pub const RT5668_PWR_MB2_BIT: c_int = 10;

pub const RT5668_PWR_JDH_BIT: c_int = 3;

pub const RT5668_PWR_JDL_BIT: c_int = 2;

pub const RT5668_PWR_RM1_L_BIT: c_int = 1;
// Power Management for Analog 3 (0x0065)

pub const RT5668_PWR_CBJ_BIT: c_int = 9;

pub const RT5668_PWR_PLL_BIT: c_int = 6;

pub const RT5668_PWR_PLL2B_BIT: c_int = 5;

pub const RT5668_PWR_PLL2F_BIT: c_int = 4;

pub const RT5668_PWR_LDO2_BIT: c_int = 2;

pub const RT5668_PWR_DET_SPKVDD_BIT: c_int = 1;
// Power Management for Mixer (0x0066)

pub const RT5668_PWR_STO1_DAC_L_BIT: c_int = 5;

pub const RT5668_PWR_STO1_DAC_R_BIT: c_int = 4;
// MCLK and System Clock Detection Control (0x006b)

pub const RT5668_SYS_CLK_DET_SFT: c_int = 15;

pub const RT5668_PLL1_CLK_DET_SFT: c_int = 14;

pub const RT5668_PLL2_CLK_DET_SFT: c_int = 13;
pub const RT5668_POW_CLK_DET2_SFT: c_int = 8;
pub const RT5668_POW_CLK_DET_SFT: c_int = 0;
// Digital Microphone Control 1 (0x006e)

pub const RT5668_DMIC_1_EN_SFT: c_int = 15;

pub const RT5668_DMIC_1_DP_SFT: c_int = 4;

pub const RT5668_DMIC_CLK_SFT: c_int = 0;
// I2S1 Audio Serial Data Port Control (0x0070)

pub const RT5668_SEL_ADCDAT_SFT: c_int = 15;

pub const RT5668_I2S1_TX_CHL_SFT: c_int = 12;

pub const RT5668_I2S1_RX_CHL_SFT: c_int = 8;

pub const RT5668_I2S1_DL_SFT: c_int = 4;

// I2S1/2 Audio Serial Data Port Control (0x0070)(0x0071)

pub const RT5668_I2S2_MS_SFT: c_int = 15;

pub const RT5668_I2S2_PIN_CFG_SFT: c_int = 14;

pub const RT5668_I2S2_CLK_SEL_SFT: c_int = 11;

pub const RT5668_I2S2_OUT_SFT: c_int = 9;

pub const RT5668_I2S_BP_SFT: c_int = 8;

pub const RT5668_I2S2_DL_SFT: c_int = 4;

pub const RT5668_I2S_DF_SFT: c_int = 0;

// ADC/DAC Clock Control 1 (0x0073)

pub const RT5668_ADC_OSR_SFT: c_int = 12;

pub const RT5668_I2S_M_DIV_SFT: c_int = 8;

pub const RT5668_I2S_CLK_SRC_SFT: c_int = 4;

pub const RT5668_DAC_OSR_SFT: c_int = 0;

// ADC/DAC Clock Control 2 (0x0074)

pub const RT5668_I2S2_BCLK_MS2_SFT: c_int = 11;

// TDM control 1 (0x0079)

pub const RT5668_TDM_ADC_LCA_SFT: c_int = 4;
pub const RT5668_TDM_ADC_DL_SFT: c_int = 0;
// TDM control 3 (0x007a)
pub const RT5668_IF1_ADC1_SEL_SFT: c_int = 14;
pub const RT5668_IF1_ADC2_SEL_SFT: c_int = 12;
pub const RT5668_IF1_ADC3_SEL_SFT: c_int = 10;
pub const RT5668_IF1_ADC4_SEL_SFT: c_int = 8;
pub const RT5668_TDM_ADC_SEL_SFT: c_int = 4;
// TDM/I2S control (0x007e)

pub const RT5668_TDM_S_BP_SFT: c_int = 15;

pub const RT5668_TDM_S_LP_SFT: c_int = 14;

pub const RT5668_TDM_DF_SFT: c_int = 11;

pub const RT5668_TDM_M_BP_SFT: c_int = 2;

pub const RT5668_TDM_M_LP_SFT: c_int = 1;

pub const RT5668_TDM_MS_SFT: c_int = 0;

// Global Clock Control (0x0080)

pub const RT5668_SCLK_SRC_SFT: c_int = 13;

pub const RT5668_PLL1_SRC_SFT: c_int = 10;

pub const RT5668_PLL2_SRC_SFT: c_int = 8;

pub const RT5668_PLL_INP_MAX: c_int = 40000000;
pub const RT5668_PLL_INP_MIN: c_int = 256000;
// PLL M/N/K Code Control 1 (0x0081)
pub const RT5668_PLL_N_MAX: c_uint = 0x001ff;

pub const RT5668_PLL_N_SFT: c_int = 7;
pub const RT5668_PLL_K_MAX: c_uint = 0x001f;

pub const RT5668_PLL_K_SFT: c_int = 0;
// PLL M/N/K Code Control 2 (0x0082)
pub const RT5668_PLL_M_MAX: c_uint = 0x00f;

pub const RT5668_PLL_M_SFT: c_int = 12;

pub const RT5668_PLL_M_BP_SFT: c_int = 11;

pub const RT5668_PLL_K_BP_SFT: c_int = 10;
// PLL tracking mode 1 (0x0083)

pub const RT5668_DA_ASRC_SFT: c_int = 13;

pub const RT5668_DAC_STO1_ASRC_SFT: c_int = 12;

pub const RT5668_AD_ASRC_SFT: c_int = 8;

pub const RT5668_AD_ASRC_SEL_SFT: c_int = 4;

pub const RT5668_DMIC_ASRC_SFT: c_int = 3;

pub const RT5668_ADC_STO1_ASRC_SFT: c_int = 2;

pub const RT5668_DA_ASRC_SEL_SFT: c_int = 0;
// PLL tracking mode 2 3 (0x0084)(0x0085)

pub const RT5668_FILTER_CLK_SEL_SFT: c_int = 12;
// ASRC Control 4 (0x0086)

pub const RT5668_ASRCIN_FTK_N1_SFT: c_int = 14;

pub const RT5668_ASRCIN_FTK_N2_SFT: c_int = 12;

pub const RT5668_ASRCIN_FTK_M1_SFT: c_int = 8;

pub const RT5668_ASRCIN_FTK_M2_SFT: c_int = 4;
// SoundWire reference clk (0x008d)

pub const RT5668_SDW_REF_2_SFT: c_int = 4;

pub const RT5668_SDW_REF_1_SFT: c_int = 0;

// Depop Mode Control 1 (0x008e)

pub const RT5668_PUMP_EN_SFT: c_int = 3;

pub const RT5668_CAPLESS_EN_SFT: c_int = 0;
// Depop Mode Control 2 (0x8f)

pub const RT5668_RAMP_SFT: c_int = 12;

pub const RT5668_BPS_SFT: c_int = 11;

pub const RT5668_FAST_UPDN_SFT: c_int = 10;

pub const RT5668_VLO_SFT: c_int = 7;

// HPOUT charge pump 1 (0x0091)

pub const RT5668_OSW_L_SFT: c_int = 11;

pub const RT5668_OSW_R_SFT: c_int = 10;

pub const RT5668_PM_HP_SFT: c_int = 8;

pub const RT5668_IB_HP_SFT: c_int = 6;

// Micbias Control1 (0x93)

pub const RT5668_MIC1_OV_SFT: c_int = 14;

pub const RT5668_MIC1_CLK_SFT: c_int = 13;

pub const RT5668_MIC1_OVCD_SFT: c_int = 12;

pub const RT5668_MIC1_OVTH_SFT: c_int = 10;

pub const RT5668_MIC2_OV_SFT: c_int = 8;

pub const RT5668_MIC2_CLK_SFT: c_int = 7;

pub const RT5668_MIC2_OVTH_SFT: c_int = 4;

pub const RT5668_PWR_MB_SFT: c_int = 3;

// Micbias Control2 (0x0094)

pub const RT5668_PWR_CLK25M_SFT: c_int = 9;

pub const RT5668_PWR_CLK1M_SFT: c_int = 8;

// RC Clock Control (0x009f)

// I2S Master Mode Clock Control 1 (0x00a0)

pub const RT5668_I2S2_SRC_SFT: c_int = 4;

pub const RT5668_I2S2_M_PD_SFT: c_int = 0;
// IRQ Control 1 (0x00b6)

pub const RT5668_JD1_PULSE_EN_SFT: c_int = 10;

// IRQ Control 2 (0x00b7)

pub const RT5668_JD1_EN_SFT: c_int = 15;

// IRQ Control 3 (0x00b8)

// GPIO Control 1 (0x00c0)

pub const RT5668_GP1_PIN_SFT: c_int = 14;

pub const RT5668_GP2_PIN_SFT: c_int = 12;

pub const RT5668_GP3_PIN_SFT: c_int = 10;

pub const RT5668_GP4_PIN_SFT: c_int = 8;

pub const RT5668_GP5_PIN_SFT: c_int = 6;

pub const RT5668_GP6_PIN_SFT: c_int = 5;

// GPIO Control 2 (0x00c1)

// GPIO Status (0x00c2)

// Soft volume and zero cross control 1 (0x00d9)

pub const RT5668_SV_SFT: c_int = 15;

pub const RT5668_ZCD_SFT: c_int = 10;

pub const RT5668_SV_DLY_SFT: c_int = 0;
// Soft volume and zero cross control 2 (0x00da)

pub const RT5668_ZCD_BST1_CBJ_SFT: c_int = 7;

pub const RT5668_ZCD_RECMIX_SFT: c_int = 0;

// 4 Button Inline Command Control 2 (0x00e3)

// Analog JD Control (0x00f0)

// Chopper and Clock control for DAC (0x013a)

pub const RT5668_CKXEN_DAC1_SFT: c_int = 13;

pub const RT5668_CKGEN_DAC1_SFT: c_int = 12;
// Chopper and Clock control for ADC (0x013b)

pub const RT5668_CKXEN_ADC1_SFT: c_int = 13;

pub const RT5668_CKGEN_ADC1_SFT: c_int = 12;
// Volume test (0x013f)

// Test Mode Control 1 (0x0145)

pub const RT5668_AD2DA_LB_SFT: c_int = 10;
// Stereo Noise Gate Control 1 (0x0160)

// Stereo1 DAC Silence Detection Control (0x0190)

// SAR ADC Inline Command Control 1 (0x0210)

// SAR ADC Inline Command Control 13 (0x021c)

// System Clock Source
// PLL Source
// filter mask
