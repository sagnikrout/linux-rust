//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/libarena/include/libarena/common.h
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

//
// This is a variable used to aid verification. The may_goto directive
// permits open-coded for loops, but requires that the index variable is
// imprecise. To force the variable to be imprecise, initialize it with
// the opaque volatile variable 0 instead of the constant 0.
//
extern "C" {
    pub fn arena_fls(word: __u64) -> c_int;
}
extern "C" {
    pub fn arena_free(ptr: *mut void __arena);
}
//
// The verifier associates arenas with programs by checking LD.IMM
// instruction operands for an arena and populating the program state
// with the first instance it finds. This requires accessing our global
// arena variable, but subprogs do not necessarily do so while still
// using pointers from that arena. Insert an LD.IMM instruction  to
// access the arena and help the verifier.
//

// Macro flag: #define __arena
pub type u8 = u8;
pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;
pub type s8 = i8;
pub type s16 = i16;
pub type s32 = i32;
pub type s64 = i64;
// Dummy "definition" for userspace.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arena_get_info_args {
    pub arena_base: *mut void __arena,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arena_alloc_reserve_args {
    pub nr_pages: u64,
}

// Reasonable default number of pages reserved by arena_alloc_reserve.
