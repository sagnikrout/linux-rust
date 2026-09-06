//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/nf_socket_ipv4.c
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
    extract_icmp4_fields(const struct sk_buff *skb, u8 *protocol,
    __be32 *raddr, __be32 *laddr,
    __be16 *rport, __be16 *lport)
    {
    let mut outside_hdrlen: c_uint = ip_hdrlen(skb);
    struct iphdr *inside_iph, _inside_iph;
    struct icmphdr *icmph, _icmph;
    __be16 *ports, _ports[2];
    icmph = skb_header_pointer(skb, outside_hdrlen,
    sizeof(_icmph), &_icmph);
    if (icmph == core::ptr::null_mut())
    return 1;
    if (!icmp_is_err(icmph.type))
    return 1;
    inside_iph = skb_header_pointer(skb, outside_hdrlen +
    sizeof(struct icmphdr),
    sizeof(_inside_iph), &_inside_iph);
    if (inside_iph == core::ptr::null_mut())
    return 1;
    if (inside_iph.protocol != IPPROTO_TCP &&
    inside_iph.protocol != IPPROTO_UDP)
    return 1;
    ports = skb_header_pointer(skb, outside_hdrlen +
    sizeof(struct icmphdr) +
    (inside_iph.ihl << 2),
    sizeof(_ports), &_ports);
    if (ports == core::ptr::null_mut())
    return 1;
// the inside IP packet is the one quoted from our side, thus
// its saddr is the local address
// protocol = inside_iph->protocol;
// laddr = inside_iph->saddr;
// lport = ports[0];
// raddr = inside_iph->daddr;
// rport = ports[1];
    return 0;
    }
    static struct sock *
    nf_socket_get_sock_v4(struct net *net, struct sk_buff *skb, const int doff,
    const u8 protocol,
    const __be32 saddr, const __be32 daddr,
    const __be16 sport, const __be16 dport,
    const struct net_device *in)
    {
    switch (protocol) {
    case IPPROTO_TCP:
    return inet_lookup(net, skb, doff, saddr, sport, daddr, dport,
    in.ifindex);
    case IPPROTO_UDP:
    return udp4_lib_lookup(net, saddr, sport, daddr, dport,
    in.ifindex);
    }
    return core::ptr::null_mut();
    }
    struct sock *nf_sk_lookup_slow_v4(struct net *net, const struct sk_buff *skb,
    const struct net_device *indev)
    {
    __be32 daddr, saddr;
    __be16 dport, sport;
    const struct iphdr *iph = ip_hdr(skb);
    struct sk_buff *data_skb = core::ptr::null_mut();
    u8 protocol;

    enum ip_conntrack_info ctinfo;
    struct nf_conn const *ct;

    let mut doff: c_int = 0;
    if (ntohs(iph.frag_off) & IP_OFFSET)
    return core::ptr::null_mut();
    if (iph.protocol == IPPROTO_UDP || iph.protocol == IPPROTO_TCP) {
    struct tcphdr _hdr;
    struct udphdr *hp;
    hp = skb_header_pointer(skb, ip_hdrlen(skb),
    iph.protocol == IPPROTO_UDP ?
    sizeof(*hp) : sizeof(_hdr), &_hdr);
    if (hp == core::ptr::null_mut())
    return core::ptr::null_mut();
    protocol = iph.protocol;
    saddr = iph.saddr;
    sport = hp.source;
    daddr = iph.daddr;
    dport = hp.dest;
    data_skb = (struct sk_buff *)skb;
    doff = iph.protocol == IPPROTO_TCP ?
    ip_hdrlen(skb) + __tcp_hdrlen((struct tcphdr *)hp) :
    ip_hdrlen(skb) + sizeof(*hp);
    } else if (iph.protocol == IPPROTO_ICMP) {
    if (extract_icmp4_fields(skb, &protocol, &saddr, &daddr,
    &sport, &dport))
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
    ((iph.protocol != IPPROTO_ICMP &&
    ctinfo == IP_CT_ESTABLISHED_REPLY) ||
    (iph.protocol == IPPROTO_ICMP &&
    ctinfo == IP_CT_RELATED_REPLY)) &&
    (ct.status & IPS_SRC_NAT_DONE)) {
    daddr = ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple.src.u3.ip;
    dport = (iph.protocol == IPPROTO_TCP) ?
    ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple.src.u.tcp.port :
    ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple.src.u.udp.port;
    }

    return nf_socket_get_sock_v4(net, data_skb, doff, protocol, saddr,
    daddr, sport, dport, indev);
    }
    EXPORT_SYMBOL_GPL(nf_sk_lookup_slow_v4);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Krisztian Kovacs, Balazs Scheidler");
    MODULE_DESCRIPTION("Netfilter IPv4 socket lookup infrastructure");
