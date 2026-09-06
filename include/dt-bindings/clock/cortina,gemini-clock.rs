//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/cortina,gemini-clock.h
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
// RTC, AHB, APB, CPU, PCI, TVC, UART clocks and 13 gates
pub const GEMINI_NUM_CLKS: c_int = 20;
pub const GEMINI_CLK_RTC: c_int = 0;
pub const GEMINI_CLK_AHB: c_int = 1;
pub const GEMINI_CLK_APB: c_int = 2;
pub const GEMINI_CLK_CPU: c_int = 3;
pub const GEMINI_CLK_PCI: c_int = 4;
pub const GEMINI_CLK_TVC: c_int = 5;
pub const GEMINI_CLK_UART: c_int = 6;
pub const GEMINI_CLK_GATES: c_int = 7;
pub const GEMINI_CLK_GATE_SECURITY: c_int = 7;
pub const GEMINI_CLK_GATE_GMAC0: c_int = 8;
pub const GEMINI_CLK_GATE_GMAC1: c_int = 9;
pub const GEMINI_CLK_GATE_SATA0: c_int = 10;
pub const GEMINI_CLK_GATE_SATA1: c_int = 11;
pub const GEMINI_CLK_GATE_USB0: c_int = 12;
pub const GEMINI_CLK_GATE_USB1: c_int = 13;
pub const GEMINI_CLK_GATE_IDE: c_int = 14;
pub const GEMINI_CLK_GATE_PCI: c_int = 15;
pub const GEMINI_CLK_GATE_DDR: c_int = 16;
pub const GEMINI_CLK_GATE_FLASH: c_int = 17;
pub const GEMINI_CLK_GATE_TVC: c_int = 18;
pub const GEMINI_CLK_GATE_BOOT: c_int = 19;
