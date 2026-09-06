//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/fexit_many_args.c
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
// Copyright (c) 2023 Tencent

    char _license[] SEC("license") = "GPL";
    let mut test1_result: __u64 = 0;
    SEC("fexit/bpf_testmod_fentry_test7")
    int BPF_PROG(test1, __u64 a, void *b, short c, int d, void *e, char f,
    int g, int ret)
    {
    test1_result = a == 16 && b == (void *)17 && c == 18 && d == 19 &&
    e == (void *)20 && f == 21 && g == 22 && ret == 133;
    return 0;
    }
    let mut test2_result: __u64 = 0;
    SEC("fexit/bpf_testmod_fentry_test11")
    int BPF_PROG(test2, __u64 a, void *b, short c, int d, void *e, char f,
    int g, unsigned int h, long i, __u64 j, unsigned long k,
    int ret)
    {
    test2_result = a == 16 && b == (void *)17 && c == 18 && d == 19 &&
    e == (void *)20 && f == 21 && g == 22 && h == 23 &&
    i == 24 && j == 25 && k == 26 && ret == 231;
    return 0;
    }
    let mut test3_result: __u64 = 0;
    SEC("fexit/bpf_testmod_fentry_test11")
    int BPF_PROG(test3, __u64 a, __u64 b, __u64 c, __u64 d, __u64 e, __u64 f,
    __u64 g, __u64 h, __u64 i, __u64 j, __u64 k, __u64 ret)
    {
    test3_result = a == 16 && b == 17 && c == 18 && d == 19 &&
    e == 20 && f == 21 && g == 22 && h == 23 &&
    i == 24 && j == 25 && k == 26 && ret == 231;
    return 0;
    }
