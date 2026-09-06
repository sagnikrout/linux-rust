//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sun55i-a523-r-ccu.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (C) 2024 Arm Ltd.
//
pub const CLK_R_AHB: c_int = 0;
pub const CLK_R_APB0: c_int = 1;
pub const CLK_R_APB1: c_int = 2;
pub const CLK_R_TIMER0: c_int = 3;
pub const CLK_R_TIMER1: c_int = 4;
pub const CLK_R_TIMER2: c_int = 5;
pub const CLK_BUS_R_TIMER: c_int = 6;
pub const CLK_BUS_R_TWD: c_int = 7;
pub const CLK_R_PWMCTRL: c_int = 8;
pub const CLK_BUS_R_PWMCTRL: c_int = 9;
pub const CLK_R_SPI: c_int = 10;
pub const CLK_BUS_R_SPI: c_int = 11;
pub const CLK_BUS_R_SPINLOCK: c_int = 12;
pub const CLK_BUS_R_MSGBOX: c_int = 13;
pub const CLK_BUS_R_UART0: c_int = 14;
pub const CLK_BUS_R_UART1: c_int = 15;
pub const CLK_BUS_R_I2C0: c_int = 16;
pub const CLK_BUS_R_I2C1: c_int = 17;
pub const CLK_BUS_R_I2C2: c_int = 18;
pub const CLK_BUS_R_PPU0: c_int = 19;
pub const CLK_BUS_R_PPU1: c_int = 20;
pub const CLK_BUS_R_CPU_BIST: c_int = 21;
pub const CLK_R_IR_RX: c_int = 22;
pub const CLK_BUS_R_IR_RX: c_int = 23;
pub const CLK_BUS_R_DMA: c_int = 24;
pub const CLK_BUS_R_RTC: c_int = 25;
pub const CLK_BUS_R_CPUCFG: c_int = 26;
