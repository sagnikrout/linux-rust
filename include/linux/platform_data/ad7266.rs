//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/ad7266.h
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
// AD7266/65 SPI ADC driver
//
// Copyright 2012 Analog Devices Inc.
//
// enum ad7266_range - AD7266 reference voltage range
// @AD7266_RANGE_VREF: Device is configured for input range 0V - VREF
// (RANGE pin set to low)
// @AD7266_RANGE_2VREF: Device is configured for input range 0V - 2VREF
// (RANGE pin set to high)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad7266_range {
    AD7266_RANGE_VREF,
    AD7266_RANGE_2VREF,
}

//
// enum ad7266_mode - AD7266 sample mode
// @AD7266_MODE_DIFF: Device is configured for full differential mode
// (SGL/DIFF pin set to low, AD0 pin set to low)
// @AD7266_MODE_PSEUDO_DIFF: Device is configured for pseudo differential mode
// (SGL/DIFF pin set to low, AD0 pin set to high)
// @AD7266_MODE_SINGLE_ENDED: Device is configured for single-ended mode
// (SGL/DIFF pin set to high)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad7266_mode {
    AD7266_MODE_DIFF,
    AD7266_MODE_PSEUDO_DIFF,
    AD7266_MODE_SINGLE_ENDED,
}

//
// struct ad7266_platform_data - Platform data for the AD7266 driver
// @range: Reference voltage range the device is configured for
// @mode: Sample mode the device is configured for
// @fixed_addr: Whether the address pins are hard-wired
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7266_platform_data {
    pub range: ad7266_range,
    pub mode: ad7266_mode,
    pub fixed_addr: bool,
}
