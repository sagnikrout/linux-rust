//! Automatically rewritten from C to Rust
//! Source: net/ipv6/tcpv6_offload.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// IPV6 GSO/GRO offload support
// Linux INET6 implementation
//
// TCPv6 GSO/GRO support
//

    static void tcp6_check_fraglist_gro(struct list_head *head, struct sk_buff *skb,
    struct tcphdr *th)
    {

    const struct ipv6hdr *hdr;
    struct sk_buff *p;
    struct sock *sk;
    struct net *net;
    int iif, sdif;
    p = tcp_gro_lookup(head, th);
    if (p) {
// flist GRO applies to consecutive non-GSO skbs
    if (!skb_is_gso(skb) || !NAPI_GRO_CB(p).is_flist) {
    NAPI_GRO_CB(skb).is_flist = NAPI_GRO_CB(p).is_flist;
    return;
    }
// Fall back to the regular GRO path
    if (NAPI_GRO_CB(p).count == 1)
    NAPI_GRO_CB(p).is_flist = 0;
    NAPI_GRO_CB(skb).is_flist = 0;
    return;
    }
    inet6_get_iif_sdif(skb, &iif, &sdif);
    hdr = skb_gro_network_header(skb);
    net = dev_net_rcu(skb.dev);
    sk = __inet6_lookup_established(net, &hdr.saddr, th.source,
    &hdr.daddr, ntohs(th.dest),
    iif, sdif);
    NAPI_GRO_CB(skb).is_flist = !sk && !skb_is_gso(skb);
    if (sk)
    sock_gen_put(sk);

    }
    static __always_inline struct sk_buff *tcp6_gro_receive(struct list_head *head,
    struct sk_buff *skb)
    {
    struct tcphdr *th;
// Don't bother verifying checksum if we're going to flush anyway.
    if (!NAPI_GRO_CB(skb).flush &&
    skb_gro_checksum_validate(skb, IPPROTO_TCP,
    ip6_gro_compute_pseudo))
    goto flush;
    th = tcp_gro_pull_header(skb);
    if (!th)
    goto flush;
    if (unlikely(skb.dev.features & NETIF_F_GRO_FRAGLIST))
    tcp6_check_fraglist_gro(head, skb, th);
    return tcp_gro_receive(head, skb, th);
    flush:
    NAPI_GRO_CB(skb).flush = 1;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn tcp6_gro_complete(skb: *mut sk_buff, thoff: c_int) -> __always_inline int {
    static __always_inline int tcp6_gro_complete(struct sk_buff *skb, int thoff)
    {
    let mut offset: u16 = NAPI_GRO_CB(skb).network_offsets[skb.encapsulation];
    const struct ipv6hdr *iph = (struct ipv6hdr *)(skb.data + offset);
    struct tcphdr *th = tcp_hdr(skb);
    if (unlikely(NAPI_GRO_CB(skb).is_flist)) {
    skb_shinfo(skb).gso_type |= SKB_GSO_FRAGLIST | SKB_GSO_TCPV6;
    skb_shinfo(skb).gso_segs = NAPI_GRO_CB(skb).count;
    __skb_incr_checksum_unnecessary(skb);
    return 0;
    }
    th.check = ~tcp_v6_check(skb.len - thoff, &iph.saddr,
    &iph.daddr, 0);
    skb_shinfo(skb).gso_type |= SKB_GSO_TCPV6;
    tcp_gro_complete(skb);
    return 0;
    }
    static void __tcpv6_gso_segment_csum(struct sk_buff *seg,
    struct in6_addr *oldip,
    const struct in6_addr *newip,
    __be16 *oldport, __be16 newport)
    {
    struct tcphdr *th = tcp_hdr(seg);
    if (!ipv6_addr_equal(oldip, newip)) {
    inet_proto_csum_replace16(&th.check, seg,
    oldip.s6_addr32,
    newip.s6_addr32,
    true);
// oldip = *newip;
    }
    if (*oldport == newport)
    return;
    inet_proto_csum_replace2(&th.check, seg, *oldport, newport, false);
// oldport = newport;
    }
    static struct sk_buff *__tcpv6_gso_segment_list_csum(struct sk_buff *segs)
    {
    const struct tcphdr *th;
    const struct ipv6hdr *iph;
    struct sk_buff *seg;
    struct tcphdr *th2;
    struct ipv6hdr *iph2;
    seg = segs;
    th = tcp_hdr(seg);
    iph = ipv6_hdr(seg);
    th2 = tcp_hdr(seg.next);
    iph2 = ipv6_hdr(seg.next);
    if (!(*(const u32 *)&th.source ^ *(const u32 *)&th2.source) &&
    ipv6_addr_equal(&iph.saddr, &iph2.saddr) &&
    ipv6_addr_equal(&iph.daddr, &iph2.daddr))
    return segs;
    while ((seg = seg.next)) {
    th2 = tcp_hdr(seg);
    iph2 = ipv6_hdr(seg);
    __tcpv6_gso_segment_csum(seg, &iph2.saddr, &iph.saddr,
    &th2.source, th.source);
    __tcpv6_gso_segment_csum(seg, &iph2.daddr, &iph.daddr,
    &th2.dest, th.dest);
    }
    return segs;
    }
    static struct sk_buff *__tcp6_gso_segment_list(struct sk_buff *skb,
    netdev_features_t features)
    {
    skb = skb_segment_list(skb, features, skb_mac_header_len(skb));
    if (IS_ERR(skb))
    return skb;
    return __tcpv6_gso_segment_list_csum(skb);
    }
    static struct sk_buff *tcp6_gso_segment(struct sk_buff *skb,
    netdev_features_t features)
    {
    struct tcphdr *th;
    if (!(skb_shinfo(skb).gso_type & SKB_GSO_TCPV6))
    return ERR_PTR(-EINVAL);
    if (!pskb_may_pull(skb, sizeof(*th)))
    return ERR_PTR(-EINVAL);
    if (skb_shinfo(skb).gso_type & SKB_GSO_FRAGLIST) {
    struct tcphdr *th = tcp_hdr(skb);
    if ((skb_pagelen(skb) - th.doff * 4 == skb_shinfo(skb).gso_size) &&
    !(skb_shinfo(skb).gso_type & SKB_GSO_DODGY))
    return __tcp6_gso_segment_list(skb, features);
    skb.ip_summed = CHECKSUM_NONE;
    }
    if (unlikely(skb.ip_summed != CHECKSUM_PARTIAL)) {
    const struct ipv6hdr *ipv6h = ipv6_hdr(skb);
    struct tcphdr *th = tcp_hdr(skb);
// Set up pseudo header, usually expect stack to have done
// this.
//
    th.check = 0;
    skb.ip_summed = CHECKSUM_PARTIAL;
    __tcp_v6_send_check(skb, &ipv6h.saddr, &ipv6h.daddr);
    }
    return tcp_gso_segment(skb, features);
    }
#[no_mangle]
pub unsafe extern "C" fn tcpv6_offload_init() -> int __init {
    int __init tcpv6_offload_init(void)
    {
    net_hotdata.tcpv6_offload = (struct net_offload) {
    .callbacks = {
    .gso_segment	=	tcp6_gso_segment,
    .gro_receive	=	tcp6_gro_receive,
    .gro_complete	=	tcp6_gro_complete,
    },
    };
    return inet6_add_offload(&net_hotdata.tcpv6_offload, IPPROTO_TCP);
    }
