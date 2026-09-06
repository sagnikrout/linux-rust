//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_helper.c
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
// iptables module to match on related connections
//
// (C) 2001 Martin Josefsson <gandalf@wlug.westbo.se>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Martin Josefsson <gandalf@netfilter.org>");
    MODULE_DESCRIPTION("Xtables: Related connection matching");
    MODULE_ALIAS("ipt_helper");
    MODULE_ALIAS("ip6t_helper");
    static bool
    helper_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_helper_info *info = par.matchinfo;
    const struct nf_conn *ct;
    const struct nf_conn_help *master_help;
    const struct nf_conntrack_helper *helper;
    enum ip_conntrack_info ctinfo;
    let mut ret: bool = info.invert;
    ct = nf_ct_get(skb, &ctinfo);
    if (!ct || !ct.master)
    return ret;
    master_help = nfct_help(ct.master);
    if (!master_help)
    return ret;
// rcu_read_lock()ed by nf_hook_thresh
    helper = rcu_dereference(master_help.helper);
    if (!helper)
    return ret;
    if (info.name[0] == '\0')
    ret = !ret;
    else
    ret ^= !strncmp(helper.name, info.name,
    strlen(helper.name));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn helper_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int helper_mt_check(const struct xt_mtchk_param *par)
    {
    struct xt_helper_info *info = par.matchinfo;
    int ret;
    ret = nf_ct_netns_get(par.net, par.family);
    if (ret < 0) {
    pr_info_ratelimited("cannot load conntrack support for proto=%u\n",
    par.family);
    return ret;
    }
    info.name[sizeof(info.name) - 1] = '\0';
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn helper_mt_destroy(par: *const xt_mtdtor_param) {
    static void helper_mt_destroy(const struct xt_mtdtor_param *par)
    {
    nf_ct_netns_put(par.net, par.family);
    }
    static struct xt_match helper_mt_reg __read_mostly = {
    .name       = "helper",
    .revision   = 0,
    .family     = NFPROTO_UNSPEC,
    .checkentry = helper_mt_check,
    .match      = helper_mt,
    .destroy    = helper_mt_destroy,
    .matchsize  = sizeof(struct xt_helper_info),
    .me         = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn helper_mt_init() -> int __init {
    static int __init helper_mt_init(void)
    {
    return xt_register_match(&helper_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn helper_mt_exit() -> void __exit {
    static void __exit helper_mt_exit(void)
    {
    xt_unregister_match(&helper_mt_reg);
    }
    module_init(helper_mt_init);
    module_exit(helper_mt_exit);
