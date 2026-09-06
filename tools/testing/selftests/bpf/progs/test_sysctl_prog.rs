//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sysctl_prog.c
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
// Copyright (c) 2019 Facebook

// Max supported length of a string with unsigned long in base 10 (pow2 - 1).
pub const MAX_ULONG_STR_LEN: c_uint = 0xF;
// Max supported length of sysctl value string (pow2).
pub const MAX_VALUE_STR_LEN: c_uint = 0x40;
    const char tcp_mem_name[] = "net/ipv4/tcp_mem";
#[no_mangle]
unsafe extern "C" fn is_tcp_mem(ctx: *mut bpf_sysctl) -> __always_inline int {
    static __always_inline int is_tcp_mem(struct bpf_sysctl *ctx)
    {
    unsigned char i;
    char name[sizeof(tcp_mem_name)];
    int ret;
    memset(name, 0, sizeof(name));
    ret = bpf_sysctl_get_name(ctx, name, sizeof(name), 0);
    if (ret < 0 || ret != sizeof(tcp_mem_name) - 1)
    return 0;
    __pragma_loop_unroll_full
    for (i = 0; i < sizeof(tcp_mem_name); ++i)
    if (name[i] != tcp_mem_name[i])
    return 0;
    return 1;
    }
    SEC("cgroup/sysctl")
#[no_mangle]
pub unsafe extern "C" fn sysctl_tcp_mem(ctx: *mut bpf_sysctl) -> c_int {
    int sysctl_tcp_mem(struct bpf_sysctl *ctx)
    {
    unsigned long tcp_mem[3] = {0, 0, 0};
    char value[MAX_VALUE_STR_LEN];
    unsigned char i, off = 0;
    volatile int ret;
    if (ctx.write)
    return 0;
    if (!is_tcp_mem(ctx))
    return 0;
    ret = bpf_sysctl_get_current_value(ctx, value, MAX_VALUE_STR_LEN);
    if (ret < 0 || ret >= MAX_VALUE_STR_LEN)
    return 0;
    __pragma_loop_unroll_full
    for (i = 0; i < ARRAY_SIZE(tcp_mem); ++i) {
    ret = bpf_strtoul(value + off, MAX_ULONG_STR_LEN, 0,
    tcp_mem + i);
    if (ret <= 0 || ret > MAX_ULONG_STR_LEN)
    return 0;
    off += ret & MAX_ULONG_STR_LEN;
    }
    return tcp_mem[0] < tcp_mem[1] && tcp_mem[1] < tcp_mem[2];
    }
    char _license[] SEC("license") = "GPL";
