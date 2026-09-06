//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_connmark.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// xt_connmark - Netfilter module to operate on connection marks
//
// Copyright (C) 2002,2004 MARA Systems AB <https://www.marasystems.com>
// by Henrik Nordstrom <hno@marasystems.com>
// Copyright © CC Computer Consultants GmbH, 2007 - 2008
// Jan Engelhardt <jengelh@medozas.de>
//

    MODULE_AUTHOR("Henrik Nordstrom <hno@marasystems.com>");
    MODULE_DESCRIPTION("Xtables: connection mark operations");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_CONNMARK");
    MODULE_ALIAS("ip6t_CONNMARK");
    MODULE_ALIAS("ipt_connmark");
    MODULE_ALIAS("ip6t_connmark");
    static unsigned int
    connmark_tg_shift(struct sk_buff *skb, const struct xt_connmark_tginfo2 *info)
    {
    enum ip_conntrack_info ctinfo;
    u_int32_t new_targetmark;
    struct nf_conn *ct;
    u_int32_t newmark;
    u_int32_t oldmark;
    ct = nf_ct_get(skb, &ctinfo);
    if (ct == core::ptr::null_mut())
    return XT_CONTINUE;
    switch (info.mode) {
    case XT_CONNMARK_SET:
    oldmark = READ_ONCE(ct.mark);
    newmark = (oldmark & ~info.ctmask) ^ info.ctmark;
    if (info.shift_dir == D_SHIFT_RIGHT)
    newmark >>= info.shift_bits;
    else
    newmark <<= info.shift_bits;
    if (READ_ONCE(ct.mark) != newmark) {
    WRITE_ONCE(ct.mark, newmark);
    nf_conntrack_event_cache(IPCT_MARK, ct);
    }
    break;
    case XT_CONNMARK_SAVE:
    new_targetmark = (skb.mark & info.nfmask);
    if (info.shift_dir == D_SHIFT_RIGHT)
    new_targetmark >>= info.shift_bits;
    else
    new_targetmark <<= info.shift_bits;
    newmark = (READ_ONCE(ct.mark) & ~info.ctmask) ^
    new_targetmark;
    if (READ_ONCE(ct.mark) != newmark) {
    WRITE_ONCE(ct.mark, newmark);
    nf_conntrack_event_cache(IPCT_MARK, ct);
    }
    break;
    case XT_CONNMARK_RESTORE:
    new_targetmark = (READ_ONCE(ct.mark) & info.ctmask);
    if (info.shift_dir == D_SHIFT_RIGHT)
    new_targetmark >>= info.shift_bits;
    else
    new_targetmark <<= info.shift_bits;
    newmark = (skb.mark & ~info.nfmask) ^
    new_targetmark;
    skb.mark = newmark;
    break;
    }
    return XT_CONTINUE;
    }
    static unsigned int
    connmark_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_connmark_tginfo1 *info = par.targinfo;
    const struct xt_connmark_tginfo2 info2 = {
    .ctmark	= info.ctmark,
    .ctmask	= info.ctmask,
    .nfmask	= info.nfmask,
    .mode	= info.mode,
    };
    return connmark_tg_shift(skb, &info2);
    }
    static unsigned int
    connmark_tg_v2(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_connmark_tginfo2 *info = par.targinfo;
    return connmark_tg_shift(skb, info);
    }
#[no_mangle]
unsafe extern "C" fn connmark_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int connmark_tg_check(const struct xt_tgchk_param *par)
    {
    int ret;
    ret = nf_ct_netns_get(par.net, par.family);
    if (ret < 0)
    pr_info_ratelimited("cannot load conntrack support for proto=%u\n",
    par.family);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn connmark_tg_check_v2(par: *const xt_tgchk_param) -> c_int {
    static int connmark_tg_check_v2(const struct xt_tgchk_param *par)
    {
    const struct xt_connmark_tginfo2 *info = par.targinfo;
    if (info.shift_dir > D_SHIFT_RIGHT || info.shift_bits >= 32)
    return -EINVAL;
    return connmark_tg_check(par);
    }
#[no_mangle]
unsafe extern "C" fn connmark_tg_destroy(par: *const xt_tgdtor_param) {
    static void connmark_tg_destroy(const struct xt_tgdtor_param *par)
    {
    nf_ct_netns_put(par.net, par.family);
    }
    static bool
    connmark_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_connmark_mtinfo1 *info = par.matchinfo;
    enum ip_conntrack_info ctinfo;
    const struct nf_conn *ct;
    ct = nf_ct_get(skb, &ctinfo);
    if (ct == core::ptr::null_mut())
    return false;
    return ((READ_ONCE(ct.mark) & info.mask) == info.mark) ^ info.invert;
    }
#[no_mangle]
unsafe extern "C" fn connmark_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int connmark_mt_check(const struct xt_mtchk_param *par)
    {
    int ret;
    ret = nf_ct_netns_get(par.net, par.family);
    if (ret < 0)
    pr_info_ratelimited("cannot load conntrack support for proto=%u\n",
    par.family);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn connmark_mt_destroy(par: *const xt_mtdtor_param) {
    static void connmark_mt_destroy(const struct xt_mtdtor_param *par)
    {
    nf_ct_netns_put(par.net, par.family);
    }
    static struct xt_target connmark_tg_reg[] __read_mostly = {
    {
    .name           = "CONNMARK",
    .revision       = 1,
    .family         = NFPROTO_IPV4,
    .checkentry     = connmark_tg_check,
    .target         = connmark_tg,
    .targetsize     = sizeof(struct xt_connmark_tginfo1),
    .destroy        = connmark_tg_destroy,
    .me             = THIS_MODULE,
    },
    {
    .name           = "CONNMARK",
    .revision       = 2,
    .family         = NFPROTO_IPV4,
    .checkentry     = connmark_tg_check_v2,
    .target         = connmark_tg_v2,
    .targetsize     = sizeof(struct xt_connmark_tginfo2),
    .destroy        = connmark_tg_destroy,
    .me             = THIS_MODULE,
    },

    {
    .name           = "CONNMARK",
    .revision       = 1,
    .family         = NFPROTO_IPV6,
    .checkentry     = connmark_tg_check,
    .target         = connmark_tg,
    .targetsize     = sizeof(struct xt_connmark_tginfo1),
    .destroy        = connmark_tg_destroy,
    .me             = THIS_MODULE,
    },
    {
    .name           = "CONNMARK",
    .revision       = 2,
    .family         = NFPROTO_IPV6,
    .checkentry     = connmark_tg_check_v2,
    .target         = connmark_tg_v2,
    .targetsize     = sizeof(struct xt_connmark_tginfo2),
    .destroy        = connmark_tg_destroy,
    .me             = THIS_MODULE,
    },

    };
    static struct xt_match connmark_mt_reg __read_mostly = {
    .name           = "connmark",
    .revision       = 1,
    .family         = NFPROTO_UNSPEC,
    .checkentry     = connmark_mt_check,
    .match          = connmark_mt,
    .matchsize      = sizeof(struct xt_connmark_mtinfo1),
    .destroy        = connmark_mt_destroy,
    .me             = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn connmark_mt_init() -> int __init {
    static int __init connmark_mt_init(void)
    {
    int ret;
    ret = xt_register_targets(connmark_tg_reg,
    ARRAY_SIZE(connmark_tg_reg));
    if (ret < 0)
    return ret;
    ret = xt_register_match(&connmark_mt_reg);
    if (ret < 0) {
    xt_unregister_targets(connmark_tg_reg,
    ARRAY_SIZE(connmark_tg_reg));
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn connmark_mt_exit() -> void __exit {
    static void __exit connmark_mt_exit(void)
    {
    xt_unregister_match(&connmark_mt_reg);
    xt_unregister_targets(connmark_tg_reg, ARRAY_SIZE(connmark_tg_reg));
    }
    module_init(connmark_mt_init);
    module_exit(connmark_mt_exit);
