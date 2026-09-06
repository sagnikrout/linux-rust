//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sock_read_xattr.c
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
// Copyright (c) 2026 Christian Brauner

    char _license[] SEC("license") = "GPL";
    char value[16];
    let mut read_ret: c_int = -1;
    let mut monitored_pid: __u32 = 0;
#[no_mangle]
unsafe extern "C" fn read_xattr(sock: *mut socket) -> __always_inline void {
    static __always_inline void read_xattr(struct socket *sock)
    {
    struct bpf_dynptr value_ptr;
    bpf_dynptr_from_mem(value, sizeof(value), 0, &value_ptr);
    bpf_sock_read_xattr(sock, "user.bpf_test", &value_ptr);
    }
    SEC("lsm.s/socket_connect")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trusted_sock_ptr_sleepable, sock: *mut socket) -> c_int {
    int BPF_PROG(trusted_sock_ptr_sleepable, struct socket *sock)
    {
    read_xattr(sock);
    return 0;
    }
    SEC("lsm/socket_connect")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trusted_sock_ptr_non_sleepable, sock: *mut socket) -> c_int {
    int BPF_PROG(trusted_sock_ptr_non_sleepable, struct socket *sock)
    {
    read_xattr(sock);
    return 0;
    }
    SEC("lsm.s/socket_connect")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: read_sock_xattr, sock: *mut socket) -> c_int {
    int BPF_PROG(read_sock_xattr, struct socket *sock)
    {
    struct bpf_dynptr value_ptr;
    let mut pid: __u32 = bpf_get_current_pid_tgid() >> 32;
    if (pid != monitored_pid)
    return 0;
    bpf_dynptr_from_mem(value, sizeof(value), 0, &value_ptr);
    read_ret = bpf_sock_read_xattr(sock, "user.bpf_test", &value_ptr);
    return 0;
    }
