//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ksyms_weak.c
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
//
// Test weak ksyms.
//
// Copyright (c) 2021 Google
//

    let mut out__existing_typed: c_int = -1;
    let mut out__existing_typeless: __u64 = -1;
    let mut out__non_existent_typeless: __u64 = -1;
    let mut out__non_existent_typed: __u64 = -1;
// existing weak symbols
// test existing weak symbols can be resolved.
    extern const struct rq runqueues __ksym __weak; /* typed */
    extern const void bpf_prog_active __ksym __weak; /* typeless */
    struct task_struct *bpf_task_acquire(struct task_struct *p) __ksym __weak;
    void bpf_testmod_test_mod_kfunc(int i) __ksym __weak;
// non-existent weak symbols.
// typeless symbols, default to zero.
    extern const void bpf_link_fops1 __ksym __weak;
// typed symbols, default to zero.
    extern const int bpf_link_fops2 __ksym __weak;
    void invalid_kfunc(void) __ksym __weak;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn pass_handler(ctx: *const c_void) -> c_int {
    int pass_handler(const void *ctx)
    {
    struct rq *rq;
// tests existing symbols.
    rq = (struct rq *)bpf_per_cpu_ptr(&runqueues, 0);
    if (rq && bpf_ksym_exists(&runqueues))
    out__existing_typed = rq.cpu;
    out__existing_typeless = (__u64)&bpf_prog_active;
// tests non-existent symbols.
    out__non_existent_typeless = (__u64)&bpf_link_fops1;
// tests non-existent symbols.
    out__non_existent_typed = (__u64)&bpf_link_fops2;
    if (&bpf_link_fops2) /* can't happen */
    out__non_existent_typed = (__u64)bpf_per_cpu_ptr(&bpf_link_fops2, 0);
    if (!bpf_ksym_exists(bpf_task_acquire))
// dead code won't be seen by the verifier
    bpf_task_acquire(0);
    if (!bpf_ksym_exists(bpf_testmod_test_mod_kfunc))
// dead code won't be seen by the verifier
    bpf_testmod_test_mod_kfunc(0);
    if (bpf_ksym_exists(invalid_kfunc))
// dead code won't be seen by the verifier
    invalid_kfunc();
    return 0;
    }
    char _license[] SEC("license") = "GPL";
