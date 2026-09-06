//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/twl4030-audio.h
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
// MFD driver for twl4030 audio submodule
//
// Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
//
// Copyright:   (C) 2009 Nokia Corporation
//
// Codec registers
pub const TWL4030_REG_CODEC_MODE: c_uint = 0x01;
pub const TWL4030_REG_OPTION: c_uint = 0x02;
pub const TWL4030_REG_UNKNOWN: c_uint = 0x03;
pub const TWL4030_REG_MICBIAS_CTL: c_uint = 0x04;
pub const TWL4030_REG_ANAMICL: c_uint = 0x05;
pub const TWL4030_REG_ANAMICR: c_uint = 0x06;
pub const TWL4030_REG_AVADC_CTL: c_uint = 0x07;
pub const TWL4030_REG_ADCMICSEL: c_uint = 0x08;
pub const TWL4030_REG_DIGMIXING: c_uint = 0x09;
pub const TWL4030_REG_ATXL1PGA: c_uint = 0x0A;
pub const TWL4030_REG_ATXR1PGA: c_uint = 0x0B;
pub const TWL4030_REG_AVTXL2PGA: c_uint = 0x0C;
pub const TWL4030_REG_AVTXR2PGA: c_uint = 0x0D;
pub const TWL4030_REG_AUDIO_IF: c_uint = 0x0E;
pub const TWL4030_REG_VOICE_IF: c_uint = 0x0F;
pub const TWL4030_REG_ARXR1PGA: c_uint = 0x10;
pub const TWL4030_REG_ARXL1PGA: c_uint = 0x11;
pub const TWL4030_REG_ARXR2PGA: c_uint = 0x12;
pub const TWL4030_REG_ARXL2PGA: c_uint = 0x13;
pub const TWL4030_REG_VRXPGA: c_uint = 0x14;
pub const TWL4030_REG_VSTPGA: c_uint = 0x15;
pub const TWL4030_REG_VRX2ARXPGA: c_uint = 0x16;
pub const TWL4030_REG_AVDAC_CTL: c_uint = 0x17;
pub const TWL4030_REG_ARX2VTXPGA: c_uint = 0x18;
pub const TWL4030_REG_ARXL1_APGA_CTL: c_uint = 0x19;
pub const TWL4030_REG_ARXR1_APGA_CTL: c_uint = 0x1A;
pub const TWL4030_REG_ARXL2_APGA_CTL: c_uint = 0x1B;
pub const TWL4030_REG_ARXR2_APGA_CTL: c_uint = 0x1C;
pub const TWL4030_REG_ATX2ARXPGA: c_uint = 0x1D;
pub const TWL4030_REG_BT_IF: c_uint = 0x1E;
pub const TWL4030_REG_BTPGA: c_uint = 0x1F;
pub const TWL4030_REG_BTSTPGA: c_uint = 0x20;
pub const TWL4030_REG_EAR_CTL: c_uint = 0x21;
pub const TWL4030_REG_HS_SEL: c_uint = 0x22;
pub const TWL4030_REG_HS_GAIN_SET: c_uint = 0x23;
pub const TWL4030_REG_HS_POPN_SET: c_uint = 0x24;
pub const TWL4030_REG_PREDL_CTL: c_uint = 0x25;
pub const TWL4030_REG_PREDR_CTL: c_uint = 0x26;
pub const TWL4030_REG_PRECKL_CTL: c_uint = 0x27;
pub const TWL4030_REG_PRECKR_CTL: c_uint = 0x28;
pub const TWL4030_REG_HFL_CTL: c_uint = 0x29;
pub const TWL4030_REG_HFR_CTL: c_uint = 0x2A;
pub const TWL4030_REG_ALC_CTL: c_uint = 0x2B;
pub const TWL4030_REG_ALC_SET1: c_uint = 0x2C;
pub const TWL4030_REG_ALC_SET2: c_uint = 0x2D;
pub const TWL4030_REG_BOOST_CTL: c_uint = 0x2E;
pub const TWL4030_REG_SOFTVOL_CTL: c_uint = 0x2F;
pub const TWL4030_REG_DTMF_FREQSEL: c_uint = 0x30;
pub const TWL4030_REG_DTMF_TONEXT1H: c_uint = 0x31;
pub const TWL4030_REG_DTMF_TONEXT1L: c_uint = 0x32;
pub const TWL4030_REG_DTMF_TONEXT2H: c_uint = 0x33;
pub const TWL4030_REG_DTMF_TONEXT2L: c_uint = 0x34;
pub const TWL4030_REG_DTMF_TONOFF: c_uint = 0x35;
pub const TWL4030_REG_DTMF_WANONOFF: c_uint = 0x36;
pub const TWL4030_REG_I2S_RX_SCRAMBLE_H: c_uint = 0x37;
pub const TWL4030_REG_I2S_RX_SCRAMBLE_M: c_uint = 0x38;
pub const TWL4030_REG_I2S_RX_SCRAMBLE_L: c_uint = 0x39;
pub const TWL4030_REG_APLL_CTL: c_uint = 0x3A;
pub const TWL4030_REG_DTMF_CTL: c_uint = 0x3B;
pub const TWL4030_REG_DTMF_PGA_CTL2: c_uint = 0x3C;
pub const TWL4030_REG_DTMF_PGA_CTL1: c_uint = 0x3D;
pub const TWL4030_REG_MISC_SET_1: c_uint = 0x3E;
pub const TWL4030_REG_PCMBTMUX: c_uint = 0x3F;
pub const TWL4030_REG_RX_PATH_SEL: c_uint = 0x43;
pub const TWL4030_REG_VDL_APGA_CTL: c_uint = 0x44;
pub const TWL4030_REG_VIBRA_CTL: c_uint = 0x45;
pub const TWL4030_REG_VIBRA_SET: c_uint = 0x46;
pub const TWL4030_REG_VIBRA_PWM_SET: c_uint = 0x47;
pub const TWL4030_REG_ANAMIC_GAIN: c_uint = 0x48;
pub const TWL4030_REG_MISC_SET_2: c_uint = 0x49;
// Bitfield Definitions
// TWL4030_CODEC_MODE (0x01) Fields
pub const TWL4030_APLL_RATE: c_uint = 0xF0;
pub const TWL4030_APLL_RATE_8000: c_uint = 0x00;
pub const TWL4030_APLL_RATE_11025: c_uint = 0x10;
pub const TWL4030_APLL_RATE_12000: c_uint = 0x20;
pub const TWL4030_APLL_RATE_16000: c_uint = 0x40;
pub const TWL4030_APLL_RATE_22050: c_uint = 0x50;
pub const TWL4030_APLL_RATE_24000: c_uint = 0x60;
pub const TWL4030_APLL_RATE_32000: c_uint = 0x80;
pub const TWL4030_APLL_RATE_44100: c_uint = 0x90;
pub const TWL4030_APLL_RATE_48000: c_uint = 0xA0;
pub const TWL4030_APLL_RATE_96000: c_uint = 0xE0;
pub const TWL4030_SEL_16K: c_uint = 0x08;
pub const TWL4030_CODECPDZ: c_uint = 0x02;
pub const TWL4030_OPT_MODE: c_uint = 0x01;

// TWL4030_OPTION (0x02) Fields

// TWL4030_REG_MICBIAS_CTL (0x04) Fields
pub const TWL4030_MICBIAS2_CTL: c_uint = 0x40;
pub const TWL4030_MICBIAS1_CTL: c_uint = 0x20;
pub const TWL4030_HSMICBIAS_EN: c_uint = 0x04;
pub const TWL4030_MICBIAS2_EN: c_uint = 0x02;
pub const TWL4030_MICBIAS1_EN: c_uint = 0x01;
// ANAMICL (0x05) Fields
pub const TWL4030_CNCL_OFFSET_START: c_uint = 0x80;
pub const TWL4030_OFFSET_CNCL_SEL: c_uint = 0x60;
pub const TWL4030_OFFSET_CNCL_SEL_ARX1: c_uint = 0x00;
pub const TWL4030_OFFSET_CNCL_SEL_ARX2: c_uint = 0x20;
pub const TWL4030_OFFSET_CNCL_SEL_VRX: c_uint = 0x40;
pub const TWL4030_OFFSET_CNCL_SEL_ALL: c_uint = 0x60;
pub const TWL4030_MICAMPL_EN: c_uint = 0x10;
pub const TWL4030_CKMIC_EN: c_uint = 0x08;
pub const TWL4030_AUXL_EN: c_uint = 0x04;
pub const TWL4030_HSMIC_EN: c_uint = 0x02;
pub const TWL4030_MAINMIC_EN: c_uint = 0x01;
// ANAMICR (0x06) Fields
pub const TWL4030_MICAMPR_EN: c_uint = 0x10;
pub const TWL4030_AUXR_EN: c_uint = 0x04;
pub const TWL4030_SUBMIC_EN: c_uint = 0x01;
// AVADC_CTL (0x07) Fields
pub const TWL4030_ADCL_EN: c_uint = 0x08;
pub const TWL4030_AVADC_CLK_PRIORITY: c_uint = 0x04;
pub const TWL4030_ADCR_EN: c_uint = 0x02;
// TWL4030_REG_ADCMICSEL (0x08) Fields
pub const TWL4030_DIGMIC1_EN: c_uint = 0x08;
pub const TWL4030_TX2IN_SEL: c_uint = 0x04;
pub const TWL4030_DIGMIC0_EN: c_uint = 0x02;
pub const TWL4030_TX1IN_SEL: c_uint = 0x01;
// AUDIO_IF (0x0E) Fields
pub const TWL4030_AIF_SLAVE_EN: c_uint = 0x80;
pub const TWL4030_DATA_WIDTH: c_uint = 0x60;
pub const TWL4030_DATA_WIDTH_16S_16W: c_uint = 0x00;
pub const TWL4030_DATA_WIDTH_32S_16W: c_uint = 0x40;
pub const TWL4030_DATA_WIDTH_32S_24W: c_uint = 0x60;
pub const TWL4030_AIF_FORMAT: c_uint = 0x18;
pub const TWL4030_AIF_FORMAT_CODEC: c_uint = 0x00;
pub const TWL4030_AIF_FORMAT_LEFT: c_uint = 0x08;
pub const TWL4030_AIF_FORMAT_RIGHT: c_uint = 0x10;
pub const TWL4030_AIF_FORMAT_TDM: c_uint = 0x18;
pub const TWL4030_AIF_TRI_EN: c_uint = 0x04;
pub const TWL4030_CLK256FS_EN: c_uint = 0x02;
pub const TWL4030_AIF_EN: c_uint = 0x01;
// VOICE_IF (0x0F) Fields
pub const TWL4030_VIF_SLAVE_EN: c_uint = 0x80;
pub const TWL4030_VIF_DIN_EN: c_uint = 0x40;
pub const TWL4030_VIF_DOUT_EN: c_uint = 0x20;
pub const TWL4030_VIF_SWAP: c_uint = 0x10;
pub const TWL4030_VIF_FORMAT: c_uint = 0x08;
pub const TWL4030_VIF_TRI_EN: c_uint = 0x04;
pub const TWL4030_VIF_SUB_EN: c_uint = 0x02;
pub const TWL4030_VIF_EN: c_uint = 0x01;
// EAR_CTL (0x21)
pub const TWL4030_EAR_GAIN: c_uint = 0x30;
// HS_GAIN_SET (0x23) Fields
pub const TWL4030_HSR_GAIN: c_uint = 0x0C;
pub const TWL4030_HSR_GAIN_PWR_DOWN: c_uint = 0x00;
pub const TWL4030_HSR_GAIN_PLUS_6DB: c_uint = 0x04;
pub const TWL4030_HSR_GAIN_0DB: c_uint = 0x08;
pub const TWL4030_HSR_GAIN_MINUS_6DB: c_uint = 0x0C;
pub const TWL4030_HSL_GAIN: c_uint = 0x03;
pub const TWL4030_HSL_GAIN_PWR_DOWN: c_uint = 0x00;
pub const TWL4030_HSL_GAIN_PLUS_6DB: c_uint = 0x01;
pub const TWL4030_HSL_GAIN_0DB: c_uint = 0x02;
pub const TWL4030_HSL_GAIN_MINUS_6DB: c_uint = 0x03;
// HS_POPN_SET (0x24) Fields
pub const TWL4030_VMID_EN: c_uint = 0x40;
pub const TWL4030_EXTMUTE: c_uint = 0x20;
pub const TWL4030_RAMP_DELAY: c_uint = 0x1C;
pub const TWL4030_RAMP_DELAY_20MS: c_uint = 0x00;
pub const TWL4030_RAMP_DELAY_40MS: c_uint = 0x04;
pub const TWL4030_RAMP_DELAY_81MS: c_uint = 0x08;
pub const TWL4030_RAMP_DELAY_161MS: c_uint = 0x0C;
pub const TWL4030_RAMP_DELAY_323MS: c_uint = 0x10;
pub const TWL4030_RAMP_DELAY_645MS: c_uint = 0x14;
pub const TWL4030_RAMP_DELAY_1291MS: c_uint = 0x18;
pub const TWL4030_RAMP_DELAY_2581MS: c_uint = 0x1C;
pub const TWL4030_RAMP_EN: c_uint = 0x02;
// PREDL_CTL (0x25)
pub const TWL4030_PREDL_GAIN: c_uint = 0x30;
// PREDR_CTL (0x26)
pub const TWL4030_PREDR_GAIN: c_uint = 0x30;
// PRECKL_CTL (0x27)
pub const TWL4030_PRECKL_GAIN: c_uint = 0x30;
// PRECKR_CTL (0x28)
pub const TWL4030_PRECKR_GAIN: c_uint = 0x30;
// HFL_CTL (0x29, 0x2A) Fields
pub const TWL4030_HF_CTL_HB_EN: c_uint = 0x04;
pub const TWL4030_HF_CTL_LOOP_EN: c_uint = 0x08;
pub const TWL4030_HF_CTL_RAMP_EN: c_uint = 0x10;
pub const TWL4030_HF_CTL_REF_EN: c_uint = 0x20;
// APLL_CTL (0x3A) Fields
pub const TWL4030_APLL_EN: c_uint = 0x10;
pub const TWL4030_APLL_INFREQ: c_uint = 0x0F;
pub const TWL4030_APLL_INFREQ_19200KHZ: c_uint = 0x05;
pub const TWL4030_APLL_INFREQ_26000KHZ: c_uint = 0x06;
pub const TWL4030_APLL_INFREQ_38400KHZ: c_uint = 0x0F;
// REG_MISC_SET_1 (0x3E) Fields
pub const TWL4030_CLK64_EN: c_uint = 0x80;
pub const TWL4030_SCRAMBLE_EN: c_uint = 0x40;
pub const TWL4030_FMLOOP_EN: c_uint = 0x20;
pub const TWL4030_SMOOTH_ANAVOL_EN: c_uint = 0x02;
pub const TWL4030_DIGMIC_LR_SWAP_EN: c_uint = 0x01;
// VIBRA_CTL (0x45)
pub const TWL4030_VIBRA_EN: c_uint = 0x01;
pub const TWL4030_VIBRA_DIR: c_uint = 0x02;

pub const TWL4030_VIBRA_SEL: c_uint = 0x10;
pub const TWL4030_VIBRA_DIR_SEL: c_uint = 0x20;
// TWL4030 codec resource IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum twl4030_audio_res {
    TWL4030_AUDIO_RES_POWER = 0,
    TWL4030_AUDIO_RES_APLL,
    TWL4030_AUDIO_RES_MAX,
}

extern "C" {
    pub fn twl4030_audio_disable_resource(id: twl4030_audio_res) -> c_int;
}
extern "C" {
    pub fn twl4030_audio_enable_resource(id: twl4030_audio_res) -> c_int;
}
extern "C" {
    pub fn twl4030_audio_get_mclk() -> c_uint;
}
