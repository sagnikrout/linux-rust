//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/r8a779a0-sysc.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2020 Renesas Electronics Corp.
//
// These power domain indices match the Power Domain Register Numbers (PDR)
//
pub const R8A779A0_PD_A1E0D0C0: c_int = 0;
pub const R8A779A0_PD_A1E0D0C1: c_int = 1;
pub const R8A779A0_PD_A1E0D1C0: c_int = 2;
pub const R8A779A0_PD_A1E0D1C1: c_int = 3;
pub const R8A779A0_PD_A1E1D0C0: c_int = 4;
pub const R8A779A0_PD_A1E1D0C1: c_int = 5;
pub const R8A779A0_PD_A1E1D1C0: c_int = 6;
pub const R8A779A0_PD_A1E1D1C1: c_int = 7;
pub const R8A779A0_PD_A2E0D0: c_int = 16;
pub const R8A779A0_PD_A2E0D1: c_int = 17;
pub const R8A779A0_PD_A2E1D0: c_int = 18;
pub const R8A779A0_PD_A2E1D1: c_int = 19;
pub const R8A779A0_PD_A3E0: c_int = 20;
pub const R8A779A0_PD_A3E1: c_int = 21;
pub const R8A779A0_PD_3DG_A: c_int = 24;
pub const R8A779A0_PD_3DG_B: c_int = 25;
pub const R8A779A0_PD_A1CNN2: c_int = 32;
pub const R8A779A0_PD_A1DSP0: c_int = 33;
pub const R8A779A0_PD_A2IMP01: c_int = 34;
pub const R8A779A0_PD_A2DP0: c_int = 35;
pub const R8A779A0_PD_A2CV0: c_int = 36;
pub const R8A779A0_PD_A2CV1: c_int = 37;
pub const R8A779A0_PD_A2CV4: c_int = 38;
pub const R8A779A0_PD_A2CV6: c_int = 39;
pub const R8A779A0_PD_A2CN2: c_int = 40;
pub const R8A779A0_PD_A1CNN0: c_int = 41;
pub const R8A779A0_PD_A2CN0: c_int = 42;
pub const R8A779A0_PD_A3IR: c_int = 43;
pub const R8A779A0_PD_A1CNN1: c_int = 44;
pub const R8A779A0_PD_A1DSP1: c_int = 45;
pub const R8A779A0_PD_A2IMP23: c_int = 46;
pub const R8A779A0_PD_A2DP1: c_int = 47;
pub const R8A779A0_PD_A2CV2: c_int = 48;
pub const R8A779A0_PD_A2CV3: c_int = 49;
pub const R8A779A0_PD_A2CV5: c_int = 50;
pub const R8A779A0_PD_A2CV7: c_int = 51;
pub const R8A779A0_PD_A2CN1: c_int = 52;
pub const R8A779A0_PD_A3VIP0: c_int = 56;
pub const R8A779A0_PD_A3VIP1: c_int = 57;
pub const R8A779A0_PD_A3VIP2: c_int = 58;
pub const R8A779A0_PD_A3VIP3: c_int = 59;
pub const R8A779A0_PD_A3ISP01: c_int = 60;
pub const R8A779A0_PD_A3ISP23: c_int = 61;
// Always-on power area
pub const R8A779A0_PD_ALWAYS_ON: c_int = 64;
