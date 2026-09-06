//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_task_btf.c
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
// Copyright (c) 2020, Oracle and/or its affiliates.

    char _license[] SEC("license") = "GPL";
    let mut tasks: c_long = 0;
    let mut seq_err: c_long = 0;
    let mut skip: bool = false;
    SEC("iter/task")
#[no_mangle]
pub unsafe extern "C" fn dump_task_struct(ctx: *mut bpf_iter__task) -> c_int {
    int dump_task_struct(struct bpf_iter__task *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct task_struct *task = ctx.task;
    let mut ptr: static struct btf_ptr = { };
    long ret;

    ptr.type_id = bpf_core_type_id_kernel(struct task_struct);
    ptr.ptr = task;
    if (ctx.meta.seq_num == 0)
    BPF_SEQ_PRINTF(seq, "Raw BTF task\n");
    ret = bpf_seq_printf_btf(seq, &ptr, sizeof(ptr), 0);
    switch (ret) {
    case 0:
    tasks++;
    break;
    case -ERANGE:
// NULL task or task->fs, don't count it as an error.
    break;
    case -E2BIG:
    return 1;
    default:
    seq_err = ret;
    break;
    }

    skip = true;

    return 0;
    }
