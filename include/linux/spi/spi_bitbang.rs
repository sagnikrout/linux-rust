//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/spi_bitbang.h
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

extern "C" {
    pub fn u32(: *mut *mut spi_bb_txrx_word_fn)(struct spi_device, int: unsigned, _arg: u32, _arg: u8, int: unsigned) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_bitbang {
    pub lock: mutex,
    pub busy: u8,
    pub use_dma: u8,
    pub /: *mut *mut u16 flags; / extra spi->mode support,
    pub ctlr: *mut spi_controller,
// setup_transfer() changes clock and/or wordsize to match settings
// for this transfer; zeroes restore defaults from spi_device.
//
    pub t): *mut spi_transfer,
    pub is_on): *mut *mut *mut void (chipselect)(struct spi_device spi, int,

pub const BITBANG_CS_INACTIVE: c_int = 0;
    pub spi): *mut *mut void (set_mosi_idle)(struct spi_device,
// txrx_bufs() may handle dma mapping for transfers that don't
// already have one (transfer.{tx,rx}_dma is zero), or use PIO
//
    pub t): *mut *mut *mut int (txrx_bufs)(struct spi_device spi, struct spi_transfer,
// txrx_word[SPI_MODE_*]() just looks like a shift register
    pub 1]: spi_bb_txrx_word_fn txrx_word[SPI_MODE_X_MASK +,
    pub output): *mut *mut *mut int (set_line_direction)(struct spi_device spi, bool,
}

// you can call these default bitbang->master methods from your custom
// methods, if you like.
//
extern "C" {
    pub fn spi_bitbang_setup(spi: *mut spi_device) -> c_int;
}
extern "C" {
    pub fn spi_bitbang_cleanup(spi: *mut spi_device);
}
// start or stop queue processing
extern "C" {
    pub fn spi_bitbang_start(spi: *mut spi_bitbang) -> c_int;
}
extern "C" {
    pub fn spi_bitbang_init(spi: *mut spi_bitbang) -> c_int;
}
extern "C" {
    pub fn spi_bitbang_stop(spi: *mut spi_bitbang);
}
