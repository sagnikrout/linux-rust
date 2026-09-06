//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/88pm80x.h
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
// Marvell 88PM80x Interface
//
// Copyright (C) 2012 Marvell International Ltd.
// Qiao Zhou <zhouqiao@marvell.com>
//

// page 0 basic: slave adder 0x60

// Wakeup Registers

// Referance and low power registers

// GPIO register

pub const PM800_GPIO3_MODE_MASK: c_uint = 0x1F;

// PWM register

// RTC Registers

// bit definitions of RTC Register 1 (0xD0)

// Regulator Control Registers: BUCK1,BUCK5,LDO1 have DVC
// buck registers

// BUCK Sleep Mode Register 1: BUCK[1..4]

pub const PM800_BUCK1_SLP1_SHIFT: c_int = 0;

// page 2 GPADC: slave adder 0x02

pub const PM800_GPADC0_LOW_TH: c_uint = 0x20;
pub const PM800_GPADC1_LOW_TH: c_uint = 0x21;
pub const PM800_GPADC2_LOW_TH: c_uint = 0x22;
pub const PM800_GPADC3_LOW_TH: c_uint = 0x23;
pub const PM800_GPADC4_LOW_TH: c_uint = 0x24;
pub const PM800_GPADC0_UPP_TH: c_uint = 0x30;
pub const PM800_GPADC1_UPP_TH: c_uint = 0x31;
pub const PM800_GPADC2_UPP_TH: c_uint = 0x32;
pub const PM800_GPADC3_UPP_TH: c_uint = 0x33;
pub const PM800_GPADC4_UPP_TH: c_uint = 0x34;
pub const PM800_VBBAT_MEAS1: c_uint = 0x40;
pub const PM800_VBBAT_MEAS2: c_uint = 0x41;
pub const PM800_VBAT_MEAS1: c_uint = 0x42;
pub const PM800_VBAT_MEAS2: c_uint = 0x43;
pub const PM800_VSYS_MEAS1: c_uint = 0x44;
pub const PM800_VSYS_MEAS2: c_uint = 0x45;
pub const PM800_VCHG_MEAS1: c_uint = 0x46;
pub const PM800_VCHG_MEAS2: c_uint = 0x47;
pub const PM800_TINT_MEAS1: c_uint = 0x50;
pub const PM800_TINT_MEAS2: c_uint = 0x51;
pub const PM800_PMOD_MEAS1: c_uint = 0x52;
pub const PM800_PMOD_MEAS2: c_uint = 0x53;
pub const PM800_GPADC0_MEAS1: c_uint = 0x54;
pub const PM800_GPADC0_MEAS2: c_uint = 0x55;
pub const PM800_GPADC1_MEAS1: c_uint = 0x56;
pub const PM800_GPADC1_MEAS2: c_uint = 0x57;
pub const PM800_GPADC2_MEAS1: c_uint = 0x58;
pub const PM800_GPADC2_MEAS2: c_uint = 0x59;
pub const PM800_GPADC3_MEAS1: c_uint = 0x5A;
pub const PM800_GPADC3_MEAS2: c_uint = 0x5B;
pub const PM800_GPADC4_MEAS1: c_uint = 0x5C;
pub const PM800_GPADC4_MEAS2: c_uint = 0x5D;
pub const PM800_GPADC4_AVG1: c_uint = 0xA8;
pub const PM800_GPADC4_AVG2: c_uint = 0xA9;
// 88PM805 Registers

// number of status and int reg in a row

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm80x_rtc_pdata {
    pub vrtc: c_int,
    pub rtc_wakeup: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm80x_subchip {
    pub /: *mut *mut *mut i2c_client power_page; / chip client for power page,
    pub /: *mut *mut *mut i2c_client gpadc_page; / chip client for gpadc page,
    pub regmap_power: *mut regmap,
    pub regmap_gpadc: *mut regmap,
    pub /: *mut *mut unsigned short power_page_addr; / power page I2C address,
    pub /: *mut *mut unsigned short gpadc_page_addr; / gpadc page I2C address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm80x_chip {
    pub subchip: *mut pm80x_subchip,
    pub dev: *mut device,
    pub client: *mut i2c_client,
    pub companion: *mut i2c_client,
    pub regmap: *mut regmap,
    pub regmap_irq_chip: *const regmap_irq_chip,
    pub irq_data: *mut regmap_irq_chip_data,
    pub type: c_int,
    pub irq: c_int,
    pub irq_mode: c_int,
    pub wu_flag: c_ulong,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm80x_platform_data {
    pub rtc: *mut pm80x_rtc_pdata,
//
// For the regulator not defined, set regulators[not_defined] to be
// NULL. num_regulators are the number of regulators supposed to be
// initialized. If all regulators are not defined, set num_regulators
// to be 0.
//
    pub regulators: [*mut regulator_init_data; PM800_ID_RG_MAX],
    pub num_regulators: c_uint,
    pub /: *mut *mut int irq_mode; / Clear interrupt by read/write(0/1),
    pub /: *mut *mut int batt_det; / enable/disable,
    pub pdata): *mut pm80x_platform_data,
}

extern "C" {
    pub fn pm80x_init(client: *mut i2c_client) -> c_int;
}
extern "C" {
    pub fn pm80x_deinit() -> c_int;
}
