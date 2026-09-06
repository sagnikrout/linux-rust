//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_setsockopt_unix.c
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
// Copyright Amazon.com Inc. or its affiliates.

pub const AUTOBIND_LEN: c_int = 6;
    char sun_path[AUTOBIND_LEN];
pub const NR_CASES: c_int = 5;
    int sndbuf_setsockopt[NR_CASES] = {-1, 0, 8192, INT_MAX / 2, INT_MAX};
    int sndbuf_getsockopt[NR_CASES] = {-1, -1, -1, -1, -1};
    int sndbuf_getsockopt_expected[NR_CASES];
#[no_mangle]
pub unsafe extern "C" fn cmpname(unix_sk: *mut unix_sock) -> c_int {
    static inline int cmpname(struct unix_sock *unix_sk)
    {
    int i;
    for (i = 0; i < AUTOBIND_LEN; i++) {
    if (unix_sk.addr.name.sun_path[i] != sun_path[i])
    return -1;
    }
    return 0;
    }
    SEC("iter/unix")
#[no_mangle]
pub unsafe extern "C" fn change_sndbuf(ctx: *mut bpf_iter__unix) -> c_int {
    int change_sndbuf(struct bpf_iter__unix *ctx)
    {
    struct unix_sock *unix_sk = ctx.unix_sk;
    int i, err;
    if (!unix_sk || !unix_sk.addr)
    return 0;
    if (unix_sk.addr.name.sun_path[0])
    return 0;
    if (cmpname(unix_sk))
    return 0;
    for (i = 0; i < NR_CASES; i++) {
    err = bpf_setsockopt(unix_sk, SOL_SOCKET, SO_SNDBUF,
    &sndbuf_setsockopt[i],
    sizeof(sndbuf_setsockopt[i]));
    if (err)
    break;
    err = bpf_getsockopt(unix_sk, SOL_SOCKET, SO_SNDBUF,
    &sndbuf_getsockopt[i],
    sizeof(sndbuf_getsockopt[i]));
    if (err)
    break;
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
