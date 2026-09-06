//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_loop_bench.c
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
// Copyright (c) 2021 Facebook

    char _license[] SEC("license") = "GPL";
    u32 nr_loops;
    long hits;
#[no_mangle]
unsafe extern "C" fn empty_callback(index: __u32, data: *mut c_void) -> c_int {
    static int empty_callback(__u32 index, void *data)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn outer_loop(index: __u32, data: *mut c_void) -> c_int {
    static int outer_loop(__u32 index, void *data)
    {
    bpf_loop(nr_loops, empty_callback, core::ptr::null_mut(), 0);
    __sync_add_and_fetch(&hits, nr_loops);
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn benchmark(ctx: *mut c_void) -> c_int {
    int benchmark(void *ctx)
    {
    bpf_loop(1000, outer_loop, core::ptr::null_mut(), 0);
    return 0;
    }
