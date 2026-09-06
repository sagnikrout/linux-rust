//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_kptr_return_fail__invalid_scalar.c
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
    struct cgroup *bpf_cgroup_acquire(struct cgroup *p) __ksym;
    void bpf_task_release(struct task_struct *p) __ksym;
// This test struct_ops BPF programs returning referenced kptr. The verifier should
// reject programs returning a non-zero scalar value.
//
    SEC("struct_ops/test_return_ref_kptr")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "At program exit the register R0 has smin=1 smax=1 should have been in, _arg: 0]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=1 smax=1 should have been in [0, 0]")
    struct task_struct *BPF_PROG(kptr_return_fail__invalid_scalar, int dummy,
    struct task_struct *task, struct cgroup *cgrp)
    {
    bpf_task_release(task);
    return (struct task_struct *)1;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_ops testmod_kptr_return = {
    .test_return_ref_kptr = (void *)kptr_return_fail__invalid_scalar,
    };
