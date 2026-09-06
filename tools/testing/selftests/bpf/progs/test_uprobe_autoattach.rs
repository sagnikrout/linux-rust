//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_uprobe_autoattach.c
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
// Copyright (c) 2022, Oracle and/or its affiliates.

    let mut uprobe_byname_parm1: c_int = 0;
    let mut uprobe_byname_ran: c_int = 0;
    let mut uretprobe_byname_rc: c_int = 0;
    let mut uretprobe_byname_ret: c_int = 0;
    let mut uretprobe_byname_ran: c_int = 0;
    let mut uprobe_byname2_parm1: u64 = 0;
    let mut uprobe_byname2_ran: c_int = 0;
    let mut uretprobe_byname2_rc: u64 = 0;
    let mut uretprobe_byname2_ran: c_int = 0;
    int test_pid;
    int a[8];
// This program cannot auto-attach, but that should not stop other
// programs from attaching.
//
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_uprobe_noautoattach(ctx: *mut pt_regs) -> c_int {
    int handle_uprobe_noautoattach(struct pt_regs *ctx)
    {
    return 0;
    }
    SEC("uprobe//proc/self/exe:autoattach_trigger_func")
    int BPF_UPROBE(handle_uprobe_byname
    , int arg1
    , int arg2
    , int arg3

    , int arg4

    , int arg5

    , int arg6

    , int arg7

    , int arg8

    )
    {
    uprobe_byname_parm1 = PT_REGS_PARM1_CORE(ctx);
    uprobe_byname_ran = 1;
    a[0] = arg1;
    a[1] = arg2;
    a[2] = arg3;

    a[3] = arg4;

    a[4] = arg5;

    a[5] = arg6;

    a[6] = arg7;

    a[7] = arg8;

    return 0;
    }
    SEC("uretprobe//proc/self/exe:autoattach_trigger_func")
#[no_mangle]
pub unsafe extern "C" fn BPF_URETPROBE(_arg: handle_uretprobe_byname, ret: c_int) -> c_int {
    int BPF_URETPROBE(handle_uretprobe_byname, int ret)
    {
    uretprobe_byname_rc = PT_REGS_RC_CORE(ctx);
    uretprobe_byname_ret = ret;
    uretprobe_byname_ran = 2;
    return 0;
    }
    SEC("uprobe/libc.so.6:fopen")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: handle_uprobe_byname2, pathname: *const c_char, mode: *const c_char) -> c_int {
    int BPF_UPROBE(handle_uprobe_byname2, const char *pathname, const char *mode)
    {
    let mut pid: c_int = bpf_get_current_pid_tgid() >> 32;
// ignore irrelevant invocations
    if (test_pid != pid)
    return 0;
    uprobe_byname2_parm1 = (u64)(long)pathname;
    uprobe_byname2_ran = 3;
    return 0;
    }
    SEC("uretprobe/libc.so.6:fopen")
#[no_mangle]
pub unsafe extern "C" fn BPF_URETPROBE(_arg: handle_uretprobe_byname2, ret: *mut c_void) -> c_int {
    int BPF_URETPROBE(handle_uretprobe_byname2, void *ret)
    {
    let mut pid: c_int = bpf_get_current_pid_tgid() >> 32;
// ignore irrelevant invocations
    if (test_pid != pid)
    return 0;
    uretprobe_byname2_rc = (u64)(long)ret;
    uretprobe_byname2_ran = 4;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
