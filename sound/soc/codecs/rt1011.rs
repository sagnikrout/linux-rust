//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt1011.h
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


// SPDX-License-Identifier: GPL-2.0
//
// rt1011.h -- RT1011 ALSA SoC amplifier component driver header
//
// Copyright(c) 2019 Realtek Semiconductor Corp.
//
pub const RT1011_DEVICE_ID_NUM: c_uint = 0x1011;
pub const RT1011_RESET: c_uint = 0x0000;
pub const RT1011_CLK_1: c_uint = 0x0002;
pub const RT1011_CLK_2: c_uint = 0x0004;
pub const RT1011_CLK_3: c_uint = 0x0006;
pub const RT1011_CLK_4: c_uint = 0x0008;
pub const RT1011_PLL_1: c_uint = 0x000a;
pub const RT1011_PLL_2: c_uint = 0x000c;
pub const RT1011_SRC_1: c_uint = 0x000e;
pub const RT1011_SRC_2: c_uint = 0x0010;
pub const RT1011_SRC_3: c_uint = 0x0012;
pub const RT1011_CLK_DET: c_uint = 0x0020;
pub const RT1011_SIL_DET: c_uint = 0x0022;
pub const RT1011_PRIV_INDEX: c_uint = 0x006a;
pub const RT1011_PRIV_DATA: c_uint = 0x006c;
pub const RT1011_CUSTOMER_ID: c_uint = 0x0076;
pub const RT1011_FM_VER: c_uint = 0x0078;
pub const RT1011_VERSION_ID: c_uint = 0x007a;
pub const RT1011_VENDOR_ID: c_uint = 0x007c;
pub const RT1011_DEVICE_ID: c_uint = 0x007d;
pub const RT1011_DUM_RW_0: c_uint = 0x00f0;
pub const RT1011_DUM_YUN: c_uint = 0x00f2;
pub const RT1011_DUM_RW_1: c_uint = 0x00f3;
pub const RT1011_DUM_RO: c_uint = 0x00f4;
pub const RT1011_MAN_I2C_DEV: c_uint = 0x0100;
pub const RT1011_DAC_SET_1: c_uint = 0x0102;
pub const RT1011_DAC_SET_2: c_uint = 0x0104;
pub const RT1011_DAC_SET_3: c_uint = 0x0106;
pub const RT1011_ADC_SET: c_uint = 0x0107;
pub const RT1011_ADC_SET_1: c_uint = 0x0108;
pub const RT1011_ADC_SET_2: c_uint = 0x010a;
pub const RT1011_ADC_SET_3: c_uint = 0x010c;
pub const RT1011_ADC_SET_4: c_uint = 0x010e;
pub const RT1011_ADC_SET_5: c_uint = 0x0110;
pub const RT1011_TDM_TOTAL_SET: c_uint = 0x0111;
pub const RT1011_TDM1_SET_TCON: c_uint = 0x0112;
pub const RT1011_TDM1_SET_1: c_uint = 0x0114;
pub const RT1011_TDM1_SET_2: c_uint = 0x0116;
pub const RT1011_TDM1_SET_3: c_uint = 0x0118;
pub const RT1011_TDM1_SET_4: c_uint = 0x011a;
pub const RT1011_TDM1_SET_5: c_uint = 0x011c;
pub const RT1011_TDM2_SET_1: c_uint = 0x011e;
pub const RT1011_TDM2_SET_2: c_uint = 0x0120;
pub const RT1011_TDM2_SET_3: c_uint = 0x0122;
pub const RT1011_TDM2_SET_4: c_uint = 0x0124;
pub const RT1011_TDM2_SET_5: c_uint = 0x0126;
pub const RT1011_PWM_CAL: c_uint = 0x0200;
pub const RT1011_MIXER_1: c_uint = 0x0300;
pub const RT1011_MIXER_2: c_uint = 0x0302;
pub const RT1011_ADRC_LIMIT: c_uint = 0x0310;
pub const RT1011_A_PRO: c_uint = 0x0311;
pub const RT1011_A_TIMING_1: c_uint = 0x0313;
pub const RT1011_A_TIMING_2: c_uint = 0x0314;
pub const RT1011_A_TEMP_SEN: c_uint = 0x0316;
pub const RT1011_SPK_VOL_DET_1: c_uint = 0x0319;
pub const RT1011_SPK_VOL_DET_2: c_uint = 0x031a;
pub const RT1011_SPK_VOL_TEST_OUT: c_uint = 0x031b;
pub const RT1011_VBAT_VOL_DET_1: c_uint = 0x031c;
pub const RT1011_VBAT_VOL_DET_2: c_uint = 0x031d;
pub const RT1011_VBAT_TEST_OUT_1: c_uint = 0x031e;
pub const RT1011_VBAT_TEST_OUT_2: c_uint = 0x031f;
pub const RT1011_VBAT_PROTECTION: c_uint = 0x0320;
pub const RT1011_VBAT_DET: c_uint = 0x0321;
pub const RT1011_POWER_1: c_uint = 0x0322;
pub const RT1011_POWER_2: c_uint = 0x0324;
pub const RT1011_POWER_3: c_uint = 0x0326;
pub const RT1011_POWER_4: c_uint = 0x0328;
pub const RT1011_POWER_5: c_uint = 0x0329;
pub const RT1011_POWER_6: c_uint = 0x032a;
pub const RT1011_POWER_7: c_uint = 0x032b;
pub const RT1011_POWER_8: c_uint = 0x032c;
pub const RT1011_POWER_9: c_uint = 0x032d;
pub const RT1011_CLASS_D_POS: c_uint = 0x032e;
pub const RT1011_BOOST_CON_1: c_uint = 0x0330;
pub const RT1011_BOOST_CON_2: c_uint = 0x0332;
pub const RT1011_ANALOG_CTRL: c_uint = 0x0334;
pub const RT1011_POWER_SEQ: c_uint = 0x0340;
pub const RT1011_SHORT_CIRCUIT_DET_1: c_uint = 0x0508;
pub const RT1011_SHORT_CIRCUIT_DET_2: c_uint = 0x050a;
pub const RT1011_SPK_TEMP_PROTECT_0: c_uint = 0x050c;
pub const RT1011_SPK_TEMP_PROTECT_1: c_uint = 0x050d;
pub const RT1011_SPK_TEMP_PROTECT_2: c_uint = 0x050e;
pub const RT1011_SPK_TEMP_PROTECT_3: c_uint = 0x050f;
pub const RT1011_SPK_TEMP_PROTECT_4: c_uint = 0x0510;
pub const RT1011_SPK_TEMP_PROTECT_5: c_uint = 0x0511;
pub const RT1011_SPK_TEMP_PROTECT_6: c_uint = 0x0512;
pub const RT1011_SPK_TEMP_PROTECT_7: c_uint = 0x0516;
pub const RT1011_SPK_TEMP_PROTECT_8: c_uint = 0x0517;
pub const RT1011_SPK_TEMP_PROTECT_9: c_uint = 0x0518;
pub const RT1011_SPK_PRO_DC_DET_1: c_uint = 0x0519;
pub const RT1011_SPK_PRO_DC_DET_2: c_uint = 0x051a;
pub const RT1011_SPK_PRO_DC_DET_3: c_uint = 0x051b;
pub const RT1011_SPK_PRO_DC_DET_4: c_uint = 0x051c;
pub const RT1011_SPK_PRO_DC_DET_5: c_uint = 0x051d;
pub const RT1011_SPK_PRO_DC_DET_6: c_uint = 0x051e;
pub const RT1011_SPK_PRO_DC_DET_7: c_uint = 0x051f;
pub const RT1011_SPK_PRO_DC_DET_8: c_uint = 0x0520;
pub const RT1011_SPL_1: c_uint = 0x0521;
pub const RT1011_SPL_2: c_uint = 0x0522;
pub const RT1011_SPL_3: c_uint = 0x0524;
pub const RT1011_SPL_4: c_uint = 0x0526;
pub const RT1011_THER_FOLD_BACK_1: c_uint = 0x0528;
pub const RT1011_THER_FOLD_BACK_2: c_uint = 0x052a;
pub const RT1011_EXCUR_PROTECT_1: c_uint = 0x0530;
pub const RT1011_EXCUR_PROTECT_2: c_uint = 0x0532;
pub const RT1011_EXCUR_PROTECT_3: c_uint = 0x0534;
pub const RT1011_EXCUR_PROTECT_4: c_uint = 0x0535;
pub const RT1011_BAT_GAIN_1: c_uint = 0x0536;
pub const RT1011_BAT_GAIN_2: c_uint = 0x0538;
pub const RT1011_BAT_GAIN_3: c_uint = 0x053a;
pub const RT1011_BAT_GAIN_4: c_uint = 0x053c;
pub const RT1011_BAT_GAIN_5: c_uint = 0x053d;
pub const RT1011_BAT_GAIN_6: c_uint = 0x053e;
pub const RT1011_BAT_GAIN_7: c_uint = 0x053f;
pub const RT1011_BAT_GAIN_8: c_uint = 0x0540;
pub const RT1011_BAT_GAIN_9: c_uint = 0x0541;
pub const RT1011_BAT_GAIN_10: c_uint = 0x0542;
pub const RT1011_BAT_GAIN_11: c_uint = 0x0543;
pub const RT1011_BAT_RT_THMAX_1: c_uint = 0x0544;
pub const RT1011_BAT_RT_THMAX_2: c_uint = 0x0545;
pub const RT1011_BAT_RT_THMAX_3: c_uint = 0x0546;
pub const RT1011_BAT_RT_THMAX_4: c_uint = 0x0547;
pub const RT1011_BAT_RT_THMAX_5: c_uint = 0x0548;
pub const RT1011_BAT_RT_THMAX_6: c_uint = 0x0549;
pub const RT1011_BAT_RT_THMAX_7: c_uint = 0x054a;
pub const RT1011_BAT_RT_THMAX_8: c_uint = 0x054b;
pub const RT1011_BAT_RT_THMAX_9: c_uint = 0x054c;
pub const RT1011_BAT_RT_THMAX_10: c_uint = 0x054d;
pub const RT1011_BAT_RT_THMAX_11: c_uint = 0x054e;
pub const RT1011_BAT_RT_THMAX_12: c_uint = 0x054f;
pub const RT1011_SPREAD_SPECTURM: c_uint = 0x0568;
pub const RT1011_PRO_GAIN_MODE: c_uint = 0x056a;
pub const RT1011_RT_DRC_CROSS: c_uint = 0x0600;
pub const RT1011_RT_DRC_HB_1: c_uint = 0x0611;
pub const RT1011_RT_DRC_HB_2: c_uint = 0x0612;
pub const RT1011_RT_DRC_HB_3: c_uint = 0x0613;
pub const RT1011_RT_DRC_HB_4: c_uint = 0x0614;
pub const RT1011_RT_DRC_HB_5: c_uint = 0x0615;
pub const RT1011_RT_DRC_HB_6: c_uint = 0x0616;
pub const RT1011_RT_DRC_HB_7: c_uint = 0x0617;
pub const RT1011_RT_DRC_HB_8: c_uint = 0x0618;
pub const RT1011_RT_DRC_BB_1: c_uint = 0x0621;
pub const RT1011_RT_DRC_BB_2: c_uint = 0x0622;
pub const RT1011_RT_DRC_BB_3: c_uint = 0x0623;
pub const RT1011_RT_DRC_BB_4: c_uint = 0x0624;
pub const RT1011_RT_DRC_BB_5: c_uint = 0x0625;
pub const RT1011_RT_DRC_BB_6: c_uint = 0x0626;
pub const RT1011_RT_DRC_BB_7: c_uint = 0x0627;
pub const RT1011_RT_DRC_BB_8: c_uint = 0x0628;
pub const RT1011_RT_DRC_POS_1: c_uint = 0x0631;
pub const RT1011_RT_DRC_POS_2: c_uint = 0x0632;
pub const RT1011_RT_DRC_POS_3: c_uint = 0x0633;
pub const RT1011_RT_DRC_POS_4: c_uint = 0x0634;
pub const RT1011_RT_DRC_POS_5: c_uint = 0x0635;
pub const RT1011_RT_DRC_POS_6: c_uint = 0x0636;
pub const RT1011_RT_DRC_POS_7: c_uint = 0x0637;
pub const RT1011_RT_DRC_POS_8: c_uint = 0x0638;
pub const RT1011_CROSS_BQ_SET_1: c_uint = 0x0702;
pub const RT1011_CROSS_BQ_SET_2: c_uint = 0x0704;
pub const RT1011_BQ_SET_0: c_uint = 0x0706;
pub const RT1011_BQ_SET_1: c_uint = 0x0708;
pub const RT1011_BQ_SET_2: c_uint = 0x070a;
pub const RT1011_BQ_PRE_GAIN_28_16: c_uint = 0x0710;
pub const RT1011_BQ_PRE_GAIN_15_0: c_uint = 0x0711;
pub const RT1011_BQ_POST_GAIN_28_16: c_uint = 0x0712;
pub const RT1011_BQ_POST_GAIN_15_0: c_uint = 0x0713;
pub const RT1011_BQ_H0_28_16: c_uint = 0x0720;
pub const RT1011_BQ_A2_15_0: c_uint = 0x0729;
pub const RT1011_BQ_1_H0_28_16: c_uint = 0x0730;
pub const RT1011_BQ_1_A2_15_0: c_uint = 0x0739;
pub const RT1011_BQ_2_H0_28_16: c_uint = 0x0740;
pub const RT1011_BQ_2_A2_15_0: c_uint = 0x0749;
pub const RT1011_BQ_3_H0_28_16: c_uint = 0x0750;
pub const RT1011_BQ_3_A2_15_0: c_uint = 0x0759;
pub const RT1011_BQ_4_H0_28_16: c_uint = 0x0760;
pub const RT1011_BQ_4_A2_15_0: c_uint = 0x0769;
pub const RT1011_BQ_5_H0_28_16: c_uint = 0x0770;
pub const RT1011_BQ_5_A2_15_0: c_uint = 0x0779;
pub const RT1011_BQ_6_H0_28_16: c_uint = 0x0780;
pub const RT1011_BQ_6_A2_15_0: c_uint = 0x0789;
pub const RT1011_BQ_7_H0_28_16: c_uint = 0x0790;
pub const RT1011_BQ_7_A2_15_0: c_uint = 0x0799;
pub const RT1011_BQ_8_H0_28_16: c_uint = 0x07a0;
pub const RT1011_BQ_8_A2_15_0: c_uint = 0x07a9;
pub const RT1011_BQ_9_H0_28_16: c_uint = 0x07b0;
pub const RT1011_BQ_9_A2_15_0: c_uint = 0x07b9;
pub const RT1011_BQ_10_H0_28_16: c_uint = 0x07c0;
pub const RT1011_BQ_10_A2_15_0: c_uint = 0x07c9;
pub const RT1011_TEST_PAD_STATUS: c_uint = 0x1000;
pub const RT1011_SYSTEM_RESET_1: c_uint = 0x1007;
pub const RT1011_SYSTEM_RESET_2: c_uint = 0x1008;
pub const RT1011_SYSTEM_RESET_3: c_uint = 0x1009;
pub const RT1011_ADCDAT_OUT_SOURCE: c_uint = 0x100D;
pub const RT1011_PLL_INTERNAL_SET: c_uint = 0x1010;
pub const RT1011_TEST_OUT_1: c_uint = 0x1020;
pub const RT1011_TEST_OUT_3: c_uint = 0x1024;
pub const RT1011_DC_CALIB_CLASSD_1: c_uint = 0x1200;
pub const RT1011_DC_CALIB_CLASSD_2: c_uint = 0x1202;
pub const RT1011_DC_CALIB_CLASSD_3: c_uint = 0x1204;
pub const RT1011_DC_CALIB_CLASSD_5: c_uint = 0x1208;
pub const RT1011_DC_CALIB_CLASSD_6: c_uint = 0x120a;
pub const RT1011_DC_CALIB_CLASSD_7: c_uint = 0x120c;
pub const RT1011_DC_CALIB_CLASSD_8: c_uint = 0x120e;
pub const RT1011_DC_CALIB_CLASSD_10: c_uint = 0x1212;
pub const RT1011_CLASSD_INTERNAL_SET_1: c_uint = 0x1300;
pub const RT1011_CLASSD_INTERNAL_SET_3: c_uint = 0x1304;
pub const RT1011_CLASSD_INTERNAL_SET_8: c_uint = 0x130c;
pub const RT1011_VREF_LV_1: c_uint = 0x131a;
pub const RT1011_SMART_BOOST_TIMING_1: c_uint = 0x1322;
pub const RT1011_SMART_BOOST_TIMING_36: c_uint = 0x1349;
pub const RT1011_SINE_GEN_REG_1: c_uint = 0x1500;
pub const RT1011_SINE_GEN_REG_2: c_uint = 0x1502;
pub const RT1011_SINE_GEN_REG_3: c_uint = 0x1504;
pub const RT1011_STP_INITIAL_RS_TEMP: c_uint = 0x1510;
pub const RT1011_STP_CALIB_RS_TEMP: c_uint = 0x152a;
pub const RT1011_INIT_RECIPROCAL_REG_24_16: c_uint = 0x1538;
pub const RT1011_INIT_RECIPROCAL_REG_15_0: c_uint = 0x1539;
pub const RT1011_STP_INITIAL_RESISTANCE_TEMP: c_uint = 0x153c;
pub const RT1011_STP_ALPHA_RECIPROCAL_MSB: c_uint = 0x153e;
pub const RT1011_SPK_RESISTANCE_1: c_uint = 0x1544;
pub const RT1011_SPK_RESISTANCE_2: c_uint = 0x1546;
pub const RT1011_SPK_THERMAL: c_uint = 0x1548;
pub const RT1011_STP_OTP_TH: c_uint = 0x1552;
pub const RT1011_ALC_BK_GAIN_O: c_uint = 0x1554;
pub const RT1011_ALC_BK_GAIN_O_PRE: c_uint = 0x1556;
pub const RT1011_SPK_DC_O_23_16: c_uint = 0x155a;
pub const RT1011_SPK_DC_O_15_0: c_uint = 0x155c;
pub const RT1011_INIT_RECIPROCAL_SYN_24_16: c_uint = 0x1560;
pub const RT1011_INIT_RECIPROCAL_SYN_15_0: c_uint = 0x1562;
pub const RT1011_STP_BQ_1_A1_L_28_16: c_uint = 0x1570;
pub const RT1011_STP_BQ_1_H0_R_15_0: c_uint = 0x1583;
pub const RT1011_STP_BQ_2_A1_L_28_16: c_uint = 0x1590;
pub const RT1011_SPK_EXCURSION_23_16: c_uint = 0x15be;
pub const RT1011_SPK_EXCURSION_15_0: c_uint = 0x15bf;
pub const RT1011_SEP_MAIN_OUT_23_16: c_uint = 0x15c0;
pub const RT1011_SEP_MAIN_OUT_15_0: c_uint = 0x15c1;
pub const RT1011_SEP_RE_REG_15_0: c_uint = 0x15f9;
pub const RT1011_DRC_CF_PARAMS_1: c_uint = 0x1600;
pub const RT1011_DRC_CF_PARAMS_12: c_uint = 0x160b;
pub const RT1011_ALC_DRC_HB_INTERNAL_1: c_uint = 0x1611;
pub const RT1011_ALC_DRC_HB_INTERNAL_5: c_uint = 0x1615;
pub const RT1011_ALC_DRC_HB_INTERNAL_6: c_uint = 0x1616;
pub const RT1011_ALC_DRC_HB_INTERNAL_7: c_uint = 0x1617;
pub const RT1011_ALC_DRC_BB_INTERNAL_1: c_uint = 0x1621;
pub const RT1011_ALC_DRC_BB_INTERNAL_5: c_uint = 0x1625;
pub const RT1011_ALC_DRC_BB_INTERNAL_6: c_uint = 0x1626;
pub const RT1011_ALC_DRC_BB_INTERNAL_7: c_uint = 0x1627;
pub const RT1011_ALC_DRC_POS_INTERNAL_1: c_uint = 0x1631;
pub const RT1011_ALC_DRC_POS_INTERNAL_5: c_uint = 0x1635;
pub const RT1011_ALC_DRC_POS_INTERNAL_6: c_uint = 0x1636;
pub const RT1011_ALC_DRC_POS_INTERNAL_7: c_uint = 0x1637;
pub const RT1011_ALC_DRC_POS_INTERNAL_8: c_uint = 0x1638;
pub const RT1011_ALC_DRC_POS_INTERNAL_9: c_uint = 0x163a;
pub const RT1011_ALC_DRC_POS_INTERNAL_10: c_uint = 0x163c;
pub const RT1011_ALC_DRC_POS_INTERNAL_11: c_uint = 0x163e;
pub const RT1011_BQ_1_PARAMS_CHECK_5: c_uint = 0x1648;
pub const RT1011_BQ_2_PARAMS_CHECK_1: c_uint = 0x1650;
pub const RT1011_BQ_2_PARAMS_CHECK_5: c_uint = 0x1658;
pub const RT1011_BQ_3_PARAMS_CHECK_1: c_uint = 0x1660;
pub const RT1011_BQ_3_PARAMS_CHECK_5: c_uint = 0x1668;
pub const RT1011_BQ_4_PARAMS_CHECK_1: c_uint = 0x1670;
pub const RT1011_BQ_4_PARAMS_CHECK_5: c_uint = 0x1678;
pub const RT1011_BQ_5_PARAMS_CHECK_1: c_uint = 0x1680;
pub const RT1011_BQ_5_PARAMS_CHECK_5: c_uint = 0x1688;
pub const RT1011_BQ_6_PARAMS_CHECK_1: c_uint = 0x1690;
pub const RT1011_BQ_6_PARAMS_CHECK_5: c_uint = 0x1698;
pub const RT1011_BQ_7_PARAMS_CHECK_1: c_uint = 0x1700;
pub const RT1011_BQ_7_PARAMS_CHECK_5: c_uint = 0x1708;
pub const RT1011_BQ_8_PARAMS_CHECK_1: c_uint = 0x1710;
pub const RT1011_BQ_8_PARAMS_CHECK_5: c_uint = 0x1718;
pub const RT1011_BQ_9_PARAMS_CHECK_1: c_uint = 0x1720;
pub const RT1011_BQ_9_PARAMS_CHECK_5: c_uint = 0x1728;
pub const RT1011_BQ_10_PARAMS_CHECK_1: c_uint = 0x1730;
pub const RT1011_BQ_10_PARAMS_CHECK_5: c_uint = 0x1738;
pub const RT1011_IRQ_1: c_uint = 0x173a;
pub const RT1011_PART_NUMBER_EFUSE: c_uint = 0x173e;
pub const RT1011_EFUSE_CONTROL_1: c_uint = 0x17bb;
pub const RT1011_EFUSE_CONTROL_2: c_uint = 0x17bd;
pub const RT1011_EFUSE_MATCH_DONE: c_uint = 0x17cb;
pub const RT1011_EFUSE_ADC_OFFSET_18_16: c_uint = 0x17e5;
pub const RT1011_EFUSE_ADC_OFFSET_15_0: c_uint = 0x17e7;
pub const RT1011_EFUSE_DAC_OFFSET_G0_20_16: c_uint = 0x17e9;
pub const RT1011_EFUSE_DAC_OFFSET_G0_15_0: c_uint = 0x17eb;
pub const RT1011_EFUSE_DAC_OFFSET_G1_20_16: c_uint = 0x17ed;
pub const RT1011_EFUSE_DAC_OFFSET_G1_15_0: c_uint = 0x17ef;
pub const RT1011_EFUSE_READ_R0_3_15_0: c_uint = 0x1803;
pub const RT1011_MAX_REG: c_uint = 0x1803;
pub const RT1011_REG_DISP_LEN: c_int = 23;
// CLOCK-2 (0x0004)

pub const RT1011_FS_SYS_PRE_SFT: c_int = 14;

pub const RT1011_PLL1_SRC_SFT: c_int = 13;

pub const RT1011_PLL2_SRC_SFT: c_int = 12;

pub const RT1011_PLL2_SRC_DIV_SFT: c_int = 10;

pub const RT1011_SRCIN_DIV_SFT: c_int = 8;

pub const RT1011_FS_SYS_DIV_SFT: c_int = 4;
// PLL-1 (0x000a)

pub const RT1011_PLL1_QM_SFT: c_int = 12;

pub const RT1011_PLL1_BPM_SFT: c_int = 11;

pub const RT1011_PLL1_QN_SFT: c_int = 0;
// PLL-2 (0x000c)

pub const RT1011_PLL2_BPK_SFT: c_int = 5;

pub const RT1011_PLL2_QK_SFT: c_int = 0;
// Clock Detect (0x0020)

pub const RT1011_EN_MCLK_DET_SFT: c_int = 15;

// DAC Setting-2 (0x0104)

pub const RT1011_EN_CKGEN_DAC_SFT: c_int = 13;

// DAC Setting-3 (0x0106)

pub const RT1011_DA_MUTE_EN_SFT: c_int = 15;
// ADC Setting-5 (0x0110)

pub const RT1011_AD_EN_CKGEN_ADC_SFT: c_int = 9;

// TDM Total Setting (0x0111)

pub const RT1011_I2S_TDM_MS_SFT: c_int = 14;

pub const RT1011_I2S_TX_DL_SFT: c_int = 8;

pub const RT1011_I2S_RX_DL_SFT: c_int = 5;

pub const RT1011_I2S_TDM_DF_SFT: c_int = 0;

// TDM_tcon Setting (0x0112)

pub const RT1011_TCON_DF_SFT: c_int = 13;

pub const RT1011_TCON_BCLK_SEL_SFT: c_int = 10;

pub const RT1011_TCON_CH_LEN_SFT: c_int = 5;

pub const RT1011_TCON_BCLK_MST_SFT: c_int = 4;

// TDM1 Setting-1 (0x0114)

pub const RT1011_TDM_INV_BCLK_SFT: c_int = 15;

pub const RT1011_I2S_CH_TX_SFT: c_int = 10;

pub const RT1011_I2S_CH_RX_SFT: c_int = 8;

pub const RT1011_I2S_LR_CH_SEL_SFT: c_int = 7;

pub const RT1011_I2S_CH_TX_LEN_SFT: c_int = 4;

pub const RT1011_I2S_CH_RX_LEN_SFT: c_int = 0;

// TDM1 Setting-2 (0x0116)

pub const RT1011_TDM_I2S_DOCK_EN_1_SFT: c_int = 3;

// TDM1 Setting-3 (0x0118)

// TDM1 Setting-4 (0x011a)

pub const RT1011_TDM_I2S_TX_L_DAC1_1_SFT: c_int = 12;
pub const RT1011_TDM_I2S_TX_R_DAC1_1_SFT: c_int = 8;
// TDM2 Setting-2 (0x0120)

pub const RT1011_TDM_I2S_DOCK_EN_2_SFT: c_int = 3;

// MIXER 1 (0x0300)

pub const RT1011_MIXER_MUTE_MIX_I_SFT: c_int = 15;

pub const RT1011_MIXER_MUTE_SUM_I_SFT: c_int = 14;

pub const RT1011_MIXER_MUTE_MIX_V_SFT: c_int = 7;

pub const RT1011_MIXER_MUTE_SUM_V_SFT: c_int = 6;

// Analog Temperature Sensor (0x0316)

pub const RT1011_POW_TEMP_REG_BIT: c_int = 2;
// POWER-1 (0x0322)

pub const RT1011_POW_LDO2_BIT: c_int = 15;

pub const RT1011_POW_DAC_BIT: c_int = 14;

pub const RT1011_POW_CLK12M_BIT: c_int = 13;

pub const RT1011_POW_TEMP_BIT: c_int = 12;

pub const RT1011_POW_ISENSE_SPK_BIT: c_int = 7;

pub const RT1011_POW_LPF_SPK_BIT: c_int = 6;

pub const RT1011_POW_VSENSE_SPK_BIT: c_int = 5;

pub const RT1011_POW_TWO_BATTERY_SPK_BIT: c_int = 4;
// POWER-2 (0x0324)

pub const RT1011_PLLEN_BIT: c_int = 2;

pub const RT1011_POW_BG_BIT: c_int = 1;

pub const RT1011_POW_BG_MBIAS_LV_BIT: c_int = 0;
// POWER-3 (0x0326)

pub const RT1011_POW_DET_SPKVDD_BIT: c_int = 15;

pub const RT1011_POW_DET_VBAT_BIT: c_int = 14;

pub const RT1011_POW_FC_BIT: c_int = 13;

pub const RT1011_POW_MBIAS_LV_BIT: c_int = 12;

pub const RT1011_POW_ADC_I_BIT: c_int = 11;

pub const RT1011_POW_ADC_V_BIT: c_int = 10;

pub const RT1011_POW_ADC_T_BIT: c_int = 9;

pub const RT1011_POWD_ADC_T_BIT: c_int = 8;

pub const RT1011_POW_MIX_I_BIT: c_int = 7;

pub const RT1011_POW_MIX_V_BIT: c_int = 6;

pub const RT1011_POW_SUM_I_BIT: c_int = 5;

pub const RT1011_POW_SUM_V_BIT: c_int = 4;

pub const RT1011_POW_MIX_T_BIT: c_int = 2;

pub const RT1011_BYPASS_MIX_T_BIT: c_int = 1;

pub const RT1011_POW_VREF_LV_BIT: c_int = 0;
// POWER-4 (0x0328)

pub const RT1011_POW_EN_SWR_BIT: c_int = 12;

pub const RT1011_POW_EN_PASS_BGOK_SWR_BIT: c_int = 10;

pub const RT1011_POW_EN_PASS_VPOK_SWR_BIT: c_int = 9;
// POWER-9 (0x032d)

pub const RT1011_POW_SDB_REG_BIT: c_int = 9;

pub const RT1011_POW_SEL_SDB_MODE_BIT: c_int = 6;

pub const RT1011_POW_MNL_SDB_BIT: c_int = 5;

// SPK Protection-Temperature Protection (0x050c)

pub const RT1011_STP_EN_BIT: c_int = 15;

pub const RT1011_STP_RS_CLB_EN_BIT: c_int = 14;

// SPK Protection-Temperature Protection-4 (0x0510)

// SPK Protection-Temperature Protection-6 (0x0512)

pub const RT1011_STP_R0_EN_BIT: c_int = 7;

pub const RT1011_STP_T0_EN_BIT: c_int = 6;

// Cross Biquad Setting-1 (0x0702)

// ClassD Internal Setting-1 (0x1300)

pub const RT1011_DRIVER_READY_SPK_BIT: c_int = 12;

pub const RT1011_RECV_MODE_SPK_BIT: c_int = 5;
// ClassD Internal Setting-3 (0x1304)

// ClassD Internal Setting-8 (0x130c)

pub const RT1011_TM_PORPVDD_SPK_BIT: c_int = 1;
// SPK Protection-Temperature Protection-SINE_GEN_REG-1 (0x1500)

pub const RT1011_STP_SIN_GEN_EN_BIT: c_int = 13;
// System Clock Source
// PLL Source 1/2
// BiQual & DRC related settings
pub const RT1011_BQ_DRC_NUM: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1011_bq_drc_params {
    pub val: c_ushort,
    pub reg: c_ushort,

    pub reserved: c_uint,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1011_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub cali_work: work_struct,
    pub bq_drc_params: *mut rt1011_bq_drc_params,
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: c_int,
    pub bclk: c_int,
    pub id: c_int,
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
    pub bq_drc_set: c_int,
    pub cali_done: unsigned int r0_reg,,
    pub temperature_calib: unsigned int r0_calib,,
    pub recv_spk_mode: c_int,
    pub i2s_ref: c_int,
}
