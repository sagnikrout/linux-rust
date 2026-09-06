//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/es8323.h
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
// Copyright Openedhand Ltd.
//
// Author: Richard Purdie <richard@openedhand.com>
// Binbin Zhou <zhoubinbin@loongson.cn>
//
// ES8323 register space
// Chip Control and Power Management
pub const ES8323_CONTROL1: c_uint = 0x00;
pub const ES8323_CONTROL2: c_uint = 0x01;
pub const ES8323_CHIPPOWER: c_uint = 0x02;
pub const ES8323_CHIPPOWER_DACVREF_OFF: c_int = 0;
pub const ES8323_CHIPPOWER_ADCVREF_OFF: c_int = 1;
pub const ES8323_CHIPPOWER_DACDLL_OFF: c_int = 2;
pub const ES8323_CHIPPOWER_ADCDLL_OFF: c_int = 3;
pub const ES8323_CHIPPOWER_DACSTM_RESET: c_int = 4;
pub const ES8323_CHIPPOWER_ADCSTM_RESET: c_int = 5;
pub const ES8323_CHIPPOWER_DACDIG_OFF: c_int = 6;
pub const ES8323_CHIPPOWER_ADCDIG_OFF: c_int = 7;
pub const ES8323_ADCPOWER: c_uint = 0x03;

pub const ES8323_ADCPOWER_PDNADCBIS_OFF: c_int = 2;
pub const ES8323_ADCPOWER_PDNMICB_OFF: c_int = 3;
pub const ES8323_ADCPOWER_PDNADCR_OFF: c_int = 4;
pub const ES8323_ADCPOWER_PDNADCL_OFF: c_int = 5;
pub const ES8323_ADCPOWER_PDNAINR_OFF: c_int = 6;
pub const ES8323_ADCPOWER_PDNAINL_OFF: c_int = 7;
pub const ES8323_DACPOWER: c_uint = 0x04;
pub const ES8323_DACPOWER_ROUT2_OFF: c_int = 2;
pub const ES8323_DACPOWER_LOUT2_OFF: c_int = 3;
pub const ES8323_DACPOWER_ROUT1_OFF: c_int = 4;
pub const ES8323_DACPOWER_LOUT1_OFF: c_int = 5;
pub const ES8323_DACPOWER_PDNDACR_OFF: c_int = 6;
pub const ES8323_DACPOWER_PDNDACL_OFF: c_int = 7;
pub const ES8323_CHIPLOPOW1: c_uint = 0x05;
pub const ES8323_CHIPLOPOW2: c_uint = 0x06;
pub const ES8323_ANAVOLMANAG: c_uint = 0x07;
pub const ES8323_MASTERMODE: c_uint = 0x08;

// ADC Control
pub const ES8323_ADCCONTROL1: c_uint = 0x09;
pub const ES8323_ADCCONTROL1_MICAMPR_OFF: c_int = 0;
pub const ES8323_ADCCONTROL1_MICAMPL_OFF: c_int = 4;
pub const ES8323_ADCCONTROL2: c_uint = 0x0a;
pub const ES8323_ADCCONTROL3: c_uint = 0x0b;
pub const ES8323_ADCCONTROL4: c_uint = 0x0c;

pub const ES8323_FMT_I2S: c_uint = 0x0;
pub const ES8323_FMT_LEFT_J: c_uint = 0x1;
pub const ES8323_FMT_RIGHT_J: c_uint = 0x2;
pub const ES8323_FMT_DSP: c_uint = 0x3;

pub const ES8323_S24_LE: c_uint = 0x0;
pub const ES8323_S20_LE: c_uint = 0x1;
pub const ES8323_S18_LE: c_uint = 0x2;
pub const ES8323_S16_LE: c_uint = 0x3;
pub const ES8323_S32_LE: c_uint = 0x4;

pub const ES8323_ADCCONTROL5: c_uint = 0x0d;

pub const ES8323_ADCCONTROL6: c_uint = 0x0e;
pub const ES8323_ADCCONTROL7: c_uint = 0x0f;
pub const ES8323_ADCCONTROL7_ADCMUTE_OFF: c_int = 2;
pub const ES8323_LADC_VOL: c_uint = 0x10;
pub const ES8323_RADC_VOL: c_uint = 0x11;
pub const ES8323_ADCCONTROL10: c_uint = 0x12;
pub const ES8323_ADCCONTROL11: c_uint = 0x13;
pub const ES8323_ADCCONTROL12: c_uint = 0x14;
pub const ES8323_ADCCONTROL12_ALCATK_OFF: c_int = 0;
pub const ES8323_ADCCONTROL12_ALCDCY_OFF: c_int = 4;
pub const ES8323_ADCCONTROL13: c_uint = 0x15;
pub const ES8323_ADCCONTROL13_TIMEOUT_OFF: c_int = 5;
pub const ES8323_ADCCONTROL13_ALCZC_OFF: c_int = 6;
pub const ES8323_ADCCONTROL14: c_uint = 0x16;
pub const ES8323_ADCCONTROL14_NGAT_OFF: c_int = 0;
pub const ES8323_ADCCONTROL14_NGG_OFF: c_int = 1;
pub const ES8323_ADCCONTROL14_NGTH_OFF: c_int = 3;
// DAC Control
pub const ES8323_DACCONTROL1: c_uint = 0x17;

pub const ES8323_DACCONTROL2: c_uint = 0x18;

pub const ES8323_DACCONTROL3: c_uint = 0x19;

pub const ES8323_LDAC_VOL: c_uint = 0x1a;
pub const ES8323_RDAC_VOL: c_uint = 0x1b;
pub const ES8323_DACCONTROL6: c_uint = 0x1c;
pub const ES8323_DACCONTROL7: c_uint = 0x1d;
pub const ES8323_DACCONTROL8: c_uint = 0x1e;
pub const ES8323_DACCONTROL9: c_uint = 0x1f;
pub const ES8323_DACCONTROL10: c_uint = 0x20;
pub const ES8323_DACCONTROL11: c_uint = 0x21;
pub const ES8323_DACCONTROL12: c_uint = 0x22;
pub const ES8323_DACCONTROL13: c_uint = 0x23;
pub const ES8323_DACCONTROL14: c_uint = 0x24;
pub const ES8323_DACCONTROL15: c_uint = 0x25;
pub const ES8323_DACCONTROL16: c_uint = 0x26;
pub const ES8323_DACCONTROL17: c_uint = 0x27;
pub const ES8323_DACCONTROL17_LI2LOVOL_OFF: c_int = 3;
pub const ES8323_DACCONTROL18: c_uint = 0x28;
pub const ES8323_DACCONTROL19: c_uint = 0x29;
pub const ES8323_DACCONTROL20: c_uint = 0x2a;
pub const ES8323_DACCONTROL20_RI2ROVOL_OFF: c_int = 3;
pub const ES8323_DACCONTROL21: c_uint = 0x2b;
pub const ES8323_DACCONTROL22: c_uint = 0x2c;
pub const ES8323_DACCONTROL23: c_uint = 0x2d;
pub const ES8323_LOUT1_VOL: c_uint = 0x2e;
pub const ES8323_ROUT1_VOL: c_uint = 0x2f;
pub const ES8323_LOUT2_VOL: c_uint = 0x30;
pub const ES8323_ROUT2_VOL: c_uint = 0x31;
pub const ES8323_DACCONTROL28: c_uint = 0x32;
pub const ES8323_DACCONTROL29: c_uint = 0x33;
pub const ES8323_DACCONTROL30: c_uint = 0x34;
