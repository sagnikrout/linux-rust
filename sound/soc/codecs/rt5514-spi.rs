//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5514-spi.h
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
// rt5514-spi.h  --  RT5514 driver
//
// Copyright 2015 Realtek Semiconductor Corp.
// Author: Oder Chiou <oder_chiou@realtek.com>
//
// RT5514_SPI_BUF_LEN is the buffer size of SPI master controller.
//
pub const RT5514_SPI_BUF_LEN: c_int = 240;
pub const RT5514_BUFFER_VOICE_BASE: c_uint = 0x18000200;
pub const RT5514_BUFFER_VOICE_LIMIT: c_uint = 0x18000204;
pub const RT5514_BUFFER_VOICE_WP: c_uint = 0x1800020c;
pub const RT5514_IRQ_CTRL: c_uint = 0x18002094;

// SPI Command
extern "C" {
    pub fn rt5514_spi_burst_read(addr: c_uint, rxbuf: *mut u8, len: usize) -> c_int;
}
extern "C" {
    pub fn rt5514_spi_burst_write(addr: u32, txbuf: *const u8, len: usize) -> c_int;
}
