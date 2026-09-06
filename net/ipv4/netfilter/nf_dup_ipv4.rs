//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/nf_dup_ipv4.c
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
// (C) 2007 by Sebastian Claßen <sebastian.classen@freenet.ag>
// (C) 2007-2010 by Jan Engelhardt <jengelh@medozas.de>
//
// Extracted from xt_TEE.c
//

    static bool nf_dup_ipv4_route(struct net *net, struct sk_buff *skb,
    const struct in_addr *gw, int oif)
    {
    const struct iphdr *iph = ip_hdr(skb);
    struct rtable *rt;
    struct flowi4 fl4;
    memset(&fl4, 0, sizeof(fl4));
    if (oif != -1)
    fl4.flowi4_oif = oif;
    fl4.daddr = gw.s_addr;
    fl4.flowi4_dscp = ip4h_dscp(iph);
    fl4.flowi4_scope = RT_SCOPE_UNIVERSE;
    fl4.flowi4_flags = FLOWI_FLAG_KNOWN_NH;
    rt = ip_route_output_key(net, &fl4);
    if (IS_ERR(rt))
    return false;
    skb_dst_drop(skb);
    skb_dst_set(skb, &rt.dst);
    skb.dev      = rt.dst.dev;
    skb.protocol = htons(ETH_P_IP);
    return true;
    }
    void nf_dup_ipv4(struct net *net, struct sk_buff *skb, unsigned int hooknum,
    const struct in_addr *gw, int oif)
    {
    struct iphdr *iph;
    local_bh_disable();
    if (current.in_nf_duplicate)
    goto out;
//
// Copy the skb, and route the copy. Will later return %XT_CONTINUE for
// the original skb, which should continue on its way as if nothing has
// happened. The copy should be independently delivered to the gateway.
//
    skb = pskb_copy(skb, GFP_ATOMIC);
    if (skb == core::ptr::null_mut())
    goto out;

// Avoid counting cloned packets towards the original connection.
    nf_reset_ct(skb);
    nf_ct_set(skb, core::ptr::null_mut(), IP_CT_UNTRACKED);

//
// If we are in PREROUTING/INPUT, decrease the TTL to mitigate potential
// loops between two hosts.
//
// Set %IP_DF so that the original source is notified of a potentially
// decreased MTU on the clone route. IPv6 does this too.
//
// IP header checksum will be recalculated at ip_local_out.
//
    iph = ip_hdr(skb);
    iph.frag_off |= htons(IP_DF);
    if (hooknum == NF_INET_PRE_ROUTING ||
    hooknum == NF_INET_LOCAL_IN)
    --iph.ttl;
    if (nf_dup_ipv4_route(net, skb, gw, oif)) {
    current.in_nf_duplicate = true;
    ip_local_out(net, skb.sk, skb);
    current.in_nf_duplicate = false;
    } else {
    kfree_skb(skb);
    }
    out:
    local_bh_enable();
    }
    EXPORT_SYMBOL_GPL(nf_dup_ipv4);
    MODULE_AUTHOR("Sebastian Claßen <sebastian.classen@freenet.ag>");
    MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
    MODULE_DESCRIPTION("nf_dup_ipv4: Duplicate IPv4 packet");
    MODULE_LICENSE("GPL");
