//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/socket_cookie_prog.c
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
// Copyright (c) 2018 Facebook

pub const AF_INET6: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket_cookie {
    pub cookie_key: __u64,
    pub cookie_value: __u32,
}

    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct socket_cookie);
    } socket_cookies SEC(".maps");
//
// These three programs get executed in a row on connect() syscalls. The
// userspace side of the test creates a client socket, issues a connect() on it
// and then checks that the local storage associated with this socket has:
// cookie_value == local_port << 8 | 0xFF
// The different parts of this cookie_value are appended by those hooks if they
// all agree on the output of bpf_get_socket_cookie().
//
    SEC("cgroup/connect6")
#[no_mangle]
pub unsafe extern "C" fn set_cookie(ctx: *mut bpf_sock_addr) -> c_int {
    int set_cookie(struct bpf_sock_addr *ctx)
    {
    struct socket_cookie *p;
    if (ctx.family != AF_INET6 || ctx.user_family != AF_INET6)
    return 1;
    p = bpf_sk_storage_get(&socket_cookies, ctx.sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
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
    struct bpf_sock *sk = ctx.sk;
    struct socket_cookie *p;
    if (ctx.family != AF_INET6)
    return 1;
    if (ctx.op != BPF_SOCK_OPS_TCP_CONNECT_CB)
    return 1;
    if (!sk)
    return 1;
    p = bpf_sk_storage_get(&socket_cookies, sk, 0, 0);
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
    p = bpf_sk_storage_get(&socket_cookies, sock.sk, 0, 0);
    if (!p)
    return 0;
    if (p.cookie_key != bpf_get_socket_cookie(sock.sk))
    return 0;
    p.cookie_value |= 0xF0;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
