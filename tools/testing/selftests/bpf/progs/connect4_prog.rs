//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/connect4_prog.c
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

pub const SRC_REWRITE_IP4: c_uint = 0x7f000004U;
pub const DST_REWRITE_IP4: c_uint = 0x7f000001U;
pub const DST_REWRITE_PORT4: c_int = 4444;

pub const TCP_CA_NAME_MAX: c_int = 16;

pub const TCP_NOTSENT_LOWAT: c_int = 25;

pub const IFNAMSIZ: c_int = 16;

pub const SOL_TCP: c_int = 6;

    const char reno[] = "reno";
    const char cubic[] = "cubic";
    __attribute__ ((noinline)) __weak
#[no_mangle]
pub unsafe extern "C" fn do_bind(ctx: *mut bpf_sock_addr) -> c_int {
    int do_bind(struct bpf_sock_addr *ctx)
    {
    let mut sa: sockaddr_in = {};
    sa.sin_family = AF_INET;
    sa.sin_port = bpf_htons(0);
    sa.sin_addr.s_addr = bpf_htonl(SRC_REWRITE_IP4);
    if (bpf_bind(ctx, (struct sockaddr *)&sa, sizeof(sa)) != 0)
    return 0;
    return 1;
    }
    static __inline int verify_cc(struct bpf_sock_addr *ctx,
    const char expected[])
    {
    char buf[TCP_CA_NAME_MAX];
    if (bpf_getsockopt(ctx, SOL_TCP, TCP_CONGESTION, &buf, sizeof(buf)))
    return 1;
    if (bpf_strncmp(buf, TCP_CA_NAME_MAX, expected))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_cc(ctx: *mut bpf_sock_addr) -> __inline int {
    static __inline int set_cc(struct bpf_sock_addr *ctx)
    {
    if (bpf_setsockopt(ctx, SOL_TCP, TCP_CONGESTION, (void *)reno, sizeof(reno)))
    return 1;
    if (verify_cc(ctx, reno))
    return 1;
    if (bpf_setsockopt(ctx, SOL_TCP, TCP_CONGESTION, (void *)cubic, sizeof(cubic)))
    return 1;
    if (verify_cc(ctx, cubic))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bind_to_device(ctx: *mut bpf_sock_addr) -> __inline int {
    static __inline int bind_to_device(struct bpf_sock_addr *ctx)
    {
    char veth1[IFNAMSIZ] = "test_sock_addr1";
    char veth2[IFNAMSIZ] = "test_sock_addr2";
    char missing[IFNAMSIZ] = "nonexistent_dev";
    char del_bind[IFNAMSIZ] = "";
    if (bpf_setsockopt(ctx, SOL_SOCKET, SO_BINDTODEVICE,
    &veth1, sizeof(veth1)))
    return 1;
    if (bpf_setsockopt(ctx, SOL_SOCKET, SO_BINDTODEVICE,
    &veth2, sizeof(veth2)))
    return 1;
    if (bpf_setsockopt(ctx, SOL_SOCKET, SO_BINDTODEVICE,
    &missing, sizeof(missing)) != -ENODEV)
    return 1;
    if (bpf_setsockopt(ctx, SOL_SOCKET, SO_BINDTODEVICE,
    &del_bind, sizeof(del_bind)))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_keepalive(ctx: *mut bpf_sock_addr) -> __inline int {
    static __inline int set_keepalive(struct bpf_sock_addr *ctx)
    {
    let mut zero: c_int = 0, one = 1;
    if (bpf_setsockopt(ctx, SOL_SOCKET, SO_KEEPALIVE, &one, sizeof(one)))
    return 1;
    if (ctx.type == SOCK_STREAM) {
    if (bpf_setsockopt(ctx, SOL_TCP, TCP_KEEPIDLE, &one, sizeof(one)))
    return 1;
    if (bpf_setsockopt(ctx, SOL_TCP, TCP_KEEPINTVL, &one, sizeof(one)))
    return 1;
    if (bpf_setsockopt(ctx, SOL_TCP, TCP_KEEPCNT, &one, sizeof(one)))
    return 1;
    if (bpf_setsockopt(ctx, SOL_TCP, TCP_SYNCNT, &one, sizeof(one)))
    return 1;
    if (bpf_setsockopt(ctx, SOL_TCP, TCP_USER_TIMEOUT, &one, sizeof(one)))
    return 1;
    }
    if (bpf_setsockopt(ctx, SOL_SOCKET, SO_KEEPALIVE, &zero, sizeof(zero)))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_notsent_lowat(ctx: *mut bpf_sock_addr) -> __inline int {
    static __inline int set_notsent_lowat(struct bpf_sock_addr *ctx)
    {
    let mut lowat: c_int = 65535;
    if (ctx.type == SOCK_STREAM) {
    if (bpf_setsockopt(ctx, SOL_TCP, TCP_NOTSENT_LOWAT, &lowat, sizeof(lowat)))
    return 1;
    }
    return 0;
    }
    SEC("cgroup/connect4")
#[no_mangle]
pub unsafe extern "C" fn connect_v4_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int connect_v4_prog(struct bpf_sock_addr *ctx)
    {
    let mut tuple: bpf_sock_tuple = {};
    struct bpf_sock *sk;
// Verify that new destination is available.
    memset(&tuple.ipv4.saddr, 0, sizeof(tuple.ipv4.saddr));
    memset(&tuple.ipv4.sport, 0, sizeof(tuple.ipv4.sport));
    tuple.ipv4.daddr = bpf_htonl(DST_REWRITE_IP4);
    tuple.ipv4.dport = bpf_htons(DST_REWRITE_PORT4);
// Bind to device and unbind it.
    if (bind_to_device(ctx))
    return 0;
    if (set_keepalive(ctx))
    return 0;
    if (set_notsent_lowat(ctx))
    return 0;
    if (ctx.type != SOCK_STREAM && ctx.type != SOCK_DGRAM)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(SOCK_STREAM: ctx->type ==) -> else {
    else if (ctx.type == SOCK_STREAM)
    sk = bpf_sk_lookup_tcp(ctx, &tuple, sizeof(tuple.ipv4),
    BPF_F_CURRENT_NETNS, 0);
    else
    sk = bpf_sk_lookup_udp(ctx, &tuple, sizeof(tuple.ipv4),
    BPF_F_CURRENT_NETNS, 0);
    if (!sk)
    return 0;
    if (sk.src_ip4 != tuple.ipv4.daddr ||
    sk.src_port != DST_REWRITE_PORT4) {
    bpf_sk_release(sk);
    return 0;
    }
    bpf_sk_release(sk);
// Rewrite congestion control.
    if (ctx.type == SOCK_STREAM && set_cc(ctx))
    return 0;
// Rewrite destination.
    ctx.user_ip4 = bpf_htonl(DST_REWRITE_IP4);
    ctx.user_port = bpf_htons(DST_REWRITE_PORT4);
    return do_bind(ctx) ? 1 : 0;
    }
    SEC("cgroup/connect4")
#[no_mangle]
pub unsafe extern "C" fn connect_v4_deny_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int connect_v4_deny_prog(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
