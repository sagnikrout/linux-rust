//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/current.h
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
// Based on arm/arm64/include/asm/current.h
//
// Copyright (C) 2016 ARM
// Copyright (C) 2017 SiFive
//

extern "C" {
    pub fn __asm__(_arg: "tp") -> *mut register struct task_struct riscv_current_is_tp;
}
//
// This only works because "struct thread_info" is at offset 0 from "struct
// task_struct".  This constraint seems to be necessary on other architectures
// as well, but __switch_to enforces it.  We can't check TASK_TI here because
// <asm/asm-offsets.h> includes this, and I can't get the definition of "struct
// task_struct" here due to some header ordering problems.
//

extern "C" {
    pub fn __asm__(_arg: "sp") -> register unsigned long current_stack_pointer;
}

