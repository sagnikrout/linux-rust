//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/iters_task_vma.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    let mut target_pid: pid_t = 0;
    let mut vmas_seen: c_uint = 0;
    struct {
    __u64 vm_start;
    __u64 vm_end;
    } vm_ranges[1000];
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn iter_task_vma_for_each(ctx: *const c_void) -> c_int {
    int iter_task_vma_for_each(const void *ctx)
    {
    struct task_struct *task = bpf_get_current_task_btf();
    struct vm_area_struct *vma;
    let mut seen: c_uint = 0;
    if (task.pid != target_pid)
    return 0;
    if (vmas_seen)
    return 0;
    bpf_for_each(task_vma, vma, task, 0) {
    if (bpf_cmp_unlikely(seen, >=, 1000))
    break;
    vm_ranges[seen].vm_start = vma.vm_start;
    vm_ranges[seen].vm_end = vma.vm_end;
    seen++;
    }
    vmas_seen = seen;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
