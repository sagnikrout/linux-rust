//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tlv320aic32x4.h
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
// tlv320aic32x4.h
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aic32x4_type {
    AIC32X4_TYPE_AIC32X4 = 0,
    AIC32X4_TYPE_AIC32X6,
    AIC32X4_TYPE_TAS2505,
}

extern "C" {
    pub fn aic32x4_remove(dev: *mut device);
}
extern "C" {
    pub fn aic32x4_register_clocks(dev: *mut device, mclk_name: *const c_char) -> c_int;
}
// tlv320aic32x4 register space (in decimal to match datasheet)

// Bits, masks, and shifts
// AIC32X4_CLKMUX

// AIC32X4_PLLPR

// AIC32X4_NDAC

// AIC32X4_MDAC

// AIC32X4_NADC

// AIC32X4_MADC

// AIC32X4_BCLKN

// AIC32X4_IFACE1

// AIC32X4_IFACE2

// AIC32X4_IFACE3

// AIC32X4_DACSETUP

// AIC32X4_DACMUTE
pub const AIC32X4_MUTEON: c_uint = 0x0C;
// AIC32X4_ADCSETUP

// AIC32X4_PWRCFG

// AIC32X4_LDOCTL

// AIC32X4_CMMODE

// AIC32X4_MICBIAS

pub const AIC32X4_MICBIAS_2075V: c_uint = 0x60;

// AIC32X4_LMICPGANIN
pub const AIC32X4_LMICPGANIN_IN2R_10K: c_uint = 0x10;
pub const AIC32X4_LMICPGANIN_CM1L_10K: c_uint = 0x40;
// AIC32X4_RMICPGANIN
pub const AIC32X4_RMICPGANIN_IN1L_10K: c_uint = 0x10;
pub const AIC32X4_RMICPGANIN_CM1R_10K: c_uint = 0x40;
// AIC32X4_REFPOWERUP
pub const AIC32X4_REFPOWERUP_SLOW: c_uint = 0x04;
pub const AIC32X4_REFPOWERUP_40MS: c_uint = 0x05;
pub const AIC32X4_REFPOWERUP_80MS: c_uint = 0x06;
pub const AIC32X4_REFPOWERUP_120MS: c_uint = 0x07;
// Common mask and enable for all of the dividers

pub const AIC32X4_DIV_MAX: c_int = 128;
// Clock Limits
pub const AIC32X4_MAX_DOSR_FREQ: c_int = 6200000;
pub const AIC32X4_MIN_DOSR_FREQ: c_int = 2800000;
pub const AIC32X4_MAX_CODEC_CLKIN_FREQ: c_int = 110000000;
pub const AIC32X4_MAX_PLL_CLKIN: c_int = 20000000;
pub const AIC32X4_PWR_MICBIAS_2075_LDOIN: c_uint = 0x00000001;
pub const AIC32X4_PWR_AVDD_DVDD_WEAK_DISABLE: c_uint = 0x00000002;
pub const AIC32X4_PWR_AIC32X4_LDO_ENABLE: c_uint = 0x00000004;
pub const AIC32X4_PWR_CMMODE_LDOIN_RANGE_18_36: c_uint = 0x00000008;
pub const AIC32X4_PWR_CMMODE_HP_LDOIN_POWERED: c_uint = 0x00000010;
pub const AIC32X4_MICPGA_ROUTE_LMIC_IN2R_10K: c_uint = 0x00000001;
pub const AIC32X4_MICPGA_ROUTE_RMIC_IN1L_10K: c_uint = 0x00000002;
// GPIO API
pub const AIC32X4_MFPX_DEFAULT_VALUE: c_uint = 0xff;
pub const AIC32X4_MFP1_DIN_DISABLED: c_int = 0;
pub const AIC32X4_MFP1_DIN_ENABLED: c_uint = 0x2;
pub const AIC32X4_MFP1_GPIO_IN: c_uint = 0x4;
pub const AIC32X4_MFP2_GPIO_OUT_LOW: c_uint = 0x0;
pub const AIC32X4_MFP2_GPIO_OUT_HIGH: c_uint = 0x1;
pub const AIC32X4_MFP_GPIO_ENABLED: c_uint = 0x4;
pub const AIC32X4_MFP5_GPIO_DISABLED: c_uint = 0x0;
pub const AIC32X4_MFP5_GPIO_INPUT: c_uint = 0x8;
pub const AIC32X4_MFP5_GPIO_OUTPUT: c_uint = 0xc;
pub const AIC32X4_MFP5_GPIO_OUT_LOW: c_uint = 0x0;
pub const AIC32X4_MFP5_GPIO_OUT_HIGH: c_uint = 0x1;
