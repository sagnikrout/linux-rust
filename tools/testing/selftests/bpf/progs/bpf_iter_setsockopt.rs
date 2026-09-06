//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_setsockopt.c
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
// Copyright (c) 2021 Facebook

    struct sock_common *_skc = skc;			\
    sk = core::ptr::null_mut();					\
    tp = core::ptr::null_mut();					\
    if (_skc) {					\
    tp = bpf_skc_to_tcp_sock(_skc);		\
    sk = (struct sock *)tp;			\
    }						\
    tp;						\
    })
    let mut reuse_listen_hport: c_ushort = 0;
    let mut listen_hport: c_ushort = 0;
    const char cubic_cc[] = "bpf_cubic";
    char dctcp_cc[TCP_CA_NAME_MAX] = "bpf_dctcp";
    let mut random_retry: bool = false;
    SEC("iter/tcp")
#[no_mangle]
pub unsafe extern "C" fn change_tcp_cc(ctx: *mut bpf_iter__tcp) -> c_int {
    int change_tcp_cc(struct bpf_iter__tcp *ctx)
    {
    char cur_cc[TCP_CA_NAME_MAX];
    struct tcp_sock *tp;
    struct sock *sk;
    if (!bpf_tcp_sk(ctx.sk_common))
    return 0;
    if (sk.sk_family != AF_INET6 ||
    (sk.sk_state != TCP_LISTEN &&
    sk.sk_state != TCP_ESTABLISHED) ||
    (sk.sk_num != reuse_listen_hport &&
    sk.sk_num != listen_hport &&
    bpf_ntohs(sk.sk_dport) != listen_hport))
    return 0;
    if (bpf_getsockopt(tp, SOL_TCP, TCP_CONGESTION,
    cur_cc, sizeof(cur_cc)))
    return 0;
    if (bpf_strncmp(cur_cc, TCP_CA_NAME_MAX, cubic_cc))
    return 0;
    if (random_retry && bpf_get_prandom_u32() % 4 == 1)
    return 1;
    bpf_setsockopt(tp, SOL_TCP, TCP_CONGESTION, dctcp_cc, sizeof(dctcp_cc));
    return 0;
    }
    char _license[] SEC("license") = "GPL";
