//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/connect6_prog.c
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

pub const SRC_REWRITE_IP6_0: c_int = 0;
pub const SRC_REWRITE_IP6_1: c_int = 0;
pub const SRC_REWRITE_IP6_2: c_int = 0;
pub const SRC_REWRITE_IP6_3: c_int = 6;
pub const DST_REWRITE_IP6_0: c_int = 0;
pub const DST_REWRITE_IP6_1: c_int = 0;
pub const DST_REWRITE_IP6_2: c_int = 0;
pub const DST_REWRITE_IP6_3: c_int = 1;
pub const DST_REWRITE_PORT6: c_int = 6666;
    SEC("cgroup/connect6")
#[no_mangle]
pub unsafe extern "C" fn connect_v6_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int connect_v6_prog(struct bpf_sock_addr *ctx)
    {
    let mut tuple: bpf_sock_tuple = {};
    struct sockaddr_in6 sa;
    struct bpf_sock *sk;
// Verify that new destination is available.
    memset(&tuple.ipv6.saddr, 0, sizeof(tuple.ipv6.saddr));
    memset(&tuple.ipv6.sport, 0, sizeof(tuple.ipv6.sport));
    tuple.ipv6.daddr[0] = bpf_htonl(DST_REWRITE_IP6_0);
    tuple.ipv6.daddr[1] = bpf_htonl(DST_REWRITE_IP6_1);
    tuple.ipv6.daddr[2] = bpf_htonl(DST_REWRITE_IP6_2);
    tuple.ipv6.daddr[3] = bpf_htonl(DST_REWRITE_IP6_3);
    tuple.ipv6.dport = bpf_htons(DST_REWRITE_PORT6);
    if (ctx.type != SOCK_STREAM && ctx.type != SOCK_DGRAM)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(SOCK_STREAM: ctx->type ==) -> else {
    else if (ctx.type == SOCK_STREAM)
    sk = bpf_sk_lookup_tcp(ctx, &tuple, sizeof(tuple.ipv6),
    BPF_F_CURRENT_NETNS, 0);
    else
    sk = bpf_sk_lookup_udp(ctx, &tuple, sizeof(tuple.ipv6),
    BPF_F_CURRENT_NETNS, 0);
    if (!sk)
    return 0;
    if (sk.src_ip6[0] != tuple.ipv6.daddr[0] ||
    sk.src_ip6[1] != tuple.ipv6.daddr[1] ||
    sk.src_ip6[2] != tuple.ipv6.daddr[2] ||
    sk.src_ip6[3] != tuple.ipv6.daddr[3] ||
    sk.src_port != DST_REWRITE_PORT6) {
    bpf_sk_release(sk);
    return 0;
    }
    bpf_sk_release(sk);
// Rewrite destination.
    ctx.user_ip6[0] = bpf_htonl(DST_REWRITE_IP6_0);
    ctx.user_ip6[1] = bpf_htonl(DST_REWRITE_IP6_1);
    ctx.user_ip6[2] = bpf_htonl(DST_REWRITE_IP6_2);
    ctx.user_ip6[3] = bpf_htonl(DST_REWRITE_IP6_3);
    ctx.user_port = bpf_htons(DST_REWRITE_PORT6);
// Rewrite source.
    memset(&sa, 0, sizeof(sa));
    sa.sin6_family = AF_INET6;
    sa.sin6_port = bpf_htons(0);
    sa.sin6_addr.s6_addr32[0] = bpf_htonl(SRC_REWRITE_IP6_0);
    sa.sin6_addr.s6_addr32[1] = bpf_htonl(SRC_REWRITE_IP6_1);
    sa.sin6_addr.s6_addr32[2] = bpf_htonl(SRC_REWRITE_IP6_2);
    sa.sin6_addr.s6_addr32[3] = bpf_htonl(SRC_REWRITE_IP6_3);
    if (bpf_bind(ctx, (struct sockaddr *)&sa, sizeof(sa)) != 0)
    return 0;
    return 1;
    }
    SEC("cgroup/connect6")
#[no_mangle]
pub unsafe extern "C" fn connect_v6_deny_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int connect_v6_deny_prog(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
