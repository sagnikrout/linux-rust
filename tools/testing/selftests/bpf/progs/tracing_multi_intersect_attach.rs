//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tracing_multi_intersect_attach.c
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

    char _license[] SEC("license") = "GPL";
    __hidden extern int tracing_multi_arg_check(__u64 *ctx, __u64 *test_result, bool is_return);
    let mut test_result_fentry_1: __u64 = 0;
    let mut test_result_fentry_2: __u64 = 0;
    let mut test_result_fexit_1: __u64 = 0;
    let mut test_result_fexit_2: __u64 = 0;
    let mut test_result_fentry: __u64 = 0;
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry) -> c_int {
    int BPF_PROG(fentry)
    {
    tracing_multi_arg_check(ctx, &test_result_fentry, false);
    return 0;
    }
    SEC("fentry.multi")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_1) -> c_int {
    int BPF_PROG(fentry_1)
    {
    tracing_multi_arg_check(ctx, &test_result_fentry_1, false);
    return 0;
    }
    SEC("fentry.multi")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_2) -> c_int {
    int BPF_PROG(fentry_2)
    {
    tracing_multi_arg_check(ctx, &test_result_fentry_2, false);
    return 0;
    }
    SEC("fexit.multi")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_1) -> c_int {
    int BPF_PROG(fexit_1)
    {
    tracing_multi_arg_check(ctx, &test_result_fexit_1, true);
    return 0;
    }
    SEC("fexit.multi")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_2) -> c_int {
    int BPF_PROG(fexit_2)
    {
    tracing_multi_arg_check(ctx, &test_result_fexit_2, true);
    return 0;
    }
