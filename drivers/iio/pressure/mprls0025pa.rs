//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/pressure/mprls0025pa.h
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
// MPRLS0025PA - Honeywell MicroPressure pressure sensor series driver
//
// Copyright (c) Andreas Klinger <ak@it-klinger.de>
//
// Data sheet:
// https://prod-edam.honeywell.com/content/dam/honeywell-edam/sps/siot/en-us/products/sensors/pressure-sensors/board-mount-pressure-sensors/micropressure-mpr-series/documents/sps-siot-mpr-series-datasheet-32332628-ciid-172626.pdf
//

pub const MPR_MEASUREMENT_RD_SIZE: c_int = 4;
pub const MPR_CMD_NOP: c_uint = 0xf0;
pub const MPR_CMD_SYNC: c_uint = 0xaa;

pub const MPR_PKT_SYNC_LEN: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpr_func_id {
    MPR_FUNCTION_A,
    MPR_FUNCTION_B,
    MPR_FUNCTION_C,
}

//
// struct mpr_data
// @dev: current device structure
// @ops: functions that implement the sensor reads/writes, bus init
// @lock: access to device during read
// @pmin: minimal pressure in pascal
// @pmax: maximal pressure in pascal
// @function: transfer function
// @outmin: minimum raw pressure in counts (based on transfer function)
// @outmax: maximum raw pressure in counts (based on transfer function)
// @scale: pressure scale
// @scale2: pressure scale, decimal number
// @offset: pressure offset
// @gpiod_reset: reset
// @irq: end of conversion irq. used to distinguish between irq mode and
// reading in a loop until data is ready
// @completion: handshake from irq to read
// @chan: channel values for buffered mode
// @chan.pres: pressure value
// @chan.ts: timestamp
// @rx_buf: raw conversion data
// @tx_buf: output buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpr_data {
    pub dev: *mut device,
    pub ops: *const mpr_ops,
    pub lock: mutex,
    pub pmin: u32,
    pub pmax: u32,
    pub function: mpr_func_id,
    pub outmin: u32,
    pub outmax: u32,
    pub scale: c_int,
    pub scale2: c_int,
    pub offset: c_int,
    pub gpiod_reset: *mut gpio_desc,
    pub irq: c_int,
    pub completion: completion,
    pub pres: i32,
    pub ts: aligned_s64,
    pub chan: },
    pub __aligned(IIO_DMA_MINALIGN): u8 rx_buf[MPR_MEASUREMENT_RD_SIZE],
    pub tx_buf: [u8; MPR_MEASUREMENT_RD_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpr_ops {
    pub cnt): *const *const *const int (read)(struct mpr_data data, u8 cmd, u8,
    pub cnt): *const *const *const int (write)(struct mpr_data data, u8 cmd, u8,
}

extern "C" {
    pub fn mpr_common_probe(dev: *mut device, ops: *const mpr_ops, irq: c_int) -> c_int;
}
