//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter.c
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
// IPv6 specific functions of netfilter core
//
// Rusty Russell (C) 2000
// Patrick McHardy (C) 2006-2012
//

#[no_mangle]
pub unsafe extern "C" fn ip6_route_me_harder(net: *mut net, sk_partial: *mut sock, skb: *mut sk_buff) -> c_int {
    int ip6_route_me_harder(struct net *net, struct sock *sk_partial, struct sk_buff *skb)
    {
    const struct ipv6hdr *iph = ipv6_hdr(skb);
    struct sock *sk = sk_to_full_sk(sk_partial);
    struct net_device *dev = skb_dst_dev(skb);
    struct flow_keys flkeys;
    unsigned int hh_len;
    struct dst_entry *dst;
    int strict = (ipv6_addr_type(&iph.daddr) &
    (IPV6_ADDR_MULTICAST | IPV6_ADDR_LINKLOCAL));
    struct flowi6 fl6 = {
    .flowi6_l3mdev = l3mdev_master_ifindex(dev),
    .flowi6_mark = skb.mark,
    .flowi6_uid = sock_net_uid(net, sk),
    .daddr = iph.daddr,
    .saddr = iph.saddr,
    .flowlabel = ip6_flowinfo(iph),
    };
    int err;
    if (sk && sk.sk_bound_dev_if)
    fl6.flowi6_oif = sk.sk_bound_dev_if;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strict) -> else {
    else if (strict)
    fl6.flowi6_oif = dev.ifindex;
    fib6_rules_early_flow_dissect(net, skb, &fl6, &flkeys);
    dst = ip6_route_output(net, sk, &fl6);
    err = dst.error;
    if (err) {
    IP6_INC_STATS(net, ip6_dst_idev(dst), IPSTATS_MIB_OUTNOROUTES);
    net_dbg_ratelimited("ip6_route_me_harder: No more route\n");
    dst_release(dst);
    return err;
    }
// Drop old route.
    skb_dst_drop(skb);
    skb_dst_set(skb, dst);

    if (!(IP6CB(skb).flags & IP6SKB_XFRM_TRANSFORMED) &&
    xfrm_decode_session(net, skb, flowi6_to_flowi(&fl6), AF_INET6) == 0) {
// ignore return value from skb_dstref_steal, xfrm_lookup takes
// care of dropping the refcnt if needed.
//
    skb_dstref_steal(skb);
    dst = xfrm_lookup(net, dst, flowi6_to_flowi(&fl6), sk, 0);
    if (IS_ERR(dst))
    return PTR_ERR(dst);
    skb_dst_set(skb, dst);
    }

// Change in oif may mean change in hh_len.
    hh_len = skb_dst_dev(skb).hard_header_len;
    if (skb_headroom(skb) < hh_len &&
    pskb_expand_head(skb, HH_DATA_ALIGN(hh_len - skb_headroom(skb)),
    0, GFP_ATOMIC))
    return -ENOMEM;
    return 0;
    }
    EXPORT_SYMBOL(ip6_route_me_harder);
    int __nf_ip6_route(struct net *net, struct dst_entry **dst,
    struct flowi *fl, bool strict)
    {
    static const struct ipv6_pinfo fake_pinfo;
    static const struct inet_sock fake_sk = {
// makes ip6_route_output set RT6_LOOKUP_F_IFACE:
    .sk.sk_bound_dev_if = 1,
    .pinet6 = (struct ipv6_pinfo *) &fake_pinfo,
    };
    const void *sk = strict ? &fake_sk : core::ptr::null_mut();
    struct dst_entry *result;
    int err;
    result = ip6_route_output(net, sk, &fl.u.ip6);
    err = result.error;
    if (err)
    dst_release(result);
    else
// dst = result;
    return err;
    }
    EXPORT_SYMBOL_GPL(__nf_ip6_route);
    int br_ip6_fragment(struct net *net, struct sock *sk, struct sk_buff *skb,
    struct nf_bridge_frag_data *data,
    int (*output)(struct net *, struct sock *sk,
    const struct nf_bridge_frag_data *data,
    struct sk_buff *))
    {
    let mut frag_max_size: c_int = BR_INPUT_SKB_CB(skb).frag_max_size;
    let mut tstamp_type: u8 = skb.tstamp_type;
    let mut tstamp: ktime_t = skb.tstamp;
    struct ip6_frag_state state;
    u8 *prevhdr, nexthdr = 0;
    unsigned int mtu, hlen, nexthdr_offset;
    int hroom, err = 0;
    __be32 frag_id;
    err = ip6_find_1stfragopt(skb, &prevhdr);
    if (err < 0)
    goto blackhole;
    hlen = err;
    nexthdr = *prevhdr;
    nexthdr_offset = prevhdr - skb_network_header(skb);
    mtu = skb.dev.mtu;
    if (frag_max_size > mtu ||
    frag_max_size < IPV6_MIN_MTU)
    goto blackhole;
    mtu = frag_max_size;
    if (mtu < hlen + sizeof(struct frag_hdr) + 8)
    goto blackhole;
    mtu -= hlen + sizeof(struct frag_hdr);
    frag_id = ipv6_select_ident(net, &ipv6_hdr(skb).daddr,
    &ipv6_hdr(skb).saddr);
    if (skb.ip_summed == CHECKSUM_PARTIAL &&
    (err = skb_checksum_help(skb)))
    goto blackhole;
    prevhdr = skb_network_header(skb) + nexthdr_offset;
    hroom = LL_RESERVED_SPACE(skb.dev);
    if (skb_has_frag_list(skb)) {
    let mut first_len: c_uint = skb_pagelen(skb);
    struct ip6_fraglist_iter iter;
    struct sk_buff *frag2;
    if (first_len - hlen > mtu)
    goto blackhole;
    if (skb_cloned(skb) ||
    skb_headroom(skb) < (hroom + sizeof(struct frag_hdr)))
    goto slow_path;
    skb_walk_frags(skb, frag2) {
    if (frag2.len > mtu)
    goto blackhole;
// Partially cloned skb?
    if (skb_shared(frag2) ||
    skb_headroom(frag2) < (hlen + hroom + sizeof(struct frag_hdr)))
    goto slow_path;
    }
    err = ip6_fraglist_init(skb, hlen, prevhdr, nexthdr, frag_id,
    &iter);
    if (err < 0)
    goto blackhole;
    for (;;) {
// Prepare header of the next frame,
// before previous one went down.
//
    if (iter.frag)
    ip6_fraglist_prepare(skb, &iter);
    skb_set_delivery_time(skb, tstamp, tstamp_type);
    err = output(net, sk, data, skb);
    if (err || !iter.frag)
    break;
    skb = ip6_fraglist_next(&iter);
    }
    kfree(iter.tmp_hdr);
    if (!err)
    return 0;
    kfree_skb_list(iter.frag);
    return err;
    }
    slow_path:
// This is a linearized skbuff, the original geometry is lost for us.
// This may also be a clone skbuff, we could preserve the geometry for
// the copies but probably not worth the effort.
//
    ip6_frag_init(skb, hlen, mtu, skb.dev.needed_tailroom,
    LL_RESERVED_SPACE(skb.dev), prevhdr, nexthdr, frag_id,
    &state);
    while (state.left > 0) {
    struct sk_buff *skb2;
    skb2 = ip6_frag_next(skb, &state);
    if (IS_ERR(skb2)) {
    err = PTR_ERR(skb2);
    goto blackhole;
    }
    skb_set_delivery_time(skb2, tstamp, tstamp_type);
    err = output(net, sk, data, skb2);
    if (err)
    goto blackhole;
    }
    consume_skb(skb);
    return err;
    blackhole:
    kfree_skb(skb);
    return 0;
    }
    EXPORT_SYMBOL_GPL(br_ip6_fragment);
