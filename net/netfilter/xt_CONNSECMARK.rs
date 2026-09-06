//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_CONNSECMARK.c
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
// This module is used to copy security markings from packets
// to connections, and restore security markings from connections
// back to packets.  This would normally be performed in conjunction
// with the SECMARK target and state match.
//
// Based somewhat on CONNMARK:
// Copyright (C) 2002,2004 MARA Systems AB <https://www.marasystems.com>
// by Henrik Nordstrom <hno@marasystems.com>
//
// (C) 2006,2008 Red Hat, Inc., James Morris <jmorris@redhat.com>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("James Morris <jmorris@redhat.com>");
    MODULE_DESCRIPTION("Xtables: target for copying between connection and security mark");
    MODULE_ALIAS("ipt_CONNSECMARK");
    MODULE_ALIAS("ip6t_CONNSECMARK");
//
// If the packet has a security mark and the connection does not, copy
// the security mark from the packet to the connection.
//
#[no_mangle]
unsafe extern "C" fn secmark_save(skb: *const sk_buff) {
    static void secmark_save(const struct sk_buff *skb)
    {
    if (skb.secmark) {
    struct nf_conn *ct;
    enum ip_conntrack_info ctinfo;
    ct = nf_ct_get(skb, &ctinfo);
    if (ct && !ct.secmark) {
    ct.secmark = skb.secmark;
    nf_conntrack_event_cache(IPCT_SECMARK, ct);
    }
    }
    }
//
// If packet has no security mark, and the connection does, restore the
// security mark from the connection to the packet.
//
#[no_mangle]
unsafe extern "C" fn secmark_restore(skb: *mut sk_buff) {
    static void secmark_restore(struct sk_buff *skb)
    {
    if (!skb.secmark) {
    const struct nf_conn *ct;
    enum ip_conntrack_info ctinfo;
    ct = nf_ct_get(skb, &ctinfo);
    if (ct && ct.secmark)
    skb.secmark = ct.secmark;
    }
    }
    static unsigned int
    connsecmark_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_connsecmark_target_info *info = par.targinfo;
    switch (info.mode) {
    case CONNSECMARK_SAVE:
    secmark_save(skb);
    break;
    case CONNSECMARK_RESTORE:
    secmark_restore(skb);
    break;
    default:
    BUG();
    }
    return XT_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn connsecmark_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int connsecmark_tg_check(const struct xt_tgchk_param *par)
    {
    const struct xt_connsecmark_target_info *info = par.targinfo;
    int ret;
    if (strcmp(par.table, "mangle") != 0 &&
    strcmp(par.table, "security") != 0) {
    pr_info_ratelimited("only valid in \'mangle\' or \'security\' table, not \'%s\'\n",
    par.table);
    return -EINVAL;
    }
    switch (info.mode) {
    case CONNSECMARK_SAVE:
    case CONNSECMARK_RESTORE:
    break;
    default:
    pr_info_ratelimited("invalid mode: %hu\n", info.mode);
    return -EINVAL;
    }
    ret = nf_ct_netns_get(par.net, par.family);
    if (ret < 0)
    pr_info_ratelimited("cannot load conntrack support for proto=%u\n",
    par.family);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn connsecmark_tg_destroy(par: *const xt_tgdtor_param) {
    static void connsecmark_tg_destroy(const struct xt_tgdtor_param *par)
    {
    nf_ct_netns_put(par.net, par.family);
    }
    static struct xt_target connsecmark_tg_reg[] __read_mostly = {
    {
    .name       = "CONNSECMARK",
    .revision   = 0,
    .family     = NFPROTO_IPV4,
    .checkentry = connsecmark_tg_check,
    .destroy    = connsecmark_tg_destroy,
    .target     = connsecmark_tg,
    .targetsize = sizeof(struct xt_connsecmark_target_info),
    .me         = THIS_MODULE,
    },

    {
    .name       = "CONNSECMARK",
    .revision   = 0,
    .family     = NFPROTO_IPV6,
    .checkentry = connsecmark_tg_check,
    .destroy    = connsecmark_tg_destroy,
    .target     = connsecmark_tg,
    .targetsize = sizeof(struct xt_connsecmark_target_info),
    .me         = THIS_MODULE,
    },

    };
#[no_mangle]
unsafe extern "C" fn connsecmark_tg_init() -> int __init {
    static int __init connsecmark_tg_init(void)
    {
    return xt_register_targets(connsecmark_tg_reg, ARRAY_SIZE(connsecmark_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn connsecmark_tg_exit() -> void __exit {
    static void __exit connsecmark_tg_exit(void)
    {
    xt_unregister_targets(connsecmark_tg_reg, ARRAY_SIZE(connsecmark_tg_reg));
    }
    module_init(connsecmark_tg_init);
    module_exit(connsecmark_tg_exit);
