//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_CLASSIFY.c
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
// This is a module which is used for setting the skb->priority field
// of an skb for qdisc classification.
//
// (C) 2001-2002 Patrick McHardy <kaber@trash.net>
//

    MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Xtables: Qdisc classification");
    MODULE_ALIAS("ipt_CLASSIFY");
    MODULE_ALIAS("ip6t_CLASSIFY");
    MODULE_ALIAS("arpt_CLASSIFY");
    static unsigned int
    classify_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_classify_target_info *clinfo = par.targinfo;
    skb.priority = clinfo.priority;
    return XT_CONTINUE;
    }
    static struct xt_target classify_tg_reg[] __read_mostly = {
    {
    .name       = "CLASSIFY",
    .revision   = 0,
    .family     = NFPROTO_IPV4,
    .hooks      = (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_FORWARD) |
    (1 << NF_INET_POST_ROUTING),
    .target     = classify_tg,
    .targetsize = sizeof(struct xt_classify_target_info),
    .me         = THIS_MODULE,
    },
    {
    .name       = "CLASSIFY",
    .revision   = 0,
    .family     = NFPROTO_ARP,
    .hooks      = (1 << NF_ARP_OUT) | (1 << NF_ARP_FORWARD),
    .target     = classify_tg,
    .targetsize = sizeof(struct xt_classify_target_info),
    .me         = THIS_MODULE,
    },

    {
    .name       = "CLASSIFY",
    .revision   = 0,
    .family     = NFPROTO_IPV6,
    .hooks      = (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_FORWARD) |
    (1 << NF_INET_POST_ROUTING),
    .target     = classify_tg,
    .targetsize = sizeof(struct xt_classify_target_info),
    .me         = THIS_MODULE,
    },

    };
#[no_mangle]
unsafe extern "C" fn classify_tg_init() -> int __init {
    static int __init classify_tg_init(void)
    {
    return xt_register_targets(classify_tg_reg, ARRAY_SIZE(classify_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn classify_tg_exit() -> void __exit {
    static void __exit classify_tg_exit(void)
    {
    xt_unregister_targets(classify_tg_reg, ARRAY_SIZE(classify_tg_reg));
    }
    module_init(classify_tg_init);
    module_exit(classify_tg_exit);
