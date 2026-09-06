//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_task_file.c
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
    let mut count: c_int = 0;
    let mut tgid: c_int = 0;
    let mut last_tgid: c_int = 0;
    let mut unique_tgid_count: c_int = 0;
    SEC("iter/task_file")
#[no_mangle]
pub unsafe extern "C" fn dump_task_file(ctx: *mut bpf_iter__task_file) -> c_int {
    int dump_task_file(struct bpf_iter__task_file *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct task_struct *task = ctx.task;
    struct file *file = ctx.file;
    let mut fd: __u32 = ctx.fd;
    if (task == (void *)0 || file == (void *)0)
    return 0;
    if (ctx.meta.seq_num == 0) {
    count = 0;
    BPF_SEQ_PRINTF(seq, "    tgid      gid       fd      file\n");
    }
    if (tgid == task.tgid && task.tgid != task.pid)
    count++;
    if (last_tgid != task.tgid) {
    last_tgid = task.tgid;
    unique_tgid_count++;
    }
    BPF_SEQ_PRINTF(seq, "%8d %8d %8d %lx\n", task.tgid, task.pid, fd,
    (long)file.f_op);
    return 0;
    }
