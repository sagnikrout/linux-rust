//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_overhead.c
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
// Copyright (c) 2019 Facebook

    struct task_struct;
    SEC("kprobe/__set_task_comm")
#[no_mangle]
pub unsafe extern "C" fn BPF_KPROBE(_arg: prog1, tsk: *mut task_struct, buf: *const c_char, exec: bool) -> c_int {
    int BPF_KPROBE(prog1, struct task_struct *tsk, const char *buf, bool exec)
    {
    return !tsk;
    }
    SEC("kretprobe/__set_task_comm")
#[no_mangle]
pub unsafe extern "C" fn BPF_KRETPROBE(_arg: prog2, ret: c_int) -> c_int {
    int BPF_KRETPROBE(prog2, int ret)
    {
    return ret;
    }
    SEC("raw_tp/task_rename")
#[no_mangle]
pub unsafe extern "C" fn prog3(ctx: *mut bpf_raw_tracepoint_args) -> c_int {
    int prog3(struct bpf_raw_tracepoint_args *ctx)
    {
    return !ctx.args[0];
    }
    SEC("fentry/__set_task_comm")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: prog4, tsk: *mut task_struct, buf: *const c_char, exec: bool) -> c_int {
    int BPF_PROG(prog4, struct task_struct *tsk, const char *buf, bool exec)
    {
    return 0;
    }
    SEC("fexit/__set_task_comm")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: prog5, tsk: *mut task_struct, buf: *const c_char, exec: bool) -> c_int {
    int BPF_PROG(prog5, struct task_struct *tsk, const char *buf, bool exec)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
