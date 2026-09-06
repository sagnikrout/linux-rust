//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/ds1305.h
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
// One-time configuration for ds1305 and ds1306 RTC chips.
//
// Put a pointer to this in spi_board_info.platform_data if you want to
// be sure that Linux (re)initializes this as needed ... after losing
// backup power, and potentially on the first boot.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds1305_platform_data {
// Trickle charge configuration:  it's OK to leave out the MAGIC
// bitmask; mask in either DS1 or DS2, and then one of 2K/4k/8K.
//
pub const DS1305_TRICKLE_MAGIC: c_uint = 0xa0;
pub const DS1305_TRICKLE_DS2: c_uint = 0x08	/* two diodes */;
pub const DS1305_TRICKLE_DS1: c_uint = 0x04	/* one diode */;
pub const DS1305_TRICKLE_2K: c_uint = 0x01	/* 2 KOhm resistance */;
pub const DS1305_TRICKLE_4K: c_uint = 0x02	/* 4 KOhm resistance */;
pub const DS1305_TRICKLE_8K: c_uint = 0x03	/* 8 KOhm resistance */;
    pub trickle: u8,
// set only on ds1306 parts
    pub is_ds1306: bool,
// ds1306 only:  enable 1 Hz output
    pub en_1hz: bool,
// REVISIT:  the driver currently expects nINT0 to be wired
// as the alarm IRQ.  ALM1 may also need to be set up ...
//
}
