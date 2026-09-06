//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/adc/ad7091r-base.h
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
// AD7091RX Analog to Digital converter driver
//
// Copyright 2014-2019 Analog Devices Inc.
//

pub const AD7091R_REG_RESULT: c_int = 0;
pub const AD7091R_REG_CHANNEL: c_int = 1;
pub const AD7091R_REG_CONF: c_int = 2;
pub const AD7091R_REG_ALERT: c_int = 3;

// AD7091R_REG_RESULT

// AD7091R_REG_CONF

// AD7091R_REG_CH_LIMIT
pub const AD7091R_HIGH_LIMIT: c_uint = 0xFFF;
pub const AD7091R_LOW_LIMIT: c_uint = 0x0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad7091r_mode {
    AD7091R_MODE_SAMPLE,
    AD7091R_MODE_COMMAND,
    AD7091R_MODE_AUTOCYCLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7091r_state {
    pub dev: *mut device,
    pub map: *mut regmap,
    pub convst_gpio: *mut gpio_desc,
    pub reset_gpio: *mut gpio_desc,
    pub vref: *mut regulator,
    pub chip_info: *const ad7091r_chip_info,
    pub mode: ad7091r_mode,
    pub /: *mut *mut mutex lock; /lock to prevent concurrent reads,
    pub __aligned(IIO_DMA_MINALIGN): __be16 tx_buf,
    pub rx_buf: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7091r_chip_info {
    pub name: *const c_char,
    pub num_channels: c_uint,
    pub channels: *const iio_chan_spec,
    pub vref_mV: c_uint,
    pub val): *mut *mut unsigned int (reg_result_chan_id)(unsigned int,
    pub mode): *mut *mut *mut int (set_mode)(struct ad7091r_state st, enum ad7091r_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7091r_init_info {
    pub info_irq: *const ad7091r_chip_info,
    pub info_no_irq: *const ad7091r_chip_info,
    pub regmap_config: *const regmap_config,
    pub regmap_conf): *const regmap_config,
    pub st): *mut *mut int (setup)(struct ad7091r_state,
}

extern "C" {
    pub fn ad7091r_volatile_reg(dev: *mut device, reg: c_uint) -> bool;
}
extern "C" {
    pub fn ad7091r_writeable_reg(dev: *mut device, reg: c_uint) -> bool;
}
