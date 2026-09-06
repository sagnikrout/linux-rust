//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/perf_event.h
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
// Performance event support - hardware-specific disambiguation
//
// For now this is a compile-time decision, but eventually it should be
// runtime.  This would allow multiplatform perf event support for e300 (fsl
// embedded perf counters) plus server/classic, and would accommodate
// devices other than the core which provide their own performance counters.
//
// Copyright 2010 Freescale Semiconductor, Inc.
//

//
// Overload regs->result to specify whether we should use the MSR (result
// is zero) or the SIAR (result is non zero).
//

// To support perf_regs sier update
extern "C" {
    pub fn is_sier_available() -> bool;
}
extern "C" {
    pub fn get_pmcs_ext_regs(idx: c_int) -> c_ulong;
}
// To define perf extended regs mask value

