//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/livepatch_trampoline.c
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

    int fentry_hit;
    int fexit_hit;
    int my_pid;
    SEC("fentry/cmdline_proc_show")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_cmdline) -> c_int {
    int BPF_PROG(fentry_cmdline)
    {
    if (my_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    fentry_hit = 1;
    return 0;
    }
    SEC("fexit/cmdline_proc_show")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_cmdline) -> c_int {
    int BPF_PROG(fexit_cmdline)
    {
    if (my_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    fexit_hit = 1;
    return 0;
    }
