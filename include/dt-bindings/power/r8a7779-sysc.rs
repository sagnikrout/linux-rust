//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/r8a7779-sysc.h
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
pub const R8A7779_PD_ARM1: c_int = 1;
pub const R8A7779_PD_ARM2: c_int = 2;
pub const R8A7779_PD_ARM3: c_int = 3;
pub const R8A7779_PD_SGX: c_int = 20;
pub const R8A7779_PD_VDP: c_int = 21;
pub const R8A7779_PD_IMP: c_int = 24;
// Always-on power area
pub const R8A7779_PD_ALWAYS_ON: c_int = 32;
