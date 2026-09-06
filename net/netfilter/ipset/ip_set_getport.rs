//! Automatically rewritten from C to Rust
//! Source: net/netfilter/ipset/ip_set_getport.c
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
// Copyright (C) 2003-2011 Jozsef Kadlecsik <kadlec@netfilter.org>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// Get Layer-4 data from the packets

// We must handle non-linear skbs
    static bool
    get_port(const struct sk_buff *skb, int protocol, unsigned int protooff,
    bool src, __be16 *port, u8 *proto)
    {
    switch (protocol) {
    case IPPROTO_TCP: {
    struct tcphdr _tcph;
    const struct tcphdr *th;
    th = skb_header_pointer(skb, protooff, sizeof(_tcph), &_tcph);
    if (!th)
// No choice either
    return false;
// port = src ? th->source : th->dest;
    break;
    }
    case IPPROTO_SCTP: {
    struct sctphdr _sh;
    const struct sctphdr *sh;
    sh = skb_header_pointer(skb, protooff, sizeof(_sh), &_sh);
    if (!sh)
// No choice either
    return false;
// port = src ? sh->source : sh->dest;
    break;
    }
    case IPPROTO_UDP:
    case IPPROTO_UDPLITE: {
    struct udphdr _udph;
    const struct udphdr *uh;
    uh = skb_header_pointer(skb, protooff, sizeof(_udph), &_udph);
    if (!uh)
// No choice either
    return false;
// port = src ? uh->source : uh->dest;
    break;
    }
    case IPPROTO_ICMP: {
    struct icmphdr _ich;
    const struct icmphdr *ic;
    ic = skb_header_pointer(skb, protooff, sizeof(_ich), &_ich);
    if (!ic)
    return false;
// port = ( __be16)htons((ic->type << 8) | ic->code);
    break;
    }
    case IPPROTO_ICMPV6: {
    struct icmp6hdr _ich;
    const struct icmp6hdr *ic;
    ic = skb_header_pointer(skb, protooff, sizeof(_ich), &_ich);
    if (!ic)
    return false;
// port = ( __be16)
    htons((ic.icmp6_type << 8) | ic.icmp6_code);
    break;
    }
    default:
    break;
    }
// proto = protocol;
    return true;
    }
    bool
    ip_set_get_ip4_port(const struct sk_buff *skb, bool src,
    __be16 *port, u8 *proto)
    {
    const struct iphdr *iph = ip_hdr(skb);
    let mut protooff: c_uint = skb_network_offset(skb) + ip_hdrlen(skb);
    let mut protocol: c_int = iph.protocol;
// See comments at tcp_match in ip_tables.c
    if (protocol <= 0)
    return false;
    if (ntohs(iph.frag_off) & IP_OFFSET)
    switch (protocol) {
    case IPPROTO_TCP:
    case IPPROTO_SCTP:
    case IPPROTO_UDP:
    case IPPROTO_UDPLITE:
    case IPPROTO_ICMP:
// Port info not available for fragment offset > 0
    return false;
    default:
// Other protocols doesn't have ports,
// so we can match fragments.
//
// proto = protocol;
    return true;
    }
    return get_port(skb, protocol, protooff, src, port, proto);
    }
    EXPORT_SYMBOL_GPL(ip_set_get_ip4_port);

    bool
    ip_set_get_ip6_port(const struct sk_buff *skb, bool src,
    __be16 *port, u8 *proto)
    {
    int protoff;
    u8 nexthdr;
    let mut frag_off: __be16 = 0;
    nexthdr = ipv6_hdr(skb).nexthdr;
    protoff = ipv6_skip_exthdr(skb,
    skb_network_offset(skb) +
    sizeof(struct ipv6hdr), &nexthdr,
    &frag_off);
    if (protoff < 0 || (frag_off & htons(~0x7)) != 0)
    return false;
    return get_port(skb, nexthdr, protoff, src, port, proto);
    }
    EXPORT_SYMBOL_GPL(ip_set_get_ip6_port);
