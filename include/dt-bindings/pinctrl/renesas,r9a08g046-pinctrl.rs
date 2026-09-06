//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/renesas,r9a08g046-pinctrl.h
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
// This header provides constants for Renesas RZ/G3L family pinctrl bindings.
//
// Copyright (C) 2026 Renesas Electronics Corp.
//

// RZG3L_Px = Offset address of PFC_P_mn  - 0x22
pub const RZG3L_P2: c_int = 2;
pub const RZG3L_P3: c_int = 3;
pub const RZG3L_P5: c_int = 5;
pub const RZG3L_P6: c_int = 6;
pub const RZG3L_P7: c_int = 7;
pub const RZG3L_P8: c_int = 8;
pub const RZG3L_PA: c_int = 10;
pub const RZG3L_PB: c_int = 11;
pub const RZG3L_PC: c_int = 12;
pub const RZG3L_PD: c_int = 13;
pub const RZG3L_PE: c_int = 14;
pub const RZG3L_PF: c_int = 15;
pub const RZG3L_PG: c_int = 16;
pub const RZG3L_PH: c_int = 17;
pub const RZG3L_PJ: c_int = 19;
pub const RZG3L_PK: c_int = 20;
pub const RZG3L_PL: c_int = 21;
pub const RZG3L_PM: c_int = 22;
pub const RZG3L_PS: c_int = 28;

