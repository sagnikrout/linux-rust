//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/netns_cookie_prog.c
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

pub const AF_INET6: c_int = 10;
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, int);
    } sockops_netns_cookies SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, int);
    } sk_msg_netns_cookies SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 2);
    __type(key, __u32);
    __type(value, __u64);
    } sock_map SEC(".maps");
    int tcx_init_netns_cookie, tcx_netns_cookie;
    int cgroup_skb_init_netns_cookie, cgroup_skb_netns_cookie;
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn get_netns_cookie_sockops(ctx: *mut bpf_sock_ops) -> c_int {
    int get_netns_cookie_sockops(struct bpf_sock_ops *ctx)
    {
    struct bpf_sock *sk = ctx.sk;
    int *cookie;
    let mut key: __u32 = 0;
    if (ctx.family != AF_INET6)
    return 1;
    if (!sk)
    return 1;
    switch (ctx.op) {
    case BPF_SOCK_OPS_TCP_CONNECT_CB:
    cookie = bpf_sk_storage_get(&sockops_netns_cookies, sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    if (!cookie)
    return 1;
// cookie = bpf_get_netns_cookie(ctx);
    break;
    case BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB:
    bpf_sock_map_update(ctx, &sock_map, &key, BPF_NOEXIST);
    break;
    default:
    break;
    }
    return 1;
    }
    SEC("sk_msg")
#[no_mangle]
pub unsafe extern "C" fn get_netns_cookie_sk_msg(msg: *mut sk_msg_md) -> c_int {
    int get_netns_cookie_sk_msg(struct sk_msg_md *msg)
    {
    struct bpf_sock *sk = msg.sk;
    int *cookie;
    if (msg.family != AF_INET6)
    return 1;
    if (!sk)
    return 1;
    cookie = bpf_sk_storage_get(&sk_msg_netns_cookies, sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    if (!cookie)
    return 1;
// cookie = bpf_get_netns_cookie(msg);
    return 1;
    }
    SEC("tcx/ingress")
#[no_mangle]
pub unsafe extern "C" fn get_netns_cookie_tcx(skb: *mut __sk_buff) -> c_int {
    int get_netns_cookie_tcx(struct __sk_buff *skb)
    {
    tcx_init_netns_cookie = bpf_get_netns_cookie(core::ptr::null_mut());
    tcx_netns_cookie = bpf_get_netns_cookie(skb);
    return TCX_PASS;
    }
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn get_netns_cookie_cgroup_skb(skb: *mut __sk_buff) -> c_int {
    int get_netns_cookie_cgroup_skb(struct __sk_buff *skb)
    {
    cgroup_skb_init_netns_cookie = bpf_get_netns_cookie(core::ptr::null_mut());
    cgroup_skb_netns_cookie = bpf_get_netns_cookie(skb);
    return SK_PASS;
    }
    char _license[] SEC("license") = "GPL";
