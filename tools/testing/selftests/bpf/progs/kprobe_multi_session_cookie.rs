//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/kprobe_multi_session_cookie.c
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
    let mut pid: c_int = 0;
    let mut test_kprobe_1_result: __u64 = 0;
    let mut test_kprobe_2_result: __u64 = 0;
    let mut test_kprobe_3_result: __u64 = 0;
//
// No tests in here, just to trigger 'bpf_fentry_test*'
// through tracing test_run
//
    SEC("fentry/bpf_modify_return_test")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trigger) -> c_int {
    int BPF_PROG(trigger)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_cookie(ctx: *mut pt_regs, val: __u64, result: *mut __u64) -> c_int {
    static int check_cookie(struct pt_regs *ctx, __u64 val, __u64 *result)
    {
    __u64 *cookie;
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 1;
    cookie = bpf_session_cookie(ctx);
    if (bpf_session_is_return(ctx))
// result = *cookie == val ? val : 0;
    else
// cookie = val;
    return 0;
    }
    SEC("kprobe.session/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn test_kprobe_1(ctx: *mut pt_regs) -> c_int {
    int test_kprobe_1(struct pt_regs *ctx)
    {
    return check_cookie(ctx, 1, &test_kprobe_1_result);
    }
    SEC("kprobe.session/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn test_kprobe_2(ctx: *mut pt_regs) -> c_int {
    int test_kprobe_2(struct pt_regs *ctx)
    {
    return check_cookie(ctx, 2, &test_kprobe_2_result);
    }
    SEC("kprobe.session/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn test_kprobe_3(ctx: *mut pt_regs) -> c_int {
    int test_kprobe_3(struct pt_regs *ctx)
    {
    return check_cookie(ctx, 3, &test_kprobe_3_result);
    }
