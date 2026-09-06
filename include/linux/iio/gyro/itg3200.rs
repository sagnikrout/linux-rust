//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/gyro/itg3200.h
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
// itg3200.h -- support InvenSense ITG3200
// Digital 3-Axis Gyroscope driver
//
// Copyright (c) 2011 Christian Strobel <christian.strobel@iis.fraunhofer.de>
// Copyright (c) 2011 Manuel Stahl <manuel.stahl@iis.fraunhofer.de>
// Copyright (c) 2012 Thorsten Nowak <thorsten.nowak@iis.fraunhofer.de>
//

// Register with I2C address (34h)
pub const ITG3200_REG_ADDRESS: c_uint = 0x00;
// Sample rate divider
// Range: 0 to 255
// Default value: 0x00
pub const ITG3200_REG_SAMPLE_RATE_DIV: c_uint = 0x15;
// Digital low pass filter settings
pub const ITG3200_REG_DLPF: c_uint = 0x16;
// DLPF full scale range
pub const ITG3200_DLPF_FS_SEL_2000: c_uint = 0x18;
// Bandwidth (Hz) and internal sample rate
// (kHz) of DLPF
pub const ITG3200_DLPF_256_8: c_uint = 0x00;
pub const ITG3200_DLPF_188_1: c_uint = 0x01;
pub const ITG3200_DLPF_98_1: c_uint = 0x02;
pub const ITG3200_DLPF_42_1: c_uint = 0x03;
pub const ITG3200_DLPF_20_1: c_uint = 0x04;
pub const ITG3200_DLPF_10_1: c_uint = 0x05;
pub const ITG3200_DLPF_5_1: c_uint = 0x06;
pub const ITG3200_DLPF_CFG_MASK: c_uint = 0x07;
// Configuration for interrupt operations
pub const ITG3200_REG_IRQ_CONFIG: c_uint = 0x17;
// Logic level
pub const ITG3200_IRQ_ACTIVE_LOW: c_uint = 0x80;
pub const ITG3200_IRQ_ACTIVE_HIGH: c_uint = 0x00;
// Drive type
pub const ITG3200_IRQ_OPEN_DRAIN: c_uint = 0x40;
pub const ITG3200_IRQ_PUSH_PULL: c_uint = 0x00;
// Latch mode
pub const ITG3200_IRQ_LATCH_UNTIL_CLEARED: c_uint = 0x20;
pub const ITG3200_IRQ_LATCH_50US_PULSE: c_uint = 0x00;
// Latch clear method
pub const ITG3200_IRQ_LATCH_CLEAR_ANY: c_uint = 0x10;
pub const ITG3200_IRQ_LATCH_CLEAR_STATUS: c_uint = 0x00;
// Enable interrupt when device is ready
pub const ITG3200_IRQ_DEVICE_RDY_ENABLE: c_uint = 0x04;
// Enable interrupt when data is available
pub const ITG3200_IRQ_DATA_RDY_ENABLE: c_uint = 0x01;
// Determine the status of ITG-3200 interrupts
pub const ITG3200_REG_IRQ_STATUS: c_uint = 0x1A;
// Status of 'device is ready'-interrupt
pub const ITG3200_IRQ_DEVICE_RDY_STATUS: c_uint = 0x04;
// Status of 'data is available'-interrupt
pub const ITG3200_IRQ_DATA_RDY_STATUS: c_uint = 0x01;
// Sensor registers
pub const ITG3200_REG_TEMP_OUT_H: c_uint = 0x1B;
pub const ITG3200_REG_TEMP_OUT_L: c_uint = 0x1C;
pub const ITG3200_REG_GYRO_XOUT_H: c_uint = 0x1D;
pub const ITG3200_REG_GYRO_XOUT_L: c_uint = 0x1E;
pub const ITG3200_REG_GYRO_YOUT_H: c_uint = 0x1F;
pub const ITG3200_REG_GYRO_YOUT_L: c_uint = 0x20;
pub const ITG3200_REG_GYRO_ZOUT_H: c_uint = 0x21;
pub const ITG3200_REG_GYRO_ZOUT_L: c_uint = 0x22;
// Power management
pub const ITG3200_REG_POWER_MANAGEMENT: c_uint = 0x3E;
// Reset device and internal registers to the
// power-up-default settings
pub const ITG3200_RESET: c_uint = 0x80;
// Enable low power sleep mode
pub const ITG3200_SLEEP: c_uint = 0x40;
// Put according gyroscope in standby mode
pub const ITG3200_STANDBY_GYRO_X: c_uint = 0x20;
pub const ITG3200_STANDBY_GYRO_Y: c_uint = 0x10;
pub const ITG3200_STANDBY_GYRO_Z: c_uint = 0x08;
// Determine the device clock source
pub const ITG3200_CLK_INTERNAL: c_uint = 0x00;
pub const ITG3200_CLK_GYRO_X: c_uint = 0x01;
pub const ITG3200_CLK_GYRO_Y: c_uint = 0x02;
pub const ITG3200_CLK_GYRO_Z: c_uint = 0x03;
pub const ITG3200_CLK_EXT_32K: c_uint = 0x04;
pub const ITG3200_CLK_EXT_19M: c_uint = 0x05;
//
// struct itg3200 - device instance specific data
// @i2c:    actual i2c_client
// @trig:   data ready trigger from itg3200 pin
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct itg3200 {
    pub i2c: *mut i2c_client,
    pub trig: *mut iio_trigger,
    pub orientation: iio_mount_matrix,
// lock to protect against multiple access to the device
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ITG3200_SCAN_INDEX {
    ITG3200_SCAN_TEMP,
    ITG3200_SCAN_GYRO_X,
    ITG3200_SCAN_GYRO_Y,
    ITG3200_SCAN_GYRO_Z,
    ITG3200_SCAN_ELEMENTS,
}

extern "C" {
    pub fn itg3200_remove_trigger(indio_dev: *mut iio_dev);
}
extern "C" {
    pub fn itg3200_probe_trigger(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn itg3200_buffer_configure(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn itg3200_buffer_unconfigure(indio_dev: *mut iio_dev);
}

