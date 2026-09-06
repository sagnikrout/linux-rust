//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/ad7887.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AD7887 SPI ADC driver
//
// Copyright 2010 Analog Devices Inc.
//
// struct ad7887_platform_data - AD7887 ADC driver platform data
// @en_dual: Whether to use dual channel mode. If set to true AIN1 becomes the
// second input channel, and Vref is internally connected to Vdd. If set to
// false the device is used in single channel mode and AIN1/Vref is used as
// VREF input.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7887_platform_data {
    pub en_dual: bool,
}
