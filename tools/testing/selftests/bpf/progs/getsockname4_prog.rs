//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/getsockname4_prog.c
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
// Copyright (c) 2024 Google LLC

pub const REWRITE_ADDRESS_IP4: c_uint = 0xc0a801fe // 192.168.1.254;
pub const REWRITE_ADDRESS_PORT4: c_int = 4040;
    SEC("cgroup/getsockname4")
#[no_mangle]
pub unsafe extern "C" fn getsockname_v4_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int getsockname_v4_prog(struct bpf_sock_addr *ctx)
    {
    ctx.user_ip4 = bpf_htonl(REWRITE_ADDRESS_IP4);
    ctx.user_port = bpf_htons(REWRITE_ADDRESS_PORT4);
    return 1;
    }
    char _license[] SEC("license") = "GPL";
