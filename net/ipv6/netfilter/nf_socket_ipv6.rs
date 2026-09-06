//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/nf_socket_ipv6.c
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

    static int
    extract_icmp6_fields(const struct sk_buff *skb,
    unsigned int outside_hdrlen,
    int *protocol,
    const struct in6_addr **raddr,
    const struct in6_addr **laddr,
    __be16 *rport,
    __be16 *lport,
    struct ipv6hdr *ipv6_var)
    {
    const struct ipv6hdr *inside_iph;
    struct icmp6hdr *icmph, _icmph;
    __be16 *ports, _ports[2];
    u8 inside_nexthdr;
    __be16 inside_fragoff;
    int inside_hdrlen;
    icmph = skb_header_pointer(skb, outside_hdrlen,
    sizeof(_icmph), &_icmph);
    if (icmph == core::ptr::null_mut())
    return 1;
    if (icmph.icmp6_type & ICMPV6_INFOMSG_MASK)
    return 1;
    inside_iph = skb_header_pointer(skb, outside_hdrlen + sizeof(_icmph),
    sizeof(*ipv6_var), ipv6_var);
    if (inside_iph == core::ptr::null_mut())
    return 1;
    inside_nexthdr = inside_iph.nexthdr;
    inside_hdrlen = ipv6_skip_exthdr(skb, outside_hdrlen + sizeof(_icmph) +
    sizeof(*ipv6_var),
    &inside_nexthdr, &inside_fragoff);
    if (inside_hdrlen < 0)
    return 1; /* hjm: Packet has no/incomplete transport layer headers. */
    if (inside_nexthdr != IPPROTO_TCP &&
    inside_nexthdr != IPPROTO_UDP)
    return 1;
    ports = skb_header_pointer(skb, inside_hdrlen,
    sizeof(_ports), &_ports);
    if (ports == core::ptr::null_mut())
    return 1;
// the inside IP packet is the one quoted from our side, thus
// its saddr is the local address
// protocol = inside_nexthdr;
// laddr = &inside_iph->saddr;
// lport = ports[0];
// raddr = &inside_iph->daddr;
// rport = ports[1];
    return 0;
    }
    static struct sock *
    nf_socket_get_sock_v6(struct net *net, struct sk_buff *skb, int doff,
    const u8 protocol,
    const struct in6_addr *saddr, const struct in6_addr *daddr,
    const __be16 sport, const __be16 dport,
    const struct net_device *in)
    {
    switch (protocol) {
    case IPPROTO_TCP:
    return inet6_lookup(net, skb, doff, saddr, sport, daddr, dport,
    in.ifindex);
    case IPPROTO_UDP:
    return udp6_lib_lookup(net, saddr, sport, daddr, dport,
    in.ifindex);
    }
    return core::ptr::null_mut();
    }
    struct sock *nf_sk_lookup_slow_v6(struct net *net, const struct sk_buff *skb,
    const struct net_device *indev)
    {
    __be16 dport, sport;
    const struct in6_addr *daddr = core::ptr::null_mut(), *saddr = core::ptr::null_mut();
    struct ipv6hdr *iph = ipv6_hdr(skb), ipv6_var;
    struct sk_buff *data_skb = core::ptr::null_mut();
    let mut fragoff: c_ushort = 0;
    let mut doff: c_int = 0;
    let mut thoff: c_int = 0, tproto;

    enum ip_conntrack_info ctinfo;
    struct nf_conn const *ct;

    tproto = ipv6_find_hdr(skb, &thoff, -1, &fragoff, core::ptr::null_mut());
    if (tproto < 0 || fragoff) {
    pr_debug("unable to find transport header in IPv6 packet, dropping\n");
    return core::ptr::null_mut();
    }
    if (tproto == IPPROTO_UDP || tproto == IPPROTO_TCP) {
    struct tcphdr _hdr;
    struct udphdr *hp;
    hp = skb_header_pointer(skb, thoff, tproto == IPPROTO_UDP ?
    sizeof(*hp) : sizeof(_hdr), &_hdr);
    if (hp == core::ptr::null_mut())
    return core::ptr::null_mut();
    saddr = &iph.saddr;
    sport = hp.source;
    daddr = &iph.daddr;
    dport = hp.dest;
    data_skb = (struct sk_buff *)skb;
    doff = tproto == IPPROTO_TCP ?
    thoff + __tcp_hdrlen((struct tcphdr *)hp) :
    thoff + sizeof(*hp);
    } else if (tproto == IPPROTO_ICMPV6) {
    if (extract_icmp6_fields(skb, thoff, &tproto, &saddr, &daddr,
    &sport, &dport, &ipv6_var))
    return core::ptr::null_mut();
    } else {
    return core::ptr::null_mut();
    }

// Do the lookup with the original socket address in
// case this is a reply packet of an established
// SNAT-ted connection.
//
    ct = nf_ct_get(skb, &ctinfo);
    if (ct &&
    ((tproto != IPPROTO_ICMPV6 &&
    ctinfo == IP_CT_ESTABLISHED_REPLY) ||
    (tproto == IPPROTO_ICMPV6 &&
    ctinfo == IP_CT_RELATED_REPLY)) &&
    (ct.status & IPS_SRC_NAT_DONE)) {
    daddr = &ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple.src.u3.in6;
    dport = (tproto == IPPROTO_TCP) ?
    ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple.src.u.tcp.port :
    ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple.src.u.udp.port;
    }

    return nf_socket_get_sock_v6(net, data_skb, doff, tproto, saddr, daddr,
    sport, dport, indev);
    }
    EXPORT_SYMBOL_GPL(nf_sk_lookup_slow_v6);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Krisztian Kovacs, Balazs Scheidler");
    MODULE_DESCRIPTION("Netfilter IPv6 socket lookup infrastructure");
