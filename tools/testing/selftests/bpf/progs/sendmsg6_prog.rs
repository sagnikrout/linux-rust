//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sendmsg6_prog.c
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

pub const SRC_REWRITE_IP6_0: c_int = 0;
pub const SRC_REWRITE_IP6_1: c_int = 0;
pub const SRC_REWRITE_IP6_2: c_int = 0;
pub const SRC_REWRITE_IP6_3: c_int = 6;
pub const DST_REWRITE_IP6_0: c_int = 0;
pub const DST_REWRITE_IP6_1: c_int = 0;
pub const DST_REWRITE_IP6_2: c_int = 0;
pub const DST_REWRITE_IP6_3: c_int = 1;
pub const DST_REWRITE_IP6_V4_MAPPED_0: c_int = 0;
pub const DST_REWRITE_IP6_V4_MAPPED_1: c_int = 0;
pub const DST_REWRITE_IP6_V4_MAPPED_2: c_uint = 0x0000FFFF;
pub const DST_REWRITE_IP6_V4_MAPPED_3: c_uint = 0xc0a80004 // 192.168.0.4;
pub const DST_REWRITE_PORT6: c_int = 6666;
    SEC("cgroup/sendmsg6")
#[no_mangle]
pub unsafe extern "C" fn sendmsg_v6_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg_v6_prog(struct bpf_sock_addr *ctx)
    {
    if (ctx.type != SOCK_DGRAM)
    return 0;
    if (!get_set_sk_priority(ctx))
    return 0;
// Rewrite source.
    if (ctx.msg_src_ip6[3] == bpf_htonl(1) ||
    ctx.msg_src_ip6[3] == bpf_htonl(0)) {
    ctx.msg_src_ip6[0] = bpf_htonl(SRC_REWRITE_IP6_0);
    ctx.msg_src_ip6[1] = bpf_htonl(SRC_REWRITE_IP6_1);
    ctx.msg_src_ip6[2] = bpf_htonl(SRC_REWRITE_IP6_2);
    ctx.msg_src_ip6[3] = bpf_htonl(SRC_REWRITE_IP6_3);
    } else {
// Unexpected source. Reject sendmsg.
    return 0;
    }
// Rewrite destination.
    if (ctx.user_ip6[0] == bpf_htonl(0xFACEB00C)) {
    ctx.user_ip6[0] = bpf_htonl(DST_REWRITE_IP6_0);
    ctx.user_ip6[1] = bpf_htonl(DST_REWRITE_IP6_1);
    ctx.user_ip6[2] = bpf_htonl(DST_REWRITE_IP6_2);
    ctx.user_ip6[3] = bpf_htonl(DST_REWRITE_IP6_3);
    ctx.user_port = bpf_htons(DST_REWRITE_PORT6);
    } else {
// Unexpected destination. Reject sendmsg.
    return 0;
    }
    return 1;
    }
    SEC("cgroup/sendmsg6")
#[no_mangle]
pub unsafe extern "C" fn sendmsg_v6_v4mapped_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg_v6_v4mapped_prog(struct bpf_sock_addr *ctx)
    {
// Rewrite source.
    ctx.msg_src_ip6[0] = bpf_htonl(SRC_REWRITE_IP6_0);
    ctx.msg_src_ip6[1] = bpf_htonl(SRC_REWRITE_IP6_1);
    ctx.msg_src_ip6[2] = bpf_htonl(SRC_REWRITE_IP6_2);
    ctx.msg_src_ip6[3] = bpf_htonl(SRC_REWRITE_IP6_3);
// Rewrite destination.
    ctx.user_ip6[0] = bpf_htonl(DST_REWRITE_IP6_V4_MAPPED_0);
    ctx.user_ip6[1] = bpf_htonl(DST_REWRITE_IP6_V4_MAPPED_1);
    ctx.user_ip6[2] = bpf_htonl(DST_REWRITE_IP6_V4_MAPPED_2);
    ctx.user_ip6[3] = bpf_htonl(DST_REWRITE_IP6_V4_MAPPED_3);
    ctx.user_port = bpf_htons(DST_REWRITE_PORT6);
    return 1;
    }
    SEC("cgroup/sendmsg6")
#[no_mangle]
pub unsafe extern "C" fn sendmsg_v6_wildcard_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg_v6_wildcard_prog(struct bpf_sock_addr *ctx)
    {
// Rewrite source.
    ctx.msg_src_ip6[0] = bpf_htonl(SRC_REWRITE_IP6_0);
    ctx.msg_src_ip6[1] = bpf_htonl(SRC_REWRITE_IP6_1);
    ctx.msg_src_ip6[2] = bpf_htonl(SRC_REWRITE_IP6_2);
    ctx.msg_src_ip6[3] = bpf_htonl(SRC_REWRITE_IP6_3);
// Rewrite destination.
    ctx.user_ip6[0] = bpf_htonl(0);
    ctx.user_ip6[1] = bpf_htonl(0);
    ctx.user_ip6[2] = bpf_htonl(0);
    ctx.user_ip6[3] = bpf_htonl(0);
    ctx.user_port = bpf_htons(DST_REWRITE_PORT6);
    return 1;
    }
    SEC("cgroup/sendmsg6")
#[no_mangle]
pub unsafe extern "C" fn sendmsg_v6_preserve_dst_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg_v6_preserve_dst_prog(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/sendmsg6")
#[no_mangle]
pub unsafe extern "C" fn sendmsg_v6_deny_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg_v6_deny_prog(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
