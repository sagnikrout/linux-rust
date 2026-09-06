//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/samsung/core.h
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
// Copyright (c) 2011 Samsung Electronics Co., Ltd
// http://www.samsung.com
//
// Macros to represent minimum voltages for LDO/BUCK
pub const MIN_3000_MV: c_int = 3000000;
pub const MIN_2500_MV: c_int = 2500000;
pub const MIN_2000_MV: c_int = 2000000;
pub const MIN_1800_MV: c_int = 1800000;
pub const MIN_1500_MV: c_int = 1500000;
pub const MIN_1400_MV: c_int = 1400000;
pub const MIN_1000_MV: c_int = 1000000;
pub const MIN_900_MV: c_int = 900000;
pub const MIN_850_MV: c_int = 850000;
pub const MIN_800_MV: c_int = 800000;
pub const MIN_750_MV: c_int = 750000;
pub const MIN_650_MV: c_int = 650000;
pub const MIN_600_MV: c_int = 600000;
pub const MIN_500_MV: c_int = 500000;
// Ramp delay in uV/us
pub const RAMP_DELAY_12_MVUS: c_int = 12000;
// Macros to represent steps for LDO/BUCK
pub const STEP_50_MV: c_int = 50000;
pub const STEP_25_MV: c_int = 25000;
pub const STEP_12_5_MV: c_int = 12500;
pub const STEP_6_25_MV: c_int = 6250;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_device_type {
    S5M8767X,
    S2DOS05,
    S2MPA01,
    S2MPG10,
    S2MPG11,
    S2MPS11X,
    S2MPS13X,
    S2MPS14X,
    S2MPS15X,
    S2MPU02,
    S2MPU05,
    S2MU005,
}

//
// struct sec_pmic_dev - s2m/s5m master device for sub-drivers
// @dev:		Master device of the chip
// @pdata:		Platform data populated with data from DTS
// or board files
// @regmap_pmic:	Regmap associated with PMIC's I2C address
// @i2c:		I2C client of the main driver
// @device_type:	Type of device, matches enum sec_device_type
// @irq_base:		Base IRQ number for device, required for IRQs
// @irq:		Generic IRQ number for device
// @irq_data:		Runtime data structure for IRQ controller
// @wakeup:		Whether or not this is a wakeup device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_pmic_dev {
    pub dev: *mut device,
    pub pdata: *mut sec_platform_data,
    pub regmap_pmic: *mut regmap,
    pub i2c: *mut i2c_client,
    pub device_type: c_int,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_platform_data {
    pub regulators: *mut sec_regulator_data,
    pub opmode: *mut sec_opmode_data,
    pub num_regulators: c_int,
    pub buck_gpios: [c_int; 3],
    pub buck_ds: [c_int; 3],
    pub buck2_voltage: [c_uint; 8],
    pub buck2_gpiodvs: bool,
    pub buck3_voltage: [c_uint; 8],
    pub buck3_gpiodvs: bool,
    pub buck4_voltage: [c_uint; 8],
    pub buck4_gpiodvs: bool,
    pub buck_default_idx: c_int,
    pub buck_ramp_delay: c_int,
    pub buck2_ramp_enable: bool,
    pub buck3_ramp_enable: bool,
    pub buck4_ramp_enable: bool,
    pub buck2_init: c_int,
    pub buck3_init: c_int,
    pub buck4_init: c_int,
// Whether or not manually set PWRHOLD to low during shutdown.
    pub manual_poweroff: bool,
// Disable the WRSTBI (buck voltage warm reset) when probing?
    pub disable_wrstbi: bool,
}

//
// sec_regulator_data - regulator data
// @id: regulator id
// @initdata: regulator init data (contraints, supplies, ...)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_regulator_data {
    pub id: c_int,
    pub initdata: *mut regulator_init_data,
    pub reg_node: *mut device_node,
    pub ext_control_gpiod: *mut gpio_desc,
}

//
// sec_opmode_data - regulator operation mode data
// @id: regulator id
// @mode: regulator operation mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_opmode_data {
    pub id: c_int,
    pub mode: c_uint,
}

//
// samsung regulator operation mode
// SEC_OPMODE_OFF	Regulator always OFF
// SEC_OPMODE_ON	Regulator always ON
// SEC_OPMODE_LOWPOWER  Regulator is on in low-power mode
// SEC_OPMODE_SUSPEND   Regulator is changed by PWREN pin
// If PWREN is high, regulator is on
// If PWREN is low, regulator is off
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_opmode {
    SEC_OPMODE_OFF,
    SEC_OPMODE_ON,
    SEC_OPMODE_LOWPOWER,
    SEC_OPMODE_SUSPEND,
}
