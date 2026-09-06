//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_mark.c
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
// xt_mark - Netfilter module to match NFMARK value
//
// (C) 1999-2001 Marc Boucher <marc@mbsi.ca>
// Copyright © CC Computer Consultants GmbH, 2007 - 2008
// Jan Engelhardt <jengelh@medozas.de>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Marc Boucher <marc@mbsi.ca>");
    MODULE_DESCRIPTION("Xtables: packet mark operations");
    MODULE_ALIAS("ipt_mark");
    MODULE_ALIAS("ip6t_mark");
    MODULE_ALIAS("ipt_MARK");
    MODULE_ALIAS("ip6t_MARK");
    MODULE_ALIAS("arpt_MARK");
    static unsigned int
    mark_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_mark_tginfo2 *info = par.targinfo;
    skb.mark = (skb.mark & ~info.mask) ^ info.mark;
    return XT_CONTINUE;
    }
    static bool
    mark_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_mark_mtinfo1 *info = par.matchinfo;
    return ((skb.mark & info.mask) == info.mark) ^ info.invert;
    }
    static struct xt_target mark_tg_reg[] __read_mostly = {
    {
    .name           = "MARK",
    .revision       = 2,
    .family         = NFPROTO_IPV4,
    .target         = mark_tg,
    .targetsize     = sizeof(struct xt_mark_tginfo2),
    .me             = THIS_MODULE,
    },

    {
    .name           = "MARK",
    .revision       = 2,
    .family         = NFPROTO_ARP,
    .target         = mark_tg,
    .targetsize     = sizeof(struct xt_mark_tginfo2),
    .me             = THIS_MODULE,
    },

    {
    .name           = "MARK",
    .revision       = 2,
    .family         = NFPROTO_IPV6,
    .target         = mark_tg,
    .targetsize     = sizeof(struct xt_mark_tginfo2),
    .me             = THIS_MODULE,
    },

    };
    static struct xt_match mark_mt_reg __read_mostly = {
    .name           = "mark",
    .revision       = 1,
    .family         = NFPROTO_UNSPEC,
    .match          = mark_mt,
    .matchsize      = sizeof(struct xt_mark_mtinfo1),
    .me             = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn mark_mt_init() -> int __init {
    static int __init mark_mt_init(void)
    {
    int ret;
    ret = xt_register_targets(mark_tg_reg, ARRAY_SIZE(mark_tg_reg));
    if (ret < 0)
    return ret;
    ret = xt_register_match(&mark_mt_reg);
    if (ret < 0) {
    xt_unregister_targets(mark_tg_reg, ARRAY_SIZE(mark_tg_reg));
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mark_mt_exit() -> void __exit {
    static void __exit mark_mt_exit(void)
    {
    xt_unregister_match(&mark_mt_reg);
    xt_unregister_targets(mark_tg_reg, ARRAY_SIZE(mark_tg_reg));
    }
    module_init(mark_mt_init);
    module_exit(mark_mt_exit);
