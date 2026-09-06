//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max8997-private.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// max8997-private.h - Voltage regulator driver for the Maxim 8997
//
// Copyright (C) 2010 Samsung Electronics
// MyungJoo Ham <myungjoo.ham@samsung.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max8997_pmic_reg {
    MAX8997_REG_PMIC_ID0	= 0x00,
    MAX8997_REG_PMIC_ID1	= 0x01,
    MAX8997_REG_INTSRC	= 0x02,
    MAX8997_REG_INT1	= 0x03,
    MAX8997_REG_INT2	= 0x04,
    MAX8997_REG_INT3	= 0x05,
    MAX8997_REG_INT4	= 0x06,

    MAX8997_REG_INT1MSK	= 0x08,
    MAX8997_REG_INT2MSK	= 0x09,
    MAX8997_REG_INT3MSK	= 0x0a,
    MAX8997_REG_INT4MSK	= 0x0b,

    MAX8997_REG_STATUS1	= 0x0d,
    MAX8997_REG_STATUS2	= 0x0e,
    MAX8997_REG_STATUS3	= 0x0f,
    MAX8997_REG_STATUS4	= 0x10,

    MAX8997_REG_MAINCON1	= 0x13,
    MAX8997_REG_MAINCON2	= 0x14,
    MAX8997_REG_BUCKRAMP	= 0x15,

    MAX8997_REG_BUCK1CTRL	= 0x18,
    MAX8997_REG_BUCK1DVS1	= 0x19,
    MAX8997_REG_BUCK1DVS2	= 0x1a,
    MAX8997_REG_BUCK1DVS3	= 0x1b,
    MAX8997_REG_BUCK1DVS4	= 0x1c,
    MAX8997_REG_BUCK1DVS5	= 0x1d,
    MAX8997_REG_BUCK1DVS6	= 0x1e,
    MAX8997_REG_BUCK1DVS7	= 0x1f,
    MAX8997_REG_BUCK1DVS8	= 0x20,
    MAX8997_REG_BUCK2CTRL	= 0x21,
    MAX8997_REG_BUCK2DVS1	= 0x22,
    MAX8997_REG_BUCK2DVS2	= 0x23,
    MAX8997_REG_BUCK2DVS3	= 0x24,
    MAX8997_REG_BUCK2DVS4	= 0x25,
    MAX8997_REG_BUCK2DVS5	= 0x26,
    MAX8997_REG_BUCK2DVS6	= 0x27,
    MAX8997_REG_BUCK2DVS7	= 0x28,
    MAX8997_REG_BUCK2DVS8	= 0x29,
    MAX8997_REG_BUCK3CTRL	= 0x2a,
    MAX8997_REG_BUCK3DVS	= 0x2b,
    MAX8997_REG_BUCK4CTRL	= 0x2c,
    MAX8997_REG_BUCK4DVS	= 0x2d,
    MAX8997_REG_BUCK5CTRL	= 0x2e,
    MAX8997_REG_BUCK5DVS1	= 0x2f,
    MAX8997_REG_BUCK5DVS2	= 0x30,
    MAX8997_REG_BUCK5DVS3	= 0x31,
    MAX8997_REG_BUCK5DVS4	= 0x32,
    MAX8997_REG_BUCK5DVS5	= 0x33,
    MAX8997_REG_BUCK5DVS6	= 0x34,
    MAX8997_REG_BUCK5DVS7	= 0x35,
    MAX8997_REG_BUCK5DVS8	= 0x36,
    MAX8997_REG_BUCK6CTRL	= 0x37,
    MAX8997_REG_BUCK6BPSKIPCTRL	= 0x38,
    MAX8997_REG_BUCK7CTRL	= 0x39,
    MAX8997_REG_BUCK7DVS	= 0x3a,
    MAX8997_REG_LDO1CTRL	= 0x3b,
    MAX8997_REG_LDO2CTRL	= 0x3c,
    MAX8997_REG_LDO3CTRL	= 0x3d,
    MAX8997_REG_LDO4CTRL	= 0x3e,
    MAX8997_REG_LDO5CTRL	= 0x3f,
    MAX8997_REG_LDO6CTRL	= 0x40,
    MAX8997_REG_LDO7CTRL	= 0x41,
    MAX8997_REG_LDO8CTRL	= 0x42,
    MAX8997_REG_LDO9CTRL	= 0x43,
    MAX8997_REG_LDO10CTRL	= 0x44,
    MAX8997_REG_LDO11CTRL	= 0x45,
    MAX8997_REG_LDO12CTRL	= 0x46,
    MAX8997_REG_LDO13CTRL	= 0x47,
    MAX8997_REG_LDO14CTRL	= 0x48,
    MAX8997_REG_LDO15CTRL	= 0x49,
    MAX8997_REG_LDO16CTRL	= 0x4a,
    MAX8997_REG_LDO17CTRL	= 0x4b,
    MAX8997_REG_LDO18CTRL	= 0x4c,
    MAX8997_REG_LDO21CTRL	= 0x4d,

    MAX8997_REG_MBCCTRL1	= 0x50,
    MAX8997_REG_MBCCTRL2	= 0x51,
    MAX8997_REG_MBCCTRL3	= 0x52,
    MAX8997_REG_MBCCTRL4	= 0x53,
    MAX8997_REG_MBCCTRL5	= 0x54,
    MAX8997_REG_MBCCTRL6	= 0x55,
    MAX8997_REG_OTPCGHCVS	= 0x56,

    MAX8997_REG_SAFEOUTCTRL	= 0x5a,

    MAX8997_REG_LBCNFG1	= 0x5e,
    MAX8997_REG_LBCNFG2	= 0x5f,
    MAX8997_REG_BBCCTRL	= 0x60,

    MAX8997_REG_FLASH1_CUR	= 0x63, /* 0x63 ~ 0x6e for FLASH */
    MAX8997_REG_FLASH2_CUR	= 0x64,
    MAX8997_REG_MOVIE_CUR	= 0x65,
    MAX8997_REG_GSMB_CUR	= 0x66,
    MAX8997_REG_BOOST_CNTL	= 0x67,
    MAX8997_REG_LEN_CNTL	= 0x68,
    MAX8997_REG_FLASH_CNTL	= 0x69,
    MAX8997_REG_WDT_CNTL	= 0x6a,
    MAX8997_REG_MAXFLASH1	= 0x6b,
    MAX8997_REG_MAXFLASH2	= 0x6c,
    MAX8997_REG_FLASHSTATUS	= 0x6d,
    MAX8997_REG_FLASHSTATUSMASK	= 0x6e,

    MAX8997_REG_GPIOCNTL1	= 0x70,
    MAX8997_REG_GPIOCNTL2	= 0x71,
    MAX8997_REG_GPIOCNTL3	= 0x72,
    MAX8997_REG_GPIOCNTL4	= 0x73,
    MAX8997_REG_GPIOCNTL5	= 0x74,
    MAX8997_REG_GPIOCNTL6	= 0x75,
    MAX8997_REG_GPIOCNTL7	= 0x76,
    MAX8997_REG_GPIOCNTL8	= 0x77,
    MAX8997_REG_GPIOCNTL9	= 0x78,
    MAX8997_REG_GPIOCNTL10	= 0x79,
    MAX8997_REG_GPIOCNTL11	= 0x7a,
    MAX8997_REG_GPIOCNTL12	= 0x7b,

    MAX8997_REG_LDO1CONFIG	= 0x80,
    MAX8997_REG_LDO2CONFIG	= 0x81,
    MAX8997_REG_LDO3CONFIG	= 0x82,
    MAX8997_REG_LDO4CONFIG	= 0x83,
    MAX8997_REG_LDO5CONFIG	= 0x84,
    MAX8997_REG_LDO6CONFIG	= 0x85,
    MAX8997_REG_LDO7CONFIG	= 0x86,
    MAX8997_REG_LDO8CONFIG	= 0x87,
    MAX8997_REG_LDO9CONFIG	= 0x88,
    MAX8997_REG_LDO10CONFIG	= 0x89,
    MAX8997_REG_LDO11CONFIG	= 0x8a,
    MAX8997_REG_LDO12CONFIG	= 0x8b,
    MAX8997_REG_LDO13CONFIG	= 0x8c,
    MAX8997_REG_LDO14CONFIG	= 0x8d,
    MAX8997_REG_LDO15CONFIG	= 0x8e,
    MAX8997_REG_LDO16CONFIG	= 0x8f,
    MAX8997_REG_LDO17CONFIG	= 0x90,
    MAX8997_REG_LDO18CONFIG	= 0x91,
    MAX8997_REG_LDO21CONFIG	= 0x92,

    MAX8997_REG_DVSOKTIMER1	= 0x97,
    MAX8997_REG_DVSOKTIMER2	= 0x98,
    MAX8997_REG_DVSOKTIMER4	= 0x99,
    MAX8997_REG_DVSOKTIMER5	= 0x9a,

    MAX8997_REG_PMIC_END	= 0x9b,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max8997_muic_reg {
    MAX8997_MUIC_REG_ID		= 0x0,
    MAX8997_MUIC_REG_INT1		= 0x1,
    MAX8997_MUIC_REG_INT2		= 0x2,
    MAX8997_MUIC_REG_INT3		= 0x3,
    MAX8997_MUIC_REG_STATUS1	= 0x4,
    MAX8997_MUIC_REG_STATUS2	= 0x5,
    MAX8997_MUIC_REG_STATUS3	= 0x6,
    MAX8997_MUIC_REG_INTMASK1	= 0x7,
    MAX8997_MUIC_REG_INTMASK2	= 0x8,
    MAX8997_MUIC_REG_INTMASK3	= 0x9,
    MAX8997_MUIC_REG_CDETCTRL	= 0xa,

    MAX8997_MUIC_REG_CONTROL1	= 0xc,
    MAX8997_MUIC_REG_CONTROL2	= 0xd,
    MAX8997_MUIC_REG_CONTROL3	= 0xe,

    MAX8997_MUIC_REG_END		= 0xf,
}

// MAX8997-MUIC STATUS1 register
pub const STATUS1_ADC_SHIFT: c_int = 0;
pub const STATUS1_ADCLOW_SHIFT: c_int = 5;
pub const STATUS1_ADCERR_SHIFT: c_int = 6;

// MAX8997-MUIC STATUS2 register
pub const STATUS2_CHGTYP_SHIFT: c_int = 0;
pub const STATUS2_CHGDETRUN_SHIFT: c_int = 3;
pub const STATUS2_DCDTMR_SHIFT: c_int = 4;
pub const STATUS2_DBCHG_SHIFT: c_int = 5;
pub const STATUS2_VBVOLT_SHIFT: c_int = 6;

// MAX8997-MUIC STATUS3 register
pub const STATUS3_OVP_SHIFT: c_int = 2;

// MAX8997-MUIC CONTROL1 register
pub const COMN1SW_SHIFT: c_int = 0;
pub const COMP2SW_SHIFT: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max8997_haptic_reg {
    MAX8997_HAPTIC_REG_GENERAL	= 0x00,
    MAX8997_HAPTIC_REG_CONF1	= 0x01,
    MAX8997_HAPTIC_REG_CONF2	= 0x02,
    MAX8997_HAPTIC_REG_DRVCONF	= 0x03,
    MAX8997_HAPTIC_REG_CYCLECONF1	= 0x04,
    MAX8997_HAPTIC_REG_CYCLECONF2	= 0x05,
    MAX8997_HAPTIC_REG_SIGCONF1	= 0x06,
    MAX8997_HAPTIC_REG_SIGCONF2	= 0x07,
    MAX8997_HAPTIC_REG_SIGCONF3	= 0x08,
    MAX8997_HAPTIC_REG_SIGCONF4	= 0x09,
    MAX8997_HAPTIC_REG_SIGDC1	= 0x0a,
    MAX8997_HAPTIC_REG_SIGDC2	= 0x0b,
    MAX8997_HAPTIC_REG_SIGPWMDC1	= 0x0c,
    MAX8997_HAPTIC_REG_SIGPWMDC2	= 0x0d,
    MAX8997_HAPTIC_REG_SIGPWMDC3	= 0x0e,
    MAX8997_HAPTIC_REG_SIGPWMDC4	= 0x0f,
    MAX8997_HAPTIC_REG_MTR_REV	= 0x10,

    MAX8997_HAPTIC_REG_END		= 0x11,
}

// slave addr = 0x0c: using "2nd part" of rev4 datasheet
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max8997_rtc_reg {
    MAX8997_RTC_CTRLMASK		= 0x02,
    MAX8997_RTC_CTRL		= 0x03,
    MAX8997_RTC_UPDATE1		= 0x04,
    MAX8997_RTC_UPDATE2		= 0x05,
    MAX8997_RTC_WTSR_SMPL		= 0x06,

    MAX8997_RTC_SEC			= 0x10,
    MAX8997_RTC_MIN			= 0x11,
    MAX8997_RTC_HOUR		= 0x12,
    MAX8997_RTC_DAY_OF_WEEK		= 0x13,
    MAX8997_RTC_MONTH		= 0x14,
    MAX8997_RTC_YEAR		= 0x15,
    MAX8997_RTC_DAY_OF_MONTH	= 0x16,
    MAX8997_RTC_ALARM1_SEC		= 0x17,
    MAX8997_RTC_ALARM1_MIN		= 0x18,
    MAX8997_RTC_ALARM1_HOUR		= 0x19,
    MAX8997_RTC_ALARM1_DAY_OF_WEEK	= 0x1a,
    MAX8997_RTC_ALARM1_MONTH	= 0x1b,
    MAX8997_RTC_ALARM1_YEAR		= 0x1c,
    MAX8997_RTC_ALARM1_DAY_OF_MONTH	= 0x1d,
    MAX8997_RTC_ALARM2_SEC		= 0x1e,
    MAX8997_RTC_ALARM2_MIN		= 0x1f,
    MAX8997_RTC_ALARM2_HOUR		= 0x20,
    MAX8997_RTC_ALARM2_DAY_OF_WEEK	= 0x21,
    MAX8997_RTC_ALARM2_MONTH	= 0x22,
    MAX8997_RTC_ALARM2_YEAR		= 0x23,
    MAX8997_RTC_ALARM2_DAY_OF_MONTH	= 0x24,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max8997_irq_source {
    PMIC_INT1 = 0,
    PMIC_INT2,
    PMIC_INT3,
    PMIC_INT4,

    FUEL_GAUGE, /* Ignored (MAX17042 driver handles) */

    MUIC_INT1,
    MUIC_INT2,
    MUIC_INT3,

    GPIO_LOW, /* Not implemented */
    GPIO_HI, /* Not implemented */

    FLASH_STATUS, /* Not implemented */

    MAX8997_IRQ_GROUP_NR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max8997_irq {
    MAX8997_PMICIRQ_PWRONR,
    MAX8997_PMICIRQ_PWRONF,
    MAX8997_PMICIRQ_PWRON1SEC,
    MAX8997_PMICIRQ_JIGONR,
    MAX8997_PMICIRQ_JIGONF,
    MAX8997_PMICIRQ_LOWBAT2,
    MAX8997_PMICIRQ_LOWBAT1,

    MAX8997_PMICIRQ_JIGR,
    MAX8997_PMICIRQ_JIGF,
    MAX8997_PMICIRQ_MR,
    MAX8997_PMICIRQ_DVS1OK,
    MAX8997_PMICIRQ_DVS2OK,
    MAX8997_PMICIRQ_DVS3OK,
    MAX8997_PMICIRQ_DVS4OK,

    MAX8997_PMICIRQ_CHGINS,
    MAX8997_PMICIRQ_CHGRM,
    MAX8997_PMICIRQ_DCINOVP,
    MAX8997_PMICIRQ_TOPOFFR,
    MAX8997_PMICIRQ_CHGRSTF,
    MAX8997_PMICIRQ_MBCHGTMEXPD,

    MAX8997_PMICIRQ_RTC60S,
    MAX8997_PMICIRQ_RTCA1,
    MAX8997_PMICIRQ_RTCA2,
    MAX8997_PMICIRQ_SMPL_INT,
    MAX8997_PMICIRQ_RTC1S,
    MAX8997_PMICIRQ_WTSR,

    MAX8997_MUICIRQ_ADCError,
    MAX8997_MUICIRQ_ADCLow,
    MAX8997_MUICIRQ_ADC,

    MAX8997_MUICIRQ_VBVolt,
    MAX8997_MUICIRQ_DBChg,
    MAX8997_MUICIRQ_DCDTmr,
    MAX8997_MUICIRQ_ChgDetRun,
    MAX8997_MUICIRQ_ChgTyp,

    MAX8997_MUICIRQ_OVP,

    MAX8997_IRQ_NR,
}

pub const MAX8997_NUM_GPIO: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8997_dev {
    pub dev: *mut device,
    pub pdata: *mut max8997_platform_data,
    pub /: *mut *mut *mut i2c_client i2c; / 0xcc / PMIC, Battery Control, and FLASH,
    pub /: *mut *mut *mut i2c_client rtc; / slave addr 0x0c,
    pub /: *mut *mut *mut i2c_client haptic; / slave addr 0x90,
    pub /: *mut *mut *mut i2c_client muic; / slave addr 0x4a,
    pub iolock: mutex,
    pub type: c_ulong,
    pub /: *mut *mut *mut platform_device battery; / battery control (not fuel gauge),
    pub irq: c_int,
    pub ono: c_int,
    pub irq_domain: *mut irq_domain,
    pub irqlock: mutex,
    pub irq_masks_cur: [c_int; MAX8997_IRQ_GROUP_NR],
    pub irq_masks_cache: [c_int; MAX8997_IRQ_GROUP_NR],
// For hibernation
    pub gpio_status: [bool; MAX8997_NUM_GPIO],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max8997_types {
    TYPE_MAX8997,
    TYPE_MAX8966,
}

extern "C" {
    pub fn max8997_irq_init(max8997: *mut max8997_dev) -> c_int;
}
extern "C" {
    pub fn max8997_irq_resume(max8997: *mut max8997_dev) -> c_int;
}
extern "C" {
    pub fn max8997_read_reg(i2c: *mut i2c_client, reg: u8, dest: *mut u8) -> c_int;
}
extern "C" {
    pub fn max8997_write_reg(i2c: *mut i2c_client, reg: u8, value: u8) -> c_int;
}
extern "C" {
    pub fn max8997_update_reg(i2c: *mut i2c_client, reg: u8, val: u8, mask: u8) -> c_int;
}

