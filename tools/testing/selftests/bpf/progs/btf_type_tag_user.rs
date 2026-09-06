//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/btf_type_tag_user.c
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
// Copyright (c) 2022 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_btf_type_tag_1 {
    pub a: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_btf_type_tag_2 {
    pub p: *mut bpf_testmod_btf_type_tag_1,
}

    int g;
    SEC("fentry/bpf_testmod_test_btf_type_tag_user_1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_user1, arg: *mut bpf_testmod_btf_type_tag_1) -> c_int {
    int BPF_PROG(test_user1, struct bpf_testmod_btf_type_tag_1 *arg)
    {
    g = arg.a;
    return 0;
    }
    SEC("fentry/bpf_testmod_test_btf_type_tag_user_2")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_user2, arg: *mut bpf_testmod_btf_type_tag_2) -> c_int {
    int BPF_PROG(test_user2, struct bpf_testmod_btf_type_tag_2 *arg)
    {
    g = arg.p.a;
    return 0;
    }
// int __sys_getsockname(int fd, struct sockaddr __user *usockaddr,
// int __user *usockaddr_len);
//
    SEC("fentry/__sys_getsockname")
    int BPF_PROG(test_sys_getsockname, int fd, struct sockaddr *usockaddr,
    int *usockaddr_len)
    {
    g = usockaddr.sa_family;
    return 0;
    }
