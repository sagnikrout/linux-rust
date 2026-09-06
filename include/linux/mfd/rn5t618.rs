//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rn5t618.h
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
// MFD core driver for Ricoh RN5T618 PMIC
//
// Copyright (C) 2014 Beniamino Galvani <b.galvani@gmail.com>
//

pub const RN5T618_LSIVER: c_uint = 0x00;
pub const RN5T618_OTPVER: c_uint = 0x01;
pub const RN5T618_IODAC: c_uint = 0x02;
pub const RN5T618_VINDAC: c_uint = 0x03;
pub const RN5T618_OUT32KEN: c_uint = 0x05;
pub const RN5T618_CPUCNT: c_uint = 0x06;
pub const RN5T618_PSWR: c_uint = 0x07;
pub const RN5T618_PONHIS: c_uint = 0x09;
pub const RN5T618_POFFHIS: c_uint = 0x0a;
pub const RN5T618_WATCHDOG: c_uint = 0x0b;
pub const RN5T618_WATCHDOGCNT: c_uint = 0x0c;
pub const RN5T618_PWRFUNC: c_uint = 0x0d;
pub const RN5T618_SLPCNT: c_uint = 0x0e;
pub const RN5T618_REPCNT: c_uint = 0x0f;
pub const RN5T618_PWRONTIMSET: c_uint = 0x10;
pub const RN5T618_NOETIMSETCNT: c_uint = 0x11;
pub const RN5T618_PWRIREN: c_uint = 0x12;
pub const RN5T618_PWRIRQ: c_uint = 0x13;
pub const RN5T618_PWRMON: c_uint = 0x14;
pub const RN5T618_PWRIRSEL: c_uint = 0x15;
pub const RN5T618_DC1_SLOT: c_uint = 0x16;
pub const RN5T618_DC2_SLOT: c_uint = 0x17;
pub const RN5T618_DC3_SLOT: c_uint = 0x18;
pub const RN5T618_DC4_SLOT: c_uint = 0x19;
pub const RN5T618_LDO1_SLOT: c_uint = 0x1b;
pub const RN5T618_LDO2_SLOT: c_uint = 0x1c;
pub const RN5T618_LDO3_SLOT: c_uint = 0x1d;
pub const RN5T618_LDO4_SLOT: c_uint = 0x1e;
pub const RN5T618_LDO5_SLOT: c_uint = 0x1f;
pub const RN5T618_PSO0_SLOT: c_uint = 0x25;
pub const RN5T618_PSO1_SLOT: c_uint = 0x26;
pub const RN5T618_PSO2_SLOT: c_uint = 0x27;
pub const RN5T618_PSO3_SLOT: c_uint = 0x28;
pub const RN5T618_LDORTC1_SLOT: c_uint = 0x2a;
pub const RN5T618_DC1CTL: c_uint = 0x2c;
pub const RN5T618_DC1CTL2: c_uint = 0x2d;
pub const RN5T618_DC2CTL: c_uint = 0x2e;
pub const RN5T618_DC2CTL2: c_uint = 0x2f;
pub const RN5T618_DC3CTL: c_uint = 0x30;
pub const RN5T618_DC3CTL2: c_uint = 0x31;
pub const RN5T618_DC4CTL: c_uint = 0x32;
pub const RN5T618_DC4CTL2: c_uint = 0x33;
pub const RN5T618_DC5CTL: c_uint = 0x34;
pub const RN5T618_DC5CTL2: c_uint = 0x35;
pub const RN5T618_DC1DAC: c_uint = 0x36;
pub const RN5T618_DC2DAC: c_uint = 0x37;
pub const RN5T618_DC3DAC: c_uint = 0x38;
pub const RN5T618_DC4DAC: c_uint = 0x39;
pub const RN5T618_DC5DAC: c_uint = 0x3a;
pub const RN5T618_DC1DAC_SLP: c_uint = 0x3b;
pub const RN5T618_DC2DAC_SLP: c_uint = 0x3c;
pub const RN5T618_DC3DAC_SLP: c_uint = 0x3d;
pub const RN5T618_DC4DAC_SLP: c_uint = 0x3e;
pub const RN5T618_DCIREN: c_uint = 0x40;
pub const RN5T618_DCIRQ: c_uint = 0x41;
pub const RN5T618_DCIRMON: c_uint = 0x42;
pub const RN5T618_LDOEN1: c_uint = 0x44;
pub const RN5T618_LDOEN2: c_uint = 0x45;
pub const RN5T618_LDODIS: c_uint = 0x46;
pub const RN5T618_LDO1DAC: c_uint = 0x4c;
pub const RN5T618_LDO2DAC: c_uint = 0x4d;
pub const RN5T618_LDO3DAC: c_uint = 0x4e;
pub const RN5T618_LDO4DAC: c_uint = 0x4f;
pub const RN5T618_LDO5DAC: c_uint = 0x50;
pub const RN5T618_LDO6DAC: c_uint = 0x51;
pub const RN5T618_LDO7DAC: c_uint = 0x52;
pub const RN5T618_LDO8DAC: c_uint = 0x53;
pub const RN5T618_LDO9DAC: c_uint = 0x54;
pub const RN5T618_LDO10DAC: c_uint = 0x55;
pub const RN5T618_LDORTCDAC: c_uint = 0x56;
pub const RN5T618_LDORTC2DAC: c_uint = 0x57;
pub const RN5T618_LDO1DAC_SLP: c_uint = 0x58;
pub const RN5T618_LDO2DAC_SLP: c_uint = 0x59;
pub const RN5T618_LDO3DAC_SLP: c_uint = 0x5a;
pub const RN5T618_LDO4DAC_SLP: c_uint = 0x5b;
pub const RN5T618_LDO5DAC_SLP: c_uint = 0x5c;
pub const RN5T618_ADCCNT1: c_uint = 0x64;
pub const RN5T618_ADCCNT2: c_uint = 0x65;
pub const RN5T618_ADCCNT3: c_uint = 0x66;
pub const RN5T618_ILIMDATAH: c_uint = 0x68;
pub const RN5T618_ILIMDATAL: c_uint = 0x69;
pub const RN5T618_VBATDATAH: c_uint = 0x6a;
pub const RN5T618_VBATDATAL: c_uint = 0x6b;
pub const RN5T618_VADPDATAH: c_uint = 0x6c;
pub const RN5T618_VADPDATAL: c_uint = 0x6d;
pub const RN5T618_VUSBDATAH: c_uint = 0x6e;
pub const RN5T618_VUSBDATAL: c_uint = 0x6f;
pub const RN5T618_VSYSDATAH: c_uint = 0x70;
pub const RN5T618_VSYSDATAL: c_uint = 0x71;
pub const RN5T618_VTHMDATAH: c_uint = 0x72;
pub const RN5T618_VTHMDATAL: c_uint = 0x73;
pub const RN5T618_AIN1DATAH: c_uint = 0x74;
pub const RN5T618_AIN1DATAL: c_uint = 0x75;
pub const RN5T618_AIN0DATAH: c_uint = 0x76;
pub const RN5T618_AIN0DATAL: c_uint = 0x77;
pub const RN5T618_ILIMTHL: c_uint = 0x78;
pub const RN5T618_ILIMTHH: c_uint = 0x79;
pub const RN5T618_VBATTHL: c_uint = 0x7a;
pub const RN5T618_VBATTHH: c_uint = 0x7b;
pub const RN5T618_VADPTHL: c_uint = 0x7c;
pub const RN5T618_VADPTHH: c_uint = 0x7d;
pub const RN5T618_VUSBTHL: c_uint = 0x7e;
pub const RN5T618_VUSBTHH: c_uint = 0x7f;
pub const RN5T618_VSYSTHL: c_uint = 0x80;
pub const RN5T618_VSYSTHH: c_uint = 0x81;
pub const RN5T618_VTHMTHL: c_uint = 0x82;
pub const RN5T618_VTHMTHH: c_uint = 0x83;
pub const RN5T618_AIN1THL: c_uint = 0x84;
pub const RN5T618_AIN1THH: c_uint = 0x85;
pub const RN5T618_AIN0THL: c_uint = 0x86;
pub const RN5T618_AIN0THH: c_uint = 0x87;
pub const RN5T618_EN_ADCIR1: c_uint = 0x88;
pub const RN5T618_EN_ADCIR2: c_uint = 0x89;
pub const RN5T618_EN_ADCIR3: c_uint = 0x8a;
pub const RN5T618_IR_ADC1: c_uint = 0x8c;
pub const RN5T618_IR_ADC2: c_uint = 0x8d;
pub const RN5T618_IR_ADC3: c_uint = 0x8e;
pub const RN5T618_IOSEL: c_uint = 0x90;
pub const RN5T618_IOOUT: c_uint = 0x91;
pub const RN5T618_GPEDGE1: c_uint = 0x92;
pub const RN5T618_GPEDGE2: c_uint = 0x93;
pub const RN5T618_EN_GPIR: c_uint = 0x94;
pub const RN5T618_IR_GPR: c_uint = 0x95;
pub const RN5T618_IR_GPF: c_uint = 0x96;
pub const RN5T618_MON_IOIN: c_uint = 0x97;
pub const RN5T618_GPLED_FUNC: c_uint = 0x98;
pub const RN5T618_INTPOL: c_uint = 0x9c;
pub const RN5T618_INTEN: c_uint = 0x9d;
pub const RN5T618_INTMON: c_uint = 0x9e;
pub const RN5T618_RTC_SECONDS: c_uint = 0xA0;
pub const RN5T618_RTC_MDAY: c_uint = 0xA4;
pub const RN5T618_RTC_MONTH: c_uint = 0xA5;
pub const RN5T618_RTC_YEAR: c_uint = 0xA6;
pub const RN5T618_RTC_ADJUST: c_uint = 0xA7;
pub const RN5T618_RTC_ALARM_Y_SEC: c_uint = 0xA8;
pub const RN5T618_RTC_DAL_MONTH: c_uint = 0xAC;
pub const RN5T618_RTC_CTRL1: c_uint = 0xAE;
pub const RN5T618_RTC_CTRL2: c_uint = 0xAF;
pub const RN5T618_PREVINDAC: c_uint = 0xb0;
pub const RN5T618_BATDAC: c_uint = 0xb1;
pub const RN5T618_CHGCTL1: c_uint = 0xb3;
pub const RN5T618_CHGCTL2: c_uint = 0xb4;
pub const RN5T618_VSYSSET: c_uint = 0xb5;
pub const RN5T618_REGISET1: c_uint = 0xb6;
pub const RN5T618_REGISET2: c_uint = 0xb7;
pub const RN5T618_CHGISET: c_uint = 0xb8;
pub const RN5T618_TIMSET: c_uint = 0xb9;
pub const RN5T618_BATSET1: c_uint = 0xba;
pub const RN5T618_BATSET2: c_uint = 0xbb;
pub const RN5T618_DIESET: c_uint = 0xbc;
pub const RN5T618_CHGSTATE: c_uint = 0xbd;
pub const RN5T618_CHGCTRL_IRFMASK: c_uint = 0xbe;
pub const RN5T618_CHGSTAT_IRFMASK1: c_uint = 0xbf;
pub const RN5T618_CHGSTAT_IRFMASK2: c_uint = 0xc0;
pub const RN5T618_CHGERR_IRFMASK: c_uint = 0xc1;
pub const RN5T618_CHGCTRL_IRR: c_uint = 0xc2;
pub const RN5T618_CHGSTAT_IRR1: c_uint = 0xc3;
pub const RN5T618_CHGSTAT_IRR2: c_uint = 0xc4;
pub const RN5T618_CHGERR_IRR: c_uint = 0xc5;
pub const RN5T618_CHGCTRL_MONI: c_uint = 0xc6;
pub const RN5T618_CHGSTAT_MONI1: c_uint = 0xc7;
pub const RN5T618_CHGSTAT_MONI2: c_uint = 0xc8;
pub const RN5T618_CHGERR_MONI: c_uint = 0xc9;
pub const RN5T618_CHGCTRL_DETMOD1: c_uint = 0xca;
pub const RN5T618_CHGCTRL_DETMOD2: c_uint = 0xcb;
pub const RN5T618_CHGSTAT_DETMOD1: c_uint = 0xcc;
pub const RN5T618_CHGSTAT_DETMOD2: c_uint = 0xcd;
pub const RN5T618_CHGSTAT_DETMOD3: c_uint = 0xce;
pub const RN5T618_CHGERR_DETMOD1: c_uint = 0xcf;
pub const RN5T618_CHGERR_DETMOD2: c_uint = 0xd0;
pub const RN5T618_CHGOSCCTL: c_uint = 0xd4;
pub const RN5T618_CHGOSCSCORESET1: c_uint = 0xd5;
pub const RN5T618_CHGOSCSCORESET2: c_uint = 0xd6;
pub const RN5T618_CHGOSCSCORESET3: c_uint = 0xd7;
pub const RN5T618_CHGOSCFREQSET1: c_uint = 0xd8;
pub const RN5T618_CHGOSCFREQSET2: c_uint = 0xd9;
pub const RN5T618_GCHGDET: c_uint = 0xda;
pub const RN5T618_CONTROL: c_uint = 0xe0;
pub const RN5T618_SOC: c_uint = 0xe1;
pub const RN5T618_RE_CAP_H: c_uint = 0xe2;
pub const RN5T618_RE_CAP_L: c_uint = 0xe3;
pub const RN5T618_FA_CAP_H: c_uint = 0xe4;
pub const RN5T618_FA_CAP_L: c_uint = 0xe5;
pub const RN5T618_AGE: c_uint = 0xe6;
pub const RN5T618_TT_EMPTY_H: c_uint = 0xe7;
pub const RN5T618_TT_EMPTY_L: c_uint = 0xe8;
pub const RN5T618_TT_FULL_H: c_uint = 0xe9;
pub const RN5T618_TT_FULL_L: c_uint = 0xea;
pub const RN5T618_VOLTAGE_1: c_uint = 0xeb;
pub const RN5T618_VOLTAGE_0: c_uint = 0xec;
pub const RN5T618_TEMP_1: c_uint = 0xed;
pub const RN5T618_TEMP_0: c_uint = 0xee;
pub const RN5T618_CC_CTRL: c_uint = 0xef;
pub const RN5T618_CC_COUNT2: c_uint = 0xf0;
pub const RN5T618_CC_COUNT1: c_uint = 0xf1;
pub const RN5T618_CC_COUNT0: c_uint = 0xf2;
pub const RN5T618_CC_SUMREG3: c_uint = 0xf3;
pub const RN5T618_CC_SUMREG2: c_uint = 0xf4;
pub const RN5T618_CC_SUMREG1: c_uint = 0xf5;
pub const RN5T618_CC_SUMREG0: c_uint = 0xf6;
pub const RN5T618_CC_OFFREG1: c_uint = 0xf7;
pub const RN5T618_CC_OFFREG0: c_uint = 0xf8;
pub const RN5T618_CC_GAINREG1: c_uint = 0xf9;
pub const RN5T618_CC_GAINREG0: c_uint = 0xfa;
pub const RN5T618_CC_AVEREG1: c_uint = 0xfb;
pub const RN5T618_CC_AVEREG0: c_uint = 0xfc;
pub const RN5T618_MAX_REG: c_uint = 0xfc;

pub const RN5T618_WATCHDOG_WDOGTIM_S: c_int = 0;

// RN5T618 IRQ definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rn5t618 {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub variant: c_long,
    pub irq: c_int,
    pub irq_data: *mut regmap_irq_chip_data,
}
