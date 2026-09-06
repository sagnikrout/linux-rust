//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/mptcp_sock.c
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
// Copyright (c) 2020, Tessares SA.
// Copyright (c) 2022, SUSE.

    char _license[] SEC("license") = "GPL";
    let mut token: __u32 = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_storage {
    pub invoked: __u32,
    pub is_mptcp: __u32,
    pub sk: *mut sock,
    pub token: __u32,
    pub first: *mut sock,
    pub ca_name: [c_char; TCP_CA_NAME_MAX],
}

    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct mptcp_storage);
    } socket_storage_map SEC(".maps");
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn _sockops(ctx: *mut bpf_sock_ops) -> c_int {
    int _sockops(struct bpf_sock_ops *ctx)
    {
    struct mptcp_storage *storage;
    struct mptcp_sock *msk;
    let mut op: c_int = (int)ctx.op;
    struct tcp_sock *tsk;
    struct bpf_sock *sk;
    bool is_mptcp;
    if (op != BPF_SOCK_OPS_TCP_CONNECT_CB)
    return 1;
    sk = ctx.sk;
    if (!sk)
    return 1;
    tsk = bpf_skc_to_tcp_sock(sk);
    if (!tsk)
    return 1;
    is_mptcp = bpf_core_field_exists(tsk.is_mptcp) ? tsk.is_mptcp : 0;
    if (!is_mptcp) {
    storage = bpf_sk_storage_get(&socket_storage_map, sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    if (!storage)
    return 1;
    storage.token = 0;
    __builtin_memset(storage.ca_name, 0, TCP_CA_NAME_MAX);
    storage.first = core::ptr::null_mut();
    } else {
    msk = bpf_skc_to_mptcp_sock(sk);
    if (!msk)
    return 1;
    storage = bpf_sk_storage_get(&socket_storage_map, msk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    if (!storage)
    return 1;
    storage.token = msk.token;
    __builtin_memcpy(storage.ca_name, msk.ca_name, TCP_CA_NAME_MAX);
    storage.first = msk.first;
    }
    storage.invoked++;
    storage.is_mptcp = is_mptcp;
    storage.sk = (struct sock *)sk;
    return 1;
    }
    SEC("fentry/mptcp_pm_new_connection")
    int BPF_PROG(trace_mptcp_pm_new_connection, struct mptcp_sock *msk,
    const struct sock *ssk, int server_side)
    {
    if (!server_side)
    token = msk.token;
    return 0;
    }
