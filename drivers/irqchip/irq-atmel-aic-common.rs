//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/irqchip/irq-atmel-aic-common.h
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


//
// Atmel AT91 common AIC (Advanced Interrupt Controller) header file
//
// Copyright (C) 2004 SAN People
// Copyright (C) 2004 ATMEL
// Copyright (C) Rick Bronson
// Copyright (C) 2014 Free Electrons
//
// Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//
extern "C" {
    pub fn aic_common_set_type(d: *mut irq_data, type: unsigned, val: *mut unsigned) -> c_int;
}
extern "C" {
    pub fn aic_common_set_priority(priority: c_int, val: *mut unsigned);
}
extern "C" {
    pub fn aic_common_rtc_irq_fixup() -> void __init;
}
extern "C" {
    pub fn aic_common_rtt_irq_fixup() -> void __init;
}
