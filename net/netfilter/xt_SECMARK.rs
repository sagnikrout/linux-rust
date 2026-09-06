//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_SECMARK.c
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
// Module for modifying the secmark field of the skb, for use by
// security subsystems.
//
// Based on the nfmark match by:
// (C) 1999-2001 Marc Boucher <marc@mbsi.ca>
//
// (C) 2006,2008 Red Hat, Inc., James Morris <jmorris@redhat.com>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("James Morris <jmorris@redhat.com>");
    MODULE_DESCRIPTION("Xtables: packet security mark modification");
    MODULE_ALIAS("ipt_SECMARK");
    MODULE_ALIAS("ip6t_SECMARK");
    static u8 mode;
    static unsigned int
    secmark_tg(struct sk_buff *skb, const struct xt_secmark_target_info_v1 *info)
    {
    let mut secmark: u32 = 0;
    switch (mode) {
    case SECMARK_MODE_SEL:
    secmark = info.secid;
    break;
    default:
    BUG();
    }
    skb.secmark = secmark;
    return XT_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn checkentry_lsm(info: *mut xt_secmark_target_info_v1) -> c_int {
    static int checkentry_lsm(struct xt_secmark_target_info_v1 *info)
    {
    int err;
    info.secctx[SECMARK_SECCTX_MAX - 1] = '\0';
    info.secid = 0;
    err = security_secctx_to_secid(info.secctx, strlen(info.secctx),
    &info.secid);
    if (err) {
    if (err == -EINVAL)
    pr_info_ratelimited("invalid security context \'%s\'\n",
    info.secctx);
    return err;
    }
    if (!info.secid) {
    pr_info_ratelimited("unable to map security context \'%s\'\n",
    info.secctx);
    return -ENOENT;
    }
    err = security_secmark_relabel_packet(info.secid);
    if (err) {
    pr_info_ratelimited("unable to obtain relabeling permission\n");
    return err;
    }
    security_secmark_refcount_inc();
    return 0;
    }
    static int
    secmark_tg_check(const char *table, struct xt_secmark_target_info_v1 *info)
    {
    int err;
    if (strcmp(table, "mangle") != 0 &&
    strcmp(table, "security") != 0) {
    pr_info_ratelimited("only valid in \'mangle\' or \'security\' table, not \'%s\'\n",
    table);
    return -EINVAL;
    }
    if (mode && mode != info.mode) {
    pr_info_ratelimited("mode already set to %hu cannot mix with rules for mode %hu\n",
    mode, info.mode);
    return -EINVAL;
    }
    switch (info.mode) {
    case SECMARK_MODE_SEL:
    break;
    default:
    pr_info_ratelimited("invalid mode: %hu\n", info.mode);
    return -EINVAL;
    }
    err = checkentry_lsm(info);
    if (err)
    return err;
    if (!mode)
    mode = info.mode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn secmark_tg_destroy(par: *const xt_tgdtor_param) {
    static void secmark_tg_destroy(const struct xt_tgdtor_param *par)
    {
    switch (mode) {
    case SECMARK_MODE_SEL:
    security_secmark_refcount_dec();
    }
    }
#[no_mangle]
unsafe extern "C" fn secmark_tg_check_v0(par: *const xt_tgchk_param) -> c_int {
    static int secmark_tg_check_v0(const struct xt_tgchk_param *par)
    {
    struct xt_secmark_target_info *info = par.targinfo;
    struct xt_secmark_target_info_v1 newinfo = {
    .mode	= info.mode,
    };
    int ret;
    memcpy(newinfo.secctx, info.secctx, SECMARK_SECCTX_MAX);
    ret = secmark_tg_check(par.table, &newinfo);
    info.secid = newinfo.secid;
    return ret;
    }
    static unsigned int
    secmark_tg_v0(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_secmark_target_info *info = par.targinfo;
    struct xt_secmark_target_info_v1 newinfo = {
    .secid	= info.secid,
    };
    return secmark_tg(skb, &newinfo);
    }
#[no_mangle]
unsafe extern "C" fn secmark_tg_check_v1(par: *const xt_tgchk_param) -> c_int {
    static int secmark_tg_check_v1(const struct xt_tgchk_param *par)
    {
    return secmark_tg_check(par.table, par.targinfo);
    }
    static unsigned int
    secmark_tg_v1(struct sk_buff *skb, const struct xt_action_param *par)
    {
    return secmark_tg(skb, par.targinfo);
    }
    static struct xt_target secmark_tg_reg[] __read_mostly = {
    {
    .name		= "SECMARK",
    .revision	= 0,
    .family		= NFPROTO_IPV4,
    .checkentry	= secmark_tg_check_v0,
    .destroy	= secmark_tg_destroy,
    .target		= secmark_tg_v0,
    .targetsize	= sizeof(struct xt_secmark_target_info),
    .me		= THIS_MODULE,
    },
    {
    .name		= "SECMARK",
    .revision	= 1,
    .family		= NFPROTO_IPV4,
    .checkentry	= secmark_tg_check_v1,
    .destroy	= secmark_tg_destroy,
    .target		= secmark_tg_v1,
    .targetsize	= sizeof(struct xt_secmark_target_info_v1),
    .usersize	= offsetof(struct xt_secmark_target_info_v1, secid),
    .me		= THIS_MODULE,
    },

    {
    .name		= "SECMARK",
    .revision	= 0,
    .family		= NFPROTO_IPV6,
    .checkentry	= secmark_tg_check_v0,
    .destroy	= secmark_tg_destroy,
    .target		= secmark_tg_v0,
    .targetsize	= sizeof(struct xt_secmark_target_info),
    .me		= THIS_MODULE,
    },
    {
    .name		= "SECMARK",
    .revision	= 1,
    .family		= NFPROTO_IPV6,
    .checkentry	= secmark_tg_check_v1,
    .destroy	= secmark_tg_destroy,
    .target		= secmark_tg_v1,
    .targetsize	= sizeof(struct xt_secmark_target_info_v1),
    .usersize	= offsetof(struct xt_secmark_target_info_v1, secid),
    .me		= THIS_MODULE,
    },

    };
#[no_mangle]
unsafe extern "C" fn secmark_tg_init() -> int __init {
    static int __init secmark_tg_init(void)
    {
    return xt_register_targets(secmark_tg_reg, ARRAY_SIZE(secmark_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn secmark_tg_exit() -> void __exit {
    static void __exit secmark_tg_exit(void)
    {
    xt_unregister_targets(secmark_tg_reg, ARRAY_SIZE(secmark_tg_reg));
    }
    module_init(secmark_tg_init);
    module_exit(secmark_tg_exit);
