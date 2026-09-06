//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/pistachio-resets.h
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
// This header provides constants for the reset controller
// present in the Pistachio SoC
//
pub const PISTACHIO_RESET_I2C0: c_int = 0;
pub const PISTACHIO_RESET_I2C1: c_int = 1;
pub const PISTACHIO_RESET_I2C2: c_int = 2;
pub const PISTACHIO_RESET_I2C3: c_int = 3;
pub const PISTACHIO_RESET_I2S_IN: c_int = 4;
pub const PISTACHIO_RESET_PRL_OUT: c_int = 5;
pub const PISTACHIO_RESET_SPDIF_OUT: c_int = 6;
pub const PISTACHIO_RESET_SPI: c_int = 7;
pub const PISTACHIO_RESET_PWM_PDM: c_int = 8;
pub const PISTACHIO_RESET_UART0: c_int = 9;
pub const PISTACHIO_RESET_UART1: c_int = 10;
pub const PISTACHIO_RESET_QSPI: c_int = 11;
pub const PISTACHIO_RESET_MDC: c_int = 12;
pub const PISTACHIO_RESET_SDHOST: c_int = 13;
pub const PISTACHIO_RESET_ETHERNET: c_int = 14;
pub const PISTACHIO_RESET_IR: c_int = 15;
pub const PISTACHIO_RESET_HASH: c_int = 16;
pub const PISTACHIO_RESET_TIMER: c_int = 17;
pub const PISTACHIO_RESET_I2S_OUT: c_int = 18;
pub const PISTACHIO_RESET_SPDIF_IN: c_int = 19;
pub const PISTACHIO_RESET_EVT: c_int = 20;
pub const PISTACHIO_RESET_USB_H: c_int = 21;
pub const PISTACHIO_RESET_USB_PR: c_int = 22;
pub const PISTACHIO_RESET_USB_PHY_PR: c_int = 23;
pub const PISTACHIO_RESET_USB_PHY_PON: c_int = 24;
pub const PISTACHIO_RESET_MAX: c_int = 24;
