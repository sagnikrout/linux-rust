//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_CHECKSUM.c
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
// iptables module for the packet checksum mangling
//
// (C) 2002 by Harald Welte <laforge@netfilter.org>
// (C) 2010 Red Hat, Inc.
//
// Author: Michael S. Tsirkin <mst@redhat.com>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Michael S. Tsirkin <mst@redhat.com>");
    MODULE_DESCRIPTION("Xtables: checksum modification");
    MODULE_ALIAS("ipt_CHECKSUM");
    MODULE_ALIAS("ip6t_CHECKSUM");
    static unsigned int
    checksum_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    if (skb.ip_summed == CHECKSUM_PARTIAL && !skb_is_gso(skb))
    skb_checksum_help(skb);
    return XT_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn checksum_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int checksum_tg_check(const struct xt_tgchk_param *par)
    {
    const struct xt_CHECKSUM_info *einfo = par.targinfo;
    const struct ip6t_ip6 *i6 = par.entryinfo;
    const struct ipt_ip *i4 = par.entryinfo;
    if (einfo.operation & ~XT_CHECKSUM_OP_FILL) {
    pr_info_ratelimited("unsupported CHECKSUM operation %x\n",
    einfo.operation);
    return -EINVAL;
    }
    if (!einfo.operation)
    return -EINVAL;
    switch (par.family) {
    case NFPROTO_IPV4:
    if (i4.proto == IPPROTO_UDP &&
    (i4.invflags & XT_INV_PROTO) == 0)
    return 0;
    break;
    case NFPROTO_IPV6:
    if ((i6.flags & IP6T_F_PROTO) &&
    i6.proto == IPPROTO_UDP &&
    (i6.invflags & XT_INV_PROTO) == 0)
    return 0;
    break;
    }
    pr_warn_once("CHECKSUM should be avoided.  If really needed, restrict with \"-p udp\" and only use in OUTPUT\n");
    return 0;
    }
    static struct xt_target checksum_tg_reg[] __read_mostly = {
    {
    .name		= "CHECKSUM",
    .family		= NFPROTO_IPV4,
    .target		= checksum_tg,
    .targetsize	= sizeof(struct xt_CHECKSUM_info),
    .table		= "mangle",
    .checkentry	= checksum_tg_check,
    .me		= THIS_MODULE,
    },

    {
    .name		= "CHECKSUM",
    .family		= NFPROTO_IPV6,
    .target		= checksum_tg,
    .targetsize	= sizeof(struct xt_CHECKSUM_info),
    .table		= "mangle",
    .checkentry	= checksum_tg_check,
    .me		= THIS_MODULE,
    },

    };
#[no_mangle]
unsafe extern "C" fn checksum_tg_init() -> int __init {
    static int __init checksum_tg_init(void)
    {
    return xt_register_targets(checksum_tg_reg, ARRAY_SIZE(checksum_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn checksum_tg_exit() -> void __exit {
    static void __exit checksum_tg_exit(void)
    {
    xt_unregister_targets(checksum_tg_reg, ARRAY_SIZE(checksum_tg_reg));
    }
    module_init(checksum_tg_init);
    module_exit(checksum_tg_exit);
