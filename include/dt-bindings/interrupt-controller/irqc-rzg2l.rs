//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interrupt-controller/irqc-rzg2l.h
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
// This header provides constants for Renesas RZ/G2L family IRQC bindings.
//
// Copyright (C) 2022 Renesas Electronics Corp.
//
// NMI maps to SPI0
pub const RZG2L_NMI: c_int = 0;
// IRQ0-7 map to SPI1-8
pub const RZG2L_IRQ0: c_int = 1;
pub const RZG2L_IRQ1: c_int = 2;
pub const RZG2L_IRQ2: c_int = 3;
pub const RZG2L_IRQ3: c_int = 4;
pub const RZG2L_IRQ4: c_int = 5;
pub const RZG2L_IRQ5: c_int = 6;
pub const RZG2L_IRQ6: c_int = 7;
pub const RZG2L_IRQ7: c_int = 8;
