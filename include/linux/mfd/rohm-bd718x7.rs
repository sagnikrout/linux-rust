//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rohm-bd718x7.h
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
// Copyright (C) 2018 ROHM Semiconductors

// Common voltage configurations
pub const BD718XX_DVS_BUCK_VOLTAGE_NUM: c_uint = 0x3D;
pub const BD718XX_4TH_NODVS_BUCK_VOLTAGE_NUM: c_uint = 0x3D;
pub const BD718XX_LDO1_VOLTAGE_NUM: c_uint = 0x08;
pub const BD718XX_LDO2_VOLTAGE_NUM: c_uint = 0x02;
pub const BD718XX_LDO3_VOLTAGE_NUM: c_uint = 0x10;
pub const BD718XX_LDO4_VOLTAGE_NUM: c_uint = 0x0A;
pub const BD718XX_LDO6_VOLTAGE_NUM: c_uint = 0x0A;
// BD71837 specific voltage configurations
pub const BD71837_BUCK5_VOLTAGE_NUM: c_uint = 0x10;
pub const BD71837_BUCK6_VOLTAGE_NUM: c_uint = 0x04;
pub const BD71837_BUCK7_VOLTAGE_NUM: c_uint = 0x08;
pub const BD71837_LDO5_VOLTAGE_NUM: c_uint = 0x10;
pub const BD71837_LDO7_VOLTAGE_NUM: c_uint = 0x10;
// BD71847 specific voltage configurations
pub const BD71847_BUCK3_VOLTAGE_NUM: c_uint = 0x18;
pub const BD71847_BUCK4_VOLTAGE_NUM: c_uint = 0x08;
pub const BD71847_LDO5_VOLTAGE_NUM: c_uint = 0x20;
// Registers specific to BD71837
// Registers common for BD71837 and BD71847
pub const REGLOCK_PWRSEQ: c_uint = 0x1;
pub const REGLOCK_VREG: c_uint = 0x10;
// Generic BUCK control masks
pub const BD718XX_BUCK_SEL: c_uint = 0x02;
pub const BD718XX_BUCK_EN: c_uint = 0x01;
pub const BD718XX_BUCK_RUN_ON: c_uint = 0x04;
// Generic LDO masks
pub const BD718XX_LDO_SEL: c_uint = 0x80;
pub const BD718XX_LDO_EN: c_uint = 0x40;
// BD71837 BUCK ramp rate CTRL reg bits
pub const BUCK_RAMPRATE_MASK: c_uint = 0xC0;
pub const BUCK_RAMPRATE_10P00MV: c_uint = 0x0;
pub const BUCK_RAMPRATE_5P00MV: c_uint = 0x1;
pub const BUCK_RAMPRATE_2P50MV: c_uint = 0x2;
pub const BUCK_RAMPRATE_1P25MV: c_uint = 0x3;
pub const DVS_BUCK_RUN_MASK: c_uint = 0x3F;
pub const DVS_BUCK_SUSP_MASK: c_uint = 0x3F;
pub const DVS_BUCK_IDLE_MASK: c_uint = 0x3F;
pub const BD718XX_1ST_NODVS_BUCK_MASK: c_uint = 0x07;
pub const BD718XX_3RD_NODVS_BUCK_MASK: c_uint = 0x07;
pub const BD718XX_4TH_NODVS_BUCK_MASK: c_uint = 0x3F;
pub const BD71847_BUCK3_MASK: c_uint = 0x07;
pub const BD71847_BUCK3_RANGE_MASK: c_uint = 0xC0;
pub const BD71847_BUCK4_MASK: c_uint = 0x03;
pub const BD71847_BUCK4_RANGE_MASK: c_uint = 0x40;
pub const BD71837_BUCK5_MASK: c_uint = 0x07;
pub const BD71837_BUCK5_RANGE_MASK: c_uint = 0x80;
pub const BD71837_BUCK6_MASK: c_uint = 0x03;
pub const BD718XX_LDO1_MASK: c_uint = 0x03;
pub const BD718XX_LDO1_RANGE_MASK: c_uint = 0x20;
pub const BD718XX_LDO2_MASK: c_uint = 0x20;
pub const BD718XX_LDO3_MASK: c_uint = 0x0F;
pub const BD718XX_LDO4_MASK: c_uint = 0x0F;
pub const BD718XX_LDO6_MASK: c_uint = 0x0F;
pub const BD71837_LDO5_MASK: c_uint = 0x0F;
pub const BD71847_LDO5_MASK: c_uint = 0x0F;
pub const BD71847_LDO5_RANGE_MASK: c_uint = 0x20;
pub const BD71837_LDO7_MASK: c_uint = 0x0F;
// BD718XX Voltage monitoring masks
pub const BD718XX_BUCK1_VRMON80: c_uint = 0x1;
pub const BD718XX_BUCK1_VRMON130: c_uint = 0x2;
pub const BD718XX_BUCK2_VRMON80: c_uint = 0x4;
pub const BD718XX_BUCK2_VRMON130: c_uint = 0x8;
pub const BD718XX_1ST_NODVS_BUCK_VRMON80: c_uint = 0x1;
pub const BD718XX_1ST_NODVS_BUCK_VRMON130: c_uint = 0x2;
pub const BD718XX_2ND_NODVS_BUCK_VRMON80: c_uint = 0x4;
pub const BD718XX_2ND_NODVS_BUCK_VRMON130: c_uint = 0x8;
pub const BD718XX_3RD_NODVS_BUCK_VRMON80: c_uint = 0x10;
pub const BD718XX_3RD_NODVS_BUCK_VRMON130: c_uint = 0x20;
pub const BD718XX_4TH_NODVS_BUCK_VRMON80: c_uint = 0x40;
pub const BD718XX_4TH_NODVS_BUCK_VRMON130: c_uint = 0x80;
pub const BD718XX_LDO1_VRMON80: c_uint = 0x1;
pub const BD718XX_LDO2_VRMON80: c_uint = 0x2;
pub const BD718XX_LDO3_VRMON80: c_uint = 0x4;
pub const BD718XX_LDO4_VRMON80: c_uint = 0x8;
pub const BD718XX_LDO5_VRMON80: c_uint = 0x10;
pub const BD718XX_LDO6_VRMON80: c_uint = 0x20;
// BD71837 specific voltage monitoring masks
pub const BD71837_BUCK3_VRMON80: c_uint = 0x10;
pub const BD71837_BUCK3_VRMON130: c_uint = 0x20;
pub const BD71837_BUCK4_VRMON80: c_uint = 0x40;
pub const BD71837_BUCK4_VRMON130: c_uint = 0x80;
pub const BD71837_LDO7_VRMON80: c_uint = 0x40;
// BD718XX_REG_IRQ bits
pub const IRQ_SWRST: c_uint = 0x40;
pub const IRQ_PWRON_S: c_uint = 0x20;
pub const IRQ_PWRON_L: c_uint = 0x10;
pub const IRQ_PWRON: c_uint = 0x08;
pub const IRQ_WDOG: c_uint = 0x04;
pub const IRQ_ON_REQ: c_uint = 0x02;
pub const IRQ_STBY_REQ: c_uint = 0x01;
// ROHM BD718XX irqs
// ROHM BD718XX interrupt masks
pub const BD718XX_INT_SWRST_MASK: c_uint = 0x40;
pub const BD718XX_INT_PWRBTN_S_MASK: c_uint = 0x20;
pub const BD718XX_INT_PWRBTN_L_MASK: c_uint = 0x10;
pub const BD718XX_INT_PWRBTN_MASK: c_uint = 0x8;
pub const BD718XX_INT_WDOG_MASK: c_uint = 0x4;
pub const BD718XX_INT_ON_REQ_MASK: c_uint = 0x2;
pub const BD718XX_INT_STBY_REQ_MASK: c_uint = 0x1;
// Register write induced reset settings
//
// Even though the bit zero is not SWRESET type we still want to write zero
// to it when changing type. Bit zero is 'SWRESET' trigger bit and if we
// write 1 to it we will trigger the action. So always write 0 to it when
// changning SWRESET action - no matter what we read from it.
//
pub const BD718XX_SWRESET_TYPE_MASK: c_int = 7;
pub const BD718XX_SWRESET_TYPE_DISABLED: c_int = 0;
pub const BD718XX_SWRESET_TYPE_COLD: c_int = 4;
pub const BD718XX_SWRESET_TYPE_WARM: c_int = 6;
pub const BD718XX_SWRESET_RESET_MASK: c_int = 1;
pub const BD718XX_SWRESET_RESET: c_int = 1;
// Poweroff state transition conditions
pub const BD718XX_ON_REQ_POWEROFF_MASK: c_int = 1;
pub const BD718XX_SWRESET_POWEROFF_MASK: c_int = 2;
pub const BD718XX_WDOG_POWEROFF_MASK: c_int = 4;
pub const BD718XX_KEY_L_POWEROFF_MASK: c_int = 8;
pub const BD718XX_POWOFF_TO_SNVS: c_int = 0;
pub const BD718XX_POWOFF_TO_RDY: c_uint = 0xF;
pub const BD718XX_POWOFF_TIME_MASK: c_uint = 0xF0;
// Poweron sequence state transition conditions
pub const BD718XX_RDY_TO_SNVS_MASK: c_uint = 0xF;
pub const BD718XX_SNVS_TO_RUN_MASK: c_uint = 0xF0;
pub const BD718XX_PWR_TRIG_KEY_L: c_int = 1;
pub const BD718XX_PWR_TRIG_KEY_S: c_int = 2;
pub const BD718XX_PWR_TRIG_PMIC_ON: c_int = 4;
pub const BD718XX_PWR_TRIG_VSYS_UVLO: c_int = 8;
pub const BD718XX_RDY_TO_SNVS_SIFT: c_int = 0;
pub const BD718XX_SNVS_TO_RUN_SIFT: c_int = 4;
pub const BD718XX_PWRBTN_PRESS_DURATION_MASK: c_uint = 0xF;
// Timeout value for detecting short press
// Timeout value for detecting LONG press
