//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sock_iter_batch.c
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
// Copyright (c) 2024 Meta

#[no_mangle]
unsafe extern "C" fn ipv6_addr_loopback(a: *const in6_addr) -> bool {
    static bool ipv6_addr_loopback(const struct in6_addr *a)
    {
    return (a.s6_addr32[0] | a.s6_addr32[1] |
    a.s6_addr32[2] | (a.s6_addr32[3] ^ bpf_htonl(1))) == 0;
    }
#[no_mangle]
unsafe extern "C" fn ipv4_addr_loopback(a: __be32) -> bool {
    static bool ipv4_addr_loopback(__be32 a)
    {
    let mut a: return = = bpf_ntohl(0x7f000001);
    }
    volatile const unsigned int sf;
    volatile const unsigned int ss;
    volatile const __u16 ports[2];
    unsigned int bucket[2];
    SEC("iter/tcp")
#[no_mangle]
pub unsafe extern "C" fn iter_tcp_soreuse(ctx: *mut bpf_iter__tcp) -> c_int {
    int iter_tcp_soreuse(struct bpf_iter__tcp *ctx)
    {
    struct sock *sk = (struct sock *)ctx.sk_common;
    struct inet_hashinfo *hinfo;
    unsigned int hash;
    __u64 sock_cookie;
    struct net *net;
    int idx;
    if (!sk)
    return 0;
    sock_cookie = bpf_get_socket_cookie(sk);
    sk = bpf_core_cast(sk, struct sock);
    if (sk.sk_family != sf ||
    (ss && sk.sk_state != ss) ||
    (sk.sk_family == AF_INET6 ?
    !ipv6_addr_loopback(&sk.sk_v6_rcv_saddr) :
    !ipv4_addr_loopback(sk.sk_rcv_saddr)))
    return 0;
    if (sk.sk_num == ports[0])
    idx = 0;
#[no_mangle]
pub unsafe extern "C" fn if(ports[1]: sk->sk_num ==) -> else {
    else if (sk.sk_num == ports[1])
    idx = 1;
#[no_mangle]
pub unsafe extern "C" fn if(!ports[1]: !ports[0] &&) -> else {
    else if (!ports[0] && !ports[1])
    idx = 0;
    else
    return 0;
// bucket selection as in inet_lhash2_bucket_sk()
    net = sk.sk_net.net;
    hash = jhash2(sk.sk_v6_rcv_saddr.s6_addr32, 4, net.hash_mix);
    hash ^= sk.sk_num;
    hinfo = net.ipv4.tcp_death_row.hashinfo;
    bucket[idx] = hash & hinfo.lhash2_mask;
    bpf_seq_write(ctx.meta.seq, &idx, sizeof(idx));
    bpf_seq_write(ctx.meta.seq, &sock_cookie, sizeof(sock_cookie));
    return 0;
    }
    volatile const __u64 destroy_cookie;
    SEC("iter/tcp")
#[no_mangle]
pub unsafe extern "C" fn iter_tcp_destroy(ctx: *mut bpf_iter__tcp) -> c_int {
    int iter_tcp_destroy(struct bpf_iter__tcp *ctx)
    {
    struct sock_common *sk_common = (struct sock_common *)ctx.sk_common;
    __u64 sock_cookie;
    if (!sk_common)
    return 0;
    sock_cookie = bpf_get_socket_cookie(sk_common);
    if (sock_cookie != destroy_cookie)
    return 0;
    bpf_sock_destroy(sk_common);
    bpf_seq_write(ctx.meta.seq, &sock_cookie, sizeof(sock_cookie));
    return 0;
    }

    SEC("iter/udp")
#[no_mangle]
pub unsafe extern "C" fn iter_udp_soreuse(ctx: *mut bpf_iter__udp) -> c_int {
    int iter_udp_soreuse(struct bpf_iter__udp *ctx)
    {
    struct sock *sk = (struct sock *)ctx.udp_sk;
    struct udp_table *udptable;
    __u64 sock_cookie;
    int idx;
    if (!sk)
    return 0;
    sock_cookie = bpf_get_socket_cookie(sk);
    sk = bpf_core_cast(sk, struct sock);
    if (sk.sk_family != sf ||
    (sk.sk_family == AF_INET6 ?
    !ipv6_addr_loopback(&sk.sk_v6_rcv_saddr) :
    !ipv4_addr_loopback(sk.sk_rcv_saddr)))
    return 0;
    if (sk.sk_num == ports[0])
    idx = 0;
#[no_mangle]
pub unsafe extern "C" fn if(ports[1]: sk->sk_num ==) -> else {
    else if (sk.sk_num == ports[1])
    idx = 1;
#[no_mangle]
pub unsafe extern "C" fn if(!ports[1]: !ports[0] &&) -> else {
    else if (!ports[0] && !ports[1])
    idx = 0;
    else
    return 0;
// bucket selection as in udp_hashslot2()
    udptable = sk.sk_net.net.ipv4.udp_table;
    bucket[idx] = udp_sk(sk).udp_portaddr_hash & udptable.mask;
    bpf_seq_write(ctx.meta.seq, &idx, sizeof(idx));
    bpf_seq_write(ctx.meta.seq, &sock_cookie, sizeof(sock_cookie));
    return 0;
    }
    char _license[] SEC("license") = "GPL";
