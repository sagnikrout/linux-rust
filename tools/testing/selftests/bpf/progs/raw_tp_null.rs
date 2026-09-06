//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/raw_tp_null.c
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
    int tid;
    int i;
    SEC("tp_btf/bpf_testmod_test_raw_tp_null_tp")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_raw_tp_null, skb: *mut sk_buff) -> c_int {
    int BPF_PROG(test_raw_tp_null, struct sk_buff *skb)
    {
    struct task_struct *task = bpf_get_current_task_btf();
    if (task.pid != tid)
    return 0;
// If dead code elimination kicks in, the increment +=2 will be
// removed. For raw_tp programs attaching to tracepoints in kernel
// modules, we mark input arguments as PTR_MAYBE_NULL, so branch
// prediction should never kick in.
//
    asm volatile ("%[i] += 1; if %[ctx] != 0 goto +1; %[i] += 2;"
    : [i]"+r"(i)
    : [ctx]"r"(skb)
    : "memory");
    return 0;
    }
