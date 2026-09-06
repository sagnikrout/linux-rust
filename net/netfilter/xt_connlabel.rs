//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_connlabel.c
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
// (C) 2013 Astaro GmbH & Co KG
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
    MODULE_DESCRIPTION("Xtables: add/match connection tracking labels");
    MODULE_ALIAS("ipt_connlabel");
    MODULE_ALIAS("ip6t_connlabel");
    static bool
    connlabel_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_connlabel_mtinfo *info = par.matchinfo;
    enum ip_conntrack_info ctinfo;
    struct nf_conn_labels *labels;
    struct nf_conn *ct;
    let mut invert: bool = info.options & XT_CONNLABEL_OP_INVERT;
    ct = nf_ct_get(skb, &ctinfo);
    if (ct == core::ptr::null_mut())
    return invert;
    labels = nf_ct_labels_find(ct);
    if (!labels)
    return invert;
    if (test_bit(info.bit, labels.bits))
    return !invert;
    if (info.options & XT_CONNLABEL_OP_SET) {
    if (!test_and_set_bit(info.bit, labels.bits))
    nf_conntrack_event_cache(IPCT_LABEL, ct);
    return !invert;
    }
    return invert;
    }
#[no_mangle]
unsafe extern "C" fn connlabel_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int connlabel_mt_check(const struct xt_mtchk_param *par)
    {
    const int options = XT_CONNLABEL_OP_INVERT |
    XT_CONNLABEL_OP_SET;
    struct xt_connlabel_mtinfo *info = par.matchinfo;
    int ret;
    if (info.options & ~options) {
    pr_info_ratelimited("Unknown options in mask %x\n",
    info.options);
    return -EINVAL;
    }
    ret = nf_ct_netns_get(par.net, par.family);
    if (ret < 0) {
    pr_info_ratelimited("cannot load conntrack support for proto=%u\n",
    par.family);
    return ret;
    }
    ret = nf_connlabels_get(par.net, info.bit);
    if (ret < 0)
    nf_ct_netns_put(par.net, par.family);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn connlabel_mt_destroy(par: *const xt_mtdtor_param) {
    static void connlabel_mt_destroy(const struct xt_mtdtor_param *par)
    {
    nf_connlabels_put(par.net);
    nf_ct_netns_put(par.net, par.family);
    }
    static struct xt_match connlabels_mt_reg __read_mostly = {
    .name           = "connlabel",
    .family         = NFPROTO_UNSPEC,
    .checkentry     = connlabel_mt_check,
    .match          = connlabel_mt,
    .matchsize      = sizeof(struct xt_connlabel_mtinfo),
    .destroy        = connlabel_mt_destroy,
    .me             = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn connlabel_mt_init() -> int __init {
    static int __init connlabel_mt_init(void)
    {
    return xt_register_match(&connlabels_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn connlabel_mt_exit() -> void __exit {
    static void __exit connlabel_mt_exit(void)
    {
    xt_unregister_match(&connlabels_mt_reg);
    }
    module_init(connlabel_mt_init);
    module_exit(connlabel_mt_exit);
