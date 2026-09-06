//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/x86/spi-intel.h
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
// Intel PCH/PCU SPI flash driver.
//
// Copyright (C) 2016, Intel Corporation
// Author: Mika Westerberg <mika.westerberg@linux.intel.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_spi_type {
    INTEL_SPI_BYT = 1,
    INTEL_SPI_LPT,
    INTEL_SPI_BXT,
    INTEL_SPI_CNL,
}

//
// struct intel_spi_boardinfo - Board specific data for Intel SPI driver
// @type: Type which this controller is compatible with
// @set_writeable: Try to make the chip writeable (optional)
// @data: Data to be passed to @set_writeable can be %NULL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_spi_boardinfo {
    pub type: intel_spi_type,
    pub data): *mut *mut *mut bool (set_writeable)(void __iomem base, void,
    pub data: *mut c_void,
}
