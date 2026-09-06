//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ns_current_pid_tgid.c
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
// Copyright (c) 2019 Carlos Neira cneirabustos@gmail.com

    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 2);
    __type(key, __u32);
    __type(value, __u32);
    } sock_map SEC(".maps");
    let mut user_pid: __u64 = 0;
    let mut user_tgid: __u64 = 0;
    let mut dev: __u64 = 0;
    let mut ino: __u64 = 0;
#[no_mangle]
unsafe extern "C" fn get_pid_tgid() {
    static void get_pid_tgid(void)
    {
    struct bpf_pidns_info nsdata;
    if (bpf_get_ns_current_pid_tgid(dev, ino, &nsdata, sizeof(struct bpf_pidns_info)))
    return;
    user_pid = nsdata.pid;
    user_tgid = nsdata.tgid;
    }
    SEC("?tracepoint/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn tp_handler(ctx: *const c_void) -> c_int {
    int tp_handler(const void *ctx)
    {
    get_pid_tgid();
    return 0;
    }
    SEC("?cgroup/bind4")
#[no_mangle]
pub unsafe extern "C" fn cgroup_bind4(ctx: *mut bpf_sock_addr) -> c_int {
    int cgroup_bind4(struct bpf_sock_addr *ctx)
    {
    get_pid_tgid();
    return 1;
    }
    SEC("?sk_msg")
#[no_mangle]
pub unsafe extern "C" fn sk_msg(msg: *mut sk_msg_md) -> c_int {
    int sk_msg(struct sk_msg_md *msg)
    {
    get_pid_tgid();
    return SK_PASS;
    }
    char _license[] SEC("license") = "GPL";
