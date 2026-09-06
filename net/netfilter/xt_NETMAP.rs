//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_NETMAP.c
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
// (C) 2000-2001 Svenning Soerensen <svenning@post5.tele.dk>
// Copyright (c) 2011 Patrick McHardy <kaber@trash.net>
//

    static unsigned int
    netmap_tg6(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct nf_nat_range2 *range = par.targinfo;
    struct nf_nat_range2 newrange;
    struct nf_conn *ct;
    enum ip_conntrack_info ctinfo;
    union nf_inet_addr new_addr, netmask;
    unsigned int i;
    ct = nf_ct_get(skb, &ctinfo);
    for (i = 0; i < ARRAY_SIZE(range.min_addr.ip6); i++)
    netmask.ip6[i] = ~(range.min_addr.ip6[i] ^
    range.max_addr.ip6[i]);
    if (xt_hooknum(par) == NF_INET_PRE_ROUTING ||
    xt_hooknum(par) == NF_INET_LOCAL_OUT)
    new_addr.in6 = ipv6_hdr(skb).daddr;
    else
    new_addr.in6 = ipv6_hdr(skb).saddr;
    for (i = 0; i < ARRAY_SIZE(new_addr.ip6); i++) {
    new_addr.ip6[i] &= ~netmask.ip6[i];
    new_addr.ip6[i] |= range.min_addr.ip6[i] &
    netmask.ip6[i];
    }
    newrange.flags	= range.flags | NF_NAT_RANGE_MAP_IPS;
    newrange.min_addr	= new_addr;
    newrange.max_addr	= new_addr;
    newrange.min_proto	= range.min_proto;
    newrange.max_proto	= range.max_proto;
    return nf_nat_setup_info(ct, &newrange, HOOK2MANIP(xt_hooknum(par)));
    }
#[no_mangle]
unsafe extern "C" fn netmap_tg6_checkentry(par: *const xt_tgchk_param) -> c_int {
    static int netmap_tg6_checkentry(const struct xt_tgchk_param *par)
    {
    const struct nf_nat_range2 *range = par.targinfo;
    if (!(range.flags & NF_NAT_RANGE_MAP_IPS))
    return -EINVAL;
    return nf_ct_netns_get(par.net, par.family);
    }
#[no_mangle]
unsafe extern "C" fn netmap_tg_destroy(par: *const xt_tgdtor_param) {
    static void netmap_tg_destroy(const struct xt_tgdtor_param *par)
    {
    nf_ct_netns_put(par.net, par.family);
    }
    static unsigned int
    netmap_tg4(struct sk_buff *skb, const struct xt_action_param *par)
    {
    struct nf_conn *ct;
    enum ip_conntrack_info ctinfo;
    __be32 new_ip, netmask;
    const struct nf_nat_ipv4_multi_range_compat *mr = par.targinfo;
    struct nf_nat_range2 newrange;
    WARN_ON(xt_hooknum(par) != NF_INET_PRE_ROUTING &&
    xt_hooknum(par) != NF_INET_POST_ROUTING &&
    xt_hooknum(par) != NF_INET_LOCAL_OUT &&
    xt_hooknum(par) != NF_INET_LOCAL_IN);
    ct = nf_ct_get(skb, &ctinfo);
    netmask = ~(mr.range[0].min_ip ^ mr.range[0].max_ip);
    if (xt_hooknum(par) == NF_INET_PRE_ROUTING ||
    xt_hooknum(par) == NF_INET_LOCAL_OUT)
    new_ip = ip_hdr(skb).daddr & ~netmask;
    else
    new_ip = ip_hdr(skb).saddr & ~netmask;
    new_ip |= mr.range[0].min_ip & netmask;
    memset(&newrange.min_addr, 0, sizeof(newrange.min_addr));
    memset(&newrange.max_addr, 0, sizeof(newrange.max_addr));
    newrange.flags	     = mr.range[0].flags | NF_NAT_RANGE_MAP_IPS;
    newrange.min_addr.ip = new_ip;
    newrange.max_addr.ip = new_ip;
    newrange.min_proto   = mr.range[0].min;
    newrange.max_proto   = mr.range[0].max;
// Hand modified range to generic setup.
    return nf_nat_setup_info(ct, &newrange, HOOK2MANIP(xt_hooknum(par)));
    }
#[no_mangle]
unsafe extern "C" fn netmap_tg4_check(par: *const xt_tgchk_param) -> c_int {
    static int netmap_tg4_check(const struct xt_tgchk_param *par)
    {
    const struct nf_nat_ipv4_multi_range_compat *mr = par.targinfo;
    if (!(mr.range[0].flags & NF_NAT_RANGE_MAP_IPS)) {
    pr_info_ratelimited("bad MAP_IPS.\n");
    return -EINVAL;
    }
    if (mr.rangesize != 1) {
    pr_info_ratelimited("bad rangesize %u.\n", mr.rangesize);
    return -EINVAL;
    }
    return nf_ct_netns_get(par.net, par.family);
    }
    static struct xt_target netmap_tg_reg[] __read_mostly = {
    {
    .name       = "NETMAP",
    .family     = NFPROTO_IPV6,
    .revision   = 0,
    .target     = netmap_tg6,
    .targetsize = sizeof(struct nf_nat_range),
    .table      = "nat",
    .hooks      = (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_LOCAL_IN),
    .checkentry = netmap_tg6_checkentry,
    .destroy    = netmap_tg_destroy,
    .me         = THIS_MODULE,
    },
    {
    .name       = "NETMAP",
    .family     = NFPROTO_IPV4,
    .revision   = 0,
    .target     = netmap_tg4,
    .targetsize = sizeof(struct nf_nat_ipv4_multi_range_compat),
    .table      = "nat",
    .hooks      = (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_LOCAL_IN),
    .checkentry = netmap_tg4_check,
    .destroy    = netmap_tg_destroy,
    .me         = THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn netmap_tg_init() -> int __init {
    static int __init netmap_tg_init(void)
    {
    return xt_register_targets(netmap_tg_reg, ARRAY_SIZE(netmap_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn netmap_tg_exit() {
    static void netmap_tg_exit(void)
    {
    xt_unregister_targets(netmap_tg_reg, ARRAY_SIZE(netmap_tg_reg));
    }
    module_init(netmap_tg_init);
    module_exit(netmap_tg_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Xtables: 1:1 NAT mapping of subnets");
    MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
    MODULE_ALIAS("ip6t_NETMAP");
    MODULE_ALIAS("ipt_NETMAP");
