//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/libarena/include/bpf_atomic.h
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

//
// __unqual_typeof(x) - Declare an unqualified scalar type, leaving
// non-scalar types unchanged,
//
// Prefer C11 _Generic for better compile-times and simpler code. Note: 'char'
// is not type-compatible with 'signed char', and we define a separate case.
//
// This is copied verbatim from kernel's include/linux/compiler_types.h, but
// with default expression (for pointers) changed from (x) to (typeof(x)0).
//
// This is because LLVM has a bug where for lvalue (x), it does not get rid of
// an extra address_space qualifier, but does in case of rvalue (typeof(x)0).
// Hence, for pointers, we need to create an rvalue expression to get the
// desired type. See https://github.com/llvm/llvm-project/issues/53400.
//

// No-op for BPF

// (pold) = __r;                            \

// Control dependency provides LOAD->STORE, provide LOAD->LOAD

//
// Clang advertises this feature when it can lower acquire/release atomic
// builtins to BPF_LOAD_ACQ/BPF_STORE_REL. Older compilers keep using the
// barrier-based fallback below. The generated instructions require kernel
// verifier/JIT support added in Linux 6.15; compile for an older BPF CPU to
// keep using the fallback when targeting older kernels.
//

