//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_iprange.c
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
// xt_iprange - Netfilter module to match IP address ranges
//
// (C) 2003 Jozsef Kadlecsik <kadlec@netfilter.org>
// (C) CC Computer Consultants GmbH, 2008
//

    static bool
    iprange_mt4(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_iprange_mtinfo *info = par.matchinfo;
    const struct iphdr *iph = ip_hdr(skb);
    bool m;
    if (info.flags & IPRANGE_SRC) {
    m  = ntohl(iph.saddr) < ntohl(info.src_min.ip);
    m |= ntohl(iph.saddr) > ntohl(info.src_max.ip);
    m ^= !!(info.flags & IPRANGE_SRC_INV);
    if (m)
    return false;
    }
    if (info.flags & IPRANGE_DST) {
    m  = ntohl(iph.daddr) < ntohl(info.dst_min.ip);
    m |= ntohl(iph.daddr) > ntohl(info.dst_max.ip);
    m ^= !!(info.flags & IPRANGE_DST_INV);
    if (m)
    return false;
    }
    return true;
    }
    static inline int
    iprange_ipv6_lt(const struct in6_addr *a, const struct in6_addr *b)
    {
    unsigned int i;
    for (i = 0; i < 4; ++i) {
    if (a.s6_addr32[i] != b.s6_addr32[i])
    return ntohl(a.s6_addr32[i]) < ntohl(b.s6_addr32[i]);
    }
    return 0;
    }
    static bool
    iprange_mt6(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_iprange_mtinfo *info = par.matchinfo;
    const struct ipv6hdr *iph = ipv6_hdr(skb);
    bool m;
    if (info.flags & IPRANGE_SRC) {
    m  = iprange_ipv6_lt(&iph.saddr, &info.src_min.in6);
    m |= iprange_ipv6_lt(&info.src_max.in6, &iph.saddr);
    m ^= !!(info.flags & IPRANGE_SRC_INV);
    if (m)
    return false;
    }
    if (info.flags & IPRANGE_DST) {
    m  = iprange_ipv6_lt(&iph.daddr, &info.dst_min.in6);
    m |= iprange_ipv6_lt(&info.dst_max.in6, &iph.daddr);
    m ^= !!(info.flags & IPRANGE_DST_INV);
    if (m)
    return false;
    }
    return true;
    }
    static struct xt_match iprange_mt_reg[] __read_mostly = {
    {
    .name      = "iprange",
    .revision  = 1,
    .family    = NFPROTO_IPV4,
    .match     = iprange_mt4,
    .matchsize = sizeof(struct xt_iprange_mtinfo),
    .me        = THIS_MODULE,
    },
    {
    .name      = "iprange",
    .revision  = 1,
    .family    = NFPROTO_IPV6,
    .match     = iprange_mt6,
    .matchsize = sizeof(struct xt_iprange_mtinfo),
    .me        = THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn iprange_mt_init() -> int __init {
    static int __init iprange_mt_init(void)
    {
    return xt_register_matches(iprange_mt_reg, ARRAY_SIZE(iprange_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn iprange_mt_exit() -> void __exit {
    static void __exit iprange_mt_exit(void)
    {
    xt_unregister_matches(iprange_mt_reg, ARRAY_SIZE(iprange_mt_reg));
    }
    module_init(iprange_mt_init);
    module_exit(iprange_mt_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jozsef Kadlecsik <kadlec@netfilter.org>");
    MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
    MODULE_DESCRIPTION("Xtables: arbitrary IPv4 range matching");
    MODULE_ALIAS("ipt_iprange");
    MODULE_ALIAS("ip6t_iprange");
