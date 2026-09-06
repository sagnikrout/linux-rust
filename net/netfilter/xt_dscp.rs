//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_dscp.c
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
// IP tables module for matching the value of the IPv4/IPv6 DSCP field
//
// (C) 2002 by Harald Welte <laforge@netfilter.org>
//

    MODULE_AUTHOR("Harald Welte <laforge@netfilter.org>");
    MODULE_DESCRIPTION("Xtables: DSCP/TOS field match");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_dscp");
    MODULE_ALIAS("ip6t_dscp");
    MODULE_ALIAS("ipt_tos");
    MODULE_ALIAS("ip6t_tos");
    static bool
    dscp_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_dscp_info *info = par.matchinfo;
    let mut dscp: u_int8_t = ipv4_get_dsfield(ip_hdr(skb)) >> XT_DSCP_SHIFT;
    return (dscp == info.dscp) ^ !!info.invert;
    }
    static bool
    dscp_mt6(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_dscp_info *info = par.matchinfo;
    let mut dscp: u_int8_t = ipv6_get_dsfield(ipv6_hdr(skb)) >> XT_DSCP_SHIFT;
    return (dscp == info.dscp) ^ !!info.invert;
    }
#[no_mangle]
unsafe extern "C" fn dscp_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int dscp_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_dscp_info *info = par.matchinfo;
    if (info.dscp > XT_DSCP_MAX)
    return -EDOM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tos_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int tos_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_tos_match_info *info = par.matchinfo;
    if (info.invert > 1)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tos_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool tos_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_tos_match_info *info = par.matchinfo;
    if (xt_family(par) == NFPROTO_IPV4)
    return ((ip_hdr(skb).tos & info.tos_mask) ==
    info.tos_value) ^ !!info.invert;
    else
    return ((ipv6_get_dsfield(ipv6_hdr(skb)) & info.tos_mask) ==
    info.tos_value) ^ !!info.invert;
    }
    static struct xt_match dscp_mt_reg[] __read_mostly = {
    {
    .name		= "dscp",
    .family		= NFPROTO_IPV4,
    .checkentry	= dscp_mt_check,
    .match		= dscp_mt,
    .matchsize	= sizeof(struct xt_dscp_info),
    .me		= THIS_MODULE,
    },
    {
    .name		= "dscp",
    .family		= NFPROTO_IPV6,
    .checkentry	= dscp_mt_check,
    .match		= dscp_mt6,
    .matchsize	= sizeof(struct xt_dscp_info),
    .me		= THIS_MODULE,
    },
    {
    .name		= "tos",
    .revision	= 1,
    .family		= NFPROTO_IPV4,
    .checkentry	= tos_mt_check,
    .match		= tos_mt,
    .matchsize	= sizeof(struct xt_tos_match_info),
    .me		= THIS_MODULE,
    },
    {
    .name		= "tos",
    .revision	= 1,
    .family		= NFPROTO_IPV6,
    .checkentry	= tos_mt_check,
    .match		= tos_mt,
    .matchsize	= sizeof(struct xt_tos_match_info),
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn dscp_mt_init() -> int __init {
    static int __init dscp_mt_init(void)
    {
    return xt_register_matches(dscp_mt_reg, ARRAY_SIZE(dscp_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn dscp_mt_exit() -> void __exit {
    static void __exit dscp_mt_exit(void)
    {
    xt_unregister_matches(dscp_mt_reg, ARRAY_SIZE(dscp_mt_reg));
    }
    module_init(dscp_mt_init);
    module_exit(dscp_mt_exit);
