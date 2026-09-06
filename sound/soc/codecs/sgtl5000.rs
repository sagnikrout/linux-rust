//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/sgtl5000.h
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
// sgtl5000.h - SGTL5000 audio codec interface
//
// Copyright 2010-2011 Freescale Semiconductor, Inc.
//
// Registers addresses
//
pub const SGTL5000_CHIP_ID: c_uint = 0x0000;
pub const SGTL5000_CHIP_DIG_POWER: c_uint = 0x0002;
pub const SGTL5000_CHIP_CLK_CTRL: c_uint = 0x0004;
pub const SGTL5000_CHIP_I2S_CTRL: c_uint = 0x0006;
pub const SGTL5000_CHIP_SSS_CTRL: c_uint = 0x000a;
pub const SGTL5000_CHIP_ADCDAC_CTRL: c_uint = 0x000e;
pub const SGTL5000_CHIP_DAC_VOL: c_uint = 0x0010;
pub const SGTL5000_CHIP_PAD_STRENGTH: c_uint = 0x0014;
pub const SGTL5000_CHIP_ANA_ADC_CTRL: c_uint = 0x0020;
pub const SGTL5000_CHIP_ANA_HP_CTRL: c_uint = 0x0022;
pub const SGTL5000_CHIP_ANA_CTRL: c_uint = 0x0024;
pub const SGTL5000_CHIP_LINREG_CTRL: c_uint = 0x0026;
pub const SGTL5000_CHIP_REF_CTRL: c_uint = 0x0028;
pub const SGTL5000_CHIP_MIC_CTRL: c_uint = 0x002a;
pub const SGTL5000_CHIP_LINE_OUT_CTRL: c_uint = 0x002c;
pub const SGTL5000_CHIP_LINE_OUT_VOL: c_uint = 0x002e;
pub const SGTL5000_CHIP_ANA_POWER: c_uint = 0x0030;
pub const SGTL5000_CHIP_PLL_CTRL: c_uint = 0x0032;
pub const SGTL5000_CHIP_CLK_TOP_CTRL: c_uint = 0x0034;
pub const SGTL5000_CHIP_ANA_STATUS: c_uint = 0x0036;
pub const SGTL5000_CHIP_SHORT_CTRL: c_uint = 0x003c;
pub const SGTL5000_CHIP_ANA_TEST2: c_uint = 0x003a;
pub const SGTL5000_DAP_CTRL: c_uint = 0x0100;
pub const SGTL5000_DAP_PEQ: c_uint = 0x0102;
pub const SGTL5000_DAP_BASS_ENHANCE: c_uint = 0x0104;
pub const SGTL5000_DAP_BASS_ENHANCE_CTRL: c_uint = 0x0106;
pub const SGTL5000_DAP_AUDIO_EQ: c_uint = 0x0108;
pub const SGTL5000_DAP_SURROUND: c_uint = 0x010a;
pub const SGTL5000_DAP_FLT_COEF_ACCESS: c_uint = 0x010c;
pub const SGTL5000_DAP_COEF_WR_B0_MSB: c_uint = 0x010e;
pub const SGTL5000_DAP_COEF_WR_B0_LSB: c_uint = 0x0110;
pub const SGTL5000_DAP_EQ_BASS_BAND0: c_uint = 0x0116;
pub const SGTL5000_DAP_EQ_BASS_BAND1: c_uint = 0x0118;
pub const SGTL5000_DAP_EQ_BASS_BAND2: c_uint = 0x011a;
pub const SGTL5000_DAP_EQ_BASS_BAND3: c_uint = 0x011c;
pub const SGTL5000_DAP_EQ_BASS_BAND4: c_uint = 0x011e;
pub const SGTL5000_DAP_MAIN_CHAN: c_uint = 0x0120;
pub const SGTL5000_DAP_MIX_CHAN: c_uint = 0x0122;
pub const SGTL5000_DAP_AVC_CTRL: c_uint = 0x0124;
pub const SGTL5000_DAP_AVC_THRESHOLD: c_uint = 0x0126;
pub const SGTL5000_DAP_AVC_ATTACK: c_uint = 0x0128;
pub const SGTL5000_DAP_AVC_DECAY: c_uint = 0x012a;
pub const SGTL5000_DAP_COEF_WR_B1_MSB: c_uint = 0x012c;
pub const SGTL5000_DAP_COEF_WR_B1_LSB: c_uint = 0x012e;
pub const SGTL5000_DAP_COEF_WR_B2_MSB: c_uint = 0x0130;
pub const SGTL5000_DAP_COEF_WR_B2_LSB: c_uint = 0x0132;
pub const SGTL5000_DAP_COEF_WR_A1_MSB: c_uint = 0x0134;
pub const SGTL5000_DAP_COEF_WR_A1_LSB: c_uint = 0x0136;
pub const SGTL5000_DAP_COEF_WR_A2_MSB: c_uint = 0x0138;
pub const SGTL5000_DAP_COEF_WR_A2_LSB: c_uint = 0x013a;
//
// Field Definitions.
//
// SGTL5000_CHIP_ID
//
pub const SGTL5000_PARTID_MASK: c_uint = 0xff00;
pub const SGTL5000_PARTID_SHIFT: c_int = 8;
pub const SGTL5000_PARTID_WIDTH: c_int = 8;
pub const SGTL5000_PARTID_PART_ID: c_uint = 0xa0;
pub const SGTL5000_REVID_MASK: c_uint = 0x00ff;
pub const SGTL5000_REVID_SHIFT: c_int = 0;
pub const SGTL5000_REVID_WIDTH: c_int = 8;
//
// SGTL5000_CHIP_DIG_POWER
//
pub const SGTL5000_DIG_POWER_DEFAULT: c_uint = 0x0000;
pub const SGTL5000_ADC_EN: c_uint = 0x0040;
pub const SGTL5000_DAC_EN: c_uint = 0x0020;
pub const SGTL5000_DAP_POWERUP: c_uint = 0x0010;
pub const SGTL5000_I2S_OUT_POWERUP: c_uint = 0x0002;
pub const SGTL5000_I2S_IN_POWERUP: c_uint = 0x0001;
//
// SGTL5000_CHIP_CLK_CTRL
//
pub const SGTL5000_CHIP_CLK_CTRL_DEFAULT: c_uint = 0x0008;
pub const SGTL5000_RATE_MODE_MASK: c_uint = 0x0030;
pub const SGTL5000_RATE_MODE_SHIFT: c_int = 4;
pub const SGTL5000_RATE_MODE_WIDTH: c_int = 2;
pub const SGTL5000_RATE_MODE_DIV_1: c_int = 0;
pub const SGTL5000_RATE_MODE_DIV_2: c_int = 1;
pub const SGTL5000_RATE_MODE_DIV_4: c_int = 2;
pub const SGTL5000_RATE_MODE_DIV_6: c_int = 3;
pub const SGTL5000_SYS_FS_MASK: c_uint = 0x000c;
pub const SGTL5000_SYS_FS_SHIFT: c_int = 2;
pub const SGTL5000_SYS_FS_WIDTH: c_int = 2;
pub const SGTL5000_SYS_FS_32k: c_uint = 0x0;
pub const SGTL5000_SYS_FS_44_1k: c_uint = 0x1;
pub const SGTL5000_SYS_FS_48k: c_uint = 0x2;
pub const SGTL5000_SYS_FS_96k: c_uint = 0x3;
pub const SGTL5000_MCLK_FREQ_MASK: c_uint = 0x0003;
pub const SGTL5000_MCLK_FREQ_SHIFT: c_int = 0;
pub const SGTL5000_MCLK_FREQ_WIDTH: c_int = 2;
pub const SGTL5000_MCLK_FREQ_256FS: c_uint = 0x0;
pub const SGTL5000_MCLK_FREQ_384FS: c_uint = 0x1;
pub const SGTL5000_MCLK_FREQ_512FS: c_uint = 0x2;
pub const SGTL5000_MCLK_FREQ_PLL: c_uint = 0x3;
//
// SGTL5000_CHIP_I2S_CTRL
//
pub const SGTL5000_I2S_SCLKFREQ_MASK: c_uint = 0x0100;
pub const SGTL5000_I2S_SCLKFREQ_SHIFT: c_int = 8;
pub const SGTL5000_I2S_SCLKFREQ_WIDTH: c_int = 1;
pub const SGTL5000_I2S_SCLKFREQ_64FS: c_uint = 0x0;
pub const SGTL5000_I2S_SCLKFREQ_32FS: c_uint = 0x1	/* Not for RJ mode */;
pub const SGTL5000_I2S_MASTER: c_uint = 0x0080;
pub const SGTL5000_I2S_SCLK_INV: c_uint = 0x0040;
pub const SGTL5000_I2S_DLEN_MASK: c_uint = 0x0030;
pub const SGTL5000_I2S_DLEN_SHIFT: c_int = 4;
pub const SGTL5000_I2S_DLEN_WIDTH: c_int = 2;
pub const SGTL5000_I2S_DLEN_32: c_uint = 0x0;
pub const SGTL5000_I2S_DLEN_24: c_uint = 0x1;
pub const SGTL5000_I2S_DLEN_20: c_uint = 0x2;
pub const SGTL5000_I2S_DLEN_16: c_uint = 0x3;
pub const SGTL5000_I2S_MODE_MASK: c_uint = 0x000c;
pub const SGTL5000_I2S_MODE_SHIFT: c_int = 2;
pub const SGTL5000_I2S_MODE_WIDTH: c_int = 2;
pub const SGTL5000_I2S_MODE_I2S_LJ: c_uint = 0x0;
pub const SGTL5000_I2S_MODE_RJ: c_uint = 0x1;
pub const SGTL5000_I2S_MODE_PCM: c_uint = 0x2;
pub const SGTL5000_I2S_LRALIGN: c_uint = 0x0002;
pub const SGTL5000_I2S_LRPOL: c_uint = 0x0001	/* set for which mode */;
//
// SGTL5000_CHIP_SSS_CTRL
//
pub const SGTL5000_DAP_MIX_LRSWAP: c_uint = 0x4000;
pub const SGTL5000_DAP_LRSWAP: c_uint = 0x2000;
pub const SGTL5000_DAC_LRSWAP: c_uint = 0x1000;
pub const SGTL5000_I2S_OUT_LRSWAP: c_uint = 0x0400;
pub const SGTL5000_DAP_MIX_SEL_MASK: c_uint = 0x0300;
pub const SGTL5000_DAP_MIX_SEL_SHIFT: c_int = 8;
pub const SGTL5000_DAP_MIX_SEL_WIDTH: c_int = 2;
pub const SGTL5000_DAP_MIX_SEL_ADC: c_uint = 0x0;
pub const SGTL5000_DAP_MIX_SEL_I2S_IN: c_uint = 0x1;
pub const SGTL5000_DAP_SEL_MASK: c_uint = 0x00c0;
pub const SGTL5000_DAP_SEL_SHIFT: c_int = 6;
pub const SGTL5000_DAP_SEL_WIDTH: c_int = 2;
pub const SGTL5000_DAP_SEL_ADC: c_uint = 0x0;
pub const SGTL5000_DAP_SEL_I2S_IN: c_uint = 0x1;
pub const SGTL5000_DAC_SEL_MASK: c_uint = 0x0030;
pub const SGTL5000_DAC_SEL_SHIFT: c_int = 4;
pub const SGTL5000_DAC_SEL_WIDTH: c_int = 2;
pub const SGTL5000_DAC_SEL_ADC: c_uint = 0x0;
pub const SGTL5000_DAC_SEL_I2S_IN: c_uint = 0x1;
pub const SGTL5000_DAC_SEL_DAP: c_uint = 0x3;
pub const SGTL5000_I2S_OUT_SEL_MASK: c_uint = 0x0003;
pub const SGTL5000_I2S_OUT_SEL_SHIFT: c_int = 0;
pub const SGTL5000_I2S_OUT_SEL_WIDTH: c_int = 2;
pub const SGTL5000_I2S_OUT_SEL_ADC: c_uint = 0x0;
pub const SGTL5000_I2S_OUT_SEL_I2S_IN: c_uint = 0x1;
pub const SGTL5000_I2S_OUT_SEL_DAP: c_uint = 0x3;
//
// SGTL5000_CHIP_ADCDAC_CTRL
//
pub const SGTL5000_VOL_BUSY_DAC_RIGHT: c_uint = 0x2000;
pub const SGTL5000_VOL_BUSY_DAC_LEFT: c_uint = 0x1000;
pub const SGTL5000_DAC_VOL_RAMP_EN: c_uint = 0x0200;
pub const SGTL5000_DAC_VOL_RAMP_EXPO: c_uint = 0x0100;
pub const SGTL5000_DAC_MUTE_RIGHT: c_uint = 0x0008;
pub const SGTL5000_DAC_MUTE_LEFT: c_uint = 0x0004;
pub const SGTL5000_ADC_HPF_FREEZE: c_uint = 0x0002;
pub const SGTL5000_ADC_HPF_BYPASS: c_uint = 0x0001;
//
// SGTL5000_CHIP_DAC_VOL
//
pub const SGTL5000_DAC_VOL_RIGHT_MASK: c_uint = 0xff00;
pub const SGTL5000_DAC_VOL_RIGHT_SHIFT: c_int = 8;
pub const SGTL5000_DAC_VOL_RIGHT_WIDTH: c_int = 8;
pub const SGTL5000_DAC_VOL_LEFT_MASK: c_uint = 0x00ff;
pub const SGTL5000_DAC_VOL_LEFT_SHIFT: c_int = 0;
pub const SGTL5000_DAC_VOL_LEFT_WIDTH: c_int = 8;
//
// SGTL5000_CHIP_PAD_STRENGTH
//
pub const SGTL5000_PAD_I2S_LRCLK_MASK: c_uint = 0x0300;
pub const SGTL5000_PAD_I2S_LRCLK_SHIFT: c_int = 8;
pub const SGTL5000_PAD_I2S_LRCLK_WIDTH: c_int = 2;
pub const SGTL5000_PAD_I2S_SCLK_MASK: c_uint = 0x00c0;
pub const SGTL5000_PAD_I2S_SCLK_SHIFT: c_int = 6;
pub const SGTL5000_PAD_I2S_SCLK_WIDTH: c_int = 2;
pub const SGTL5000_PAD_I2S_DOUT_MASK: c_uint = 0x0030;
pub const SGTL5000_PAD_I2S_DOUT_SHIFT: c_int = 4;
pub const SGTL5000_PAD_I2S_DOUT_WIDTH: c_int = 2;
pub const SGTL5000_PAD_I2C_SDA_MASK: c_uint = 0x000c;
pub const SGTL5000_PAD_I2C_SDA_SHIFT: c_int = 2;
pub const SGTL5000_PAD_I2C_SDA_WIDTH: c_int = 2;
pub const SGTL5000_PAD_I2C_SCL_MASK: c_uint = 0x0003;
pub const SGTL5000_PAD_I2C_SCL_SHIFT: c_int = 0;
pub const SGTL5000_PAD_I2C_SCL_WIDTH: c_int = 2;
//
// SGTL5000_CHIP_ANA_ADC_CTRL
//
pub const SGTL5000_ADC_VOL_M6DB: c_uint = 0x0100;
pub const SGTL5000_ADC_VOL_RIGHT_MASK: c_uint = 0x00f0;
pub const SGTL5000_ADC_VOL_RIGHT_SHIFT: c_int = 4;
pub const SGTL5000_ADC_VOL_RIGHT_WIDTH: c_int = 4;
pub const SGTL5000_ADC_VOL_LEFT_MASK: c_uint = 0x000f;
pub const SGTL5000_ADC_VOL_LEFT_SHIFT: c_int = 0;
pub const SGTL5000_ADC_VOL_LEFT_WIDTH: c_int = 4;
//
// SGTL5000_CHIP_ANA_HP_CTRL
//
pub const SGTL5000_HP_VOL_RIGHT_MASK: c_uint = 0x7f00;
pub const SGTL5000_HP_VOL_RIGHT_SHIFT: c_int = 8;
pub const SGTL5000_HP_VOL_RIGHT_WIDTH: c_int = 7;
pub const SGTL5000_HP_VOL_LEFT_MASK: c_uint = 0x007f;
pub const SGTL5000_HP_VOL_LEFT_SHIFT: c_int = 0;
pub const SGTL5000_HP_VOL_LEFT_WIDTH: c_int = 7;
//
// SGTL5000_CHIP_ANA_CTRL
//
pub const SGTL5000_CHIP_ANA_CTRL_DEFAULT: c_uint = 0x0133;
pub const SGTL5000_LINE_OUT_MUTE: c_uint = 0x0100;
pub const SGTL5000_HP_SEL_MASK: c_uint = 0x0040;
pub const SGTL5000_HP_SEL_SHIFT: c_int = 6;
pub const SGTL5000_HP_SEL_WIDTH: c_int = 1;
pub const SGTL5000_HP_SEL_DAC: c_uint = 0x0;
pub const SGTL5000_HP_SEL_LINE_IN: c_uint = 0x1;
pub const SGTL5000_HP_ZCD_EN: c_uint = 0x0020;
pub const SGTL5000_HP_MUTE: c_uint = 0x0010;
pub const SGTL5000_ADC_SEL_MASK: c_uint = 0x0004;
pub const SGTL5000_ADC_SEL_SHIFT: c_int = 2;
pub const SGTL5000_ADC_SEL_WIDTH: c_int = 1;
pub const SGTL5000_ADC_SEL_MIC: c_uint = 0x0;
pub const SGTL5000_ADC_SEL_LINE_IN: c_uint = 0x1;
pub const SGTL5000_ADC_ZCD_EN: c_uint = 0x0002;
pub const SGTL5000_ADC_MUTE: c_uint = 0x0001;
//
// SGTL5000_CHIP_LINREG_CTRL
//
pub const SGTL5000_VDDC_MAN_ASSN_MASK: c_uint = 0x0040;
pub const SGTL5000_VDDC_MAN_ASSN_SHIFT: c_int = 6;
pub const SGTL5000_VDDC_MAN_ASSN_WIDTH: c_int = 1;
pub const SGTL5000_VDDC_MAN_ASSN_VDDA: c_uint = 0x0;
pub const SGTL5000_VDDC_MAN_ASSN_VDDIO: c_uint = 0x1;
pub const SGTL5000_VDDC_ASSN_OVRD: c_uint = 0x0020;
pub const SGTL5000_LINREG_VDDD_MASK: c_uint = 0x000f;
pub const SGTL5000_LINREG_VDDD_SHIFT: c_int = 0;
pub const SGTL5000_LINREG_VDDD_WIDTH: c_int = 4;
//
// SGTL5000_CHIP_REF_CTRL
//
pub const SGTL5000_ANA_GND_MASK: c_uint = 0x01f0;
pub const SGTL5000_ANA_GND_SHIFT: c_int = 4;
pub const SGTL5000_ANA_GND_WIDTH: c_int = 5;

pub const SGTL5000_BIAS_CTRL_MASK: c_uint = 0x000e;
pub const SGTL5000_BIAS_CTRL_SHIFT: c_int = 1;
pub const SGTL5000_BIAS_CTRL_WIDTH: c_int = 3;
pub const SGTL5000_SMALL_POP: c_uint = 0x0001;
//
// SGTL5000_CHIP_MIC_CTRL
//
pub const SGTL5000_BIAS_R_MASK: c_uint = 0x0300;
pub const SGTL5000_BIAS_R_SHIFT: c_int = 8;
pub const SGTL5000_BIAS_R_WIDTH: c_int = 2;
pub const SGTL5000_BIAS_R_off: c_uint = 0x0;
pub const SGTL5000_BIAS_R_2K: c_uint = 0x1;
pub const SGTL5000_BIAS_R_4k: c_uint = 0x2;
pub const SGTL5000_BIAS_R_8k: c_uint = 0x3;
pub const SGTL5000_BIAS_VOLT_MASK: c_uint = 0x0070;
pub const SGTL5000_BIAS_VOLT_SHIFT: c_int = 4;
pub const SGTL5000_BIAS_VOLT_WIDTH: c_int = 3;
pub const SGTL5000_MIC_GAIN_MASK: c_uint = 0x0003;
pub const SGTL5000_MIC_GAIN_SHIFT: c_int = 0;
pub const SGTL5000_MIC_GAIN_WIDTH: c_int = 2;
//
// SGTL5000_CHIP_LINE_OUT_CTRL
//
pub const SGTL5000_LINE_OUT_CURRENT_MASK: c_uint = 0x0f00;
pub const SGTL5000_LINE_OUT_CURRENT_SHIFT: c_int = 8;
pub const SGTL5000_LINE_OUT_CURRENT_WIDTH: c_int = 4;
pub const SGTL5000_LINE_OUT_CURRENT_180u: c_uint = 0x0;
pub const SGTL5000_LINE_OUT_CURRENT_270u: c_uint = 0x1;
pub const SGTL5000_LINE_OUT_CURRENT_360u: c_uint = 0x3;
pub const SGTL5000_LINE_OUT_CURRENT_450u: c_uint = 0x7;
pub const SGTL5000_LINE_OUT_CURRENT_540u: c_uint = 0xf;
pub const SGTL5000_LINE_OUT_GND_MASK: c_uint = 0x003f;
pub const SGTL5000_LINE_OUT_GND_SHIFT: c_int = 0;
pub const SGTL5000_LINE_OUT_GND_WIDTH: c_int = 6;

pub const SGTL5000_LINE_OUT_GND_STP: c_int = 25;
pub const SGTL5000_LINE_OUT_GND_MAX: c_uint = 0x23;
//
// SGTL5000_CHIP_LINE_OUT_VOL
//
pub const SGTL5000_LINE_OUT_VOL_RIGHT_MASK: c_uint = 0x1f00;
pub const SGTL5000_LINE_OUT_VOL_RIGHT_SHIFT: c_int = 8;
pub const SGTL5000_LINE_OUT_VOL_RIGHT_WIDTH: c_int = 5;
pub const SGTL5000_LINE_OUT_VOL_LEFT_MASK: c_uint = 0x001f;
pub const SGTL5000_LINE_OUT_VOL_LEFT_SHIFT: c_int = 0;
pub const SGTL5000_LINE_OUT_VOL_LEFT_WIDTH: c_int = 5;
//
// SGTL5000_CHIP_ANA_POWER
//
pub const SGTL5000_ANA_POWER_DEFAULT: c_uint = 0x7060;
pub const SGTL5000_DAC_STEREO: c_uint = 0x4000;
pub const SGTL5000_LINREG_SIMPLE_POWERUP: c_uint = 0x2000;
pub const SGTL5000_STARTUP_POWERUP: c_uint = 0x1000;
pub const SGTL5000_VDDC_CHRGPMP_POWERUP: c_uint = 0x0800;
pub const SGTL5000_PLL_POWERUP: c_uint = 0x0400;
pub const SGTL5000_LINEREG_D_POWERUP: c_uint = 0x0200;
pub const SGTL5000_VCOAMP_POWERUP: c_uint = 0x0100;
pub const SGTL5000_VAG_POWERUP: c_uint = 0x0080;
pub const SGTL5000_ADC_STEREO: c_uint = 0x0040;
pub const SGTL5000_REFTOP_POWERUP: c_uint = 0x0020;
pub const SGTL5000_HP_POWERUP: c_uint = 0x0010;
pub const SGTL5000_DAC_POWERUP: c_uint = 0x0008;
pub const SGTL5000_CAPLESS_HP_POWERUP: c_uint = 0x0004;
pub const SGTL5000_ADC_POWERUP: c_uint = 0x0002;
pub const SGTL5000_LINE_OUT_POWERUP: c_uint = 0x0001;
//
// SGTL5000_CHIP_PLL_CTRL
//
pub const SGTL5000_PLL_INT_DIV_MASK: c_uint = 0xf800;
pub const SGTL5000_PLL_INT_DIV_SHIFT: c_int = 11;
pub const SGTL5000_PLL_INT_DIV_WIDTH: c_int = 5;
pub const SGTL5000_PLL_FRAC_DIV_MASK: c_uint = 0x07ff;
pub const SGTL5000_PLL_FRAC_DIV_SHIFT: c_int = 0;
pub const SGTL5000_PLL_FRAC_DIV_WIDTH: c_int = 11;
//
// SGTL5000_CHIP_CLK_TOP_CTRL
//
pub const SGTL5000_INT_OSC_EN: c_uint = 0x0800;
pub const SGTL5000_INPUT_FREQ_DIV2: c_uint = 0x0008;
//
// SGTL5000_CHIP_ANA_STATUS
//
pub const SGTL5000_HP_LRSHORT: c_uint = 0x0200;
pub const SGTL5000_CAPLESS_SHORT: c_uint = 0x0100;
pub const SGTL5000_PLL_LOCKED: c_uint = 0x0010;
//
// SGTL5000_CHIP_SHORT_CTRL
//
pub const SGTL5000_LVLADJR_MASK: c_uint = 0x7000;
pub const SGTL5000_LVLADJR_SHIFT: c_int = 12;
pub const SGTL5000_LVLADJR_WIDTH: c_int = 3;
pub const SGTL5000_LVLADJL_MASK: c_uint = 0x0700;
pub const SGTL5000_LVLADJL_SHIFT: c_int = 8;
pub const SGTL5000_LVLADJL_WIDTH: c_int = 3;
pub const SGTL5000_LVLADJC_MASK: c_uint = 0x0070;
pub const SGTL5000_LVLADJC_SHIFT: c_int = 4;
pub const SGTL5000_LVLADJC_WIDTH: c_int = 3;
pub const SGTL5000_LR_SHORT_MOD_MASK: c_uint = 0x000c;
pub const SGTL5000_LR_SHORT_MOD_SHIFT: c_int = 2;
pub const SGTL5000_LR_SHORT_MOD_WIDTH: c_int = 2;
pub const SGTL5000_CM_SHORT_MOD_MASK: c_uint = 0x0003;
pub const SGTL5000_CM_SHORT_MOD_SHIFT: c_int = 0;
pub const SGTL5000_CM_SHORT_MOD_WIDTH: c_int = 2;
//
// SGTL5000_CHIP_ANA_TEST2
//
pub const SGTL5000_MONO_DAC: c_uint = 0x1000;
//
// SGTL5000_DAP_CTRL
//
pub const SGTL5000_DAP_MIX_EN: c_uint = 0x0010;
pub const SGTL5000_DAP_EN: c_uint = 0x0001;
pub const SGTL5000_SYSCLK: c_uint = 0x00;
pub const SGTL5000_LRCLK: c_uint = 0x01;
//
// SGTL5000_DAP_AUDIO_EQ
//
pub const SGTL5000_DAP_SEL_PEQ: c_int = 1;
pub const SGTL5000_DAP_SEL_TONE_CTRL: c_int = 2;
pub const SGTL5000_DAP_SEL_GEQ: c_int = 3;
