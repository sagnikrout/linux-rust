//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/libertas_spi.h
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
// board-specific data for the libertas_spi driver.
//
// Copyright 2008 Analog Devices Inc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libertas_spi_platform_data {
// There are two ways to read data from the WLAN module's SPI
// interface. Setting 0 or 1 here controls which one is used.
//
// Usually you want to set use_dummy_writes = 1.
// However, if that doesn't work or if you are using a slow SPI clock
// speed, you may want to use 0 here.
    pub use_dummy_writes: u16,
// Board specific setup/teardown
    pub spi): *mut *mut int (setup)(struct spi_device,
    pub spi): *mut *mut int (teardown)(struct spi_device,
}
