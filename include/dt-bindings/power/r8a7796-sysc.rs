//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/r8a7796-sysc.h
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
// Copyright (C) 2016 Glider bvba
//
// These power domain indices match the numbers of the interrupt bits
// representing the power areas in the various Interrupt Registers
// (e.g. SYSCISR, Interrupt Status Register)
//
pub const R8A7796_PD_CA57_CPU0: c_int = 0;
pub const R8A7796_PD_CA57_CPU1: c_int = 1;
pub const R8A7796_PD_CA53_CPU0: c_int = 5;
pub const R8A7796_PD_CA53_CPU1: c_int = 6;
pub const R8A7796_PD_CA53_CPU2: c_int = 7;
pub const R8A7796_PD_CA53_CPU3: c_int = 8;
pub const R8A7796_PD_CA57_SCU: c_int = 12;
pub const R8A7796_PD_CR7: c_int = 13;
pub const R8A7796_PD_A3VC: c_int = 14;
pub const R8A7796_PD_3DG_A: c_int = 17;
pub const R8A7796_PD_3DG_B: c_int = 18;
pub const R8A7796_PD_CA53_SCU: c_int = 21;
pub const R8A7796_PD_A3IR: c_int = 24;
pub const R8A7796_PD_A2VC0: c_int = 25;
pub const R8A7796_PD_A2VC1: c_int = 26;
// Always-on power area
pub const R8A7796_PD_ALWAYS_ON: c_int = 32;
