//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/fimc-is-sensor.h
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
// Samsung EXYNOS4x12 FIMC-IS (Imaging Subsystem) driver
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
//
// Authors:  Sylwester Nawrocki <s.nawrocki@samsung.com>
// Younghwan Joo <yhwan.joo@samsung.com>
//

pub const S5K6A3_SENSOR_WIDTH: c_int = 1392;
pub const S5K6A3_SENSOR_HEIGHT: c_int = 1392;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fimc_is_sensor_id {
    FIMC_IS_SENSOR_ID_S5K3H2 = 1,
    FIMC_IS_SENSOR_ID_S5K6A3,
    FIMC_IS_SENSOR_ID_S5K4E5,
    FIMC_IS_SENSOR_ID_S5K3H7,
    FIMC_IS_SENSOR_ID_CUSTOM,
    FIMC_IS_SENSOR_ID_END
}

pub const IS_SENSOR_CTRL_BUS_I2C0: c_int = 0;
pub const IS_SENSOR_CTRL_BUS_I2C1: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sensor_drv_data {
    pub id: fimc_is_sensor_id,
// sensor open timeout in ms
    pub open_timeout: c_ushort,
}

//
// struct fimc_is_sensor - fimc-is sensor data structure
// @drvdata: a pointer to the sensor's parameters data structure
// @i2c_bus: ISP I2C bus index (0...1)
// @test_pattern: true to enable video test pattern
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_is_sensor {
    pub drvdata: *const sensor_drv_data,
    pub i2c_bus: c_uint,
    pub test_pattern: u8,
}
