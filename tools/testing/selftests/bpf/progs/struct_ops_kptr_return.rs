//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_kptr_return.c
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


    char _license[] SEC("license") = "GPL";
    void bpf_task_release(struct task_struct *p) __ksym;
// This test struct_ops BPF programs returning referenced kptr. The verifier should
// allow a referenced kptr or a NULL pointer to be returned. A referenced kptr to task
// here is acquired automatically as the task argument is tagged with "__ref".
//
    SEC("struct_ops/test_return_ref_kptr")
    struct task_struct *BPF_PROG(kptr_return, int dummy,
    struct task_struct *task, struct cgroup *cgrp)
    {
    if (dummy % 2) {
    bpf_task_release(task);
    return core::ptr::null_mut();
    }
    return task;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_ops testmod_kptr_return = {
    .test_return_ref_kptr = (void *)kptr_return,
    };
