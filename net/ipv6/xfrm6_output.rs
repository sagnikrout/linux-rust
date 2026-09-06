//! Automatically rewritten from C to Rust
//! Source: net/ipv6/xfrm6_output.c
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
// xfrm6_output.c - Common IPsec encapsulation code for IPv6.
// Copyright (C) 2002 USAGI/WIDE Project
// Copyright (c) 2004 Herbert Xu <herbert@gondor.apana.org.au>
//

#[no_mangle]
pub unsafe extern "C" fn xfrm6_local_rxpmtu(skb: *mut sk_buff, mtu: u32) {
    void xfrm6_local_rxpmtu(struct sk_buff *skb, u32 mtu)
    {
    struct flowi6 fl6;
    struct sock *sk = skb.sk;
    fl6.flowi6_oif = sk.sk_bound_dev_if;
    fl6.daddr = ipv6_hdr(skb).daddr;
    ipv6_local_rxpmtu(sk, &fl6, mtu);
    }
#[no_mangle]
pub unsafe extern "C" fn xfrm6_local_error(skb: *mut sk_buff, mtu: u32) {
    void xfrm6_local_error(struct sk_buff *skb, u32 mtu)
    {
    struct flowi6 fl6;
    const struct ipv6hdr *hdr;
    struct sock *sk = skb.sk;
    hdr = skb.encapsulation ? inner_ipv6_hdr(skb) : ipv6_hdr(skb);
    fl6.fl6_dport = inet_sk(sk).inet_dport;
    fl6.daddr = hdr.daddr;
    ipv6_local_error(sk, EMSGSIZE, &fl6, mtu);
    }
#[no_mangle]
unsafe extern "C" fn __xfrm6_output_finish(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    static int __xfrm6_output_finish(struct net *net, struct sock *sk, struct sk_buff *skb)
    {
    return xfrm_output(sk, skb);
    }
#[no_mangle]
unsafe extern "C" fn xfrm6_noneed_fragment(skb: *mut sk_buff) -> c_int {
    static int xfrm6_noneed_fragment(struct sk_buff *skb)
    {
    struct frag_hdr *fh;
    let mut prevhdr: u8 = ipv6_hdr(skb).nexthdr;
    if (prevhdr != NEXTHDR_FRAGMENT)
    return 0;
    fh = (struct frag_hdr *)(skb.data + sizeof(struct ipv6hdr));
    if (fh.nexthdr == NEXTHDR_ESP || fh.nexthdr == NEXTHDR_AUTH)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __xfrm6_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    static int __xfrm6_output(struct net *net, struct sock *sk, struct sk_buff *skb)
    {
    struct dst_entry *dst = skb_dst(skb);
    struct xfrm_state *x = dst.xfrm;
    unsigned int mtu;
    bool toobig;

    if (!x) {
    IP6CB(skb).flags |= IP6SKB_REROUTED;
    return dst_output(net, sk, skb);
    }

    if (x.props.mode != XFRM_MODE_TUNNEL)
    goto skip_frag;
    if (skb.protocol == htons(ETH_P_IPV6))
    mtu = ip6_skb_dst_mtu(skb);
    else
    mtu = dst_mtu(skb_dst(skb));
    toobig = skb.len > mtu && !skb_is_gso(skb);
    if (toobig && xfrm6_local_dontfrag(sk)) {
    xfrm6_local_rxpmtu(skb, mtu);
    kfree_skb(skb);
    return -EMSGSIZE;
    } else if (toobig && xfrm6_noneed_fragment(skb)) {
    skb.ignore_df = 1;
    goto skip_frag;
    } else if (!skb.ignore_df && toobig && sk) {
    xfrm_local_error(skb, mtu);
    kfree_skb(skb);
    return -EMSGSIZE;
    }
    if (toobig)
    return ip6_fragment(net, sk, skb,
    __xfrm6_output_finish);
    skip_frag:
    return xfrm_output(sk, skb);
    }
#[no_mangle]
pub unsafe extern "C" fn xfrm6_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    int xfrm6_output(struct net *net, struct sock *sk, struct sk_buff *skb)
    {
    return NF_HOOK_COND(NFPROTO_IPV6, NF_INET_POST_ROUTING,
    net, sk, skb,  skb.dev, skb_dst_dev(skb),
    __xfrm6_output,
    !(IP6CB(skb).flags & IP6SKB_REROUTED));
    }
