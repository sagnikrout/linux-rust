//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/iio/adc/at91-sama5d2_adc.h
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
// This header provides constants for configuring the AT91 SAMA5D2 ADC
//
// X relative position channel index
pub const AT91_SAMA5D2_ADC_X_CHANNEL: c_int = 24;
// Y relative position channel index
pub const AT91_SAMA5D2_ADC_Y_CHANNEL: c_int = 25;
// pressure channel index
pub const AT91_SAMA5D2_ADC_P_CHANNEL: c_int = 26;
// SAMA7G5 Temperature sensor channel index.
pub const AT91_SAMA7G5_ADC_TEMP_CHANNEL: c_int = 31;
