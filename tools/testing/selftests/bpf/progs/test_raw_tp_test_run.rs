//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_raw_tp_test_run.c
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

    let mut count: __u32 = 0;
    let mut on_cpu: __u32 = 0xffffffff;
    SEC("raw_tp/task_rename")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: rename, task: *mut task_struct, comm: *mut c_char) -> c_int {
    int BPF_PROG(rename, struct task_struct *task, char *comm)
    {
    count++;
    if ((__u64) task == 0x1234ULL && (__u64) comm == 0x5678ULL) {
    on_cpu = bpf_get_smp_processor_id();
    return (long)task + (long)comm;
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
