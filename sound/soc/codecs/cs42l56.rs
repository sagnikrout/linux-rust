//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs42l56.h
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
// cs42l52.h -- CS42L56 ALSA SoC audio driver
//
// Copyright 2014 CirrusLogic, Inc.
//
// Author: Brian Austin <brian.austin@cirrus.com>
//
pub const CS42L56_CHIP_ID_1: c_uint = 0x01;
pub const CS42L56_CHIP_ID_2: c_uint = 0x02;
pub const CS42L56_PWRCTL_1: c_uint = 0x03;
pub const CS42L56_PWRCTL_2: c_uint = 0x04;
pub const CS42L56_CLKCTL_1: c_uint = 0x05;
pub const CS42L56_CLKCTL_2: c_uint = 0x06;
pub const CS42L56_SERIAL_FMT: c_uint = 0x07;
pub const CS42L56_CLASSH_CTL: c_uint = 0x08;
pub const CS42L56_MISC_CTL: c_uint = 0x09;
pub const CS42L56_INT_STATUS: c_uint = 0x0a;
pub const CS42L56_PLAYBACK_CTL: c_uint = 0x0b;
pub const CS42L56_DSP_MUTE_CTL: c_uint = 0x0c;
pub const CS42L56_ADCA_MIX_VOLUME: c_uint = 0x0d;
pub const CS42L56_ADCB_MIX_VOLUME: c_uint = 0x0e;
pub const CS42L56_PCMA_MIX_VOLUME: c_uint = 0x0f;
pub const CS42L56_PCMB_MIX_VOLUME: c_uint = 0x10;
pub const CS42L56_ANAINPUT_ADV_VOLUME: c_uint = 0x11;
pub const CS42L56_DIGINPUT_ADV_VOLUME: c_uint = 0x12;
pub const CS42L56_MASTER_A_VOLUME: c_uint = 0x13;
pub const CS42L56_MASTER_B_VOLUME: c_uint = 0x14;
pub const CS42L56_BEEP_FREQ_ONTIME: c_uint = 0x15;
pub const CS42L56_BEEP_FREQ_OFFTIME: c_uint = 0x16;
pub const CS42L56_BEEP_TONE_CFG: c_uint = 0x17;
pub const CS42L56_TONE_CTL: c_uint = 0x18;
pub const CS42L56_CHAN_MIX_SWAP: c_uint = 0x19;
pub const CS42L56_AIN_REFCFG_ADC_MUX: c_uint = 0x1a;
pub const CS42L56_HPF_CTL: c_uint = 0x1b;
pub const CS42L56_MISC_ADC_CTL: c_uint = 0x1c;
pub const CS42L56_GAIN_BIAS_CTL: c_uint = 0x1d;
pub const CS42L56_PGAA_MUX_VOLUME: c_uint = 0x1e;
pub const CS42L56_PGAB_MUX_VOLUME: c_uint = 0x1f;
pub const CS42L56_ADCA_ATTENUATOR: c_uint = 0x20;
pub const CS42L56_ADCB_ATTENUATOR: c_uint = 0x21;
pub const CS42L56_ALC_EN_ATTACK_RATE: c_uint = 0x22;
pub const CS42L56_ALC_RELEASE_RATE: c_uint = 0x23;
pub const CS42L56_ALC_THRESHOLD: c_uint = 0x24;
pub const CS42L56_NOISE_GATE_CTL: c_uint = 0x25;
pub const CS42L56_ALC_LIM_SFT_ZC: c_uint = 0x26;
pub const CS42L56_AMUTE_HPLO_MUX: c_uint = 0x27;
pub const CS42L56_HPA_VOLUME: c_uint = 0x28;
pub const CS42L56_HPB_VOLUME: c_uint = 0x29;
pub const CS42L56_LOA_VOLUME: c_uint = 0x2a;
pub const CS42L56_LOB_VOLUME: c_uint = 0x2b;
pub const CS42L56_LIM_THRESHOLD_CTL: c_uint = 0x2c;
pub const CS42L56_LIM_CTL_RELEASE_RATE: c_uint = 0x2d;
pub const CS42L56_LIM_ATTACK_RATE: c_uint = 0x2e;
// Device ID and Rev ID Masks
pub const CS42L56_DEVID: c_uint = 0x56;
pub const CS42L56_CHIP_ID_MASK: c_uint = 0xff;
pub const CS42L56_AREV_MASK: c_uint = 0x1c;
pub const CS42L56_MTLREV_MASK: c_uint = 0x03;
// Power bit masks
pub const CS42L56_PDN_ALL_MASK: c_uint = 0x01;
pub const CS42L56_PDN_ADCA_MASK: c_uint = 0x02;
pub const CS42L56_PDN_ADCB_MASK: c_uint = 0x04;
pub const CS42L56_PDN_CHRG_MASK: c_uint = 0x08;
pub const CS42L56_PDN_BIAS_MASK: c_uint = 0x10;
pub const CS42L56_PDN_VBUF_MASK: c_uint = 0x20;
pub const CS42L56_PDN_LOA_MASK: c_uint = 0x03;
pub const CS42L56_PDN_LOB_MASK: c_uint = 0x0c;
pub const CS42L56_PDN_HPA_MASK: c_uint = 0x30;
pub const CS42L56_PDN_HPB_MASK: c_uint = 0xc0;
// serial port and clk masks
pub const CS42L56_MASTER_MODE: c_uint = 0x40;
pub const CS42L56_SLAVE_MODE: c_int = 0;
pub const CS42L56_MS_MODE_MASK: c_uint = 0x40;
pub const CS42L56_SCLK_INV: c_uint = 0x20;
pub const CS42L56_SCLK_INV_MASK: c_uint = 0x20;
pub const CS42L56_SCLK_MCLK_MASK: c_uint = 0x18;
pub const CS42L56_MCLK_PREDIV: c_uint = 0x04;
pub const CS42L56_MCLK_PREDIV_MASK: c_uint = 0x04;
pub const CS42L56_MCLK_DIV2: c_uint = 0x02;
pub const CS42L56_MCLK_DIV2_MASK: c_uint = 0x02;
pub const CS42L56_MCLK_DIS_MASK: c_uint = 0x01;
pub const CS42L56_CLK_AUTO_MASK: c_uint = 0x20;
pub const CS42L56_CLK_RATIO_MASK: c_uint = 0x1f;
pub const CS42L56_DIG_FMT_I2S: c_int = 0;
pub const CS42L56_DIG_FMT_LEFT_J: c_uint = 0x08;
pub const CS42L56_DIG_FMT_MASK: c_uint = 0x08;
// Class H and misc ctl masks
pub const CS42L56_ADAPT_PWR_MASK: c_uint = 0xc0;
pub const CS42L56_CHRG_FREQ_MASK: c_uint = 0x0f;
pub const CS42L56_DIG_MUX_MASK: c_uint = 0x80;
pub const CS42L56_ANLGSFT_MASK: c_uint = 0x10;
pub const CS42L56_ANLGZC_MASK: c_uint = 0x08;
pub const CS42L56_DIGSFT_MASK: c_uint = 0x04;
pub const CS42L56_FREEZE_MASK: c_uint = 0x01;
pub const CS42L56_MIC_BIAS_MASK: c_uint = 0x03;
pub const CS42L56_HPFA_FREQ_MASK: c_uint = 0x03;
pub const CS42L56_HPFB_FREQ_MASK: c_uint = 0xc0;
pub const CS42L56_AIN1A_REF_MASK: c_uint = 0x10;
pub const CS42L56_AIN2A_REF_MASK: c_uint = 0x40;
pub const CS42L56_AIN1B_REF_MASK: c_uint = 0x20;
pub const CS42L56_AIN2B_REF_MASK: c_uint = 0x80;
// Playback Capture ctl masks
pub const CS42L56_PDN_DSP_MASK: c_uint = 0x80;
pub const CS42L56_DEEMPH_MASK: c_uint = 0x40;
pub const CS42L56_PLYBCK_GANG_MASK: c_uint = 0x10;
pub const CS42L56_PCM_INV_MASK: c_uint = 0x0c;
pub const CS42L56_MUTE_ALL: c_uint = 0xff;
pub const CS42L56_UNMUTE: c_int = 0;
pub const CS42L56_ADCAMIX_MUTE_MASK: c_uint = 0x40;
pub const CS42L56_ADCBMIX_MUTE_MASK: c_uint = 0x80;
pub const CS42L56_PCMAMIX_MUTE_MASK: c_uint = 0x10;
pub const CS42L56_PCMBMIX_MUTE_MASK: c_uint = 0x20;
pub const CS42L56_MSTB_MUTE_MASK: c_uint = 0x02;
pub const CS42L56_MSTA_MUTE_MASK: c_uint = 0x01;
pub const CS42L56_ADCA_MUTE_MASK: c_uint = 0x01;
pub const CS42L56_ADCB_MUTE_MASK: c_uint = 0x02;
pub const CS42L56_HP_MUTE_MASK: c_uint = 0x80;
pub const CS42L56_LO_MUTE_MASK: c_uint = 0x80;
// Beep masks
pub const CS42L56_BEEP_FREQ_MASK: c_uint = 0xf0;
pub const CS42L56_BEEP_ONTIME_MASK: c_uint = 0x0f;
pub const CS42L56_BEEP_OFFTIME_MASK: c_uint = 0xe0;
pub const CS42L56_BEEP_CFG_MASK: c_uint = 0xc0;
pub const CS42L56_BEEP_TREBCF_MASK: c_uint = 0x18;
pub const CS42L56_BEEP_BASSCF_MASK: c_uint = 0x06;
pub const CS42L56_BEEP_TCEN_MASK: c_uint = 0x01;
pub const CS42L56_BEEP_RATE_SHIFT: c_int = 4;
pub const CS42L56_BEEP_EN_MASK: c_uint = 0x3f;
// Supported MCLKS
pub const CS42L56_MCLK_5P6448MHZ: c_int = 5644800;
pub const CS42L56_MCLK_6MHZ: c_int = 6000000;
pub const CS42L56_MCLK_6P144MHZ: c_int = 6144000;
pub const CS42L56_MCLK_11P2896MHZ: c_int = 11289600;
pub const CS42L56_MCLK_12MHZ: c_int = 12000000;
pub const CS42L56_MCLK_12P288MHZ: c_int = 12288000;
pub const CS42L56_MCLK_22P5792MHZ: c_int = 22579200;
pub const CS42L56_MCLK_24MHZ: c_int = 24000000;
pub const CS42L56_MCLK_24P576MHZ: c_int = 24576000;
// Clock ratios
pub const CS42L56_MCLK_LRCLK_128: c_uint = 0x08;
pub const CS42L56_MCLK_LRCLK_125: c_uint = 0x09;
pub const CS42L56_MCLK_LRCLK_136: c_uint = 0x0b;
pub const CS42L56_MCLK_LRCLK_192: c_uint = 0x0c;
pub const CS42L56_MCLK_LRCLK_187P5: c_uint = 0x0d;
pub const CS42L56_MCLK_LRCLK_256: c_uint = 0x10;
pub const CS42L56_MCLK_LRCLK_250: c_uint = 0x11;
pub const CS42L56_MCLK_LRCLK_272: c_uint = 0x13;
pub const CS42L56_MCLK_LRCLK_384: c_uint = 0x14;
pub const CS42L56_MCLK_LRCLK_375: c_uint = 0x15;
pub const CS42L56_MCLK_LRCLK_512: c_uint = 0x18;
pub const CS42L56_MCLK_LRCLK_500: c_uint = 0x19;
pub const CS42L56_MCLK_LRCLK_544: c_uint = 0x1b;
pub const CS42L56_MCLK_LRCLK_750: c_uint = 0x1c;
pub const CS42L56_MCLK_LRCLK_768: c_uint = 0x1d;
pub const CS42L56_MAX_REGISTER: c_uint = 0x34;
