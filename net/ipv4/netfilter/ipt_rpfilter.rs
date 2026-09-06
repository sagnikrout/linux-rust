//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/ipt_rpfilter.c
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
// based on fib_frontend.c; Author: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
    MODULE_DESCRIPTION("iptables: ipv4 reverse path filter match");
// don't try to find route from mcast/bcast/zeronet
#[no_mangle]
unsafe extern "C" fn rpfilter_get_saddr(addr: __be32) -> __be32 {
    static __be32 rpfilter_get_saddr(__be32 addr)
    {
    if (ipv4_is_multicast(addr) || ipv4_is_lbcast(addr) ||
    ipv4_is_zeronet(addr))
    return 0;
    return addr;
    }
    static bool rpfilter_lookup_reverse(struct net *net, struct flowi4 *fl4,
    const struct net_device *dev, u8 flags)
    {
    struct fib_result res;
    if (fib_lookup(net, fl4, &res, FIB_LOOKUP_IGNORE_LINKSTATE))
    return false;
    if (res.type != RTN_UNICAST) {
    if (res.type != RTN_LOCAL || !(flags & XT_RPFILTER_ACCEPT_LOCAL))
    return false;
    }
    return fib_info_nh_uses_dev(res.fi, dev) || flags & XT_RPFILTER_LOOSE;
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
    const struct xt_rpfilter_info *info;
    const struct iphdr *iph;
    struct flowi4 flow;
    bool invert;
    info = par.matchinfo;
    invert = info.flags & XT_RPFILTER_INVERT;
    if (rpfilter_is_loopback(skb, xt_in(par)))
    return true ^ invert;
    iph = ip_hdr(skb);
    if (ipv4_is_zeronet(iph.saddr)) {
    if (ipv4_is_lbcast(iph.daddr) ||
    ipv4_is_local_multicast(iph.daddr))
    return true ^ invert;
    }
    memset(&flow, 0, sizeof(flow));
    flow.flowi4_iif = LOOPBACK_IFINDEX;
    flow.daddr = iph.saddr;
    flow.saddr = rpfilter_get_saddr(iph.daddr);
    flow.flowi4_mark = info.flags & XT_RPFILTER_VALID_MARK ? skb.mark : 0;
    flow.flowi4_dscp = ip4h_dscp(iph);
    flow.flowi4_scope = RT_SCOPE_UNIVERSE;
    flow.flowi4_l3mdev = l3mdev_master_ifindex_rcu(xt_in(par));
    flow.flowi4_uid = sock_net_uid(xt_net(par), core::ptr::null_mut());
    return rpfilter_lookup_reverse(xt_net(par), &flow, xt_in(par), info.flags) ^ invert;
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
    .family		= NFPROTO_IPV4,
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
