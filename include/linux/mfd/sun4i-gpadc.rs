//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/sun4i-gpadc.h
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
// Header of ADC MFD core driver for sunxi platforms
//
// Copyright (c) 2016 Quentin Schulz <quentin.schulz@free-electrons.com>
//
pub const SUN4I_GPADC_CTRL0: c_uint = 0x00;

pub const SUN4I_GPADC_CTRL1: c_uint = 0x04;

// TP_CTRL1 bits for sun6i SOCs

// TP_CTRL1 bits for sun8i SoCs

pub const SUN4I_GPADC_CTRL2: c_uint = 0x08;

pub const SUN4I_GPADC_CTRL3: c_uint = 0x0c;

pub const SUN4I_GPADC_TPR: c_uint = 0x18;

pub const SUN4I_GPADC_INT_FIFOC: c_uint = 0x10;

pub const SUN4I_GPADC_INT_FIFOS: c_uint = 0x14;

pub const SUN4I_GPADC_CDAT: c_uint = 0x1c;
pub const SUN4I_GPADC_TEMP_DATA: c_uint = 0x20;
pub const SUN4I_GPADC_DATA: c_uint = 0x24;
pub const SUN4I_GPADC_IRQ_FIFO_DATA: c_int = 1;
pub const SUN4I_GPADC_IRQ_TEMP_DATA: c_int = 2;
// 10s delay before suspending the IP
pub const SUN4I_GPADC_AUTOSUSPEND_DELAY: c_int = 10000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_gpadc_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub regmap_irqc: *mut regmap_irq_chip_data,
    pub base: *mut void __iomem,
}
