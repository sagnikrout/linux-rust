//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max8907.h
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
// Functions to access MAX8907 power management chip.
//
// Copyright (C) 2010 Gyungoh Yoo <jack.yoo@maxim-ic.com>
// Copyright (C) 2012, NVIDIA CORPORATION. All rights reserved.
//

// MAX8907 register map
pub const MAX8907_REG_SYSENSEL: c_uint = 0x00;
pub const MAX8907_REG_ON_OFF_IRQ1: c_uint = 0x01;
pub const MAX8907_REG_ON_OFF_IRQ1_MASK: c_uint = 0x02;
pub const MAX8907_REG_ON_OFF_STAT: c_uint = 0x03;
pub const MAX8907_REG_SDCTL1: c_uint = 0x04;
pub const MAX8907_REG_SDSEQCNT1: c_uint = 0x05;
pub const MAX8907_REG_SDV1: c_uint = 0x06;
pub const MAX8907_REG_SDCTL2: c_uint = 0x07;
pub const MAX8907_REG_SDSEQCNT2: c_uint = 0x08;
pub const MAX8907_REG_SDV2: c_uint = 0x09;
pub const MAX8907_REG_SDCTL3: c_uint = 0x0A;
pub const MAX8907_REG_SDSEQCNT3: c_uint = 0x0B;
pub const MAX8907_REG_SDV3: c_uint = 0x0C;
pub const MAX8907_REG_ON_OFF_IRQ2: c_uint = 0x0D;
pub const MAX8907_REG_ON_OFF_IRQ2_MASK: c_uint = 0x0E;
pub const MAX8907_REG_RESET_CNFG: c_uint = 0x0F;
pub const MAX8907_REG_LDOCTL16: c_uint = 0x10;
pub const MAX8907_REG_LDOSEQCNT16: c_uint = 0x11;
pub const MAX8907_REG_LDO16VOUT: c_uint = 0x12;
pub const MAX8907_REG_SDBYSEQCNT: c_uint = 0x13;
pub const MAX8907_REG_LDOCTL17: c_uint = 0x14;
pub const MAX8907_REG_LDOSEQCNT17: c_uint = 0x15;
pub const MAX8907_REG_LDO17VOUT: c_uint = 0x16;
pub const MAX8907_REG_LDOCTL1: c_uint = 0x18;
pub const MAX8907_REG_LDOSEQCNT1: c_uint = 0x19;
pub const MAX8907_REG_LDO1VOUT: c_uint = 0x1A;
pub const MAX8907_REG_LDOCTL2: c_uint = 0x1C;
pub const MAX8907_REG_LDOSEQCNT2: c_uint = 0x1D;
pub const MAX8907_REG_LDO2VOUT: c_uint = 0x1E;
pub const MAX8907_REG_LDOCTL3: c_uint = 0x20;
pub const MAX8907_REG_LDOSEQCNT3: c_uint = 0x21;
pub const MAX8907_REG_LDO3VOUT: c_uint = 0x22;
pub const MAX8907_REG_LDOCTL4: c_uint = 0x24;
pub const MAX8907_REG_LDOSEQCNT4: c_uint = 0x25;
pub const MAX8907_REG_LDO4VOUT: c_uint = 0x26;
pub const MAX8907_REG_LDOCTL5: c_uint = 0x28;
pub const MAX8907_REG_LDOSEQCNT5: c_uint = 0x29;
pub const MAX8907_REG_LDO5VOUT: c_uint = 0x2A;
pub const MAX8907_REG_LDOCTL6: c_uint = 0x2C;
pub const MAX8907_REG_LDOSEQCNT6: c_uint = 0x2D;
pub const MAX8907_REG_LDO6VOUT: c_uint = 0x2E;
pub const MAX8907_REG_LDOCTL7: c_uint = 0x30;
pub const MAX8907_REG_LDOSEQCNT7: c_uint = 0x31;
pub const MAX8907_REG_LDO7VOUT: c_uint = 0x32;
pub const MAX8907_REG_LDOCTL8: c_uint = 0x34;
pub const MAX8907_REG_LDOSEQCNT8: c_uint = 0x35;
pub const MAX8907_REG_LDO8VOUT: c_uint = 0x36;
pub const MAX8907_REG_LDOCTL9: c_uint = 0x38;
pub const MAX8907_REG_LDOSEQCNT9: c_uint = 0x39;
pub const MAX8907_REG_LDO9VOUT: c_uint = 0x3A;
pub const MAX8907_REG_LDOCTL10: c_uint = 0x3C;
pub const MAX8907_REG_LDOSEQCNT10: c_uint = 0x3D;
pub const MAX8907_REG_LDO10VOUT: c_uint = 0x3E;
pub const MAX8907_REG_LDOCTL11: c_uint = 0x40;
pub const MAX8907_REG_LDOSEQCNT11: c_uint = 0x41;
pub const MAX8907_REG_LDO11VOUT: c_uint = 0x42;
pub const MAX8907_REG_LDOCTL12: c_uint = 0x44;
pub const MAX8907_REG_LDOSEQCNT12: c_uint = 0x45;
pub const MAX8907_REG_LDO12VOUT: c_uint = 0x46;
pub const MAX8907_REG_LDOCTL13: c_uint = 0x48;
pub const MAX8907_REG_LDOSEQCNT13: c_uint = 0x49;
pub const MAX8907_REG_LDO13VOUT: c_uint = 0x4A;
pub const MAX8907_REG_LDOCTL14: c_uint = 0x4C;
pub const MAX8907_REG_LDOSEQCNT14: c_uint = 0x4D;
pub const MAX8907_REG_LDO14VOUT: c_uint = 0x4E;
pub const MAX8907_REG_LDOCTL15: c_uint = 0x50;
pub const MAX8907_REG_LDOSEQCNT15: c_uint = 0x51;
pub const MAX8907_REG_LDO15VOUT: c_uint = 0x52;
pub const MAX8907_REG_OUT5VEN: c_uint = 0x54;
pub const MAX8907_REG_OUT5VSEQ: c_uint = 0x55;
pub const MAX8907_REG_OUT33VEN: c_uint = 0x58;
pub const MAX8907_REG_OUT33VSEQ: c_uint = 0x59;
pub const MAX8907_REG_LDOCTL19: c_uint = 0x5C;
pub const MAX8907_REG_LDOSEQCNT19: c_uint = 0x5D;
pub const MAX8907_REG_LDO19VOUT: c_uint = 0x5E;
pub const MAX8907_REG_LBCNFG: c_uint = 0x60;
pub const MAX8907_REG_SEQ1CNFG: c_uint = 0x64;
pub const MAX8907_REG_SEQ2CNFG: c_uint = 0x65;
pub const MAX8907_REG_SEQ3CNFG: c_uint = 0x66;
pub const MAX8907_REG_SEQ4CNFG: c_uint = 0x67;
pub const MAX8907_REG_SEQ5CNFG: c_uint = 0x68;
pub const MAX8907_REG_SEQ6CNFG: c_uint = 0x69;
pub const MAX8907_REG_SEQ7CNFG: c_uint = 0x6A;
pub const MAX8907_REG_LDOCTL18: c_uint = 0x72;
pub const MAX8907_REG_LDOSEQCNT18: c_uint = 0x73;
pub const MAX8907_REG_LDO18VOUT: c_uint = 0x74;
pub const MAX8907_REG_BBAT_CNFG: c_uint = 0x78;
pub const MAX8907_REG_CHG_CNTL1: c_uint = 0x7C;
pub const MAX8907_REG_CHG_CNTL2: c_uint = 0x7D;
pub const MAX8907_REG_CHG_IRQ1: c_uint = 0x7E;
pub const MAX8907_REG_CHG_IRQ2: c_uint = 0x7F;
pub const MAX8907_REG_CHG_IRQ1_MASK: c_uint = 0x80;
pub const MAX8907_REG_CHG_IRQ2_MASK: c_uint = 0x81;
pub const MAX8907_REG_CHG_STAT: c_uint = 0x82;
pub const MAX8907_REG_WLED_MODE_CNTL: c_uint = 0x84;
pub const MAX8907_REG_ILED_CNTL: c_uint = 0x84;
pub const MAX8907_REG_II1RR: c_uint = 0x8E;
pub const MAX8907_REG_II2RR: c_uint = 0x8F;
pub const MAX8907_REG_LDOCTL20: c_uint = 0x9C;
pub const MAX8907_REG_LDOSEQCNT20: c_uint = 0x9D;
pub const MAX8907_REG_LDO20VOUT: c_uint = 0x9E;
// RTC register map
pub const MAX8907_REG_RTC_SEC: c_uint = 0x00;
pub const MAX8907_REG_RTC_MIN: c_uint = 0x01;
pub const MAX8907_REG_RTC_HOURS: c_uint = 0x02;
pub const MAX8907_REG_RTC_WEEKDAY: c_uint = 0x03;
pub const MAX8907_REG_RTC_DATE: c_uint = 0x04;
pub const MAX8907_REG_RTC_MONTH: c_uint = 0x05;
pub const MAX8907_REG_RTC_YEAR1: c_uint = 0x06;
pub const MAX8907_REG_RTC_YEAR2: c_uint = 0x07;
pub const MAX8907_REG_ALARM0_SEC: c_uint = 0x08;
pub const MAX8907_REG_ALARM0_MIN: c_uint = 0x09;
pub const MAX8907_REG_ALARM0_HOURS: c_uint = 0x0A;
pub const MAX8907_REG_ALARM0_WEEKDAY: c_uint = 0x0B;
pub const MAX8907_REG_ALARM0_DATE: c_uint = 0x0C;
pub const MAX8907_REG_ALARM0_MONTH: c_uint = 0x0D;
pub const MAX8907_REG_ALARM0_YEAR1: c_uint = 0x0E;
pub const MAX8907_REG_ALARM0_YEAR2: c_uint = 0x0F;
pub const MAX8907_REG_ALARM1_SEC: c_uint = 0x10;
pub const MAX8907_REG_ALARM1_MIN: c_uint = 0x11;
pub const MAX8907_REG_ALARM1_HOURS: c_uint = 0x12;
pub const MAX8907_REG_ALARM1_WEEKDAY: c_uint = 0x13;
pub const MAX8907_REG_ALARM1_DATE: c_uint = 0x14;
pub const MAX8907_REG_ALARM1_MONTH: c_uint = 0x15;
pub const MAX8907_REG_ALARM1_YEAR1: c_uint = 0x16;
pub const MAX8907_REG_ALARM1_YEAR2: c_uint = 0x17;
pub const MAX8907_REG_ALARM0_CNTL: c_uint = 0x18;
pub const MAX8907_REG_ALARM1_CNTL: c_uint = 0x19;
pub const MAX8907_REG_RTC_STATUS: c_uint = 0x1A;
pub const MAX8907_REG_RTC_CNTL: c_uint = 0x1B;
pub const MAX8907_REG_RTC_IRQ: c_uint = 0x1C;
pub const MAX8907_REG_RTC_IRQ_MASK: c_uint = 0x1D;
pub const MAX8907_REG_MPL_CNTL: c_uint = 0x1E;
// ADC and Touch Screen Controller register map
pub const MAX8907_CTL: c_int = 0;
pub const MAX8907_SEQCNT: c_int = 1;
pub const MAX8907_VOUT: c_int = 2;
// mask bit fields
pub const MAX8907_MASK_LDO_SEQ: c_uint = 0x1C;
pub const MAX8907_MASK_LDO_EN: c_uint = 0x01;
pub const MAX8907_MASK_VBBATTCV: c_uint = 0x03;
pub const MAX8907_MASK_OUT5V_VINEN: c_uint = 0x10;
pub const MAX8907_MASK_OUT5V_ENSRC: c_uint = 0x0E;
pub const MAX8907_MASK_OUT5V_EN: c_uint = 0x01;
pub const MAX8907_MASK_POWER_OFF: c_uint = 0x40;
// Regulator IDs
pub const MAX8907_MBATT: c_int = 0;
pub const MAX8907_SD1: c_int = 1;
pub const MAX8907_SD2: c_int = 2;
pub const MAX8907_SD3: c_int = 3;
pub const MAX8907_LDO1: c_int = 4;
pub const MAX8907_LDO2: c_int = 5;
pub const MAX8907_LDO3: c_int = 6;
pub const MAX8907_LDO4: c_int = 7;
pub const MAX8907_LDO5: c_int = 8;
pub const MAX8907_LDO6: c_int = 9;
pub const MAX8907_LDO7: c_int = 10;
pub const MAX8907_LDO8: c_int = 11;
pub const MAX8907_LDO9: c_int = 12;
pub const MAX8907_LDO10: c_int = 13;
pub const MAX8907_LDO11: c_int = 14;
pub const MAX8907_LDO12: c_int = 15;
pub const MAX8907_LDO13: c_int = 16;
pub const MAX8907_LDO14: c_int = 17;
pub const MAX8907_LDO15: c_int = 18;
pub const MAX8907_LDO16: c_int = 19;
pub const MAX8907_LDO17: c_int = 20;
pub const MAX8907_LDO18: c_int = 21;
pub const MAX8907_LDO19: c_int = 22;
pub const MAX8907_LDO20: c_int = 23;
pub const MAX8907_OUT5V: c_int = 24;
pub const MAX8907_OUT33V: c_int = 25;
pub const MAX8907_BBAT: c_int = 26;
pub const MAX8907_SDBY: c_int = 27;
pub const MAX8907_VRTC: c_int = 28;

// IRQ definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8907_platform_data {
    pub init_data: [*mut regulator_init_data; MAX8907_NUM_REGULATORS],
    pub pm_off: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8907 {
    pub dev: *mut device,
    pub irq_lock: mutex,
    pub i2c_gen: *mut i2c_client,
    pub i2c_rtc: *mut i2c_client,
    pub regmap_gen: *mut regmap,
    pub regmap_rtc: *mut regmap,
    pub irqc_chg: *mut regmap_irq_chip_data,
    pub irqc_on_off: *mut regmap_irq_chip_data,
    pub irqc_rtc: *mut regmap_irq_chip_data,
}
