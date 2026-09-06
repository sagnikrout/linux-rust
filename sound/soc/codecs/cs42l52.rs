//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs42l52.h
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
// cs42l52.h -- CS42L52 ALSA SoC audio driver
//
// Copyright 2012 CirrusLogic, Inc.
//
// Author: Georgi Vlaev <joe@nucleusys.com>
// Author: Brian Austin <brian.austin@cirrus.com>
//

pub const CS42L52_DEFAULT_CLK: c_int = 12000000;
pub const CS42L52_MIN_CLK: c_int = 11000000;
pub const CS42L52_MAX_CLK: c_int = 27000000;

pub const CS42L52_DEFAULT_MAX_CHANS: c_int = 2;
pub const CS42L52_SYSCLK: c_int = 1;

pub const CS42L52_CHIP_ONE: c_uint = 0x00;
pub const CS42L52_CHIP_TWO: c_uint = 0x01;
pub const CS42L52_CHIP_THR: c_uint = 0x02;
pub const CS42L52_CHIP_MASK: c_uint = 0x0f;
pub const CS42L52_FIX_BITS_CTL: c_uint = 0x00;
pub const CS42L52_CHIP: c_uint = 0x01;
pub const CS42L52_CHIP_ID: c_uint = 0xE0;
pub const CS42L52_CHIP_ID_MASK: c_uint = 0xF8;
pub const CS42L52_CHIP_REV_A0: c_uint = 0x00;
pub const CS42L52_CHIP_REV_A1: c_uint = 0x01;
pub const CS42L52_CHIP_REV_B0: c_uint = 0x02;
pub const CS42L52_CHIP_REV_MASK: c_uint = 0x07;
pub const CS42L52_PWRCTL1: c_uint = 0x02;
pub const CS42L52_PWRCTL1_PDN_ALL: c_uint = 0x9F;
pub const CS42L52_PWRCTL1_PDN_CHRG: c_uint = 0x80;
pub const CS42L52_PWRCTL1_PDN_PGAB: c_uint = 0x10;
pub const CS42L52_PWRCTL1_PDN_PGAA: c_uint = 0x08;
pub const CS42L52_PWRCTL1_PDN_ADCB: c_uint = 0x04;
pub const CS42L52_PWRCTL1_PDN_ADCA: c_uint = 0x02;
pub const CS42L52_PWRCTL1_PDN_CODEC: c_uint = 0x01;
pub const CS42L52_PWRCTL2: c_uint = 0x03;

pub const CS42L52_PWRCTL2_PDN_MICB_SHIFT: c_int = 2;

pub const CS42L52_PWRCTL2_PDN_MICA_SHIFT: c_int = 1;

pub const CS42L52_PWRCTL2_PDN_MICBIAS_SHIFT: c_int = 0;
pub const CS42L52_PWRCTL3: c_uint = 0x04;
pub const CS42L52_PWRCTL3_HPB_PDN_SHIFT: c_int = 6;
pub const CS42L52_PWRCTL3_HPB_ON_LOW: c_uint = 0x00;
pub const CS42L52_PWRCTL3_HPB_ON_HIGH: c_uint = 0x01;
pub const CS42L52_PWRCTL3_HPB_ALWAYS_ON: c_uint = 0x02;
pub const CS42L52_PWRCTL3_HPB_ALWAYS_OFF: c_uint = 0x03;
pub const CS42L52_PWRCTL3_HPA_PDN_SHIFT: c_int = 4;
pub const CS42L52_PWRCTL3_HPA_ON_LOW: c_uint = 0x00;
pub const CS42L52_PWRCTL3_HPA_ON_HIGH: c_uint = 0x01;
pub const CS42L52_PWRCTL3_HPA_ALWAYS_ON: c_uint = 0x02;
pub const CS42L52_PWRCTL3_HPA_ALWAYS_OFF: c_uint = 0x03;
pub const CS42L52_PWRCTL3_SPKB_PDN_SHIFT: c_int = 2;
pub const CS42L52_PWRCTL3_SPKB_ON_LOW: c_uint = 0x00;
pub const CS42L52_PWRCTL3_SPKB_ON_HIGH: c_uint = 0x01;
pub const CS42L52_PWRCTL3_SPKB_ALWAYS_ON: c_uint = 0x02;

pub const CS42L52_PWRCTL3_SPKA_PDN_SHIFT: c_int = 0;
pub const CS42L52_PWRCTL3_SPKA_ON_LOW: c_uint = 0x00;
pub const CS42L52_PWRCTL3_SPKA_ON_HIGH: c_uint = 0x01;
pub const CS42L52_PWRCTL3_SPKA_ALWAYS_ON: c_uint = 0x02;
pub const CS42L52_DEFAULT_OUTPUT_STATE: c_uint = 0x05;
pub const CS42L52_PWRCTL3_CONF_MASK: c_uint = 0x03;
pub const CS42L52_CLK_CTL: c_uint = 0x05;

pub const CLK_SPEED_SHIFT: c_int = 5;
pub const CLK_DS_MODE: c_uint = 0x00;
pub const CLK_SS_MODE: c_uint = 0x01;
pub const CLK_HS_MODE: c_uint = 0x02;
pub const CLK_QS_MODE: c_uint = 0x03;
pub const CLK_32K_SR_SHIFT: c_int = 4;
pub const CLK_32K: c_uint = 0x01;
pub const CLK_NO_32K: c_uint = 0x00;
pub const CLK_27M_MCLK_SHIFT: c_int = 3;
pub const CLK_27M_MCLK: c_uint = 0x01;
pub const CLK_NO_27M: c_uint = 0x00;
pub const CLK_RATIO_SHIFT: c_int = 1;
pub const CLK_R_128: c_uint = 0x00;
pub const CLK_R_125: c_uint = 0x01;
pub const CLK_R_132: c_uint = 0x02;
pub const CLK_R_136: c_uint = 0x03;
pub const CS42L52_IFACE_CTL1: c_uint = 0x06;

pub const CS42L52_IFACE_CTL1_WL_MASK: c_uint = 0xFFFF;
pub const CS42L52_IFACE_CTL2: c_uint = 0x07;

pub const CS42L52_IFACE_CTL2_BIAS_LVL: c_uint = 0x07;
pub const CS42L52_ADC_PGA_A: c_uint = 0x08;
pub const CS42L52_ADC_PGA_B: c_uint = 0x09;
pub const CS42L52_ADC_SEL_SHIFT: c_int = 5;
pub const CS42L52_ADC_SEL_AIN1: c_uint = 0x00;
pub const CS42L52_ADC_SEL_AIN2: c_uint = 0x01;
pub const CS42L52_ADC_SEL_AIN3: c_uint = 0x02;
pub const CS42L52_ADC_SEL_AIN4: c_uint = 0x03;
pub const CS42L52_ADC_SEL_PGA: c_uint = 0x04;
pub const CS42L52_ANALOG_HPF_CTL: c_uint = 0x0A;

pub const CS42L52_ADC_HPF_FREQ: c_uint = 0x0B;
pub const CS42L52_ADC_MISC_CTL: c_uint = 0x0C;

pub const CS42L52_PB_CTL1: c_uint = 0x0D;
pub const CS42L52_PB_CTL1_HP_GAIN_SHIFT: c_int = 5;
pub const CS42L52_PB_CTL1_HP_GAIN_03959: c_uint = 0x00;
pub const CS42L52_PB_CTL1_HP_GAIN_04571: c_uint = 0x01;
pub const CS42L52_PB_CTL1_HP_GAIN_05111: c_uint = 0x02;
pub const CS42L52_PB_CTL1_HP_GAIN_06047: c_uint = 0x03;
pub const CS42L52_PB_CTL1_HP_GAIN_07099: c_uint = 0x04;
pub const CS42L52_PB_CTL1_HP_GAIN_08399: c_uint = 0x05;
pub const CS42L52_PB_CTL1_HP_GAIN_10000: c_uint = 0x06;
pub const CS42L52_PB_CTL1_HP_GAIN_11430: c_uint = 0x07;

pub const CS42L52_PB_CTL1_MUTE_MASK: c_uint = 0x03;
pub const CS42L52_PB_CTL1_MUTE: c_int = 3;
pub const CS42L52_PB_CTL1_UNMUTE: c_int = 0;
pub const CS42L52_MISC_CTL: c_uint = 0x0E;

pub const CS42L52_PB_CTL2: c_uint = 0x0F;

pub const CS42L52_MICA_CTL: c_uint = 0x10;
pub const CS42L52_MICB_CTL: c_uint = 0x11;
pub const CS42L52_MIC_CTL_MIC_SEL_MASK: c_uint = 0xBF;
pub const CS42L52_MIC_CTL_MIC_SEL_SHIFT: c_int = 6;
pub const CS42L52_MIC_CTL_TYPE_MASK: c_uint = 0x20;
pub const CS42L52_MIC_CTL_TYPE_SHIFT: c_int = 5;
pub const CS42L52_PGAA_CTL: c_uint = 0x12;
pub const CS42L52_PGAB_CTL: c_uint = 0x13;
pub const CS42L52_PGAX_CTL_VOL_12DB: c_int = 24;

pub const CS42L52_PASSTHRUA_VOL: c_uint = 0x14;
pub const CS42L52_PASSTHRUB_VOL: c_uint = 0x15;
pub const CS42L52_ADCA_VOL: c_uint = 0x16;
pub const CS42L52_ADCB_VOL: c_uint = 0x17;

pub const CS42L52_ADCX_VOL_12DB: c_int = 12;
pub const CS42L52_ADCX_VOL_6DB: c_int = 6;
pub const CS42L52_ADCA_MIXER_VOL: c_uint = 0x18;
pub const CS42L52_ADCB_MIXER_VOL: c_uint = 0x19;
pub const CS42L52_ADC_MIXER_VOL_12DB: c_uint = 0x18;
pub const CS42L52_PCMA_MIXER_VOL: c_uint = 0x1A;
pub const CS42L52_PCMB_MIXER_VOL: c_uint = 0x1B;
pub const CS42L52_BEEP_FREQ: c_uint = 0x1C;
pub const CS42L52_BEEP_VOL: c_uint = 0x1D;
pub const CS42L52_BEEP_TONE_CTL: c_uint = 0x1E;
pub const CS42L52_BEEP_RATE_SHIFT: c_int = 4;
pub const CS42L52_BEEP_RATE_MASK: c_uint = 0x0F;
pub const CS42L52_TONE_CTL: c_uint = 0x1F;
pub const CS42L52_BEEP_EN_MASK: c_uint = 0x3F;
pub const CS42L52_MASTERA_VOL: c_uint = 0x20;
pub const CS42L52_MASTERB_VOL: c_uint = 0x21;
pub const CS42L52_HPA_VOL: c_uint = 0x22;
pub const CS42L52_HPB_VOL: c_uint = 0x23;
pub const CS42L52_DEFAULT_HP_VOL: c_uint = 0xF0;
pub const CS42L52_SPKA_VOL: c_uint = 0x24;
pub const CS42L52_SPKB_VOL: c_uint = 0x25;
pub const CS42L52_DEFAULT_SPK_VOL: c_uint = 0xF0;
pub const CS42L52_ADC_PCM_MIXER: c_uint = 0x26;
pub const CS42L52_LIMITER_CTL1: c_uint = 0x27;
pub const CS42L52_LIMITER_CTL2: c_uint = 0x28;
pub const CS42L52_LIMITER_AT_RATE: c_uint = 0x29;
pub const CS42L52_ALC_CTL: c_uint = 0x2A;
pub const CS42L52_ALC_CTL_ALCB_ENABLE_SHIFT: c_int = 7;
pub const CS42L52_ALC_CTL_ALCA_ENABLE_SHIFT: c_int = 6;
pub const CS42L52_ALC_CTL_FASTEST_ATTACK: c_int = 0;
pub const CS42L52_ALC_RATE: c_uint = 0x2B;
pub const CS42L52_ALC_SLOWEST_RELEASE: c_uint = 0x3F;
pub const CS42L52_ALC_THRESHOLD: c_uint = 0x2C;
pub const CS42L52_ALC_MAX_RATE_SHIFT: c_int = 5;
pub const CS42L52_ALC_MIN_RATE_SHIFT: c_int = 2;
pub const CS42L52_ALC_RATE_0DB: c_int = 0;
pub const CS42L52_ALC_RATE_3DB: c_int = 1;
pub const CS42L52_ALC_RATE_6DB: c_int = 2;
pub const CS42L52_NOISE_GATE_CTL: c_uint = 0x2D;
pub const CS42L52_NG_ENABLE_SHIFT: c_int = 6;
pub const CS42L52_NG_THRESHOLD_SHIFT: c_int = 2;
pub const CS42L52_NG_MIN_70DB: c_int = 2;
pub const CS42L52_NG_DELAY_SHIFT: c_int = 0;
pub const CS42L52_NG_DELAY_100MS: c_int = 1;
pub const CS42L52_CLK_STATUS: c_uint = 0x2E;
pub const CS42L52_BATT_COMPEN: c_uint = 0x2F;
pub const CS42L52_BATT_LEVEL: c_uint = 0x30;
pub const CS42L52_SPK_STATUS: c_uint = 0x31;
pub const CS42L52_SPK_STATUS_PIN_SHIFT: c_int = 3;
pub const CS42L52_SPK_STATUS_PIN_HIGH: c_int = 1;
pub const CS42L52_TEM_CTL: c_uint = 0x32;
pub const CS42L52_TEM_CTL_SET: c_uint = 0x80;
pub const CS42L52_THE_FOLDBACK: c_uint = 0x33;
pub const CS42L52_CHARGE_PUMP: c_uint = 0x34;
pub const CS42L52_CHARGE_PUMP_MASK: c_uint = 0xF0;
pub const CS42L52_CHARGE_PUMP_SHIFT: c_int = 4;
pub const CS42L52_FIX_BITS1: c_uint = 0x3E;
pub const CS42L52_FIX_BITS2: c_uint = 0x47;
pub const CS42L52_MAX_REGISTER: c_uint = 0x47;
