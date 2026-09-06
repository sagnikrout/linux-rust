//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt1015.h
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
// rt1015.h  --  RT1015 ALSA SoC audio amplifier driver
//
// Copyright 2019 Realtek Semiconductor Corp.
// Author: Jack Yu <jack.yu@realtek.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

pub const RT1015_DEVICE_ID_VAL: c_uint = 0x1011;
pub const RT1015_DEVICE_ID_VAL2: c_uint = 0x1015;
pub const RT1015_RESET: c_uint = 0x0000;
pub const RT1015_CLK2: c_uint = 0x0004;
pub const RT1015_CLK3: c_uint = 0x0006;
pub const RT1015_PLL1: c_uint = 0x000a;
pub const RT1015_PLL2: c_uint = 0x000c;
pub const RT1015_DUM_RW1: c_uint = 0x000e;
pub const RT1015_DUM_RW2: c_uint = 0x0010;
pub const RT1015_DUM_RW3: c_uint = 0x0012;
pub const RT1015_DUM_RW4: c_uint = 0x0014;
pub const RT1015_DUM_RW5: c_uint = 0x0016;
pub const RT1015_DUM_RW6: c_uint = 0x0018;
pub const RT1015_CLK_DET: c_uint = 0x0020;
pub const RT1015_SIL_DET: c_uint = 0x0022;
pub const RT1015_CUSTOMER_ID: c_uint = 0x0076;
pub const RT1015_PCODE_FWVER: c_uint = 0x0078;
pub const RT1015_VER_ID: c_uint = 0x007a;
pub const RT1015_VENDOR_ID: c_uint = 0x007c;
pub const RT1015_DEVICE_ID: c_uint = 0x007d;
pub const RT1015_PAD_DRV1: c_uint = 0x00f0;
pub const RT1015_PAD_DRV2: c_uint = 0x00f2;
pub const RT1015_GAT_BOOST: c_uint = 0x00f3;
pub const RT1015_PRO_ALT: c_uint = 0x00f4;
pub const RT1015_OSCK_STA: c_uint = 0x00f6;
pub const RT1015_MAN_I2C: c_uint = 0x0100;
pub const RT1015_DAC1: c_uint = 0x0102;
pub const RT1015_DAC2: c_uint = 0x0104;
pub const RT1015_DAC3: c_uint = 0x0106;
pub const RT1015_ADC1: c_uint = 0x010c;
pub const RT1015_ADC2: c_uint = 0x010e;
pub const RT1015_TDM_MASTER: c_uint = 0x0111;
pub const RT1015_TDM_TCON: c_uint = 0x0112;
pub const RT1015_TDM1_1: c_uint = 0x0114;
pub const RT1015_TDM1_2: c_uint = 0x0116;
pub const RT1015_TDM1_3: c_uint = 0x0118;
pub const RT1015_TDM1_4: c_uint = 0x011a;
pub const RT1015_TDM1_5: c_uint = 0x011c;
pub const RT1015_MIXER1: c_uint = 0x0300;
pub const RT1015_MIXER2: c_uint = 0x0302;
pub const RT1015_ANA_PROTECT1: c_uint = 0x0311;
pub const RT1015_ANA_CTRL_SEQ1: c_uint = 0x0313;
pub const RT1015_ANA_CTRL_SEQ2: c_uint = 0x0314;
pub const RT1015_VBAT_DET_DEB: c_uint = 0x031a;
pub const RT1015_VBAT_VOLT_DET1: c_uint = 0x031c;
pub const RT1015_VBAT_VOLT_DET2: c_uint = 0x031d;
pub const RT1015_VBAT_TEST_OUT1: c_uint = 0x031e;
pub const RT1015_VBAT_TEST_OUT2: c_uint = 0x031f;
pub const RT1015_VBAT_PROT_ATT: c_uint = 0x0320;
pub const RT1015_VBAT_DET_CODE: c_uint = 0x0321;
pub const RT1015_PWR1: c_uint = 0x0322;
pub const RT1015_PWR4: c_uint = 0x0328;
pub const RT1015_PWR5: c_uint = 0x0329;
pub const RT1015_PWR6: c_uint = 0x032a;
pub const RT1015_PWR7: c_uint = 0x032b;
pub const RT1015_PWR8: c_uint = 0x032c;
pub const RT1015_PWR9: c_uint = 0x032d;
pub const RT1015_CLASSD_SEQ: c_uint = 0x032e;
pub const RT1015_SMART_BST_CTRL1: c_uint = 0x0330;
pub const RT1015_SMART_BST_CTRL2: c_uint = 0x0332;
pub const RT1015_ANA_CTRL1: c_uint = 0x0334;
pub const RT1015_ANA_CTRL2: c_uint = 0x0336;
pub const RT1015_PWR_STATE_CTRL: c_uint = 0x0338;
pub const RT1015_MONO_DYNA_CTRL: c_uint = 0x04fa;
pub const RT1015_MONO_DYNA_CTRL1: c_uint = 0x04fc;
pub const RT1015_MONO_DYNA_CTRL2: c_uint = 0x04fe;
pub const RT1015_MONO_DYNA_CTRL3: c_uint = 0x0500;
pub const RT1015_MONO_DYNA_CTRL4: c_uint = 0x0502;
pub const RT1015_MONO_DYNA_CTRL5: c_uint = 0x0504;
pub const RT1015_SPK_VOL: c_uint = 0x0506;
pub const RT1015_SHORT_DETTOP1: c_uint = 0x0508;
pub const RT1015_SHORT_DETTOP2: c_uint = 0x050a;
pub const RT1015_SPK_DC_DETECT1: c_uint = 0x0519;
pub const RT1015_SPK_DC_DETECT2: c_uint = 0x051a;
pub const RT1015_SPK_DC_DETECT3: c_uint = 0x051b;
pub const RT1015_SPK_DC_DETECT4: c_uint = 0x051d;
pub const RT1015_SPK_DC_DETECT5: c_uint = 0x051f;
pub const RT1015_BAT_RPO_STEP1: c_uint = 0x0536;
pub const RT1015_BAT_RPO_STEP2: c_uint = 0x0538;
pub const RT1015_BAT_RPO_STEP3: c_uint = 0x053a;
pub const RT1015_BAT_RPO_STEP4: c_uint = 0x053c;
pub const RT1015_BAT_RPO_STEP5: c_uint = 0x053d;
pub const RT1015_BAT_RPO_STEP6: c_uint = 0x053e;
pub const RT1015_BAT_RPO_STEP7: c_uint = 0x053f;
pub const RT1015_BAT_RPO_STEP8: c_uint = 0x0540;
pub const RT1015_BAT_RPO_STEP9: c_uint = 0x0541;
pub const RT1015_BAT_RPO_STEP10: c_uint = 0x0542;
pub const RT1015_BAT_RPO_STEP11: c_uint = 0x0543;
pub const RT1015_BAT_RPO_STEP12: c_uint = 0x0544;
pub const RT1015_SPREAD_SPEC1: c_uint = 0x0568;
pub const RT1015_SPREAD_SPEC2: c_uint = 0x056a;
pub const RT1015_PAD_STATUS: c_uint = 0x1000;
pub const RT1015_PADS_PULLING_CTRL1: c_uint = 0x1002;
pub const RT1015_PADS_DRIVING: c_uint = 0x1006;
pub const RT1015_SYS_RST1: c_uint = 0x1007;
pub const RT1015_SYS_RST2: c_uint = 0x1009;
pub const RT1015_SYS_GATING1: c_uint = 0x100a;
pub const RT1015_TEST_MODE1: c_uint = 0x100c;
pub const RT1015_TEST_MODE2: c_uint = 0x100d;
pub const RT1015_TIMING_CTRL1: c_uint = 0x100e;
pub const RT1015_PLL_INT: c_uint = 0x1010;
pub const RT1015_TEST_OUT1: c_uint = 0x1020;
pub const RT1015_DC_CALIB_CLSD1: c_uint = 0x1200;
pub const RT1015_DC_CALIB_CLSD2: c_uint = 0x1202;
pub const RT1015_DC_CALIB_CLSD3: c_uint = 0x1204;
pub const RT1015_DC_CALIB_CLSD4: c_uint = 0x1206;
pub const RT1015_DC_CALIB_CLSD5: c_uint = 0x1208;
pub const RT1015_DC_CALIB_CLSD6: c_uint = 0x120a;
pub const RT1015_DC_CALIB_CLSD7: c_uint = 0x120c;
pub const RT1015_DC_CALIB_CLSD8: c_uint = 0x120e;
pub const RT1015_DC_CALIB_CLSD9: c_uint = 0x1210;
pub const RT1015_DC_CALIB_CLSD10: c_uint = 0x1212;
pub const RT1015_CLSD_INTERNAL1: c_uint = 0x1300;
pub const RT1015_CLSD_INTERNAL2: c_uint = 0x1302;
pub const RT1015_CLSD_INTERNAL3: c_uint = 0x1304;
pub const RT1015_CLSD_INTERNAL4: c_uint = 0x1305;
pub const RT1015_CLSD_INTERNAL5: c_uint = 0x1306;
pub const RT1015_CLSD_INTERNAL6: c_uint = 0x1308;
pub const RT1015_CLSD_INTERNAL7: c_uint = 0x130a;
pub const RT1015_CLSD_INTERNAL8: c_uint = 0x130c;
pub const RT1015_CLSD_INTERNAL9: c_uint = 0x130e;
pub const RT1015_CLSD_OCP_CTRL: c_uint = 0x130f;
pub const RT1015_VREF_LV: c_uint = 0x1310;
pub const RT1015_MBIAS1: c_uint = 0x1312;
pub const RT1015_MBIAS2: c_uint = 0x1314;
pub const RT1015_MBIAS3: c_uint = 0x1316;
pub const RT1015_MBIAS4: c_uint = 0x1318;
pub const RT1015_VREF_LV1: c_uint = 0x131a;
pub const RT1015_S_BST_TIMING_INTER1: c_uint = 0x1322;
pub const RT1015_S_BST_TIMING_INTER2: c_uint = 0x1323;
pub const RT1015_S_BST_TIMING_INTER3: c_uint = 0x1324;
pub const RT1015_S_BST_TIMING_INTER4: c_uint = 0x1325;
pub const RT1015_S_BST_TIMING_INTER5: c_uint = 0x1326;
pub const RT1015_S_BST_TIMING_INTER6: c_uint = 0x1327;
pub const RT1015_S_BST_TIMING_INTER7: c_uint = 0x1328;
pub const RT1015_S_BST_TIMING_INTER8: c_uint = 0x1329;
pub const RT1015_S_BST_TIMING_INTER9: c_uint = 0x132a;
pub const RT1015_S_BST_TIMING_INTER10: c_uint = 0x132b;
pub const RT1015_S_BST_TIMING_INTER11: c_uint = 0x1330;
pub const RT1015_S_BST_TIMING_INTER12: c_uint = 0x1331;
pub const RT1015_S_BST_TIMING_INTER13: c_uint = 0x1332;
pub const RT1015_S_BST_TIMING_INTER14: c_uint = 0x1333;
pub const RT1015_S_BST_TIMING_INTER15: c_uint = 0x1334;
pub const RT1015_S_BST_TIMING_INTER16: c_uint = 0x1335;
pub const RT1015_S_BST_TIMING_INTER17: c_uint = 0x1336;
pub const RT1015_S_BST_TIMING_INTER18: c_uint = 0x1337;
pub const RT1015_S_BST_TIMING_INTER19: c_uint = 0x1338;
pub const RT1015_S_BST_TIMING_INTER20: c_uint = 0x1339;
pub const RT1015_S_BST_TIMING_INTER21: c_uint = 0x133a;
pub const RT1015_S_BST_TIMING_INTER22: c_uint = 0x133b;
pub const RT1015_S_BST_TIMING_INTER23: c_uint = 0x133c;
pub const RT1015_S_BST_TIMING_INTER24: c_uint = 0x133d;
pub const RT1015_S_BST_TIMING_INTER25: c_uint = 0x133e;
pub const RT1015_S_BST_TIMING_INTER26: c_uint = 0x133f;
pub const RT1015_S_BST_TIMING_INTER27: c_uint = 0x1340;
pub const RT1015_S_BST_TIMING_INTER28: c_uint = 0x1341;
pub const RT1015_S_BST_TIMING_INTER29: c_uint = 0x1342;
pub const RT1015_S_BST_TIMING_INTER30: c_uint = 0x1343;
pub const RT1015_S_BST_TIMING_INTER31: c_uint = 0x1344;
pub const RT1015_S_BST_TIMING_INTER32: c_uint = 0x1345;
pub const RT1015_S_BST_TIMING_INTER33: c_uint = 0x1346;
pub const RT1015_S_BST_TIMING_INTER34: c_uint = 0x1347;
pub const RT1015_S_BST_TIMING_INTER35: c_uint = 0x1348;
pub const RT1015_S_BST_TIMING_INTER36: c_uint = 0x1349;
// 0x0004

pub const RT1015_CLK_SYS_PRE_SEL_SFT: c_int = 14;

pub const RT1015_PLL_SEL_SFT: c_int = 13;

pub const RT1015_FS_PD_SFT: c_int = 4;
// 0x000a
pub const RT1015_PLL_M_MAX: c_uint = 0xf;

pub const RT1015_PLL_M_SFT: c_int = 12;

pub const RT1015_PLL_M_BP_SFT: c_int = 11;
pub const RT1015_PLL_N_MAX: c_uint = 0x1ff;

pub const RT1015_PLL_N_SFT: c_int = 0;
// 0x000c

pub const RT1015_PLL_K_MAX: c_uint = 0x1f;

pub const RT1015_PLL_K_SFT: c_int = 0;
// 0x0020

// 0x007a
pub const RT1015_ID_MASK: c_uint = 0xff;
pub const RT1015_ID_VERA: c_uint = 0x0;
pub const RT1015_ID_VERB: c_uint = 0x1;
// 0x00f2

// 0x0102

pub const RT1015_DAC_VOL_SFT: c_int = 9;
// 0x0104

pub const RT1015_DAC_CLK_BIT: c_int = 13;
// 0x0106

pub const RT1015_DA_MUTE_SFT: c_int = 15;
pub const RT1015_DVOL_MUTE_FLAG_SFT: c_int = 12;
// 0x0111

pub const RT1015_TCON_TDM_MS_SFT: c_int = 14;

pub const RT1015_I2S_DL_SFT: c_int = 8;

pub const RT1015_I2S_M_DF_SFT: c_int = 0;

// TDM_tcon Setting (0x0112)

pub const RT1015_I2S_TCON_DF_SFT: c_int = 13;

pub const RT1015_TCON_BCLK_SEL_SFT: c_int = 10;

pub const RT1015_TCON_CH_LEN_SFT: c_int = 5;

pub const RT1015_TCON_BCLK_MST_SFT: c_int = 4;

// TDM1 Setting-1 (0x0114)

pub const RT1015_TDM_INV_BCLK_SFT: c_int = 15;

pub const RT1015_I2S_CH_TX_SFT: c_int = 10;

pub const RT1015_I2S_CH_RX_SFT: c_int = 8;

pub const RT1015_I2S_LR_CH_SEL_SFT: c_int = 7;

pub const RT1015_I2S_CH_TX_LEN_SFT: c_int = 4;

pub const RT1015_I2S_CH_RX_LEN_SFT: c_int = 0;

// TDM1 Setting-4 (0x011a)

pub const RT1015_TDM_I2S_TX_L_DAC1_1_SFT: c_int = 12;
pub const RT1015_TDM_I2S_TX_R_DAC1_1_SFT: c_int = 8;
// 0x0330

// 0x0322

pub const RT1015_PWR_LDO2_BIT: c_int = 15;

pub const RT1015_PWR_DAC_BIT: c_int = 14;

pub const RT1015_PWR_INTCLK_BIT: c_int = 13;

pub const RT1015_PWR_ISENSE_BIT: c_int = 12;

pub const RT1015_PWR_VSENSE_BIT: c_int = 10;

pub const RT1015_PWR_PLL_BIT: c_int = 9;

pub const RT1015_PWR_BG_1_2_BIT: c_int = 8;

pub const RT1015_PWR_MBIAS_BG_BIT: c_int = 7;

pub const RT1015_PWR_VBAT_BIT: c_int = 6;

pub const RT1015_PWR_MBIAS_BIT: c_int = 4;

pub const RT1015_PWR_ADCV_BIT: c_int = 3;

pub const RT1015_PWR_MIXERV_BIT: c_int = 2;

pub const RT1015_PWR_SUMV_BIT: c_int = 1;

pub const RT1015_PWR_VREFLV_BIT: c_int = 0;
// 0x0324

pub const RT1015_PWR_BASIC_BIT: c_int = 15;

pub const RT1015_PWR_SD_BIT: c_int = 14;

pub const RT1015_PWR_IBIAS_BIT: c_int = 13;

pub const RT1015_PWR_VCM_BIT: c_int = 11;
// 0x0328

pub const RT1015_PWR_SWR_BIT: c_int = 12;
// 0x0519

// 0x1300

pub const RT1015_PWR_CLSD_BIT: c_int = 12;
// 0x007a
pub const RT1015_ID_MASK: c_uint = 0xff;
pub const RT1015_ID_VERA: c_uint = 0x0;
pub const RT1015_ID_VERB: c_uint = 0x1;
// System Clock Source
// PLL1 Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1015_priv {
    pub component: *mut snd_soc_component,
    pub pdata: rt1015_platform_data,
    pub regmap: *mut regmap,
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
    pub boost_mode: c_int,
    pub bypass_boost: c_int,
    pub dac_is_used: c_int,
    pub cali_done: c_int,
}
