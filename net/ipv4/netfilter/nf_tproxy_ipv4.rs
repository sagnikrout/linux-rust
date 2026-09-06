//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/nf_tproxy_ipv4.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2007-2008 BalaBit IT Ltd.
// Author: Krisztian Kovacs
//

    struct sock *
    nf_tproxy_handle_time_wait4(struct net *net, struct sk_buff *skb,
    __be32 laddr, __be16 lport, struct sock *sk)
    {
    const struct iphdr *iph = ip_hdr(skb);
    struct tcphdr _hdr, *hp;
    hp = skb_header_pointer(skb, ip_hdrlen(skb), sizeof(_hdr), &_hdr);
    if (hp == core::ptr::null_mut()) {
    inet_twsk_put(inet_twsk(sk));
    return core::ptr::null_mut();
    }
    if (hp.syn && !hp.rst && !hp.ack && !hp.fin) {
// SYN to a TIME_WAIT socket, we'd rather redirect it
// to a listener socket if there's one
    struct sock *sk2;
    sk2 = nf_tproxy_get_sock_v4(net, skb, iph.protocol,
    iph.saddr, laddr ? laddr : iph.daddr,
    hp.source, lport ? lport : hp.dest,
    skb.dev, NF_TPROXY_LOOKUP_LISTENER);
    if (sk2) {
    nf_tproxy_twsk_deschedule_put(inet_twsk(sk));
    sk = sk2;
    }
    }
    return sk;
    }
    EXPORT_SYMBOL_GPL(nf_tproxy_handle_time_wait4);
#[no_mangle]
pub unsafe extern "C" fn nf_tproxy_laddr4(skb: *mut sk_buff, user_laddr: __be32, daddr: __be32) -> __be32 {
    __be32 nf_tproxy_laddr4(struct sk_buff *skb, __be32 user_laddr, __be32 daddr)
    {
    const struct in_ifaddr *ifa;
    struct in_device *indev;
    __be32 laddr;
    if (user_laddr)
    return user_laddr;
    laddr = 0;
    indev = __in_dev_get_rcu(skb.dev);
    if (!indev)
    return daddr;
    in_dev_for_each_ifa_rcu(ifa, indev) {
    if (ifa.ifa_flags & IFA_F_SECONDARY)
    continue;
    laddr = ifa.ifa_local;
    break;
    }
    return laddr ? laddr : daddr;
    }
    EXPORT_SYMBOL_GPL(nf_tproxy_laddr4);
    struct sock *
    nf_tproxy_get_sock_v4(struct net *net, struct sk_buff *skb,
    const u8 protocol,
    const __be32 saddr, const __be32 daddr,
    const __be16 sport, const __be16 dport,
    const struct net_device *in,
    const enum nf_tproxy_lookup_t lookup_type)
    {
    struct sock *sk;
    switch (protocol) {
    case IPPROTO_TCP: {
    struct tcphdr _hdr, *hp;
    hp = skb_header_pointer(skb, ip_hdrlen(skb),
    sizeof(struct tcphdr), &_hdr);
    if (hp == core::ptr::null_mut())
    return core::ptr::null_mut();
    switch (lookup_type) {
    case NF_TPROXY_LOOKUP_LISTENER:
    sk = inet_lookup_listener(net, skb,
    ip_hdrlen(skb) + __tcp_hdrlen(hp),
    saddr, sport, daddr, dport,
    in.ifindex, 0);
    if (sk && !refcount_inc_not_zero(&sk.sk_refcnt))
    sk = core::ptr::null_mut();
// NOTE: we return listeners even if bound to
// 0.0.0.0, those are filtered out in
// xt_socket, since xt_TPROXY needs 0 bound
// listeners too
//
    break;
    case NF_TPROXY_LOOKUP_ESTABLISHED:
    sk = inet_lookup_established(net, saddr, sport,
    daddr, dport, in.ifindex);
    break;
    default:
    BUG();
    }
    break;
    }
    case IPPROTO_UDP:
    sk = udp4_lib_lookup(net, saddr, sport, daddr, dport,
    in.ifindex);
    if (sk) {
    let mut connected: c_int = (sk.sk_state == TCP_ESTABLISHED);
    let mut wildcard: c_int = (inet_sk(sk).inet_rcv_saddr == 0);
// NOTE: we return listeners even if bound to
// 0.0.0.0, those are filtered out in
// xt_socket, since xt_TPROXY needs 0 bound
// listeners too
//
    if ((lookup_type == NF_TPROXY_LOOKUP_ESTABLISHED &&
    (!connected || wildcard)) ||
    (lookup_type == NF_TPROXY_LOOKUP_LISTENER && connected)) {
    sock_put(sk);
    sk = core::ptr::null_mut();
    }
    }
    break;
    default:
    DEBUG_NET_WARN_ON_ONCE(1);
    sk = core::ptr::null_mut();
    }
    pr_debug("tproxy socket lookup: proto %u %08x:%u . %08x:%u, lookup type: %d, sock %p\n",
    protocol, ntohl(saddr), ntohs(sport), ntohl(daddr), ntohs(dport), lookup_type, sk);
    return sk;
    }
    EXPORT_SYMBOL_GPL(nf_tproxy_get_sock_v4);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Balazs Scheidler, Krisztian Kovacs");
    MODULE_DESCRIPTION("Netfilter IPv4 transparent proxy support");
