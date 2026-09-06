//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm831x/watchdog.h
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
// include/linux/mfd/wm831x/watchdog.h -- Watchdog for WM831x
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// R16388 (0x4004) - Watchdog
//
pub const WM831X_WDOG_ENA: c_uint = 0x8000  /* WDOG_ENA */;
pub const WM831X_WDOG_ENA_MASK: c_uint = 0x8000  /* WDOG_ENA */;

pub const WM831X_WDOG_DEBUG: c_uint = 0x4000  /* WDOG_DEBUG */;
pub const WM831X_WDOG_DEBUG_MASK: c_uint = 0x4000  /* WDOG_DEBUG */;

pub const WM831X_WDOG_RST_SRC: c_uint = 0x2000  /* WDOG_RST_SRC */;
pub const WM831X_WDOG_RST_SRC_MASK: c_uint = 0x2000  /* WDOG_RST_SRC */;

pub const WM831X_WDOG_SLPENA: c_uint = 0x1000  /* WDOG_SLPENA */;
pub const WM831X_WDOG_SLPENA_MASK: c_uint = 0x1000  /* WDOG_SLPENA */;

pub const WM831X_WDOG_RESET: c_uint = 0x0800  /* WDOG_RESET */;
pub const WM831X_WDOG_RESET_MASK: c_uint = 0x0800  /* WDOG_RESET */;

pub const WM831X_WDOG_SECACT_MASK: c_uint = 0x0300  /* WDOG_SECACT - [9:8] */;

pub const WM831X_WDOG_PRIMACT_MASK: c_uint = 0x0030  /* WDOG_PRIMACT - [5:4] */;

pub const WM831X_WDOG_TO_MASK: c_uint = 0x0007  /* WDOG_TO - [2:0] */;

