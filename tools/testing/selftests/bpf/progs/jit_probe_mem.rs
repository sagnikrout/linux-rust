//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/jit_probe_mem.c
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

    static struct prog_test_ref_kfunc __kptr *v;
    let mut total_sum: c_long = -1;
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_jit_probe_mem(ctx: *mut __sk_buff) -> c_int {
    int test_jit_probe_mem(struct __sk_buff *ctx)
    {
    struct prog_test_ref_kfunc *p;
    let mut zero: c_ulong = 0, sum;
    p = bpf_kfunc_call_test_acquire(&zero);
    if (!p)
    return 1;
    p = bpf_kptr_xchg(&v, p);
    if (p)
    goto release_out;
// Direct map value access of kptr, should be PTR_UNTRUSTED
    p = v;
    if (!p)
    return 1;
    asm volatile (
    "r9 = %[p];"
    "%[sum] = 0;"
// r8 = p->a
    "r8 = *(u32 *)(r9 + 0);"
    "%[sum] += r8;"
// r8 = p->b
    "r8 = *(u32 *)(r9 + 4);"
    "%[sum] += r8;"
    "r9 += 8;"
// r9 = p->a
    "r9 = *(u32 *)(r9 - 8);"
    "%[sum] += r9;"
    : [sum] "=r"(sum)
    : [p] "r"(p)
    : "r8", "r9"
    );
    total_sum = sum;
    return 0;
    release_out:
    bpf_kfunc_call_test_release(p);
    return 1;
    }
    char _license[] SEC("license") = "GPL";
