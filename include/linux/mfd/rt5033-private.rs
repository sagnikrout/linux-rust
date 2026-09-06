//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rt5033-private.h
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
// MFD core driver for Richtek RT5033
//
// Copyright (C) 2014 Samsung Electronics, Co., Ltd.
// Author: Beomho Seo <beomho.seo@samsung.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt5033_reg {
    RT5033_REG_CHG_STAT		= 0x00,
    RT5033_REG_CHG_CTRL1		= 0x01,
    RT5033_REG_CHG_CTRL2		= 0x02,
    RT5033_REG_DEVICE_ID		= 0x03,
    RT5033_REG_CHG_CTRL3		= 0x04,
    RT5033_REG_CHG_CTRL4		= 0x05,
    RT5033_REG_CHG_CTRL5		= 0x06,
    RT5033_REG_RT_CTRL0		= 0x07,
    RT5033_REG_CHG_RESET		= 0x08,
// Reserved 0x09~0x18
    RT5033_REG_RT_CTRL1		= 0x19,
// Reserved 0x1A~0x20
    RT5033_REG_FLED_FUNCTION1	= 0x21,
    RT5033_REG_FLED_FUNCTION2	= 0x22,
    RT5033_REG_FLED_STROBE_CTRL1	= 0x23,
    RT5033_REG_FLED_STROBE_CTRL2	= 0x24,
    RT5033_REG_FLED_CTRL1		= 0x25,
    RT5033_REG_FLED_CTRL2		= 0x26,
    RT5033_REG_FLED_CTRL3		= 0x27,
    RT5033_REG_FLED_CTRL4		= 0x28,
    RT5033_REG_FLED_CTRL5		= 0x29,
// Reserved 0x2A~0x40
    RT5033_REG_CTRL			= 0x41,
    RT5033_REG_BUCK_CTRL		= 0x42,
    RT5033_REG_LDO_CTRL		= 0x43,
// Reserved 0x44~0x46
    RT5033_REG_MANUAL_RESET_CTRL	= 0x47,
// Reserved 0x48~0x5F
    RT5033_REG_CHG_IRQ1		= 0x60,
    RT5033_REG_CHG_IRQ2		= 0x61,
    RT5033_REG_CHG_IRQ3		= 0x62,
    RT5033_REG_CHG_IRQ1_CTRL	= 0x63,
    RT5033_REG_CHG_IRQ2_CTRL	= 0x64,
    RT5033_REG_CHG_IRQ3_CTRL	= 0x65,
    RT5033_REG_LED_IRQ_STAT		= 0x66,
    RT5033_REG_LED_IRQ_CTRL		= 0x67,
    RT5033_REG_PMIC_IRQ_STAT	= 0x68,
    RT5033_REG_PMIC_IRQ_CTRL	= 0x69,
    RT5033_REG_SHDN_CTRL		= 0x6A,
    RT5033_REG_OFF_EVENT		= 0x6B,

    RT5033_REG_END,
}

// RT5033 Charger state register
pub const RT5033_CHG_STAT_TYPE_MASK: c_uint = 0x60;
pub const RT5033_CHG_STAT_TYPE_PRE: c_uint = 0x20;
pub const RT5033_CHG_STAT_TYPE_FAST: c_uint = 0x60;
pub const RT5033_CHG_STAT_MASK: c_uint = 0x30;
pub const RT5033_CHG_STAT_DISCHARGING: c_uint = 0x00;
pub const RT5033_CHG_STAT_FULL: c_uint = 0x10;
pub const RT5033_CHG_STAT_CHARGING: c_uint = 0x20;
pub const RT5033_CHG_STAT_NOT_CHARGING: c_uint = 0x30;
// RT5033 CHGCTRL1 register
pub const RT5033_CHGCTRL1_IAICR_MASK: c_uint = 0xe0;
pub const RT5033_CHGCTRL1_TE_EN_MASK: c_uint = 0x08;
pub const RT5033_CHGCTRL1_HZ_MASK: c_uint = 0x02;
pub const RT5033_CHGCTRL1_MODE_MASK: c_uint = 0x01;
// RT5033 CHGCTRL2 register
pub const RT5033_CHGCTRL2_CV_MASK: c_uint = 0xfc;
pub const RT5033_CHGCTRL2_CV_SHIFT: c_uint = 0x02;
// RT5033 DEVICE_ID register
pub const RT5033_VENDOR_ID_MASK: c_uint = 0xf0;
pub const RT5033_CHIP_REV_MASK: c_uint = 0x0f;
// RT5033 CHGCTRL3 register
pub const RT5033_CHGCTRL3_CFO_EN_MASK: c_uint = 0x40;
pub const RT5033_CHGCTRL3_TIMER_MASK: c_uint = 0x38;
pub const RT5033_CHGCTRL3_TIMER_EN_MASK: c_uint = 0x01;
// RT5033 CHGCTRL4 register
pub const RT5033_CHGCTRL4_MIVR_MASK: c_uint = 0xe0;
pub const RT5033_CHGCTRL4_IPREC_MASK: c_uint = 0x18;
pub const RT5033_CHGCTRL4_IPREC_SHIFT: c_uint = 0x03;
pub const RT5033_CHGCTRL4_EOC_MASK: c_uint = 0x07;
// RT5033 CHGCTRL5 register
pub const RT5033_CHGCTRL5_ICHG_MASK: c_uint = 0xf0;
pub const RT5033_CHGCTRL5_ICHG_SHIFT: c_uint = 0x04;
pub const RT5033_CHGCTRL5_VPREC_MASK: c_uint = 0x0f;
// RT5033 RT CTRL1 register
pub const RT5033_RT_CTRL1_UUG_MASK: c_uint = 0x02;
// RT5033 control register

// RT5033 BUCK control register
pub const RT5033_BUCK_CTRL_MASK: c_uint = 0x1f;
// RT5033 LDO control register
pub const RT5033_LDO_CTRL_MASK: c_uint = 0x1f;
// RT5033 charger property - model, manufacturer

//
// While RT5033 charger can limit the fast-charge current (as in CHGCTRL1
// register), AICR mode limits the input current. For example, the AIRC 100
// mode limits the input current to 100 mA.
//
pub const RT5033_AICR_DISABLE: c_uint = 0x00;
pub const RT5033_AICR_100_MODE: c_uint = 0x20;
pub const RT5033_AICR_500_MODE: c_uint = 0x40;
pub const RT5033_AICR_700_MODE: c_uint = 0x60;
pub const RT5033_AICR_900_MODE: c_uint = 0x80;
pub const RT5033_AICR_1000_MODE: c_uint = 0xa0;
pub const RT5033_AICR_1500_MODE: c_uint = 0xc0;
pub const RT5033_AICR_2000_MODE: c_uint = 0xe0;
// RT5033 charger minimum input voltage regulation
pub const RT5033_CHARGER_MIVR_DISABLE: c_uint = 0x00;
pub const RT5033_CHARGER_MIVR_4200MV: c_uint = 0x20;
pub const RT5033_CHARGER_MIVR_4300MV: c_uint = 0x40;
pub const RT5033_CHARGER_MIVR_4400MV: c_uint = 0x60;
pub const RT5033_CHARGER_MIVR_4500MV: c_uint = 0x80;
pub const RT5033_CHARGER_MIVR_4600MV: c_uint = 0xa0;
pub const RT5033_CHARGER_MIVR_4700MV: c_uint = 0xc0;
pub const RT5033_CHARGER_MIVR_4800MV: c_uint = 0xe0;
// RT5033 use internal timer need to set time
pub const RT5033_FAST_CHARGE_TIMER4: c_uint = 0x00 /*  4 hrs */;
pub const RT5033_FAST_CHARGE_TIMER6: c_uint = 0x08 /*  6 hrs */;
pub const RT5033_FAST_CHARGE_TIMER8: c_uint = 0x10 /*  8 hrs */;
pub const RT5033_FAST_CHARGE_TIMER10: c_uint = 0x18 /* 10 hrs */;
pub const RT5033_FAST_CHARGE_TIMER12: c_uint = 0x20 /* 12 hrs */;
pub const RT5033_FAST_CHARGE_TIMER14: c_uint = 0x28 /* 14 hrs */;
pub const RT5033_FAST_CHARGE_TIMER16: c_uint = 0x30 /* 16 hrs */;
pub const RT5033_INT_TIMER_DISABLE: c_uint = 0x00;
pub const RT5033_INT_TIMER_ENABLE: c_uint = 0x01;
//
// RT5033 charger opa mode. RT5033 has two opa modes for OTG: charger mode
// and boost mode.
//
pub const RT5033_CHARGER_MODE: c_uint = 0x00;
pub const RT5033_BOOST_MODE: c_uint = 0x01;
// RT5033 charger termination enable
pub const RT5033_TE_DISABLE: c_uint = 0x00;
pub const RT5033_TE_ENABLE: c_uint = 0x08;
// RT5033 charger CFO enable
pub const RT5033_CFO_DISABLE: c_uint = 0x00;
pub const RT5033_CFO_ENABLE: c_uint = 0x40;
// RT5033 charger constant charge voltage (as in CHGCTRL2 register), uV

pub const RT5033_CV_MAX_VOLTAGE: c_uint = 0x1e;
// RT5033 charger pre-charge current limits (as in CHGCTRL4 register), uA

pub const RT5033_CHG_MAX_PRE_CURRENT: c_uint = 0x03;
// RT5033 charger fast-charge current (as in CHGCTRL5 register), uA

pub const RT5033_CHG_MAX_CURRENT: c_uint = 0x0d;
//
// RT5033 charger const-charge end of charger current (
// as in CHGCTRL4 register), uA
//

//
// RT5033 charger pre-charge threshold volt limits
// (as in CHGCTRL5 register), uV
//

//
// RT5033 charger UUG. It enables MOS auto control by H/W charger
// circuit.
//
pub const RT5033_CHARGER_UUG_DISABLE: c_uint = 0x00;
pub const RT5033_CHARGER_UUG_ENABLE: c_uint = 0x02;
// RT5033 charger high impedance mode
pub const RT5033_CHARGER_HZ_DISABLE: c_uint = 0x00;
pub const RT5033_CHARGER_HZ_ENABLE: c_uint = 0x02;
// RT5033 regulator BUCK output voltage uV

pub const RT5033_REGULATOR_BUCK_VOLTAGE_STEP_NUM: c_int = 32;
// RT5033 regulator LDO output voltage uV

pub const RT5033_REGULATOR_LDO_VOLTAGE_STEP_NUM: c_int = 32;
// RT5033 regulator SAFE LDO output voltage uV

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt5033_fuel_reg {
    RT5033_FUEL_REG_OCV_H		= 0x00,
    RT5033_FUEL_REG_OCV_L		= 0x01,
    RT5033_FUEL_REG_VBAT_H		= 0x02,
    RT5033_FUEL_REG_VBAT_L		= 0x03,
    RT5033_FUEL_REG_SOC_H		= 0x04,
    RT5033_FUEL_REG_SOC_L		= 0x05,
    RT5033_FUEL_REG_CTRL_H		= 0x06,
    RT5033_FUEL_REG_CTRL_L		= 0x07,
    RT5033_FUEL_REG_CRATE		= 0x08,
    RT5033_FUEL_REG_DEVICE_ID	= 0x09,
    RT5033_FUEL_REG_AVG_VOLT_H	= 0x0A,
    RT5033_FUEL_REG_AVG_VOLT_L	= 0x0B,
    RT5033_FUEL_REG_CONFIG_H	= 0x0C,
    RT5033_FUEL_REG_CONFIG_L	= 0x0D,
// Reserved 0x0E~0x0F
    RT5033_FUEL_REG_IRQ_CTRL	= 0x10,
    RT5033_FUEL_REG_IRQ_FLAG	= 0x11,
    RT5033_FUEL_VMIN		= 0x12,
    RT5033_FUEL_SMIN		= 0x13,
// Reserved 0x14~0x1F
    RT5033_FUEL_VGCOMP1		= 0x20,
    RT5033_FUEL_VGCOMP2		= 0x21,
    RT5033_FUEL_VGCOMP3		= 0x22,
    RT5033_FUEL_VGCOMP4		= 0x23,
// Reserved 0x24~0xFD
    RT5033_FUEL_MFA_H		= 0xFE,
    RT5033_FUEL_MFA_L		= 0xFF,

    RT5033_FUEL_REG_END,
}

// RT5033 fuel gauge battery present property
pub const RT5033_FUEL_BAT_PRESENT: c_uint = 0x02;
// RT5033 PMIC interrupts

