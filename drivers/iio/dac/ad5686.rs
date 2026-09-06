//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/dac/ad5686.h
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
// This file is part of AD5686 DAC driver
//
// Copyright 2018 Analog Devices Inc.
//

pub const AD5686_ADDR_ALL_DAC: c_uint = 0xF;
pub const AD5686_MAX_CHANNELS: c_int = 16;
pub const AD5686_CMD_NOOP: c_uint = 0x0;
pub const AD5686_CMD_WRITE_INPUT_N: c_uint = 0x1;
pub const AD5686_CMD_UPDATE_DAC_N: c_uint = 0x2;
pub const AD5686_CMD_WRITE_INPUT_N_UPDATE_N: c_uint = 0x3;
pub const AD5686_CMD_POWERDOWN_DAC: c_uint = 0x4;
pub const AD5686_CMD_LDAC_MASK: c_uint = 0x5;
pub const AD5686_CMD_RESET: c_uint = 0x6;
pub const AD5686_CMD_INTERNAL_REFER_SETUP: c_uint = 0x7;
pub const AD5686_CMD_DAISY_CHAIN_ENABLE: c_uint = 0x8;
pub const AD5686_CMD_READBACK_ENABLE: c_uint = 0x9;
pub const AD5686_CMD_CONTROL_REG: c_uint = 0x4;
pub const AD5686_CMD_READBACK_ENABLE_V2: c_uint = 0x5;

pub const AD5686_PD_MODE_1K_TO_GND: c_uint = 0x1;
pub const AD5686_PD_MODE_100K_TO_GND: c_uint = 0x2;
pub const AD5686_PD_MODE_THREE_STATE: c_uint = 0x3;
pub const AD5686_PD_MSK_PWR_UP: c_uint = 0x0;
pub const AD5686_PD_MSK_PWR_DOWN: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad5686_regmap_type {
    AD5310_REGMAP,
    AD5683_REGMAP,
    AD5686_REGMAP,
}

//
// struct ad5686_bus_ops - bus specific read/write operations
// @read: read a register value at the given address
// @write: write a command, address and value to the device
// @sync: ensure the completion of the write operation (optional)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5686_bus_ops {
    pub addr): *mut *mut *mut int (read)(struct ad5686_state st, u8,
    pub val): *mut *mut *mut int (write)(struct ad5686_state st, u8 cmd, u8 addr, u16,
    pub st): *mut *mut int (sync)(struct ad5686_state,
}

//
// struct ad5686_chip_info - chip specific information
// @int_vref_mv:	the internal reference voltage
// @num_channels:	number of channels
// @channel:		channel specification
// @regmap_type:	register map layout variant
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5686_chip_info {
    pub int_vref_mv: u16,
    pub num_channels: c_uint,
    pub channels: *const iio_chan_spec,
    pub regmap_type: ad5686_regmap_type,
}

// single-channel instances
// dual-channel instances
// quad-channel instances
// 8-channel instances
// 16-channel instances
//
// struct ad5686_state - driver instance specific data
// @dev:		device instance
// @chip_info:		chip model specific constants, available modes etc
// @ops:		bus specific operations
// @ldac_gpio:		LDAC pin GPIO descriptor
// @gain_gpio:		GAIN pin GPIO descriptor
// @pwr_down_mask:	power down mask
// @pwr_down_mode:	current power down mode
// @scale_avail:	pre-calculated available scale values
// @vref_mv:		actual reference voltage used
// @double_scale:	flag to indicate the gain multiplier is applied
// @use_internal_vref:	set to true if the internal reference voltage is used
// @lock:		lock to protect access to state fields, which includes
// the data buffer during regmap ops
// @bus_data:		bus specific data
// @data:		transfer buffers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5686_state {
    pub dev: *mut device,
    pub chip_info: *const ad5686_chip_info,
    pub ops: *const ad5686_bus_ops,
    pub ldac_gpio: *mut gpio_desc,
    pub gain_gpio: *mut gpio_desc,
    pub pwr_down_mask: c_uint,
    pub pwr_down_mode: c_uint,
    pub scale_avail: [c_int; 4],
    pub vref_mv: c_ushort,
    pub double_scale: bool,
    pub use_internal_vref: bool,
    pub lock: mutex,
    pub bus_data: *mut c_void,
//
// DMA (thus cache coherency maintenance) may require the
// transfer buffers to live in their own cache lines.
//
    pub d32: __be32,
    pub d16: __be16,
    pub d8: [u8; 4],
    pub __aligned(IIO_DMA_MINALIGN): } data[AD5686_MAX_CHANNELS],
}
