//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/dwc3/io.h
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
// io.h - DesignWare USB3 DRD IO Header
//
// Copyright (C) 2010-2011 Texas Instruments Incorporated - https://www.ti.com
//
// Authors: Felipe Balbi <balbi@ti.com>,
// Sebastian Andrzej Siewior <bigeasy@linutronix.de>
//

//
// We requested the mem region starting from the Globals address
// space, see dwc3_probe in core.c.
// However, the offsets are given starting from xHCI address space.
//
// When tracing we want to make it easy to find the correct address on
// documentation, so we revert it back to the proper addresses, the
// same way they are described on SNPS documentation
//
// We requested the mem region starting from the Globals address
// space, see dwc3_probe in core.c.
// However, the offsets are given starting from xHCI address space.
//
// When tracing we want to make it easy to find the correct address on
// documentation, so we revert it back to the proper addresses, the
// same way they are described on SNPS documentation
//
