//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/irqchip/irq-ast2700.h
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
// Aspeed AST2700 Interrupt Controller.
//
// Copyright (C) 2026 ASPEED Technology Inc.
//

// Macro flag: #define DRIVERS_IRQCHIP_AST2700

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_intc_interrupt_range {
    pub start: u32,
    pub count: u32,
    pub upstream: irq_fwspec,
    pub domain: *mut irq_domain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_intc_interrupt_ranges {
    pub ranges: *mut aspeed_intc_interrupt_range,
    pub nranges: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_intc0 {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub intc_lock: raw_spinlock_t,
    pub local: *mut irq_domain,
    pub parent: *mut device_node,
    pub ranges: aspeed_intc_interrupt_ranges,
}
