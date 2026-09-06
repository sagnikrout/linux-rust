//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/st/st_thermal.h
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
// ST Thermal Sensor Driver for STi series of SoCs
// Author: Ajit Pal Singh <ajitpal.singh@st.com>
//
// Copyright (C) 2003-2014 STMicroelectronics (R&D) Limited
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum st_thermal_regfield_ids {
    INT_THRESH_HI = 0, /* Top two regfield IDs are mutually exclusive */
    TEMP_PWR = 0,
    DCORRECT,
    OVERFLOW,
    DATA,
    INT_ENABLE,

    MAX_REGFIELDS
}

// Thermal sensor power states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum st_thermal_power_state {
    POWER_OFF = 0,
    POWER_ON
}

//
// Description of private thermal sensor ops.
//
// @power_ctrl:		Function for powering on/off a sensor. Clock to the
// sensor is also controlled from this function.
// @alloc_regfields:	Allocate regmap register fields, specific to a sensor.
// @do_memmap_regmap:	Memory map the thermal register space and init regmap
// instance or find regmap instance.
// @register_irq:	Register an interrupt handler for a sensor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_thermal_sensor_ops {
    pub st_thermal_power_state): *mut *mut *mut int (power_ctrl)(struct st_thermal_sensor , enum,
    pub ): *mut *mut int (alloc_regfields)(struct st_thermal_sensor,
    pub ): *mut *mut int (regmap_init)(struct st_thermal_sensor,
    pub ): *mut *mut int (register_enable_irq)(struct st_thermal_sensor,
    pub ): *mut *mut int (enable_irq)(struct st_thermal_sensor,
}

//
// Description of thermal driver compatible data.
//
// @reg_fields:		Pointer to the regfields array for a sensor.
// @sys_compat:		Pointer to the syscon node compatible string.
// @ops:		Pointer to private thermal ops for a sensor.
// @calibration_val:	Default calibration value to be written to the DCORRECT
// register field for a sensor.
// @temp_adjust_val:	Value to be added/subtracted from the data read from
// the sensor. If value needs to be added please provide a
// positive value and if it is to be subtracted please
// provide a negative value.
// @crit_temp:		The temperature beyond which the SoC should be shutdown
// to prevent damage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_thermal_compat_data {
    pub sys_compat: *mut c_char,
    pub reg_fields: *const reg_field,
    pub ops: *const st_thermal_sensor_ops,
    pub calibration_val: c_uint,
    pub temp_adjust_val: c_int,
    pub crit_temp: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_thermal_sensor {
    pub dev: *mut device,
    pub thermal_dev: *mut thermal_zone_device,
    pub ops: *const st_thermal_sensor_ops,
    pub cdata: *const st_thermal_compat_data,
    pub clk: *mut clk,
    pub regmap: *mut regmap,
    pub pwr: *mut regmap_field,
    pub dcorrect: *mut regmap_field,
    pub overflow: *mut regmap_field,
    pub temp_data: *mut regmap_field,
    pub int_thresh_hi: *mut regmap_field,
    pub int_enable: *mut regmap_field,
    pub irq: c_int,
    pub mmio_base: *mut void __iomem,
}

extern "C" {
    pub fn st_thermal_unregister(pdev: *mut platform_device);
}
