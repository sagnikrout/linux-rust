//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_LOG.c
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
// This is a module which is used for logging packets.
//
// (C) 1999-2001 Paul `Rusty' Russell
// (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
//

    static unsigned int
    log_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_log_info *loginfo = par.targinfo;
    struct net *net = xt_net(par);
    struct nf_loginfo li;
    li.type = NF_LOG_TYPE_LOG;
    li.u.log.level = loginfo.level;
    li.u.log.logflags = loginfo.logflags;
    nf_log_packet(net, xt_family(par), xt_hooknum(par), skb, xt_in(par),
    xt_out(par), &li, "%s", loginfo.prefix);
    return XT_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn log_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int log_tg_check(const struct xt_tgchk_param *par)
    {
    const struct xt_log_info *loginfo = par.targinfo;
    int ret;
    if (par.family != NFPROTO_IPV4 && par.family != NFPROTO_IPV6)
    return -EINVAL;
    if (loginfo.level >= 8) {
    pr_info_ratelimited("level %u >= 8\n", loginfo.level);
    return -EINVAL;
    }
    if (loginfo.prefix[sizeof(loginfo.prefix)-1] != '\0') {
    pr_info_ratelimited("prefix is not null-terminated\n");
    return -EINVAL;
    }
    ret = nf_logger_find_get(par.family, NF_LOG_TYPE_LOG);
    if (ret != 0 && !par.nft_compat) {
    request_module("%s", "nf_log_syslog");
    ret = nf_logger_find_get(par.family, NF_LOG_TYPE_LOG);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn log_tg_destroy(par: *const xt_tgdtor_param) {
    static void log_tg_destroy(const struct xt_tgdtor_param *par)
    {
    nf_logger_put(par.family, NF_LOG_TYPE_LOG);
    }
    static struct xt_target log_tg_regs[] __read_mostly = {
    {
    .name		= "LOG",
    .family		= NFPROTO_IPV4,
    .target		= log_tg,
    .targetsize	= sizeof(struct xt_log_info),
    .checkentry	= log_tg_check,
    .destroy	= log_tg_destroy,
    .me		= THIS_MODULE,
    },

    {
    .name		= "LOG",
    .family		= NFPROTO_IPV6,
    .target		= log_tg,
    .targetsize	= sizeof(struct xt_log_info),
    .checkentry	= log_tg_check,
    .destroy	= log_tg_destroy,
    .me		= THIS_MODULE,
    },

    };
#[no_mangle]
unsafe extern "C" fn log_tg_init() -> int __init {
    static int __init log_tg_init(void)
    {
    return xt_register_targets(log_tg_regs, ARRAY_SIZE(log_tg_regs));
    }
#[no_mangle]
unsafe extern "C" fn log_tg_exit() -> void __exit {
    static void __exit log_tg_exit(void)
    {
    xt_unregister_targets(log_tg_regs, ARRAY_SIZE(log_tg_regs));
    }
    module_init(log_tg_init);
    module_exit(log_tg_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Netfilter Core Team <coreteam@netfilter.org>");
    MODULE_AUTHOR("Jan Rekorajski <baggins@pld.org.pl>");
    MODULE_DESCRIPTION("Xtables: IPv4/IPv6 packet logging");
    MODULE_ALIAS("ipt_LOG");
    MODULE_ALIAS("ip6t_LOG");
    MODULE_SOFTDEP("pre: nf_log_syslog");
