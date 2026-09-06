//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_usdt_multispec.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

// this file is linked together with test_usdt.c to validate that usdt.bpf.h
// can be included in multiple .bpf.c files forming single final BPF object
// file
//
    extern int my_pid;
    int usdt_100_called;
    int usdt_100_sum;
    SEC("usdt//proc/self/exe:test:usdt_100")
#[no_mangle]
pub unsafe extern "C" fn BPF_USDT(_arg: usdt_100, x: c_int) -> c_int {
    int BPF_USDT(usdt_100, int x)
    {
    if (my_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    __sync_fetch_and_add(&usdt_100_called, 1);
    __sync_fetch_and_add(&usdt_100_sum, x);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
