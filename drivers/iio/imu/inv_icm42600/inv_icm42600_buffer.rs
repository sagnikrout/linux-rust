//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/imu/inv_icm42600/inv_icm42600_buffer.h
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
// Copyright (C) 2020 Invensense, Inc.
//

//
// struct inv_icm42600_fifo - FIFO state variables
// @on:		reference counter for FIFO on.
// @en:		bits field of INV_ICM42600_SENSOR_* for FIFO EN bits.
// @period:	FIFO internal period.
// @watermark:	watermark configuration values for accel and gyro.
// @count:	number of bytes in the FIFO data buffer.
// @nb:		gyro, accel and total samples in the FIFO data buffer.
// @data:	FIFO data buffer aligned for DMA (2kB + 32 bytes of read cache).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42600_fifo {
    pub on: c_uint,
    pub en: c_uint,
    pub period: u32,
    pub gyro: c_uint,
    pub accel: c_uint,
    pub eff_gyro: c_uint,
    pub eff_accel: c_uint,
    pub value: c_uint,
    pub watermark: },
    pub count: usize,
    pub gyro: usize,
    pub accel: usize,
    pub total: usize,
    pub nb: },
    pub __aligned(IIO_DMA_MINALIGN): u8 data[2080],
}

// FIFO data packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42600_fifo_sensor_data {
    pub x: __be16,
    pub y: __be16,
    pub z: __be16,
    pub __packed: },

    pub be16_to_cpu(d): return,
    pub z: s16 x, y,,
    pub inv_icm42600_fifo_get_sensor_data(s->x): x =,
    pub inv_icm42600_fifo_get_sensor_data(s->y): y =,
    pub inv_icm42600_fifo_get_sensor_data(s->z): z =,
    pub false: return,
    pub true: return,
    pub odr): *const *const *const void timestamp, unsigned int,
    pub inv_icm42600_buffer_ops: extern struct iio_buffer_setup_ops,
    pub inv_icm42600_buffer_attrs: [*const extern struct iio_dev_attr; ],
    pub st): *mut int inv_icm42600_buffer_init(struct inv_icm42600_state,
    pub st): *mut void inv_icm42600_buffer_update_fifo_period(struct inv_icm42600_state,
    pub fifo_en): c_uint,
    pub st): *mut int inv_icm42600_buffer_update_watermark(struct inv_icm42600_state,
    pub max): c_uint,
    pub st): *mut int inv_icm42600_buffer_fifo_parse(struct inv_icm42600_state,
    pub count): c_uint,
