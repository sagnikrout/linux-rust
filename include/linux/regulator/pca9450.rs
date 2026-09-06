//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/pca9450.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright 2020 NXP.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pca9450_chip_type {
    PCA9450_TYPE_PCA9450A = 0,
    PCA9450_TYPE_PCA9450BC,
    PCA9450_TYPE_PCA9451A,
    PCA9450_TYPE_PCA9452,
    PCA9450_TYPE_AMOUNT,
}

pub const PCA9450_RESTART_HANDLER_PRIORITY: c_int = 130;
pub const PCA9450_BUCK1_VOLTAGE_NUM: c_uint = 0x80;
pub const PCA9450_BUCK2_VOLTAGE_NUM: c_uint = 0x80;
pub const PCA9450_BUCK3_VOLTAGE_NUM: c_uint = 0x80;
pub const PCA9450_BUCK4_VOLTAGE_NUM: c_uint = 0x80;
pub const PCA9450_BUCK5_VOLTAGE_NUM: c_uint = 0x80;
pub const PCA9450_BUCK6_VOLTAGE_NUM: c_uint = 0x80;
pub const PCA9450_LDO1_VOLTAGE_NUM: c_uint = 0x08;
pub const PCA9450_LDO2_VOLTAGE_NUM: c_uint = 0x08;
pub const PCA9450_LDO3_VOLTAGE_NUM: c_uint = 0x20;
pub const PCA9450_LDO4_VOLTAGE_NUM: c_uint = 0x20;
pub const PCA9450_LDO5_VOLTAGE_NUM: c_uint = 0x10;
// PCA9450 BUCK ENMODE bits
pub const BUCK_ENMODE_OFF: c_uint = 0x00;
pub const BUCK_ENMODE_ONREQ: c_uint = 0x01;
pub const BUCK_ENMODE_ONREQ_STBYREQ: c_uint = 0x02;
pub const BUCK_ENMODE_ON: c_uint = 0x03;
// PCA9450_REG_BUCK1_CTRL bits
pub const BUCK1_RAMP_MASK: c_uint = 0xC0;
pub const BUCK1_RAMP_25MV: c_uint = 0x0;
pub const BUCK1_RAMP_12P5MV: c_uint = 0x1;
pub const BUCK1_RAMP_6P25MV: c_uint = 0x2;
pub const BUCK1_RAMP_3P125MV: c_uint = 0x3;
pub const BUCK1_DVS_CTRL: c_uint = 0x10;
pub const BUCK1_AD: c_uint = 0x08;
pub const BUCK1_FPWM: c_uint = 0x04;
pub const BUCK1_ENMODE_MASK: c_uint = 0x03;
// PCA9450_REG_BUCK2_CTRL bits
pub const BUCK2_RAMP_MASK: c_uint = 0xC0;
pub const BUCK2_RAMP_25MV: c_uint = 0x0;
pub const BUCK2_RAMP_12P5MV: c_uint = 0x1;
pub const BUCK2_RAMP_6P25MV: c_uint = 0x2;
pub const BUCK2_RAMP_3P125MV: c_uint = 0x3;
pub const BUCK2_DVS_CTRL: c_uint = 0x10;
pub const BUCK2_AD: c_uint = 0x08;
pub const BUCK2_FPWM: c_uint = 0x04;
pub const BUCK2_ENMODE_MASK: c_uint = 0x03;
// PCA9450_REG_BUCK3_CTRL bits
pub const BUCK3_RAMP_MASK: c_uint = 0xC0;
pub const BUCK3_RAMP_25MV: c_uint = 0x0;
pub const BUCK3_RAMP_12P5MV: c_uint = 0x1;
pub const BUCK3_RAMP_6P25MV: c_uint = 0x2;
pub const BUCK3_RAMP_3P125MV: c_uint = 0x3;
pub const BUCK3_DVS_CTRL: c_uint = 0x10;
pub const BUCK3_AD: c_uint = 0x08;
pub const BUCK3_FPWM: c_uint = 0x04;
pub const BUCK3_ENMODE_MASK: c_uint = 0x03;
// PCA9450_REG_BUCK4_CTRL bits
pub const BUCK4_AD: c_uint = 0x08;
pub const BUCK4_FPWM: c_uint = 0x04;
pub const BUCK4_ENMODE_MASK: c_uint = 0x03;
// PCA9450_REG_BUCK5_CTRL bits
pub const BUCK5_AD: c_uint = 0x08;
pub const BUCK5_FPWM: c_uint = 0x04;
pub const BUCK5_ENMODE_MASK: c_uint = 0x03;
// PCA9450_REG_BUCK6_CTRL bits
pub const BUCK6_AD: c_uint = 0x08;
pub const BUCK6_FPWM: c_uint = 0x04;
pub const BUCK6_ENMODE_MASK: c_uint = 0x03;
// PCA9450_REG_BUCK123_PRESET_EN bit
pub const BUCK123_PRESET_EN: c_uint = 0x80;
// PCA9450_BUCK1OUT_DVS0 bits
pub const BUCK1OUT_DVS0_MASK: c_uint = 0x7F;
pub const BUCK1OUT_DVS0_DEFAULT: c_uint = 0x14;
// PCA9450_BUCK1OUT_DVS1 bits
pub const BUCK1OUT_DVS1_MASK: c_uint = 0x7F;
pub const BUCK1OUT_DVS1_DEFAULT: c_uint = 0x14;
// PCA9450_BUCK2OUT_DVS0 bits
pub const BUCK2OUT_DVS0_MASK: c_uint = 0x7F;
pub const BUCK2OUT_DVS0_DEFAULT: c_uint = 0x14;
// PCA9450_BUCK2OUT_DVS1 bits
pub const BUCK2OUT_DVS1_MASK: c_uint = 0x7F;
pub const BUCK2OUT_DVS1_DEFAULT: c_uint = 0x14;
// PCA9450_BUCK3OUT_DVS0 bits
pub const BUCK3OUT_DVS0_MASK: c_uint = 0x7F;
pub const BUCK3OUT_DVS0_DEFAULT: c_uint = 0x14;
// PCA9450_BUCK3OUT_DVS1 bits
pub const BUCK3OUT_DVS1_MASK: c_uint = 0x7F;
pub const BUCK3OUT_DVS1_DEFAULT: c_uint = 0x14;
// PCA9450_REG_BUCK4OUT bits
pub const BUCK4OUT_MASK: c_uint = 0x7F;
pub const BUCK4OUT_DEFAULT: c_uint = 0x6C;
// PCA9450_REG_BUCK5OUT bits
pub const BUCK5OUT_MASK: c_uint = 0x7F;
pub const BUCK5OUT_DEFAULT: c_uint = 0x30;
// PCA9450_REG_BUCK6OUT bits
pub const BUCK6OUT_MASK: c_uint = 0x7F;
pub const BUCK6OUT_DEFAULT: c_uint = 0x14;
// PCA9450_REG_LDO1_VOLT bits
pub const LDO1_EN_MASK: c_uint = 0xC0;
pub const LDO1OUT_MASK: c_uint = 0x07;
// PCA9450_REG_LDO2_VOLT bits
pub const LDO2_EN_MASK: c_uint = 0xC0;
pub const LDO2OUT_MASK: c_uint = 0x07;
// PCA9450_REG_LDO3_VOLT bits
pub const LDO3_EN_MASK: c_uint = 0xC0;
pub const LDO3OUT_MASK: c_uint = 0x1F;
// PCA9450_REG_LDO4_VOLT bits
pub const LDO4_EN_MASK: c_uint = 0xC0;
pub const LDO4OUT_MASK: c_uint = 0x1F;
// PCA9450_REG_LDO5_VOLT bits
pub const LDO5L_EN_MASK: c_uint = 0xC0;
pub const LDO5LOUT_MASK: c_uint = 0x0F;
pub const LDO5H_EN_MASK: c_uint = 0xC0;
pub const LDO5HOUT_MASK: c_uint = 0x0F;
// PCA9450_REG_IRQ bits
pub const IRQ_PWRON: c_uint = 0x80;
pub const IRQ_WDOGB: c_uint = 0x40;
pub const IRQ_RSVD: c_uint = 0x20;
pub const IRQ_VR_FLT1: c_uint = 0x10;
pub const IRQ_VR_FLT2: c_uint = 0x08;
pub const IRQ_LOWVSYS: c_uint = 0x04;
pub const IRQ_THERM_105: c_uint = 0x02;
pub const IRQ_THERM_125: c_uint = 0x01;
// PCA9450_REG_PWRCTRL bits
pub const T_ON_DEB_MASK: c_uint = 0xC0;

pub const T_OFF_DEB_MASK: c_uint = 0x20;

pub const T_ON_STEP_MASK: c_uint = 0x18;

pub const T_OFF_STEP_MASK: c_uint = 0x06;

pub const T_RESTART_MASK: c_uint = 0x01;
pub const T_RESTART_250MS: c_int = 0;
pub const T_RESTART_500MS: c_int = 1;
// PCA9450_REG_RESET_CTRL bits
pub const WDOG_B_CFG_MASK: c_uint = 0xC0;
pub const WDOG_B_CFG_NONE: c_uint = 0x00;
pub const WDOG_B_CFG_WARM: c_uint = 0x40;
pub const WDOG_B_CFG_COLD_LDO12: c_uint = 0x80;
pub const WDOG_B_CFG_COLD: c_uint = 0xC0;
pub const T_PMIC_RST_DEB_MASK: c_uint = 0x07;
pub const T_PMIC_RST_DEB_10MS: c_uint = 0x00;
pub const T_PMIC_RST_DEB_50MS: c_uint = 0x01;
pub const T_PMIC_RST_DEB_100MS: c_uint = 0x02;
pub const T_PMIC_RST_DEB_500MS: c_uint = 0x03;
pub const T_PMIC_RST_DEB_1S: c_uint = 0x04;
pub const T_PMIC_RST_DEB_2S: c_uint = 0x05;
pub const T_PMIC_RST_DEB_4S: c_uint = 0x06;
pub const T_PMIC_RST_DEB_8S: c_uint = 0x07;
// PCA9450_REG_CONFIG2 bits
pub const I2C_LT_MASK: c_uint = 0x03;
pub const I2C_LT_FORCE_DISABLE: c_uint = 0x00;
pub const I2C_LT_ON_STANDBY_RUN: c_uint = 0x01;
pub const I2C_LT_ON_RUN: c_uint = 0x02;
pub const I2C_LT_FORCE_ENABLE: c_uint = 0x03;
// PCA9450_REG_SW_RST command
pub const SW_RST_COMMAND: c_uint = 0x14;
