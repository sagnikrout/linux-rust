//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/spi/spi-bitbang-txrx.h
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
// Mix this utility code with some glue code to get one of several types of
// simple SPI master driver.  Two do polled word-at-a-time I/O:
//
// -	GPIO/parport bitbangers.  Provide chipselect() and txrx_word[](),
// expanding the per-word routines from the inline templates below.
//
// -	Drivers for controllers resembling bare shift registers.  Provide
// chipselect() and txrx_word[](), with custom setup()/cleanup() methods
// that use your controller's clock and chipselect registers.
//
// Some hardware works well with requests at spi_transfer scope:
//
// -	Drivers leveraging smarter hardware, with fifos or DMA; or for half
// duplex (MicroWire) controllers.  Provide chipselect() and txrx_bufs(),
// and custom setup()/cleanup() methods.
//
// The code that knows what GPIO pins do what should have declared four
// functions, ideally as inlines, before including this header:
//
// void setsck(struct spi_device *, int is_on);
// void setmosi(struct spi_device *, int is_on);
// int getmiso(struct spi_device *);
// void spidelay(unsigned);
//
// setsck()'s is_on parameter is a zero/nonzero boolean.
//
// setmosi()'s is_on parameter is a zero/nonzero boolean.
//
// getmiso() is required to return 0 or 1 only. Any other value is invalid
// and will result in improper operation.
//
// A non-inlined routine would call bitbang_txrx_*() routines.  The
// main loop could easily compile down to a handful of instructions,
// especially if the delay is a NOP (to run at peak speed).
//
// Since this is software, the timings may not be exactly what your board's
// chips need ... there may be several reasons you'd need to tweak timings
// in these routines, not just to make it faster or slower to match a
// particular CPU clock rate.
//
// ToDo: Maybe the bitrev macros can be used to improve the code?
//
// if (cpol == 0) this is SPI_MODE_0; else this is SPI_MODE_2
// clock starts at inactive polarity
// setup MSB (to slave) on trailing edge
// sample MSB (from slave) on leading edge
// if (cpol == 0) this is SPI_MODE_1; else this is SPI_MODE_3
// clock starts at inactive polarity
// setup MSB (to slave) on leading edge
// sample MSB (from slave) on trailing edge
// if (cpol == 0) this is SPI_MODE_0; else this is SPI_MODE_2
// clock starts at inactive polarity
// setup LSB (to slave) on trailing edge
// sample LSB (from slave) on leading edge
// if (cpol == 0) this is SPI_MODE_1; else this is SPI_MODE_3
// clock starts at inactive polarity
// setup LSB (to slave) on leading edge
// sample LSB (from slave) on trailing edge
