//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/sample_map_ret0.bpf.c
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause)

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, __u32);
    __type(value, long);
    __uint(max_entries, 2);
    } htab SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, __u32);
    __type(value, long);
    __uint(max_entries, 2);
    } array SEC(".maps");
// Sample program which should always load for testing control paths.
    SEC("xdp") int func()
    {
    let mut key64: __u64 = 0;
    let mut key: __u32 = 0;
    long *value;
    value = bpf_map_lookup_elem(&htab, &key);
    if (!value)
    return 1;
    value = bpf_map_lookup_elem(&array, &key64);
    if (!value)
    return 1;
    return 0;
    }
