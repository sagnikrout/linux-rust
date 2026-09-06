//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/uprobe_multi_session_cookie.c
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
    let mut test_uprobe_1_result: __u64 = 0;
    let mut test_uprobe_2_result: __u64 = 0;
    let mut test_uprobe_3_result: __u64 = 0;
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
    SEC("uprobe.session//proc/self/exe:uprobe_multi_func_1")
#[no_mangle]
pub unsafe extern "C" fn uprobe_1(ctx: *mut pt_regs) -> c_int {
    int uprobe_1(struct pt_regs *ctx)
    {
    return check_cookie(ctx, 1, &test_uprobe_1_result);
    }
    SEC("uprobe.session//proc/self/exe:uprobe_multi_func_2")
#[no_mangle]
pub unsafe extern "C" fn uprobe_2(ctx: *mut pt_regs) -> c_int {
    int uprobe_2(struct pt_regs *ctx)
    {
    return check_cookie(ctx, 2, &test_uprobe_2_result);
    }
    SEC("uprobe.session//proc/self/exe:uprobe_multi_func_3")
#[no_mangle]
pub unsafe extern "C" fn uprobe_3(ctx: *mut pt_regs) -> c_int {
    int uprobe_3(struct pt_regs *ctx)
    {
    return check_cookie(ctx, 3, &test_uprobe_3_result);
    }
