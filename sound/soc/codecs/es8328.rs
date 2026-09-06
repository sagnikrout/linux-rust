//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/es8328.h
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
// es8328.h  --  ES8328 ALSA SoC Audio driver
//

extern "C" {
    pub fn es8328_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
}
pub const ES8328_DACLVOL: c_int = 46;
pub const ES8328_DACRVOL: c_int = 47;
pub const ES8328_DACCTL: c_int = 28;

pub const ES8328_CONTROL1: c_uint = 0x00;

pub const ES8328_CONTROL2: c_uint = 0x01;

pub const ES8328_CHIPPOWER: c_uint = 0x02;
pub const ES8328_CHIPPOWER_DACVREF_OFF: c_int = 0;
pub const ES8328_CHIPPOWER_ADCVREF_OFF: c_int = 1;
pub const ES8328_CHIPPOWER_DACDLL_OFF: c_int = 2;
pub const ES8328_CHIPPOWER_ADCDLL_OFF: c_int = 3;
pub const ES8328_CHIPPOWER_DACSTM_RESET: c_int = 4;
pub const ES8328_CHIPPOWER_ADCSTM_RESET: c_int = 5;
pub const ES8328_CHIPPOWER_DACDIG_OFF: c_int = 6;
pub const ES8328_CHIPPOWER_ADCDIG_OFF: c_int = 7;
pub const ES8328_ADCPOWER: c_uint = 0x03;
pub const ES8328_ADCPOWER_INT1_LOWPOWER: c_int = 0;
pub const ES8328_ADCPOWER_FLASH_ADC_LOWPOWER: c_int = 1;
pub const ES8328_ADCPOWER_ADC_BIAS_GEN_OFF: c_int = 2;
pub const ES8328_ADCPOWER_MIC_BIAS_OFF: c_int = 3;
pub const ES8328_ADCPOWER_ADCR_OFF: c_int = 4;
pub const ES8328_ADCPOWER_ADCL_OFF: c_int = 5;
pub const ES8328_ADCPOWER_AINR_OFF: c_int = 6;
pub const ES8328_ADCPOWER_AINL_OFF: c_int = 7;
pub const ES8328_DACPOWER: c_uint = 0x04;
pub const ES8328_DACPOWER_OUT3_ON: c_int = 0;
pub const ES8328_DACPOWER_MONO_ON: c_int = 1;
pub const ES8328_DACPOWER_ROUT2_ON: c_int = 2;
pub const ES8328_DACPOWER_LOUT2_ON: c_int = 3;
pub const ES8328_DACPOWER_ROUT1_ON: c_int = 4;
pub const ES8328_DACPOWER_LOUT1_ON: c_int = 5;
pub const ES8328_DACPOWER_RDAC_OFF: c_int = 6;
pub const ES8328_DACPOWER_LDAC_OFF: c_int = 7;
pub const ES8328_CHIPLOPOW1: c_uint = 0x05;
pub const ES8328_CHIPLOPOW2: c_uint = 0x06;
pub const ES8328_ANAVOLMANAG: c_uint = 0x07;
pub const ES8328_MASTERMODE: c_uint = 0x08;

pub const ES8328_ADCCONTROL1: c_uint = 0x09;
pub const ES8328_ADCCONTROL2: c_uint = 0x0a;
pub const ES8328_ADCCONTROL3: c_uint = 0x0b;
pub const ES8328_ADCCONTROL4: c_uint = 0x0c;

pub const ES8328_ADCCONTROL4_ADCWL_SHIFT: c_int = 2;

pub const ES8328_ADCCONTROL5: c_uint = 0x0d;

pub const ES8328_ADCCONTROL6: c_uint = 0x0e;
pub const ES8328_ADCCONTROL7: c_uint = 0x0f;

pub const ES8328_ADCCONTROL8: c_uint = 0x10;
pub const ES8328_ADCCONTROL9: c_uint = 0x11;
pub const ES8328_ADCCONTROL10: c_uint = 0x12;
pub const ES8328_ADCCONTROL11: c_uint = 0x13;
pub const ES8328_ADCCONTROL12: c_uint = 0x14;
pub const ES8328_ADCCONTROL13: c_uint = 0x15;
pub const ES8328_ADCCONTROL14: c_uint = 0x16;
pub const ES8328_DACCONTROL1: c_uint = 0x17;

pub const ES8328_DACCONTROL1_DACWL_SHIFT: c_int = 3;

pub const ES8328_DACCONTROL2: c_uint = 0x18;

pub const ES8328_DACCONTROL3: c_uint = 0x19;

pub const ES8328_LDACVOL: c_uint = 0x1a;

pub const ES8328_RDACVOL: c_uint = 0x1b;

pub const ES8328_DACCONTROL4: c_uint = 0x1a;
pub const ES8328_DACCONTROL5: c_uint = 0x1b;
pub const ES8328_DACCONTROL6: c_uint = 0x1c;

pub const ES8328_DACCONTROL7: c_uint = 0x1d;

// Shelving filter
pub const ES8328_DACCONTROL8: c_uint = 0x1e;
pub const ES8328_DACCONTROL9: c_uint = 0x1f;
pub const ES8328_DACCONTROL10: c_uint = 0x20;
pub const ES8328_DACCONTROL11: c_uint = 0x21;
pub const ES8328_DACCONTROL12: c_uint = 0x22;
pub const ES8328_DACCONTROL13: c_uint = 0x23;
pub const ES8328_DACCONTROL14: c_uint = 0x24;
pub const ES8328_DACCONTROL15: c_uint = 0x25;
pub const ES8328_DACCONTROL16: c_uint = 0x26;

pub const ES8328_DACCONTROL17: c_uint = 0x27;

pub const ES8328_DACCONTROL18: c_uint = 0x28;

pub const ES8328_DACCONTROL19: c_uint = 0x29;

pub const ES8328_DACCONTROL20: c_uint = 0x2a;

pub const ES8328_DACCONTROL21: c_uint = 0x2b;

pub const ES8328_DACCONTROL22: c_uint = 0x2c;

pub const ES8328_DACCONTROL23: c_uint = 0x2d;

// LOUT1 Amplifier
pub const ES8328_LOUT1VOL: c_uint = 0x2e;

// ROUT1 Amplifier
pub const ES8328_ROUT1VOL: c_uint = 0x2f;

// LOUT2 Amplifier
pub const ES8328_LOUT2VOL: c_uint = 0x30;

// ROUT2 Amplifier
pub const ES8328_ROUT2VOL: c_uint = 0x31;

// Mono Out Amplifier
pub const ES8328_MONOOUTVOL: c_uint = 0x32;

pub const ES8328_DACCONTROL29: c_uint = 0x33;
pub const ES8328_DACCONTROL30: c_uint = 0x34;
pub const ES8328_SYSCLK: c_int = 0;
pub const ES8328_REG_MAX: c_uint = 0x35;
pub const ES8328_1536FS: c_int = 1536;
pub const ES8328_1024FS: c_int = 1024;
pub const ES8328_768FS: c_int = 768;
pub const ES8328_512FS: c_int = 512;
pub const ES8328_384FS: c_int = 384;
pub const ES8328_256FS: c_int = 256;
pub const ES8328_128FS: c_int = 128;
