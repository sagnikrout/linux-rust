//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/vrf_socket_lookup.c
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

    int lookup_status;
    bool test_xdp;
    bool tcp_skc;

#[no_mangle]
unsafe extern "C" fn socket_lookup(ctx: *mut c_void, data_end: *mut c_void, data: *mut c_void) {
    static void socket_lookup(void *ctx, void *data_end, void *data)
    {
    struct ethhdr *eth = data;
    struct bpf_sock_tuple *tp;
    struct bpf_sock *sk;
    struct iphdr *iph;
    int tplen;
    if (eth + 1 > data_end)
    return;
    if (eth.h_proto != bpf_htons(ETH_P_IP))
    return;
    iph = (struct iphdr *)(eth + 1);
    if (iph + 1 > data_end)
    return;
    tp = (struct bpf_sock_tuple *)&iph.saddr;
    tplen = sizeof(tp.ipv4);
    if ((void *)tp + tplen > data_end)
    return;
    switch (iph.protocol) {
    case IPPROTO_TCP:
    if (tcp_skc)
    sk = bpf_skc_lookup_tcp(ctx, tp, tplen, CUR_NS, 0);
    else
    sk = bpf_sk_lookup_tcp(ctx, tp, tplen, CUR_NS, 0);
    break;
    case IPPROTO_UDP:
    sk = bpf_sk_lookup_udp(ctx, tp, tplen, CUR_NS, 0);
    break;
    default:
    return;
    }
    lookup_status = 0;
    if (sk) {
    bpf_sk_release(sk);
    lookup_status = 1;
    }
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_socket_lookup(skb: *mut __sk_buff) -> c_int {
    int tc_socket_lookup(struct __sk_buff *skb)
    {
    void *data_end = (void *)(long)skb.data_end;
    void *data = (void *)(long)skb.data;
    if (test_xdp)
    return TC_ACT_UNSPEC;
    socket_lookup(skb, data_end, data);
    return TC_ACT_UNSPEC;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_socket_lookup(xdp: *mut xdp_md) -> c_int {
    int xdp_socket_lookup(struct xdp_md *xdp)
    {
    void *data_end = (void *)(long)xdp.data_end;
    void *data = (void *)(long)xdp.data;
    if (!test_xdp)
    return XDP_PASS;
    socket_lookup(xdp, data_end, data);
    return XDP_PASS;
    }
    char _license[] SEC("license") = "GPL";
