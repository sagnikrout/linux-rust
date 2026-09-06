//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sun50i-a100-r-ccu.h
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
// Copyright (c) 2020 Yangtao Li <frank@allwinnertech.com>
//
pub const CLK_R_APB1: c_int = 2;
pub const CLK_R_APB1_TIMER: c_int = 4;
pub const CLK_R_APB1_TWD: c_int = 5;
pub const CLK_R_APB1_PWM: c_int = 6;
pub const CLK_R_APB1_BUS_PWM: c_int = 7;
pub const CLK_R_APB1_PPU: c_int = 8;
pub const CLK_R_APB2_UART: c_int = 9;
pub const CLK_R_APB2_I2C0: c_int = 10;
pub const CLK_R_APB2_I2C1: c_int = 11;
pub const CLK_R_APB1_IR: c_int = 12;
pub const CLK_R_APB1_BUS_IR: c_int = 13;
pub const CLK_R_AHB_BUS_RTC: c_int = 14;
