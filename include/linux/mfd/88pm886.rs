//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/88pm886.h
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

pub const PM886_A1_CHIP_ID: c_uint = 0xa1;
pub const PM886_IRQ_ONKEY: c_int = 0;
pub const PM886_PAGE_OFFSET_REGULATORS: c_int = 1;
pub const PM886_PAGE_OFFSET_GPADC: c_int = 2;
pub const PM886_PAGE_OFFSET_BATTERY: c_int = 3;
pub const PM886_REG_ID: c_uint = 0x00;
pub const PM886_REG_STATUS1: c_uint = 0x01;

pub const PM886_REG_INT_STATUS1: c_uint = 0x05;
pub const PM886_REG_INT_ENA_1: c_uint = 0x0a;

pub const PM886_REG_MISC_CONFIG1: c_uint = 0x14;

pub const PM886_REG_MISC_CONFIG2: c_uint = 0x15;

pub const PM886_INT_RC: c_uint = 0x00;

pub const PM886_REG_RTC_CNT1: c_uint = 0xd1;
pub const PM886_REG_RTC_CNT2: c_uint = 0xd2;
pub const PM886_REG_RTC_CNT3: c_uint = 0xd3;
pub const PM886_REG_RTC_CNT4: c_uint = 0xd4;
pub const PM886_REG_RTC_SPARE1: c_uint = 0xea;
pub const PM886_REG_RTC_SPARE2: c_uint = 0xeb;
pub const PM886_REG_RTC_SPARE3: c_uint = 0xec;
pub const PM886_REG_RTC_SPARE4: c_uint = 0xed;
pub const PM886_REG_RTC_SPARE5: c_uint = 0xee;
pub const PM886_REG_RTC_SPARE6: c_uint = 0xef;
pub const PM886_REG_BUCK_EN: c_uint = 0x08;
pub const PM886_REG_LDO_EN1: c_uint = 0x09;
pub const PM886_REG_LDO_EN2: c_uint = 0x0a;
pub const PM886_REG_LDO1_VOUT: c_uint = 0x20;
pub const PM886_REG_LDO2_VOUT: c_uint = 0x26;
pub const PM886_REG_LDO3_VOUT: c_uint = 0x2c;
pub const PM886_REG_LDO4_VOUT: c_uint = 0x32;
pub const PM886_REG_LDO5_VOUT: c_uint = 0x38;
pub const PM886_REG_LDO6_VOUT: c_uint = 0x3e;
pub const PM886_REG_LDO7_VOUT: c_uint = 0x44;
pub const PM886_REG_LDO8_VOUT: c_uint = 0x4a;
pub const PM886_REG_LDO9_VOUT: c_uint = 0x50;
pub const PM886_REG_LDO10_VOUT: c_uint = 0x56;
pub const PM886_REG_LDO11_VOUT: c_uint = 0x5c;
pub const PM886_REG_LDO12_VOUT: c_uint = 0x62;
pub const PM886_REG_LDO13_VOUT: c_uint = 0x68;
pub const PM886_REG_LDO14_VOUT: c_uint = 0x6e;
pub const PM886_REG_LDO15_VOUT: c_uint = 0x74;
pub const PM886_REG_LDO16_VOUT: c_uint = 0x7a;
pub const PM886_REG_BUCK1_VOUT: c_uint = 0xa5;
pub const PM886_REG_BUCK2_VOUT: c_uint = 0xb3;
pub const PM886_REG_BUCK3_VOUT: c_uint = 0xc1;
pub const PM886_REG_BUCK4_VOUT: c_uint = 0xcf;
pub const PM886_REG_BUCK5_VOUT: c_uint = 0xdd;
pub const PM886_LDO_VSEL_MASK: c_uint = 0x0f;
pub const PM886_BUCK_VSEL_MASK: c_uint = 0x7f;
// GPADC enable/disable registers

// No CONFIG3_EN_ALL because this is the only bit there.

// GPADC channel registers
pub const PM886_REG_GPADC_VSC: c_uint = 0x40;
pub const PM886_REG_GPADC_VCHG_PWR: c_uint = 0x4c;
pub const PM886_REG_GPADC_VCF_OUT: c_uint = 0x4e;
pub const PM886_REG_GPADC_TINT: c_uint = 0x50;
pub const PM886_REG_GPADC_GPADC0: c_uint = 0x54;
pub const PM886_REG_GPADC_GPADC1: c_uint = 0x56;
pub const PM886_REG_GPADC_GPADC2: c_uint = 0x58;
pub const PM886_REG_GPADC_VBAT: c_uint = 0xa0;
pub const PM886_REG_GPADC_GND_DET1: c_uint = 0xa4;
pub const PM886_REG_GPADC_GND_DET2: c_uint = 0xa6;
pub const PM886_REG_GPADC_VBUS: c_uint = 0xa8;
pub const PM886_REG_GPADC_GPADC3: c_uint = 0xaa;
pub const PM886_REG_GPADC_MIC_DET: c_uint = 0xac;
pub const PM886_REG_GPADC_VBAT_SLP: c_uint = 0xb0;
// VBAT_SLP is the last register and is 2 bytes wide like other channels.

pub const PM886_GPADC_BIAS_LEVELS: c_int = 16;

// Battery block register definitions
pub const PM886_REG_CLS_CONFIG1: c_uint = 0x71;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm886_chip {
    pub client: *mut i2c_client,
    pub chip_id: c_uint,
    pub regmap: *mut regmap,
    pub regmap_battery: *mut regmap,
}
