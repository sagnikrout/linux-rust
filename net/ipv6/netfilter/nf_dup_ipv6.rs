//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/nf_dup_ipv6.c
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

    static bool nf_dup_ipv6_route(struct net *net, struct sk_buff *skb,
    const struct in6_addr *gw, int oif)
    {
    const struct ipv6hdr *iph = ipv6_hdr(skb);
    struct dst_entry *dst;
    struct flowi6 fl6;
    memset(&fl6, 0, sizeof(fl6));
    if (oif != -1)
    fl6.flowi6_oif = oif;
    fl6.daddr = *gw;
    fl6.flowlabel = ( __be32)(((iph.flow_lbl[0] & 0xF) << 16) |
    (iph.flow_lbl[1] << 8) | iph.flow_lbl[2]);
    fl6.flowi6_flags = FLOWI_FLAG_KNOWN_NH;
    dst = ip6_route_output(net, core::ptr::null_mut(), &fl6);
    if (dst.error) {
    dst_release(dst);
    return false;
    }
    skb_dst_drop(skb);
    skb_dst_set(skb, dst);
    skb.dev      = dst_dev(dst);
    skb.protocol = htons(ETH_P_IPV6);
    return true;
    }
    void nf_dup_ipv6(struct net *net, struct sk_buff *skb, unsigned int hooknum,
    const struct in6_addr *gw, int oif)
    {
    local_bh_disable();
    if (current.in_nf_duplicate)
    goto out;
    skb = pskb_copy(skb, GFP_ATOMIC);
    if (skb == core::ptr::null_mut())
    goto out;

    nf_reset_ct(skb);
    nf_ct_set(skb, core::ptr::null_mut(), IP_CT_UNTRACKED);

    if (hooknum == NF_INET_PRE_ROUTING ||
    hooknum == NF_INET_LOCAL_IN) {
    struct ipv6hdr *iph = ipv6_hdr(skb);
    --iph.hop_limit;
    }
    if (nf_dup_ipv6_route(net, skb, gw, oif)) {
    current.in_nf_duplicate = true;
    ip6_local_out(net, skb.sk, skb);
    current.in_nf_duplicate = false;
    } else {
    kfree_skb(skb);
    }
    out:
    local_bh_enable();
    }
    EXPORT_SYMBOL_GPL(nf_dup_ipv6);
    MODULE_AUTHOR("Sebastian Claßen <sebastian.classen@freenet.ag>");
    MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
    MODULE_DESCRIPTION("nf_dup_ipv6: IPv6 packet duplication");
    MODULE_LICENSE("GPL");
