//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/timex.h
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
// Copyright (C) 2012 Regents of the University of California
//

pub type cycles_t = c_ulong;

extern "C" {
    pub fn readq_relaxed(_arg: clint_time_val) -> return;
}

extern "C" {
    pub fn readl_relaxed()clint_time_val): *mut ((u32 __iomem) -> return;
}

extern "C" {
    pub fn readl_relaxed(1: *mut *mut ((u32 __iomem )clint_time_val) +) -> return;
}

//
// Much like MIPS, we may not have a viable counter to use at an early point
// in the boot process. Unfortunately we don't have a fallback, so instead
// we just return 0.
//
extern "C" {
    pub fn random_get_entropy_fallback() -> return;
}
extern "C" {
    pub fn get_cycles() -> return;
}

extern "C" {
    pub fn csr_read(_arg: CSR_TIME) -> return;
}

extern "C" {
    pub fn csr_read(_arg: CSR_TIMEH) -> return;
}

extern "C" {
    pub fn get_cycles() -> return;
}

