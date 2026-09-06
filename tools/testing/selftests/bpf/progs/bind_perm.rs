//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bind_perm.c
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

#[no_mangle]
unsafe extern "C" fn bind_prog(ctx: *mut bpf_sock_addr, family: c_int) -> __always_inline int {
    static __always_inline int bind_prog(struct bpf_sock_addr *ctx, int family)
    {
    struct bpf_sock *sk;
    sk = ctx.sk;
    if (!sk)
    return 0;
    if (sk.family != family)
    return 0;
    if (ctx.type != SOCK_STREAM)
    return 0;
// Return 1 OR'ed with the first bit set to indicate
// that CAP_NET_BIND_SERVICE should be bypassed.
//
    if (ctx.user_port == bpf_htons(111))
    return (1 | 2);
    return 1;
    }
    SEC("cgroup/bind4")
#[no_mangle]
pub unsafe extern "C" fn bind_v4_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int bind_v4_prog(struct bpf_sock_addr *ctx)
    {
    return bind_prog(ctx, AF_INET);
    }
    SEC("cgroup/bind6")
#[no_mangle]
pub unsafe extern "C" fn bind_v6_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int bind_v6_prog(struct bpf_sock_addr *ctx)
    {
    return bind_prog(ctx, AF_INET6);
    }
    char _license[] SEC("license") = "GPL";
