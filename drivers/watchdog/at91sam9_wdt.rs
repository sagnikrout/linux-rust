//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/watchdog/at91sam9_wdt.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// drivers/watchdog/at91sam9_wdt.h
//
// Copyright (C) 2007 Andrew Victor
// Copyright (C) 2007 Atmel Corporation.
// Copyright (C) 2019 Microchip Technology Inc. and its subsidiaries
//
// Watchdog Timer (WDT) - System peripherals regsters.
// Based on AT91SAM9261 datasheet revision D.
// Based on SAM9X60 datasheet.
// Based on SAMA7G5 datasheet.
// Based on SAM9X75 datasheet.
//

pub const AT91_WDT_CR: c_uint = 0x00			/* Watchdog Control Register */;

pub const AT91_WDT_MR: c_uint = 0x04			/* Watchdog Mode Register */;

pub const AT91_WDT_SR: c_uint = 0x08		/* Watchdog Status Register */;

// Watchdog Timer Value Register
pub const AT91_SAM9X60_VR: c_uint = 0x08;
// Watchdog Window Level Register
pub const AT91_SAM9X60_WLR: c_uint = 0x0c;
// Watchdog Period Value

// Interrupt Enable Register
pub const AT91_SAM9X60_IER: c_uint = 0x14;
// Period Interrupt Enable

// Interrupt Disable Register
pub const AT91_SAM9X60_IDR: c_uint = 0x18;
// Interrupt Status Register
pub const AT91_SAM9X60_ISR: c_uint = 0x1c;
