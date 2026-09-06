//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max98095.h
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
// max98095.h -- MAX98095 ALSA SoC Audio driver
//
// Copyright 2011 Maxim Integrated Products
//
// MAX98095 Registers Definition
//
pub const M98095_000_HOST_DATA: c_uint = 0x00;
pub const M98095_001_HOST_INT_STS: c_uint = 0x01;
pub const M98095_002_HOST_RSP_STS: c_uint = 0x02;
pub const M98095_003_HOST_CMD_STS: c_uint = 0x03;
pub const M98095_004_CODEC_STS: c_uint = 0x04;
pub const M98095_005_DAI1_ALC_STS: c_uint = 0x05;
pub const M98095_006_DAI2_ALC_STS: c_uint = 0x06;
pub const M98095_007_JACK_AUTO_STS: c_uint = 0x07;
pub const M98095_008_JACK_MANUAL_STS: c_uint = 0x08;
pub const M98095_009_JACK_VBAT_STS: c_uint = 0x09;
pub const M98095_00A_ACC_ADC_STS: c_uint = 0x0A;
pub const M98095_00B_MIC_NG_AGC_STS: c_uint = 0x0B;
pub const M98095_00C_SPK_L_VOLT_STS: c_uint = 0x0C;
pub const M98095_00D_SPK_R_VOLT_STS: c_uint = 0x0D;
pub const M98095_00E_TEMP_SENSOR_STS: c_uint = 0x0E;
pub const M98095_00F_HOST_CFG: c_uint = 0x0F;
pub const M98095_010_HOST_INT_CFG: c_uint = 0x10;
pub const M98095_011_HOST_INT_EN: c_uint = 0x11;
pub const M98095_012_CODEC_INT_EN: c_uint = 0x12;
pub const M98095_013_JACK_INT_EN: c_uint = 0x13;
pub const M98095_014_JACK_INT_EN: c_uint = 0x14;
pub const M98095_015_DEC: c_uint = 0x15;
pub const M98095_016_RESERVED: c_uint = 0x16;
pub const M98095_017_RESERVED: c_uint = 0x17;
pub const M98095_018_KEYCODE3: c_uint = 0x18;
pub const M98095_019_KEYCODE2: c_uint = 0x19;
pub const M98095_01A_KEYCODE1: c_uint = 0x1A;
pub const M98095_01B_KEYCODE0: c_uint = 0x1B;
pub const M98095_01C_OEMCODE1: c_uint = 0x1C;
pub const M98095_01D_OEMCODE0: c_uint = 0x1D;
pub const M98095_01E_XCFG1: c_uint = 0x1E;
pub const M98095_01F_XCFG2: c_uint = 0x1F;
pub const M98095_020_XCFG3: c_uint = 0x20;
pub const M98095_021_XCFG4: c_uint = 0x21;
pub const M98095_022_XCFG5: c_uint = 0x22;
pub const M98095_023_XCFG6: c_uint = 0x23;
pub const M98095_024_XGPIO: c_uint = 0x24;
pub const M98095_025_XCLKCFG: c_uint = 0x25;
pub const M98095_026_SYS_CLK: c_uint = 0x26;
pub const M98095_027_DAI1_CLKMODE: c_uint = 0x27;
pub const M98095_028_DAI1_CLKCFG_HI: c_uint = 0x28;
pub const M98095_029_DAI1_CLKCFG_LO: c_uint = 0x29;
pub const M98095_02A_DAI1_FORMAT: c_uint = 0x2A;
pub const M98095_02B_DAI1_CLOCK: c_uint = 0x2B;
pub const M98095_02C_DAI1_IOCFG: c_uint = 0x2C;
pub const M98095_02D_DAI1_TDM: c_uint = 0x2D;
pub const M98095_02E_DAI1_FILTERS: c_uint = 0x2E;
pub const M98095_02F_DAI1_LVL1: c_uint = 0x2F;
pub const M98095_030_DAI1_LVL2: c_uint = 0x30;
pub const M98095_031_DAI2_CLKMODE: c_uint = 0x31;
pub const M98095_032_DAI2_CLKCFG_HI: c_uint = 0x32;
pub const M98095_033_DAI2_CLKCFG_LO: c_uint = 0x33;
pub const M98095_034_DAI2_FORMAT: c_uint = 0x34;
pub const M98095_035_DAI2_CLOCK: c_uint = 0x35;
pub const M98095_036_DAI2_IOCFG: c_uint = 0x36;
pub const M98095_037_DAI2_TDM: c_uint = 0x37;
pub const M98095_038_DAI2_FILTERS: c_uint = 0x38;
pub const M98095_039_DAI2_LVL1: c_uint = 0x39;
pub const M98095_03A_DAI2_LVL2: c_uint = 0x3A;
pub const M98095_03B_DAI3_CLKMODE: c_uint = 0x3B;
pub const M98095_03C_DAI3_CLKCFG_HI: c_uint = 0x3C;
pub const M98095_03D_DAI3_CLKCFG_LO: c_uint = 0x3D;
pub const M98095_03E_DAI3_FORMAT: c_uint = 0x3E;
pub const M98095_03F_DAI3_CLOCK: c_uint = 0x3F;
pub const M98095_040_DAI3_IOCFG: c_uint = 0x40;
pub const M98095_041_DAI3_TDM: c_uint = 0x41;
pub const M98095_042_DAI3_FILTERS: c_uint = 0x42;
pub const M98095_043_DAI3_LVL1: c_uint = 0x43;
pub const M98095_044_DAI3_LVL2: c_uint = 0x44;
pub const M98095_045_CFG_DSP: c_uint = 0x45;
pub const M98095_046_DAC_CTRL1: c_uint = 0x46;
pub const M98095_047_DAC_CTRL2: c_uint = 0x47;
pub const M98095_048_MIX_DAC_LR: c_uint = 0x48;
pub const M98095_049_MIX_DAC_M: c_uint = 0x49;
pub const M98095_04A_MIX_ADC_LEFT: c_uint = 0x4A;
pub const M98095_04B_MIX_ADC_RIGHT: c_uint = 0x4B;
pub const M98095_04C_MIX_HP_LEFT: c_uint = 0x4C;
pub const M98095_04D_MIX_HP_RIGHT: c_uint = 0x4D;
pub const M98095_04E_CFG_HP: c_uint = 0x4E;
pub const M98095_04F_MIX_RCV: c_uint = 0x4F;
pub const M98095_050_MIX_SPK_LEFT: c_uint = 0x50;
pub const M98095_051_MIX_SPK_RIGHT: c_uint = 0x51;
pub const M98095_052_MIX_SPK_CFG: c_uint = 0x52;
pub const M98095_053_MIX_LINEOUT1: c_uint = 0x53;
pub const M98095_054_MIX_LINEOUT2: c_uint = 0x54;
pub const M98095_055_MIX_LINEOUT_CFG: c_uint = 0x55;
pub const M98095_056_LVL_SIDETONE_DAI12: c_uint = 0x56;
pub const M98095_057_LVL_SIDETONE_DAI3: c_uint = 0x57;
pub const M98095_058_LVL_DAI1_PLAY: c_uint = 0x58;
pub const M98095_059_LVL_DAI1_EQ: c_uint = 0x59;
pub const M98095_05A_LVL_DAI2_PLAY: c_uint = 0x5A;
pub const M98095_05B_LVL_DAI2_EQ: c_uint = 0x5B;
pub const M98095_05C_LVL_DAI3_PLAY: c_uint = 0x5C;
pub const M98095_05D_LVL_ADC_L: c_uint = 0x5D;
pub const M98095_05E_LVL_ADC_R: c_uint = 0x5E;
pub const M98095_05F_LVL_MIC1: c_uint = 0x5F;
pub const M98095_060_LVL_MIC2: c_uint = 0x60;
pub const M98095_061_LVL_LINEIN: c_uint = 0x61;
pub const M98095_062_LVL_LINEOUT1: c_uint = 0x62;
pub const M98095_063_LVL_LINEOUT2: c_uint = 0x63;
pub const M98095_064_LVL_HP_L: c_uint = 0x64;
pub const M98095_065_LVL_HP_R: c_uint = 0x65;
pub const M98095_066_LVL_RCV: c_uint = 0x66;
pub const M98095_067_LVL_SPK_L: c_uint = 0x67;
pub const M98095_068_LVL_SPK_R: c_uint = 0x68;
pub const M98095_069_MICAGC_CFG: c_uint = 0x69;
pub const M98095_06A_MICAGC_THRESH: c_uint = 0x6A;
pub const M98095_06B_SPK_NOISEGATE: c_uint = 0x6B;
pub const M98095_06C_DAI1_ALC1_TIME: c_uint = 0x6C;
pub const M98095_06D_DAI1_ALC1_COMP: c_uint = 0x6D;
pub const M98095_06E_DAI1_ALC1_EXPN: c_uint = 0x6E;
pub const M98095_06F_DAI1_ALC1_GAIN: c_uint = 0x6F;
pub const M98095_070_DAI1_ALC2_TIME: c_uint = 0x70;
pub const M98095_071_DAI1_ALC2_COMP: c_uint = 0x71;
pub const M98095_072_DAI1_ALC2_EXPN: c_uint = 0x72;
pub const M98095_073_DAI1_ALC2_GAIN: c_uint = 0x73;
pub const M98095_074_DAI1_ALC3_TIME: c_uint = 0x74;
pub const M98095_075_DAI1_ALC3_COMP: c_uint = 0x75;
pub const M98095_076_DAI1_ALC3_EXPN: c_uint = 0x76;
pub const M98095_077_DAI1_ALC3_GAIN: c_uint = 0x77;
pub const M98095_078_DAI2_ALC1_TIME: c_uint = 0x78;
pub const M98095_079_DAI2_ALC1_COMP: c_uint = 0x79;
pub const M98095_07A_DAI2_ALC1_EXPN: c_uint = 0x7A;
pub const M98095_07B_DAI2_ALC1_GAIN: c_uint = 0x7B;
pub const M98095_07C_DAI2_ALC2_TIME: c_uint = 0x7C;
pub const M98095_07D_DAI2_ALC2_COMP: c_uint = 0x7D;
pub const M98095_07E_DAI2_ALC2_EXPN: c_uint = 0x7E;
pub const M98095_07F_DAI2_ALC2_GAIN: c_uint = 0x7F;
pub const M98095_080_DAI2_ALC3_TIME: c_uint = 0x80;
pub const M98095_081_DAI2_ALC3_COMP: c_uint = 0x81;
pub const M98095_082_DAI2_ALC3_EXPN: c_uint = 0x82;
pub const M98095_083_DAI2_ALC3_GAIN: c_uint = 0x83;
pub const M98095_084_HP_NOISE_GATE: c_uint = 0x84;
pub const M98095_085_AUX_ADC: c_uint = 0x85;
pub const M98095_086_CFG_LINE: c_uint = 0x86;
pub const M98095_087_CFG_MIC: c_uint = 0x87;
pub const M98095_088_CFG_LEVEL: c_uint = 0x88;
pub const M98095_089_JACK_DET_AUTO: c_uint = 0x89;
pub const M98095_08A_JACK_DET_MANUAL: c_uint = 0x8A;
pub const M98095_08B_JACK_KEYSCAN_DBC: c_uint = 0x8B;
pub const M98095_08C_JACK_KEYSCAN_DLY: c_uint = 0x8C;
pub const M98095_08D_JACK_KEY_THRESH: c_uint = 0x8D;
pub const M98095_08E_JACK_DC_SLEW: c_uint = 0x8E;
pub const M98095_08F_JACK_TEST_CFG: c_uint = 0x8F;
pub const M98095_090_PWR_EN_IN: c_uint = 0x90;
pub const M98095_091_PWR_EN_OUT: c_uint = 0x91;
pub const M98095_092_PWR_EN_OUT: c_uint = 0x92;
pub const M98095_093_BIAS_CTRL: c_uint = 0x93;
pub const M98095_094_PWR_DAC_21: c_uint = 0x94;
pub const M98095_095_PWR_DAC_03: c_uint = 0x95;
pub const M98095_096_PWR_DAC_CK: c_uint = 0x96;
pub const M98095_097_PWR_SYS: c_uint = 0x97;
pub const M98095_0FF_REV_ID: c_uint = 0xFF;

// MAX98095 Registers Bit Fields
// M98095_007_JACK_AUTO_STS

// M98095_00F_HOST_CFG

// M98095_013_JACK_INT_EN

// M98095_027_DAI1_CLKMODE, M98095_031_DAI2_CLKMODE, M98095_03B_DAI3_CLKMODE
pub const M98095_CLKMODE_MASK: c_uint = 0xFF;
// M98095_02A_DAI1_FORMAT, M98095_034_DAI2_FORMAT, M98095_03E_DAI3_FORMAT

// M98095_02B_DAI1_CLOCK, M98095_035_DAI2_CLOCK, M98095_03F_DAI3_CLOCK

// M98095_02C_DAI1_IOCFG, M98095_036_DAI2_IOCFG, M98095_040_DAI3_IOCFG

// M98095_02E_DAI1_FILTERS, M98095_038_DAI2_FILTERS, M98095_042_DAI3_FILTERS

// M98095_045_DSP_CFG

// M98095_048_MIX_DAC_LR

// M98095_049_MIX_DAC_M

// M98095_04E_MIX_HP_CFG

// M98095_05F_LVL_MIC1, M98095_060_LVL_MIC2

pub const M98095_MICPRE_SHIFT: c_int = 5;
// M98095_064_LVL_HP_L, M98095_065_LVL_HP_R

// M98095_066_LVL_RCV

// M98095_067_LVL_SPK_L, M98095_068_LVL_SPK_R

// M98095_087_CFG_MIC

// M98095_088_CFG_LEVEL

// M98095_089_JACK_DET_AUTO

// M98095_090_PWR_EN_IN

// M98095_091_PWR_EN_OUT

// M98095_092_PWR_EN_OUT

// M98095_097_PWR_SYS

pub const M98095_COEFS_PER_BAND: c_int = 5;

// Equalizer filter coefficients
pub const M98095_110_DAI1_EQ_BASE: c_uint = 0x10;
pub const M98095_142_DAI2_EQ_BASE: c_uint = 0x42;
// Biquad filter coefficients
pub const M98095_174_DAI1_BQ_BASE: c_uint = 0x74;
pub const M98095_17E_DAI2_BQ_BASE: c_uint = 0x7E;
// Default Delay used in Slew Rate Calculation for Jack detection
pub const M98095_DEFAULT_SLEW_DELAY: c_uint = 0x18;
