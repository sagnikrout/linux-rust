//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mobileye,eyeq6lplus-clk.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) 2025 Mobileye Vision Technologies Ltd.
//
pub const EQ6LPC_PLL_CPU: c_int = 0;
pub const EQ6LPC_PLL_DDR: c_int = 1;
pub const EQ6LPC_PLL_PER: c_int = 2;
pub const EQ6LPC_PLL_VDI: c_int = 3;
pub const EQ6LPC_PLL_ACC: c_int = 4;
pub const EQ6LPC_CPU_OCC: c_int = 5;
pub const EQ6LPC_ACC_VDI: c_int = 6;
pub const EQ6LPC_ACC_OCC: c_int = 7;
pub const EQ6LPC_ACC_FCMU: c_int = 8;
pub const EQ6LPC_DDR_OCC: c_int = 9;
pub const EQ6LPC_PER_OCC: c_int = 10;
pub const EQ6LPC_PER_I2C_SER: c_int = 11;
pub const EQ6LPC_PER_PCLK: c_int = 12;
pub const EQ6LPC_PER_TSU: c_int = 13;
pub const EQ6LPC_PER_OSPI: c_int = 14;
pub const EQ6LPC_PER_GPIO: c_int = 15;
pub const EQ6LPC_PER_TIMER: c_int = 16;
pub const EQ6LPC_PER_I2C: c_int = 17;
pub const EQ6LPC_PER_UART: c_int = 18;
pub const EQ6LPC_PER_SPI: c_int = 19;
pub const EQ6LPC_PER_PERIPH: c_int = 20;
pub const EQ6LPC_VDI_OCC: c_int = 21;
