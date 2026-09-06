//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ctx.c
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
// Copyright (c) 2026 Valve Corporation.
// Author: Changwoo Min <changwoo@igalia.com>
//

    char _license[] SEC("license") = "GPL";
    extern void bpf_kfunc_trigger_ctx_check(void) __ksym;
    int count_hardirq;
    int count_softirq;
    int count_task;
// Triggered via bpf_prog_test_run from user-space
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn trigger_all_contexts(ctx: *mut c_void) -> c_int {
    int trigger_all_contexts(void *ctx)
    {
    if (bpf_in_task())
    __sync_fetch_and_add(&count_task, 1);
// Trigger the firing of a hardirq and softirq for test.
    bpf_kfunc_trigger_ctx_check();
    return 0;
    }
// Observer for HardIRQ
    SEC("fentry/bpf_testmod_test_hardirq_fn")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_hardirq) -> c_int {
    int BPF_PROG(on_hardirq)
    {
    if (bpf_in_hardirq())
    __sync_fetch_and_add(&count_hardirq, 1);
    return 0;
    }
// Observer for SoftIRQ
    SEC("fentry/bpf_testmod_test_softirq_fn")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_softirq) -> c_int {
    int BPF_PROG(on_softirq)
    {
    if (bpf_in_serving_softirq())
    __sync_fetch_and_add(&count_softirq, 1);
    return 0;
    }
