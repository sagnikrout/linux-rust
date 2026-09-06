//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/r8a779f0-sysc.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (C) 2021 Renesas Electronics Corp.
//
// These power domain indices match the Power Domain Register Numbers (PDR)
//
pub const R8A779F0_PD_A1E0D0C0: c_int = 0;
pub const R8A779F0_PD_A1E0D0C1: c_int = 1;
pub const R8A779F0_PD_A1E0D1C0: c_int = 2;
pub const R8A779F0_PD_A1E0D1C1: c_int = 3;
pub const R8A779F0_PD_A1E1D0C0: c_int = 4;
pub const R8A779F0_PD_A1E1D0C1: c_int = 5;
pub const R8A779F0_PD_A1E1D1C0: c_int = 6;
pub const R8A779F0_PD_A1E1D1C1: c_int = 7;
pub const R8A779F0_PD_A2E0D0: c_int = 16;
pub const R8A779F0_PD_A2E0D1: c_int = 17;
pub const R8A779F0_PD_A2E1D0: c_int = 18;
pub const R8A779F0_PD_A2E1D1: c_int = 19;
pub const R8A779F0_PD_A3E0: c_int = 20;
pub const R8A779F0_PD_A3E1: c_int = 21;
// Always-on power area
pub const R8A779F0_PD_ALWAYS_ON: c_int = 64;
