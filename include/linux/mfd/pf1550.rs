//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/pf1550.h
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
// Declarations for the PF1550 PMIC
//
// Copyright (C) 2016 Freescale Semiconductor, Inc.
// Robin Gong <yibin.gong@freescale.com>
//
// Portions Copyright (c) 2025 Savoir-faire Linux Inc.
// Samuel Kayode <samuel.kayode@savoirfairelinux.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pf1550_pmic_reg {
// PMIC regulator part
    PF1550_PMIC_REG_DEVICE_ID		= 0x00,
    PF1550_PMIC_REG_OTP_FLAVOR		= 0x01,
    PF1550_PMIC_REG_SILICON_REV		= 0x02,

    PF1550_PMIC_REG_INT_CATEGORY		= 0x06,
    PF1550_PMIC_REG_SW_INT_STAT0		= 0x08,
    PF1550_PMIC_REG_SW_INT_MASK0		= 0x09,
    PF1550_PMIC_REG_SW_INT_SENSE0		= 0x0a,
    PF1550_PMIC_REG_SW_INT_STAT1		= 0x0b,
    PF1550_PMIC_REG_SW_INT_MASK1		= 0x0c,
    PF1550_PMIC_REG_SW_INT_SENSE1		= 0x0d,
    PF1550_PMIC_REG_SW_INT_STAT2		= 0x0e,
    PF1550_PMIC_REG_SW_INT_MASK2		= 0x0f,
    PF1550_PMIC_REG_SW_INT_SENSE2		= 0x10,
    PF1550_PMIC_REG_LDO_INT_STAT0		= 0x18,
    PF1550_PMIC_REG_LDO_INT_MASK0		= 0x19,
    PF1550_PMIC_REG_LDO_INT_SENSE0		= 0x1a,
    PF1550_PMIC_REG_TEMP_INT_STAT0		= 0x20,
    PF1550_PMIC_REG_TEMP_INT_MASK0		= 0x21,
    PF1550_PMIC_REG_TEMP_INT_SENSE0		= 0x22,
    PF1550_PMIC_REG_ONKEY_INT_STAT0		= 0x24,
    PF1550_PMIC_REG_ONKEY_INT_MASK0		= 0x25,
    PF1550_PMIC_REG_ONKEY_INT_SENSE0	= 0x26,
    PF1550_PMIC_REG_MISC_INT_STAT0		= 0x28,
    PF1550_PMIC_REG_MISC_INT_MASK0		= 0x29,
    PF1550_PMIC_REG_MISC_INT_SENSE0		= 0x2a,

    PF1550_PMIC_REG_COINCELL_CONTROL	= 0x30,

    PF1550_PMIC_REG_SW1_VOLT		= 0x32,
    PF1550_PMIC_REG_SW1_STBY_VOLT		= 0x33,
    PF1550_PMIC_REG_SW1_SLP_VOLT		= 0x34,
    PF1550_PMIC_REG_SW1_CTRL		= 0x35,
    PF1550_PMIC_REG_SW1_CTRL1		= 0x36,
    PF1550_PMIC_REG_SW2_VOLT		= 0x38,
    PF1550_PMIC_REG_SW2_STBY_VOLT		= 0x39,
    PF1550_PMIC_REG_SW2_SLP_VOLT		= 0x3a,
    PF1550_PMIC_REG_SW2_CTRL		= 0x3b,
    PF1550_PMIC_REG_SW2_CTRL1		= 0x3c,
    PF1550_PMIC_REG_SW3_VOLT		= 0x3e,
    PF1550_PMIC_REG_SW3_STBY_VOLT		= 0x3f,
    PF1550_PMIC_REG_SW3_SLP_VOLT		= 0x40,
    PF1550_PMIC_REG_SW3_CTRL		= 0x41,
    PF1550_PMIC_REG_SW3_CTRL1		= 0x42,
    PF1550_PMIC_REG_VSNVS_CTRL		= 0x48,
    PF1550_PMIC_REG_VREFDDR_CTRL		= 0x4a,
    PF1550_PMIC_REG_LDO1_VOLT		= 0x4c,
    PF1550_PMIC_REG_LDO1_CTRL		= 0x4d,
    PF1550_PMIC_REG_LDO2_VOLT		= 0x4f,
    PF1550_PMIC_REG_LDO2_CTRL		= 0x50,
    PF1550_PMIC_REG_LDO3_VOLT		= 0x52,
    PF1550_PMIC_REG_LDO3_CTRL		= 0x53,
    PF1550_PMIC_REG_PWRCTRL0		= 0x58,
    PF1550_PMIC_REG_PWRCTRL1		= 0x59,
    PF1550_PMIC_REG_PWRCTRL2		= 0x5a,
    PF1550_PMIC_REG_PWRCTRL3		= 0x5b,
    PF1550_PMIC_REG_SW1_PWRDN_SEQ		= 0x5f,
    PF1550_PMIC_REG_SW2_PWRDN_SEQ		= 0x60,
    PF1550_PMIC_REG_SW3_PWRDN_SEQ		= 0x61,
    PF1550_PMIC_REG_LDO1_PWRDN_SEQ		= 0x62,
    PF1550_PMIC_REG_LDO2_PWRDN_SEQ		= 0x63,
    PF1550_PMIC_REG_LDO3_PWRDN_SEQ		= 0x64,
    PF1550_PMIC_REG_VREFDDR_PWRDN_SEQ	= 0x65,

    PF1550_PMIC_REG_STATE_INFO		= 0x67,
    PF1550_PMIC_REG_I2C_ADDR		= 0x68,
    PF1550_PMIC_REG_IO_DRV0			= 0x69,
    PF1550_PMIC_REG_IO_DRV1			= 0x6a,
    PF1550_PMIC_REG_RC_16MHZ		= 0x6b,
    PF1550_PMIC_REG_KEY			= 0x6f,

// Charger part
    PF1550_CHARG_REG_CHG_INT		= 0x80,
    PF1550_CHARG_REG_CHG_INT_MASK		= 0x82,
    PF1550_CHARG_REG_CHG_INT_OK		= 0x84,
    PF1550_CHARG_REG_VBUS_SNS		= 0x86,
    PF1550_CHARG_REG_CHG_SNS		= 0x87,
    PF1550_CHARG_REG_BATT_SNS		= 0x88,
    PF1550_CHARG_REG_CHG_OPER		= 0x89,
    PF1550_CHARG_REG_CHG_TMR		= 0x8a,
    PF1550_CHARG_REG_CHG_EOC_CNFG		= 0x8d,
    PF1550_CHARG_REG_CHG_CURR_CNFG		= 0x8e,
    PF1550_CHARG_REG_BATT_REG		= 0x8f,
    PF1550_CHARG_REG_BATFET_CNFG		= 0x91,
    PF1550_CHARG_REG_THM_REG_CNFG		= 0x92,
    PF1550_CHARG_REG_VBUS_INLIM_CNFG	= 0x94,
    PF1550_CHARG_REG_VBUS_LIN_DPM		= 0x95,
    PF1550_CHARG_REG_USB_PHY_LDO_CNFG	= 0x96,
    PF1550_CHARG_REG_DBNC_DELAY_TIME	= 0x98,
    PF1550_CHARG_REG_CHG_INT_CNFG		= 0x99,
    PF1550_CHARG_REG_THM_ADJ_SETTING	= 0x9a,
    PF1550_CHARG_REG_VBUS2SYS_CNFG		= 0x9b,
    PF1550_CHARG_REG_LED_PWM		= 0x9c,
    PF1550_CHARG_REG_FAULT_BATFET_CNFG	= 0x9d,
    PF1550_CHARG_REG_LED_CNFG		= 0x9e,
    PF1550_CHARG_REG_CHGR_KEY2		= 0x9f,

    PF1550_TEST_REG_FMRADDR			= 0xc4,
    PF1550_TEST_REG_FMRDATA			= 0xc5,
    PF1550_TEST_REG_KEY3			= 0xdf,

    PF1550_PMIC_REG_END			= 0xff,
}

// One-Time Programmable(OTP) memory
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pf1550_otp_reg {
    PF1550_OTP_SW1_SW2			= 0x1e,
    PF1550_OTP_SW2_SW3			= 0x1f,
}

pub const PF1550_DEVICE_ID: c_uint = 0x7c;
// Keys for reading OTP
pub const PF1550_OTP_PMIC_KEY: c_uint = 0x15;
pub const PF1550_OTP_CHGR_KEY: c_uint = 0x50;
pub const PF1550_OTP_TEST_KEY: c_uint = 0xab;
// Supported charger modes
pub const PF1550_CHG_BAT_OFF: c_int = 1;
pub const PF1550_CHG_BAT_ON: c_int = 2;
pub const PF1550_CHG_PRECHARGE: c_int = 0;
pub const PF1550_CHG_CONSTANT_CURRENT: c_int = 1;
pub const PF1550_CHG_CONSTANT_VOL: c_int = 2;
pub const PF1550_CHG_EOC: c_int = 3;
pub const PF1550_CHG_DONE: c_int = 4;
pub const PF1550_CHG_TIMER_FAULT: c_int = 6;
pub const PF1550_CHG_SUSPEND: c_int = 7;
pub const PF1550_CHG_OFF_INV: c_int = 8;
pub const PF1550_CHG_BAT_OVER: c_int = 9;
pub const PF1550_CHG_OFF_TEMP: c_int = 10;
pub const PF1550_CHG_LINEAR_ONLY: c_int = 12;
pub const PF1550_CHG_SNS_MASK: c_uint = 0xf;
pub const PF1550_CHG_INT_MASK: c_uint = 0x51;
pub const PF1550_BAT_NO_VBUS: c_int = 0;
pub const PF1550_BAT_LOW_THAN_PRECHARG: c_int = 1;
pub const PF1550_BAT_CHARG_FAIL: c_int = 2;
pub const PF1550_BAT_HIGH_THAN_PRECHARG: c_int = 4;
pub const PF1550_BAT_OVER_VOL: c_int = 5;
pub const PF1550_BAT_NO_DETECT: c_int = 6;
pub const PF1550_BAT_SNS_MASK: c_uint = 0x7;

pub const PF1550_CHARG_REG_BATT_REG_CHGCV_MASK: c_uint = 0x3f;
pub const PF1550_CHARG_REG_BATT_REG_VMINSYS_SHIFT: c_int = 6;

pub const PF1550_CHARG_REG_THM_REG_CNFG_REGTEMP_SHIFT: c_int = 2;

// DVS enable masks

// Top level interrupt masks

// Regulator interrupt masks

// Onkey interrupt masks

// Charger interrupt masks

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pf1550_irq {
    PF1550_IRQ_CHG,
    PF1550_IRQ_REGULATOR,
    PF1550_IRQ_ONKEY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pf1550_pmic_irq {
    PF1550_PMIC_IRQ_SW1_LS,
    PF1550_PMIC_IRQ_SW2_LS,
    PF1550_PMIC_IRQ_SW3_LS,
    PF1550_PMIC_IRQ_SW1_HS,
    PF1550_PMIC_IRQ_SW2_HS,
    PF1550_PMIC_IRQ_SW3_HS,
    PF1550_PMIC_IRQ_LDO1_FAULT,
    PF1550_PMIC_IRQ_LDO2_FAULT,
    PF1550_PMIC_IRQ_LDO3_FAULT,
    PF1550_PMIC_IRQ_TEMP_110,
    PF1550_PMIC_IRQ_TEMP_125,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pf1550_onkey_irq {
    PF1550_ONKEY_IRQ_PUSHI,
    PF1550_ONKEY_IRQ_1SI,
    PF1550_ONKEY_IRQ_2SI,
    PF1550_ONKEY_IRQ_3SI,
    PF1550_ONKEY_IRQ_4SI,
    PF1550_ONKEY_IRQ_8SI,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pf1550_charg_irq {
    PF1550_CHARG_IRQ_BAT2SOCI,
    PF1550_CHARG_IRQ_BATI,
    PF1550_CHARG_IRQ_CHGI,
    PF1550_CHARG_IRQ_VBUSI,
    PF1550_CHARG_IRQ_THMI,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pf1550_regulators {
    PF1550_SW1,
    PF1550_SW2,
    PF1550_SW3,
    PF1550_VREFDDR,
    PF1550_LDO1,
    PF1550_LDO2,
    PF1550_LDO3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf1550_ddata {
    pub irq_data_regulator: *mut regmap_irq_chip_data,
    pub irq_data_charger: *mut regmap_irq_chip_data,
    pub irq_data_onkey: *mut regmap_irq_chip_data,
    pub irq_data: *mut regmap_irq_chip_data,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub dvs1_enable: bool,
    pub dvs2_enable: bool,
    pub irq: c_int,
}
