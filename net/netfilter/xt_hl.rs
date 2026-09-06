//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_hl.c
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
// IP tables module for matching the value of the TTL
// (C) 2000,2001 by Harald Welte <laforge@netfilter.org>
//
// Hop Limit matching module
// (C) 2001-2002 Maciej Soltysiak <solt@dns.toxicfilms.tv>
//

    MODULE_AUTHOR("Maciej Soltysiak <solt@dns.toxicfilms.tv>");
    MODULE_DESCRIPTION("Xtables: Hoplimit/TTL field match");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_ttl");
    MODULE_ALIAS("ip6t_hl");
#[no_mangle]
unsafe extern "C" fn ttl_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int ttl_mt_check(const struct xt_mtchk_param *par)
    {
    const struct ipt_ttl_info *info = par.matchinfo;
    if (info.mode > IPT_TTL_GT) {
    pr_info_ratelimited("Unknown TTL match mode: %d\n", info.mode);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttl_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool ttl_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct ipt_ttl_info *info = par.matchinfo;
    let mut ttl: u8 = ip_hdr(skb).ttl;
    switch (info.mode) {
    case IPT_TTL_EQ:
    let mut ttl: return = = info.ttl;
    case IPT_TTL_NE:
    return ttl != info.ttl;
    case IPT_TTL_LT:
    return ttl < info.ttl;
    case IPT_TTL_GT:
    return ttl > info.ttl;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn hl_mt6_check(par: *const xt_mtchk_param) -> c_int {
    static int hl_mt6_check(const struct xt_mtchk_param *par)
    {
    const struct ip6t_hl_info *info = par.matchinfo;
    if (info.mode > IP6T_HL_GT) {
    pr_info_ratelimited("Unknown Hop Limit match mode: %d\n", info.mode);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hl_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool hl_mt6(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct ip6t_hl_info *info = par.matchinfo;
    const struct ipv6hdr *ip6h = ipv6_hdr(skb);
    switch (info.mode) {
    case IP6T_HL_EQ:
    return ip6h.hop_limit == info.hop_limit;
    case IP6T_HL_NE:
    return ip6h.hop_limit != info.hop_limit;
    case IP6T_HL_LT:
    return ip6h.hop_limit < info.hop_limit;
    case IP6T_HL_GT:
    return ip6h.hop_limit > info.hop_limit;
    }
    return false;
    }
    static struct xt_match hl_mt_reg[] __read_mostly = {
    {
    .name       = "ttl",
    .revision   = 0,
    .family     = NFPROTO_IPV4,
    .checkentry = ttl_mt_check,
    .match      = ttl_mt,
    .matchsize  = sizeof(struct ipt_ttl_info),
    .me         = THIS_MODULE,
    },
    {
    .name       = "hl",
    .revision   = 0,
    .family     = NFPROTO_IPV6,
    .checkentry = hl_mt6_check,
    .match      = hl_mt6,
    .matchsize  = sizeof(struct ip6t_hl_info),
    .me         = THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn hl_mt_init() -> int __init {
    static int __init hl_mt_init(void)
    {
    return xt_register_matches(hl_mt_reg, ARRAY_SIZE(hl_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn hl_mt_exit() -> void __exit {
    static void __exit hl_mt_exit(void)
    {
    xt_unregister_matches(hl_mt_reg, ARRAY_SIZE(hl_mt_reg));
    }
    module_init(hl_mt_init);
    module_exit(hl_mt_exit);
