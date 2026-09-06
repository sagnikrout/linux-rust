//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgrp_ls_attach_cgroup.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket_cookie {
    pub cookie_key: __u64,
    pub cookie_value: __u64,
}

    struct {
    __uint(type, BPF_MAP_TYPE_CGRP_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct socket_cookie);
    } socket_cookies SEC(".maps");
    SEC("cgroup/connect6")
#[no_mangle]
pub unsafe extern "C" fn set_cookie(ctx: *mut bpf_sock_addr) -> c_int {
    int set_cookie(struct bpf_sock_addr *ctx)
    {
    struct socket_cookie *p;
    struct tcp_sock *tcp_sk;
    struct bpf_sock *sk;
    if (ctx.family != AF_INET6 || ctx.user_family != AF_INET6)
    return 1;
    sk = ctx.sk;
    if (!sk)
    return 1;
    tcp_sk = bpf_skc_to_tcp_sock(sk);
    if (!tcp_sk)
    return 1;
    p = bpf_cgrp_storage_get(&socket_cookies,
    tcp_sk.inet_conn.icsk_inet.sk.sk_cgrp_data.cgroup, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!p)
    return 1;
    p.cookie_value = 0xF;
    p.cookie_key = bpf_get_socket_cookie(ctx);
    return 1;
    }
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn update_cookie_sockops(ctx: *mut bpf_sock_ops) -> c_int {
    int update_cookie_sockops(struct bpf_sock_ops *ctx)
    {
    struct socket_cookie *p;
    struct tcp_sock *tcp_sk;
    struct bpf_sock *sk;
    if (ctx.family != AF_INET6 || ctx.op != BPF_SOCK_OPS_TCP_CONNECT_CB)
    return 1;
    sk = ctx.sk;
    if (!sk)
    return 1;
    tcp_sk = bpf_skc_to_tcp_sock(sk);
    if (!tcp_sk)
    return 1;
    p = bpf_cgrp_storage_get(&socket_cookies,
    tcp_sk.inet_conn.icsk_inet.sk.sk_cgrp_data.cgroup, 0, 0);
    if (!p)
    return 1;
    if (p.cookie_key != bpf_get_socket_cookie(ctx))
    return 1;
    p.cookie_value |= (ctx.local_port << 8);
    return 1;
    }
    SEC("fexit/inet_stream_connect")
    int BPF_PROG(update_cookie_tracing, struct socket *sock,
    struct sockaddr *uaddr, int addr_len, int flags)
    {
    struct socket_cookie *p;
    if (uaddr.sa_family != AF_INET6)
    return 0;
    p = bpf_cgrp_storage_get(&socket_cookies, sock.sk.sk_cgrp_data.cgroup, 0, 0);
    if (!p)
    return 0;
    if (p.cookie_key != bpf_get_socket_cookie(sock.sk))
    return 0;
    p.cookie_value |= 0xF0;
    return 0;
    }
