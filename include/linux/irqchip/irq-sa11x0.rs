//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqchip/irq-sa11x0.h
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
// Generic IRQ handling for the SA11x0.
//
// Copyright (C) 2015 Dmitry Eremin-Solenikov
// Copyright (C) 1999-2001 Nicolas Pitre
//

// Macro flag: #define __INCLUDE_LINUX_IRQCHIP_IRQ_SA11x0_H
extern "C" {
    pub fn sa11x0_init_irq_nodt(irq_start: c_int, io_start: resource_size_t) -> void __init;
}
