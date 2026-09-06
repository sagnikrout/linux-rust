//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/ip6t_rpfilter.c
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
// Copyright (c) 2011 Florian Westphal <fw@strlen.de>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
    MODULE_DESCRIPTION("Xtables: IPv6 reverse path filter match");
#[no_mangle]
unsafe extern "C" fn rpfilter_addr_unicast(addr: *const in6_addr) -> bool {
    static bool rpfilter_addr_unicast(const struct in6_addr *addr)
    {
    let mut addr_type: c_int = ipv6_addr_type(addr);
    return addr_type & IPV6_ADDR_UNICAST;
    }
#[no_mangle]
unsafe extern "C" fn rpfilter_addr_linklocal(addr: *const in6_addr) -> bool {
    static bool rpfilter_addr_linklocal(const struct in6_addr *addr)
    {
    let mut addr_type: c_int = ipv6_addr_type(addr);
    return addr_type & IPV6_ADDR_LINKLOCAL;
    }
    static bool rpfilter_lookup_reverse6(struct net *net, const struct sk_buff *skb,
    const struct net_device *dev, u8 flags)
    {
    struct rt6_info *rt;
    struct ipv6hdr *iph = ipv6_hdr(skb);
    let mut ret: bool = false;
    struct flowi6 fl6 = {
    .flowi6_iif = LOOPBACK_IFINDEX,
    .flowi6_l3mdev = l3mdev_master_ifindex_rcu(dev),
    .flowlabel = (* (__be32 *) iph) & IPV6_FLOWINFO_MASK,
    .flowi6_proto = iph.nexthdr,
    .flowi6_uid = sock_net_uid(net, core::ptr::null_mut()),
    .daddr = iph.saddr,
    };
    int lookup_flags;
    if (rpfilter_addr_unicast(&iph.daddr)) {
    memcpy(&fl6.saddr, &iph.daddr, sizeof(struct in6_addr));
    lookup_flags = RT6_LOOKUP_F_HAS_SADDR;
    } else {
    lookup_flags = 0;
    }
    fl6.flowi6_mark = flags & XT_RPFILTER_VALID_MARK ? skb.mark : 0;
    if (rpfilter_addr_linklocal(&iph.saddr)) {
    lookup_flags |= RT6_LOOKUP_F_IFACE;
    fl6.flowi6_oif = dev.ifindex;
    } else if ((flags & XT_RPFILTER_LOOSE) == 0)
    fl6.flowi6_oif = dev.ifindex;
    rt = (void *)ip6_route_lookup(net, &fl6, skb, lookup_flags);
    if (rt.dst.error)
    goto out;
    if (rt.rt6i_flags & (RTF_REJECT|RTF_ANYCAST))
    goto out;
    if (rt.rt6i_flags & RTF_LOCAL) {
    ret = flags & XT_RPFILTER_ACCEPT_LOCAL;
    goto out;
    }
    if (rt.rt6i_idev.dev == dev ||
    l3mdev_master_ifindex_rcu(rt.rt6i_idev.dev) == dev.ifindex ||
    (flags & XT_RPFILTER_LOOSE))
    ret = true;
    out:
    ip6_rt_put(rt);
    return ret;
    }
    static bool
    rpfilter_is_loopback(const struct sk_buff *skb, const struct net_device *in)
    {
    return skb.pkt_type == PACKET_LOOPBACK || in.flags & IFF_LOOPBACK;
    }
#[no_mangle]
unsafe extern "C" fn rpfilter_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool rpfilter_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_rpfilter_info *info = par.matchinfo;
    int saddrtype;
    struct ipv6hdr *iph;
    let mut invert: bool = info.flags & XT_RPFILTER_INVERT;
    if (rpfilter_is_loopback(skb, xt_in(par)))
    return true ^ invert;
    iph = ipv6_hdr(skb);
    saddrtype = ipv6_addr_type(&iph.saddr);
    if (unlikely(saddrtype == IPV6_ADDR_ANY))
    return true ^ invert; /* not routable: forward path will drop it */
    return rpfilter_lookup_reverse6(xt_net(par), skb, xt_in(par),
    info.flags) ^ invert;
    }
#[no_mangle]
unsafe extern "C" fn rpfilter_check(par: *const xt_mtchk_param) -> c_int {
    static int rpfilter_check(const struct xt_mtchk_param *par)
    {
    const struct xt_rpfilter_info *info = par.matchinfo;
    let mut options: c_uint = ~XT_RPFILTER_OPTION_MASK;
    if (info.flags & options) {
    pr_info_ratelimited("unknown options\n");
    return -EINVAL;
    }
    if (strcmp(par.table, "mangle") != 0 &&
    strcmp(par.table, "raw") != 0) {
    pr_info_ratelimited("only valid in \'raw\' or \'mangle\' table, not \'%s\'\n",
    par.table);
    return -EINVAL;
    }
    return 0;
    }
    static struct xt_match rpfilter_mt_reg __read_mostly = {
    .name		= "rpfilter",
    .family		= NFPROTO_IPV6,
    .checkentry	= rpfilter_check,
    .match		= rpfilter_mt,
    .matchsize	= sizeof(struct xt_rpfilter_info),
    .hooks		= (1 << NF_INET_PRE_ROUTING),
    .me		= THIS_MODULE
    };
#[no_mangle]
unsafe extern "C" fn rpfilter_mt_init() -> int __init {
    static int __init rpfilter_mt_init(void)
    {
    return xt_register_match(&rpfilter_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn rpfilter_mt_exit() -> void __exit {
    static void __exit rpfilter_mt_exit(void)
    {
    xt_unregister_match(&rpfilter_mt_reg);
    }
    module_init(rpfilter_mt_init);
    module_exit(rpfilter_mt_exit);
