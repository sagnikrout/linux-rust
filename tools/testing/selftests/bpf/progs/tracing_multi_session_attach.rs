//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tracing_multi_session_attach.c
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
    let mut test_result_fentry: __u64 = 0;
    let mut test_result_fexit: __u64 = 0;
    SEC("fsession.multi/bpf_fentry_test*")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_session_1) -> c_int {
    int BPF_PROG(test_session_1)
    {
    volatile __u64 *cookie = bpf_session_cookie(ctx);
    if (bpf_session_is_return(ctx)) {
    if (tracing_multi_arg_check(ctx, &test_result_fexit, true))
    return 0;
// extra count for test_result_fexit cookie
    test_result_fexit += *cookie == 0xbeafbeafbeafbeaf;
    } else {
    if (tracing_multi_arg_check(ctx, &test_result_fentry, false))
    return 0;
// cookie = 0xbeafbeafbeafbeaf;
    }
    return 0;
    }
    SEC("fsession.multi.s/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_fsession_s) -> c_int {
    int BPF_PROG(test_fsession_s)
    {
    volatile __u64 *cookie = bpf_session_cookie(ctx);
    if (bpf_session_is_return(ctx)) {
    if (tracing_multi_arg_check(ctx, &test_result_fexit, true))
    return 0;
// extra count for test_result_fexit cookie
    test_result_fexit += *cookie == 0xbeafbeafbeafbeaf;
    } else {
    if (tracing_multi_arg_check(ctx, &test_result_fentry, false))
    return 0;
// cookie = 0xbeafbeafbeafbeaf;
    }
    return 0;
    }
    SEC("fsession.multi/bpf_testmod:bpf_testmod_fentry_test*")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_session_2) -> c_int {
    int BPF_PROG(test_session_2)
    {
    volatile __u64 *cookie = bpf_session_cookie(ctx);
    if (bpf_session_is_return(ctx)) {
    if (tracing_multi_arg_check(ctx, &test_result_fexit, true))
    return 0;
// extra count for test_result_fexit cookie
    test_result_fexit += *cookie == 0xbeafbeafbeafbeaf;
    } else {
    if (tracing_multi_arg_check(ctx, &test_result_fentry, false))
    return 0;
// cookie = 0xbeafbeafbeafbeaf;
    }
    return 0;
    }
