//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rohm-bd71815.h
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
//
// Copyright 2021 ROHM Semiconductors.
//
// Author: Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>
//
// Copyright 2014 Embest Technology Co. Ltd. Inc.
//
// Author: yanglsh@embest-tech.com
//

// General Purpose
// LDOs for SD Card and SD Card Interface
// LDO for DDR Reference Voltage
// LDO for Low-Power State Retention
pub const BD71815_SUPPLY_STATE_ENABLED: c_uint = 0x1;
// BD71815_REG_BUCK1_MODE bits
pub const BD71815_BUCK_RAMPRATE_MASK: c_uint = 0xC0;
pub const BD71815_BUCK_RAMPRATE_10P00MV: c_uint = 0x0;
pub const BD71815_BUCK_RAMPRATE_5P00MV: c_uint = 0x01;
pub const BD71815_BUCK_RAMPRATE_2P50MV: c_uint = 0x02;
pub const BD71815_BUCK_RAMPRATE_1P25MV: c_uint = 0x03;

// BD71815_REG_BUCK1_VOLT_H bits

pub const BD71815_VOLT_MASK: c_uint = 0x3F;
pub const BD71815_BUCK1_H_DEFAULT: c_uint = 0x14;
pub const BD71815_BUCK1_L_DEFAULT: c_uint = 0x14;
// BD71815_REG_BUCK2_VOLT_H bits
pub const BD71815_BUCK2_H_DEFAULT: c_uint = 0x14;
pub const BD71815_BUCK2_L_DEFAULT: c_uint = 0x14;
// WLED output
// current register mask
pub const LED_DIMM_MASK: c_uint = 0x3f;
// LED enable bits at LED_CTRL reg

// BD71815_REG_LDO1_CTRL bits

// LDO_MODE1_register

// set => register control, unset => GPIO control

pub const LDO4_MODE_GPIO: c_int = 0;
// set => register control, unset => start when DCIN connected

pub const LDO3_MODE_DCIN: c_int = 0;
// LDO_MODE2 register

// LDO_MODE3 register

// LDO_MODE4 register

// BD71815_REG_OUT32K bits

pub const OUT32K_MODE_OPEN_DRAIN: c_int = 0;
// BD71815_REG_BAT_STAT bits

pub const BAT_DET_OFFSET: c_int = 5;

// BD71815_REG_VBUS_STAT bits

// BD71815_REG_ALM0_MASK bits

// BD71815_REG_INT_EN_00 bits

// BD71815_REG_INT_STAT_03 bits

// BD71805_REG_INT_STAT_08 bits

// BD71805_REG_INT_STAT_11 bits

// BD71815_REG_PWRCTRL bits

// BD71815_REG_GPO bits

pub const BD71815_GPIO_OPEN_DRAIN: c_int = 0;

// BD71815 interrupt masks
// BD71815 interrupt irqs
// BUCK reg interrupts
// DCIN1 interrupts
// DCIN2 interrupts
// Vsys INT_STAT_04
// Charger INT_STAT_05
// Battery  INT_STAT_06
// Battery Mon 1 INT_STAT_07
// Battery Mon 2 INT_STAT_08
// Battery Mon 3 (Coulomb counter) INT_STAT_09
// Battery Mon 4 INT_STAT_10
// Temperature INT_STAT_11
// RTC Alarm INT_STAT_12

// BD71815_REG_CC_CTRL bits
pub const CCNTRST: c_uint = 0x80;
pub const CCNTENB: c_uint = 0x40;
pub const CCCALIB: c_uint = 0x20;
// BD71815_REG_CC_CURCD
pub const CURDIR_Discharging: c_uint = 0x8000;
// BD71815_REG_VM_SA_IBAT
pub const IBAT_SA_DIR_Discharging: c_uint = 0x8000;
// BD71815_REG_REX_CTRL_1 bits

// BD71815_REG_REX_CTRL_1 bits

// BD71815_REG_LED_CTRL bits

