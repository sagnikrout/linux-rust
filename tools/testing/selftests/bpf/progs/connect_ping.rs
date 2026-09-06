//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/connect_ping.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2022 Google LLC.
//

// 2001:db8::1

    let mut do_bind: __u32 = 0;
    let mut has_error: __u32 = 0;
    let mut invocations_v4: __u32 = 0;
    let mut invocations_v6: __u32 = 0;
    SEC("cgroup/connect4")
#[no_mangle]
pub unsafe extern "C" fn connect_v4_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int connect_v4_prog(struct bpf_sock_addr *ctx)
    {
    struct sockaddr_in sa = {
    .sin_family = AF_INET,
    .sin_addr.s_addr = bpf_htonl(0x01010101),
    };
    __sync_fetch_and_add(&invocations_v4, 1);
    if (do_bind && bpf_bind(ctx, (struct sockaddr *)&sa, sizeof(sa)))
    has_error = 1;
    return 1;
    }
    SEC("cgroup/connect6")
#[no_mangle]
pub unsafe extern "C" fn connect_v6_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int connect_v6_prog(struct bpf_sock_addr *ctx)
    {
    struct sockaddr_in6 sa = {
    .sin6_family = AF_INET6,
    .sin6_addr = BINDADDR_V6,
    };
    __sync_fetch_and_add(&invocations_v6, 1);
    if (do_bind && bpf_bind(ctx, (struct sockaddr *)&sa, sizeof(sa)))
    has_error = 1;
    return 1;
    }
    char _license[] SEC("license") = "GPL";
