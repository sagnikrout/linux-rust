//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/uretprobe_stack.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    __u64 entry_stack1[32], exit_stack1[32];
    __u64 entry_stack1_recur[32], exit_stack1_recur[32];
    __u64 entry_stack2[32];
    __u64 entry_stack3[32];
    __u64 entry_stack4[32], exit_stack4[32];
    __u64 usdt_stack[32];
    int entry1_len, exit1_len;
    int entry1_recur_len, exit1_recur_len;
    int entry2_len, exit2_len;
    int entry3_len, exit3_len;
    int entry4_len, exit4_len;
    int usdt_len;

    SEC("uprobe//proc/self/exe:target_1")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: uprobe_1) -> c_int {
    int BPF_UPROBE(uprobe_1)
    {
// target_1 is recursive with depth of 2, so we capture two separate
// stack traces, depending on which occurrence it is
//
    let mut recur: static bool = false;
    if (!recur)
    entry1_len = bpf_get_stack(ctx, &entry_stack1, SZ, BPF_F_USER_STACK);
    else
    entry1_recur_len = bpf_get_stack(ctx, &entry_stack1_recur, SZ, BPF_F_USER_STACK);
    recur = true;
    return 0;
    }
    SEC("uretprobe//proc/self/exe:target_1")
#[no_mangle]
pub unsafe extern "C" fn BPF_URETPROBE(_arg: uretprobe_1) -> c_int {
    int BPF_URETPROBE(uretprobe_1)
    {
// see above, target_1 is recursive
    let mut recur: static bool = false;
// NOTE: order of returns is reversed to order of entries
    if (!recur)
    exit1_recur_len = bpf_get_stack(ctx, &exit_stack1_recur, SZ, BPF_F_USER_STACK);
    else
    exit1_len = bpf_get_stack(ctx, &exit_stack1, SZ, BPF_F_USER_STACK);
    recur = true;
    return 0;
    }
    SEC("uprobe//proc/self/exe:target_2")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: uprobe_2) -> c_int {
    int BPF_UPROBE(uprobe_2)
    {
    entry2_len = bpf_get_stack(ctx, &entry_stack2, SZ, BPF_F_USER_STACK);
    return 0;
    }
// no uretprobe for target_2
    SEC("uprobe//proc/self/exe:target_3")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: uprobe_3) -> c_int {
    int BPF_UPROBE(uprobe_3)
    {
    entry3_len = bpf_get_stack(ctx, &entry_stack3, SZ, BPF_F_USER_STACK);
    return 0;
    }
// no uretprobe for target_3
    SEC("uprobe//proc/self/exe:target_4")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: uprobe_4) -> c_int {
    int BPF_UPROBE(uprobe_4)
    {
    entry4_len = bpf_get_stack(ctx, &entry_stack4, SZ, BPF_F_USER_STACK);
    return 0;
    }
    SEC("uretprobe//proc/self/exe:target_4")
#[no_mangle]
pub unsafe extern "C" fn BPF_URETPROBE(_arg: uretprobe_4) -> c_int {
    int BPF_URETPROBE(uretprobe_4)
    {
    exit4_len = bpf_get_stack(ctx, &exit_stack4, SZ, BPF_F_USER_STACK);
    return 0;
    }
    SEC("usdt//proc/self/exe:uretprobe_stack:target")
#[no_mangle]
pub unsafe extern "C" fn BPF_USDT(_arg: usdt_probe) -> c_int {
    int BPF_USDT(usdt_probe)
    {
    usdt_len = bpf_get_stack(ctx, &usdt_stack, SZ, BPF_F_USER_STACK);
    return 0;
    }
