//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_NFLOG.c
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
// Copyright (c) 2006 Patrick McHardy <kaber@trash.net>
//

    MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
    MODULE_DESCRIPTION("Xtables: packet logging to netlink using NFLOG");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_NFLOG");
    MODULE_ALIAS("ip6t_NFLOG");
    static unsigned int
    nflog_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_nflog_info *info = par.targinfo;
    struct net *net = xt_net(par);
    struct nf_loginfo li;
    li.type		     = NF_LOG_TYPE_ULOG;
    li.u.ulog.copy_len   = info.len;
    li.u.ulog.group	     = info.group;
    li.u.ulog.qthreshold = info.threshold;
    li.u.ulog.flags	     = 0;
    if (info.flags & XT_NFLOG_F_COPY_LEN)
    li.u.ulog.flags |= NF_LOG_F_COPY_LEN;
    nf_log_packet(net, xt_family(par), xt_hooknum(par), skb, xt_in(par),
    xt_out(par), &li, "%s", info.prefix);
    return XT_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn nflog_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int nflog_tg_check(const struct xt_tgchk_param *par)
    {
    const struct xt_nflog_info *info = par.targinfo;
    int ret;
    if (info.flags & ~XT_NFLOG_MASK)
    return -EINVAL;
    if (info.prefix[sizeof(info.prefix) - 1] != '\0')
    return -EINVAL;
    ret = nf_logger_find_get(par.family, NF_LOG_TYPE_ULOG);
    if (ret != 0 && !par.nft_compat) {
    request_module("%s", "nfnetlink_log");
    ret = nf_logger_find_get(par.family, NF_LOG_TYPE_ULOG);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nflog_tg_destroy(par: *const xt_tgdtor_param) {
    static void nflog_tg_destroy(const struct xt_tgdtor_param *par)
    {
    nf_logger_put(par.family, NF_LOG_TYPE_ULOG);
    }
    static struct xt_target nflog_tg_reg[] __read_mostly = {
    {
    .name       = "NFLOG",
    .revision   = 0,
    .family     = NFPROTO_IPV4,
    .checkentry = nflog_tg_check,
    .destroy    = nflog_tg_destroy,
    .target     = nflog_tg,
    .targetsize = sizeof(struct xt_nflog_info),
    .me         = THIS_MODULE,
    },

    {
    .name       = "NFLOG",
    .revision   = 0,
    .family     = NFPROTO_IPV6,
    .checkentry = nflog_tg_check,
    .destroy    = nflog_tg_destroy,
    .target     = nflog_tg,
    .targetsize = sizeof(struct xt_nflog_info),
    .me         = THIS_MODULE,
    },

    };
#[no_mangle]
unsafe extern "C" fn nflog_tg_init() -> int __init {
    static int __init nflog_tg_init(void)
    {
    return xt_register_targets(nflog_tg_reg, ARRAY_SIZE(nflog_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn nflog_tg_exit() -> void __exit {
    static void __exit nflog_tg_exit(void)
    {
    xt_unregister_targets(nflog_tg_reg, ARRAY_SIZE(nflog_tg_reg));
    }
    module_init(nflog_tg_init);
    module_exit(nflog_tg_exit);
    MODULE_SOFTDEP("pre: nfnetlink_log");
