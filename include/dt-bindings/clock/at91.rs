//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/at91.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// This header provides constants for AT91 pmc status.
//
// The constants defined in this header are being used in dts.
//
pub const PMC_TYPE_CORE: c_int = 0;
pub const PMC_TYPE_SYSTEM: c_int = 1;
pub const PMC_TYPE_PERIPHERAL: c_int = 2;
pub const PMC_TYPE_GCK: c_int = 3;
pub const PMC_TYPE_PROGRAMMABLE: c_int = 4;
pub const PMC_SLOW: c_int = 0;
pub const PMC_MCK: c_int = 1;
pub const PMC_UTMI: c_int = 2;
pub const PMC_MAIN: c_int = 3;
pub const PMC_MCK2: c_int = 4;
pub const PMC_I2S0_MUX: c_int = 5;
pub const PMC_I2S1_MUX: c_int = 6;
pub const PMC_PLLACK: c_int = 7;
pub const PMC_PLLBCK: c_int = 8;
pub const PMC_AUDIOPLLCK: c_int = 9;
pub const PMC_AUDIOPINCK: c_int = 10;
// SAMA7G5

// SAM9X7

// SAMA7D65

// Slow clock.
pub const SCKC_MD_SLCK: c_int = 0;
pub const SCKC_TD_SLCK: c_int = 1;
