//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/recvmsg6_prog.c
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

pub const SERV6_IP_0: c_uint = 0xfaceb00c /* face:b00c:1234:5678::abcd */;
pub const SERV6_IP_1: c_uint = 0x12345678;
pub const SERV6_IP_2: c_uint = 0x00000000;
pub const SERV6_IP_3: c_uint = 0x0000abcd;
pub const SERV6_PORT: c_int = 6060;
    SEC("cgroup/recvmsg6")
#[no_mangle]
pub unsafe extern "C" fn recvmsg6_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int recvmsg6_prog(struct bpf_sock_addr *ctx)
    {
    struct bpf_sock *sk;
    sk = ctx.sk;
    if (!sk)
    return 1;
    if (sk.family != AF_INET6)
    return 1;
    if (ctx.type != SOCK_STREAM && ctx.type != SOCK_DGRAM)
    return 1;
    if (!get_set_sk_priority(ctx))
    return 1;
    ctx.user_ip6[0] = bpf_htonl(SERV6_IP_0);
    ctx.user_ip6[1] = bpf_htonl(SERV6_IP_1);
    ctx.user_ip6[2] = bpf_htonl(SERV6_IP_2);
    ctx.user_ip6[3] = bpf_htonl(SERV6_IP_3);
    ctx.user_port = bpf_htons(SERV6_PORT);
    return 1;
    }
    char _license[] SEC("license") = "GPL";
