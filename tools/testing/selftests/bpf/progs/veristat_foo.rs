//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/veristat_foo.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

//
// Programs below exist only to exercise veristat's -f name filters,
// their bodies are irrelevant, only the names matter.
// This file is also included by veristat_bar.c, so that the same set of
// program names is available in two differently named object files.
//
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn foo(ctx: *mut c_void) -> c_int {
    int foo(void *ctx)
    {
    return 0;
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn bar(ctx: *mut c_void) -> c_int {
    int bar(void *ctx)
    {
    return 0;
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn buz(ctx: *mut c_void) -> c_int {
    int buz(void *ctx)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
