//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_usdt.c
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

    int my_pid;
    int usdt0_called;
    u64 usdt0_cookie;
    int usdt0_arg_cnt;
    int usdt0_arg_ret;
    int usdt0_arg_size;
    SEC("usdt")
#[no_mangle]
pub unsafe extern "C" fn usdt0(ctx: *mut pt_regs) -> c_int {
    int usdt0(struct pt_regs *ctx)
    {
    long tmp;
    if (my_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    __sync_fetch_and_add(&usdt0_called, 1);
    usdt0_cookie = bpf_usdt_cookie(ctx);
    usdt0_arg_cnt = bpf_usdt_arg_cnt(ctx);
// should return -ENOENT for any arg_num
    usdt0_arg_ret = bpf_usdt_arg(ctx, bpf_get_prandom_u32(), &tmp);
    usdt0_arg_size = bpf_usdt_arg_size(ctx, bpf_get_prandom_u32());
    return 0;
    }
    int usdt3_called;
    u64 usdt3_cookie;
    int usdt3_arg_cnt;
    int usdt3_arg_rets[3];
    u64 usdt3_args[3];
    int usdt3_arg_sizes[3];
    SEC("usdt//proc/self/exe:test:usdt3")
#[no_mangle]
pub unsafe extern "C" fn usdt3(ctx: *mut pt_regs) -> c_int {
    int usdt3(struct pt_regs *ctx)
    {
    long tmp;
    if (my_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    __sync_fetch_and_add(&usdt3_called, 1);
    usdt3_cookie = bpf_usdt_cookie(ctx);
    usdt3_arg_cnt = bpf_usdt_arg_cnt(ctx);
    usdt3_arg_rets[0] = bpf_usdt_arg(ctx, 0, &tmp);
    usdt3_args[0] = (int)tmp;
    usdt3_arg_sizes[0] = bpf_usdt_arg_size(ctx, 0);
    usdt3_arg_rets[1] = bpf_usdt_arg(ctx, 1, &tmp);
    usdt3_args[1] = (long)tmp;
    usdt3_arg_sizes[1] = bpf_usdt_arg_size(ctx, 1);
    usdt3_arg_rets[2] = bpf_usdt_arg(ctx, 2, &tmp);
    usdt3_args[2] = (uintptr_t)tmp;
    usdt3_arg_sizes[2] = bpf_usdt_arg_size(ctx, 2);
    return 0;
    }
    int usdt12_called;
    u64 usdt12_cookie;
    int usdt12_arg_cnt;
    u64 usdt12_args[12];
    int usdt12_arg_sizes[12];
    SEC("usdt//proc/self/exe:test:usdt12")
    int BPF_USDT(usdt12, int a1, int a2, long a3, long a4, unsigned a5,
    long a6, __u64 a7, uintptr_t a8, int a9, short a10,
    short a11, signed char a12)
    {
    int i;
    if (my_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    __sync_fetch_and_add(&usdt12_called, 1);
    usdt12_cookie = bpf_usdt_cookie(ctx);
    usdt12_arg_cnt = bpf_usdt_arg_cnt(ctx);
    usdt12_args[0] = a1;
    usdt12_args[1] = a2;
    usdt12_args[2] = a3;
    usdt12_args[3] = a4;
    usdt12_args[4] = a5;
    usdt12_args[5] = a6;
    usdt12_args[6] = a7;
    usdt12_args[7] = a8;
    usdt12_args[8] = a9;
    usdt12_args[9] = a10;
    usdt12_args[10] = a11;
    usdt12_args[11] = a12;
    bpf_for(i, 0, 12) {
    usdt12_arg_sizes[i] = bpf_usdt_arg_size(ctx, i);
    }
    return 0;
    }
    int usdt_sib_called;
    u64 usdt_sib_cookie;
    int usdt_sib_arg_cnt;
    int usdt_sib_arg_ret;
    short usdt_sib_arg;
    int usdt_sib_arg_size;
//
// usdt_sib is only tested on x86-related architectures, so it requires
// manual attach since auto-attach will panic tests under other architectures
//
    SEC("usdt")
#[no_mangle]
pub unsafe extern "C" fn usdt_sib(ctx: *mut pt_regs) -> c_int {
    int usdt_sib(struct pt_regs *ctx)
    {
    long tmp;
    if (my_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    __sync_fetch_and_add(&usdt_sib_called, 1);
    usdt_sib_cookie = bpf_usdt_cookie(ctx);
    usdt_sib_arg_cnt = bpf_usdt_arg_cnt(ctx);
    usdt_sib_arg_ret = bpf_usdt_arg(ctx, 0, &tmp);
    usdt_sib_arg = (short)tmp;
    usdt_sib_arg_size = bpf_usdt_arg_size(ctx, 0);
    return 0;
    }

    int executed;
    unsigned long expected_ip;
    SEC("usdt")
#[no_mangle]
pub unsafe extern "C" fn usdt_executed(ctx: *mut pt_regs) -> c_int {
    int usdt_executed(struct pt_regs *ctx)
    {
    if (expected_ip == ctx.ip)
    executed++;
    return 0;
    }
    int arg_total;
    int arg_bad;
    long arg_last[3];
    long expected_arg[3];
    int expected_pid;
    SEC("usdt")
#[no_mangle]
pub unsafe extern "C" fn BPF_USDT(_arg: usdt_check_arg, arg1: c_long, arg2: c_long, arg3: c_long) -> c_int {
    int BPF_USDT(usdt_check_arg, long arg1, long arg2, long arg3)
    {
    if (expected_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    __sync_fetch_and_add(&arg_total, 1);
    arg_last[0] = arg1;
    arg_last[1] = arg2;
    arg_last[2] = arg3;
    if (arg1 != expected_arg[0] ||
    arg2 != expected_arg[1] ||
    arg3 != expected_arg[2])
    __sync_fetch_and_add(&arg_bad, 1);
    return 0;
    }

    char _license[] SEC("license") = "GPL";
