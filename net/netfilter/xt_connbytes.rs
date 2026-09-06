//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_connbytes.c
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


// SPDX-License-Identifier: GPL-2.0
// Kernel module to match connection tracking byte counter.
// (C) 2002 Martin Devera (devik@cdi.cz).
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Harald Welte <laforge@netfilter.org>");
    MODULE_DESCRIPTION("Xtables: Number of packets/bytes per connection matching");
    MODULE_ALIAS("ipt_connbytes");
    MODULE_ALIAS("ip6t_connbytes");
    static bool
    connbytes_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_connbytes_info *sinfo = par.matchinfo;
    const struct nf_conn *ct;
    enum ip_conntrack_info ctinfo;
    u_int64_t what = 0;	/* initialize to make gcc happy */
    let mut bytes: u_int64_t = 0;
    let mut pkts: u_int64_t = 0;
    const struct nf_conn_acct *acct;
    const struct nf_conn_counter *counters;
    ct = nf_ct_get(skb, &ctinfo);
    if (!ct)
    return false;
    acct = nf_conn_acct_find(ct);
    if (!acct)
    return false;
    counters = acct.counter;
    switch (sinfo.what) {
    case XT_CONNBYTES_PKTS:
    switch (sinfo.direction) {
    case XT_CONNBYTES_DIR_ORIGINAL:
    what = atomic64_read(&counters[IP_CT_DIR_ORIGINAL].packets);
    break;
    case XT_CONNBYTES_DIR_REPLY:
    what = atomic64_read(&counters[IP_CT_DIR_REPLY].packets);
    break;
    case XT_CONNBYTES_DIR_BOTH:
    what = atomic64_read(&counters[IP_CT_DIR_ORIGINAL].packets);
    what += atomic64_read(&counters[IP_CT_DIR_REPLY].packets);
    break;
    }
    break;
    case XT_CONNBYTES_BYTES:
    switch (sinfo.direction) {
    case XT_CONNBYTES_DIR_ORIGINAL:
    what = atomic64_read(&counters[IP_CT_DIR_ORIGINAL].bytes);
    break;
    case XT_CONNBYTES_DIR_REPLY:
    what = atomic64_read(&counters[IP_CT_DIR_REPLY].bytes);
    break;
    case XT_CONNBYTES_DIR_BOTH:
    what = atomic64_read(&counters[IP_CT_DIR_ORIGINAL].bytes);
    what += atomic64_read(&counters[IP_CT_DIR_REPLY].bytes);
    break;
    }
    break;
    case XT_CONNBYTES_AVGPKT:
    switch (sinfo.direction) {
    case XT_CONNBYTES_DIR_ORIGINAL:
    bytes = atomic64_read(&counters[IP_CT_DIR_ORIGINAL].bytes);
    pkts  = atomic64_read(&counters[IP_CT_DIR_ORIGINAL].packets);
    break;
    case XT_CONNBYTES_DIR_REPLY:
    bytes = atomic64_read(&counters[IP_CT_DIR_REPLY].bytes);
    pkts  = atomic64_read(&counters[IP_CT_DIR_REPLY].packets);
    break;
    case XT_CONNBYTES_DIR_BOTH:
    bytes = atomic64_read(&counters[IP_CT_DIR_ORIGINAL].bytes) +
    atomic64_read(&counters[IP_CT_DIR_REPLY].bytes);
    pkts  = atomic64_read(&counters[IP_CT_DIR_ORIGINAL].packets) +
    atomic64_read(&counters[IP_CT_DIR_REPLY].packets);
    break;
    }
    if (pkts != 0)
    what = div64_u64(bytes, pkts);
    break;
    }
    if (sinfo.count.to >= sinfo.count.from)
    return what <= sinfo.count.to && what >= sinfo.count.from;
    else /* inverted */
    return what < sinfo.count.to || what > sinfo.count.from;
    }
#[no_mangle]
unsafe extern "C" fn connbytes_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int connbytes_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_connbytes_info *sinfo = par.matchinfo;
    int ret;
    if (sinfo.what != XT_CONNBYTES_PKTS &&
    sinfo.what != XT_CONNBYTES_BYTES &&
    sinfo.what != XT_CONNBYTES_AVGPKT)
    return -EINVAL;
    if (sinfo.direction != XT_CONNBYTES_DIR_ORIGINAL &&
    sinfo.direction != XT_CONNBYTES_DIR_REPLY &&
    sinfo.direction != XT_CONNBYTES_DIR_BOTH)
    return -EINVAL;
    ret = nf_ct_netns_get(par.net, par.family);
    if (ret < 0) {
    pr_info_ratelimited("cannot load conntrack support for proto=%u\n",
    par.family);
    return ret;
    }
//
// This filter cannot function correctly unless connection tracking
// accounting is enabled, so complain in the hope that someone notices.
//
    if (!nf_ct_acct_enabled(par.net)) {
    pr_warn("Forcing CT accounting to be enabled\n");
    nf_ct_set_acct(par.net, true);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn connbytes_mt_destroy(par: *const xt_mtdtor_param) {
    static void connbytes_mt_destroy(const struct xt_mtdtor_param *par)
    {
    nf_ct_netns_put(par.net, par.family);
    }
    static struct xt_match connbytes_mt_reg __read_mostly = {
    .name       = "connbytes",
    .revision   = 0,
    .family     = NFPROTO_UNSPEC,
    .checkentry = connbytes_mt_check,
    .match      = connbytes_mt,
    .destroy    = connbytes_mt_destroy,
    .matchsize  = sizeof(struct xt_connbytes_info),
    .me         = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn connbytes_mt_init() -> int __init {
    static int __init connbytes_mt_init(void)
    {
    return xt_register_match(&connbytes_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn connbytes_mt_exit() -> void __exit {
    static void __exit connbytes_mt_exit(void)
    {
    xt_unregister_match(&connbytes_mt_reg);
    }
    module_init(connbytes_mt_init);
    module_exit(connbytes_mt_exit);
