//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/htab_reuse.c
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
// Copyright (C) 2023. Huawei Technologies Co., Ltd

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htab_val {
    pub lock: bpf_spin_lock,
    pub data: c_uint,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 64);
    __type(key, unsigned int);
    __type(value, struct htab_val);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    } htab SEC(".maps");
pub const HTAB_NDATA: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htab_val_large {
    pub lock: bpf_spin_lock,
    pub seq: __u32,
    pub data: [__u64; HTAB_NDATA],
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 8);
    __type(key, unsigned int);
    __type(value, struct htab_val_large);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    } htab_lock_consistency SEC(".maps");
