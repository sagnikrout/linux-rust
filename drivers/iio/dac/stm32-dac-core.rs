//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/dac/stm32-dac-core.h
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
// This file is part of STM32 DAC driver
//
// Copyright (C) 2017, STMicroelectronics - All Rights Reserved
// Author: Fabrice Gasnier <fabrice.gasnier@st.com>.
//

// STM32 DAC registers
pub const STM32_DAC_CR: c_uint = 0x00;
pub const STM32_DAC_DHR12R1: c_uint = 0x08;
pub const STM32_DAC_DHR12R2: c_uint = 0x14;
pub const STM32_DAC_DOR1: c_uint = 0x2C;
pub const STM32_DAC_DOR2: c_uint = 0x30;
// STM32_DAC_CR bit fields

//
// struct stm32_dac_common - stm32 DAC driver common data (for all instances)
// @regmap: DAC registers shared via regmap
// @vref_mv: reference voltage (mv)
// @hfsel: high speed bus clock selected
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_dac_common {
    pub regmap: *mut regmap,
    pub vref_mv: c_int,
    pub hfsel: bool,
}
