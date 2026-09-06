//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/connect_force_port6.c
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

    char _license[] SEC("license") = "GPL";
    let mut port: __u16 = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_addr {
    pub addr: [__be32; 4],
    pub port: __be16,
}

    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct svc_addr);
    } service_mapping SEC(".maps");
    SEC("cgroup/connect6")
#[no_mangle]
pub unsafe extern "C" fn connect6(ctx: *mut bpf_sock_addr) -> c_int {
    int connect6(struct bpf_sock_addr *ctx)
    {
    let mut sa: sockaddr_in6 = {};
    struct svc_addr *orig;
// Force local address to [::1]:22223.
    sa.sin6_family = AF_INET6;
    sa.sin6_port = bpf_htons(22223);
    sa.sin6_addr.s6_addr32[3] = bpf_htonl(1);
    if (bpf_bind(ctx, (struct sockaddr *)&sa, sizeof(sa)) != 0)
    return 0;
// Rewire service [fc00::1]:60000 to backend [::1]:port.
    if (ctx.user_port == bpf_htons(60000)) {
    orig = bpf_sk_storage_get(&service_mapping, ctx.sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    if (!orig)
    return 0;
    orig.addr[0] = ctx.user_ip6[0];
    orig.addr[1] = ctx.user_ip6[1];
    orig.addr[2] = ctx.user_ip6[2];
    orig.addr[3] = ctx.user_ip6[3];
    orig.port = ctx.user_port;
    ctx.user_ip6[0] = 0;
    ctx.user_ip6[1] = 0;
    ctx.user_ip6[2] = 0;
    ctx.user_ip6[3] = bpf_htonl(1);
    ctx.user_port = bpf_htons(port);
    }
    return 1;
    }
    SEC("cgroup/getsockname6")
#[no_mangle]
pub unsafe extern "C" fn getsockname6(ctx: *mut bpf_sock_addr) -> c_int {
    int getsockname6(struct bpf_sock_addr *ctx)
    {
    if (!get_set_sk_priority(ctx))
    return 1;
// Expose local server as [fc00::1]:60000 to client.
    if (ctx.user_port == bpf_htons(port)) {
    ctx.user_ip6[0] = bpf_htonl(0xfc000000);
    ctx.user_ip6[1] = 0;
    ctx.user_ip6[2] = 0;
    ctx.user_ip6[3] = bpf_htonl(1);
    ctx.user_port = bpf_htons(60000);
    }
    return 1;
    }
    SEC("cgroup/getpeername6")
#[no_mangle]
pub unsafe extern "C" fn getpeername6(ctx: *mut bpf_sock_addr) -> c_int {
    int getpeername6(struct bpf_sock_addr *ctx)
    {
    struct svc_addr *orig;
    if (!get_set_sk_priority(ctx))
    return 1;
// Expose service [fc00::1]:60000 as peer instead of backend.
    if (ctx.user_port == bpf_htons(port)) {
    orig = bpf_sk_storage_get(&service_mapping, ctx.sk, 0, 0);
    if (orig) {
    ctx.user_ip6[0] = orig.addr[0];
    ctx.user_ip6[1] = orig.addr[1];
    ctx.user_ip6[2] = orig.addr[2];
    ctx.user_ip6[3] = orig.addr[3];
    ctx.user_port = orig.port;
    }
    }
    return 1;
    }
