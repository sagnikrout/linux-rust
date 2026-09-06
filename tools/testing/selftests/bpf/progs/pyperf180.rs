//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/pyperf180.c
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
pub const STACK_MAX_LEN: c_int = 180;
// llvm upstream commit at clang18
// https://github.com/llvm/llvm-project/commit/1a2e77cf9e11dbf56b5720c607313a566eebb16e
// changed inlining behavior and caused compilation failure as some branch
// target distance exceeded 16bit representation which is the maximum for
// cpu v1/v2/v3. Macro __BPF_CPU_VERSION__ is later implemented in clang18
// to specify which cpu version is used for compilation. So a smaller
// unroll_count can be set if __BPF_CPU_VERSION__ is less than 4, which
// reduced some branch target distances and resolved the compilation failure.
//
// To capture the case where a developer/ci uses clang18 but the corresponding
// repo checkpoint does not have __BPF_CPU_VERSION__, a smaller unroll_count
// will be set as well to prevent potential compilation failures.
//

pub const UNROLL_COUNT: c_int = 90;

pub const UNROLL_COUNT: c_int = 90;

