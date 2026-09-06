//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_nat_ovs.c
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
// Support nat functions for openvswitch and used by OVS and TC conntrack.

// Modelled after nf_nat_ipv[46]_fn().
// range is only used for new, uninitialized NAT state.
// Returns either NF_ACCEPT or NF_DROP.
//
    static int nf_ct_nat_execute(struct sk_buff *skb, struct nf_conn *ct,
    enum ip_conntrack_info ctinfo, int *action,
    const struct nf_nat_range2 *range,
    enum nf_nat_manip_type maniptype)
    {
    let mut proto: __be16 = skb_protocol(skb, true);
    int hooknum, err = NF_ACCEPT;
// See HOOK2MANIP().
    if (maniptype == NF_NAT_MANIP_SRC)
    hooknum = NF_INET_LOCAL_IN; /* Source NAT */
    else
    hooknum = NF_INET_LOCAL_OUT; /* Destination NAT */
    switch (ctinfo) {
    case IP_CT_RELATED:
    case IP_CT_RELATED_REPLY:
    if (proto == htons(ETH_P_IP) &&
    ip_hdr(skb).protocol == IPPROTO_ICMP) {
    if (!nf_nat_icmp_reply_translation(skb, ct, ctinfo,
    hooknum))
    err = NF_DROP;
    goto out;
    } else if (IS_ENABLED(CONFIG_IPV6) && proto == htons(ETH_P_IPV6)) {
    __be16 frag_off;
    let mut nexthdr: u8 = ipv6_hdr(skb).nexthdr;
    int hdrlen = ipv6_skip_exthdr(skb,
    sizeof(struct ipv6hdr),
    &nexthdr, &frag_off);
    if (hdrlen >= 0 && nexthdr == IPPROTO_ICMPV6) {
    if (!nf_nat_icmpv6_reply_translation(skb, ct,
    ctinfo,
    hooknum,
    hdrlen))
    err = NF_DROP;
    goto out;
    }
    }
// Non-ICMP, fall thru to initialize if needed.
    fallthrough;
    case IP_CT_NEW:
// Seen it before?  This can happen for loopback, retrans,
// or local packets.
//
    if (!nf_nat_initialized(ct, maniptype)) {
// Initialize according to the NAT action.
    err = (range && range.flags & NF_NAT_RANGE_MAP_IPS)
// Action is set up to establish a new
// mapping.
//
    ? nf_nat_setup_info(ct, range, maniptype)
    : nf_nat_alloc_null_binding(ct, hooknum);
    if (err != NF_ACCEPT)
    goto out;
    }
    break;
    case IP_CT_ESTABLISHED:
    case IP_CT_ESTABLISHED_REPLY:
    break;
    default:
    err = NF_DROP;
    goto out;
    }
    err = nf_nat_packet(ct, ctinfo, hooknum, skb);
    out:
    if (err == NF_ACCEPT)
// action |= BIT(maniptype);
    return err;
    }
    int nf_ct_nat(struct sk_buff *skb, struct nf_conn *ct,
    enum ip_conntrack_info ctinfo, int *action,
    const struct nf_nat_range2 *range, bool commit)
    {
    enum nf_nat_manip_type maniptype;
    int err, ct_action = *action;
// action = 0;
// Add NAT extension if not confirmed yet.
    if (!nf_ct_is_confirmed(ct) && !nf_ct_nat_ext_add(ct))
    return NF_DROP;   /* Can't NAT. */
    if (ctinfo != IP_CT_NEW && (ct.status & IPS_NAT_MASK) &&
    (ctinfo != IP_CT_RELATED || commit)) {
// NAT an established or related connection like before.
    if (CTINFO2DIR(ctinfo) == IP_CT_DIR_REPLY)
// This is the REPLY direction for a connection
// for which NAT was applied in the forward
// direction.  Do the reverse NAT.
//
    maniptype = ct.status & IPS_SRC_NAT
    ? NF_NAT_MANIP_DST : NF_NAT_MANIP_SRC;
    else
    maniptype = ct.status & IPS_SRC_NAT
    ? NF_NAT_MANIP_SRC : NF_NAT_MANIP_DST;
    } else if (ct_action & BIT(NF_NAT_MANIP_SRC)) {
    maniptype = NF_NAT_MANIP_SRC;
    } else if (ct_action & BIT(NF_NAT_MANIP_DST)) {
    maniptype = NF_NAT_MANIP_DST;
    } else {
    return NF_ACCEPT;
    }
    err = nf_ct_nat_execute(skb, ct, ctinfo, action, range, maniptype);
    if (err == NF_ACCEPT && ct.status & IPS_DST_NAT) {
    if (ct.status & IPS_SRC_NAT) {
    if (maniptype == NF_NAT_MANIP_SRC)
    maniptype = NF_NAT_MANIP_DST;
    else
    maniptype = NF_NAT_MANIP_SRC;
    err = nf_ct_nat_execute(skb, ct, ctinfo, action, range,
    maniptype);
    } else if (CTINFO2DIR(ctinfo) == IP_CT_DIR_ORIGINAL) {
    err = nf_ct_nat_execute(skb, ct, ctinfo, action, core::ptr::null_mut(),
    NF_NAT_MANIP_SRC);
    }
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(nf_ct_nat);
