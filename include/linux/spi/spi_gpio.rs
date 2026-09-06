//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/spi_gpio.h
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
// For each bitbanged SPI bus, set up a platform_device node with:
// - name "spi_gpio"
// - id the same as the SPI bus number it implements
// - dev.platform data pointing to a struct spi_gpio_platform_data
//
// Use spi_board_info with these busses in the usual way.
//
// If the bitbanged bus is later switched to a "native" controller,
// that platform_device and controller_data should be removed.
//
// struct spi_gpio_platform_data - parameter for bitbanged SPI host controller
// @num_chipselect: how many target devices to allow
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_gpio_platform_data {
    pub num_chipselect: u16,
}
