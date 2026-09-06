//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/freplace_attach_probe.c
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
// Copyright (c) 2020 Facebook

pub const VAR_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmap_elem {
    pub lock: bpf_spin_lock,
    pub var: [c_int; VAR_NUM],
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct hmap_elem);
    } hash_map SEC(".maps");
    SEC("freplace/handle_kprobe")
#[no_mangle]
pub unsafe extern "C" fn new_handle_kprobe(ctx: *mut pt_regs) -> c_int {
    int new_handle_kprobe(struct pt_regs *ctx)
    {
    struct hmap_elem *val;
    let mut key: c_int = 0;
    val = bpf_map_lookup_elem(&hash_map, &key);
    if (!val)
    return 1;
// spin_lock in hash map
    bpf_spin_lock(&val.lock);
    val.var[0] = 99;
    bpf_spin_unlock(&val.lock);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
