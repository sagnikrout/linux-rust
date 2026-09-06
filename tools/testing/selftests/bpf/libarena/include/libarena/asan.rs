//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/libarena/include/libarena/asan.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asan_init_args {
    pub arena_all_pages: u64,
    pub arena_globals_pages: u64,
}

extern "C" {
    pub fn asan_init(args: *mut asan_init_args) -> c_int;
}

pub const ASAN_SHADOW_SHIFT: c_int = 3;

extern "C" {
    pub fn asan_poison(addr: *mut void __arena, val: i8, size: usize) -> c_int;
}
extern "C" {
    pub fn asan_unpoison(addr: *mut void __arena, size: usize) -> c_int;
}
extern "C" {
    pub fn asan_shadow_set(addr: *mut void __arena) -> bool;
}
//
// Dummy calls to ensure the ASAN runtime's BTF information is present
// in every object file when compiling the runtime and local BPF code
// separately. The runtime calls are injected into the LLVM IR file
//

extern "C" {
    pub fn __asan_storeN(addr: intptr_t, size: isize);
}
extern "C" {
    pub fn __asan_storeN_noabort(addr: intptr_t, size: isize);
}
extern "C" {
    pub fn __asan_loadN(addr: intptr_t, size: isize);
}
extern "C" {
    pub fn __asan_loadN_noabort(addr: intptr_t, size: isize);
}
//
// Force LLVM to emit BTF information for the stubs,
// because the ASAN pass in LLVM by itself doesn't.
//

