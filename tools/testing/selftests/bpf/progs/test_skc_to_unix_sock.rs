//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_skc_to_unix_sock.c
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
// Copyright (c) 2021 Hengqi Chen

    let mut my_pid: volatile pid_t = 0;
    char path[256] = {};
    SEC("fentry/unix_listen")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: unix_listen, sock: *mut socket, backlog: c_int) -> c_int {
    int BPF_PROG(unix_listen, struct socket *sock, int backlog)
    {
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    struct unix_sock *unix_sk;
    int i, len;
    if (pid != my_pid)
    return 0;
    unix_sk = (struct unix_sock *)bpf_skc_to_unix_sock(sock.sk);
    if (!unix_sk)
    return 0;
    if (unix_sk.addr.name.sun_path[0])
    return 0;
    len = unix_sk.addr.len - sizeof(short);
    path[0] = '@';
    for (i = 1; i < len; i++) {
    if (i >= (int)sizeof(struct sockaddr_un))
    break;
    path[i] = unix_sk.addr.name.sun_path[i];
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
