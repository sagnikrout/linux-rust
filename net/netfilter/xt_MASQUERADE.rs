//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_MASQUERADE.c
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
// Masquerade.  Simple mapping which alters range to a local IP address
    (depending on route). */
// (C) 1999-2001 Paul `Rusty' Russell
// (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Netfilter Core Team <coreteam@netfilter.org>");
    MODULE_DESCRIPTION("Xtables: automatic-address SNAT");
// FIXME: Multiple targets. --RR
#[no_mangle]
unsafe extern "C" fn masquerade_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int masquerade_tg_check(const struct xt_tgchk_param *par)
    {
    const struct nf_nat_ipv4_multi_range_compat *mr = par.targinfo;
    if (mr.range[0].flags & NF_NAT_RANGE_MAP_IPS) {
    pr_info_ratelimited("bad MAP_IPS.\n");
    return -EINVAL;
    }
    if (mr.rangesize != 1) {
    pr_info_ratelimited("bad rangesize %u\n", mr.rangesize);
    return -EINVAL;
    }
    return nf_ct_netns_get(par.net, par.family);
    }
    static unsigned int
    masquerade_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    struct nf_nat_range2 range;
    const struct nf_nat_ipv4_multi_range_compat *mr;
    mr = par.targinfo;
    range.flags = mr.range[0].flags;
    range.min_proto = mr.range[0].min;
    range.max_proto = mr.range[0].max;
    return nf_nat_masquerade_ipv4(skb, xt_hooknum(par), &range,
    xt_out(par));
    }
#[no_mangle]
unsafe extern "C" fn masquerade_tg_destroy(par: *const xt_tgdtor_param) {
    static void masquerade_tg_destroy(const struct xt_tgdtor_param *par)
    {
    nf_ct_netns_put(par.net, par.family);
    }

    static unsigned int
    masquerade_tg6(struct sk_buff *skb, const struct xt_action_param *par)
    {
    return nf_nat_masquerade_ipv6(skb, par.targinfo, xt_out(par));
    }
#[no_mangle]
unsafe extern "C" fn masquerade_tg6_checkentry(par: *const xt_tgchk_param) -> c_int {
    static int masquerade_tg6_checkentry(const struct xt_tgchk_param *par)
    {
    const struct nf_nat_range2 *range = par.targinfo;
    if (range.flags & NF_NAT_RANGE_MAP_IPS)
    return -EINVAL;
    return nf_ct_netns_get(par.net, par.family);
    }

    static struct xt_target masquerade_tg_reg[] __read_mostly = {
    {

    .name		= "MASQUERADE",
    .family		= NFPROTO_IPV6,
    .target		= masquerade_tg6,
    .targetsize	= sizeof(struct nf_nat_range),
    .table		= "nat",
    .hooks		= 1 << NF_INET_POST_ROUTING,
    .checkentry	= masquerade_tg6_checkentry,
    .destroy	= masquerade_tg_destroy,
    .me		= THIS_MODULE,
    }, {

    .name		= "MASQUERADE",
    .family		= NFPROTO_IPV4,
    .target		= masquerade_tg,
    .targetsize	= sizeof(struct nf_nat_ipv4_multi_range_compat),
    .table		= "nat",
    .hooks		= 1 << NF_INET_POST_ROUTING,
    .checkentry	= masquerade_tg_check,
    .destroy	= masquerade_tg_destroy,
    .me		= THIS_MODULE,
    }
    };
#[no_mangle]
unsafe extern "C" fn masquerade_tg_init() -> int __init {
    static int __init masquerade_tg_init(void)
    {
    int ret;
    ret = xt_register_targets(masquerade_tg_reg,
    ARRAY_SIZE(masquerade_tg_reg));
    if (ret)
    return ret;
    ret = nf_nat_masquerade_inet_register_notifiers();
    if (ret) {
    xt_unregister_targets(masquerade_tg_reg,
    ARRAY_SIZE(masquerade_tg_reg));
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn masquerade_tg_exit() -> void __exit {
    static void __exit masquerade_tg_exit(void)
    {
    xt_unregister_targets(masquerade_tg_reg, ARRAY_SIZE(masquerade_tg_reg));
    nf_nat_masquerade_inet_unregister_notifiers();
    }
    module_init(masquerade_tg_init);
    module_exit(masquerade_tg_exit);

    MODULE_ALIAS("ip6t_MASQUERADE");

    MODULE_ALIAS("ipt_MASQUERADE");
