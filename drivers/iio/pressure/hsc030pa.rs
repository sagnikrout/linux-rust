//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/pressure/hsc030pa.h
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
// Honeywell TruStability HSC Series pressure/temperature sensor
//
// Copyright (c) 2023 Petre Rodan <petre.rodan@subdimension.ro>
//

pub const HSC_REG_MEASUREMENT_RD_SIZE: c_int = 4;
pub const HSC_RESP_TIME_MS: c_int = 2;
extern "C" {
    pub fn int(: *mut *mut hsc_recv_fn)(struct hsc_data) -> typedef;
}
//
// struct hsc_data
// @dev: current device structure
// @chip: structure containing chip's channel properties
// @recv_cb: function that implements the chip reads
// @is_valid: true if last transfer has been validated
// @pmin: minimum measurable pressure limit
// @pmax: maximum measurable pressure limit
// @outmin: minimum raw pressure in counts (based on transfer function)
// @outmax: maximum raw pressure in counts (based on transfer function)
// @function: transfer function
// @p_scale: pressure scale
// @p_scale_dec: pressure scale, decimal places
// @p_offset: pressure offset
// @p_offset_dec: pressure offset, decimal places
// @buffer: raw conversion data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsc_data {
    pub dev: *mut device,
    pub chip: *const hsc_chip_data,
    pub recv_cb: hsc_recv_fn,
    pub is_valid: bool,
    pub pmin: i32,
    pub pmax: i32,
    pub outmin: u32,
    pub outmax: u32,
    pub function: u32,
    pub p_scale: i64,
    pub p_scale_dec: i32,
    pub p_offset: i64,
    pub p_offset_dec: i32,
    pub chan: [__be16; 2],
    pub timestamp: aligned_s64,
    pub scan: },
    pub __aligned(IIO_DMA_MINALIGN): u8 buffer[HSC_REG_MEASUREMENT_RD_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsc_chip_data {
    pub data): *mut *mut bool (valid)(struct hsc_data,
    pub channels: *const iio_chan_spec,
    pub num_channels: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hsc_func_id {
    HSC_FUNCTION_A,
    HSC_FUNCTION_B,
    HSC_FUNCTION_C,
    HSC_FUNCTION_F,
}

extern "C" {
    pub fn hsc_common_probe(dev: *mut device, recv: hsc_recv_fn) -> c_int;
}
