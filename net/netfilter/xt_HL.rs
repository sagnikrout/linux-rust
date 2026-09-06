//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_HL.c
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
// TTL modification target for IP tables
// (C) 2000,2005 by Harald Welte <laforge@netfilter.org>
//
// Hop Limit modification target for ip6tables
// Maciej Soltysiak <solt@dns.toxicfilms.tv>
//

    MODULE_AUTHOR("Harald Welte <laforge@netfilter.org>");
    MODULE_AUTHOR("Maciej Soltysiak <solt@dns.toxicfilms.tv>");
    MODULE_DESCRIPTION("Xtables: Hoplimit/TTL Limit field modification target");
    MODULE_LICENSE("GPL");
    static unsigned int
    ttl_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    struct iphdr *iph;
    const struct ipt_TTL_info *info = par.targinfo;
    int new_ttl;
    if (skb_ensure_writable(skb, sizeof(*iph)))
    return NF_DROP;
    iph = ip_hdr(skb);
    switch (info.mode) {
    case IPT_TTL_SET:
    new_ttl = info.ttl;
    break;
    case IPT_TTL_INC:
    new_ttl = iph.ttl + info.ttl;
    if (new_ttl > 255)
    new_ttl = 255;
    break;
    case IPT_TTL_DEC:
    new_ttl = iph.ttl - info.ttl;
    if (new_ttl < 0)
    new_ttl = 0;
    break;
    default:
    new_ttl = iph.ttl;
    break;
    }
    if (new_ttl != iph.ttl) {
    csum_replace2(&iph.check, htons(iph.ttl << 8),
    htons(new_ttl << 8));
    iph.ttl = new_ttl;
    }
    return XT_CONTINUE;
    }
    static unsigned int
    hl_tg6(struct sk_buff *skb, const struct xt_action_param *par)
    {
    struct ipv6hdr *ip6h;
    const struct ip6t_HL_info *info = par.targinfo;
    int new_hl;
    if (skb_ensure_writable(skb, sizeof(*ip6h)))
    return NF_DROP;
    ip6h = ipv6_hdr(skb);
    switch (info.mode) {
    case IP6T_HL_SET:
    new_hl = info.hop_limit;
    break;
    case IP6T_HL_INC:
    new_hl = ip6h.hop_limit + info.hop_limit;
    if (new_hl > 255)
    new_hl = 255;
    break;
    case IP6T_HL_DEC:
    new_hl = ip6h.hop_limit - info.hop_limit;
    if (new_hl < 0)
    new_hl = 0;
    break;
    default:
    new_hl = ip6h.hop_limit;
    break;
    }
    ip6h.hop_limit = new_hl;
    return XT_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn ttl_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int ttl_tg_check(const struct xt_tgchk_param *par)
    {
    const struct ipt_TTL_info *info = par.targinfo;
    if (info.mode > IPT_TTL_MAXMODE)
    return -EINVAL;
    if (info.mode != IPT_TTL_SET && info.ttl == 0)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hl_tg6_check(par: *const xt_tgchk_param) -> c_int {
    static int hl_tg6_check(const struct xt_tgchk_param *par)
    {
    const struct ip6t_HL_info *info = par.targinfo;
    if (info.mode > IP6T_HL_MAXMODE)
    return -EINVAL;
    if (info.mode != IP6T_HL_SET && info.hop_limit == 0)
    return -EINVAL;
    return 0;
    }
    static struct xt_target hl_tg_reg[] __read_mostly = {
    {
    .name       = "TTL",
    .revision   = 0,
    .family     = NFPROTO_IPV4,
    .target     = ttl_tg,
    .targetsize = sizeof(struct ipt_TTL_info),
    .table      = "mangle",
    .checkentry = ttl_tg_check,
    .me         = THIS_MODULE,
    },
    {
    .name       = "HL",
    .revision   = 0,
    .family     = NFPROTO_IPV6,
    .target     = hl_tg6,
    .targetsize = sizeof(struct ip6t_HL_info),
    .table      = "mangle",
    .checkentry = hl_tg6_check,
    .me         = THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn hl_tg_init() -> int __init {
    static int __init hl_tg_init(void)
    {
    return xt_register_targets(hl_tg_reg, ARRAY_SIZE(hl_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn hl_tg_exit() -> void __exit {
    static void __exit hl_tg_exit(void)
    {
    xt_unregister_targets(hl_tg_reg, ARRAY_SIZE(hl_tg_reg));
    }
    module_init(hl_tg_init);
    module_exit(hl_tg_exit);
    MODULE_ALIAS("ipt_TTL");
    MODULE_ALIAS("ip6t_HL");
