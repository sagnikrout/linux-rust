//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_bpf_sk_storage_helpers.c
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
// Copyright (c) 2020 Google LLC.

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, int);
    } sk_stg_map SEC(".maps");
    SEC("iter/bpf_sk_storage_map")
#[no_mangle]
pub unsafe extern "C" fn delete_bpf_sk_storage_map(ctx: *mut bpf_iter__bpf_sk_storage_map) -> c_int {
    int delete_bpf_sk_storage_map(struct bpf_iter__bpf_sk_storage_map *ctx)
    {
    if (ctx.sk)
    bpf_sk_storage_delete(&sk_stg_map, ctx.sk);
    return 0;
    }
    SEC("iter/task_file")
#[no_mangle]
pub unsafe extern "C" fn fill_socket_owner(ctx: *mut bpf_iter__task_file) -> c_int {
    int fill_socket_owner(struct bpf_iter__task_file *ctx)
    {
    struct task_struct *task = ctx.task;
    struct file *file = ctx.file;
    struct socket *sock;
    int *sock_tgid;
    if (!task || !file)
    return 0;
    sock = bpf_sock_from_file(file);
    if (!sock)
    return 0;
    sock_tgid = bpf_sk_storage_get(&sk_stg_map, sock.sk, 0, 0);
    if (!sock_tgid)
    return 0;
// sock_tgid = task->tgid;
    return 0;
    }
    SEC("iter/tcp")
#[no_mangle]
pub unsafe extern "C" fn negate_socket_local_storage(ctx: *mut bpf_iter__tcp) -> c_int {
    int negate_socket_local_storage(struct bpf_iter__tcp *ctx)
    {
    struct sock_common *sk_common = ctx.sk_common;
    int *sock_tgid;
    if (!sk_common)
    return 0;
    sock_tgid = bpf_sk_storage_get(&sk_stg_map, sk_common, 0, 0);
    if (!sock_tgid)
    return 0;
// sock_tgid = -*sock_tgid;
    return 0;
    }
