//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/ksock_lsm_verifier.c
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
// Copyright (c) 2026 Isovalent

    char send_data[11] = "dummy data";
    SEC("lsm.s/socket_sendmsg")
    __description("bpf_ksock_send is rejected from socket_sendmsg LSM hook")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "calling kernel function bpf_ksock_send is not) -> __failure {
    __failure __msg("calling kernel function bpf_ksock_send is not allowed")
    int BPF_PROG(ksock_socket_sendmsg, struct socket *sock, struct msghdr *msg,
    int size, int ret)
    {
    struct __ksock_ctx_value *v;
    struct bpf_ksock *ks;
    v = ksock_ctx_value_lookup();
    if (!v)
    return ret;
    ks = bpf_kptr_xchg(&v.ctx, core::ptr::null_mut());
    if (!ks)
    return ret;
    bpf_ksock_send(ks, send_data, sizeof(send_data));
    bpf_ksock_release(ks);
    return ret;
    }
    char __license[] SEC("license") = "GPL";
