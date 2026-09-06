//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_build_id.c
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

    struct bpf_stack_build_id stack_sleepable[128];
    int res_sleepable;
    struct bpf_stack_build_id stack_nofault[128];
    int res_nofault;
    SEC("uprobe.multi/./uprobe_multi:uprobe")
#[no_mangle]
pub unsafe extern "C" fn uprobe_nofault(ctx: *mut pt_regs) -> c_int {
    int uprobe_nofault(struct pt_regs *ctx)
    {
    res_nofault = bpf_get_stack(ctx, stack_nofault, sizeof(stack_nofault),
    BPF_F_USER_STACK | BPF_F_USER_BUILD_ID);
    return 0;
    }
    SEC("uprobe.multi.s/./uprobe_multi:uprobe")
#[no_mangle]
pub unsafe extern "C" fn uprobe_sleepable(ctx: *mut pt_regs) -> c_int {
    int uprobe_sleepable(struct pt_regs *ctx)
    {
    res_sleepable = bpf_get_stack(ctx, stack_sleepable, sizeof(stack_sleepable),
    BPF_F_USER_STACK | BPF_F_USER_BUILD_ID);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
