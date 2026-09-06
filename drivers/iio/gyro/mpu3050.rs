//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/gyro/mpu3050.h
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
// enum mpu3050_fullscale - indicates the full range of the sensor in deg/sec
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpu3050_fullscale {
    FS_250_DPS = 0,
    FS_500_DPS,
    FS_1000_DPS,
    FS_2000_DPS,
}

//
// enum mpu3050_lpf - indicates the low pass filter width
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpu3050_lpf {
// This implicity sets sample frequency to 8 kHz
    LPF_256_HZ_NOLPF = 0,
// All others sets the sample frequency to 1 kHz
    LPF_188_HZ,
    LPF_98_HZ,
    LPF_42_HZ,
    LPF_20_HZ,
    LPF_10_HZ,
    LPF_5_HZ,
    LPF_2100_HZ_NOLPF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpu3050_axis {
    AXIS_X = 0,
    AXIS_Y,
    AXIS_Z,
    AXIS_MAX,
}

//
// struct mpu3050 - instance state container for the device
// @dev: parent device for this instance
// @orientation: mounting matrix, flipped axis etc
// @map: regmap to reach the registers
// @lock: serialization lock to marshal all requests
// @irq: the IRQ used for this device
// @regs: the regulators to power this device
// @fullscale: the current fullscale setting for the device
// @lpf: digital low pass filter setting for the device
// @divisor: base frequency divider: divides 8 or 1 kHz
// @calibration: the three signed 16-bit calibration settings that
// get written into the offset registers for each axis to compensate
// for DC offsets
// @trig: trigger for the MPU-3050 interrupt, if present
// @hw_irq_trigger: hardware interrupt trigger is in use
// @irq_actl: interrupt is active low
// @irq_latch: latched IRQ, this means that it is a level IRQ
// @irq_opendrain: the interrupt line shall be configured open drain
// @pending_fifo_footer: tells us if there is a pending footer in the FIFO
// that we have to read out first when handling the FIFO
// @hw_timestamp: latest hardware timestamp from the trigger IRQ, when in
// use
// @i2cmux: an I2C mux reflecting the fact that this sensor is a hub with
// a pass-through I2C interface coming out of it: this device needs to be
// powered up in order to reach devices on the other side of this mux
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpu3050 {
    pub dev: *mut device,
    pub orientation: iio_mount_matrix,
    pub map: *mut regmap,
    pub lock: mutex,
    pub irq: c_int,
    pub regs: [regulator_bulk_data; 2],
    pub fullscale: mpu3050_fullscale,
    pub lpf: mpu3050_lpf,
    pub divisor: u8,
    pub calibration: [i16; 3],
    pub trig: *mut iio_trigger,
    pub hw_irq_trigger: bool,
    pub irq_actl: bool,
    pub irq_latch: bool,
    pub irq_opendrain: bool,
    pub pending_fifo_footer: bool,
    pub hw_timestamp: i64,
    pub i2cmux: *mut i2c_mux_core,
}

// Probe called from different transports
extern "C" {
    pub fn mpu3050_common_remove(dev: *mut device);
}
// PM ops
