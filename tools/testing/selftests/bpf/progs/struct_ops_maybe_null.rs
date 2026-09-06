//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_maybe_null.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    let mut tgid: pid_t = 0;
// This is a test BPF program that uses struct_ops to access an argument
// that may be NULL. This is a test for the verifier to ensure that it can
// rip PTR_MAYBE_NULL correctly.
//
    SEC("struct_ops/test_maybe_null")
    int BPF_PROG(test_maybe_null, int dummy,
    struct task_struct *task)
    {
    if (task)
    tgid = task.tgid;
    return 0;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_ops testmod_1 = {
    .test_maybe_null = (void *)test_maybe_null,
    };
