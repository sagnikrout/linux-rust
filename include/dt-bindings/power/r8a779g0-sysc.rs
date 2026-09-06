//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/r8a779g0-sysc.h
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
// Copyright (C) 2022 Renesas Electronics Corp.
//
// These power domain indices match the Power Domain Register Numbers (PDR)
//
pub const R8A779G0_PD_A1E0D0C0: c_int = 0;
pub const R8A779G0_PD_A1E0D0C1: c_int = 1;
pub const R8A779G0_PD_A1E0D1C0: c_int = 2;
pub const R8A779G0_PD_A1E0D1C1: c_int = 3;
pub const R8A779G0_PD_A2E0D0: c_int = 16;
pub const R8A779G0_PD_A2E0D1: c_int = 17;
pub const R8A779G0_PD_A3E0: c_int = 20;
pub const R8A779G0_PD_A33DGA: c_int = 24;
pub const R8A779G0_PD_A23DGB: c_int = 25;
pub const R8A779G0_PD_A1DSP0: c_int = 33;
pub const R8A779G0_PD_A2IMP01: c_int = 34;
pub const R8A779G0_PD_A2PSC: c_int = 35;
pub const R8A779G0_PD_A2CV0: c_int = 36;
pub const R8A779G0_PD_A2CV1: c_int = 37;
pub const R8A779G0_PD_A1CNN0: c_int = 41;
pub const R8A779G0_PD_A2CN0: c_int = 42;
pub const R8A779G0_PD_A3IR: c_int = 43;
pub const R8A779G0_PD_A1DSP1: c_int = 45;
pub const R8A779G0_PD_A2IMP23: c_int = 46;
pub const R8A779G0_PD_A2DMA: c_int = 47;
pub const R8A779G0_PD_A2CV2: c_int = 48;
pub const R8A779G0_PD_A2CV3: c_int = 49;
pub const R8A779G0_PD_A1DSP2: c_int = 53;
pub const R8A779G0_PD_A1DSP3: c_int = 54;
pub const R8A779G0_PD_A3VIP0: c_int = 56;
pub const R8A779G0_PD_A3VIP1: c_int = 57;
pub const R8A779G0_PD_A3VIP2: c_int = 58;
pub const R8A779G0_PD_A3ISP0: c_int = 60;
pub const R8A779G0_PD_A3ISP1: c_int = 61;
pub const R8A779G0_PD_A3DUL: c_int = 62;
// Always-on power area
pub const R8A779G0_PD_ALWAYS_ON: c_int = 64;
