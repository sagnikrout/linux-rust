//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ksyms_btf_write_check.c
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
// Copyright (c) 2021 Google

    extern const int bpf_prog_active __ksym; /* int type global var. */
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handler1(ctx: *const c_void) -> c_int {
    int handler1(const void *ctx)
    {
    int *active;
    __u32 cpu;
    cpu = bpf_get_smp_processor_id();
    active = (int *)bpf_per_cpu_ptr(&bpf_prog_active, cpu);
    if (active) {
// Kernel memory obtained from bpf_{per,this}_cpu_ptr
// is read-only, should _not_ pass verification.
//
// WRITE_ONCE
// (volatile int *)active = -1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn write_active(p: *mut c_int) -> __noinline int {
    __noinline int write_active(int *p)
    {
    return p ? (*p = 42) : 0;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handler2(ctx: *const c_void) -> c_int {
    int handler2(const void *ctx)
    {
    int *active;
    active = bpf_this_cpu_ptr(&bpf_prog_active);
    write_active(active);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
