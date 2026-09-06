//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/toshiba,tmpv770x.h
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
// Reset
pub const TMPV770X_RESET_PIETHER_2P5M: c_int = 0;
pub const TMPV770X_RESET_PIETHER_25M: c_int = 1;
pub const TMPV770X_RESET_PIETHER_50M: c_int = 2;
pub const TMPV770X_RESET_PIETHER_125M: c_int = 3;
pub const TMPV770X_RESET_HOX: c_int = 4;
pub const TMPV770X_RESET_PCIE_MSTR: c_int = 5;
pub const TMPV770X_RESET_PCIE_AUX: c_int = 6;
pub const TMPV770X_RESET_PIINTC: c_int = 7;
pub const TMPV770X_RESET_PIETHER_BUS: c_int = 8;
pub const TMPV770X_RESET_PISPI0: c_int = 9;
pub const TMPV770X_RESET_PISPI1: c_int = 10;
pub const TMPV770X_RESET_PISPI2: c_int = 11;
pub const TMPV770X_RESET_PISPI3: c_int = 12;
pub const TMPV770X_RESET_PISPI4: c_int = 13;
pub const TMPV770X_RESET_PISPI5: c_int = 14;
pub const TMPV770X_RESET_PISPI6: c_int = 15;
pub const TMPV770X_RESET_PIUART0: c_int = 16;
pub const TMPV770X_RESET_PIUART1: c_int = 17;
pub const TMPV770X_RESET_PIUART2: c_int = 18;
pub const TMPV770X_RESET_PIUART3: c_int = 19;
pub const TMPV770X_RESET_PII2C0: c_int = 20;
pub const TMPV770X_RESET_PII2C1: c_int = 21;
pub const TMPV770X_RESET_PII2C2: c_int = 22;
pub const TMPV770X_RESET_PII2C3: c_int = 23;
pub const TMPV770X_RESET_PII2C4: c_int = 24;
pub const TMPV770X_RESET_PII2C5: c_int = 25;
pub const TMPV770X_RESET_PII2C6: c_int = 26;
pub const TMPV770X_RESET_PII2C7: c_int = 27;
pub const TMPV770X_RESET_PII2C8: c_int = 28;
pub const TMPV770X_RESET_PIPCMIF: c_int = 29;
pub const TMPV770X_RESET_PICKMON: c_int = 30;
pub const TMPV770X_RESET_SBUSCLK: c_int = 31;
pub const TMPV770X_RESET_VIIFBS0: c_int = 32;
pub const TMPV770X_RESET_VIIFBS0_APB: c_int = 33;
pub const TMPV770X_RESET_VIIFBS0_L2ISP: c_int = 34;
pub const TMPV770X_RESET_VIIFBS0_L1ISP: c_int = 35;
pub const TMPV770X_RESET_VIIFBS1: c_int = 36;
pub const TMPV770X_RESET_VIIFBS1_APB: c_int = 37;
pub const TMPV770X_RESET_VIIFBS1_L2ISP: c_int = 38;
pub const TMPV770X_RESET_VIIFBS1_L1ISP: c_int = 39;
