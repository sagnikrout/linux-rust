//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/uprobe_multi_session_single.c
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
    __u64 uprobe_session_result[3] = {};
    let mut pid: c_int = 0;
#[no_mangle]
unsafe extern "C" fn uprobe_multi_check(ctx: *mut c_void, idx: c_int) -> c_int {
    static int uprobe_multi_check(void *ctx, int idx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 1;
    uprobe_session_result[idx]++;
// only consumer 1 executes return probe
    if (idx == 0 || idx == 2)
    return 1;
    return 0;
    }
    SEC("uprobe.session//proc/self/exe:uprobe_multi_func_1")
#[no_mangle]
pub unsafe extern "C" fn uprobe_0(ctx: *mut pt_regs) -> c_int {
    int uprobe_0(struct pt_regs *ctx)
    {
    return uprobe_multi_check(ctx, 0);
    }
    SEC("uprobe.session//proc/self/exe:uprobe_multi_func_1")
#[no_mangle]
pub unsafe extern "C" fn uprobe_1(ctx: *mut pt_regs) -> c_int {
    int uprobe_1(struct pt_regs *ctx)
    {
    return uprobe_multi_check(ctx, 1);
    }
    SEC("uprobe.session//proc/self/exe:uprobe_multi_func_1")
#[no_mangle]
pub unsafe extern "C" fn uprobe_2(ctx: *mut pt_regs) -> c_int {
    int uprobe_2(struct pt_regs *ctx)
    {
    return uprobe_multi_check(ctx, 2);
    }
