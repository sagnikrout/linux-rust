//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/ip6t_NPT.c
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
// Copyright (c) 2011, 2012 Patrick McHardy <kaber@trash.net>
//

#[no_mangle]
unsafe extern "C" fn ip6t_npt_checkentry(par: *const xt_tgchk_param) -> c_int {
    static int ip6t_npt_checkentry(const struct xt_tgchk_param *par)
    {
    struct ip6t_npt_tginfo *npt = par.targinfo;
    struct in6_addr pfx;
    __wsum src_sum, dst_sum;
    if (npt.src_pfx_len > 64 || npt.dst_pfx_len > 64)
    return -EINVAL;
// Ensure that LSB of prefix is zero
    ipv6_addr_prefix(&pfx, &npt.src_pfx.in6, npt.src_pfx_len);
    if (!ipv6_addr_equal(&pfx, &npt.src_pfx.in6))
    return -EINVAL;
    ipv6_addr_prefix(&pfx, &npt.dst_pfx.in6, npt.dst_pfx_len);
    if (!ipv6_addr_equal(&pfx, &npt.dst_pfx.in6))
    return -EINVAL;
    src_sum = csum_partial(&npt.src_pfx.in6, sizeof(npt.src_pfx.in6), 0);
    dst_sum = csum_partial(&npt.dst_pfx.in6, sizeof(npt.dst_pfx.in6), 0);
    npt.adjustment = ~csum_fold(csum_sub(src_sum, dst_sum));
    return 0;
    }
    static bool ip6t_npt_map_pfx(const struct ip6t_npt_tginfo *npt,
    struct in6_addr *addr)
    {
    unsigned int pfx_len;
    unsigned int i, idx;
    __be32 mask;
    __sum16 sum;
    pfx_len = max(npt.src_pfx_len, npt.dst_pfx_len);
    for (i = 0; i < pfx_len; i += 32) {
    if (pfx_len - i >= 32)
    mask = 0;
    else
    mask = htonl((1 << (i - pfx_len + 32)) - 1);
    idx = i / 32;
    addr.s6_addr32[idx] &= mask;
    addr.s6_addr32[idx] |= ~mask & npt.dst_pfx.in6.s6_addr32[idx];
    }
    if (pfx_len <= 48)
    idx = 3;
    else {
    for (idx = 4; idx < ARRAY_SIZE(addr.s6_addr16); idx++) {
    if (( __sum16)addr.s6_addr16[idx] !=
    CSUM_MANGLED_0)
    break;
    }
    if (idx == ARRAY_SIZE(addr.s6_addr16))
    return false;
    }
    sum = ~csum_fold(csum_add(csum_unfold(( __sum16)addr.s6_addr16[idx]),
    csum_unfold(npt.adjustment)));
    if (sum == CSUM_MANGLED_0)
    sum = 0;
// ( __sum16 *)&addr->s6_addr16[idx] = sum;
    return true;
    }
    static struct ipv6hdr *icmpv6_bounced_ipv6hdr(struct sk_buff *skb,
    struct ipv6hdr *_bounced_hdr)
    {
    if (ipv6_hdr(skb).nexthdr != IPPROTO_ICMPV6)
    return core::ptr::null_mut();
    if (!icmpv6_is_err(icmp6_hdr(skb).icmp6_type))
    return core::ptr::null_mut();
    return skb_header_pointer(skb,
    skb_transport_offset(skb) + sizeof(struct icmp6hdr),
    sizeof(struct ipv6hdr),
    _bounced_hdr);
    }
    static unsigned int
    ip6t_snpt_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct ip6t_npt_tginfo *npt = par.targinfo;
    struct ipv6hdr _bounced_hdr;
    struct ipv6hdr *bounced_hdr;
    struct in6_addr bounced_pfx;
    if (!ip6t_npt_map_pfx(npt, &ipv6_hdr(skb).saddr)) {
    icmpv6_send(skb, ICMPV6_PARAMPROB, ICMPV6_HDR_FIELD,
    offsetof(struct ipv6hdr, saddr));
    return NF_DROP;
    }
// rewrite dst addr of bounced packet which was sent to dst range
    bounced_hdr = icmpv6_bounced_ipv6hdr(skb, &_bounced_hdr);
    if (bounced_hdr) {
    ipv6_addr_prefix(&bounced_pfx, &bounced_hdr.daddr, npt.src_pfx_len);
    if (ipv6_addr_cmp(&bounced_pfx, &npt.src_pfx.in6) == 0)
    ip6t_npt_map_pfx(npt, &bounced_hdr.daddr);
    }
    return XT_CONTINUE;
    }
    static unsigned int
    ip6t_dnpt_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct ip6t_npt_tginfo *npt = par.targinfo;
    struct ipv6hdr _bounced_hdr;
    struct ipv6hdr *bounced_hdr;
    struct in6_addr bounced_pfx;
    if (!ip6t_npt_map_pfx(npt, &ipv6_hdr(skb).daddr)) {
    icmpv6_send(skb, ICMPV6_PARAMPROB, ICMPV6_HDR_FIELD,
    offsetof(struct ipv6hdr, daddr));
    return NF_DROP;
    }
// rewrite src addr of bounced packet which was sent from dst range
    bounced_hdr = icmpv6_bounced_ipv6hdr(skb, &_bounced_hdr);
    if (bounced_hdr) {
    ipv6_addr_prefix(&bounced_pfx, &bounced_hdr.saddr, npt.src_pfx_len);
    if (ipv6_addr_cmp(&bounced_pfx, &npt.src_pfx.in6) == 0)
    ip6t_npt_map_pfx(npt, &bounced_hdr.saddr);
    }
    return XT_CONTINUE;
    }
    static struct xt_target ip6t_npt_target_reg[] __read_mostly = {
    {
    .name		= "SNPT",
    .table		= "mangle",
    .target		= ip6t_snpt_tg,
    .targetsize	= sizeof(struct ip6t_npt_tginfo),
    .usersize	= offsetof(struct ip6t_npt_tginfo, adjustment),
    .checkentry	= ip6t_npt_checkentry,
    .family		= NFPROTO_IPV6,
    .hooks		= (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_POST_ROUTING),
    .me		= THIS_MODULE,
    },
    {
    .name		= "DNPT",
    .table		= "mangle",
    .target		= ip6t_dnpt_tg,
    .targetsize	= sizeof(struct ip6t_npt_tginfo),
    .usersize	= offsetof(struct ip6t_npt_tginfo, adjustment),
    .checkentry	= ip6t_npt_checkentry,
    .family		= NFPROTO_IPV6,
    .hooks		= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_OUT),
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn ip6t_npt_init() -> int __init {
    static int __init ip6t_npt_init(void)
    {
    return xt_register_targets(ip6t_npt_target_reg,
    ARRAY_SIZE(ip6t_npt_target_reg));
    }
#[no_mangle]
unsafe extern "C" fn ip6t_npt_exit() -> void __exit {
    static void __exit ip6t_npt_exit(void)
    {
    xt_unregister_targets(ip6t_npt_target_reg,
    ARRAY_SIZE(ip6t_npt_target_reg));
    }
    module_init(ip6t_npt_init);
    module_exit(ip6t_npt_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("IPv6-to-IPv6 Network Prefix Translation (RFC 6296)");
    MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
    MODULE_ALIAS("ip6t_SNPT");
    MODULE_ALIAS("ip6t_DNPT");
