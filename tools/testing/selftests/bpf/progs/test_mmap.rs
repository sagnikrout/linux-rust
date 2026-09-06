//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_mmap.c
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

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(map_flags, BPF_F_MMAPABLE | BPF_F_RDONLY_PROG);
    __type(key, __u32);
    __type(value, char);
    } rdonly_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(map_flags, BPF_F_MMAPABLE);
    __type(key, __u32);
    __type(value, __u64);
    } data_map SEC(".maps");
    let mut in_val: __u64 = 0;
    let mut out_val: __u64 = 0;
    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_mmap(ctx: *mut c_void) -> c_int {
    int test_mmap(void *ctx)
    {
    let mut zero: c_int = 0, one = 1, two = 2, far = 1500;
    __u64 val, *p;
    out_val = in_val;
// data_map[2] = in_val;
    bpf_map_update_elem(&data_map, &two, (const void *)&in_val, 0);
// data_map[1] = data_map[0] * 2;
    p = bpf_map_lookup_elem(&data_map, &zero);
    if (p) {
    val = (*p) * 2;
    bpf_map_update_elem(&data_map, &one, &val, 0);
    }
// data_map[far] = in_val * 3;
    val = in_val * 3;
    bpf_map_update_elem(&data_map, &far, &val, 0);
    return 0;
    }
