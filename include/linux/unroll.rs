//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/unroll.h
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
// Copyright (C) 2023 Google LLC.
//

//
// unrolled - loop attributes to ask the compiler to unroll it
//
// Usage:
//
// #define BATCH 8
//
// unrolled_count(BATCH)
// for (u32 i = 0; i < BATCH; i++)
// // loop body without cross-iteration dependencies
//
// This is only a hint and the compiler is free to disable unrolling if it
// thinks the count is suboptimal and may hurt performance and/or hugely
// increase object code size.
// Not having any cross-iteration dependencies (i.e. when iter x + 1 depends
// on what iter x will do with variables) is not a strict requirement, but
// provides best performance and object code size.
// Available only on Clang and GCC 8.x onwards.
//
// Ask the compiler to pick an optimal unroll count, Clang only

// Unroll each @n iterations of the loop

// Unroll the whole loop

// Never unroll the loop

