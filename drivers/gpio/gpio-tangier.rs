//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpio/gpio-tangier.h
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
// Intel Tangier GPIO functions
//
// Copyright (c) 2016, 2021, 2023 Intel Corporation.
//
// Authors: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
// Pandith N <pandith.n@intel.com>
// Raag Jadav <raag.jadav@intel.com>
//

// Elkhart Lake specific wake registers
pub const GWMR_EHL: c_uint = 0x100	/* Wake mask */;
pub const GWSR_EHL: c_uint = 0x118	/* Wake source */;
pub const GSIR_EHL: c_uint = 0x130	/* Secure input */;
// Merrifield specific wake registers
pub const GWMR_MRFLD: c_uint = 0x400	/* Wake mask */;
pub const GWSR_MRFLD: c_uint = 0x418	/* Wake source */;
pub const GSIR_MRFLD: c_uint = 0xc00	/* Secure input */;
//
// struct tng_wake_regs - Platform specific wake registers
// @gwmr: Wake mask
// @gwsr: Wake source
// @gsir: Secure input
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tng_wake_regs {
    pub gwmr: u32,
    pub gwsr: u32,
    pub gsir: u32,
}

//
// struct tng_gpio_pinrange - Map pin numbers to gpio numbers
// @gpio_base: Starting GPIO number of this range
// @pin_base: Starting pin number of this range
// @npins: Number of pins in this range
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tng_gpio_pinrange {
    pub gpio_base: c_uint,
    pub pin_base: c_uint,
    pub npins: c_uint,
}

//
// struct tng_gpio_pin_info - Platform specific pinout information
// @pin_ranges: Pin to GPIO mapping
// @nranges: Number of pin ranges
// @name: Respective pinctrl device name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tng_gpio_pin_info {
    pub pin_ranges: *const tng_gpio_pinrange,
    pub nranges: c_uint,
    pub name: *const c_char,
}

//
// struct tng_gpio_info - Platform specific GPIO and IRQ information
// @base: GPIO base to start numbering with
// @ngpio: Amount of GPIOs supported by the controller
// @first: First IRQ to start numbering with
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tng_gpio_info {
    pub base: c_int,
    pub ngpio: u16,
    pub first: c_uint,
}

//
// struct tng_gpio - Platform specific private data
// @chip: Instance of the struct gpio_chip
// @reg_base: Base address of MMIO registers
// @irq: Interrupt for the GPIO device
// @lock: Synchronization lock to prevent I/O race conditions
// @dev: The GPIO device
// @ctx: Context to be saved during suspend-resume
// @wake_regs: Platform specific wake registers
// @pin_info: Platform specific pinout information
// @info: Platform specific GPIO and IRQ information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tng_gpio {
    pub chip: gpio_chip,
    pub reg_base: *mut void __iomem,
    pub irq: c_int,
    pub lock: raw_spinlock_t,
    pub dev: *mut device,
    pub ctx: *mut tng_gpio_context,
    pub wake_regs: tng_wake_regs,
    pub pin_info: tng_gpio_pin_info,
    pub info: tng_gpio_info,
}

extern "C" {
    pub fn devm_tng_gpio_probe(dev: *mut device, gpio: *mut tng_gpio) -> c_int;
}
