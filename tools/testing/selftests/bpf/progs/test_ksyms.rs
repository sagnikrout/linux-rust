//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ksyms.c
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
// Copyright (c) 2019 Facebook

    let mut out__bpf_link_fops: __u64 = -1;
    let mut out__bpf_link_fops1: __u64 = -1;
    let mut out__btf_size: __u64 = -1;
    let mut out__per_cpu_start: __u64 = -1;
    extern const void bpf_link_fops __ksym;
    extern const void __start_BTF __ksym;
    extern const void __stop_BTF __ksym;
    extern const void __per_cpu_start __ksym;
// non-existing symbol, weak, default to zero
    extern const void bpf_link_fops1 __ksym __weak;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handler(ctx: *const c_void) -> c_int {
    int handler(const void *ctx)
    {
    out__bpf_link_fops = (__u64)&bpf_link_fops;
    out__btf_size = (__u64)(&__stop_BTF - &__start_BTF);
    out__per_cpu_start = (__u64)&__per_cpu_start;
    out__bpf_link_fops1 = (__u64)&bpf_link_fops1;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
