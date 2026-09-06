//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/iio/adc/mediatek,mt6359-auxadc.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
// ADC Channel Index
pub const MT6359_AUXADC_BATADC: c_int = 0;
pub const MT6359_AUXADC_BAT_TEMP: c_int = 1;
pub const MT6359_AUXADC_CHIP_TEMP: c_int = 2;
pub const MT6359_AUXADC_ACCDET: c_int = 3;
pub const MT6359_AUXADC_VDCXO: c_int = 4;
pub const MT6359_AUXADC_TSX_TEMP: c_int = 5;
pub const MT6359_AUXADC_HPOFS_CAL: c_int = 6;
pub const MT6359_AUXADC_DCXO_TEMP: c_int = 7;
pub const MT6359_AUXADC_VBIF: c_int = 8;
pub const MT6359_AUXADC_VCORE_TEMP: c_int = 9;
pub const MT6359_AUXADC_VPROC_TEMP: c_int = 10;
pub const MT6359_AUXADC_VGPU_TEMP: c_int = 11;
pub const MT6359_AUXADC_VBAT: c_int = 12;
pub const MT6359_AUXADC_IBAT: c_int = 13;
