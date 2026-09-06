//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/ksock_lsm.c
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

    char send_data[32] = "hello from bpf ksock";
    __be32 ipv4_remote;
    __u16 remote_port;
    int target_pid;
    let mut send_ret: c_int = -1;
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn ksock_setup(ctx: *mut c_void) -> c_int {
    int ksock_setup(void *ctx)
    {
    let mut create_opts: bpf_ksock_create_opts = {};
    let mut addr: union bpf_ksock_addr = {};
    struct bpf_ksock *ks;
    let mut err: c_int = 0;
    create_opts.family = AF_INET;
    create_opts.type = SOCK_DGRAM;
    create_opts.protocol = IPPROTO_UDP;
    ks = bpf_ksock_create(&create_opts, sizeof(create_opts), &err);
    if (!ks)
    return err;
    addr.sin.sin_family = AF_INET;
    addr.sin.sin_port = bpf_htons(remote_port);
    addr.sin.sin_addr.s_addr = ipv4_remote;
    err = bpf_ksock_connect(ks, &addr, sizeof(addr));
    if (err) {
    bpf_ksock_release(ks);
    return err;
    }
    err = ksock_ctx_insert(ks);
    if (err && err != -EEXIST)
    return err;
    return 0;
    }
    SEC("lsm.s/socket_bind")
    int BPF_PROG(ksock_socket_bind, struct socket *sock, struct sockaddr *address,
    int addrlen, int ret)
    {
    struct bpf_ksock *ks;
    let mut pid: u32 = bpf_get_current_pid_tgid() >> 32;
    if (ret || pid != target_pid)
    return ret;
    ks = ksock_ctx_get();
    if (!ks) {
    send_ret = -ENOENT;
    return ret;
    }
    send_ret = bpf_ksock_send(ks, send_data, sizeof(send_data));
    bpf_ksock_release(ks);
    return ret;
    }
    char __license[] SEC("license") = "GPL";
