//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sendmsg4_prog.c
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
// Copyright (c) 2018 Facebook

pub const SRC1_IP4: c_uint = 0xAC100001U /* 172.16.0.1 */;
pub const SRC2_IP4: c_uint = 0x00000000U;
pub const SRC_REWRITE_IP4: c_uint = 0x7f000004U;
pub const DST_IP4: c_uint = 0xC0A801FEU /* 192.168.1.254 */;
pub const DST_REWRITE_IP4: c_uint = 0x7f000001U;
pub const DST_PORT: c_int = 4040;
pub const DST_REWRITE_PORT4: c_int = 4444;
    SEC("cgroup/sendmsg4")
#[no_mangle]
pub unsafe extern "C" fn sendmsg_v4_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg_v4_prog(struct bpf_sock_addr *ctx)
    {
    if (ctx.type != SOCK_DGRAM)
    return 0;
    if (!get_set_sk_priority(ctx))
    return 0;
// Rewrite source.
    if (ctx.msg_src_ip4 == bpf_htonl(SRC1_IP4) ||
    ctx.msg_src_ip4 == bpf_htonl(SRC2_IP4)) {
    ctx.msg_src_ip4 = bpf_htonl(SRC_REWRITE_IP4);
    } else {
// Unexpected source. Reject sendmsg.
    return 0;
    }
// Rewrite destination.
    if ((ctx.user_ip4 >> 24) == (bpf_htonl(DST_IP4) >> 24) &&
    ctx.user_port == bpf_htons(DST_PORT)) {
    ctx.user_ip4 = bpf_htonl(DST_REWRITE_IP4);
    ctx.user_port = bpf_htons(DST_REWRITE_PORT4);
    } else {
// Unexpected source. Reject sendmsg.
    return 0;
    }
    return 1;
    }
    SEC("cgroup/sendmsg4")
#[no_mangle]
pub unsafe extern "C" fn sendmsg_v4_deny_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg_v4_deny_prog(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
