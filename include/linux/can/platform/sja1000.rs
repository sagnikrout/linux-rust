//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/can/platform/sja1000.h
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
// clock divider register
pub const CDR_CLKOUT_MASK: c_uint = 0x07;
pub const CDR_CLK_OFF: c_uint = 0x08 /* Clock off (CLKOUT pin) */;
pub const CDR_RXINPEN: c_uint = 0x20 /* TX1 output is RX irq output */;
pub const CDR_CBP: c_uint = 0x40 /* CAN input comparator bypass */;
pub const CDR_PELICAN: c_uint = 0x80 /* PeliCAN mode */;
// output control register
pub const OCR_MODE_BIPHASE: c_uint = 0x00;
pub const OCR_MODE_TEST: c_uint = 0x01;
pub const OCR_MODE_NORMAL: c_uint = 0x02;
pub const OCR_MODE_CLOCK: c_uint = 0x03;
pub const OCR_MODE_MASK: c_uint = 0x03;
pub const OCR_TX0_INVERT: c_uint = 0x04;
pub const OCR_TX0_PULLDOWN: c_uint = 0x08;
pub const OCR_TX0_PULLUP: c_uint = 0x10;
pub const OCR_TX0_PUSHPULL: c_uint = 0x18;
pub const OCR_TX1_INVERT: c_uint = 0x20;
pub const OCR_TX1_PULLDOWN: c_uint = 0x40;
pub const OCR_TX1_PULLUP: c_uint = 0x80;
pub const OCR_TX1_PUSHPULL: c_uint = 0xc0;
pub const OCR_TX_MASK: c_uint = 0xfc;
pub const OCR_TX_SHIFT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1000_platform_data {
    pub /: *mut *mut u32 osc_freq; / CAN bus oscillator frequency in Hz,
    pub /: *mut *mut u8 ocr; / output control register,
    pub /: *mut *mut u8 cdr; / clock divider register,
}
