//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/accel/adxl313.h
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
// ADXL313 3-Axis Digital Accelerometer
//
// Copyright (c) 2021 Lucas Stankus <lucas.p.stankus@gmail.com>
//

// ADXL313 register definitions
pub const ADXL313_REG_DEVID0: c_uint = 0x00;
pub const ADXL313_REG_DEVID1: c_uint = 0x01;
pub const ADXL313_REG_PARTID: c_uint = 0x02;
pub const ADXL313_REG_XID: c_uint = 0x04;
pub const ADXL313_REG_SOFT_RESET: c_uint = 0x18;

pub const ADXL313_REG_THRESH_ACT: c_uint = 0x24;
pub const ADXL313_REG_THRESH_INACT: c_uint = 0x25;
pub const ADXL313_REG_TIME_INACT: c_uint = 0x26;
pub const ADXL313_REG_ACT_INACT_CTL: c_uint = 0x27;
pub const ADXL313_REG_BW_RATE: c_uint = 0x2C;
pub const ADXL313_REG_POWER_CTL: c_uint = 0x2D;
pub const ADXL313_REG_INT_ENABLE: c_uint = 0x2E;
pub const ADXL313_REG_INT_MAP: c_uint = 0x2F;
pub const ADXL313_REG_INT_SOURCE: c_uint = 0x30;
pub const ADXL313_REG_DATA_FORMAT: c_uint = 0x31;

pub const ADXL313_REG_FIFO_CTL: c_uint = 0x38;
pub const ADXL313_REG_FIFO_STATUS: c_uint = 0x39;
pub const ADXL313_DEVID0: c_uint = 0xAD;
pub const ADXL313_DEVID0_ADXL312_314: c_uint = 0xE5;
pub const ADXL313_DEVID1: c_uint = 0x1D;
pub const ADXL313_PARTID: c_uint = 0xCB;
pub const ADXL313_SOFT_RESET: c_uint = 0x52;

pub const ADXL313_RATE_BASE: c_int = 6;

pub const ADXL313_RANGE_MAX: c_int = 3;

// FIFO entries: how many values are stored in the FIFO

// FIFO samples: number of samples needed for watermark (FIFO mode)

pub const ADXL313_FIFO_BYPASS: c_int = 0;
pub const ADXL313_FIFO_STREAM: c_int = 2;
pub const ADXL313_FIFO_SIZE: c_int = 32;
pub const ADXL313_NUM_AXIS: c_int = 3;
extern "C" {
    pub fn adxl313_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adxl313_device_type {
    ADXL312,
    ADXL313,
    ADXL314,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adxl313_data {
    pub regmap: *mut regmap,
    pub chip_info: *const adxl313_chip_info,
    pub /: *mut *mut mutex lock; / lock to protect transf_buf,
    pub watermark: u8,
    pub __aligned(IIO_DMA_MINALIGN): __le16 transf_buf,
    pub 1]: *mut *mut __le16 fifo_buf[ADXL313_NUM_AXIS  ADXL313_FIFO_SIZE +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adxl313_chip_info {
    pub name: *const c_char,
    pub type: adxl313_device_type,
    pub scale_factor: c_int,
    pub variable_range: bool,
    pub soft_reset: bool,
    pub data): *mut *mut *mut int (check_id)(struct device dev, struct adxl313_data,
}
