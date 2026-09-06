//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/recvmsg4_prog.c
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

pub const SERV4_IP: c_uint = 0xc0a801feU /* 192.168.1.254 */;
pub const SERV4_PORT: c_int = 4040;
    SEC("cgroup/recvmsg4")
#[no_mangle]
pub unsafe extern "C" fn recvmsg4_prog(ctx: *mut bpf_sock_addr) -> c_int {
    int recvmsg4_prog(struct bpf_sock_addr *ctx)
    {
    struct bpf_sock *sk;
    sk = ctx.sk;
    if (!sk)
    return 1;
    if (sk.family != AF_INET)
    return 1;
    if (ctx.type != SOCK_STREAM && ctx.type != SOCK_DGRAM)
    return 1;
    if (!get_set_sk_priority(ctx))
    return 1;
    ctx.user_ip4 = bpf_htonl(SERV4_IP);
    ctx.user_port = bpf_htons(SERV4_PORT);
    return 1;
    }
    char _license[] SEC("license") = "GPL";
