//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_AUDIT.c
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
// Creates audit record for dropped/accepted packets
//
// (C) 2010-2011 Thomas Graf <tgraf@redhat.com>
// (C) 2010-2011 Red Hat, Inc.
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Thomas Graf <tgraf@redhat.com>");
    MODULE_DESCRIPTION("Xtables: creates audit records for dropped/accepted packets");
    MODULE_ALIAS("ipt_AUDIT");
    MODULE_ALIAS("ip6t_AUDIT");
    MODULE_ALIAS("ebt_AUDIT");
    MODULE_ALIAS("arpt_AUDIT");
    static unsigned int
    audit_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    struct audit_buffer *ab;
    if (audit_enabled == AUDIT_OFF)
    goto errout;
    ab = audit_log_start(core::ptr::null_mut(), GFP_ATOMIC, AUDIT_NETFILTER_PKT);
    if (ab == core::ptr::null_mut())
    goto errout;
    audit_log_format(ab, "mark=%#x", skb.mark);
    audit_log_nf_skb(ab, skb, xt_family(par));
    audit_log_end(ab);
    errout:
    return XT_CONTINUE;
    }
    static unsigned int
    audit_tg_ebt(struct sk_buff *skb, const struct xt_action_param *par)
    {
    audit_tg(skb, par);
    return EBT_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn audit_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int audit_tg_check(const struct xt_tgchk_param *par)
    {
    const struct xt_audit_info *info = par.targinfo;
    if (info.type > XT_AUDIT_TYPE_MAX) {
    pr_info_ratelimited("Audit type out of range (valid range: 0..%u)\n",
    XT_AUDIT_TYPE_MAX);
    return -ERANGE;
    }
    return 0;
    }
    static struct xt_target audit_tg_reg[] __read_mostly = {
    {
    .name		= "AUDIT",
    .family		= NFPROTO_UNSPEC,
    .target		= audit_tg,
    .targetsize	= sizeof(struct xt_audit_info),
    .checkentry	= audit_tg_check,
    .me		= THIS_MODULE,
    },
    {
    .name		= "AUDIT",
    .family		= NFPROTO_BRIDGE,
    .target		= audit_tg_ebt,
    .targetsize	= sizeof(struct xt_audit_info),
    .checkentry	= audit_tg_check,
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn audit_tg_init() -> int __init {
    static int __init audit_tg_init(void)
    {
    return xt_register_targets(audit_tg_reg, ARRAY_SIZE(audit_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn audit_tg_exit() -> void __exit {
    static void __exit audit_tg_exit(void)
    {
    xt_unregister_targets(audit_tg_reg, ARRAY_SIZE(audit_tg_reg));
    }
    module_init(audit_tg_init);
    module_exit(audit_tg_exit);
