//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_vma_offset.c
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

    char _license[] SEC("license") = "GPL";
    let mut unique_tgid_cnt: __u32 = 0;
    let mut address: uintptr_t = 0;
    let mut offset: uintptr_t = 0;
    let mut last_tgid: __u32 = 0;
    let mut pid: __u32 = 0;
    let mut page_shift: __u32 = 0;
    SEC("iter/task_vma")
#[no_mangle]
pub unsafe extern "C" fn get_vma_offset(ctx: *mut bpf_iter__task_vma) -> c_int {
    int get_vma_offset(struct bpf_iter__task_vma *ctx)
    {
    struct vm_area_struct *vma = ctx.vma;
    struct seq_file *seq = ctx.meta.seq;
    struct task_struct *task = ctx.task;
    if (task == core::ptr::null_mut() || vma == core::ptr::null_mut())
    return 0;
    if (last_tgid != task.tgid)
    unique_tgid_cnt++;
    last_tgid = task.tgid;
    if (task.tgid != pid)
    return 0;
    if (vma.vm_start <= address && vma.vm_end > address) {
    offset = address - vma.vm_start + (vma.vm_pgoff << page_shift);
    BPF_SEQ_PRINTF(seq, "OK\n");
    }
    return 0;
    }
