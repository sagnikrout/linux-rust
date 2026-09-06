//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/es9356.h
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
// ES9356 Implementation-define
pub const ES9356_FLAGS_HP: c_uint = 0x2003;
pub const ES9356_CSM_RESET: c_uint = 0x2020;
pub const ES9356_FUC_RESET: c_uint = 0x2021;
pub const ES9356_STATE: c_uint = 0x2022;
pub const ES9356_VMID_TIME: c_uint = 0x2023;
pub const ES9356_STATE_TIME: c_uint = 0x2024;
pub const ES9356_HP_SPK_TIME: c_uint = 0x2025;
pub const ES9356_WP_ENABLE: c_uint = 0x2026;
pub const ES9356_DMIC_GPIO: c_uint = 0x2027;
pub const ES9356_ENDPOINT_MODE: c_uint = 0x2028;
// HP DETECT
pub const ES9356_HP_TYPE: c_uint = 0x2029;
pub const ES9356_HP_DETECTTIME: c_uint = 0x202A;
pub const ES9356_MICBIAS_SEL: c_uint = 0x202B;
pub const ES9356_KEY_PRESS_TIME: c_uint = 0x202C;
pub const ES9356_KEY_RELEASE_TIME: c_uint = 0x202D;
pub const ES9356_KEY_HOLD_TIME: c_uint = 0x202E;
pub const ES9356_BTSEL_REF: c_uint = 0x202F;
pub const ES9356_BUTTON_CHARGE: c_uint = 0x2030;
pub const ES9356_KEYD_DETECT: c_uint = 0x2031;
pub const ES9356_DPEN_TIME: c_uint = 0x2032;
pub const ES9356_TIMER_CHECK: c_uint = 0x2033;
pub const ES9356_IBIASGEN: c_uint = 0x2041;
pub const ES9356_VMID1SEL: c_uint = 0x2042;
pub const ES9356_VMID1STL: c_uint = 0x2043;
pub const ES9356_VMID2SEL: c_uint = 0x2044;
pub const ES9356_VMID2STL: c_uint = 0x2045;
pub const ES9356_VSEL: c_uint = 0x2046;
pub const ES9356_MICBIAS_CTL: c_uint = 0x2047;
pub const ES9356_HPDETECT_CTL: c_uint = 0x2048;
pub const ES9356_MICBIAS_RES: c_uint = 0x2049;
// CLK
pub const ES9356_CLK_SEL: c_uint = 0x2050;
pub const ES9356_CLK_CTL: c_uint = 0x2051;
pub const ES9356_DETCLK_CTL: c_uint = 0x2052;
pub const ES9356_CPCLK_CTL: c_uint = 0x2053;
pub const ES9356_SPKCLK_CTL: c_uint = 0x2054;
pub const ES9356_PRE_DIV_CTL: c_uint = 0x2055;
pub const ES9356_DLL_MODE: c_uint = 0x2056;
pub const ES9356_ANACLK_SEL: c_uint = 0x2057;
pub const ES9356_OSRCLK_SEL: c_uint = 0x2058;
pub const ES9356_DSPCLK_SEL: c_uint = 0x2059;
pub const ES9356_SPK9M_MODE: c_uint = 0x205a;
// ADC DIG CTL
pub const ES9356_DMIC_POL: c_uint = 0x2061;
pub const ES9356_ADC_SWAP: c_uint = 0x2062;
pub const ES9356_ADC_OSR: c_uint = 0x2063;
pub const ES9356_ADC_OSRGAIN: c_uint = 0x2064;
pub const ES9356_ADC_CLEARRAM: c_uint = 0x2065;
pub const ES9356_ADC_RAMP: c_uint = 0x2066;
pub const ES9356_ADC_HPF1: c_uint = 0x2067;
pub const ES9356_ADC_HPF2: c_uint = 0x2068;
pub const ES9356_ADC_ALC: c_uint = 0x206C;
pub const ES9356_ALC_LEVEL: c_uint = 0x206D;
pub const ES9356_ALC_RAMP_WINSIZE: c_uint = 0x206E;
// ADC ANA CTL
pub const ES9356_ADC_REF_EN: c_uint = 0x2080;
pub const ES9356_ADC_AMIC_CTL: c_uint = 0x2081;
pub const ES9356_ADC_ANA: c_uint = 0x2082;
pub const ES9356_PGA_CTL: c_uint = 0x2083;
pub const ES9356_ADC_INT: c_uint = 0x2084;
pub const ES9356_ADC_VCM: c_uint = 0x2085;
pub const ES9356_ADC_VRPBIAS: c_uint = 0x2086;
pub const ES9356_ADC_LP: c_uint = 0x2087;
// DAC DIG CTL
pub const ES9356_DAC_FSMODE: c_uint = 0x2090;
pub const ES9356_DAC_OSR: c_uint = 0x2091;
pub const ES9356_DAC_INV: c_uint = 0x2092;
pub const ES9356_DAC_RAMP: c_uint = 0x2093;
pub const ES9356_DAC_VPPSCALE: c_uint = 0x2094;
pub const ES9356_DAC_SWAP: c_uint = 0x2097;
pub const ES9356_SPKCMP_VPPSC: c_uint = 0x20A0;
pub const ES9356_CALIBRATION_TIME: c_uint = 0x20A1;
pub const ES9356_CALIBRATION_SETTING: c_uint = 0x20A2;
pub const ES9356_DAC_OFFSET_LH: c_uint = 0x20A3;
pub const ES9356_DAC_OFFSET_LL: c_uint = 0x20A4;
pub const ES9356_DAC_OFFSET_RH: c_uint = 0x20A5;
pub const ES9356_DAC_OFFSET_RL: c_uint = 0x20A6;
// DAC ANA CTL
pub const ES9356_DAC_REF_EN: c_uint = 0x20B0;
pub const ES9356_DAC_ENABLE: c_uint = 0x20B1;
pub const ES9356_DAC_VROI: c_uint = 0x20B2;
pub const ES9356_DAC_LP: c_uint = 0x20B3;
// HP CTL
pub const ES9356_CHARGEPUMP_CTL: c_uint = 0x20C0;
pub const ES9356_CPLDO_CTL: c_uint = 0x20C1;
pub const ES9356_HP_REF_CTL: c_uint = 0x20C2;
pub const ES9356_HP_IBIAS: c_uint = 0x20C3;
pub const ES9356_HP_EN: c_uint = 0x20C4;
pub const ES9356_HP_VOLUME: c_uint = 0x20C5;
pub const ES9356_HP_LP: c_uint = 0x20C6;
// SPK CTL
pub const ES9356_SPKLDO_CTL: c_uint = 0x20D0;
pub const ES9356_CLASSD_CTL: c_uint = 0x20D1;
pub const ES9356_SPK_HBDG: c_uint = 0x20D5;
pub const ES9356_SPK_VOLUME: c_uint = 0x20D7;
pub const ES9356_SPK_SCP: c_uint = 0x20D8;
pub const ES9356_SPK_DT: c_uint = 0x20D9;
pub const ES9356_SPK_OTP: c_uint = 0x20DA;
pub const ES9356_SPKBIAS_COMP: c_uint = 0x20DB;
// ES9356 SDCA Control - function number
pub const FUNC_NUM_UAJ: c_uint = 0x01;
pub const FUNC_NUM_MIC: c_uint = 0x02;
pub const FUNC_NUM_AMP: c_uint = 0x03;
pub const FUNC_NUM_HID: c_uint = 0x04;
// ES9356 SDCA entity
pub const ES9356_SDCA_ENT0: c_uint = 0x00;
pub const ES9356_SDCA_ENT_PDE11: c_uint = 0x03;
pub const ES9356_SDCA_ENT_FU11: c_uint = 0x04;
pub const ES9356_SDCA_ENT_XU12: c_uint = 0x05;
pub const ES9356_SDCA_ENT_FU113: c_uint = 0x07;
pub const ES9356_SDCA_ENT_CS113: c_uint = 0x09;
pub const ES9356_SDCA_ENT_PPU11: c_uint = 0x0C;
pub const ES9356_SDCA_ENT_CS21: c_uint = 0x02;
pub const ES9356_SDCA_ENT_PPU21: c_uint = 0x03;

pub const ES9356_SDCA_ENT_XU22: c_uint = 0x06;
pub const ES9356_SDCA_ENT_SAPU29: c_uint = 0x03;
pub const ES9356_SDCA_ENT_PDE23: c_uint = 0x0B;
pub const ES9356_SDCA_ENT_HID01: c_uint = 0x01;
pub const ES9356_SDCA_ENT_CS41: c_uint = 0x02;
pub const ES9356_SDCA_ENT_FU35: c_uint = 0x04;
pub const ES9356_SDCA_ENT_XU42: c_uint = 0x06;
pub const ES9356_SDCA_ENT_FU41: c_uint = 0x07;
pub const ES9356_SDCA_ENT_PDE47: c_uint = 0x0E;
pub const ES9356_SDCA_ENT_IT33: c_uint = 0x0F;
pub const ES9356_SDCA_ENT_PDE34: c_uint = 0x10;
pub const ES9356_SDCA_ENT_FU33: c_uint = 0x11;
pub const ES9356_SDCA_ENT_XU36: c_uint = 0x13;
pub const ES9356_SDCA_ENT_FU36: c_uint = 0x15;
pub const ES9356_SDCA_ENT_CS36: c_uint = 0x17;
pub const ES9356_SDCA_ENT_GE35: c_uint = 0x18;
// ES9356 SDCA control
pub const ES9356_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint = 0x10;
pub const ES9356_SDCA_CTL_FU_MUTE: c_uint = 0x01;
pub const ES9356_SDCA_CTL_FU_VOLUME: c_uint = 0x02;
pub const ES9356_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint = 0x10;
pub const ES9356_SDCA_CTL_SELECTED_MODE: c_uint = 0x01;
pub const ES9356_SDCA_CTL_DETECTED_MODE: c_uint = 0x02;
pub const ES9356_SDCA_CTL_REQ_POWER_STATE: c_uint = 0x01;
pub const ES9356_SDCA_CTL_FU_CH_GAIN: c_uint = 0x0b;
pub const ES9356_SDCA_CTL_FUNC_STATUS: c_uint = 0x10;
pub const ES9356_SDCA_CTL_ACTUAL_POWER_STATE: c_uint = 0x10;
pub const ES9356_SDCA_CTL_POSTURE_NUMBER: c_uint = 0x00;
// ES9356 SDCA channel
pub const CH_L: c_uint = 0x01;
pub const CH_R: c_uint = 0x02;
pub const MBQ: c_uint = 0x2000;
// ES9356 HID
pub const ES9356_BUF_ADDR_HID: c_uint = 0x44000000;
pub const ES9356_HID_BYTE2: c_uint = 0x44000001;
pub const ES9356_HID_BYTE3: c_uint = 0x44000002;
pub const ES9356_HID_BYTE4: c_uint = 0x44000003;
// ES9356 Volume Setting
pub const ES9356_VU_BASE: c_int = 768;
pub const ES9356_OFFSET_HIGH: c_uint = 0x07F8;
pub const ES9356_OFFSET_LOW: c_uint = 0x0007;
pub const ES9356_DEFAULT_VOLUME: c_uint = 0x00;
pub const ES9356_VOLUME_STEP: c_int = 32;

pub const ES9356_VOLUME_MAX: c_int = 285;
pub const ES9356_AMIC_GAIN_STEP: c_int = 768;
pub const ES9356_DMIC_GAIN_STEP: c_int = 1536;
pub const ES9356_GAIN_MIN: c_int = 0;
pub const ES9356_AMIC_GAIN_MAX: c_int = 10;
pub const ES9356_DMIC_GAIN_MAX: c_int = 3;
