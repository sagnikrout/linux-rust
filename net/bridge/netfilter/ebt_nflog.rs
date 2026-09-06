//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_nflog.c
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
// ebt_nflog
//
// Author:
// Peter Warasin <peter@endian.com>
//
// February, 2008
//
// Based on:
// xt_NFLOG.c, (C) 2006 by Patrick McHardy <kaber@trash.net>
// ebt_ulog.c, (C) 2004 by Bart De Schuymer <bdschuym@pandora.be>
//

    static unsigned int
    ebt_nflog_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct ebt_nflog_info *info = par.targinfo;
    struct net *net = xt_net(par);
    struct nf_loginfo li;
    li.type = NF_LOG_TYPE_ULOG;
    li.u.ulog.copy_len = info.len;
    li.u.ulog.group = info.group;
    li.u.ulog.qthreshold = info.threshold;
    li.u.ulog.flags = 0;
    nf_log_packet(net, PF_BRIDGE, xt_hooknum(par), skb, xt_in(par),
    xt_out(par), &li, "%s", info.prefix);
    return EBT_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn ebt_nflog_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int ebt_nflog_tg_check(const struct xt_tgchk_param *par)
    {
    struct ebt_nflog_info *info = par.targinfo;
    int ret;
    if (info.flags & ~EBT_NFLOG_MASK)
    return -EINVAL;
    info.prefix[EBT_NFLOG_PREFIX_SIZE - 1] = '\0';
    ret = nf_logger_find_get(par.family, NF_LOG_TYPE_ULOG);
    if (ret != 0 && !par.nft_compat) {
    request_module("%s", "nfnetlink_log");
    ret = nf_logger_find_get(par.family, NF_LOG_TYPE_ULOG);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ebt_nflog_tg_destroy(par: *const xt_tgdtor_param) {
    static void ebt_nflog_tg_destroy(const struct xt_tgdtor_param *par)
    {
    nf_logger_put(par.family, NF_LOG_TYPE_ULOG);
    }
    static struct xt_target ebt_nflog_tg_reg __read_mostly = {
    .name       = "nflog",
    .revision   = 0,
    .family     = NFPROTO_BRIDGE,
    .target     = ebt_nflog_tg,
    .checkentry = ebt_nflog_tg_check,
    .destroy    = ebt_nflog_tg_destroy,
    .targetsize = sizeof(struct ebt_nflog_info),
    .me         = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_nflog_init() -> int __init {
    static int __init ebt_nflog_init(void)
    {
    return xt_register_target(&ebt_nflog_tg_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_nflog_fini() -> void __exit {
    static void __exit ebt_nflog_fini(void)
    {
    xt_unregister_target(&ebt_nflog_tg_reg);
    }
    module_init(ebt_nflog_init);
    module_exit(ebt_nflog_fini);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Peter Warasin <peter@endian.com>");
    MODULE_DESCRIPTION("ebtables NFLOG netfilter logging module");
