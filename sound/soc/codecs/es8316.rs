//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/es8316.h
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
// Copyright Everest Semiconductor Co.,Ltd
//
// Author: David Yang <yangxiaohua@everest-semi.com>
//
// ES8316 register space
//
// Reset Control
pub const ES8316_RESET: c_uint = 0x00;
// Clock Management
pub const ES8316_CLKMGR_CLKSW: c_uint = 0x01;
pub const ES8316_CLKMGR_CLKSEL: c_uint = 0x02;
pub const ES8316_CLKMGR_ADCOSR: c_uint = 0x03;
pub const ES8316_CLKMGR_ADCDIV1: c_uint = 0x04;
pub const ES8316_CLKMGR_ADCDIV2: c_uint = 0x05;
pub const ES8316_CLKMGR_DACDIV1: c_uint = 0x06;
pub const ES8316_CLKMGR_DACDIV2: c_uint = 0x07;
pub const ES8316_CLKMGR_CPDIV: c_uint = 0x08;
// Serial Data Port Control
pub const ES8316_SERDATA1: c_uint = 0x09;
pub const ES8316_SERDATA_ADC: c_uint = 0x0a;
pub const ES8316_SERDATA_DAC: c_uint = 0x0b;
// System Control
pub const ES8316_SYS_VMIDSEL: c_uint = 0x0c;
pub const ES8316_SYS_PDN: c_uint = 0x0d;
pub const ES8316_SYS_LP1: c_uint = 0x0e;
pub const ES8316_SYS_LP2: c_uint = 0x0f;
pub const ES8316_SYS_VMIDLOW: c_uint = 0x10;
pub const ES8316_SYS_VSEL: c_uint = 0x11;
pub const ES8316_SYS_REF: c_uint = 0x12;
// Headphone Mixer
pub const ES8316_HPMIX_SEL: c_uint = 0x13;
pub const ES8316_HPMIX_SWITCH: c_uint = 0x14;
pub const ES8316_HPMIX_PDN: c_uint = 0x15;
pub const ES8316_HPMIX_VOL: c_uint = 0x16;
// Charge Pump Headphone driver
pub const ES8316_CPHP_OUTEN: c_uint = 0x17;
pub const ES8316_CPHP_ICAL_VOL: c_uint = 0x18;
pub const ES8316_CPHP_PDN1: c_uint = 0x19;
pub const ES8316_CPHP_PDN2: c_uint = 0x1a;
pub const ES8316_CPHP_LDOCTL: c_uint = 0x1b;
// Calibration
pub const ES8316_CAL_TYPE: c_uint = 0x1c;
pub const ES8316_CAL_SET: c_uint = 0x1d;
pub const ES8316_CAL_HPLIV: c_uint = 0x1e;
pub const ES8316_CAL_HPRIV: c_uint = 0x1f;
pub const ES8316_CAL_HPLMV: c_uint = 0x20;
pub const ES8316_CAL_HPRMV: c_uint = 0x21;
// ADC Control
pub const ES8316_ADC_PDN_LINSEL: c_uint = 0x22;
pub const ES8316_ADC_PGAGAIN: c_uint = 0x23;
pub const ES8316_ADC_D2SEPGA: c_uint = 0x24;
pub const ES8316_ADC_DMIC: c_uint = 0x25;
pub const ES8316_ADC_MUTE: c_uint = 0x26;
pub const ES8316_ADC_VOLUME: c_uint = 0x27;
pub const ES8316_ADC_ALC1: c_uint = 0x29;
pub const ES8316_ADC_ALC2: c_uint = 0x2a;
pub const ES8316_ADC_ALC3: c_uint = 0x2b;
pub const ES8316_ADC_ALC4: c_uint = 0x2c;
pub const ES8316_ADC_ALC5: c_uint = 0x2d;
pub const ES8316_ADC_ALC_NG: c_uint = 0x2e;
// DAC Control
pub const ES8316_DAC_PDN: c_uint = 0x2f;
pub const ES8316_DAC_SET1: c_uint = 0x30;
pub const ES8316_DAC_SET2: c_uint = 0x31;
pub const ES8316_DAC_SET3: c_uint = 0x32;
pub const ES8316_DAC_VOLL: c_uint = 0x33;
pub const ES8316_DAC_VOLR: c_uint = 0x34;
// GPIO
pub const ES8316_GPIO_SEL: c_uint = 0x4d;
pub const ES8316_GPIO_DEBOUNCE: c_uint = 0x4e;
pub const ES8316_GPIO_FLAG: c_uint = 0x4f;
// Test mode
pub const ES8316_TESTMODE: c_uint = 0x50;
pub const ES8316_TEST1: c_uint = 0x51;
pub const ES8316_TEST2: c_uint = 0x52;
pub const ES8316_TEST3: c_uint = 0x53;
//
// Field definitions
//
// ES8316_RESET
pub const ES8316_RESET_CSM_ON: c_uint = 0x80;
// ES8316_CLKMGR_CLKSW
pub const ES8316_CLKMGR_CLKSW_MCLK_ON: c_uint = 0x40;
pub const ES8316_CLKMGR_CLKSW_BCLK_ON: c_uint = 0x20;
// ES8316_SERDATA1
pub const ES8316_SERDATA1_MASTER: c_uint = 0x80;
pub const ES8316_SERDATA1_BCLK_INV: c_uint = 0x20;
// ES8316_SERDATA_ADC and _DAC
pub const ES8316_SERDATA2_FMT_MASK: c_uint = 0x3;
pub const ES8316_SERDATA2_FMT_I2S: c_uint = 0x00;
pub const ES8316_SERDATA2_FMT_LEFTJ: c_uint = 0x01;
pub const ES8316_SERDATA2_FMT_RIGHTJ: c_uint = 0x02;
pub const ES8316_SERDATA2_FMT_PCM: c_uint = 0x03;
pub const ES8316_SERDATA2_ADCLRP: c_uint = 0x20;
pub const ES8316_SERDATA2_LEN_MASK: c_uint = 0x1c;
pub const ES8316_SERDATA2_LEN_24: c_uint = 0x00;
pub const ES8316_SERDATA2_LEN_20: c_uint = 0x04;
pub const ES8316_SERDATA2_LEN_18: c_uint = 0x08;
pub const ES8316_SERDATA2_LEN_16: c_uint = 0x0c;
pub const ES8316_SERDATA2_LEN_32: c_uint = 0x10;
// ES8316_GPIO_DEBOUNCE
pub const ES8316_GPIO_ENABLE_INTERRUPT: c_uint = 0x02;
// ES8316_GPIO_FLAG
pub const ES8316_GPIO_FLAG_GM_NOT_SHORTED: c_uint = 0x02;
pub const ES8316_GPIO_FLAG_HP_NOT_INSERTED: c_uint = 0x04;
// ES8316_CLKMGR_CLKSW
pub const ES8316_CLKMGR_CLKSW_MCLK_DIV: c_uint = 0x80;
