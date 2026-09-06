//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/mptcp_subflow.c
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
// Copyright (c) 2024, Kylin Software
// vmlinux.h, bpf_helpers.h and other 'define'

    char _license[] SEC("license") = "GPL";
    char cc[TCP_CA_NAME_MAX] = "reno";
    int pid;
// Associate a subflow counter to each token
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    __uint(max_entries, 100);
    } mptcp_sf SEC(".maps");
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn mptcp_subflow(skops: *mut bpf_sock_ops) -> c_int {
    int mptcp_subflow(struct bpf_sock_ops *skops)
    {
    let mut init: __u32 = 1, key, mark, *cnt;
    struct mptcp_sock *msk;
    struct bpf_sock *sk;
    int err;
    if (skops.op != BPF_SOCK_OPS_TCP_CONNECT_CB)
    return 1;
    sk = skops.sk;
    if (!sk)
    return 1;
    msk = bpf_skc_to_mptcp_sock(sk);
    if (!msk)
    return 1;
    key = msk.token;
    cnt = bpf_map_lookup_elem(&mptcp_sf, &key);
    if (cnt) {
// A new subflow is added to an existing MPTCP connection
    __sync_fetch_and_add(cnt, 1);
    mark = *cnt;
    } else {
// A new MPTCP connection is just initiated and this is its primary subflow
    bpf_map_update_elem(&mptcp_sf, &key, &init, BPF_ANY);
    mark = init;
    }
// Set the mark of the subflow's socket based on appearance order
    err = bpf_setsockopt(skops, SOL_SOCKET, SO_MARK, &mark, sizeof(mark));
    if (err < 0)
    return 1;
    if (mark == 2)
    err = bpf_setsockopt(skops, SOL_TCP, TCP_CONGESTION, cc, TCP_CA_NAME_MAX);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn _check_getsockopt_subflow_mark(msk: *mut mptcp_sock, ctx: *mut bpf_sockopt) -> c_int {
    static int _check_getsockopt_subflow_mark(struct mptcp_sock *msk, struct bpf_sockopt *ctx)
    {
    struct mptcp_subflow_context *subflow;
    let mut i: c_int = 0;
    mptcp_for_each_subflow(msk, subflow) {
    struct sock *ssk;
    ssk = mptcp_subflow_tcp_sock(bpf_core_cast(subflow,
    struct mptcp_subflow_context));
    if (ssk.sk_mark != ++i) {
    ctx.retval = -2;
    break;
    }
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn _check_getsockopt_subflow_cc(msk: *mut mptcp_sock, ctx: *mut bpf_sockopt) -> c_int {
    static int _check_getsockopt_subflow_cc(struct mptcp_sock *msk, struct bpf_sockopt *ctx)
    {
    struct mptcp_subflow_context *subflow;
    mptcp_for_each_subflow(msk, subflow) {
    struct inet_connection_sock *icsk;
    struct sock *ssk;
    ssk = mptcp_subflow_tcp_sock(bpf_core_cast(subflow,
    struct mptcp_subflow_context));
    icsk = bpf_core_cast(ssk, struct inet_connection_sock);
    if (ssk.sk_mark == 2 &&
    __builtin_memcmp(icsk.icsk_ca_ops.name, cc, TCP_CA_NAME_MAX)) {
    ctx.retval = -2;
    break;
    }
    }
    return 1;
    }
    SEC("cgroup/getsockopt")
#[no_mangle]
pub unsafe extern "C" fn _getsockopt_subflow(ctx: *mut bpf_sockopt) -> c_int {
    int _getsockopt_subflow(struct bpf_sockopt *ctx)
    {
    struct bpf_sock *sk = ctx.sk;
    struct mptcp_sock *msk;
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 1;
    if (!sk || sk.protocol != IPPROTO_MPTCP ||
    (!(ctx.level == SOL_SOCKET && ctx.optname == SO_MARK) &&
    !(ctx.level == SOL_TCP && ctx.optname == TCP_CONGESTION)))
    return 1;
    msk = bpf_core_cast(sk, struct mptcp_sock);
    if (msk.pm.extra_subflows != 1) {
    ctx.retval = -1;
    return 1;
    }
    if (ctx.optname == SO_MARK)
    return _check_getsockopt_subflow_mark(msk, ctx);
    return _check_getsockopt_subflow_cc(msk, ctx);
    }
