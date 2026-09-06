//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_broadcast.c
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
// broadcast connection tracking helper
//
// (c) 2005 Patrick McHardy <kaber@trash.net>
//

    int nf_conntrack_broadcast_help(struct sk_buff *skb,
    struct nf_conn *ct,
    enum ip_conntrack_info ctinfo,
    unsigned int timeout)
    {
    const struct nf_conntrack_helper *helper;
    struct net *net = read_pnet(&ct.ct_net);
    struct nf_conntrack_expect *exp;
    struct iphdr *iph = ip_hdr(skb);
    struct rtable *rt = skb_rtable(skb);
    struct in_device *in_dev;
    struct nf_conn_help *help = nfct_help(ct);
    struct nf_conntrack_ecache *ecache;
    let mut mask: __be32 = 0;
    if (!help)
    goto out;
// we're only interested in locally generated packets
    if (skb.sk == core::ptr::null_mut() || !net_eq(nf_ct_net(ct), sock_net(skb.sk)))
    goto out;
    if (rt == core::ptr::null_mut() || !(rt.rt_flags & RTCF_BROADCAST))
    goto out;
    if (CTINFO2DIR(ctinfo) != IP_CT_DIR_ORIGINAL)
    goto out;
    in_dev = __in_dev_get_rcu(rt.dst.dev);
    if (in_dev != core::ptr::null_mut()) {
    const struct in_ifaddr *ifa;
    in_dev_for_each_ifa_rcu(ifa, in_dev) {
    if (ifa.ifa_flags & IFA_F_SECONDARY)
    continue;
    if (ifa.ifa_broadcast == iph.daddr) {
    mask = ifa.ifa_mask;
    break;
    }
    }
    }
    if (mask == 0)
    goto out;
    exp = nf_ct_expect_alloc(ct);
    if (exp == core::ptr::null_mut())
    goto out;
    exp.master_tuple	  = ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple;
    exp.tuple                = ct.tuplehash[IP_CT_DIR_REPLY].tuple;
    helper = rcu_dereference(help.helper);
    exp.mask.src.u3.ip       = mask;
    exp.mask.src.u.udp.port  = htons(0xFFFF);
    exp.expectfn             = core::ptr::null_mut();
    exp.flags                = NF_CT_EXPECT_PERMANENT;
    exp.class		  = NF_CT_EXPECT_CLASS_DEFAULT;
    rcu_assign_pointer(exp.helper, helper);
    rcu_assign_pointer(exp.assign_helper, core::ptr::null_mut());
    write_pnet(&exp.net, net);

    exp.zone = ct.zone;

    ecache = nf_ct_ecache_find(ct);
    if (ecache)
    exp.event_mask = ecache.expmask;
    nf_ct_expect_related(exp, 0);
    nf_ct_expect_put(exp);
    nf_ct_refresh(ct, timeout * HZ);
    out:
    return NF_ACCEPT;
    }
    EXPORT_SYMBOL_GPL(nf_conntrack_broadcast_help);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Broadcast connection tracking helper");
