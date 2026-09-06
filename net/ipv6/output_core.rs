//! Automatically rewritten from C to Rust
//! Source: net/ipv6/output_core.c
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
// IPv6 library code, needed by static components when full IPv6 support is
// not configured or static.  These functions are needed by GSO/GRO implementation.
//

    static u32 __ipv6_select_ident(struct net *net,
    const struct in6_addr *dst,
    const struct in6_addr *src)
    {
    return get_random_u32_above(0);
    }
// This function exists only for tap drivers that must support broken
// clients requesting UFO without specifying an IPv6 fragment ID.
//
// This is similar to ipv6_select_ident() but we use an independent hash
// seed to limit information leakage.
//
// The network header must be set before calling this.
//
#[no_mangle]
pub unsafe extern "C" fn ipv6_proxy_select_ident(net: *mut net, skb: *mut sk_buff) -> __be32 {
    __be32 ipv6_proxy_select_ident(struct net *net, struct sk_buff *skb)
    {
    struct in6_addr buf[2];
    struct in6_addr *addrs;
    u32 id;
    addrs = skb_header_pointer(skb,
    skb_network_offset(skb) +
    offsetof(struct ipv6hdr, saddr),
    sizeof(buf), buf);
    if (!addrs)
    return 0;
    id = __ipv6_select_ident(net, &addrs[1], &addrs[0]);
    return htonl(id);
    }
    __be32 ipv6_select_ident(struct net *net,
    const struct in6_addr *daddr,
    const struct in6_addr *saddr)
    {
    u32 id;
    id = __ipv6_select_ident(net, daddr, saddr);
    return htonl(id);
    }
    EXPORT_SYMBOL(ipv6_select_ident);
#[no_mangle]
pub unsafe extern "C" fn ip6_find_1stfragopt(skb: *mut sk_buff, nexthdr: *mut u8) -> c_int {
    int ip6_find_1stfragopt(struct sk_buff *skb, u8 **nexthdr)
    {
    let mut offset: c_uint = sizeof(struct ipv6hdr);
    unsigned int packet_len = skb_tail_pointer(skb) -
    skb_network_header(skb);
    let mut found_rhdr: c_int = 0;
// nexthdr = &ipv6_hdr(skb)->nexthdr;
    while (offset <= packet_len) {
    struct ipv6_opt_hdr *exthdr;
    switch (**nexthdr) {
    case NEXTHDR_HOP:
    break;
    case NEXTHDR_ROUTING:
    found_rhdr = 1;
    break;
    case NEXTHDR_DEST:

    if (ipv6_find_tlv(skb, offset, IPV6_TLV_HAO) >= 0)
    break;

    if (found_rhdr)
    return offset;
    break;
    default:
    return offset;
    }
    if (offset + sizeof(struct ipv6_opt_hdr) > packet_len)
    return -EINVAL;
    exthdr = (struct ipv6_opt_hdr *)(skb_network_header(skb) +
    offset);
    offset += ipv6_optlen(exthdr);
    if (offset > IPV6_MAXPLEN)
    return -EINVAL;
// nexthdr = &exthdr->nexthdr;
    }
    return -EINVAL;
    }
    EXPORT_SYMBOL(ip6_find_1stfragopt);
#[no_mangle]
pub unsafe extern "C" fn __ip6_local_out(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    int __ip6_local_out(struct net *net, struct sock *sk, struct sk_buff *skb)
    {
    ipv6_set_payload_len(ipv6_hdr(skb), skb.len - sizeof(struct ipv6hdr));
    IP6CB(skb).nhoff = offsetof(struct ipv6hdr, nexthdr);
// if egress device is enslaved to an L3 master device pass the
// skb to its handler for processing
//
    skb = l3mdev_ip6_out(sk, skb);
    if (unlikely(!skb))
    return 0;
    skb.protocol = htons(ETH_P_IPV6);
    return nf_hook(NFPROTO_IPV6, NF_INET_LOCAL_OUT,
    net, sk, skb, core::ptr::null_mut(), skb_dst_dev(skb),
    dst_output);
    }
    EXPORT_SYMBOL_GPL(__ip6_local_out);
#[no_mangle]
pub unsafe extern "C" fn ip6_local_out(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    int ip6_local_out(struct net *net, struct sock *sk, struct sk_buff *skb)
    {
    int err;
    err = __ip6_local_out(net, sk, skb);
    if (likely(err == 1))
    err = dst_output(net, sk, skb);
    return err;
    }
    EXPORT_SYMBOL_GPL(ip6_local_out);
