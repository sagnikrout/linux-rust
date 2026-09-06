//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_DSCP.c
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
// x_tables module for setting the IPv4/IPv6 DSCP field, Version 1.8
//
// (C) 2002 by Harald Welte <laforge@netfilter.org>
// based on ipt_FTOS.c (C) 2000 by Matthew G. Marsh <mgm@paktronix.com>
//
// See RFC2474 for a description of the DSCP field within the IP Header.
//

    MODULE_AUTHOR("Harald Welte <laforge@netfilter.org>");
    MODULE_DESCRIPTION("Xtables: DSCP/TOS field modification");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_DSCP");
    MODULE_ALIAS("ip6t_DSCP");
    MODULE_ALIAS("ipt_TOS");
    MODULE_ALIAS("ip6t_TOS");

    static unsigned int
    dscp_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_DSCP_info *dinfo = par.targinfo;
    let mut dscp: u_int8_t = ipv4_get_dsfield(ip_hdr(skb)) >> XT_DSCP_SHIFT;
    if (dscp != dinfo.dscp) {
    if (skb_ensure_writable(skb, sizeof(struct iphdr)))
    return NF_DROP;
    ipv4_change_dsfield(ip_hdr(skb), XT_DSCP_ECN_MASK,
    dinfo.dscp << XT_DSCP_SHIFT);
    }
    return XT_CONTINUE;
    }
    static unsigned int
    dscp_tg6(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_DSCP_info *dinfo = par.targinfo;
    let mut dscp: u_int8_t = ipv6_get_dsfield(ipv6_hdr(skb)) >> XT_DSCP_SHIFT;
    if (dscp != dinfo.dscp) {
    if (skb_ensure_writable(skb, sizeof(struct ipv6hdr)))
    return NF_DROP;
    ipv6_change_dsfield(ipv6_hdr(skb), XT_DSCP_ECN_MASK,
    dinfo.dscp << XT_DSCP_SHIFT);
    }
    return XT_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn dscp_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int dscp_tg_check(const struct xt_tgchk_param *par)
    {
    const struct xt_DSCP_info *info = par.targinfo;
    if (info.dscp > XT_DSCP_MAX)
    return -EDOM;
    return 0;
    }
    static unsigned int
    tos_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_tos_target_info *info = par.targinfo;
    struct iphdr *iph = ip_hdr(skb);
    u_int8_t orig, nv;
    orig = ipv4_get_dsfield(iph);
    nv   = (orig & ~info.tos_mask) ^ info.tos_value;
    if (orig != nv) {
    if (skb_ensure_writable(skb, sizeof(struct iphdr)))
    return NF_DROP;
    iph = ip_hdr(skb);
    ipv4_change_dsfield(iph, 0, nv);
    }
    return XT_CONTINUE;
    }
    static unsigned int
    tos_tg6(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_tos_target_info *info = par.targinfo;
    struct ipv6hdr *iph = ipv6_hdr(skb);
    u_int8_t orig, nv;
    orig = ipv6_get_dsfield(iph);
    nv   = (orig & ~info.tos_mask) ^ info.tos_value;
    if (orig != nv) {
    if (skb_ensure_writable(skb, sizeof(struct iphdr)))
    return NF_DROP;
    iph = ipv6_hdr(skb);
    ipv6_change_dsfield(iph, 0, nv);
    }
    return XT_CONTINUE;
    }
    static struct xt_target dscp_tg_reg[] __read_mostly = {
    {
    .name		= "DSCP",
    .family		= NFPROTO_IPV4,
    .checkentry	= dscp_tg_check,
    .target		= dscp_tg,
    .targetsize	= sizeof(struct xt_DSCP_info),
    .table		= "mangle",
    .me		= THIS_MODULE,
    },
    {
    .name		= "DSCP",
    .family		= NFPROTO_IPV6,
    .checkentry	= dscp_tg_check,
    .target		= dscp_tg6,
    .targetsize	= sizeof(struct xt_DSCP_info),
    .table		= "mangle",
    .me		= THIS_MODULE,
    },
    {
    .name		= "TOS",
    .revision	= 1,
    .family		= NFPROTO_IPV4,
    .table		= "mangle",
    .target		= tos_tg,
    .targetsize	= sizeof(struct xt_tos_target_info),
    .me		= THIS_MODULE,
    },
    {
    .name		= "TOS",
    .revision	= 1,
    .family		= NFPROTO_IPV6,
    .table		= "mangle",
    .target		= tos_tg6,
    .targetsize	= sizeof(struct xt_tos_target_info),
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn dscp_tg_init() -> int __init {
    static int __init dscp_tg_init(void)
    {
    return xt_register_targets(dscp_tg_reg, ARRAY_SIZE(dscp_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn dscp_tg_exit() -> void __exit {
    static void __exit dscp_tg_exit(void)
    {
    xt_unregister_targets(dscp_tg_reg, ARRAY_SIZE(dscp_tg_reg));
    }
    module_init(dscp_tg_init);
    module_exit(dscp_tg_exit);
