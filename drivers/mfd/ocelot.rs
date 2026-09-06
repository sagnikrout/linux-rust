//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mfd/ocelot.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
// Copyright 2021, 2022 Innovative Advantage Inc.

//
// struct ocelot_ddata - Private data for an external Ocelot chip
// @gcb_regmap:		General Configuration Block regmap. Used for
// operations like chip reset.
// @cpuorg_regmap:	CPU Device Origin Block regmap. Used for operations
// like SPI bus configuration.
// @spi_padding_bytes:	Number of padding bytes that must be thrown out before
// read data gets returned. This is calculated during
// initialization based on bus speed.
// @dummy_buf:		Zero-filled buffer of spi_padding_bytes size. The dummy
// bytes that will be sent out between the address and
// data of a SPI read operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_ddata {
    pub gcb_regmap: *mut regmap,
    pub cpuorg_regmap: *mut regmap,
    pub spi_padding_bytes: c_int,
    pub dummy_buf: *mut c_void,
}

extern "C" {
    pub fn ocelot_chip_reset(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ocelot_core_init(dev: *mut device) -> c_int;
}
// SPI-specific routines that won't be necessary for other interfaces
pub const OCELOT_SPI_BYTE_ORDER_LE: c_uint = 0x00000000;
pub const OCELOT_SPI_BYTE_ORDER_BE: c_uint = 0x81818181;

