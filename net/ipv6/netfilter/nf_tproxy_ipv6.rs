//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/nf_tproxy_ipv6.c
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

    const struct in6_addr *
    nf_tproxy_laddr6(struct sk_buff *skb, const struct in6_addr *user_laddr,
    const struct in6_addr *daddr)
    {
    struct inet6_dev *indev;
    struct inet6_ifaddr *ifa;
    struct in6_addr *laddr;
    if (!ipv6_addr_any(user_laddr))
    return user_laddr;
    laddr = core::ptr::null_mut();
    indev = __in6_dev_get(skb.dev);
    if (indev) {
    read_lock_bh(&indev.lock);
    list_for_each_entry(ifa, &indev.addr_list, if_list) {
    if (ifa.flags & (IFA_F_TENTATIVE | IFA_F_DEPRECATED))
    continue;
    laddr = &ifa.addr;
    break;
    }
    read_unlock_bh(&indev.lock);
    }
    return laddr ? laddr : daddr;
    }
    EXPORT_SYMBOL_GPL(nf_tproxy_laddr6);
    struct sock *
    nf_tproxy_handle_time_wait6(struct sk_buff *skb, int tproto, int thoff,
    struct net *net,
    const struct in6_addr *laddr,
    const __be16 lport,
    struct sock *sk)
    {
    const struct ipv6hdr *iph = ipv6_hdr(skb);
    struct tcphdr _hdr, *hp;
    hp = skb_header_pointer(skb, thoff, sizeof(_hdr), &_hdr);
    if (hp == core::ptr::null_mut()) {
    inet_twsk_put(inet_twsk(sk));
    return core::ptr::null_mut();
    }
    if (hp.syn && !hp.rst && !hp.ack && !hp.fin) {
// SYN to a TIME_WAIT socket, we'd rather redirect it
// to a listener socket if there's one
    struct sock *sk2;
    sk2 = nf_tproxy_get_sock_v6(net, skb, thoff, tproto,
    &iph.saddr,
    nf_tproxy_laddr6(skb, laddr, &iph.daddr),
    hp.source,
    lport ? lport : hp.dest,
    skb.dev, NF_TPROXY_LOOKUP_LISTENER);
    if (sk2) {
    nf_tproxy_twsk_deschedule_put(inet_twsk(sk));
    sk = sk2;
    }
    }
    return sk;
    }
    EXPORT_SYMBOL_GPL(nf_tproxy_handle_time_wait6);
    struct sock *
    nf_tproxy_get_sock_v6(struct net *net, struct sk_buff *skb, int thoff,
    const u8 protocol,
    const struct in6_addr *saddr, const struct in6_addr *daddr,
    const __be16 sport, const __be16 dport,
    const struct net_device *in,
    const enum nf_tproxy_lookup_t lookup_type)
    {
    struct sock *sk;
    switch (protocol) {
    case IPPROTO_TCP: {
    struct tcphdr _hdr, *hp;
    hp = skb_header_pointer(skb, thoff,
    sizeof(struct tcphdr), &_hdr);
    if (hp == core::ptr::null_mut())
    return core::ptr::null_mut();
    switch (lookup_type) {
    case NF_TPROXY_LOOKUP_LISTENER:
    sk = inet6_lookup_listener(net, skb,
    thoff + __tcp_hdrlen(hp),
    saddr, sport,
    daddr, ntohs(dport),
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
    sk = __inet6_lookup_established(net, saddr, sport, daddr,
    ntohs(dport), in.ifindex, 0);
    break;
    default:
    BUG();
    }
    break;
    }
    case IPPROTO_UDP:
    sk = udp6_lib_lookup(net, saddr, sport, daddr, dport,
    in.ifindex);
    if (sk) {
    let mut connected: c_int = (sk.sk_state == TCP_ESTABLISHED);
    let mut wildcard: c_int = ipv6_addr_any(&sk.sk_v6_rcv_saddr);
// NOTE: we return listeners even if bound to
// 0.0.0.0, those are filtered out in
// xt_socket, since xt_TPROXY needs 0 bound
// listeners too
//
    if ((lookup_type == NF_TPROXY_LOOKUP_ESTABLISHED && (!connected || wildcard)) ||
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
    pr_debug("tproxy socket lookup: proto %u %pI6:%u . %pI6:%u, lookup type: %d, sock %p\n",
    protocol, saddr, ntohs(sport), daddr, ntohs(dport), lookup_type, sk);
    return sk;
    }
    EXPORT_SYMBOL_GPL(nf_tproxy_get_sock_v6);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Balazs Scheidler, Krisztian Kovacs");
    MODULE_DESCRIPTION("Netfilter IPv6 transparent proxy support");
