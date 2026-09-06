//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sockopt_qos_to_cc.c
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

    char _license[] SEC("license") = "GPL";
    let mut page_size: __s32 = 0;
    const char cc_reno[TCP_CA_NAME_MAX] = "reno";
    const char cc_cubic[TCP_CA_NAME_MAX] = "cubic";
    SEC("cgroup/setsockopt")
#[no_mangle]
pub unsafe extern "C" fn sockopt_qos_to_cc(ctx: *mut bpf_sockopt) -> c_int {
    int sockopt_qos_to_cc(struct bpf_sockopt *ctx)
    {
    void *optval_end = ctx.optval_end;
    int *optval = ctx.optval;
    char buf[TCP_CA_NAME_MAX];
    if (ctx.level != SOL_IPV6 || ctx.optname != IPV6_TCLASS)
    goto out;
    if (optval + 1 > optval_end)
    return 0; /* EPERM, bounds check */
    if (bpf_getsockopt(ctx.sk, SOL_TCP, TCP_CONGESTION, &buf, sizeof(buf)))
    return 0;
    if (bpf_strncmp(buf, sizeof(buf), cc_cubic))
    return 0;
    if (*optval == 0x2d) {
    if (bpf_setsockopt(ctx.sk, SOL_TCP, TCP_CONGESTION, (void *)&cc_reno,
    sizeof(cc_reno)))
    return 0;
    }
    return 1;
    out:
// optval larger than PAGE_SIZE use kernel's buffer.
    if (ctx.optlen > page_size)
    ctx.optlen = 0;
    return 1;
    }
