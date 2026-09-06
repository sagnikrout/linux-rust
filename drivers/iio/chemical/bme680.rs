//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/chemical/bme680.h
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

pub const BME680_REG_CHIP_ID: c_uint = 0xD0;
pub const BME680_CHIP_ID_VAL: c_uint = 0x61;
pub const BME680_REG_SOFT_RESET: c_uint = 0xE0;
pub const BME680_CMD_SOFTRESET: c_uint = 0xB6;
pub const BME680_REG_STATUS: c_uint = 0x73;

pub const BME680_SPI_MEM_PAGE_1_VAL: c_int = 1;
pub const BME680_REG_TEMP_MSB: c_uint = 0x22;
pub const BME680_REG_PRESS_MSB: c_uint = 0x1F;
pub const BME680_REG_HUMIDITY_MSB: c_uint = 0x25;
pub const BME680_REG_GAS_MSB: c_uint = 0x2A;
pub const BME680_REG_GAS_R_LSB: c_uint = 0x2B;

pub const BME680_REG_CTRL_HUMIDITY: c_uint = 0x72;

pub const BME680_REG_CTRL_MEAS: c_uint = 0x74;

pub const BME680_REG_CONFIG: c_uint = 0x75;

// TEMP/PRESS/HUMID reading skipped
pub const BME680_MEAS_SKIPPED: c_uint = 0x8000;
pub const BME680_MAX_OVERFLOW_VAL: c_uint = 0x40000000;
pub const BME680_HUM_REG_SHIFT_VAL: c_int = 4;

pub const BME680_REG_RES_HEAT_VAL: c_uint = 0x00;

pub const BME680_REG_IDAC_HEAT_0: c_uint = 0x50;
pub const BME680_REG_RES_HEAT_0: c_uint = 0x5A;
pub const BME680_REG_GAS_WAIT_0: c_uint = 0x64;

pub const BME680_AMB_TEMP: c_int = 25;
pub const BME680_REG_CTRL_GAS_1: c_uint = 0x71;

pub const BME680_REG_MEAS_STAT_0: c_uint = 0x1D;

pub const BME680_TEMP_NUM_BYTES: c_int = 3;
pub const BME680_PRESS_NUM_BYTES: c_int = 3;
pub const BME680_HUMID_NUM_BYTES: c_int = 2;
pub const BME680_GAS_NUM_BYTES: c_int = 2;

// Datasheet Section 1.1, Table 1
pub const BME680_STARTUP_TIME_US: c_int = 2000;
pub const BME680_NUM_CHANNELS: c_int = 4;
pub const BME680_NUM_BULK_READ_REGS: c_int = 15;
// Calibration Parameters
pub const BME680_T2_LSB_REG: c_uint = 0x8A;
pub const BME680_H2_MSB_REG: c_uint = 0xE1;
pub const BME680_GH3_REG: c_uint = 0xEE;
pub const BME680_CALIB_RANGE_1_LEN: c_int = 23;
pub const BME680_CALIB_RANGE_2_LEN: c_int = 14;
pub const BME680_CALIB_RANGE_3_LEN: c_int = 5;
