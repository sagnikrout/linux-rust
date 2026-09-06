//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/cortina,gemini-reset.h
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
pub const GEMINI_RESET_DRAM: c_int = 0;
pub const GEMINI_RESET_FLASH: c_int = 1;
pub const GEMINI_RESET_IDE: c_int = 2;
pub const GEMINI_RESET_RAID: c_int = 3;
pub const GEMINI_RESET_SECURITY: c_int = 4;
pub const GEMINI_RESET_GMAC0: c_int = 5;
pub const GEMINI_RESET_GMAC1: c_int = 6;
pub const GEMINI_RESET_PCI: c_int = 7;
pub const GEMINI_RESET_USB0: c_int = 8;
pub const GEMINI_RESET_USB1: c_int = 9;
pub const GEMINI_RESET_DMAC: c_int = 10;
pub const GEMINI_RESET_APB: c_int = 11;
pub const GEMINI_RESET_LPC: c_int = 12;
pub const GEMINI_RESET_LCD: c_int = 13;
pub const GEMINI_RESET_INTCON0: c_int = 14;
pub const GEMINI_RESET_INTCON1: c_int = 15;
pub const GEMINI_RESET_RTC: c_int = 16;
pub const GEMINI_RESET_TIMER: c_int = 17;
pub const GEMINI_RESET_UART: c_int = 18;
pub const GEMINI_RESET_SSP: c_int = 19;
pub const GEMINI_RESET_GPIO0: c_int = 20;
pub const GEMINI_RESET_GPIO1: c_int = 21;
pub const GEMINI_RESET_GPIO2: c_int = 22;
pub const GEMINI_RESET_WDOG: c_int = 23;
pub const GEMINI_RESET_EXTERN: c_int = 24;
pub const GEMINI_RESET_CIR: c_int = 25;
pub const GEMINI_RESET_SATA0: c_int = 26;
pub const GEMINI_RESET_SATA1: c_int = 27;
pub const GEMINI_RESET_TVC: c_int = 28;
pub const GEMINI_RESET_CPU1: c_int = 30;
pub const GEMINI_RESET_GLOBAL: c_int = 31;
