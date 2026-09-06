//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/uprobe_multi_pid_filter.c
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
    __u32 pids[3];
    __u32 test[3][2];
#[no_mangle]
unsafe extern "C" fn update_pid(idx: c_int) {
    static void update_pid(int idx)
    {
    let mut pid: __u32 = bpf_get_current_pid_tgid() >> 32;
    if (pid == pids[idx])
    test[idx][0]++;
    else
    test[idx][1]++;
    }
    SEC("uprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn uprobe_multi_0(ctx: *mut pt_regs) -> c_int {
    int uprobe_multi_0(struct pt_regs *ctx)
    {
    update_pid(0);
    return 0;
    }
    SEC("uprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn uprobe_multi_1(ctx: *mut pt_regs) -> c_int {
    int uprobe_multi_1(struct pt_regs *ctx)
    {
    update_pid(1);
    return 0;
    }
    SEC("uprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn uprobe_multi_2(ctx: *mut pt_regs) -> c_int {
    int uprobe_multi_2(struct pt_regs *ctx)
    {
    update_pid(2);
    return 0;
    }
