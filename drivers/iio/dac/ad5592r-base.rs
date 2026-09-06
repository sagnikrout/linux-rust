//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/dac/ad5592r-base.h
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
// AD5592R / AD5593R Digital <-> Analog converters driver
//
// Copyright 2015-2016 Analog Devices Inc.
// Author: Paul Cercueil <paul.cercueil@analog.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad5592r_registers {
    AD5592R_REG_NOOP		= 0x0,
    AD5592R_REG_DAC_READBACK	= 0x1,
    AD5592R_REG_ADC_SEQ		= 0x2,
    AD5592R_REG_CTRL		= 0x3,
    AD5592R_REG_ADC_EN		= 0x4,
    AD5592R_REG_DAC_EN		= 0x5,
    AD5592R_REG_PULLDOWN		= 0x6,
    AD5592R_REG_LDAC		= 0x7,
    AD5592R_REG_GPIO_OUT_EN		= 0x8,
    AD5592R_REG_GPIO_SET		= 0x9,
    AD5592R_REG_GPIO_IN_EN		= 0xA,
    AD5592R_REG_PD			= 0xB,
    AD5592R_REG_OPEN_DRAIN		= 0xC,
    AD5592R_REG_TRISTATE		= 0xD,
    AD5592R_REG_RESET		= 0xF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5592r_rw_ops {
    pub value): *mut *mut *mut int (write_dac)(struct ad5592r_state st, unsigned chan, u16,
    pub value): *mut *mut *mut int (read_adc)(struct ad5592r_state st, unsigned chan, u16,
    pub value): *mut *mut *mut int (reg_write)(struct ad5592r_state st, u8 reg, u16,
    pub value): *mut *mut *mut int (reg_read)(struct ad5592r_state st, u8 reg, u16,
    pub value): *mut *mut *mut int (gpio_read)(struct ad5592r_state st, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5592r_state {
    pub dev: *mut device,
    pub reg: *mut regulator,
    pub gpiochip: gpio_chip,
    pub /: *mut *mut mutex gpio_lock; / Protect cached gpio_out, gpio_val, etc.,
    pub lock: mutex,
    pub num_channels: c_uint,
    pub ops: *const ad5592r_rw_ops,
    pub scale_avail: [c_int; 2][2],
    pub cached_dac: [u16; 8],
    pub cached_gp_ctrl: u16,
    pub channel_modes: [u8; 8],
    pub channel_offstate: [u8; 8],
    pub gpio_map: u8,
    pub gpio_out: u8,
    pub gpio_in: u8,
    pub gpio_val: u8,
    pub __aligned(IIO_DMA_MINALIGN): __be16 spi_msg,
    pub spi_msg_nop: __be16,
}

extern "C" {
    pub fn ad5592r_remove(dev: *mut device);
}
