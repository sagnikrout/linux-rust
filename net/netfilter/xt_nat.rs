//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_nat.c
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
// (C) 2011 Patrick McHardy <kaber@trash.net>
//

#[no_mangle]
unsafe extern "C" fn xt_nat_checkentry_v0(par: *const xt_tgchk_param) -> c_int {
    static int xt_nat_checkentry_v0(const struct xt_tgchk_param *par)
    {
    const struct nf_nat_ipv4_multi_range_compat *mr = par.targinfo;
    if (mr.rangesize != 1) {
    pr_info_ratelimited("multiple ranges no longer supported\n");
    return -EINVAL;
    }
    return nf_ct_netns_get(par.net, par.family);
    }
#[no_mangle]
unsafe extern "C" fn xt_nat_checkentry(par: *const xt_tgchk_param) -> c_int {
    static int xt_nat_checkentry(const struct xt_tgchk_param *par)
    {
    switch (par.family) {
    case NFPROTO_IPV4:
    case NFPROTO_IPV6:
    case NFPROTO_INET:
    break;
    default:
    return -EINVAL;
    }
    return nf_ct_netns_get(par.net, par.family);
    }
#[no_mangle]
unsafe extern "C" fn xt_nat_destroy(par: *const xt_tgdtor_param) {
    static void xt_nat_destroy(const struct xt_tgdtor_param *par)
    {
    nf_ct_netns_put(par.net, par.family);
    }
    static void xt_nat_convert_range(struct nf_nat_range2 *dst,
    const struct nf_nat_ipv4_range *src)
    {
    memset(&dst.min_addr, 0, sizeof(dst.min_addr));
    memset(&dst.max_addr, 0, sizeof(dst.max_addr));
    memset(&dst.base_proto, 0, sizeof(dst.base_proto));
    dst.flags	 = src.flags;
    dst.min_addr.ip = src.min_ip;
    dst.max_addr.ip = src.max_ip;
    dst.min_proto	 = src.min;
    dst.max_proto	 = src.max;
    }
    static unsigned int
    xt_snat_target_v0(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct nf_nat_ipv4_multi_range_compat *mr = par.targinfo;
    struct nf_nat_range2 range;
    enum ip_conntrack_info ctinfo;
    struct nf_conn *ct;
    ct = nf_ct_get(skb, &ctinfo);
    WARN_ON(!(ct != core::ptr::null_mut() &&
    (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED ||
    ctinfo == IP_CT_RELATED_REPLY)));
    xt_nat_convert_range(&range, &mr.range[0]);
    return nf_nat_setup_info(ct, &range, NF_NAT_MANIP_SRC);
    }
    static unsigned int
    xt_dnat_target_v0(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct nf_nat_ipv4_multi_range_compat *mr = par.targinfo;
    struct nf_nat_range2 range;
    enum ip_conntrack_info ctinfo;
    struct nf_conn *ct;
    ct = nf_ct_get(skb, &ctinfo);
    WARN_ON(!(ct != core::ptr::null_mut() &&
    (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED)));
    xt_nat_convert_range(&range, &mr.range[0]);
    return nf_nat_setup_info(ct, &range, NF_NAT_MANIP_DST);
    }
    static unsigned int
    xt_snat_target_v1(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct nf_nat_range *range_v1 = par.targinfo;
    struct nf_nat_range2 range;
    enum ip_conntrack_info ctinfo;
    struct nf_conn *ct;
    ct = nf_ct_get(skb, &ctinfo);
    WARN_ON(!(ct != core::ptr::null_mut() &&
    (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED ||
    ctinfo == IP_CT_RELATED_REPLY)));
    memcpy(&range, range_v1, sizeof(*range_v1));
    memset(&range.base_proto, 0, sizeof(range.base_proto));
    return nf_nat_setup_info(ct, &range, NF_NAT_MANIP_SRC);
    }
    static unsigned int
    xt_dnat_target_v1(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct nf_nat_range *range_v1 = par.targinfo;
    struct nf_nat_range2 range;
    enum ip_conntrack_info ctinfo;
    struct nf_conn *ct;
    ct = nf_ct_get(skb, &ctinfo);
    WARN_ON(!(ct != core::ptr::null_mut() &&
    (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED)));
    memcpy(&range, range_v1, sizeof(*range_v1));
    memset(&range.base_proto, 0, sizeof(range.base_proto));
    return nf_nat_setup_info(ct, &range, NF_NAT_MANIP_DST);
    }
    static unsigned int
    xt_snat_target_v2(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct nf_nat_range2 *range = par.targinfo;
    enum ip_conntrack_info ctinfo;
    struct nf_conn *ct;
    ct = nf_ct_get(skb, &ctinfo);
    WARN_ON(!(ct != core::ptr::null_mut() &&
    (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED ||
    ctinfo == IP_CT_RELATED_REPLY)));
    return nf_nat_setup_info(ct, range, NF_NAT_MANIP_SRC);
    }
    static unsigned int
    xt_dnat_target_v2(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct nf_nat_range2 *range = par.targinfo;
    enum ip_conntrack_info ctinfo;
    struct nf_conn *ct;
    ct = nf_ct_get(skb, &ctinfo);
    WARN_ON(!(ct != core::ptr::null_mut() &&
    (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED)));
    return nf_nat_setup_info(ct, range, NF_NAT_MANIP_DST);
    }
    static struct xt_target xt_nat_target_reg[] __read_mostly = {
    {
    .name		= "SNAT",
    .revision	= 0,
    .checkentry	= xt_nat_checkentry_v0,
    .destroy	= xt_nat_destroy,
    .target		= xt_snat_target_v0,
    .targetsize	= sizeof(struct nf_nat_ipv4_multi_range_compat),
    .family		= NFPROTO_IPV4,
    .table		= "nat",
    .hooks		= (1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    .me		= THIS_MODULE,
    },
    {
    .name		= "DNAT",
    .revision	= 0,
    .checkentry	= xt_nat_checkentry_v0,
    .destroy	= xt_nat_destroy,
    .target		= xt_dnat_target_v0,
    .targetsize	= sizeof(struct nf_nat_ipv4_multi_range_compat),
    .family		= NFPROTO_IPV4,
    .table		= "nat",
    .hooks		= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_OUT),
    .me		= THIS_MODULE,
    },
    {
    .name		= "SNAT",
    .revision	= 1,
    .checkentry	= xt_nat_checkentry,
    .destroy	= xt_nat_destroy,
    .target		= xt_snat_target_v1,
    .targetsize	= sizeof(struct nf_nat_range),
    .table		= "nat",
    .hooks		= (1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    .me		= THIS_MODULE,
    },
    {
    .name		= "DNAT",
    .revision	= 1,
    .checkentry	= xt_nat_checkentry,
    .destroy	= xt_nat_destroy,
    .target		= xt_dnat_target_v1,
    .targetsize	= sizeof(struct nf_nat_range),
    .table		= "nat",
    .hooks		= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_OUT),
    .me		= THIS_MODULE,
    },
    {
    .name		= "SNAT",
    .revision	= 2,
    .checkentry	= xt_nat_checkentry,
    .destroy	= xt_nat_destroy,
    .target		= xt_snat_target_v2,
    .targetsize	= sizeof(struct nf_nat_range2),
    .table		= "nat",
    .hooks		= (1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    .me		= THIS_MODULE,
    },
    {
    .name		= "DNAT",
    .revision	= 2,
    .checkentry	= xt_nat_checkentry,
    .destroy	= xt_nat_destroy,
    .target		= xt_dnat_target_v2,
    .targetsize	= sizeof(struct nf_nat_range2),
    .table		= "nat",
    .hooks		= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_OUT),
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn xt_nat_init() -> int __init {
    static int __init xt_nat_init(void)
    {
    return xt_register_targets(xt_nat_target_reg,
    ARRAY_SIZE(xt_nat_target_reg));
    }
#[no_mangle]
unsafe extern "C" fn xt_nat_exit() -> void __exit {
    static void __exit xt_nat_exit(void)
    {
    xt_unregister_targets(xt_nat_target_reg, ARRAY_SIZE(xt_nat_target_reg));
    }
    module_init(xt_nat_init);
    module_exit(xt_nat_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
    MODULE_ALIAS("ipt_SNAT");
    MODULE_ALIAS("ipt_DNAT");
    MODULE_ALIAS("ip6t_SNAT");
    MODULE_ALIAS("ip6t_DNAT");
    MODULE_DESCRIPTION("SNAT and DNAT targets support");
