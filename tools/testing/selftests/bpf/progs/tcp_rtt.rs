//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tcp_rtt.c
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

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_rtt_storage {
    pub invoked: __u32,
    pub dsack_dups: __u32,
    pub delivered: __u32,
    pub delivered_ce: __u32,
    pub icsk_retransmits: __u32,
    pub /: *mut *mut __u32 mrtt_us; / args[0],
    pub /: *mut *mut __u32 srtt; / args[1],
}

    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct tcp_rtt_storage);
    } socket_storage_map SEC(".maps");
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn _sockops(ctx: *mut bpf_sock_ops) -> c_int {
    int _sockops(struct bpf_sock_ops *ctx)
    {
    struct tcp_rtt_storage *storage;
    struct bpf_tcp_sock *tcp_sk;
    let mut op: c_int = (int) ctx.op;
    struct bpf_sock *sk;
    sk = ctx.sk;
    if (!sk)
    return 1;
    storage = bpf_sk_storage_get(&socket_storage_map, sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    if (!storage)
    return 1;
    if (op == BPF_SOCK_OPS_TCP_CONNECT_CB) {
    bpf_sock_ops_cb_flags_set(ctx, BPF_SOCK_OPS_RTT_CB_FLAG);
    return 1;
    }
    if (op != BPF_SOCK_OPS_RTT_CB)
    return 1;
    tcp_sk = bpf_tcp_sock(sk);
    if (!tcp_sk)
    return 1;
    storage.invoked++;
    storage.dsack_dups = tcp_sk.dsack_dups;
    storage.delivered = tcp_sk.delivered;
    storage.delivered_ce = tcp_sk.delivered_ce;
    storage.icsk_retransmits = tcp_sk.icsk_retransmits;
    storage.mrtt_us = ctx.args[0];
    storage.srtt = ctx.args[1];
    return 1;
    }
