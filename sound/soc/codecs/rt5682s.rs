//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5682s.h
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
// rt5682s.h  --  RT5682I-VS ALSA SoC audio driver
//
// Copyright 2021 Realtek Microelectronics
// Author: Derek Fang <derek.fang@realtek.com>
//

// Info
pub const RT5682S_RESET: c_uint = 0x0000;
pub const RT5682S_VERSION_ID: c_uint = 0x00fd;
pub const RT5682S_VENDOR_ID: c_uint = 0x00fe;
pub const RT5682S_DEVICE_ID: c_uint = 0x00ff;
// I/O - Output
pub const RT5682S_HP_CTRL_1: c_uint = 0x0002;
pub const RT5682S_HP_CTRL_2: c_uint = 0x0003;
pub const RT5682S_HPL_GAIN: c_uint = 0x0005;
pub const RT5682S_HPR_GAIN: c_uint = 0x0006;
pub const RT5682S_I2C_CTRL: c_uint = 0x0008;
// I/O - Input
pub const RT5682S_CBJ_BST_CTRL: c_uint = 0x000b;
pub const RT5682S_CBJ_DET_CTRL: c_uint = 0x000f;
pub const RT5682S_CBJ_CTRL_1: c_uint = 0x0010;
pub const RT5682S_CBJ_CTRL_2: c_uint = 0x0011;
pub const RT5682S_CBJ_CTRL_3: c_uint = 0x0012;
pub const RT5682S_CBJ_CTRL_4: c_uint = 0x0013;
pub const RT5682S_CBJ_CTRL_5: c_uint = 0x0014;
pub const RT5682S_CBJ_CTRL_6: c_uint = 0x0015;
pub const RT5682S_CBJ_CTRL_7: c_uint = 0x0016;
pub const RT5682S_CBJ_CTRL_8: c_uint = 0x0017;
// I/O - ADC/DAC/DMIC
pub const RT5682S_DAC1_DIG_VOL: c_uint = 0x0019;
pub const RT5682S_STO1_ADC_DIG_VOL: c_uint = 0x001c;
pub const RT5682S_STO1_ADC_BOOST: c_uint = 0x001f;
pub const RT5682S_HP_IMP_GAIN_1: c_uint = 0x0022;
pub const RT5682S_HP_IMP_GAIN_2: c_uint = 0x0023;
// Mixer - D-D
pub const RT5682S_SIDETONE_CTRL: c_uint = 0x0024;
pub const RT5682S_STO1_ADC_MIXER: c_uint = 0x0026;
pub const RT5682S_AD_DA_MIXER: c_uint = 0x0029;
pub const RT5682S_STO1_DAC_MIXER: c_uint = 0x002a;
pub const RT5682S_A_DAC1_MUX: c_uint = 0x002b;
pub const RT5682S_DIG_INF2_DATA: c_uint = 0x0030;
// Mixer - ADC
pub const RT5682S_REC_MIXER: c_uint = 0x003c;
pub const RT5682S_CAL_REC: c_uint = 0x0044;
// HP Analog Offset Control
pub const RT5682S_HP_ANA_OST_CTRL_1: c_uint = 0x004b;
pub const RT5682S_HP_ANA_OST_CTRL_2: c_uint = 0x004c;
pub const RT5682S_HP_ANA_OST_CTRL_3: c_uint = 0x004d;
// Power
pub const RT5682S_PWR_DIG_1: c_uint = 0x0061;
pub const RT5682S_PWR_DIG_2: c_uint = 0x0062;
pub const RT5682S_PWR_ANLG_1: c_uint = 0x0063;
pub const RT5682S_PWR_ANLG_2: c_uint = 0x0064;
pub const RT5682S_PWR_ANLG_3: c_uint = 0x0065;
pub const RT5682S_PWR_MIXER: c_uint = 0x0066;
pub const RT5682S_MB_CTRL: c_uint = 0x0067;
pub const RT5682S_CLK_GATE_TCON_1: c_uint = 0x0068;
pub const RT5682S_CLK_GATE_TCON_2: c_uint = 0x0069;
pub const RT5682S_CLK_GATE_TCON_3: c_uint = 0x006a;
// Clock Detect
pub const RT5682S_CLK_DET: c_uint = 0x006b;
// Filter Auto Reset
pub const RT5682S_RESET_LPF_CTRL: c_uint = 0x006c;
pub const RT5682S_RESET_HPF_CTRL: c_uint = 0x006d;
// DMIC
pub const RT5682S_DMIC_CTRL_1: c_uint = 0x006e;
pub const RT5682S_LPF_AD_DMIC: c_uint = 0x006f;
// Format - ADC/DAC
pub const RT5682S_I2S1_SDP: c_uint = 0x0070;
pub const RT5682S_I2S2_SDP: c_uint = 0x0071;
pub const RT5682S_ADDA_CLK_1: c_uint = 0x0073;
pub const RT5682S_ADDA_CLK_2: c_uint = 0x0074;
pub const RT5682S_I2S1_F_DIV_CTRL_1: c_uint = 0x0075;
pub const RT5682S_I2S1_F_DIV_CTRL_2: c_uint = 0x0076;
// Format - TDM Control
pub const RT5682S_TDM_CTRL: c_uint = 0x0079;
pub const RT5682S_TDM_ADDA_CTRL_1: c_uint = 0x007a;
pub const RT5682S_TDM_ADDA_CTRL_2: c_uint = 0x007b;
pub const RT5682S_DATA_SEL_CTRL_1: c_uint = 0x007c;
pub const RT5682S_TDM_TCON_CTRL_1: c_uint = 0x007e;
pub const RT5682S_TDM_TCON_CTRL_2: c_uint = 0x007f;
// Function - Analog
pub const RT5682S_GLB_CLK: c_uint = 0x0080;
pub const RT5682S_PLL_TRACK_1: c_uint = 0x0083;
pub const RT5682S_PLL_TRACK_2: c_uint = 0x0084;
pub const RT5682S_PLL_TRACK_3: c_uint = 0x0085;
pub const RT5682S_PLL_TRACK_4: c_uint = 0x0086;
pub const RT5682S_PLL_TRACK_5: c_uint = 0x0087;
pub const RT5682S_PLL_TRACK_6: c_uint = 0x0088;
pub const RT5682S_PLL_TRACK_11: c_uint = 0x008c;
pub const RT5682S_DEPOP_1: c_uint = 0x008e;
pub const RT5682S_HP_CHARGE_PUMP_1: c_uint = 0x008f;
pub const RT5682S_HP_CHARGE_PUMP_2: c_uint = 0x0091;
pub const RT5682S_HP_CHARGE_PUMP_3: c_uint = 0x0092;
pub const RT5682S_MICBIAS_1: c_uint = 0x0093;
pub const RT5682S_MICBIAS_2: c_uint = 0x0094;
pub const RT5682S_MICBIAS_3: c_uint = 0x0095;
pub const RT5682S_PLL_TRACK_12: c_uint = 0x0096;
pub const RT5682S_PLL_TRACK_14: c_uint = 0x0097;
pub const RT5682S_PLL_CTRL_1: c_uint = 0x0098;
pub const RT5682S_PLL_CTRL_2: c_uint = 0x0099;
pub const RT5682S_PLL_CTRL_3: c_uint = 0x009a;
pub const RT5682S_PLL_CTRL_4: c_uint = 0x009b;
pub const RT5682S_PLL_CTRL_5: c_uint = 0x009c;
pub const RT5682S_PLL_CTRL_6: c_uint = 0x009d;
pub const RT5682S_PLL_CTRL_7: c_uint = 0x009e;
pub const RT5682S_RC_CLK_CTRL: c_uint = 0x009f;
pub const RT5682S_I2S2_M_CLK_CTRL_1: c_uint = 0x00a0;
pub const RT5682S_I2S2_F_DIV_CTRL_1: c_uint = 0x00a3;
pub const RT5682S_I2S2_F_DIV_CTRL_2: c_uint = 0x00a4;
pub const RT5682S_IRQ_CTRL_1: c_uint = 0x00b6;
pub const RT5682S_IRQ_CTRL_2: c_uint = 0x00b7;
pub const RT5682S_IRQ_CTRL_3: c_uint = 0x00b8;
pub const RT5682S_IRQ_CTRL_4: c_uint = 0x00b9;
pub const RT5682S_INT_ST_1: c_uint = 0x00be;
pub const RT5682S_GPIO_CTRL_1: c_uint = 0x00c0;
pub const RT5682S_GPIO_CTRL_2: c_uint = 0x00c1;
pub const RT5682S_GPIO_ST: c_uint = 0x00c2;
pub const RT5682S_HP_AMP_DET_CTRL_1: c_uint = 0x00d0;
pub const RT5682S_MID_HP_AMP_DET: c_uint = 0x00d2;
pub const RT5682S_LOW_HP_AMP_DET: c_uint = 0x00d3;
pub const RT5682S_DELAY_BUF_CTRL: c_uint = 0x00d4;
pub const RT5682S_SV_ZCD_1: c_uint = 0x00d9;
pub const RT5682S_SV_ZCD_2: c_uint = 0x00da;
pub const RT5682S_IL_CMD_1: c_uint = 0x00db;
pub const RT5682S_IL_CMD_2: c_uint = 0x00dc;
pub const RT5682S_IL_CMD_3: c_uint = 0x00dd;
pub const RT5682S_IL_CMD_4: c_uint = 0x00de;
pub const RT5682S_IL_CMD_5: c_uint = 0x00df;
pub const RT5682S_IL_CMD_6: c_uint = 0x00e0;
pub const RT5682S_4BTN_IL_CMD_1: c_uint = 0x00e2;
pub const RT5682S_4BTN_IL_CMD_2: c_uint = 0x00e3;
pub const RT5682S_4BTN_IL_CMD_3: c_uint = 0x00e4;
pub const RT5682S_4BTN_IL_CMD_4: c_uint = 0x00e5;
pub const RT5682S_4BTN_IL_CMD_5: c_uint = 0x00e6;
pub const RT5682S_4BTN_IL_CMD_6: c_uint = 0x00e7;
pub const RT5682S_4BTN_IL_CMD_7: c_uint = 0x00e8;
pub const RT5682S_ADC_STO1_HP_CTRL_1: c_uint = 0x00ea;
pub const RT5682S_ADC_STO1_HP_CTRL_2: c_uint = 0x00eb;
pub const RT5682S_AJD1_CTRL: c_uint = 0x00f0;
pub const RT5682S_JD_CTRL_1: c_uint = 0x00f6;
// General Control
pub const RT5682S_DUMMY_1: c_uint = 0x00fa;
pub const RT5682S_DUMMY_2: c_uint = 0x00fb;
pub const RT5682S_DUMMY_3: c_uint = 0x00fc;
pub const RT5682S_DAC_ADC_DIG_VOL1: c_uint = 0x0100;
pub const RT5682S_BIAS_CUR_CTRL_2: c_uint = 0x010b;
pub const RT5682S_BIAS_CUR_CTRL_3: c_uint = 0x010c;
pub const RT5682S_BIAS_CUR_CTRL_4: c_uint = 0x010d;
pub const RT5682S_BIAS_CUR_CTRL_5: c_uint = 0x010e;
pub const RT5682S_BIAS_CUR_CTRL_6: c_uint = 0x010f;
pub const RT5682S_BIAS_CUR_CTRL_7: c_uint = 0x0110;
pub const RT5682S_BIAS_CUR_CTRL_8: c_uint = 0x0111;
pub const RT5682S_BIAS_CUR_CTRL_9: c_uint = 0x0112;
pub const RT5682S_BIAS_CUR_CTRL_10: c_uint = 0x0113;
pub const RT5682S_VREF_REC_OP_FB_CAP_CTRL_1: c_uint = 0x0117;
pub const RT5682S_VREF_REC_OP_FB_CAP_CTRL_2: c_uint = 0x0118;
pub const RT5682S_CHARGE_PUMP_1: c_uint = 0x0125;
pub const RT5682S_DIG_IN_CTRL_1: c_uint = 0x0132;
pub const RT5682S_PAD_DRIVING_CTRL: c_uint = 0x0136;
pub const RT5682S_CHOP_DAC_1: c_uint = 0x0139;
pub const RT5682S_CHOP_DAC_2: c_uint = 0x013a;
pub const RT5682S_CHOP_ADC: c_uint = 0x013b;
pub const RT5682S_CALIB_ADC_CTRL: c_uint = 0x013c;
pub const RT5682S_VOL_TEST: c_uint = 0x013f;
pub const RT5682S_SPKVDD_DET_ST: c_uint = 0x0142;
pub const RT5682S_TEST_MODE_CTRL_1: c_uint = 0x0145;
pub const RT5682S_TEST_MODE_CTRL_2: c_uint = 0x0146;
pub const RT5682S_TEST_MODE_CTRL_3: c_uint = 0x0147;
pub const RT5682S_TEST_MODE_CTRL_4: c_uint = 0x0148;
pub const RT5682S_PLL_INTERNAL_1: c_uint = 0x0156;
pub const RT5682S_PLL_INTERNAL_2: c_uint = 0x0157;
pub const RT5682S_PLL_INTERNAL_3: c_uint = 0x0158;
pub const RT5682S_PLL_INTERNAL_4: c_uint = 0x0159;
pub const RT5682S_STO_NG2_CTRL_1: c_uint = 0x0160;
pub const RT5682S_STO_NG2_CTRL_2: c_uint = 0x0161;
pub const RT5682S_STO_NG2_CTRL_3: c_uint = 0x0162;
pub const RT5682S_STO_NG2_CTRL_4: c_uint = 0x0163;
pub const RT5682S_STO_NG2_CTRL_5: c_uint = 0x0164;
pub const RT5682S_STO_NG2_CTRL_6: c_uint = 0x0165;
pub const RT5682S_STO_NG2_CTRL_7: c_uint = 0x0166;
pub const RT5682S_STO_NG2_CTRL_8: c_uint = 0x0167;
pub const RT5682S_STO_NG2_CTRL_9: c_uint = 0x0168;
pub const RT5682S_STO_NG2_CTRL_10: c_uint = 0x0169;
pub const RT5682S_STO1_DAC_SIL_DET: c_uint = 0x0190;
pub const RT5682S_SIL_PSV_CTRL1: c_uint = 0x0194;
pub const RT5682S_SIL_PSV_CTRL2: c_uint = 0x0195;
pub const RT5682S_SIL_PSV_CTRL3: c_uint = 0x0197;
pub const RT5682S_SIL_PSV_CTRL4: c_uint = 0x0198;
pub const RT5682S_SIL_PSV_CTRL5: c_uint = 0x0199;
pub const RT5682S_HP_IMP_SENS_CTRL_1: c_uint = 0x01ac;
pub const RT5682S_HP_IMP_SENS_CTRL_2: c_uint = 0x01ad;
pub const RT5682S_HP_IMP_SENS_CTRL_3: c_uint = 0x01ae;
pub const RT5682S_HP_IMP_SENS_CTRL_4: c_uint = 0x01af;
pub const RT5682S_HP_IMP_SENS_CTRL_5: c_uint = 0x01b0;
pub const RT5682S_HP_IMP_SENS_CTRL_6: c_uint = 0x01b1;
pub const RT5682S_HP_IMP_SENS_CTRL_7: c_uint = 0x01b2;
pub const RT5682S_HP_IMP_SENS_CTRL_8: c_uint = 0x01b3;
pub const RT5682S_HP_IMP_SENS_CTRL_9: c_uint = 0x01b4;
pub const RT5682S_HP_IMP_SENS_CTRL_10: c_uint = 0x01b5;
pub const RT5682S_HP_IMP_SENS_CTRL_11: c_uint = 0x01b6;
pub const RT5682S_HP_IMP_SENS_CTRL_12: c_uint = 0x01b7;
pub const RT5682S_HP_IMP_SENS_CTRL_13: c_uint = 0x01b8;
pub const RT5682S_HP_IMP_SENS_CTRL_14: c_uint = 0x01b9;
pub const RT5682S_HP_IMP_SENS_CTRL_15: c_uint = 0x01ba;
pub const RT5682S_HP_IMP_SENS_CTRL_16: c_uint = 0x01bb;
pub const RT5682S_HP_IMP_SENS_CTRL_17: c_uint = 0x01bc;
pub const RT5682S_HP_IMP_SENS_CTRL_18: c_uint = 0x01bd;
pub const RT5682S_HP_IMP_SENS_CTRL_19: c_uint = 0x01be;
pub const RT5682S_HP_IMP_SENS_CTRL_20: c_uint = 0x01bf;
pub const RT5682S_HP_IMP_SENS_CTRL_21: c_uint = 0x01c0;
pub const RT5682S_HP_IMP_SENS_CTRL_22: c_uint = 0x01c1;
pub const RT5682S_HP_IMP_SENS_CTRL_23: c_uint = 0x01c2;
pub const RT5682S_HP_IMP_SENS_CTRL_24: c_uint = 0x01c3;
pub const RT5682S_HP_IMP_SENS_CTRL_25: c_uint = 0x01c4;
pub const RT5682S_HP_IMP_SENS_CTRL_26: c_uint = 0x01c5;
pub const RT5682S_HP_IMP_SENS_CTRL_27: c_uint = 0x01c6;
pub const RT5682S_HP_IMP_SENS_CTRL_28: c_uint = 0x01c7;
pub const RT5682S_HP_IMP_SENS_CTRL_29: c_uint = 0x01c8;
pub const RT5682S_HP_IMP_SENS_CTRL_30: c_uint = 0x01c9;
pub const RT5682S_HP_IMP_SENS_CTRL_31: c_uint = 0x01ca;
pub const RT5682S_HP_IMP_SENS_CTRL_32: c_uint = 0x01cb;
pub const RT5682S_HP_IMP_SENS_CTRL_33: c_uint = 0x01cc;
pub const RT5682S_HP_IMP_SENS_CTRL_34: c_uint = 0x01cd;
pub const RT5682S_HP_IMP_SENS_CTRL_35: c_uint = 0x01ce;
pub const RT5682S_HP_IMP_SENS_CTRL_36: c_uint = 0x01cf;
pub const RT5682S_HP_IMP_SENS_CTRL_37: c_uint = 0x01d0;
pub const RT5682S_HP_IMP_SENS_CTRL_38: c_uint = 0x01d1;
pub const RT5682S_HP_IMP_SENS_CTRL_39: c_uint = 0x01d2;
pub const RT5682S_HP_IMP_SENS_CTRL_40: c_uint = 0x01d3;
pub const RT5682S_HP_IMP_SENS_CTRL_41: c_uint = 0x01d4;
pub const RT5682S_HP_IMP_SENS_CTRL_42: c_uint = 0x01d5;
pub const RT5682S_HP_IMP_SENS_CTRL_43: c_uint = 0x01d6;
pub const RT5682S_HP_IMP_SENS_CTRL_44: c_uint = 0x01d7;
pub const RT5682S_HP_IMP_SENS_CTRL_45: c_uint = 0x01d8;
pub const RT5682S_HP_IMP_SENS_CTRL_46: c_uint = 0x01d9;
pub const RT5682S_HP_LOGIC_CTRL_1: c_uint = 0x01da;
pub const RT5682S_HP_LOGIC_CTRL_2: c_uint = 0x01db;
pub const RT5682S_HP_LOGIC_CTRL_3: c_uint = 0x01dc;
pub const RT5682S_HP_CALIB_CTRL_1: c_uint = 0x01de;
pub const RT5682S_HP_CALIB_CTRL_2: c_uint = 0x01df;
pub const RT5682S_HP_CALIB_CTRL_3: c_uint = 0x01e0;
pub const RT5682S_HP_CALIB_CTRL_4: c_uint = 0x01e1;
pub const RT5682S_HP_CALIB_CTRL_5: c_uint = 0x01e2;
pub const RT5682S_HP_CALIB_CTRL_6: c_uint = 0x01e3;
pub const RT5682S_HP_CALIB_CTRL_7: c_uint = 0x01e4;
pub const RT5682S_HP_CALIB_CTRL_8: c_uint = 0x01e5;
pub const RT5682S_HP_CALIB_CTRL_9: c_uint = 0x01e6;
pub const RT5682S_HP_CALIB_CTRL_10: c_uint = 0x01e7;
pub const RT5682S_HP_CALIB_CTRL_11: c_uint = 0x01e8;
pub const RT5682S_HP_CALIB_ST_1: c_uint = 0x01ea;
pub const RT5682S_HP_CALIB_ST_2: c_uint = 0x01eb;
pub const RT5682S_HP_CALIB_ST_3: c_uint = 0x01ec;
pub const RT5682S_HP_CALIB_ST_4: c_uint = 0x01ed;
pub const RT5682S_HP_CALIB_ST_5: c_uint = 0x01ee;
pub const RT5682S_HP_CALIB_ST_6: c_uint = 0x01ef;
pub const RT5682S_HP_CALIB_ST_7: c_uint = 0x01f0;
pub const RT5682S_HP_CALIB_ST_8: c_uint = 0x01f1;
pub const RT5682S_HP_CALIB_ST_9: c_uint = 0x01f2;
pub const RT5682S_HP_CALIB_ST_10: c_uint = 0x01f3;
pub const RT5682S_HP_CALIB_ST_11: c_uint = 0x01f4;
pub const RT5682S_SAR_IL_CMD_1: c_uint = 0x0210;
pub const RT5682S_SAR_IL_CMD_2: c_uint = 0x0211;
pub const RT5682S_SAR_IL_CMD_3: c_uint = 0x0212;
pub const RT5682S_SAR_IL_CMD_4: c_uint = 0x0213;
pub const RT5682S_SAR_IL_CMD_5: c_uint = 0x0214;
pub const RT5682S_SAR_IL_CMD_6: c_uint = 0x0215;
pub const RT5682S_SAR_IL_CMD_7: c_uint = 0x0216;
pub const RT5682S_SAR_IL_CMD_8: c_uint = 0x0217;
pub const RT5682S_SAR_IL_CMD_9: c_uint = 0x0218;
pub const RT5682S_SAR_IL_CMD_10: c_uint = 0x0219;
pub const RT5682S_SAR_IL_CMD_11: c_uint = 0x021a;
pub const RT5682S_SAR_IL_CMD_12: c_uint = 0x021b;
pub const RT5682S_SAR_IL_CMD_13: c_uint = 0x021c;
pub const RT5682S_SAR_IL_CMD_14: c_uint = 0x021d;
pub const RT5682S_DUMMY_4: c_uint = 0x02fa;
pub const RT5682S_DUMMY_5: c_uint = 0x02fb;
pub const RT5682S_DUMMY_6: c_uint = 0x02fc;
pub const RT5682S_VERSION_ID_HIDE: c_uint = 0x03fe;
pub const RT5682S_VERSION_ID_CUS: c_uint = 0x03ff;
pub const RT5682S_SCAN_CTL: c_uint = 0x0500;
pub const RT5682S_HP_AMP_DET: c_uint = 0x0600;
pub const RT5682S_BIAS_CUR_CTRL_11: c_uint = 0x0610;
pub const RT5682S_BIAS_CUR_CTRL_12: c_uint = 0x0611;
pub const RT5682S_BIAS_CUR_CTRL_13: c_uint = 0x0620;
pub const RT5682S_BIAS_CUR_CTRL_14: c_uint = 0x0621;
pub const RT5682S_BIAS_CUR_CTRL_15: c_uint = 0x0630;
pub const RT5682S_BIAS_CUR_CTRL_16: c_uint = 0x0631;
pub const RT5682S_BIAS_CUR_CTRL_17: c_uint = 0x0640;
pub const RT5682S_BIAS_CUR_CTRL_18: c_uint = 0x0641;
pub const RT5682S_I2C_TRANS_CTRL: c_uint = 0x07fa;
pub const RT5682S_DUMMY_7: c_uint = 0x08fa;
pub const RT5682S_DUMMY_8: c_uint = 0x08fb;
pub const RT5682S_DMIC_FLOAT_DET: c_uint = 0x0d00;
pub const RT5682S_HA_CMP_OP_1: c_uint = 0x1100;
pub const RT5682S_HA_CMP_OP_2: c_uint = 0x1101;
pub const RT5682S_HA_CMP_OP_3: c_uint = 0x1102;
pub const RT5682S_HA_CMP_OP_4: c_uint = 0x1103;
pub const RT5682S_HA_CMP_OP_5: c_uint = 0x1104;
pub const RT5682S_HA_CMP_OP_6: c_uint = 0x1105;
pub const RT5682S_HA_CMP_OP_7: c_uint = 0x1106;
pub const RT5682S_HA_CMP_OP_8: c_uint = 0x1107;
pub const RT5682S_HA_CMP_OP_9: c_uint = 0x1108;
pub const RT5682S_HA_CMP_OP_10: c_uint = 0x1109;
pub const RT5682S_HA_CMP_OP_11: c_uint = 0x110a;
pub const RT5682S_HA_CMP_OP_12: c_uint = 0x110b;
pub const RT5682S_HA_CMP_OP_13: c_uint = 0x110c;
pub const RT5682S_HA_CMP_OP_14: c_uint = 0x1111;
pub const RT5682S_HA_CMP_OP_15: c_uint = 0x1112;
pub const RT5682S_HA_CMP_OP_16: c_uint = 0x1113;
pub const RT5682S_HA_CMP_OP_17: c_uint = 0x1114;
pub const RT5682S_HA_CMP_OP_18: c_uint = 0x1115;
pub const RT5682S_HA_CMP_OP_19: c_uint = 0x1116;
pub const RT5682S_HA_CMP_OP_20: c_uint = 0x1117;
pub const RT5682S_HA_CMP_OP_21: c_uint = 0x1118;
pub const RT5682S_HA_CMP_OP_22: c_uint = 0x1119;
pub const RT5682S_HA_CMP_OP_23: c_uint = 0x111a;
pub const RT5682S_HA_CMP_OP_24: c_uint = 0x111b;
pub const RT5682S_HA_CMP_OP_25: c_uint = 0x111c;
pub const RT5682S_NEW_CBJ_DET_CTL_1: c_uint = 0x1401;
pub const RT5682S_NEW_CBJ_DET_CTL_2: c_uint = 0x1402;
pub const RT5682S_NEW_CBJ_DET_CTL_3: c_uint = 0x1403;
pub const RT5682S_NEW_CBJ_DET_CTL_4: c_uint = 0x1404;
pub const RT5682S_NEW_CBJ_DET_CTL_5: c_uint = 0x1406;
pub const RT5682S_NEW_CBJ_DET_CTL_6: c_uint = 0x1407;
pub const RT5682S_NEW_CBJ_DET_CTL_7: c_uint = 0x1408;
pub const RT5682S_NEW_CBJ_DET_CTL_8: c_uint = 0x1409;
pub const RT5682S_NEW_CBJ_DET_CTL_9: c_uint = 0x140a;
pub const RT5682S_NEW_CBJ_DET_CTL_10: c_uint = 0x140b;
pub const RT5682S_NEW_CBJ_DET_CTL_11: c_uint = 0x140c;
pub const RT5682S_NEW_CBJ_DET_CTL_12: c_uint = 0x140d;
pub const RT5682S_NEW_CBJ_DET_CTL_13: c_uint = 0x140e;
pub const RT5682S_NEW_CBJ_DET_CTL_14: c_uint = 0x140f;
pub const RT5682S_NEW_CBJ_DET_CTL_15: c_uint = 0x1410;
pub const RT5682S_NEW_CBJ_DET_CTL_16: c_uint = 0x1411;
pub const RT5682S_DA_FILTER_1: c_uint = 0x1801;
pub const RT5682S_DA_FILTER_2: c_uint = 0x1802;
pub const RT5682S_DA_FILTER_3: c_uint = 0x1803;
pub const RT5682S_DA_FILTER_4: c_uint = 0x1804;
pub const RT5682S_DA_FILTER_5: c_uint = 0x1805;
pub const RT5682S_CLK_SW_TEST_1: c_uint = 0x2c00;
pub const RT5682S_CLK_SW_TEST_2: c_uint = 0x3400;
pub const RT5682S_CLK_SW_TEST_3: c_uint = 0x3404;
pub const RT5682S_CLK_SW_TEST_4: c_uint = 0x3405;
pub const RT5682S_CLK_SW_TEST_5: c_uint = 0x3406;
pub const RT5682S_CLK_SW_TEST_6: c_uint = 0x3407;
pub const RT5682S_CLK_SW_TEST_7: c_uint = 0x3408;
pub const RT5682S_CLK_SW_TEST_8: c_uint = 0x3409;
pub const RT5682S_CLK_SW_TEST_9: c_uint = 0x340a;
pub const RT5682S_CLK_SW_TEST_10: c_uint = 0x340b;
pub const RT5682S_CLK_SW_TEST_11: c_uint = 0x340c;
pub const RT5682S_CLK_SW_TEST_12: c_uint = 0x340d;
pub const RT5682S_CLK_SW_TEST_13: c_uint = 0x340e;
pub const RT5682S_CLK_SW_TEST_14: c_uint = 0x340f;
pub const RT5682S_EFUSE_MANU_WRITE_1: c_uint = 0x3410;
pub const RT5682S_EFUSE_MANU_WRITE_2: c_uint = 0x3411;
pub const RT5682S_EFUSE_MANU_WRITE_3: c_uint = 0x3412;
pub const RT5682S_EFUSE_MANU_WRITE_4: c_uint = 0x3413;
pub const RT5682S_EFUSE_MANU_WRITE_5: c_uint = 0x3414;
pub const RT5682S_EFUSE_MANU_WRITE_6: c_uint = 0x3415;
pub const RT5682S_EFUSE_READ_1: c_uint = 0x3424;
pub const RT5682S_EFUSE_READ_2: c_uint = 0x3425;
pub const RT5682S_EFUSE_READ_3: c_uint = 0x3426;
pub const RT5682S_EFUSE_READ_4: c_uint = 0x3427;
pub const RT5682S_EFUSE_READ_5: c_uint = 0x3428;
pub const RT5682S_EFUSE_READ_6: c_uint = 0x3429;
pub const RT5682S_EFUSE_READ_7: c_uint = 0x342a;
pub const RT5682S_EFUSE_READ_8: c_uint = 0x342b;
pub const RT5682S_EFUSE_READ_9: c_uint = 0x342c;
pub const RT5682S_EFUSE_READ_10: c_uint = 0x342d;
pub const RT5682S_EFUSE_READ_11: c_uint = 0x342e;
pub const RT5682S_EFUSE_READ_12: c_uint = 0x342f;
pub const RT5682S_EFUSE_READ_13: c_uint = 0x3430;
pub const RT5682S_EFUSE_READ_14: c_uint = 0x3431;
pub const RT5682S_EFUSE_READ_15: c_uint = 0x3432;
pub const RT5682S_EFUSE_READ_16: c_uint = 0x3433;
pub const RT5682S_EFUSE_READ_17: c_uint = 0x3434;
pub const RT5682S_EFUSE_READ_18: c_uint = 0x3435;
pub const RT5682S_EFUSE_TIMING_CTL_1: c_uint = 0x3440;
pub const RT5682S_EFUSE_TIMING_CTL_2: c_uint = 0x3441;
pub const RT5682S_PILOT_DIG_CTL_1: c_uint = 0x3500;
pub const RT5682S_PILOT_DIG_CTL_2: c_uint = 0x3501;
pub const RT5682S_HP_AMP_DET_CTL_1: c_uint = 0x3b00;
pub const RT5682S_HP_AMP_DET_CTL_2: c_uint = 0x3b01;
pub const RT5682S_HP_AMP_DET_CTL_3: c_uint = 0x3b02;
pub const RT5682S_HP_AMP_DET_CTL_4: c_uint = 0x3b03;

// global definition

pub const RT5682S_L_MUTE_SFT: c_int = 15;

pub const RT5682S_R_MUTE_SFT: c_int = 7;
pub const RT5682S_L_VOL_SFT: c_int = 8;
pub const RT5682S_R_VOL_SFT: c_int = 0;

// Headphone Amp Control 2 (0x0003)

// Headphone Amp L/R Analog Gain and Digital NG2 Gain Control (0x0005 0x0006)

pub const RT5682S_G_HP_SFT: c_int = 8;

pub const RT5682S_G_STO_DA_SFT: c_int = 0;
// Embeeded Jack and Type Detection Control 2 (0x0010)

pub const RT5682S_EMB_JD_EN_SFT: c_int = 15;

pub const RT5682S_JD_MODE_SFT: c_int = 13;

pub const RT5682S_DET_TYPE_SFT: c_int = 12;

pub const RT5682S_SEL_FAST_OFF_SFT: c_int = 9;

pub const RT5682S_MB1_PATH_BIT: c_int = 5;

pub const RT5682S_MB2_PATH_BIT: c_int = 4;

// Embeeded Jack and Type Detection Control 3 (0x0011)

pub const RT5682S_EXT_JD_SRC_SFT: c_int = 4;

// Combo Jack and Type Detection Control 4 (0x0012)

pub const RT5682S_CBJ_IN_BUF_BIT: c_int = 7;
// Combo Jack and Type Detection Control 5 (0x0013)

// Combo Jack and Type Detection Control 6 (0x0014)

// DAC1 Digital Volume (0x0019)

pub const RT5682S_DAC_L1_VOL_SFT: c_int = 8;

pub const RT5682S_DAC_R1_VOL_SFT: c_int = 0;
// ADC Digital Volume Control (0x001c)

pub const RT5682S_ADC_L_VOL_SFT: c_int = 8;

pub const RT5682S_ADC_R_VOL_SFT: c_int = 0;
// Stereo1 ADC Boost Gain Control (0x001f)

pub const RT5682S_STO1_ADC_L_BST_SFT: c_int = 14;

pub const RT5682S_STO1_ADC_R_BST_SFT: c_int = 12;
// Sidetone Control (0x0024)

pub const RT5682S_ST_SRC_SFT: c_int = 8;

pub const RT5682S_ST_EN_SFT: c_int = 6;
// Stereo1 ADC Mixer Control (0x0026)

pub const RT5682S_M_STO1_ADC_L1_SFT: c_int = 15;

pub const RT5682S_M_STO1_ADC_L2_SFT: c_int = 14;

pub const RT5682S_STO1_ADC1L_SRC_SFT: c_int = 13;

pub const RT5682S_STO1_ADC2L_SRC_SFT: c_int = 12;

pub const RT5682S_STO1_ADCL_SRC_SFT: c_int = 10;

pub const RT5682S_M_STO1_ADC_R1_SFT: c_int = 7;

pub const RT5682S_M_STO1_ADC_R2_SFT: c_int = 6;

pub const RT5682S_STO1_ADC1R_SRC_SFT: c_int = 5;

pub const RT5682S_STO1_ADC2R_SRC_SFT: c_int = 4;

pub const RT5682S_STO1_ADCR_SRC_SFT: c_int = 2;
// ADC Mixer to DAC Mixer Control (0x0029)

pub const RT5682S_M_ADCMIX_L_SFT: c_int = 15;

pub const RT5682S_M_DAC1_L_SFT: c_int = 14;

pub const RT5682S_M_ADCMIX_R_SFT: c_int = 7;

pub const RT5682S_M_DAC1_R_SFT: c_int = 6;
// Stereo1 DAC Mixer Control (0x002a)

pub const RT5682S_M_DAC_L1_STO_L_SFT: c_int = 15;

pub const RT5682S_G_DAC_L1_STO_L_SFT: c_int = 14;

pub const RT5682S_M_DAC_R1_STO_L_SFT: c_int = 13;

pub const RT5682S_G_DAC_R1_STO_L_SFT: c_int = 12;

pub const RT5682S_M_DAC_L1_STO_R_SFT: c_int = 7;

pub const RT5682S_G_DAC_L1_STO_R_SFT: c_int = 6;

pub const RT5682S_M_DAC_R1_STO_R_SFT: c_int = 5;

pub const RT5682S_G_DAC_R1_STO_R_SFT: c_int = 4;
// Analog DAC1 Input Source Control (0x002b)

pub const RT5682S_M_ST_STO_L_SFT: c_int = 9;

pub const RT5682S_M_ST_STO_R_SFT: c_int = 8;

pub const RT5682S_A_DACL1_SFT: c_int = 4;

pub const RT5682S_A_DACR1_SFT: c_int = 0;
// Digital Interface Data Control (0x0030)

pub const RT5682S_IF2_DAC_SEL_SFT: c_int = 2;

pub const RT5682S_IF2_ADC_SEL_SFT: c_int = 0;
// REC Left/Right Mixer Control 2 (0x003c)

pub const RT5682S_BST_CBJ_SFT: c_int = 8;

pub const RT5682S_M_CBJ_RM1_L_SFT: c_int = 7;

pub const RT5682S_M_CBJ_RM1_R_SFT: c_int = 6;
// REC Left/Right Mixer Calibration Control(0x0044)
pub const RT5682S_PWR_RM1_R_BIT: c_int = 8;
pub const RT5682S_PWR_RM1_L_BIT: c_int = 0;
// Power Management for Digital 1 (0x0061)

pub const RT5682S_PWR_I2S1_BIT: c_int = 15;

pub const RT5682S_PWR_I2S2_BIT: c_int = 14;

pub const RT5682S_PRE_CHR_DAC_L1_BIT: c_int = 13;

pub const RT5682S_PRE_CHR_DAC_R1_BIT: c_int = 12;

pub const RT5682S_PWR_DAC_L1_BIT: c_int = 11;

pub const RT5682S_PWR_DAC_R1_BIT: c_int = 10;

pub const RT5682S_PWR_LDO_BIT: c_int = 8;

pub const RT5682S_PWR_D2S_L_BIT: c_int = 7;

pub const RT5682S_PWR_D2S_R_BIT: c_int = 6;

pub const RT5682S_PWR_ADC_L1_BIT: c_int = 4;

pub const RT5682S_PWR_ADC_R1_BIT: c_int = 3;

pub const RT5682S_PWR_EFUSE_BIT: c_int = 1;

pub const RT5682S_DIG_GATE_CTRL_SFT: c_int = 0;
// Power Management for Digital 2 (0x0062)

pub const RT5682S_PWR_ADC_S1F_BIT: c_int = 15;

pub const RT5682S_PWR_DAC_S1F_BIT: c_int = 10;

// Power Management for Analog 1 (0x0063)

pub const RT5682S_PWR_VREF1_BIT: c_int = 15;

pub const RT5682S_PWR_FV1_BIT: c_int = 14;

pub const RT5682S_PWR_VREF2_BIT: c_int = 13;

pub const RT5682S_PWR_FV2_BIT: c_int = 12;

pub const RT5682S_PWR_MB_BIT: c_int = 9;

pub const RT5682S_PWR_BG_BIT: c_int = 7;

// Power Management for Analog 2 (0x0064)

pub const RT5682S_PWR_MCLK0_WD_BIT: c_int = 15;

pub const RT5682S_PWR_MCLK1_WD_BIT: c_int = 14;

pub const RT5682S_RST_MCLK0_BIT: c_int = 13;

pub const RT5682S_RST_MCLK1_BIT: c_int = 12;

pub const RT5682S_PWR_MB1_BIT: c_int = 11;

pub const RT5682S_PWR_MB2_BIT: c_int = 10;

// Power Management for Analog 3 (0x0065)

pub const RT5682S_PWR_LDO_PLLA_BIT: c_int = 15;

pub const RT5682S_PWR_LDO_PLLB_BIT: c_int = 14;

pub const RT5682S_PWR_BIAS_PLLA_BIT: c_int = 13;

pub const RT5682S_PWR_BIAS_PLLB_BIT: c_int = 12;

pub const RT5682S_PWR_CBJ_BIT: c_int = 9;

pub const RT5682S_RSTB_PLLB_BIT: c_int = 7;

pub const RT5682S_RSTB_PLLA_BIT: c_int = 6;

pub const RT5682S_PWR_PLLB_BIT: c_int = 5;

pub const RT5682S_PWR_PLLA_BIT: c_int = 4;

pub const RT5682S_PWR_LDO_MB2_BIT: c_int = 2;

pub const RT5682S_PWR_LDO_MB1_BIT: c_int = 1;

pub const RT5682S_PWR_BGLDO_BIT: c_int = 0;
// Power Management for Mixer (0x0066)

pub const RT5682S_PWR_CLK_COMP_8FS_BIT: c_int = 15;

pub const RT5682S_DBG_BGLDO_SFT: c_int = 12;

pub const RT5682S_DBG_BGLDO_MB1_SFT: c_int = 10;

pub const RT5682S_DBG_BGLDO_MB2_SFT: c_int = 8;

pub const RT5682S_DLDO_BGLDO_MB2_SFT: c_int = 6;

pub const RT5682S_PWR_STO1_DAC_L_BIT: c_int = 5;

pub const RT5682S_PWR_STO1_DAC_R_BIT: c_int = 4;

pub const RT5682S_DVO_BGLDO_MB1_SFT: c_int = 2;

// MCLK and System Clock Detection Control (0x006b)

pub const RT5682S_SYS_CLK_DET_SFT: c_int = 15;

pub const RT5682S_PLL1_CLK_DET_SFT: c_int = 14;
// Digital Microphone Control 1 (0x006e)

pub const RT5682S_DMIC_1_EN_SFT: c_int = 15;

pub const RT5682S_DMIC_1_DP_SFT: c_int = 4;

pub const RT5682S_DMIC_CLK_SFT: c_int = 0;
// I2S1 Audio Serial Data Port Control (0x0070)

pub const RT5682S_SEL_ADCDAT_SFT: c_int = 15;

pub const RT5682S_I2S1_TX_CHL_SFT: c_int = 12;

pub const RT5682S_I2S1_RX_CHL_SFT: c_int = 8;

pub const RT5682S_I2S1_DL_SFT: c_int = 4;

// I2S1/2 Audio Serial Data Port Control (0x0071)

pub const RT5682S_I2S2_MS_SFT: c_int = 15;

pub const RT5682S_I2S2_PIN_CFG_SFT: c_int = 14;

pub const RT5682S_I2S2_OUT_SFT: c_int = 9;

pub const RT5682S_I2S_BP_SFT: c_int = 8;

pub const RT5682S_I2S2_DL_SFT: c_int = 4;

pub const RT5682S_I2S_DF_SFT: c_int = 0;

// ADC/DAC Clock Control 1 (0x0073)

pub const RT5682S_ADC_OSR_SFT: c_int = 12;

pub const RT5682S_I2S_M_D_SFT: c_int = 8;

pub const RT5682S_I2S_M_CLK_SRC_SFT: c_int = 4;

pub const RT5682S_DAC_OSR_SFT: c_int = 0;

// ADC/DAC Clock Control 2 (0x0074)

pub const RT5682S_I2S2_BCLK_MS2_SFT: c_int = 11;

// TDM control 1 (0x0079)

pub const RT5682S_TDM_ADC_LCA_SFT: c_int = 4;

pub const RT5682S_TDM_ADC_DL_SFT: c_int = 0;
// TDM control 2 (0x007a)
pub const RT5682S_IF1_ADC1_SEL_SFT: c_int = 14;
pub const RT5682S_IF1_ADC2_SEL_SFT: c_int = 12;
pub const RT5682S_IF1_ADC3_SEL_SFT: c_int = 10;
pub const RT5682S_IF1_ADC4_SEL_SFT: c_int = 8;
pub const RT5682S_TDM_ADC_SEL_SFT: c_int = 3;
// TDM control 3 (0x007b)

// TDM/I2S control (0x007e)

pub const RT5682S_TDM_S_BP_SFT: c_int = 15;

pub const RT5682S_TDM_S_LP_SFT: c_int = 14;

pub const RT5682S_TDM_DF_SFT: c_int = 11;

pub const RT5682S_TDM_BCLK_MS1_SFT: c_int = 8;

pub const RT5682S_TDM_M_BP_SFT: c_int = 2;

pub const RT5682S_TDM_M_LP_SFT: c_int = 1;

pub const RT5682S_TDM_MS_SFT: c_int = 0;

// Global Clock Control (0x0080)

pub const RT5682S_SCLK_SRC_SFT: c_int = 13;

pub const RT5682S_PLL_SRC_SFT: c_int = 8;

// PLL tracking mode 1 (0x0083)

pub const RT5682S_DA_ASRC_SFT: c_int = 13;

pub const RT5682S_DAC_STO1_ASRC_SFT: c_int = 12;

pub const RT5682S_AD_ASRC_SFT: c_int = 8;

pub const RT5682S_AD_ASRC_SEL_SFT: c_int = 4;

pub const RT5682S_DMIC_ASRC_SFT: c_int = 3;

pub const RT5682S_ADC_STO1_ASRC_SFT: c_int = 2;

pub const RT5682S_DA_ASRC_SEL_SFT: c_int = 0;
// PLL tracking mode 2 3 (0x0084)(0x0085)

pub const RT5682S_FILTER_CLK_SEL_SFT: c_int = 12;

pub const RT5682S_FILTER_CLK_DIV_SFT: c_int = 8;
// ASRC Control 4 (0x0086)

pub const RT5682S_ASRCIN_FTK_N1_SFT: c_int = 14;

pub const RT5682S_ASRCIN_FTK_N2_SFT: c_int = 12;

pub const RT5682S_ASRCIN_FTK_M1_SFT: c_int = 8;

pub const RT5682S_ASRCIN_FTK_M2_SFT: c_int = 4;
// ASRC Control 11 (0x008c)

// Depop Mode Control 1 (0x008e)

pub const RT5682S_LDO_PUMP_EN_SFT: c_int = 4;

pub const RT5682S_PUMP_EN_SFT: c_int = 3;

pub const RT5682S_CAPLESS_L_EN_SFT: c_int = 1;

pub const RT5682S_CAPLESS_R_EN_SFT: c_int = 0;
// Depop Mode Control 2 (0x8f)

pub const RT5682S_RAMP_SFT: c_int = 12;

pub const RT5682S_BPS_SFT: c_int = 11;

pub const RT5682S_FAST_UPDN_SFT: c_int = 10;

pub const RT5682S_VLO_SFT: c_int = 7;

// HPOUT charge pump 1 (0x0091)

pub const RT5682S_OSW_L_SFT: c_int = 11;

pub const RT5682S_OSW_R_SFT: c_int = 10;

pub const RT5682S_PM_HP_SFT: c_int = 8;

// Micbias Control1 (0x93)

pub const RT5682S_MIC1_OV_SFT: c_int = 14;

pub const RT5682S_MIC2_OV_SFT: c_int = 8;

// Micbias Control2 (0x0094)

pub const RT5682S_PWR_CLK25M_SFT: c_int = 9;

pub const RT5682S_PWR_CLK1M_SFT: c_int = 8;

// PLL M/N/K Code Control 1 (0x0098)

// PLL M/N/K Code Control 2 (0x0099)

pub const RT5682S_PLLA_M_SFT: c_int = 8;

// PLL M/N/K Code Control 3 (0x009a)

// PLL M/N/K Code Control 4 (0x009b)

pub const RT5682S_PLLB_M_SFT: c_int = 8;

// PLL M/N/K Code Control 6 (0x009d)

pub const RT5682S_PLLB_SEL_PS_SFT: c_int = 13;

pub const RT5682S_PLLB_BYP_PS_SFT: c_int = 12;

pub const RT5682S_PLLB_M_BP_SFT: c_int = 11;

pub const RT5682S_PLLB_K_BP_SFT: c_int = 10;

pub const RT5682S_PLLA_M_BP_SFT: c_int = 7;

pub const RT5682S_PLLA_K_BP_SFT: c_int = 6;
// PLL M/N/K Code Control 7 (0x009e)

// RC Clock Control (0x009f)

// I2S2 Master Mode Clock Control 1 (0x00a0)

pub const RT5682S_I2S2_M_CLK_SRC_SFT: c_int = 4;

pub const RT5682S_I2S2_M_D_SFT: c_int = 0;
// IRQ Control 1 (0x00b6)

pub const RT5682S_JD1_PULSE_EN_SFT: c_int = 10;

// IRQ Control 2 (0x00b7)

pub const RT5682S_JD1_EN_SFT: c_int = 15;

// IRQ Control 3 (0x00b8)

// GPIO Control 1 (0x00c0)

pub const RT5682S_GP1_PIN_SFT: c_int = 14;

pub const RT5682S_GP2_PIN_SFT: c_int = 12;

pub const RT5682S_GP3_PIN_SFT: c_int = 10;

pub const RT5682S_GP4_PIN_SFT: c_int = 8;

pub const RT5682S_GP5_PIN_SFT: c_int = 6;

pub const RT5682S_GP6_PIN_SFT: c_int = 5;

// GPIO Control 2 (0x00c1)

// GPIO Status (0x00c2)

// Soft volume and zero cross control 1 (0x00d9)

pub const RT5682S_ZCD_SFT: c_int = 10;

// 4 Button Inline Command Control 2 (0x00e3)

// 4 Button Inline Command Control 3~6 (0x00e5~0x00e8)

pub const RT5682S_4BTN_IL_HOLD_WIN_SFT: c_int = 8;

pub const RT5682S_4BTN_IL_CLICK_WIN_SFT: c_int = 0;
// Analog JD Control (0x00f0)

// Bias current control 7  (0x0110)

// Charge Pump Internal Register1 (0x0125)

// Pad Driving Control (0x0136)

// Chopper and Clock control for DAC (0x013a)

pub const RT5682S_CKXEN_DAC1_SFT: c_int = 13;

pub const RT5682S_CKGEN_DAC1_SFT: c_int = 12;
// Chopper and Clock control for ADC (0x013b)

pub const RT5682S_CKXEN_ADC1_SFT: c_int = 13;

pub const RT5682S_CKGEN_ADC1_SFT: c_int = 12;
// Volume test (0x013f)

// Test Mode Control 1 (0x0145)

pub const RT5682S_AD2DA_LB_SFT: c_int = 10;
// Stereo Noise Gate Control 1 (0x0160)

// Stereo1 DAC Silence Detection Control (0x0190)

// HP Behavior Logic Control 2 (0x01db)

// SAR ADC Inline Command Control 1 (0x0210)

pub const RT5682S_SAR_SEL_MB1_2_SFT: c_int = 8;

// SAR ADC Inline Command Control 2 (0x0211)

// SAR ADC Inline Command Control 13 (0x021c)

// Headphone Amp Detection Control 1 (0x3b00)

// System Clock Source
// PLL Source
// filter mask
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_calc_map {
    pub freq_in: c_uint,
    pub freq_out: c_uint,
    pub m: c_int,
    pub n: c_int,
    pub k: c_int,
    pub m_bp: bool,
    pub k_bp: bool,
    pub byp_ps: bool,
    pub sel_ps: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5682s_priv {
    pub component: *mut snd_soc_component,
    pub pdata: rt5682s_platform_data,
    pub ldo1_en: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub hs_jack: *mut snd_soc_jack,
    pub supplies: [regulator_bulk_data; RT5682S_NUM_SUPPLIES],
    pub jack_detect_work: delayed_work,
    pub jd_check_work: delayed_work,
    pub calibrate_mutex: mutex,
    pub sar_mutex: mutex,
    pub wclk_mutex: mutex,
    pub dai_clks_hw: [clk_hw; RT5682S_DAI_NUM_CLKS],
    pub mclk: *mut clk,

    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: [c_int; RT5682S_AIFS],
    pub bclk: [c_int; RT5682S_AIFS],
    pub master: [c_int; RT5682S_AIFS],
    pub pll_src: [c_int; RT5682S_PLLS],
    pub pll_in: [c_int; RT5682S_PLLS],
    pub pll_out: [c_int; RT5682S_PLLS],
    pub pll_comb: c_int,
    pub jack_type: c_int,
    pub irq: c_uint,
    pub irq_work_delay_time: c_int,
    pub wclk_enabled: c_int,
}
