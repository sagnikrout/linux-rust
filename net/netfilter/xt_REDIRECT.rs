//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_REDIRECT.c
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
// (C) 1999-2001 Paul `Rusty' Russell
// (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
// Copyright (c) 2011 Patrick McHardy <kaber@trash.net>
//
// Based on Rusty Russell's IPv4 REDIRECT target. Development of IPv6
// NAT funded by Astaro.
//

    static unsigned int
    redirect_tg6(struct sk_buff *skb, const struct xt_action_param *par)
    {
    return nf_nat_redirect_ipv6(skb, par.targinfo, xt_hooknum(par));
    }
#[no_mangle]
unsafe extern "C" fn redirect_tg6_checkentry(par: *const xt_tgchk_param) -> c_int {
    static int redirect_tg6_checkentry(const struct xt_tgchk_param *par)
    {
    const struct nf_nat_range2 *range = par.targinfo;
    if (range.flags & NF_NAT_RANGE_MAP_IPS)
    return -EINVAL;
    return nf_ct_netns_get(par.net, par.family);
    }
#[no_mangle]
unsafe extern "C" fn redirect_tg_destroy(par: *const xt_tgdtor_param) {
    static void redirect_tg_destroy(const struct xt_tgdtor_param *par)
    {
    nf_ct_netns_put(par.net, par.family);
    }
#[no_mangle]
unsafe extern "C" fn redirect_tg4_check(par: *const xt_tgchk_param) -> c_int {
    static int redirect_tg4_check(const struct xt_tgchk_param *par)
    {
    const struct nf_nat_ipv4_multi_range_compat *mr = par.targinfo;
    if (mr.range[0].flags & NF_NAT_RANGE_MAP_IPS) {
    pr_info_ratelimited("bad MAP_IPS.\n");
    return -EINVAL;
    }
    if (mr.rangesize != 1) {
    pr_info_ratelimited("bad rangesize %u.\n", mr.rangesize);
    return -EINVAL;
    }
    return nf_ct_netns_get(par.net, par.family);
    }
    static unsigned int
    redirect_tg4(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct nf_nat_ipv4_multi_range_compat *mr = par.targinfo;
    struct nf_nat_range2 range = {
    .flags       = mr.range[0].flags,
    .min_proto   = mr.range[0].min,
    .max_proto   = mr.range[0].max,
    };
    return nf_nat_redirect_ipv4(skb, &range, xt_hooknum(par));
    }
    static struct xt_target redirect_tg_reg[] __read_mostly = {
    {
    .name       = "REDIRECT",
    .family     = NFPROTO_IPV6,
    .revision   = 0,
    .table      = "nat",
    .checkentry = redirect_tg6_checkentry,
    .destroy    = redirect_tg_destroy,
    .target     = redirect_tg6,
    .targetsize = sizeof(struct nf_nat_range),
    .hooks      = (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_OUT),
    .me         = THIS_MODULE,
    },
    {
    .name       = "REDIRECT",
    .family     = NFPROTO_IPV4,
    .revision   = 0,
    .table      = "nat",
    .target     = redirect_tg4,
    .checkentry = redirect_tg4_check,
    .destroy    = redirect_tg_destroy,
    .targetsize = sizeof(struct nf_nat_ipv4_multi_range_compat),
    .hooks      = (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_OUT),
    .me         = THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn redirect_tg_init() -> int __init {
    static int __init redirect_tg_init(void)
    {
    return xt_register_targets(redirect_tg_reg,
    ARRAY_SIZE(redirect_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn redirect_tg_exit() -> void __exit {
    static void __exit redirect_tg_exit(void)
    {
    xt_unregister_targets(redirect_tg_reg, ARRAY_SIZE(redirect_tg_reg));
    }
    module_init(redirect_tg_init);
    module_exit(redirect_tg_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
    MODULE_DESCRIPTION("Xtables: Connection redirection to localhost");
    MODULE_ALIAS("ip6t_REDIRECT");
    MODULE_ALIAS("ipt_REDIRECT");
