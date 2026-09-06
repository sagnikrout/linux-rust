//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/dac/ad5446.h
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
// struct ad5446_state - driver instance specific data
// @dev:		this device
// @chip_info:		chip model specific constants, available modes etc
// @vref_mv:		actual reference voltage used
// @cached_val:		store/retrieve values during power down
// @pwr_down_mode:	power down mode (1k, 100k or tristate)
// @pwr_down:		true if the device is in power down
// @lock:		lock to protect the data buffer during write ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5446_state {
    pub dev: *mut device,
    pub chip_info: *const ad5446_chip_info,
    pub vref_mv: c_ushort,
    pub cached_val: c_uint,
    pub pwr_down_mode: c_uint,
    pub pwr_down: c_uint,
// mutex to protect device shared data
    pub lock: mutex,
    pub d16: __be16,
    pub d24: [u8; 3],
    pub __aligned(IIO_DMA_MINALIGN): },
}

//
// struct ad5446_chip_info - chip specific information
// @channel:		channel spec for the DAC
// @int_vref_mv:	AD5620/40/60: the internal reference voltage
// @write:		chip specific helper function to write to the register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5446_chip_info {
    pub channel: iio_chan_spec,
    pub int_vref_mv: u16,
    pub val): *mut *mut *mut int (write)(struct ad5446_state st, unsigned int,
}
