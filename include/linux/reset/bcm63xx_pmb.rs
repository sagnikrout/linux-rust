//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/reset/bcm63xx_pmb.h
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
// Broadcom BCM63xx Processor Monitor Bus shared routines (SMP and reset)
//
// Copyright (C) 2015, Broadcom Corporation
// Author: Florian Fainelli <f.fainelli@gmail.com>
//

// PMB Master controller register
pub const PMB_CTRL: c_uint = 0x00;

pub const PMB_WR_DATA: c_uint = 0x04;
pub const PMB_TIMEOUT: c_uint = 0x08;
pub const PMB_RD_DATA: c_uint = 0x0C;
pub const PMB_BUS_ID_SHIFT: c_int = 8;
// Perform the low-level PMB master operation, shared between reads and
// writes.
//
// val = readl(master + PMB_RD_DATA);
