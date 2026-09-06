//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_nfacct.c
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
// (C) 2011 Pablo Neira Ayuso <pablo@netfilter.org>
// (C) 2011 Intra2net AG <https://www.intra2net.com>
//

    MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
    MODULE_DESCRIPTION("Xtables: match for the extended accounting infrastructure");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_nfacct");
    MODULE_ALIAS("ip6t_nfacct");
#[no_mangle]
unsafe extern "C" fn nfacct_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool nfacct_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    int overquota;
    const struct xt_nfacct_match_info *info = par.targinfo;
    nfnl_acct_update(skb, info.nfacct);
    overquota = nfnl_acct_overquota(xt_net(par), info.nfacct);
    return overquota != NFACCT_UNDERQUOTA;
    }
    static int
    nfacct_mt_checkentry(const struct xt_mtchk_param *par)
    {
    struct xt_nfacct_match_info *info = par.matchinfo;
    struct nf_acct *nfacct;
    nfacct = nfnl_acct_find_get(par.net, info.name);
    if (nfacct == core::ptr::null_mut()) {
    pr_info_ratelimited("accounting object `%.*s' does not exist\n",
    NFACCT_NAME_MAX, info.name);
    return -ENOENT;
    }
    info.nfacct = nfacct;
    return 0;
    }
    static void
    nfacct_mt_destroy(const struct xt_mtdtor_param *par)
    {
    const struct xt_nfacct_match_info *info = par.matchinfo;
    nfnl_acct_put(info.nfacct);
    }
    static struct xt_match nfacct_mt_reg[] __read_mostly = {
    {
    .name       = "nfacct",
    .revision   = 0,
    .family     = NFPROTO_UNSPEC,
    .checkentry = nfacct_mt_checkentry,
    .match      = nfacct_mt,
    .destroy    = nfacct_mt_destroy,
    .matchsize  = sizeof(struct xt_nfacct_match_info),
    .usersize   = offsetof(struct xt_nfacct_match_info, nfacct),
    .me         = THIS_MODULE,
    },
    {
    .name       = "nfacct",
    .revision   = 1,
    .family     = NFPROTO_UNSPEC,
    .checkentry = nfacct_mt_checkentry,
    .match      = nfacct_mt,
    .destroy    = nfacct_mt_destroy,
    .matchsize  = sizeof(struct xt_nfacct_match_info_v1),
    .usersize   = offsetof(struct xt_nfacct_match_info_v1, nfacct),
    .me         = THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn nfacct_mt_init() -> int __init {
    static int __init nfacct_mt_init(void)
    {
    return xt_register_matches(nfacct_mt_reg, ARRAY_SIZE(nfacct_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn nfacct_mt_exit() -> void __exit {
    static void __exit nfacct_mt_exit(void)
    {
    xt_unregister_matches(nfacct_mt_reg, ARRAY_SIZE(nfacct_mt_reg));
    }
    module_init(nfacct_mt_init);
    module_exit(nfacct_mt_exit);
