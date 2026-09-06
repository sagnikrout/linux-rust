//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/clint.h
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
// Copyright (C) 2020 Google, Inc
//

//
// This lives in the CLINT driver, but is accessed directly by timex.h to avoid
// any overhead when accessing the MMIO timer.
//
// The ISA defines mtime as a 64-bit memory-mapped register that increments at
// a constant frequency, but it doesn't define some other constraints we depend
// on (most notably ordering constraints, but also some simpler stuff like the
// memory layout).  Thus, this is called "clint_time_val" instead of something
// like "riscv_mtime", to signify that these non-ISA assumptions must hold.
//

