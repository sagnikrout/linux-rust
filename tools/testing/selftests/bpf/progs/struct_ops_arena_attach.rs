//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_arena_attach.c
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

    SEC("fentry")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_test_arena, st_ops_ctx: *mut c_ulonglong) -> c_int {
    int BPF_PROG(fentry_test_arena, unsigned long long *st_ops_ctx)
    {
    return 0;
    }
    SEC("fexit")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_test_arena, st_ops_ctx: *mut c_ulonglong, ret: c_int) -> c_int {
    int BPF_PROG(fexit_test_arena, unsigned long long *st_ops_ctx, int ret)
    {
    return 0;
    }
    SEC("freplace")
#[no_mangle]
pub unsafe extern "C" fn freplace_test_arena(st_ops_ctx: *mut c_ulonglong) -> c_int {
    int freplace_test_arena(unsigned long long *st_ops_ctx)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
