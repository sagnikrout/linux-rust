//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_task_stack.c
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
// Copyright (c) 2020 Facebook

    char _license[] SEC("license") = "GPL";
pub const MAX_STACK_TRACE_DEPTH: c_int = 64;
    unsigned long entries[MAX_STACK_TRACE_DEPTH] = {};

    SEC("iter/task")
#[no_mangle]
pub unsafe extern "C" fn dump_task_stack(ctx: *mut bpf_iter__task) -> c_int {
    int dump_task_stack(struct bpf_iter__task *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct task_struct *task = ctx.task;
    long i, retlen;
    if (task == (void *)0)
    return 0;
    retlen = bpf_get_task_stack(task, entries,
    MAX_STACK_TRACE_DEPTH * SIZE_OF_ULONG, 0);
    if (retlen < 0)
    return 0;
    BPF_SEQ_PRINTF(seq, "pid: %8u num_entries: %8u\n", task.pid,
    retlen / SIZE_OF_ULONG);
    for (i = 0; i < MAX_STACK_TRACE_DEPTH; i++) {
    if (retlen > i * SIZE_OF_ULONG)
    BPF_SEQ_PRINTF(seq, "[<0>] %pB\n", (void *)entries[i]);
    }
    BPF_SEQ_PRINTF(seq, "\n");
    return 0;
    }
    let mut num_user_stacks: c_int = 0;
    SEC("iter/task")
#[no_mangle]
pub unsafe extern "C" fn get_task_user_stacks(ctx: *mut bpf_iter__task) -> c_int {
    int get_task_user_stacks(struct bpf_iter__task *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct task_struct *task = ctx.task;
    let mut buf_sz: u64 = 0;
    int64_t res;
    if (task == (void *)0)
    return 0;
    res = bpf_get_task_stack(task, entries,
    MAX_STACK_TRACE_DEPTH * SIZE_OF_ULONG, BPF_F_USER_STACK);
    if (res <= 0)
    return 0;
// Only one task, the current one, should succeed
    ++num_user_stacks;
    buf_sz += res;
// If the verifier doesn't refine bpf_get_task_stack res, and instead
// assumes res is entirely unknown, this program will fail to load as
// the verifier will believe that max buf_sz value allows reading
// past the end of entries in bpf_seq_write call
//
    bpf_seq_write(seq, &entries, buf_sz);
    return 0;
    }
