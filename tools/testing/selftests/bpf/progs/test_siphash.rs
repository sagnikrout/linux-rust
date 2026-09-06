//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/test_siphash.h
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
// Copyright Amazon.com Inc. or its affiliates.
// include/linux/bitops.h
// include/linux/siphash.h

pub const SIPHASH_CONST_0: c_uint = 0x736f6d6570736575ULL;
pub const SIPHASH_CONST_1: c_uint = 0x646f72616e646f6dULL;
pub const SIPHASH_CONST_2: c_uint = 0x6c7967656e657261ULL;
pub const SIPHASH_CONST_3: c_uint = 0x7465646279746573ULL;
// lib/siphash.c

