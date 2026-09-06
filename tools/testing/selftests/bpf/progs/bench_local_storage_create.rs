//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bench_local_storage_create.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    let mut create_errs: c_long = 0;
    let mut create_cnts: c_long = 0;
    let mut bench_pid: __u32 = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct storage {
    pub data: [__u8; 64],
}

    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct storage);
    } sk_storage_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct storage);
    } task_storage_map SEC(".maps");
    SEC("tp_btf/sched_process_fork")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: sched_process_fork, parent: *mut task_struct, child: *mut task_struct) -> c_int {
    int BPF_PROG(sched_process_fork, struct task_struct *parent, struct task_struct *child)
    {
    struct storage *stg;
    if (parent.tgid != bench_pid)
    return 0;
    stg = bpf_task_storage_get(&task_storage_map, child, core::ptr::null_mut(),
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (stg)
    __sync_fetch_and_add(&create_cnts, 1);
    else
    __sync_fetch_and_add(&create_errs, 1);
    return 0;
    }
    SEC("lsm.s/socket_post_create")
    int BPF_PROG(socket_post_create, struct socket *sock, int family, int type,
    int protocol, int kern)
    {
    struct sock *sk = sock.sk;
    struct storage *stg;
    __u32 pid;
    pid = bpf_get_current_pid_tgid() >> 32;
    if (pid != bench_pid || !sk)
    return 0;
    stg = bpf_sk_storage_get(&sk_storage_map, sk, core::ptr::null_mut(),
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (stg)
    __sync_fetch_and_add(&create_cnts, 1);
    else
    __sync_fetch_and_add(&create_errs, 1);
    return 0;
    }
    char __license[] SEC("license") = "GPL";
