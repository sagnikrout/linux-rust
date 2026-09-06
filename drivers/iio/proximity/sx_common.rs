//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/proximity/sx_common.h
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
// Copyright 2021 Google LLC.
//
// Code shared between most Semtech SAR sensor driver.
//

pub const SX_COMMON_REG_IRQ_SRC: c_uint = 0x00;
pub const SX_COMMON_MAX_NUM_CHANNELS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sx_common_reg_default {
    pub reg: u8,
    pub def: u8,
    pub property: *const c_char,
}

//
// struct sx_common_ops: function pointers needed by common code
//
// List functions needed by common code to gather information or configure
// the sensor.
//
// @read_prox_data:	Function to read raw proximity data.
// @check_whoami:	Set device name based on whoami register.
// @init_compensation:	Function to set initial compensation.
// @wait_for_sample:	When there are no physical IRQ, function to wait for a
// sample to be ready.
// @get_default_reg:	Populate the initial value for a given register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sx_common_ops {
    pub val): *const *const iio_chan_spec chan, __be16,
    pub indio_dev): *mut *mut *mut int (check_whoami)(struct device dev, struct iio_dev,
    pub indio_dev): *mut *mut int (init_compensation)(struct iio_dev,
    pub data): *mut *mut int (wait_for_sample)(struct sx_common_data,
    pub reg_def): *mut sx_common_reg_default,
}

//
// struct sx_common_chip_info: Semtech Sensor private chip information
//
// @reg_stat:		Main status register address.
// @reg_irq_msk:	IRQ mask register address.
// @reg_enable_chan:	Address to enable/disable channels.
// Each phase presented by the sensor is an IIO channel..
// @reg_reset:		Reset register address.
// @mask_enable_chan:	Mask over the channels bits in the enable channel
// register.
// @stat_offset:	Offset to check phase status.
// @irq_msk_offset:	Offset to enable interrupt in the IRQ mask
// register.
// @num_channels:	Number of channels.
// @num_default_regs:	Number of internal registers that can be configured.
//
// @ops:		Private functions pointers.
// @iio_channels:	Description of exposed iio channels.
// @num_iio_channels:	Number of iio_channels.
// @iio_info:		iio_info structure for this driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sx_common_chip_info {
    pub reg_stat: c_uint,
    pub reg_irq_msk: c_uint,
    pub reg_enable_chan: c_uint,
    pub reg_reset: c_uint,
    pub mask_enable_chan: c_uint,
    pub stat_offset: c_uint,
    pub irq_msk_offset: c_uint,
    pub num_channels: c_uint,
    pub num_default_regs: c_int,
    pub ops: sx_common_ops,
    pub iio_channels: *const iio_chan_spec,
    pub num_iio_channels: c_int,
    pub iio_info: iio_info,
}

//
// struct sx_common_data: Semtech Sensor private data structure.
//
// @chip_info:		Structure defining sensor internals.
// @mutex:		Serialize access to registers and channel configuration.
// @completion:		completion object to wait for data acquisition.
// @client:		I2C client structure.
// @trig:		IIO trigger object.
// @regmap:		Register map.
// @chan_prox_stat:	Last reading of the proximity status for each channel.
// We only send an event to user space when this changes.
// @trigger_enabled:	True when the device trigger is enabled.
// @buffer:		Buffer to store raw samples.
// @suspend_ctrl:	Remember enabled channels and sample rate during suspend.
// @chan_read:		Bit field for each raw channel enabled.
// @chan_event:		Bit field for each event enabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sx_common_data {
    pub chip_info: *const sx_common_chip_info,
    pub mutex: mutex,
    pub completion: completion,
    pub client: *mut i2c_client,
    pub trig: *mut iio_trigger,
    pub regmap: *mut regmap,
    pub chan_prox_stat: c_ulong,
    pub trigger_enabled: bool,
// Ensure correct alignment of timestamp when present.
    pub channels: [__be16; SX_COMMON_MAX_NUM_CHANNELS],
    pub ts: aligned_s64,
    pub buffer: },
    pub suspend_ctrl: c_uint,
    pub chan_read: c_ulong,
    pub chan_event: c_ulong,
}

// 3 is the number of events defined by a single phase.
