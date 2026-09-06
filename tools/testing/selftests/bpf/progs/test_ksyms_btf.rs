//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ksyms_btf.c
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
// Copyright (c) 2020 Google

    let mut out__runqueues_addr: __u64 = -1;
    let mut out__bpf_prog_active_addr: __u64 = -1;
    __u32 out__rq_cpu = -1; /* percpu struct fields */
    int out__bpf_prog_active = -1; /* percpu int */
    let mut out__this_rq_cpu: __u32 = -1;
    let mut out__this_bpf_prog_active: c_int = -1;
    __u32 out__cpu_0_rq_cpu = -1; /* cpu_rq(0).cpu */
    extern const struct rq runqueues __ksym; /* struct type global var. */
    extern const int bpf_prog_active __ksym; /* int type global var. */
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handler(ctx: *const c_void) -> c_int {
    int handler(const void *ctx)
    {
    struct rq *rq;
    int *active;
    __u32 cpu;
    out__runqueues_addr = (__u64)&runqueues;
    out__bpf_prog_active_addr = (__u64)&bpf_prog_active;
    cpu = bpf_get_smp_processor_id();
// test bpf_per_cpu_ptr()
    rq = (struct rq *)bpf_per_cpu_ptr(&runqueues, cpu);
    if (rq)
    out__rq_cpu = rq.cpu;
    active = (int *)bpf_per_cpu_ptr(&bpf_prog_active, cpu);
    if (active)
    out__bpf_prog_active = *active;
    rq = (struct rq *)bpf_per_cpu_ptr(&runqueues, 0);
    if (rq) /* should always be valid, but we can't spare the check. */
    out__cpu_0_rq_cpu = rq.cpu;
// test bpf_this_cpu_ptr
    rq = (struct rq *)bpf_this_cpu_ptr(&runqueues);
    out__this_rq_cpu = rq.cpu;
    active = (int *)bpf_this_cpu_ptr(&bpf_prog_active);
    out__this_bpf_prog_active = *active;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
