//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwmon/eic7700-pvt.h
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
// ESWIN EIC7700 Voltage, Temperature sensor driver
//
// Copyright 2026, Beijing ESWIN Computing Technology Co., Ltd.
//

// ESWIN EIC7700 PVT registers and their bitfields
pub const PVT_TRIM: c_uint = 0x04;
pub const PVT_MODE: c_uint = 0x08;

pub const PVT_CTRL_MODE_TEMP: c_uint = 0x0;
pub const PVT_CTRL_MODE_VOLT: c_uint = 0x4;
pub const PVT_ENA: c_uint = 0x0c;

pub const PVT_INT: c_uint = 0x10;

pub const PVT_DATA: c_uint = 0x14;

//
// PVT sensors-related limits and default values
// @PVT_TEMP_CHS: Number of temperature hwmon channels.
// @PVT_VOLT_CHS: Number of voltage hwmon channels.
// @PVT_TRIM_DEF: Default temperature sensor trim value (set a proper value
// when one is determined for ESWIN EIC7700 SoC).
// @PVT_TOUT_MIN: Minimal timeout between samples in nanoseconds.
//
pub const PVT_TEMP_CHS: c_int = 1;
pub const PVT_VOLT_CHS: c_int = 1;
pub const PVT_TRIM_DEF: c_int = 0;

//
// enum pvt_sensor_type - ESWIN EIC7700 PVT sensor types (correspond to each PVT
// sampling mode)
// @PVT_TEMP: PVT Temperature sensor.
// @PVT_VOLT: PVT Voltage sensor.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvt_sensor_type {
    PVT_TEMP = 0,
    PVT_VOLT
}

pub const PVT_CLK_NUM: c_int = 2;
//
// struct pvt_sensor_info - ESWIN EIC7700 PVT sensor informational structure
// @channel: Sensor channel ID.
// @label: hwmon sensor label.
// @mode: PVT mode corresponding to the channel.
// @type: Sensor type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvt_sensor_info {
    pub channel: c_int,
    pub label: *const c_char,
    pub mode: u32,
    pub type: hwmon_sensor_types,
}

//
// struct pvt_hwmon - Eswin EIC7700 PVT private data
// @dev: device structure of the PVT platform device.
// @hwmon: hwmon device structure.
// @regs: pointer to the Eswin EIC7700 PVT registers region.
// @irq: PVT events IRQ number.
// @clks: PVT clock descriptors.
// @data_cache: data cache in raw format.
// @conversion: data conversion completion.
// @timeout: conversion timeout.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvt_hwmon {
    pub dev: *mut device,
    pub hwmon: *mut device,
    pub regs: *mut void __iomem,
    pub irq: c_int,
    pub clks: [clk_bulk_data; PVT_CLK_NUM],
    pub data_cache: u32,
    pub conversion: completion,
    pub timeout: ktime_t,
}
