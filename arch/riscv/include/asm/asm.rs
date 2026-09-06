//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/asm.h
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
// Copyright (C) 2015 Regents of the University of California
//

pub const RISCV_SZPTR: c_int = 8;
pub const RISCV_LGPTR: c_int = 3;

pub const RISCV_SZPTR: c_int = 4;
pub const RISCV_LGPTR: c_int = 2;

// Common assembly source macros
//
// NOP sequence
//

// gp is used as the shadow call stack pointer instead

// load __global_pointer to gp

// save all GPs except x1 ~ x5
// restore all GPs except x1 ~ x5
// Annotate a function as being unsuitable for kprobes.

// Macro flag: #define ASM_NOKPROBE(name)

