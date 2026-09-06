//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/ksock_wq.c
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
// Copyright (c) 2026 Isovalent

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksock_wq_value {
    pub work: bpf_wq,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, u32);
    __type(value, struct ksock_wq_value);
    } work_map SEC(".maps");
    int create_err;
    u32 callback_done;
#[no_mangle]
unsafe extern "C" fn ksock_wq_callback(map: *mut c_void, key: *mut c_int, value: *mut c_void) -> c_int {
    static int ksock_wq_callback(void *map, int *key, void *value)
    {
    struct bpf_ksock_create_opts opts = {
    .family = AF_INET,
    .type = SOCK_DGRAM,
    .protocol = IPPROTO_UDP,
    };
    struct bpf_ksock *ks;
    let mut err: c_int = 0;
    ks = bpf_ksock_create(&opts, sizeof(opts), &err);
    if (ks)
    bpf_ksock_release(ks);
    create_err = err;
    __sync_fetch_and_add(&callback_done, 1);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn ksock_wq_start(ctx: *mut c_void) -> c_int {
    int ksock_wq_start(void *ctx)
    {
    struct ksock_wq_value *value;
    let mut key: u32 = 0;
    int err;
    value = bpf_map_lookup_elem(&work_map, &key);
    if (!value)
    return -ENOENT;
    err = bpf_wq_init(&value.work, &work_map, 0);
    if (err)
    return err;
    err = bpf_wq_set_callback(&value.work, ksock_wq_callback, 0);
    if (err)
    return err;
    return bpf_wq_start(&value.work, 0);
    }
    char __license[] SEC("license") = "GPL";
