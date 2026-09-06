//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sock_destroy_prog.c
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

    let mut serv_port: __be16 = 0;
    int bpf_sock_destroy(struct sock_common *sk) __ksym;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } tcp_conn_sockets SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } udp_conn_sockets SEC(".maps");
    SEC("cgroup/connect6")
#[no_mangle]
pub unsafe extern "C" fn sock_connect(ctx: *mut bpf_sock_addr) -> c_int {
    int sock_connect(struct bpf_sock_addr *ctx)
    {
    let mut sock_cookie: __u64 = 0;
    let mut key: c_int = 0;
    let mut keyc: __u32 = 0;
    if (ctx.family != AF_INET6 || ctx.user_family != AF_INET6)
    return 1;
    sock_cookie = bpf_get_socket_cookie(ctx);
    if (ctx.protocol == IPPROTO_TCP)
    bpf_map_update_elem(&tcp_conn_sockets, &key, &sock_cookie, 0);
#[no_mangle]
pub unsafe extern "C" fn if(IPPROTO_UDP: ctx->protocol ==) -> else {
    else if (ctx.protocol == IPPROTO_UDP)
    bpf_map_update_elem(&udp_conn_sockets, &keyc, &sock_cookie, 0);
    else
    return 1;
    return 1;
    }
    SEC("iter/tcp")
#[no_mangle]
pub unsafe extern "C" fn iter_tcp6_client(ctx: *mut bpf_iter__tcp) -> c_int {
    int iter_tcp6_client(struct bpf_iter__tcp *ctx)
    {
    struct sock_common *sk_common = ctx.sk_common;
    let mut sock_cookie: __u64 = 0;
    __u64 *val;
    let mut key: c_int = 0;
    if (!sk_common)
    return 0;
    if (sk_common.skc_family != AF_INET6)
    return 0;
    sock_cookie  = bpf_get_socket_cookie(sk_common);
    val = bpf_map_lookup_elem(&tcp_conn_sockets, &key);
    if (!val)
    return 0;
// Destroy connected client sockets.
    if (sock_cookie == *val)
    bpf_sock_destroy(sk_common);
    return 0;
    }
    SEC("iter/tcp")
#[no_mangle]
pub unsafe extern "C" fn iter_tcp6_server(ctx: *mut bpf_iter__tcp) -> c_int {
    int iter_tcp6_server(struct bpf_iter__tcp *ctx)
    {
    struct sock_common *sk_common = ctx.sk_common;
    const struct inet_connection_sock *icsk;
    const struct inet_sock *inet;
    struct tcp6_sock *tcp_sk;
    __be16 srcp;
    if (!sk_common)
    return 0;
    if (sk_common.skc_family != AF_INET6)
    return 0;
    tcp_sk = bpf_skc_to_tcp6_sock(sk_common);
    if (!tcp_sk)
    return 0;
    icsk = &tcp_sk.tcp.inet_conn;
    inet = &icsk.icsk_inet;
    srcp = inet.inet_sport;
// Destroy server sockets.
    if (srcp == serv_port)
    bpf_sock_destroy(sk_common);
    return 0;
    }
    SEC("iter/udp")
#[no_mangle]
pub unsafe extern "C" fn iter_udp6_client(ctx: *mut bpf_iter__udp) -> c_int {
    int iter_udp6_client(struct bpf_iter__udp *ctx)
    {
    struct udp_sock *udp_sk = ctx.udp_sk;
    struct sock *sk = (struct sock *) udp_sk;
    let mut sock_cookie: __u64 = 0, *val;
    let mut key: c_int = 0;
    if (!sk)
    return 0;
    sock_cookie  = bpf_get_socket_cookie(sk);
    val = bpf_map_lookup_elem(&udp_conn_sockets, &key);
    if (!val)
    return 0;
// Destroy connected client sockets.
    if (sock_cookie == *val)
    bpf_sock_destroy((struct sock_common *)sk);
    return 0;
    }
    SEC("iter/udp")
#[no_mangle]
pub unsafe extern "C" fn iter_udp6_server(ctx: *mut bpf_iter__udp) -> c_int {
    int iter_udp6_server(struct bpf_iter__udp *ctx)
    {
    struct udp_sock *udp_sk = ctx.udp_sk;
    struct sock *sk = (struct sock *) udp_sk;
    struct inet_sock *inet;
    __be16 srcp;
    if (!sk)
    return 0;
    inet = &udp_sk.inet;
    srcp = inet.inet_sport;
    if (srcp == serv_port)
    bpf_sock_destroy((struct sock_common *)sk);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
