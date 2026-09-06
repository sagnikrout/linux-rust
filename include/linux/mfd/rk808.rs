//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rk808.h
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
// Register definitions for Rockchip's RK808/RK818 PMIC
//
// Copyright (c) 2014, Fuzhou Rockchip Electronics Co., Ltd
//
// Author: Chris Zhong <zyw@rock-chips.com>
// Author: Zhang Qing <zhangqing@rock-chips.com>
//
// Copyright (C) 2016 PHYTEC Messtechnik GmbH
//
// Author: Wadim Egorov <w.egorov@phytec.de>
//

//
// rk808 Global Register Map.
//

pub const RK808_NUM_REGULATORS: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk808_reg {
    RK808_ID_DCDC1,
    RK808_ID_DCDC2,
    RK808_ID_DCDC3,
    RK808_ID_DCDC4,
    RK808_ID_LDO1,
    RK808_ID_LDO2,
    RK808_ID_LDO3,
    RK808_ID_LDO4,
    RK808_ID_LDO5,
    RK808_ID_LDO6,
    RK808_ID_LDO7,
    RK808_ID_LDO8,
    RK808_ID_SWITCH1,
    RK808_ID_SWITCH2,
}

pub const RK808_SECONDS_REG: c_uint = 0x00;
pub const RK808_MINUTES_REG: c_uint = 0x01;
pub const RK808_HOURS_REG: c_uint = 0x02;
pub const RK808_DAYS_REG: c_uint = 0x03;
pub const RK808_MONTHS_REG: c_uint = 0x04;
pub const RK808_YEARS_REG: c_uint = 0x05;
pub const RK808_WEEKS_REG: c_uint = 0x06;
pub const RK808_ALARM_SECONDS_REG: c_uint = 0x08;
pub const RK808_ALARM_MINUTES_REG: c_uint = 0x09;
pub const RK808_ALARM_HOURS_REG: c_uint = 0x0a;
pub const RK808_ALARM_DAYS_REG: c_uint = 0x0b;
pub const RK808_ALARM_MONTHS_REG: c_uint = 0x0c;
pub const RK808_ALARM_YEARS_REG: c_uint = 0x0d;
pub const RK808_RTC_CTRL_REG: c_uint = 0x10;
pub const RK808_RTC_STATUS_REG: c_uint = 0x11;
pub const RK808_RTC_INT_REG: c_uint = 0x12;
pub const RK808_RTC_COMP_LSB_REG: c_uint = 0x13;
pub const RK808_RTC_COMP_MSB_REG: c_uint = 0x14;
pub const RK808_ID_MSB: c_uint = 0x17;
pub const RK808_ID_LSB: c_uint = 0x18;
pub const RK808_CLK32OUT_REG: c_uint = 0x20;
pub const RK808_VB_MON_REG: c_uint = 0x21;
pub const RK808_THERMAL_REG: c_uint = 0x22;
pub const RK808_DCDC_EN_REG: c_uint = 0x23;
pub const RK808_LDO_EN_REG: c_uint = 0x24;
pub const RK808_SLEEP_SET_OFF_REG1: c_uint = 0x25;
pub const RK808_SLEEP_SET_OFF_REG2: c_uint = 0x26;
pub const RK808_DCDC_UV_STS_REG: c_uint = 0x27;
pub const RK808_DCDC_UV_ACT_REG: c_uint = 0x28;
pub const RK808_LDO_UV_STS_REG: c_uint = 0x29;
pub const RK808_LDO_UV_ACT_REG: c_uint = 0x2a;
pub const RK808_DCDC_PG_REG: c_uint = 0x2b;
pub const RK808_LDO_PG_REG: c_uint = 0x2c;
pub const RK808_VOUT_MON_TDB_REG: c_uint = 0x2d;
pub const RK808_BUCK1_CONFIG_REG: c_uint = 0x2e;
pub const RK808_BUCK1_ON_VSEL_REG: c_uint = 0x2f;
pub const RK808_BUCK1_SLP_VSEL_REG: c_uint = 0x30;
pub const RK808_BUCK1_DVS_VSEL_REG: c_uint = 0x31;
pub const RK808_BUCK2_CONFIG_REG: c_uint = 0x32;
pub const RK808_BUCK2_ON_VSEL_REG: c_uint = 0x33;
pub const RK808_BUCK2_SLP_VSEL_REG: c_uint = 0x34;
pub const RK808_BUCK2_DVS_VSEL_REG: c_uint = 0x35;
pub const RK808_BUCK3_CONFIG_REG: c_uint = 0x36;
pub const RK808_BUCK4_CONFIG_REG: c_uint = 0x37;
pub const RK808_BUCK4_ON_VSEL_REG: c_uint = 0x38;
pub const RK808_BUCK4_SLP_VSEL_REG: c_uint = 0x39;
pub const RK808_BOOST_CONFIG_REG: c_uint = 0x3a;
pub const RK808_LDO1_ON_VSEL_REG: c_uint = 0x3b;
pub const RK808_LDO1_SLP_VSEL_REG: c_uint = 0x3c;
pub const RK808_LDO2_ON_VSEL_REG: c_uint = 0x3d;
pub const RK808_LDO2_SLP_VSEL_REG: c_uint = 0x3e;
pub const RK808_LDO3_ON_VSEL_REG: c_uint = 0x3f;
pub const RK808_LDO3_SLP_VSEL_REG: c_uint = 0x40;
pub const RK808_LDO4_ON_VSEL_REG: c_uint = 0x41;
pub const RK808_LDO4_SLP_VSEL_REG: c_uint = 0x42;
pub const RK808_LDO5_ON_VSEL_REG: c_uint = 0x43;
pub const RK808_LDO5_SLP_VSEL_REG: c_uint = 0x44;
pub const RK808_LDO6_ON_VSEL_REG: c_uint = 0x45;
pub const RK808_LDO6_SLP_VSEL_REG: c_uint = 0x46;
pub const RK808_LDO7_ON_VSEL_REG: c_uint = 0x47;
pub const RK808_LDO7_SLP_VSEL_REG: c_uint = 0x48;
pub const RK808_LDO8_ON_VSEL_REG: c_uint = 0x49;
pub const RK808_LDO8_SLP_VSEL_REG: c_uint = 0x4a;
pub const RK808_DEVCTRL_REG: c_uint = 0x4b;
pub const RK808_INT_STS_REG1: c_uint = 0x4c;
pub const RK808_INT_STS_MSK_REG1: c_uint = 0x4d;
pub const RK808_INT_STS_REG2: c_uint = 0x4e;
pub const RK808_INT_STS_MSK_REG2: c_uint = 0x4f;
pub const RK808_IO_POL_REG: c_uint = 0x50;
// RK816
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk816_reg {
    RK816_ID_DCDC1,
    RK816_ID_DCDC2,
    RK816_ID_DCDC3,
    RK816_ID_DCDC4,
    RK816_ID_LDO1,
    RK816_ID_LDO2,
    RK816_ID_LDO3,
    RK816_ID_LDO4,
    RK816_ID_LDO5,
    RK816_ID_LDO6,
    RK816_ID_BOOST,
    RK816_ID_OTG_SW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk816_irqs {
// INT_STS_REG1
    RK816_IRQ_PWRON_FALL,
    RK816_IRQ_PWRON_RISE,

// INT_STS_REG2
    RK816_IRQ_VB_LOW,
    RK816_IRQ_PWRON,
    RK816_IRQ_PWRON_LP,
    RK816_IRQ_HOTDIE,
    RK816_IRQ_RTC_ALARM,
    RK816_IRQ_RTC_PERIOD,
    RK816_IRQ_USB_OV,

// INT_STS_REG3
    RK816_IRQ_PLUG_IN,
    RK816_IRQ_PLUG_OUT,
    RK816_IRQ_CHG_OK,
    RK816_IRQ_CHG_TE,
    RK816_IRQ_CHG_TS,
    RK816_IRQ_CHG_CVTLIM,
    RK816_IRQ_DISCHG_ILIM,
}

// power channel registers
pub const RK816_DCDC_EN_REG1: c_uint = 0x23;
pub const RK816_DCDC_EN_REG2: c_uint = 0x24;

pub const RK816_LDO_EN_REG1: c_uint = 0x27;
pub const RK816_LDO_EN_REG2: c_uint = 0x28;
// interrupt registers and irq definitions
pub const RK816_INT_STS_REG1: c_uint = 0x49;
pub const RK816_INT_STS_MSK_REG1: c_uint = 0x4a;

pub const RK816_INT_STS_REG2: c_uint = 0x4c;
pub const RK816_INT_STS_MSK_REG2: c_uint = 0x4d;

pub const RK816_INT_STS_REG3: c_uint = 0x4e;
pub const RK816_INT_STS_MSK_REG3: c_uint = 0x4f;

// charger, boost and OTG registers
pub const RK816_OTG_BUCK_LDO_CONFIG_REG: c_uint = 0x2a;
pub const RK816_CHRG_CONFIG_REG: c_uint = 0x2b;
pub const RK816_BOOST_ON_VESL_REG: c_uint = 0x54;
pub const RK816_BOOST_SLP_VSEL_REG: c_uint = 0x55;
pub const RK816_CHRG_BOOST_CONFIG_REG: c_uint = 0x9a;
pub const RK816_SUP_STS_REG: c_uint = 0xa0;
pub const RK816_USB_CTRL_REG: c_uint = 0xa1;

pub const RK816_BAT_CTRL_REG: c_uint = 0xa6;
pub const RK816_BAT_HTS_TS_REG: c_uint = 0xa8;
pub const RK816_BAT_LTS_TS_REG: c_uint = 0xa9;
// adc and fuel gauge registers
pub const RK816_TS_CTRL_REG: c_uint = 0xac;
pub const RK816_ADC_CTRL_REG: c_uint = 0xad;
pub const RK816_GGCON_REG: c_uint = 0xb0;
pub const RK816_GGSTS_REG: c_uint = 0xb1;
pub const RK816_ZERO_CUR_ADC_REGH: c_uint = 0xb2;
pub const RK816_ZERO_CUR_ADC_REGL: c_uint = 0xb3;

pub const RK816_BAT_CUR_AVG_REGH: c_uint = 0xbc;
pub const RK816_BAT_CUR_AVG_REGL: c_uint = 0xbd;
pub const RK816_TS_ADC_REGH: c_uint = 0xbe;
pub const RK816_TS_ADC_REGL: c_uint = 0xbf;
pub const RK816_USB_ADC_REGH: c_uint = 0xc0;
pub const RK816_USB_ADC_REGL: c_uint = 0xc1;
pub const RK816_BAT_OCV_REGH: c_uint = 0xc2;
pub const RK816_BAT_OCV_REGL: c_uint = 0xc3;
pub const RK816_BAT_VOL_REGH: c_uint = 0xc4;
pub const RK816_BAT_VOL_REGL: c_uint = 0xc5;
pub const RK816_RELAX_ENTRY_THRES_REGH: c_uint = 0xc6;
pub const RK816_RELAX_ENTRY_THRES_REGL: c_uint = 0xc7;
pub const RK816_RELAX_EXIT_THRES_REGH: c_uint = 0xc8;
pub const RK816_RELAX_EXIT_THRES_REGL: c_uint = 0xc9;
pub const RK816_RELAX_VOL1_REGH: c_uint = 0xca;
pub const RK816_RELAX_VOL1_REGL: c_uint = 0xcb;
pub const RK816_RELAX_VOL2_REGH: c_uint = 0xcc;
pub const RK816_RELAX_VOL2_REGL: c_uint = 0xcd;
pub const RK816_RELAX_CUR1_REGH: c_uint = 0xce;
pub const RK816_RELAX_CUR1_REGL: c_uint = 0xcf;
pub const RK816_RELAX_CUR2_REGH: c_uint = 0xd0;
pub const RK816_RELAX_CUR2_REGL: c_uint = 0xd1;
pub const RK816_CAL_OFFSET_REGH: c_uint = 0xd2;
pub const RK816_CAL_OFFSET_REGL: c_uint = 0xd3;
pub const RK816_NON_ACT_TIMER_CNT_REG: c_uint = 0xd4;
pub const RK816_VCALIB0_REGH: c_uint = 0xd5;
pub const RK816_VCALIB0_REGL: c_uint = 0xd6;
pub const RK816_VCALIB1_REGH: c_uint = 0xd7;
pub const RK816_VCALIB1_REGL: c_uint = 0xd8;

pub const RK816_IOFFSET_REGH: c_uint = 0xdd;
pub const RK816_IOFFSET_REGL: c_uint = 0xde;
pub const RK816_SLEEP_CON_SAMP_CUR_REG: c_uint = 0xdf;
// general purpose data registers 0xe0 ~ 0xf2

// RK818
pub const RK818_DCDC1: c_int = 0;
pub const RK818_LDO1: c_int = 4;
pub const RK818_NUM_REGULATORS: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk818_reg {
    RK818_ID_DCDC1,
    RK818_ID_DCDC2,
    RK818_ID_DCDC3,
    RK818_ID_DCDC4,
    RK818_ID_BOOST,
    RK818_ID_LDO1,
    RK818_ID_LDO2,
    RK818_ID_LDO3,
    RK818_ID_LDO4,
    RK818_ID_LDO5,
    RK818_ID_LDO6,
    RK818_ID_LDO7,
    RK818_ID_LDO8,
    RK818_ID_LDO9,
    RK818_ID_SWITCH,
    RK818_ID_HDMI_SWITCH,
    RK818_ID_OTG_SWITCH,
}

pub const RK818_DCDC_EN_REG: c_uint = 0x23;
pub const RK818_LDO_EN_REG: c_uint = 0x24;
pub const RK818_SLEEP_SET_OFF_REG1: c_uint = 0x25;
pub const RK818_SLEEP_SET_OFF_REG2: c_uint = 0x26;
pub const RK818_DCDC_UV_STS_REG: c_uint = 0x27;
pub const RK818_DCDC_UV_ACT_REG: c_uint = 0x28;
pub const RK818_LDO_UV_STS_REG: c_uint = 0x29;
pub const RK818_LDO_UV_ACT_REG: c_uint = 0x2a;
pub const RK818_DCDC_PG_REG: c_uint = 0x2b;
pub const RK818_LDO_PG_REG: c_uint = 0x2c;
pub const RK818_VOUT_MON_TDB_REG: c_uint = 0x2d;
pub const RK818_BUCK1_CONFIG_REG: c_uint = 0x2e;
pub const RK818_BUCK1_ON_VSEL_REG: c_uint = 0x2f;
pub const RK818_BUCK1_SLP_VSEL_REG: c_uint = 0x30;
pub const RK818_BUCK2_CONFIG_REG: c_uint = 0x32;
pub const RK818_BUCK2_ON_VSEL_REG: c_uint = 0x33;
pub const RK818_BUCK2_SLP_VSEL_REG: c_uint = 0x34;
pub const RK818_BUCK3_CONFIG_REG: c_uint = 0x36;
pub const RK818_BUCK4_CONFIG_REG: c_uint = 0x37;
pub const RK818_BUCK4_ON_VSEL_REG: c_uint = 0x38;
pub const RK818_BUCK4_SLP_VSEL_REG: c_uint = 0x39;
pub const RK818_BOOST_CONFIG_REG: c_uint = 0x3a;
pub const RK818_LDO1_ON_VSEL_REG: c_uint = 0x3b;
pub const RK818_LDO1_SLP_VSEL_REG: c_uint = 0x3c;
pub const RK818_LDO2_ON_VSEL_REG: c_uint = 0x3d;
pub const RK818_LDO2_SLP_VSEL_REG: c_uint = 0x3e;
pub const RK818_LDO3_ON_VSEL_REG: c_uint = 0x3f;
pub const RK818_LDO3_SLP_VSEL_REG: c_uint = 0x40;
pub const RK818_LDO4_ON_VSEL_REG: c_uint = 0x41;
pub const RK818_LDO4_SLP_VSEL_REG: c_uint = 0x42;
pub const RK818_LDO5_ON_VSEL_REG: c_uint = 0x43;
pub const RK818_LDO5_SLP_VSEL_REG: c_uint = 0x44;
pub const RK818_LDO6_ON_VSEL_REG: c_uint = 0x45;
pub const RK818_LDO6_SLP_VSEL_REG: c_uint = 0x46;
pub const RK818_LDO7_ON_VSEL_REG: c_uint = 0x47;
pub const RK818_LDO7_SLP_VSEL_REG: c_uint = 0x48;
pub const RK818_LDO8_ON_VSEL_REG: c_uint = 0x49;
pub const RK818_LDO8_SLP_VSEL_REG: c_uint = 0x4a;
pub const RK818_BOOST_LDO9_ON_VSEL_REG: c_uint = 0x54;
pub const RK818_BOOST_LDO9_SLP_VSEL_REG: c_uint = 0x55;
pub const RK818_DEVCTRL_REG: c_uint = 0x4b;

pub const RK818_INT_STS_MSK_REG1: c_uint = 0x4d;
pub const RK818_INT_STS_REG2: c_uint = 0x4e;
pub const RK818_INT_STS_MSK_REG2: c_uint = 0x4f;
pub const RK818_IO_POL_REG: c_uint = 0x50;
pub const RK818_H5V_EN_REG: c_uint = 0x52;
pub const RK818_SLEEP_SET_OFF_REG3: c_uint = 0x53;
pub const RK818_BOOST_LDO9_ON_VSEL_REG: c_uint = 0x54;
pub const RK818_BOOST_LDO9_SLP_VSEL_REG: c_uint = 0x55;
pub const RK818_BOOST_CTRL_REG: c_uint = 0x56;
pub const RK818_DCDC_ILMAX: c_uint = 0x90;
pub const RK818_USB_CTRL_REG: c_uint = 0xa1;

pub const RK818_USB_ILIM_SEL_MASK: c_uint = 0xf;
pub const RK818_USB_ILMIN_2000MA: c_uint = 0x7;
pub const RK818_USB_CHG_SD_VSEL_MASK: c_uint = 0x70;
// RK801
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk801_reg {
    RK801_ID_DCDC1,
    RK801_ID_DCDC2,
    RK801_ID_DCDC4,
    RK801_ID_DCDC3,
    RK801_ID_LDO1,
    RK801_ID_LDO2,
    RK801_ID_SWITCH,
    RK801_ID_MAX,
}

pub const RK801_SLP_REG_OFFSET: c_int = 5;
pub const RK801_NUM_REGULATORS: c_int = 7;
pub const RK801_HW_SYNC_US: c_int = 32;
// RK801 Register Definitions
pub const RK801_ID_MSB: c_uint = 0x00;
pub const RK801_ID_LSB: c_uint = 0x01;
pub const RK801_OTP_VER_REG: c_uint = 0x02;
pub const RK801_POWER_EN0_REG: c_uint = 0x03;
pub const RK801_POWER_EN1_REG: c_uint = 0x04;
pub const RK801_POWER_SLP_EN_REG: c_uint = 0x05;
pub const RK801_POWER_FPWM_EN_REG: c_uint = 0x06;
pub const RK801_SLP_LP_CONFIG_REG: c_uint = 0x07;
pub const RK801_BUCK_CONFIG_REG: c_uint = 0x08;
pub const RK801_BUCK1_ON_VSEL_REG: c_uint = 0x09;
pub const RK801_BUCK2_ON_VSEL_REG: c_uint = 0x0a;
pub const RK801_BUCK4_ON_VSEL_REG: c_uint = 0x0b;
pub const RK801_LDO1_ON_VSEL_REG: c_uint = 0x0c;
pub const RK801_LDO2_ON_VSEL_REG: c_uint = 0x0d;
pub const RK801_BUCK1_SLP_VSEL_REG: c_uint = 0x0e;
pub const RK801_BUCK2_SLP_VSEL_REG: c_uint = 0x0f;
pub const RK801_BUCK4_SLP_VSEL_REG: c_uint = 0x10;
pub const RK801_LDO1_SLP_VSEL_REG: c_uint = 0x11;
pub const RK801_LDO2_SLP_VSEL_REG: c_uint = 0x12;
pub const RK801_LDO_SW_IMAX_REG: c_uint = 0x13;
pub const RK801_SYS_STS_REG: c_uint = 0x14;
pub const RK801_SYS_CFG0_REG: c_uint = 0x15;
pub const RK801_SYS_CFG1_REG: c_uint = 0x16;
pub const RK801_SYS_CFG2_REG: c_uint = 0x17;
pub const RK801_SYS_CFG3_REG: c_uint = 0x18;
pub const RK801_SYS_CFG4_REG: c_uint = 0x19;
pub const RK801_SLEEP_CFG_REG: c_uint = 0x1a;
pub const RK801_ON_SOURCE_REG: c_uint = 0x1b;
pub const RK801_OFF_SOURCE_REG: c_uint = 0x1c;
pub const RK801_PWRON_KEY_REG: c_uint = 0x1d;
pub const RK801_INT_STS0_REG: c_uint = 0x1e;
pub const RK801_INT_MASK0_REG: c_uint = 0x1f;
pub const RK801_INT_CONFIG_REG: c_uint = 0x20;
pub const RK801_CON_BACK1_REG: c_uint = 0x21;
pub const RK801_CON_BACK2_REG: c_uint = 0x22;
pub const RK801_DATA_CON0_REG: c_uint = 0x23;
pub const RK801_DATA_CON1_REG: c_uint = 0x24;
pub const RK801_DATA_CON2_REG: c_uint = 0x25;
pub const RK801_DATA_CON3_REG: c_uint = 0x26;
pub const RK801_POWER_EXIT_SLP_SEQ0_REG: c_uint = 0x27;
pub const RK801_POWER_EXIT_SLP_SEQ1_REG: c_uint = 0x28;
pub const RK801_POWER_EXIT_SLP_SEQ2_REG: c_uint = 0x29;
pub const RK801_POWER_EXIT_SLP_SEQ3_REG: c_uint = 0x2a;
pub const RK801_POWER_ENTER_SLP_OR_SHTD_SEQ0_REG: c_uint = 0x2b;
pub const RK801_POWER_ENTER_SLP_OR_SHTD_SEQ1_REG: c_uint = 0x2c;
pub const RK801_POWER_ENTER_SLP_OR_SHTD_SEQ2_REG: c_uint = 0x2d;
pub const RK801_POWER_ENTER_SLP_OR_SHTD_SEQ3_REG: c_uint = 0x2e;
pub const RK801_BUCK_DEBUG1_REG: c_uint = 0x2f;
pub const RK801_BUCK_DEBUG2_REG: c_uint = 0x30;
pub const RK801_BUCK_DEBUG3_REG: c_uint = 0x31;
pub const RK801_BUCK_DEBUG4_REG: c_uint = 0x32;
pub const RK801_BUCK_DEBUG5_REG: c_uint = 0x33;
pub const RK801_BUCK_DEBUG7_REG: c_uint = 0x34;
pub const RK801_OTP_EN_CON_REG: c_uint = 0x35;
pub const RK801_TEST_CON_REG: c_uint = 0x36;
pub const RK801_EFUSE_CONTROL_REG: c_uint = 0x37;
pub const RK801_SYS_CFG3_OTP_REG: c_uint = 0x38;
// RK801 IRQ Definitions
pub const RK801_IRQ_PWRON_FALL: c_int = 0;
pub const RK801_IRQ_PWRON_RISE: c_int = 1;
pub const RK801_IRQ_PWRON: c_int = 2;
pub const RK801_IRQ_PWRON_LP: c_int = 3;
pub const RK801_IRQ_HOTDIE: c_int = 4;
pub const RK801_IRQ_VDC_RISE: c_int = 5;
pub const RK801_IRQ_VDC_FALL: c_int = 6;

// RK801_SLP_LP_CONFIG_REG

// RK801_SLEEP_CFG_REG
pub const RK801_SLEEP_FUN_MSK: c_uint = 0x3;
pub const RK801_NONE_FUN: c_uint = 0x0;
pub const RK801_SLEEP_FUN: c_uint = 0x1;
pub const RK801_SHUTDOWN_FUN: c_uint = 0x2;
pub const RK801_RESET_FUN: c_uint = 0x3;
// RK801_SYS_CFG2_REG

pub const RK801_SLEEP_ACT_L: c_int = 0;

// RK801_INT_CONFIG_REG

pub const RK801_INT_ACT_L: c_int = 0;
pub const RK801_FPWM_MODE: c_int = 1;
pub const RK801_AUTO_PWM_MODE: c_int = 0;

// RK805
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk805_reg {
    RK805_ID_DCDC1,
    RK805_ID_DCDC2,
    RK805_ID_DCDC3,
    RK805_ID_DCDC4,
    RK805_ID_LDO1,
    RK805_ID_LDO2,
    RK805_ID_LDO3,
}

// CONFIG REGISTER
pub const RK805_VB_MON_REG: c_uint = 0x21;
pub const RK805_THERMAL_REG: c_uint = 0x22;
// POWER CHANNELS ENABLE REGISTER
pub const RK805_DCDC_EN_REG: c_uint = 0x23;
pub const RK805_SLP_DCDC_EN_REG: c_uint = 0x25;
pub const RK805_SLP_LDO_EN_REG: c_uint = 0x26;
pub const RK805_LDO_EN_REG: c_uint = 0x27;
// BUCK AND LDO CONFIG REGISTER
pub const RK805_BUCK_LDO_SLP_LP_EN_REG: c_uint = 0x2A;
pub const RK805_BUCK1_CONFIG_REG: c_uint = 0x2E;
pub const RK805_BUCK1_ON_VSEL_REG: c_uint = 0x2F;
pub const RK805_BUCK1_SLP_VSEL_REG: c_uint = 0x30;
pub const RK805_BUCK2_CONFIG_REG: c_uint = 0x32;
pub const RK805_BUCK2_ON_VSEL_REG: c_uint = 0x33;
pub const RK805_BUCK2_SLP_VSEL_REG: c_uint = 0x34;
pub const RK805_BUCK3_CONFIG_REG: c_uint = 0x36;
pub const RK805_BUCK4_CONFIG_REG: c_uint = 0x37;
pub const RK805_BUCK4_ON_VSEL_REG: c_uint = 0x38;
pub const RK805_BUCK4_SLP_VSEL_REG: c_uint = 0x39;
pub const RK805_LDO1_ON_VSEL_REG: c_uint = 0x3B;
pub const RK805_LDO1_SLP_VSEL_REG: c_uint = 0x3C;
pub const RK805_LDO2_ON_VSEL_REG: c_uint = 0x3D;
pub const RK805_LDO2_SLP_VSEL_REG: c_uint = 0x3E;
pub const RK805_LDO3_ON_VSEL_REG: c_uint = 0x3F;
pub const RK805_LDO3_SLP_VSEL_REG: c_uint = 0x40;
// INTERRUPT REGISTER
pub const RK805_PWRON_LP_INT_TIME_REG: c_uint = 0x47;
pub const RK805_PWRON_DB_REG: c_uint = 0x48;
pub const RK805_DEV_CTRL_REG: c_uint = 0x4B;
pub const RK805_INT_STS_REG: c_uint = 0x4C;
pub const RK805_INT_STS_MSK_REG: c_uint = 0x4D;
pub const RK805_GPIO_IO_POL_REG: c_uint = 0x50;
pub const RK805_OUT_REG: c_uint = 0x52;
pub const RK805_ON_SOURCE_REG: c_uint = 0xAE;
pub const RK805_OFF_SOURCE_REG: c_uint = 0xAF;
pub const RK805_NUM_REGULATORS: c_int = 7;
pub const RK805_PWRON_FALL_RISE_INT_EN: c_uint = 0x0;
pub const RK805_PWRON_FALL_RISE_INT_MSK: c_uint = 0x81;
// RK805 IRQ Definitions
pub const RK805_IRQ_PWRON_RISE: c_int = 0;
pub const RK805_IRQ_VB_LOW: c_int = 1;
pub const RK805_IRQ_PWRON: c_int = 2;
pub const RK805_IRQ_PWRON_LP: c_int = 3;
pub const RK805_IRQ_HOTDIE: c_int = 4;
pub const RK805_IRQ_RTC_ALARM: c_int = 5;
pub const RK805_IRQ_RTC_PERIOD: c_int = 6;
pub const RK805_IRQ_PWRON_FALL: c_int = 7;

// RK806
pub const RK806_POWER_EN0: c_uint = 0x0;
pub const RK806_POWER_EN1: c_uint = 0x1;
pub const RK806_POWER_EN2: c_uint = 0x2;
pub const RK806_POWER_EN3: c_uint = 0x3;
pub const RK806_POWER_EN4: c_uint = 0x4;
pub const RK806_POWER_EN5: c_uint = 0x5;
pub const RK806_POWER_SLP_EN0: c_uint = 0x6;
pub const RK806_POWER_SLP_EN1: c_uint = 0x7;
pub const RK806_POWER_SLP_EN2: c_uint = 0x8;
pub const RK806_POWER_DISCHRG_EN0: c_uint = 0x9;
pub const RK806_POWER_DISCHRG_EN1: c_uint = 0xA;
pub const RK806_POWER_DISCHRG_EN2: c_uint = 0xB;
pub const RK806_BUCK_FB_CONFIG: c_uint = 0xC;
pub const RK806_SLP_LP_CONFIG: c_uint = 0xD;
pub const RK806_POWER_FPWM_EN0: c_uint = 0xE;
pub const RK806_POWER_FPWM_EN1: c_uint = 0xF;
pub const RK806_BUCK1_CONFIG: c_uint = 0x10;
pub const RK806_BUCK2_CONFIG: c_uint = 0x11;
pub const RK806_BUCK3_CONFIG: c_uint = 0x12;
pub const RK806_BUCK4_CONFIG: c_uint = 0x13;
pub const RK806_BUCK5_CONFIG: c_uint = 0x14;
pub const RK806_BUCK6_CONFIG: c_uint = 0x15;
pub const RK806_BUCK7_CONFIG: c_uint = 0x16;
pub const RK806_BUCK8_CONFIG: c_uint = 0x17;
pub const RK806_BUCK9_CONFIG: c_uint = 0x18;
pub const RK806_BUCK10_CONFIG: c_uint = 0x19;
pub const RK806_BUCK1_ON_VSEL: c_uint = 0x1A;
pub const RK806_BUCK2_ON_VSEL: c_uint = 0x1B;
pub const RK806_BUCK3_ON_VSEL: c_uint = 0x1C;
pub const RK806_BUCK4_ON_VSEL: c_uint = 0x1D;
pub const RK806_BUCK5_ON_VSEL: c_uint = 0x1E;
pub const RK806_BUCK6_ON_VSEL: c_uint = 0x1F;
pub const RK806_BUCK7_ON_VSEL: c_uint = 0x20;
pub const RK806_BUCK8_ON_VSEL: c_uint = 0x21;
pub const RK806_BUCK9_ON_VSEL: c_uint = 0x22;
pub const RK806_BUCK10_ON_VSEL: c_uint = 0x23;
pub const RK806_BUCK1_SLP_VSEL: c_uint = 0x24;
pub const RK806_BUCK2_SLP_VSEL: c_uint = 0x25;
pub const RK806_BUCK3_SLP_VSEL: c_uint = 0x26;
pub const RK806_BUCK4_SLP_VSEL: c_uint = 0x27;
pub const RK806_BUCK5_SLP_VSEL: c_uint = 0x28;
pub const RK806_BUCK6_SLP_VSEL: c_uint = 0x29;
pub const RK806_BUCK7_SLP_VSEL: c_uint = 0x2A;
pub const RK806_BUCK8_SLP_VSEL: c_uint = 0x2B;
pub const RK806_BUCK9_SLP_VSEL: c_uint = 0x2D;
pub const RK806_BUCK10_SLP_VSEL: c_uint = 0x2E;
pub const RK806_BUCK_DEBUG1: c_uint = 0x30;
pub const RK806_BUCK_DEBUG2: c_uint = 0x31;
pub const RK806_BUCK_DEBUG3: c_uint = 0x32;
pub const RK806_BUCK_DEBUG4: c_uint = 0x33;
pub const RK806_BUCK_DEBUG5: c_uint = 0x34;
pub const RK806_BUCK_DEBUG6: c_uint = 0x35;
pub const RK806_BUCK_DEBUG7: c_uint = 0x36;
pub const RK806_BUCK_DEBUG8: c_uint = 0x37;
pub const RK806_BUCK_DEBUG9: c_uint = 0x38;
pub const RK806_BUCK_DEBUG10: c_uint = 0x39;
pub const RK806_BUCK_DEBUG11: c_uint = 0x3A;
pub const RK806_BUCK_DEBUG12: c_uint = 0x3B;
pub const RK806_BUCK_DEBUG13: c_uint = 0x3C;
pub const RK806_BUCK_DEBUG14: c_uint = 0x3D;
pub const RK806_BUCK_DEBUG15: c_uint = 0x3E;
pub const RK806_BUCK_DEBUG16: c_uint = 0x3F;
pub const RK806_BUCK_DEBUG17: c_uint = 0x40;
pub const RK806_BUCK_DEBUG18: c_uint = 0x41;
pub const RK806_NLDO_IMAX: c_uint = 0x42;
pub const RK806_NLDO1_ON_VSEL: c_uint = 0x43;
pub const RK806_NLDO2_ON_VSEL: c_uint = 0x44;
pub const RK806_NLDO3_ON_VSEL: c_uint = 0x45;
pub const RK806_NLDO4_ON_VSEL: c_uint = 0x46;
pub const RK806_NLDO5_ON_VSEL: c_uint = 0x47;
pub const RK806_NLDO1_SLP_VSEL: c_uint = 0x48;
pub const RK806_NLDO2_SLP_VSEL: c_uint = 0x49;
pub const RK806_NLDO3_SLP_VSEL: c_uint = 0x4A;
pub const RK806_NLDO4_SLP_VSEL: c_uint = 0x4B;
pub const RK806_NLDO5_SLP_VSEL: c_uint = 0x4C;
pub const RK806_PLDO_IMAX: c_uint = 0x4D;
pub const RK806_PLDO1_ON_VSEL: c_uint = 0x4E;
pub const RK806_PLDO2_ON_VSEL: c_uint = 0x4F;
pub const RK806_PLDO3_ON_VSEL: c_uint = 0x50;
pub const RK806_PLDO4_ON_VSEL: c_uint = 0x51;
pub const RK806_PLDO5_ON_VSEL: c_uint = 0x52;
pub const RK806_PLDO6_ON_VSEL: c_uint = 0x53;
pub const RK806_PLDO1_SLP_VSEL: c_uint = 0x54;
pub const RK806_PLDO2_SLP_VSEL: c_uint = 0x55;
pub const RK806_PLDO3_SLP_VSEL: c_uint = 0x56;
pub const RK806_PLDO4_SLP_VSEL: c_uint = 0x57;
pub const RK806_PLDO5_SLP_VSEL: c_uint = 0x58;
pub const RK806_PLDO6_SLP_VSEL: c_uint = 0x59;
pub const RK806_CHIP_NAME: c_uint = 0x5A;
pub const RK806_CHIP_VER: c_uint = 0x5B;
pub const RK806_OTP_VER: c_uint = 0x5C;
pub const RK806_SYS_STS: c_uint = 0x5D;
pub const RK806_SYS_CFG0: c_uint = 0x5E;
pub const RK806_SYS_CFG1: c_uint = 0x5F;
pub const RK806_SYS_OPTION: c_uint = 0x61;
pub const RK806_SLEEP_CONFIG0: c_uint = 0x62;
pub const RK806_SLEEP_CONFIG1: c_uint = 0x63;
pub const RK806_SLEEP_CTR_SEL0: c_uint = 0x64;
pub const RK806_SLEEP_CTR_SEL1: c_uint = 0x65;
pub const RK806_SLEEP_CTR_SEL2: c_uint = 0x66;
pub const RK806_SLEEP_CTR_SEL3: c_uint = 0x67;
pub const RK806_SLEEP_CTR_SEL4: c_uint = 0x68;
pub const RK806_SLEEP_CTR_SEL5: c_uint = 0x69;
pub const RK806_DVS_CTRL_SEL0: c_uint = 0x6A;
pub const RK806_DVS_CTRL_SEL1: c_uint = 0x6B;
pub const RK806_DVS_CTRL_SEL2: c_uint = 0x6C;
pub const RK806_DVS_CTRL_SEL3: c_uint = 0x6D;
pub const RK806_DVS_CTRL_SEL4: c_uint = 0x6E;
pub const RK806_DVS_CTRL_SEL5: c_uint = 0x6F;
pub const RK806_DVS_START_CTRL: c_uint = 0x70;
pub const RK806_SLEEP_GPIO: c_uint = 0x71;
pub const RK806_SYS_CFG3: c_uint = 0x72;
pub const RK806_ON_SOURCE: c_uint = 0x74;
pub const RK806_OFF_SOURCE: c_uint = 0x75;
pub const RK806_PWRON_KEY: c_uint = 0x76;
pub const RK806_INT_STS0: c_uint = 0x77;
pub const RK806_INT_MSK0: c_uint = 0x78;
pub const RK806_INT_STS1: c_uint = 0x79;
pub const RK806_INT_MSK1: c_uint = 0x7A;
pub const RK806_GPIO_INT_CONFIG: c_uint = 0x7B;
pub const RK806_DATA_REG0: c_uint = 0x7C;
pub const RK806_DATA_REG1: c_uint = 0x7D;
pub const RK806_DATA_REG2: c_uint = 0x7E;
pub const RK806_DATA_REG3: c_uint = 0x7F;
pub const RK806_DATA_REG4: c_uint = 0x80;
pub const RK806_DATA_REG5: c_uint = 0x81;
pub const RK806_DATA_REG6: c_uint = 0x82;
pub const RK806_DATA_REG7: c_uint = 0x83;
pub const RK806_DATA_REG8: c_uint = 0x84;
pub const RK806_DATA_REG9: c_uint = 0x85;
pub const RK806_DATA_REG10: c_uint = 0x86;
pub const RK806_DATA_REG11: c_uint = 0x87;
pub const RK806_DATA_REG12: c_uint = 0x88;
pub const RK806_DATA_REG13: c_uint = 0x89;
pub const RK806_DATA_REG14: c_uint = 0x8A;
pub const RK806_DATA_REG15: c_uint = 0x8B;
pub const RK806_TM_REG: c_uint = 0x8C;
pub const RK806_OTP_EN_REG: c_uint = 0x8D;
pub const RK806_FUNC_OTP_EN_REG: c_uint = 0x8E;
pub const RK806_TEST_REG1: c_uint = 0x8F;
pub const RK806_TEST_REG2: c_uint = 0x90;
pub const RK806_TEST_REG3: c_uint = 0x91;
pub const RK806_TEST_REG4: c_uint = 0x92;
pub const RK806_TEST_REG5: c_uint = 0x93;
pub const RK806_BUCK_VSEL_OTP_REG0: c_uint = 0x94;
pub const RK806_BUCK_VSEL_OTP_REG1: c_uint = 0x95;
pub const RK806_BUCK_VSEL_OTP_REG2: c_uint = 0x96;
pub const RK806_BUCK_VSEL_OTP_REG3: c_uint = 0x97;
pub const RK806_BUCK_VSEL_OTP_REG4: c_uint = 0x98;
pub const RK806_BUCK_VSEL_OTP_REG5: c_uint = 0x99;
pub const RK806_BUCK_VSEL_OTP_REG6: c_uint = 0x9A;
pub const RK806_BUCK_VSEL_OTP_REG7: c_uint = 0x9B;
pub const RK806_BUCK_VSEL_OTP_REG8: c_uint = 0x9C;
pub const RK806_BUCK_VSEL_OTP_REG9: c_uint = 0x9D;
pub const RK806_NLDO1_VSEL_OTP_REG0: c_uint = 0x9E;
pub const RK806_NLDO1_VSEL_OTP_REG1: c_uint = 0x9F;
pub const RK806_NLDO1_VSEL_OTP_REG2: c_uint = 0xA0;
pub const RK806_NLDO1_VSEL_OTP_REG3: c_uint = 0xA1;
pub const RK806_NLDO1_VSEL_OTP_REG4: c_uint = 0xA2;
pub const RK806_PLDO_VSEL_OTP_REG0: c_uint = 0xA3;
pub const RK806_PLDO_VSEL_OTP_REG1: c_uint = 0xA4;
pub const RK806_PLDO_VSEL_OTP_REG2: c_uint = 0xA5;
pub const RK806_PLDO_VSEL_OTP_REG3: c_uint = 0xA6;
pub const RK806_PLDO_VSEL_OTP_REG4: c_uint = 0xA7;
pub const RK806_PLDO_VSEL_OTP_REG5: c_uint = 0xA8;
pub const RK806_BUCK_EN_OTP_REG1: c_uint = 0xA9;
pub const RK806_NLDO_EN_OTP_REG1: c_uint = 0xAA;
pub const RK806_PLDO_EN_OTP_REG1: c_uint = 0xAB;
pub const RK806_BUCK_FB_RES_OTP_REG1: c_uint = 0xAC;
pub const RK806_OTP_RESEV_REG0: c_uint = 0xAD;
pub const RK806_OTP_RESEV_REG1: c_uint = 0xAE;
pub const RK806_OTP_RESEV_REG2: c_uint = 0xAF;
pub const RK806_OTP_RESEV_REG3: c_uint = 0xB0;
pub const RK806_OTP_RESEV_REG4: c_uint = 0xB1;
pub const RK806_BUCK_SEQ_REG0: c_uint = 0xB2;
pub const RK806_BUCK_SEQ_REG1: c_uint = 0xB3;
pub const RK806_BUCK_SEQ_REG2: c_uint = 0xB4;
pub const RK806_BUCK_SEQ_REG3: c_uint = 0xB5;
pub const RK806_BUCK_SEQ_REG4: c_uint = 0xB6;
pub const RK806_BUCK_SEQ_REG5: c_uint = 0xB7;
pub const RK806_BUCK_SEQ_REG6: c_uint = 0xB8;
pub const RK806_BUCK_SEQ_REG7: c_uint = 0xB9;
pub const RK806_BUCK_SEQ_REG8: c_uint = 0xBA;
pub const RK806_BUCK_SEQ_REG9: c_uint = 0xBB;
pub const RK806_BUCK_SEQ_REG10: c_uint = 0xBC;
pub const RK806_BUCK_SEQ_REG11: c_uint = 0xBD;
pub const RK806_BUCK_SEQ_REG12: c_uint = 0xBE;
pub const RK806_BUCK_SEQ_REG13: c_uint = 0xBF;
pub const RK806_BUCK_SEQ_REG14: c_uint = 0xC0;
pub const RK806_BUCK_SEQ_REG15: c_uint = 0xC1;
pub const RK806_BUCK_SEQ_REG16: c_uint = 0xC2;
pub const RK806_BUCK_SEQ_REG17: c_uint = 0xC3;
pub const RK806_HK_TRIM_REG1: c_uint = 0xC4;
pub const RK806_HK_TRIM_REG2: c_uint = 0xC5;
pub const RK806_BUCK_REF_TRIM_REG1: c_uint = 0xC6;
pub const RK806_BUCK_REF_TRIM_REG2: c_uint = 0xC7;
pub const RK806_BUCK_REF_TRIM_REG3: c_uint = 0xC8;
pub const RK806_BUCK_REF_TRIM_REG4: c_uint = 0xC9;
pub const RK806_BUCK_REF_TRIM_REG5: c_uint = 0xCA;
pub const RK806_BUCK_OSC_TRIM_REG1: c_uint = 0xCB;
pub const RK806_BUCK_OSC_TRIM_REG2: c_uint = 0xCC;
pub const RK806_BUCK_OSC_TRIM_REG3: c_uint = 0xCD;
pub const RK806_BUCK_OSC_TRIM_REG4: c_uint = 0xCE;
pub const RK806_BUCK_OSC_TRIM_REG5: c_uint = 0xCF;
pub const RK806_BUCK_TRIM_ZCDIOS_REG1: c_uint = 0xD0;
pub const RK806_BUCK_TRIM_ZCDIOS_REG2: c_uint = 0xD1;
pub const RK806_NLDO_TRIM_REG1: c_uint = 0xD2;
pub const RK806_NLDO_TRIM_REG2: c_uint = 0xD3;
pub const RK806_NLDO_TRIM_REG3: c_uint = 0xD4;
pub const RK806_PLDO_TRIM_REG1: c_uint = 0xD5;
pub const RK806_PLDO_TRIM_REG2: c_uint = 0xD6;
pub const RK806_PLDO_TRIM_REG3: c_uint = 0xD7;
pub const RK806_TRIM_ICOMP_REG1: c_uint = 0xD8;
pub const RK806_TRIM_ICOMP_REG2: c_uint = 0xD9;
pub const RK806_EFUSE_CONTROL_REGH: c_uint = 0xDA;
pub const RK806_FUSE_PROG_REG: c_uint = 0xDB;
pub const RK806_MAIN_FSM_STS_REG: c_uint = 0xDD;
pub const RK806_FSM_REG: c_uint = 0xDE;
pub const RK806_TOP_RESEV_OFFR: c_uint = 0xEC;
pub const RK806_TOP_RESEV_POR: c_uint = 0xED;
pub const RK806_BUCK_VRSN_REG1: c_uint = 0xEE;
pub const RK806_BUCK_VRSN_REG2: c_uint = 0xEF;
pub const RK806_NLDO_RLOAD_SEL_REG1: c_uint = 0xF0;
pub const RK806_PLDO_RLOAD_SEL_REG1: c_uint = 0xF1;
pub const RK806_PLDO_RLOAD_SEL_REG2: c_uint = 0xF2;
pub const RK806_BUCK_CMIN_MX_REG1: c_uint = 0xF3;
pub const RK806_BUCK_CMIN_MX_REG2: c_uint = 0xF4;
pub const RK806_BUCK_FREQ_SET_REG1: c_uint = 0xF5;
pub const RK806_BUCK_FREQ_SET_REG2: c_uint = 0xF6;
pub const RK806_BUCK_RS_MEABS_REG1: c_uint = 0xF7;
pub const RK806_BUCK_RS_MEABS_REG2: c_uint = 0xF8;
pub const RK806_BUCK_RS_ZDLEB_REG1: c_uint = 0xF9;
pub const RK806_BUCK_RS_ZDLEB_REG2: c_uint = 0xFA;
pub const RK806_BUCK_RSERVE_REG1: c_uint = 0xFB;
pub const RK806_BUCK_RSERVE_REG2: c_uint = 0xFC;
pub const RK806_BUCK_RSERVE_REG3: c_uint = 0xFD;
pub const RK806_BUCK_RSERVE_REG4: c_uint = 0xFE;
pub const RK806_BUCK_RSERVE_REG5: c_uint = 0xFF;
// INT_STS Register field definitions

// SPI command
pub const RK806_CMD_READ: c_int = 0;

pub const RK806_CMD_CRC_DIS: c_int = 0;
pub const RK806_CMD_LEN_MSK: c_uint = 0x0f;
pub const RK806_REG_H: c_uint = 0x00;
pub const VERSION_AB: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk806_reg_id {
    RK806_ID_DCDC1 = 0,
    RK806_ID_DCDC2,
    RK806_ID_DCDC3,
    RK806_ID_DCDC4,
    RK806_ID_DCDC5,
    RK806_ID_DCDC6,
    RK806_ID_DCDC7,
    RK806_ID_DCDC8,
    RK806_ID_DCDC9,
    RK806_ID_DCDC10,

    RK806_ID_NLDO1,
    RK806_ID_NLDO2,
    RK806_ID_NLDO3,
    RK806_ID_NLDO4,
    RK806_ID_NLDO5,

    RK806_ID_PLDO1,
    RK806_ID_PLDO2,
    RK806_ID_PLDO3,
    RK806_ID_PLDO4,
    RK806_ID_PLDO5,
    RK806_ID_PLDO6,
    RK806_ID_END,
}

// Define the RK806 IRQ numbers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk806_irqs {
// INT_STS0 registers
    RK806_IRQ_PWRON_FALL,
    RK806_IRQ_PWRON_RISE,
    RK806_IRQ_PWRON,
    RK806_IRQ_PWRON_LP,
    RK806_IRQ_HOTDIE,
    RK806_IRQ_VDC_RISE,
    RK806_IRQ_VDC_FALL,
    RK806_IRQ_VB_LO,

// INT_STS0 registers
    RK806_IRQ_REV0,
    RK806_IRQ_REV1,
    RK806_IRQ_REV2,
    RK806_IRQ_CRC_ERROR,
    RK806_IRQ_SLP3_GPIO,
    RK806_IRQ_SLP2_GPIO,
    RK806_IRQ_SLP1_GPIO,
    RK806_IRQ_WDT,
}

// VCC1 Low Voltage Threshold
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk806_lv_sel {
    VB_LO_SEL_2800,
    VB_LO_SEL_2900,
    VB_LO_SEL_3000,
    VB_LO_SEL_3100,
    VB_LO_SEL_3200,
    VB_LO_SEL_3300,
    VB_LO_SEL_3400,
    VB_LO_SEL_3500,
}

// System Shutdown Voltage Select
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk806_uv_sel {
    VB_UV_SEL_2700,
    VB_UV_SEL_2800,
    VB_UV_SEL_2900,
    VB_UV_SEL_3000,
    VB_UV_SEL_3100,
    VB_UV_SEL_3200,
    VB_UV_SEL_3300,
    VB_UV_SEL_3400,
}

// Pin Function
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk806_pwrctrl_fun {
    PWRCTRL_NULL_FUN,
    PWRCTRL_SLP_FUN,
    PWRCTRL_POWOFF_FUN,
    PWRCTRL_RST_FUN,
    PWRCTRL_DVS_FUN,
    PWRCTRL_GPIO_FUN,
}

// Pin Polarity
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk806_pin_level {
    POL_LOW,
    POL_HIGH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk806_vsel_ctr_sel {
    CTR_BY_NO_EFFECT,
    CTR_BY_PWRCTRL1,
    CTR_BY_PWRCTRL2,
    CTR_BY_PWRCTRL3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk806_dvs_ctr_sel {
    CTR_SEL_NO_EFFECT,
    CTR_SEL_DVS_START1,
    CTR_SEL_DVS_START2,
    CTR_SEL_DVS_START3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk806_pin_dr_sel {
    RK806_PIN_INPUT,
    RK806_PIN_OUTPUT,
}

pub const RK806_INT_POL_L: c_int = 0;
// SYS_CFG3

pub const RK806_SLAVE_RESTART_FUN_OFF: c_int = 0;

pub const RK806_SYS_ENB2_2M_OFF: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk806_int_fun {
    RK806_INT_ONLY,
    RK806_INT_ADN_WKUP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk806_dvs_mode {
    RK806_DVS_NOT_SUPPORT,
    RK806_DVS_START1,
    RK806_DVS_START2,
    RK806_DVS_START3,
    RK806_DVS_PWRCTRL1,
    RK806_DVS_PWRCTRL2,
    RK806_DVS_PWRCTRL3,
    RK806_DVS_START_PWRCTR1,
    RK806_DVS_START_PWRCTR2,
    RK806_DVS_START_PWRCTR3,
    RK806_DVS_END,
}

// RK808 IRQ Definitions
pub const RK808_IRQ_VOUT_LO: c_int = 0;
pub const RK808_IRQ_VB_LO: c_int = 1;
pub const RK808_IRQ_PWRON: c_int = 2;
pub const RK808_IRQ_PWRON_LP: c_int = 3;
pub const RK808_IRQ_HOTDIE: c_int = 4;
pub const RK808_IRQ_RTC_ALARM: c_int = 5;
pub const RK808_IRQ_RTC_PERIOD: c_int = 6;
pub const RK808_IRQ_PLUG_IN_INT: c_int = 7;
pub const RK808_IRQ_PLUG_OUT_INT: c_int = 8;
pub const RK808_NUM_IRQ: c_int = 9;

// RK818 IRQ Definitions
pub const RK818_IRQ_VOUT_LO: c_int = 0;
pub const RK818_IRQ_VB_LO: c_int = 1;
pub const RK818_IRQ_PWRON: c_int = 2;
pub const RK818_IRQ_PWRON_LP: c_int = 3;
pub const RK818_IRQ_HOTDIE: c_int = 4;
pub const RK818_IRQ_RTC_ALARM: c_int = 5;
pub const RK818_IRQ_RTC_PERIOD: c_int = 6;
pub const RK818_IRQ_USB_OV: c_int = 7;
pub const RK818_IRQ_PLUG_IN: c_int = 8;
pub const RK818_IRQ_PLUG_OUT: c_int = 9;
pub const RK818_IRQ_CHG_OK: c_int = 10;
pub const RK818_IRQ_CHG_TE: c_int = 11;
pub const RK818_IRQ_CHG_TS1: c_int = 12;
pub const RK818_IRQ_TS2: c_int = 13;
pub const RK818_IRQ_CHG_CVTLIM: c_int = 14;
pub const RK818_IRQ_DISCHG_ILIM: c_int = 15;

pub const RK818_NUM_IRQ: c_int = 16;
pub const RK808_VBAT_LOW_2V8: c_uint = 0x00;
pub const RK808_VBAT_LOW_2V9: c_uint = 0x01;
pub const RK808_VBAT_LOW_3V0: c_uint = 0x02;
pub const RK808_VBAT_LOW_3V1: c_uint = 0x03;
pub const RK808_VBAT_LOW_3V2: c_uint = 0x04;
pub const RK808_VBAT_LOW_3V3: c_uint = 0x05;
pub const RK808_VBAT_LOW_3V4: c_uint = 0x06;
pub const RK808_VBAT_LOW_3V5: c_uint = 0x07;

pub const MASK_ALL: c_uint = 0xff;
pub const BUCK_UV_ACT_MASK: c_uint = 0x0f;
pub const BUCK_UV_ACT_DISABLE: c_int = 0;

pub const TEMP105C: c_uint = 0x08;
pub const TEMP115C: c_uint = 0x0c;
pub const TEMP_HOTDIE_MSK: c_uint = 0x0c;

pub const RK8XX_ID_MSK: c_uint = 0xfff0;

pub const AUTO_PWM_MODE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk817_reg_id {
    RK817_ID_DCDC1 = 0,
    RK817_ID_DCDC2,
    RK817_ID_DCDC3,
    RK817_ID_DCDC4,
    RK817_ID_LDO1,
    RK817_ID_LDO2,
    RK817_ID_LDO3,
    RK817_ID_LDO4,
    RK817_ID_LDO5,
    RK817_ID_LDO6,
    RK817_ID_LDO7,
    RK817_ID_LDO8,
    RK817_ID_LDO9,
    RK817_ID_BOOST,
    RK817_ID_BOOST_OTG_SW,
    RK817_NUM_REGULATORS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rk809_reg_id {
    RK809_ID_DCDC5 = RK817_ID_BOOST,
    RK809_ID_SW1,
    RK809_ID_SW2,
    RK809_NUM_REGULATORS
}

pub const RK817_SECONDS_REG: c_uint = 0x00;
pub const RK817_MINUTES_REG: c_uint = 0x01;
pub const RK817_HOURS_REG: c_uint = 0x02;
pub const RK817_DAYS_REG: c_uint = 0x03;
pub const RK817_MONTHS_REG: c_uint = 0x04;
pub const RK817_YEARS_REG: c_uint = 0x05;
pub const RK817_WEEKS_REG: c_uint = 0x06;
pub const RK817_ALARM_SECONDS_REG: c_uint = 0x07;
pub const RK817_ALARM_MINUTES_REG: c_uint = 0x08;
pub const RK817_ALARM_HOURS_REG: c_uint = 0x09;
pub const RK817_ALARM_DAYS_REG: c_uint = 0x0a;
pub const RK817_ALARM_MONTHS_REG: c_uint = 0x0b;
pub const RK817_ALARM_YEARS_REG: c_uint = 0x0c;
pub const RK817_RTC_CTRL_REG: c_uint = 0xd;
pub const RK817_RTC_STATUS_REG: c_uint = 0xe;
pub const RK817_RTC_INT_REG: c_uint = 0xf;
pub const RK817_RTC_COMP_LSB_REG: c_uint = 0x10;
pub const RK817_RTC_COMP_MSB_REG: c_uint = 0x11;
// RK817 Codec Registers
pub const RK817_CODEC_DTOP_VUCTL: c_uint = 0x12;
pub const RK817_CODEC_DTOP_VUCTIME: c_uint = 0x13;
pub const RK817_CODEC_DTOP_LPT_SRST: c_uint = 0x14;
pub const RK817_CODEC_DTOP_DIGEN_CLKE: c_uint = 0x15;
pub const RK817_CODEC_AREF_RTCFG0: c_uint = 0x16;
pub const RK817_CODEC_AREF_RTCFG1: c_uint = 0x17;
pub const RK817_CODEC_AADC_CFG0: c_uint = 0x18;
pub const RK817_CODEC_AADC_CFG1: c_uint = 0x19;
pub const RK817_CODEC_DADC_VOLL: c_uint = 0x1a;
pub const RK817_CODEC_DADC_VOLR: c_uint = 0x1b;
pub const RK817_CODEC_DADC_SR_ACL0: c_uint = 0x1e;
pub const RK817_CODEC_DADC_ALC1: c_uint = 0x1f;
pub const RK817_CODEC_DADC_ALC2: c_uint = 0x20;
pub const RK817_CODEC_DADC_NG: c_uint = 0x21;
pub const RK817_CODEC_DADC_HPF: c_uint = 0x22;
pub const RK817_CODEC_DADC_RVOLL: c_uint = 0x23;
pub const RK817_CODEC_DADC_RVOLR: c_uint = 0x24;
pub const RK817_CODEC_AMIC_CFG0: c_uint = 0x27;
pub const RK817_CODEC_AMIC_CFG1: c_uint = 0x28;
pub const RK817_CODEC_DMIC_PGA_GAIN: c_uint = 0x29;
pub const RK817_CODEC_DMIC_LMT1: c_uint = 0x2a;
pub const RK817_CODEC_DMIC_LMT2: c_uint = 0x2b;
pub const RK817_CODEC_DMIC_NG1: c_uint = 0x2c;
pub const RK817_CODEC_DMIC_NG2: c_uint = 0x2d;
pub const RK817_CODEC_ADAC_CFG0: c_uint = 0x2e;
pub const RK817_CODEC_ADAC_CFG1: c_uint = 0x2f;
pub const RK817_CODEC_DDAC_POPD_DACST: c_uint = 0x30;
pub const RK817_CODEC_DDAC_VOLL: c_uint = 0x31;
pub const RK817_CODEC_DDAC_VOLR: c_uint = 0x32;
pub const RK817_CODEC_DDAC_SR_LMT0: c_uint = 0x35;
pub const RK817_CODEC_DDAC_LMT1: c_uint = 0x36;
pub const RK817_CODEC_DDAC_LMT2: c_uint = 0x37;
pub const RK817_CODEC_DDAC_MUTE_MIXCTL: c_uint = 0x38;
pub const RK817_CODEC_DDAC_RVOLL: c_uint = 0x39;
pub const RK817_CODEC_DDAC_RVOLR: c_uint = 0x3a;
pub const RK817_CODEC_AHP_ANTI0: c_uint = 0x3b;
pub const RK817_CODEC_AHP_ANTI1: c_uint = 0x3c;
pub const RK817_CODEC_AHP_CFG0: c_uint = 0x3d;
pub const RK817_CODEC_AHP_CFG1: c_uint = 0x3e;
pub const RK817_CODEC_AHP_CP: c_uint = 0x3f;
pub const RK817_CODEC_ACLASSD_CFG1: c_uint = 0x40;
pub const RK817_CODEC_ACLASSD_CFG2: c_uint = 0x41;
pub const RK817_CODEC_APLL_CFG0: c_uint = 0x42;
pub const RK817_CODEC_APLL_CFG1: c_uint = 0x43;
pub const RK817_CODEC_APLL_CFG2: c_uint = 0x44;
pub const RK817_CODEC_APLL_CFG3: c_uint = 0x45;
pub const RK817_CODEC_APLL_CFG4: c_uint = 0x46;
pub const RK817_CODEC_APLL_CFG5: c_uint = 0x47;
pub const RK817_CODEC_DI2S_CKM: c_uint = 0x48;
pub const RK817_CODEC_DI2S_RSD: c_uint = 0x49;
pub const RK817_CODEC_DI2S_RXCR1: c_uint = 0x4a;
pub const RK817_CODEC_DI2S_RXCR2: c_uint = 0x4b;
pub const RK817_CODEC_DI2S_RXCMD_TSD: c_uint = 0x4c;
pub const RK817_CODEC_DI2S_TXCR1: c_uint = 0x4d;
pub const RK817_CODEC_DI2S_TXCR2: c_uint = 0x4e;
pub const RK817_CODEC_DI2S_TXCR3_TXCMD: c_uint = 0x4f;
// RK817_CODEC_DI2S_CKM

// RK817_CODEC_DDAC_MUTE_MIXCTL

// RK817_CODEC_DI2S_RXCR2

// RK817_CODEC_DI2S_TXCR2

// RK817_CODEC_AMIC_CFG0

// RK817 Battery Registers
pub const RK817_GAS_GAUGE_ADC_CONFIG0: c_uint = 0x50;

pub const RK817_GAS_GAUGE_ADC_CONFIG1: c_uint = 0x55;

pub const RK817_GAS_GAUGE_GG_CON: c_uint = 0x56;
pub const RK817_GAS_GAUGE_GG_STS: c_uint = 0x57;

pub const RK817_GAS_GAUGE_RELAX_THRE_H: c_uint = 0x58;
pub const RK817_GAS_GAUGE_RELAX_THRE_L: c_uint = 0x59;
pub const RK817_GAS_GAUGE_OCV_THRE_VOL: c_uint = 0x62;
pub const RK817_GAS_GAUGE_OCV_VOL_H: c_uint = 0x63;
pub const RK817_GAS_GAUGE_OCV_VOL_L: c_uint = 0x64;
pub const RK817_GAS_GAUGE_PWRON_VOL_H: c_uint = 0x6b;
pub const RK817_GAS_GAUGE_PWRON_VOL_L: c_uint = 0x6c;
pub const RK817_GAS_GAUGE_PWRON_CUR_H: c_uint = 0x6d;
pub const RK817_GAS_GAUGE_PWRON_CUR_L: c_uint = 0x6e;
pub const RK817_GAS_GAUGE_OFF_CNT: c_uint = 0x6f;
pub const RK817_GAS_GAUGE_Q_INIT_H3: c_uint = 0x70;
pub const RK817_GAS_GAUGE_Q_INIT_H2: c_uint = 0x71;
pub const RK817_GAS_GAUGE_Q_INIT_L1: c_uint = 0x72;
pub const RK817_GAS_GAUGE_Q_INIT_L0: c_uint = 0x73;
pub const RK817_GAS_GAUGE_Q_PRES_H3: c_uint = 0x74;
pub const RK817_GAS_GAUGE_Q_PRES_H2: c_uint = 0x75;
pub const RK817_GAS_GAUGE_Q_PRES_L1: c_uint = 0x76;
pub const RK817_GAS_GAUGE_Q_PRES_L0: c_uint = 0x77;
pub const RK817_GAS_GAUGE_BAT_VOL_H: c_uint = 0x78;
pub const RK817_GAS_GAUGE_BAT_VOL_L: c_uint = 0x79;
pub const RK817_GAS_GAUGE_BAT_CUR_H: c_uint = 0x7a;
pub const RK817_GAS_GAUGE_BAT_CUR_L: c_uint = 0x7b;
pub const RK817_GAS_GAUGE_USB_VOL_H: c_uint = 0x7e;
pub const RK817_GAS_GAUGE_USB_VOL_L: c_uint = 0x7f;
pub const RK817_GAS_GAUGE_SYS_VOL_H: c_uint = 0x80;
pub const RK817_GAS_GAUGE_SYS_VOL_L: c_uint = 0x81;
pub const RK817_GAS_GAUGE_Q_MAX_H3: c_uint = 0x82;
pub const RK817_GAS_GAUGE_Q_MAX_H2: c_uint = 0x83;
pub const RK817_GAS_GAUGE_Q_MAX_L1: c_uint = 0x84;
pub const RK817_GAS_GAUGE_Q_MAX_L0: c_uint = 0x85;
pub const RK817_GAS_GAUGE_SLEEP_CON_SAMP_CUR_H: c_uint = 0x8f;
pub const RK817_GAS_GAUGE_SLEEP_CON_SAMP_CUR_L: c_uint = 0x90;
pub const RK817_GAS_GAUGE_CAL_OFFSET_H: c_uint = 0x91;
pub const RK817_GAS_GAUGE_CAL_OFFSET_L: c_uint = 0x92;
pub const RK817_GAS_GAUGE_VCALIB0_H: c_uint = 0x93;
pub const RK817_GAS_GAUGE_VCALIB0_L: c_uint = 0x94;
pub const RK817_GAS_GAUGE_VCALIB1_H: c_uint = 0x95;
pub const RK817_GAS_GAUGE_VCALIB1_L: c_uint = 0x96;
pub const RK817_GAS_GAUGE_IOFFSET_H: c_uint = 0x97;
pub const RK817_GAS_GAUGE_IOFFSET_L: c_uint = 0x98;
pub const RK817_GAS_GAUGE_BAT_R1: c_uint = 0x9a;
pub const RK817_GAS_GAUGE_BAT_R2: c_uint = 0x9b;
pub const RK817_GAS_GAUGE_BAT_R3: c_uint = 0x9c;
pub const RK817_GAS_GAUGE_DATA0: c_uint = 0x9d;
pub const RK817_GAS_GAUGE_DATA1: c_uint = 0x9e;
pub const RK817_GAS_GAUGE_DATA2: c_uint = 0x9f;
pub const RK817_GAS_GAUGE_DATA3: c_uint = 0xa0;
pub const RK817_GAS_GAUGE_DATA4: c_uint = 0xa1;
pub const RK817_GAS_GAUGE_DATA5: c_uint = 0xa2;
pub const RK817_GAS_GAUGE_CUR_ADC_K0: c_uint = 0xb0;

pub const RK817_BUCK1_ON_VSEL_REG: c_uint = 0xBB;
pub const RK817_BUCK1_SLP_VSEL_REG: c_uint = 0xBC;
pub const RK817_BUCK2_CONFIG_REG: c_uint = 0xBD;
pub const RK817_BUCK2_ON_VSEL_REG: c_uint = 0xBE;
pub const RK817_BUCK2_SLP_VSEL_REG: c_uint = 0xBF;
pub const RK817_BUCK3_CONFIG_REG: c_uint = 0xC0;
pub const RK817_BUCK3_ON_VSEL_REG: c_uint = 0xC1;
pub const RK817_BUCK3_SLP_VSEL_REG: c_uint = 0xC2;
pub const RK817_BUCK4_CONFIG_REG: c_uint = 0xC3;
pub const RK817_BUCK4_ON_VSEL_REG: c_uint = 0xC4;
pub const RK817_BUCK4_SLP_VSEL_REG: c_uint = 0xC5;

pub const RK817_PMIC_CHRG_OUT: c_uint = 0xe4;

pub const RK817_PMIC_CHRG_IN: c_uint = 0xe5;

pub const RK817_PMIC_CHRG_TERM: c_uint = 0xe6;

pub const RK817_PMIC_CHRG_STS: c_uint = 0xeb;

pub const RK817_ID_MSB: c_uint = 0xed;
pub const RK817_ID_LSB: c_uint = 0xee;
pub const RK817_SYS_STS: c_uint = 0xf0;

pub const RK817_ON_SOURCE_REG: c_uint = 0xf5;
pub const RK817_OFF_SOURCE_REG: c_uint = 0xf6;
// INTERRUPT REGISTER
pub const RK817_INT_STS_REG0: c_uint = 0xf8;
pub const RK817_INT_STS_MSK_REG0: c_uint = 0xf9;
pub const RK817_INT_STS_REG1: c_uint = 0xfa;
pub const RK817_INT_STS_MSK_REG1: c_uint = 0xfb;
pub const RK817_INT_STS_REG2: c_uint = 0xfc;
pub const RK817_INT_STS_MSK_REG2: c_uint = 0xfd;
pub const RK817_GPIO_INT_CFG: c_uint = 0xfe;
// IRQ Definitions
pub const RK817_IRQ_PWRON_FALL: c_int = 0;
pub const RK817_IRQ_PWRON_RISE: c_int = 1;
pub const RK817_IRQ_PWRON: c_int = 2;
pub const RK817_IRQ_PWMON_LP: c_int = 3;
pub const RK817_IRQ_HOTDIE: c_int = 4;
pub const RK817_IRQ_RTC_ALARM: c_int = 5;
pub const RK817_IRQ_RTC_PERIOD: c_int = 6;
pub const RK817_IRQ_VB_LO: c_int = 7;
pub const RK817_IRQ_PLUG_IN: c_int = 8;
pub const RK817_IRQ_PLUG_OUT: c_int = 9;
pub const RK817_IRQ_CHRG_TERM: c_int = 10;
pub const RK817_IRQ_CHRG_TIME: c_int = 11;
pub const RK817_IRQ_CHRG_TS: c_int = 12;
pub const RK817_IRQ_USB_OV: c_int = 13;
pub const RK817_IRQ_CHRG_IN_CLMP: c_int = 14;
pub const RK817_IRQ_BAT_DIS_ILIM: c_int = 15;
pub const RK817_IRQ_GATE_GPIO: c_int = 16;
pub const RK817_IRQ_TS_GPIO: c_int = 17;
pub const RK817_IRQ_CODEC_PD: c_int = 18;
pub const RK817_IRQ_CODEC_PO: c_int = 19;
pub const RK817_IRQ_CLASSD_MUTE_DONE: c_int = 20;
pub const RK817_IRQ_CLASSD_OCP: c_int = 21;
pub const RK817_IRQ_BAT_OVP: c_int = 22;
pub const RK817_IRQ_CHRG_BAT_HI: c_int = 23;

//
// rtc_ctrl 0xd
// same as 808, except bit4
//

// power config 0xb9

pub const RK817_BUCK3_FB_RES_EXT: c_int = 0;
// buck config 0xba
pub const RK817_RAMP_RATE_OFFSET: c_int = 6;

// sys_cfg1 0xf2

pub const RK817_TSD_140: c_int = 0;

// sys_cfg3 0xf4

// gpio&int 0xfe

pub const RK817_INT_POL_L: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk808 {
    pub dev: *mut device,
    pub irq_data: *mut regmap_irq_chip_data,
    pub regmap: *mut regmap,
    pub variant: c_long,
    pub regmap_cfg: *const regmap_config,
    pub regmap_irq_chip: *const regmap_irq_chip,
}

extern "C" {
    pub fn rk8xx_shutdown(dev: *mut device);
}
extern "C" {
    pub fn rk8xx_probe(dev: *mut device, variant: c_int, irq: c_uint, regmap: *mut regmap) -> c_int;
}
extern "C" {
    pub fn rk8xx_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn rk8xx_resume(dev: *mut device) -> c_int;
}
