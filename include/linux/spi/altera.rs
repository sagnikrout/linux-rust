//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/altera.h
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
// Header File for Altera SPI Driver.
//

pub const ALTERA_SPI_MAX_CS: c_int = 32;
//
// struct altera_spi_platform_data - Platform data of the Altera SPI driver
// @mode_bits:		Mode bits of SPI host.
// @num_chipselect:	Number of chipselects.
// @bits_per_word_mask:	bitmask of supported bits_per_word for transfers.
// @num_devices:	Number of devices that shall be added when the driver
// is probed.
// @devices:		The devices to add.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct altera_spi_platform_data {
    pub mode_bits: u16,
    pub num_chipselect: u16,
    pub bits_per_word_mask: u32,
    pub num_devices: u16,
    pub devices: *mut spi_board_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct altera_spi {
    pub irq: c_int,
    pub len: c_int,
    pub count: c_int,
    pub bytes_per_word: c_int,
    pub imr: u32,
// data buffers
    pub tx: *const c_uchar,
    pub rx: *mut c_uchar,
    pub regmap: *mut regmap,
    pub regoff: u32,
    pub dev: *mut device,
}

extern "C" {
    pub fn altera_spi_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn altera_spi_init_host(host: *mut spi_controller);
}
