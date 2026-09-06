//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm831x/gpio.h
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
// include/linux/mfd/wm831x/gpio.h -- GPIO for WM831x
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// R16440-16455 (0x4038-0x4047) - GPIOx Control
//
pub const WM831X_GPN_DIR: c_uint = 0x8000  /* GPN_DIR */;
pub const WM831X_GPN_DIR_MASK: c_uint = 0x8000  /* GPN_DIR */;

pub const WM831X_GPN_PULL_MASK: c_uint = 0x6000  /* GPN_PULL - [14:13] */;

pub const WM831X_GPN_INT_MODE: c_uint = 0x1000  /* GPN_INT_MODE */;
pub const WM831X_GPN_INT_MODE_MASK: c_uint = 0x1000  /* GPN_INT_MODE */;

pub const WM831X_GPN_PWR_DOM: c_uint = 0x0800  /* GPN_PWR_DOM */;
pub const WM831X_GPN_PWR_DOM_MASK: c_uint = 0x0800  /* GPN_PWR_DOM */;

pub const WM831X_GPN_POL: c_uint = 0x0400  /* GPN_POL */;
pub const WM831X_GPN_POL_MASK: c_uint = 0x0400  /* GPN_POL */;

pub const WM831X_GPN_OD: c_uint = 0x0200  /* GPN_OD */;
pub const WM831X_GPN_OD_MASK: c_uint = 0x0200  /* GPN_OD */;

pub const WM831X_GPN_ENA: c_uint = 0x0080  /* GPN_ENA */;
pub const WM831X_GPN_ENA_MASK: c_uint = 0x0080  /* GPN_ENA */;

pub const WM831X_GPN_TRI: c_uint = 0x0080  /* GPN_TRI */;
pub const WM831X_GPN_TRI_MASK: c_uint = 0x0080  /* GPN_TRI */;

pub const WM831X_GPN_FN_MASK: c_uint = 0x000F  /* GPN_FN - [3:0] */;

