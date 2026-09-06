//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/syscon/atmel-st.h
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
// Copyright (C) 2005 Ivan Kokshaysky
// Copyright (C) SAN People
//
// System Timer (ST) - System peripherals registers.
// Based on AT91RM9200 datasheet revision E.
//

pub const AT91_ST_CR: c_uint = 0x00	/* Control Register */;

pub const AT91_ST_PIMR: c_uint = 0x04	/* Period Interval Mode Register */;
pub const AT91_ST_PIV: c_uint = 0xffff	/* Period Interval Value */;
pub const AT91_ST_WDMR: c_uint = 0x08	/* Watchdog Mode Register */;
pub const AT91_ST_WDV: c_uint = 0xffff	/* Watchdog Counter Value */;

pub const AT91_ST_RTMR: c_uint = 0x0c	/* Real-time Mode Register */;
pub const AT91_ST_RTPRES: c_uint = 0xffff	/* Real-time Prescalar Value */;
pub const AT91_ST_SR: c_uint = 0x10	/* Status Register */;

pub const AT91_ST_IER: c_uint = 0x14	/* Interrupt Enable Register */;
pub const AT91_ST_IDR: c_uint = 0x18	/* Interrupt Disable Register */;
pub const AT91_ST_IMR: c_uint = 0x1c	/* Interrupt Mask Register */;
pub const AT91_ST_RTAR: c_uint = 0x20	/* Real-time Alarm Register */;
pub const AT91_ST_ALMV: c_uint = 0xfffff	/* Alarm Value */;
pub const AT91_ST_CRTR: c_uint = 0x24	/* Current Real-time Register */;
pub const AT91_ST_CRTV: c_uint = 0xfffff	/* Current Real-Time Value */;
