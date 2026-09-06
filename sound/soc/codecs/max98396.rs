//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max98396.h
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
// max98396.h -- MAX98396 ALSA SoC audio driver header
//
// Copyright(c) 2022, Analog Devices Inc.
//
pub const MAX98396_R2000_SW_RESET: c_uint = 0x2000;
pub const MAX98396_R2001_INT_RAW1: c_uint = 0x2001;
pub const MAX98396_R2002_INT_RAW2: c_uint = 0x2002;
pub const MAX98396_R2003_INT_RAW3: c_uint = 0x2003;
pub const MAX98396_R2004_INT_RAW4: c_uint = 0x2004;
pub const MAX98396_R2006_INT_STATE1: c_uint = 0x2006;
pub const MAX98396_R2007_INT_STATE2: c_uint = 0x2007;
pub const MAX98396_R2008_INT_STATE3: c_uint = 0x2008;
pub const MAX98396_R2009_INT_STATE4: c_uint = 0x2009;
pub const MAX98396_R200B_INT_FLAG1: c_uint = 0x200B;
pub const MAX98396_R200C_INT_FLAG2: c_uint = 0x200C;
pub const MAX98396_R200D_INT_FLAG3: c_uint = 0x200D;
pub const MAX98396_R200E_INT_FLAG4: c_uint = 0x200E;
pub const MAX98396_R2010_INT_EN1: c_uint = 0x2010;
pub const MAX98396_R2011_INT_EN2: c_uint = 0x2011;
pub const MAX98396_R2012_INT_EN3: c_uint = 0x2012;
pub const MAX98396_R2013_INT_EN4: c_uint = 0x2013;
pub const MAX98396_R2015_INT_FLAG_CLR1: c_uint = 0x2015;
pub const MAX98396_R2016_INT_FLAG_CLR2: c_uint = 0x2016;
pub const MAX98396_R2017_INT_FLAG_CLR3: c_uint = 0x2017;
pub const MAX98396_R2018_INT_FLAG_CLR4: c_uint = 0x2018;
pub const MAX98396_R201F_IRQ_CTRL: c_uint = 0x201F;
pub const MAX98396_R2020_THERM_WARN_THRESH: c_uint = 0x2020;
pub const MAX98396_R2021_THERM_WARN_THRESH2: c_uint = 0x2021;
pub const MAX98396_R2022_THERM_SHDN_THRESH: c_uint = 0x2022;
pub const MAX98396_R2023_THERM_HYSTERESIS: c_uint = 0x2023;
pub const MAX98396_R2024_THERM_FOLDBACK_SET: c_uint = 0x2024;
pub const MAX98396_R2027_THERM_FOLDBACK_EN: c_uint = 0x2027;
pub const MAX98396_R2030_NOISEGATE_MODE_CTRL: c_uint = 0x2030;
pub const MAX98396_R2033_NOISEGATE_MODE_EN: c_uint = 0x2033;
pub const MAX98396_R2038_CLK_MON_CTRL: c_uint = 0x2038;
pub const MAX98396_R2039_DATA_MON_CTRL: c_uint = 0x2039;
pub const MAX98396_R203F_ENABLE_CTRLS: c_uint = 0x203F;
pub const MAX98396_R2040_PIN_CFG: c_uint = 0x2040;
pub const MAX98396_R2041_PCM_MODE_CFG: c_uint = 0x2041;
pub const MAX98396_R2042_PCM_CLK_SETUP: c_uint = 0x2042;
pub const MAX98396_R2043_PCM_SR_SETUP: c_uint = 0x2043;
pub const MAX98396_R2044_PCM_TX_CTRL_1: c_uint = 0x2044;
pub const MAX98396_R2045_PCM_TX_CTRL_2: c_uint = 0x2045;
pub const MAX98396_R2046_PCM_TX_CTRL_3: c_uint = 0x2046;
pub const MAX98396_R2047_PCM_TX_CTRL_4: c_uint = 0x2047;
pub const MAX98396_R2048_PCM_TX_CTRL_5: c_uint = 0x2048;
pub const MAX98396_R2049_PCM_TX_CTRL_6: c_uint = 0x2049;
pub const MAX98396_R204A_PCM_TX_CTRL_7: c_uint = 0x204A;
pub const MAX98396_R204B_PCM_TX_CTRL_8: c_uint = 0x204B;
pub const MAX98396_R204C_PCM_TX_HIZ_CTRL_1: c_uint = 0x204C;
pub const MAX98396_R204D_PCM_TX_HIZ_CTRL_2: c_uint = 0x204D;
pub const MAX98396_R204E_PCM_TX_HIZ_CTRL_3: c_uint = 0x204E;
pub const MAX98396_R204F_PCM_TX_HIZ_CTRL_4: c_uint = 0x204F;
pub const MAX98396_R2050_PCM_TX_HIZ_CTRL_5: c_uint = 0x2050;
pub const MAX98396_R2051_PCM_TX_HIZ_CTRL_6: c_uint = 0x2051;
pub const MAX98396_R2052_PCM_TX_HIZ_CTRL_7: c_uint = 0x2052;
pub const MAX98396_R2053_PCM_TX_HIZ_CTRL_8: c_uint = 0x2053;
pub const MAX98396_R2055_PCM_RX_SRC1: c_uint = 0x2055;
pub const MAX98396_R2056_PCM_RX_SRC2: c_uint = 0x2056;
pub const MAX98396_R2058_PCM_BYPASS_SRC: c_uint = 0x2058;
pub const MAX98396_R205D_PCM_TX_SRC_EN: c_uint = 0x205D;
pub const MAX98396_R205E_PCM_RX_EN: c_uint = 0x205E;
pub const MAX98396_R205F_PCM_TX_EN: c_uint = 0x205F;
pub const MAX98396_R2070_ICC_RX_EN_A: c_uint = 0x2070;
pub const MAX98396_R2071_ICC_RX_EN_B: c_uint = 0x2071;
pub const MAX98396_R2072_ICC_TX_CTRL: c_uint = 0x2072;
pub const MAX98396_R207F_ICC_EN: c_uint = 0x207F;
pub const MAX98396_R2083_TONE_GEN_DC_CFG: c_uint = 0x2083;
pub const MAX98396_R2084_TONE_GEN_DC_LVL1: c_uint = 0x2084;
pub const MAX98396_R2085_TONE_GEN_DC_LVL2: c_uint = 0x2085;
pub const MAX98396_R2086_TONE_GEN_DC_LVL3: c_uint = 0x2086;
pub const MAX98396_R208F_TONE_GEN_EN: c_uint = 0x208F;
pub const MAX98396_R2090_AMP_VOL_CTRL: c_uint = 0x2090;
pub const MAX98396_R2091_AMP_PATH_GAIN: c_uint = 0x2091;
pub const MAX98396_R2092_AMP_DSP_CFG: c_uint = 0x2092;
pub const MAX98396_R2093_SSM_CFG: c_uint = 0x2093;
pub const MAX98396_R2094_SPK_CLS_DG_THRESH: c_uint = 0x2094;
pub const MAX98396_R2095_SPK_CLS_DG_HDR: c_uint = 0x2095;
pub const MAX98396_R2096_SPK_CLS_DG_HOLD_TIME: c_uint = 0x2096;
pub const MAX98396_R2097_SPK_CLS_DG_DELAY: c_uint = 0x2097;
pub const MAX98396_R2098_SPK_CLS_DG_MODE: c_uint = 0x2098;
pub const MAX98396_R2099_SPK_CLS_DG_VBAT_LVL: c_uint = 0x2099;
pub const MAX98396_R209A_SPK_EDGE_CTRL: c_uint = 0x209A;
pub const MAX98396_R209C_SPK_EDGE_CTRL1: c_uint = 0x209C;
pub const MAX98396_R209D_SPK_EDGE_CTRL2: c_uint = 0x209D;
pub const MAX98396_R209E_AMP_CLIP_GAIN: c_uint = 0x209E;
pub const MAX98396_R209F_BYPASS_PATH_CFG: c_uint = 0x209F;
pub const MAX98396_R20A0_AMP_SUPPLY_CTL: c_uint = 0x20A0;
pub const MAX98396_R20AF_AMP_EN: c_uint = 0x20AF;
pub const MAX98396_R20B0_ADC_SR: c_uint = 0x20B0;
pub const MAX98396_R20B1_ADC_PVDD_CFG: c_uint = 0x20B1;
pub const MAX98396_R20B2_ADC_VBAT_CFG: c_uint = 0x20B2;
pub const MAX98396_R20B3_ADC_THERMAL_CFG: c_uint = 0x20B3;
pub const MAX98396_R20B4_ADC_READBACK_CTRL1: c_uint = 0x20B4;
pub const MAX98396_R20B5_ADC_READBACK_CTRL2: c_uint = 0x20B5;
pub const MAX98396_R20B6_ADC_PVDD_READBACK_MSB: c_uint = 0x20B6;
pub const MAX98396_R20B7_ADC_PVDD_READBACK_LSB: c_uint = 0x20B7;
pub const MAX98396_R20B8_ADC_VBAT_READBACK_MSB: c_uint = 0x20B8;
pub const MAX98396_R20B9_ADC_VBAT_READBACK_LSB: c_uint = 0x20B9;
pub const MAX98396_R20BA_ADC_TEMP_READBACK_MSB: c_uint = 0x20BA;
pub const MAX98396_R20BB_ADC_TEMP_READBACK_LSB: c_uint = 0x20BB;
pub const MAX98396_R20BC_ADC_LO_PVDD_READBACK_MSB: c_uint = 0x20BC;
pub const MAX98396_R20BD_ADC_LO_PVDD_READBACK_LSB: c_uint = 0x20BD;
pub const MAX98396_R20BE_ADC_LO_VBAT_READBACK_MSB: c_uint = 0x20BE;
pub const MAX98396_R20BF_ADC_LO_VBAT_READBACK_LSB: c_uint = 0x20BF;
pub const MAX98396_R20C7_ADC_CFG: c_uint = 0x20C7;
pub const MAX98396_R20D0_DHT_CFG1: c_uint = 0x20D0;
pub const MAX98396_R20D1_LIMITER_CFG1: c_uint = 0x20D1;
pub const MAX98396_R20D2_LIMITER_CFG2: c_uint = 0x20D2;
pub const MAX98396_R20D3_DHT_CFG2: c_uint = 0x20D3;
pub const MAX98396_R20D4_DHT_CFG3: c_uint = 0x20D4;
pub const MAX98396_R20D5_DHT_CFG4: c_uint = 0x20D5;
pub const MAX98396_R20D6_DHT_HYSTERESIS_CFG: c_uint = 0x20D6;
pub const MAX98396_R20DF_DHT_EN: c_uint = 0x20DF;
pub const MAX98396_R20E0_IV_SENSE_PATH_CFG: c_uint = 0x20E0;
pub const MAX98396_R20E4_IV_SENSE_PATH_EN: c_uint = 0x20E4;
pub const MAX98396_R20E5_BPE_STATE: c_uint = 0x20E5;
pub const MAX98396_R20E6_BPE_L3_THRESH_MSB: c_uint = 0x20E6;
pub const MAX98396_R20E7_BPE_L3_THRESH_LSB: c_uint = 0x20E7;
pub const MAX98396_R20E8_BPE_L2_THRESH_MSB: c_uint = 0x20E8;
pub const MAX98396_R20E9_BPE_L2_THRESH_LSB: c_uint = 0x20E9;
pub const MAX98396_R20EA_BPE_L1_THRESH_MSB: c_uint = 0x20EA;
pub const MAX98396_R20EB_BPE_L1_THRESH_LSB: c_uint = 0x20EB;
pub const MAX98396_R20EC_BPE_L0_THRESH_MSB: c_uint = 0x20EC;
pub const MAX98396_R20ED_BPE_L0_THRESH_LSB: c_uint = 0x20ED;
pub const MAX98396_R20EE_BPE_L3_DWELL_HOLD_TIME: c_uint = 0x20EE;
pub const MAX98396_R20EF_BPE_L2_DWELL_HOLD_TIME: c_uint = 0x20EF;
pub const MAX98396_R20F0_BPE_L1_DWELL_HOLD_TIME: c_uint = 0x20F0;
pub const MAX98396_R20F1_BPE_L0_HOLD_TIME: c_uint = 0x20F1;
pub const MAX98396_R20F2_BPE_L3_ATTACK_REL_STEP: c_uint = 0x20F2;
pub const MAX98396_R20F3_BPE_L2_ATTACK_REL_STEP: c_uint = 0x20F3;
pub const MAX98396_R20F4_BPE_L1_ATTACK_REL_STEP: c_uint = 0x20F4;
pub const MAX98396_R20F5_BPE_L0_ATTACK_REL_STEP: c_uint = 0x20F5;
pub const MAX98396_R20F6_BPE_L3_MAX_GAIN_ATTN: c_uint = 0x20F6;
pub const MAX98396_R20F7_BPE_L2_MAX_GAIN_ATTN: c_uint = 0x20F7;
pub const MAX98396_R20F8_BPE_L1_MAX_GAIN_ATTN: c_uint = 0x20F8;
pub const MAX98396_R20F9_BPE_L0_MAX_GAIN_ATTN: c_uint = 0x20F9;
pub const MAX98396_R20FA_BPE_L3_ATT_REL_RATE: c_uint = 0x20FA;
pub const MAX98396_R20FB_BPE_L2_ATT_REL_RATE: c_uint = 0x20FB;
pub const MAX98396_R20FC_BPE_L1_ATT_REL_RATE: c_uint = 0x20FC;
pub const MAX98396_R20FD_BPE_L0_ATT_REL_RATE: c_uint = 0x20FD;
pub const MAX98396_R20FE_BPE_L3_LIMITER_CFG: c_uint = 0x20FE;
pub const MAX98396_R20FF_BPE_L2_LIMITER_CFG: c_uint = 0x20FF;
pub const MAX98396_R2100_BPE_L1_LIMITER_CFG: c_uint = 0x2100;
pub const MAX98396_R2101_BPE_L0_LIMITER_CFG: c_uint = 0x2101;
pub const MAX98396_R2102_BPE_L3_LIM_ATT_REL_RATE: c_uint = 0x2102;
pub const MAX98396_R2103_BPE_L2_LIM_ATT_REL_RATE: c_uint = 0x2103;
pub const MAX98396_R2104_BPE_L1_LIM_ATT_REL_RATE: c_uint = 0x2104;
pub const MAX98396_R2105_BPE_L0_LIM_ATT_REL_RATE: c_uint = 0x2105;
pub const MAX98396_R2106_BPE_THRESH_HYSTERESIS: c_uint = 0x2106;
pub const MAX98396_R2107_BPE_INFINITE_HOLD_CLR: c_uint = 0x2107;
pub const MAX98396_R2108_BPE_SUPPLY_SRC: c_uint = 0x2108;
pub const MAX98396_R2109_BPE_LOW_STATE: c_uint = 0x2109;
pub const MAX98396_R210A_BPE_LOW_GAIN: c_uint = 0x210A;
pub const MAX98396_R210B_BPE_LOW_LIMITER: c_uint = 0x210B;
pub const MAX98396_R210D_BPE_EN: c_uint = 0x210D;
pub const MAX98396_R210E_AUTO_RESTART: c_uint = 0x210E;
pub const MAX98396_R210F_GLOBAL_EN: c_uint = 0x210F;
pub const MAX98396_R21FF_REVISION_ID: c_uint = 0x21FF;
// MAX98927 Registers
pub const MAX98397_R203A_SPK_MON_THRESH: c_uint = 0x203A;
pub const MAX98397_R204C_PCM_TX_CTRL_9: c_uint = 0x204C;
pub const MAX98397_R204D_PCM_TX_HIZ_CTRL_1: c_uint = 0x204D;
pub const MAX98397_R204E_PCM_TX_HIZ_CTRL_2: c_uint = 0x204E;
pub const MAX98397_R204F_PCM_TX_HIZ_CTRL_3: c_uint = 0x204F;
pub const MAX98397_R2050_PCM_TX_HIZ_CTRL_4: c_uint = 0x2050;
pub const MAX98397_R2051_PCM_TX_HIZ_CTRL_5: c_uint = 0x2051;
pub const MAX98397_R2052_PCM_TX_HIZ_CTRL_6: c_uint = 0x2052;
pub const MAX98397_R2053_PCM_TX_HIZ_CTRL_7: c_uint = 0x2053;
pub const MAX98397_R2054_PCM_TX_HIZ_CTRL_8: c_uint = 0x2054;
pub const MAX98397_R2056_PCM_RX_SRC1: c_uint = 0x2056;
pub const MAX98397_R2057_PCM_RX_SRC2: c_uint = 0x2057;
pub const MAX98397_R2060_PCM_TX_SUPPLY_SEL: c_uint = 0x2060;
pub const MAX98397_R209B_SPK_PATH_WB_ONLY: c_uint = 0x209B;
pub const MAX98397_R20B4_ADC_VDDH_CFG: c_uint = 0x20B4;
pub const MAX98397_R20B5_ADC_READBACK_CTRL1: c_uint = 0x20B5;
pub const MAX98397_R20B6_ADC_READBACK_CTRL2: c_uint = 0x20B6;
pub const MAX98397_R20B7_ADC_PVDD_READBACK_MSB: c_uint = 0x20B7;
pub const MAX98397_R20B8_ADC_PVDD_READBACK_LSB: c_uint = 0x20B8;
pub const MAX98397_R20B9_ADC_VBAT_READBACK_MSB: c_uint = 0x20B9;
pub const MAX98397_R20BA_ADC_VBAT_READBACK_LSB: c_uint = 0x20BA;
pub const MAX98397_R20BB_ADC_TEMP_READBACK_MSB: c_uint = 0x20BB;
pub const MAX98397_R20BC_ADC_TEMP_READBACK_LSB: c_uint = 0x20BC;
pub const MAX98397_R20BD_ADC_VDDH__READBACK_MSB: c_uint = 0x20BD;
pub const MAX98397_R20BE_ADC_VDDH_READBACK_LSB: c_uint = 0x20BE;
pub const MAX98397_R20BF_ADC_LO_PVDD_READBACK_MSB: c_uint = 0x20BF;
pub const MAX98397_R20C0_ADC_LO_PVDD_READBACK_LSB: c_uint = 0x20C0;
pub const MAX98397_R20C1_ADC_LO_VBAT_READBACK_MSB: c_uint = 0x20C1;
pub const MAX98397_R20C2_ADC_LO_VBAT_READBACK_LSB: c_uint = 0x20C2;
pub const MAX98397_R20C3_ADC_LO_VDDH_READBACK_MSB: c_uint = 0x20C3;
pub const MAX98397_R20C4_ADC_LO_VDDH_READBACK_LSB: c_uint = 0x20C4;
pub const MAX98397_R20C5_MEAS_ADC_OPTIMAL_MODE: c_uint = 0x20C5;
pub const MAX98397_R22FF_REVISION_ID: c_uint = 0x22FF;
// Macro flag: #define GET_REG_ADDR_REV_ID(x)\
// MAX98396_R2024_THERM_FOLDBACK_SET

// MAX98396_R2038_CLK_MON_CTRL

// MAX98396_R2039_DATA_MON_CTRL

// MAX98396_R203F_ENABLE_CTRLS

// MAX98396_R2041_PCM_MODE_CFG

// MAX98396_R2042_PCM_CLK_SETUP

// MAX98396_R2043_PCM_SR_SETUP

// MAX98396_R2055_PCM_RX_SRC1

// MAX98396_R2056_PCM_RX_SRC2

// MAX98396_R205E_PCM_RX_EN

// MAX98396_R2092_AMP_DSP_CFG

// MAX98396_R20A0_AMP_SUPPLY_CTL

// MAX98396_R20E0_IV_SENSE_PATH_CFG

// MAX98396_R210E_AUTO_RESTART_BEHAVIOR

pub const MAX98396_NUM_CORE_SUPPLIES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98396_priv {
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub core_supplies: [regulator_bulk_data; MAX98396_NUM_CORE_SUPPLIES],
    pub vbat: *mut *mut regulator pvdd,,
    pub v_slot: c_uint,
    pub i_slot: c_uint,
    pub spkfb_slot: c_uint,
    pub bypass_slot: c_uint,
    pub dmon_stuck_enable: bool,
    pub dmon_stuck_threshold: c_uint,
    pub dmon_mag_enable: bool,
    pub dmon_mag_threshold: c_uint,
    pub dmon_duration: c_uint,
    pub interleave_mode: bool,
    pub tdm_mode: bool,
    pub tdm_max_samplerate: c_int,
    pub device_id: c_int,
}
