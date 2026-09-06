//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/rwonce.h
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
//
// Copyright (C) 2020 Google LLC.
//

//
// Replace this with typeof_unqual() when minimum compiler versions are
// increased to GCC 14 and Clang 19. For the time being, we need this
// workaround, which relies on function return values dropping qualifiers.
//

//
// When building with LTO, there is an increased risk of the compiler
// converting an address dependency headed by a READ_ONCE() invocation
// into a control dependency and consequently allowing for harmful
// reordering by the CPU.
//
// Ensure that such transformations are harmless by overriding the generic
// READ_ONCE() definition with one that provides RCpc acquire semantics
// when building with LTO.
//

// Hides alias reassignment from Clang's -Wthread-safety. */	\
// __retp = &__u.__val;						\
// __ret;								\

