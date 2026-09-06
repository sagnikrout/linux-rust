//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmc/pm.h
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
// linux/include/linux/mmc/pm.h
//
// Author:	Nicolas Pitre
// Copyright:	(C) 2009 Marvell Technology Group Ltd.
//
// These flags are used to describe power management features that
// some cards (typically SDIO cards) might wish to benefit from when
// the host system is being suspended.  There are several layers of
// abstractions involved, from the host controller driver, to the MMC core
// code, to the SDIO core code, to finally get to the actual SDIO function
// driver.  This file is therefore used for common definitions shared across
// all those layers.
//
pub type mmc_pm_flag_t = c_uint;

