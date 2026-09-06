//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/pressure/abp2030pa.h
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
// Honeywell ABP2 series pressure sensor driver
//
// Copyright (c) 2025 Petre Rodan <petre.rodan@subdimension.ro>
//

pub const ABP2_MEASUREMENT_RD_SIZE: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum abp2_func_id {
    ABP2_FUNCTION_A,
}

//
// struct abp2_data
// @dev: current device structure
// @ops: pointers for bus specific read and write functions
// @pmin: minimal pressure in pascal
// @pmax: maximal pressure in pascal
// @outmin: minimum raw pressure in counts (based on transfer function)
// @outmax: maximum raw pressure in counts (based on transfer function)
// @function: transfer function
// @p_scale: pressure scale
// @p_scale_dec: pressure scale, decimal number
// @p_offset: pressure offset
// @irq: end of conversion - applies only to the i2c sensor
// @completion: handshake from irq to read
// @scan: channel values for buffered mode
// @tx_buf: transmit buffer used during the SPI communication
// @rx_buf: raw data provided by sensor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abp2_data {
    pub dev: *mut device,
    pub ops: *const abp2_ops,
    pub pmin: i32,
    pub pmax: i32,
    pub outmin: u32,
    pub outmax: u32,
    pub function: abp2_func_id,
    pub p_scale: c_int,
    pub p_scale_dec: c_int,
    pub p_offset: c_int,
    pub irq: c_int,
    pub completion: completion,
    pub chan: [u32; 2],
    pub timestamp: aligned_s64,
    pub scan: },
    pub __aligned(IIO_DMA_MINALIGN): u8 rx_buf[ABP2_MEASUREMENT_RD_SIZE],
    pub tx_buf: [u8; ABP2_MEASUREMENT_RD_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct abp2_ops {
    pub nbytes): *mut *mut *mut int (read)(struct abp2_data data, u8 cmd, u8,
    pub nbytes): *mut *mut *mut int (write)(struct abp2_data data, u8 cmd, u8,
}

extern "C" {
    pub fn abp2_common_probe(dev: *mut device, ops: *const abp2_ops, irq: c_int) -> c_int;
}
