//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/getpeername_unix_prog.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    __u8 SERVUN_REWRITE_ADDRESS[] = "\0bpf_cgroup_unix_test_rewrite";
    SEC("cgroup/getpeername_unix")
#[no_mangle]
pub unsafe extern "C" fn getpeername_unix_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int getpeername_unix_prog(struct bpf_sock_addr *ctx)
    {
    struct bpf_sock_addr_kern *sa_kern = bpf_cast_to_kern_ctx(ctx);
    struct sockaddr_un *sa_kern_unaddr;
    __u32 unaddrlen = offsetof(struct sockaddr_un, sun_path) +
    sizeof(SERVUN_REWRITE_ADDRESS) - 1;
    int ret;
    ret = bpf_sock_addr_set_sun_path(sa_kern, SERVUN_REWRITE_ADDRESS,
    sizeof(SERVUN_REWRITE_ADDRESS) - 1);
    if (ret)
    return 1;
    if (sa_kern.uaddrlen != unaddrlen)
    return 1;
    sa_kern_unaddr = bpf_core_cast(sa_kern.uaddr, struct sockaddr_un);
    if (memcmp(sa_kern_unaddr.sun_path, SERVUN_REWRITE_ADDRESS,
    sizeof(SERVUN_REWRITE_ADDRESS) - 1) != 0)
    return 1;
    return 1;
    }
    char _license[] SEC("license") = "GPL";
