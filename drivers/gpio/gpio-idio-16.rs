//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpio/gpio-idio-16.h
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
// Copyright 2022 William Breathitt Gray
//
// struct idio_16_regmap_config - Configuration for the IDIO-16 register map
// @parent:		parent device
// @map:		regmap for the IDIO-16 device
// @regmap_irqs:	descriptors for individual IRQs
// @num_regmap_irqs:	number of IRQ descriptors
// @irq:		IRQ number for the IDIO-16 device
// @no_status:		device has no status register
// @filters:		device has input filters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idio_16_regmap_config {
    pub parent: *mut device,
    pub map: *mut regmap,
    pub regmap_irqs: *const regmap_irq,
    pub num_regmap_irqs: c_int,
    pub irq: c_uint,
    pub no_status: bool,
    pub filters: bool,
}

extern "C" {
    pub fn devm_idio_16_regmap_register(dev: *mut device, config: *const idio_16_regmap_config) -> c_int;
}
