//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/neon-intrinsics.h
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
// Copyright (C) 2018 Linaro, Ltd. <ard.biesheuvel@linaro.org>
//

//
// In the kernel, u64/s64 are [un]signed long long, not [un]signed long.
// So by redefining these macros to the former, we can force gcc-stdint.h
// to define uint64_t / in64_t in a compatible manner.
//

//
// genksyms chokes on the ARM NEON instrinsics system header, but we
// don't export anything it defines anyway, so just disregard when
// genksyms execute.
//

