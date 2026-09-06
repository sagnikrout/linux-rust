//! Automatically rewritten from C to Rust
//! Source: net/ipv6/mcast_snoop.c
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
// Copyright (C) 2010: YOSHIFUJI Hideaki <yoshfuji@linux-ipv6.org>
// Copyright (C) 2015: Linus Lüssing <linus.luessing@c0d3.blue>
//
// Based on the MLD support added to br_multicast.c by YOSHIFUJI Hideaki.
//

#[no_mangle]
unsafe extern "C" fn ipv6_mc_check_ip6hdr(skb: *mut sk_buff) -> c_int {
    static int ipv6_mc_check_ip6hdr(struct sk_buff *skb)
    {
    const struct ipv6hdr *ip6h;
    unsigned int len;
    let mut offset: c_uint = skb_network_offset(skb) + sizeof(*ip6h);
    if (!pskb_may_pull(skb, offset))
    return -EINVAL;
    ip6h = ipv6_hdr(skb);
    if (ip6h.version != 6)
    return -EINVAL;
    len = offset + ntohs(ip6h.payload_len);
    if (skb.len < len || len <= offset)
    return -EINVAL;
    skb_set_transport_header(skb, offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipv6_mc_check_exthdrs(skb: *mut sk_buff) -> c_int {
    static int ipv6_mc_check_exthdrs(struct sk_buff *skb)
    {
    const struct ipv6hdr *ip6h;
    int offset;
    u8 nexthdr;
    __be16 frag_off;
    ip6h = ipv6_hdr(skb);
    if (ip6h.nexthdr != IPPROTO_HOPOPTS)
    return -ENOMSG;
    nexthdr = ip6h.nexthdr;
    offset = skb_network_offset(skb) + sizeof(*ip6h);
    offset = ipv6_skip_exthdr(skb, offset, &nexthdr, &frag_off);
    if (offset < 0)
    return -EINVAL;
    if (nexthdr != IPPROTO_ICMPV6)
    return -ENOMSG;
    skb_set_transport_header(skb, offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipv6_mc_check_mld_reportv2(skb: *mut sk_buff) -> c_int {
    static int ipv6_mc_check_mld_reportv2(struct sk_buff *skb)
    {
    let mut len: c_uint = skb_transport_offset(skb);
    len += sizeof(struct mld2_report);
    return ipv6_mc_may_pull(skb, len) ? 0 : -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn ipv6_mc_check_mld_query(skb: *mut sk_buff) -> c_int {
    static int ipv6_mc_check_mld_query(struct sk_buff *skb)
    {
    let mut transport_len: c_uint = ipv6_transport_len(skb);
    struct mld_msg *mld;
    unsigned int len;
// RFC2710+RFC3810 (MLDv1+MLDv2) require link-local source addresses
    if (!(ipv6_addr_type(&ipv6_hdr(skb).saddr) & IPV6_ADDR_LINKLOCAL))
    return -EINVAL;
// MLDv1?
    if (transport_len != sizeof(struct mld_msg)) {
// or MLDv2?
    if (transport_len < sizeof(struct mld2_query))
    return -EINVAL;
    len = skb_transport_offset(skb) + sizeof(struct mld2_query);
    if (!ipv6_mc_may_pull(skb, len))
    return -EINVAL;
    }
    mld = (struct mld_msg *)skb_transport_header(skb);
// RFC2710+RFC3810 (MLDv1+MLDv2) require the multicast link layer
// all-nodes destination address (ff02::1) for general queries
//
    if (ipv6_addr_any(&mld.mld_mca) &&
    !ipv6_addr_is_ll_all_nodes(&ipv6_hdr(skb).daddr))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipv6_mc_check_mld_msg(skb: *mut sk_buff) -> c_int {
    static int ipv6_mc_check_mld_msg(struct sk_buff *skb)
    {
    let mut len: c_uint = skb_transport_offset(skb) + sizeof(struct mld_msg);
    struct mld_msg *mld;
    if (!ipv6_mc_may_pull(skb, len))
    return -ENODATA;
    mld = (struct mld_msg *)skb_transport_header(skb);
    switch (mld.mld_type) {
    case ICMPV6_MGM_REDUCTION:
    case ICMPV6_MGM_REPORT:
    return 0;
    case ICMPV6_MLD2_REPORT:
    return ipv6_mc_check_mld_reportv2(skb);
    case ICMPV6_MGM_QUERY:
    return ipv6_mc_check_mld_query(skb);
    default:
    return -ENODATA;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ipv6_mc_validate_checksum(skb: *mut sk_buff) -> __sum16 {
    static inline __sum16 ipv6_mc_validate_checksum(struct sk_buff *skb)
    {
    return skb_checksum_validate(skb, IPPROTO_ICMPV6, ip6_compute_pseudo);
    }
#[no_mangle]
unsafe extern "C" fn ipv6_mc_check_icmpv6(skb: *mut sk_buff) -> c_int {
    static int ipv6_mc_check_icmpv6(struct sk_buff *skb)
    {
    let mut len: c_uint = skb_transport_offset(skb) + sizeof(struct icmp6hdr);
    let mut transport_len: c_uint = ipv6_transport_len(skb);
    struct sk_buff *skb_chk;
    if (!ipv6_mc_may_pull(skb, len))
    return -EINVAL;
    skb_chk = skb_checksum_trimmed(skb, transport_len,
    ipv6_mc_validate_checksum);
    if (!skb_chk)
    return -EINVAL;
    if (skb_chk != skb)
    kfree_skb(skb_chk);
    return 0;
    }
//
// ipv6_mc_check_mld - checks whether this is a sane MLD packet
// @skb: the skb to validate
//
// Checks whether an IPv6 packet is a valid MLD packet. If so sets
// skb transport header accordingly and returns zero.
//
// -EINVAL: A broken packet was detected, i.e. it violates some internet
// standard
// -ENOMSG: IP header validation succeeded but it is not an ICMPv6 packet
// with a hop-by-hop option.
// -ENODATA: IP+ICMPv6 header with hop-by-hop option validation succeeded
// but it is not an MLD packet.
// -ENOMEM: A memory allocation failure happened.
//
// Caller needs to set the skb network header and free any returned skb if it
// differs from the provided skb.
//
#[no_mangle]
pub unsafe extern "C" fn ipv6_mc_check_mld(skb: *mut sk_buff) -> c_int {
    int ipv6_mc_check_mld(struct sk_buff *skb)
    {
    int ret;
    ret = ipv6_mc_check_ip6hdr(skb);
    if (ret < 0)
    return ret;
    ret = ipv6_mc_check_exthdrs(skb);
    if (ret < 0)
    return ret;
    ret = ipv6_mc_check_icmpv6(skb);
    if (ret < 0)
    return ret;
    return ipv6_mc_check_mld_msg(skb);
    }
    EXPORT_SYMBOL(ipv6_mc_check_mld);
