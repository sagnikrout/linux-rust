//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_btf_nokv.c
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
// Copyright (c) 2018 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv_counts {
    pub v4: c_uint,
    pub v6: c_uint,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(key_size, sizeof(int));
    __uint(value_size, sizeof(struct ipv_counts));
    __uint(max_entries, 4);
    } btf_map SEC(".maps");
    __attribute__((noinline))
#[no_mangle]
pub unsafe extern "C" fn test_long_fname_2() -> c_int {
    int test_long_fname_2(void)
    {
    struct ipv_counts *counts;
    let mut key: c_int = 0;
    counts = bpf_map_lookup_elem(&btf_map, &key);
    if (!counts)
    return 0;
    counts.v6++;
    return 0;
    }
    __attribute__((noinline))
#[no_mangle]
pub unsafe extern "C" fn test_long_fname_1() -> c_int {
    int test_long_fname_1(void)
    {
    return test_long_fname_2();
    }
    SEC("dummy_tracepoint")
#[no_mangle]
pub unsafe extern "C" fn _dummy_tracepoint(arg: *mut c_void) -> c_int {
    int _dummy_tracepoint(void *arg)
    {
    return test_long_fname_1();
    }
    char _license[] SEC("license") = "GPL";
