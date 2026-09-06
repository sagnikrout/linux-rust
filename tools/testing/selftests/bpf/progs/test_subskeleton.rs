//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_subskeleton.c
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
// Copyright (c) Meta Platforms, Inc. and affiliates.

// volatile to force a read, compiler may assume 0 otherwise
    const volatile int rovar1;
    int out1;
// Override weak symbol in test_subskeleton_lib
    let mut var5: c_int = 5;
    extern volatile bool CONFIG_BPF_SYSCALL __kconfig;
    extern int lib_routine(void);
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handler1(ctx: *const c_void) -> c_int {
    int handler1(const void *ctx)
    {
    (void) CONFIG_BPF_SYSCALL;
    out1 = lib_routine() * rovar1;
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
