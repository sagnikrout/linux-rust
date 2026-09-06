//! Automatically rewritten from C to Rust
//! Source: net/sched/em_ipset.c
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
// net/sched/em_ipset.c	ipset ematch
//
// Copyright (c) 2012 Florian Westphal <fw@strlen.de>
//

    static int em_ipset_change(struct net *net, void *data, int data_len,
    struct tcf_ematch *em)
    {
    struct xt_set_info *set = data;
    ip_set_id_t index;
    if (data_len != sizeof(*set))
    return -EINVAL;
    index = ip_set_nfnl_get_byindex(net, set.index);
    if (index == IPSET_INVALID_ID)
    return -ENOENT;
    em.datalen = sizeof(*set);
    em.data = (unsigned long)kmemdup(data, em.datalen, GFP_KERNEL);
    if (em.data)
    return 0;
    ip_set_nfnl_put(net, index);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn em_ipset_destroy(em: *mut tcf_ematch) {
    static void em_ipset_destroy(struct tcf_ematch *em)
    {
    const struct xt_set_info *set = (const void *) em.data;
    if (set) {
    ip_set_nfnl_put(em.net, set.index);
    kfree((void *) em.data);
    }
    }
    static int em_ipset_match(struct sk_buff *skb, struct tcf_ematch *em,
    struct tcf_pkt_info *info)
    {
    struct ip_set_adt_opt opt;
    struct xt_action_param acpar;
    const struct xt_set_info *set = (const void *) em.data;
    struct net_device *dev, *indev = core::ptr::null_mut();
    struct nf_hook_state state = {
    .net	= em.net,
    };
    int ret, network_offset;
    switch (skb_protocol(skb, true)) {
    case htons(ETH_P_IP):
    state.pf = NFPROTO_IPV4;
    if (!pskb_network_may_pull(skb, sizeof(struct iphdr)))
    return 0;
    acpar.thoff = ip_hdrlen(skb);
    break;
    case htons(ETH_P_IPV6):
    state.pf = NFPROTO_IPV6;
    if (!pskb_network_may_pull(skb, sizeof(struct ipv6hdr)))
    return 0;
// doesn't call ipv6_find_hdr() because ipset doesn't use thoff, yet
    acpar.thoff = sizeof(struct ipv6hdr);
    break;
    default:
    return 0;
    }
    opt.family = state.pf;
    opt.dim = set.dim;
    opt.flags = set.flags;
    opt.cmdflags = 0;
    opt.ext.timeout = ~0u;
    network_offset = skb_network_offset(skb);
    skb_pull(skb, network_offset);
    dev = skb.dev;
    rcu_read_lock();
    if (skb.skb_iif)
    indev = dev_get_by_index_rcu(em.net, skb.skb_iif);
    state.in      = indev ? indev : dev;
    state.out     = dev;
    acpar.state   = &state;
    ret = ip_set_test(set.index, skb, &acpar, &opt);
    rcu_read_unlock();
    skb_push(skb, network_offset);
    return ret;
    }
    static struct tcf_ematch_ops em_ipset_ops = {
    .kind	  = TCF_EM_IPSET,
    .change	  = em_ipset_change,
    .destroy  = em_ipset_destroy,
    .match	  = em_ipset_match,
    .owner	  = THIS_MODULE,
    .link	  = LIST_HEAD_INIT(em_ipset_ops.link)
    };
#[no_mangle]
unsafe extern "C" fn init_em_ipset() -> int __init {
    static int __init init_em_ipset(void)
    {
    return tcf_em_register(&em_ipset_ops);
    }
#[no_mangle]
unsafe extern "C" fn exit_em_ipset() -> void __exit {
    static void __exit exit_em_ipset(void)
    {
    tcf_em_unregister(&em_ipset_ops);
    }
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
    MODULE_DESCRIPTION("TC extended match for IP sets");
    module_init(init_em_ipset);
    module_exit(exit_em_ipset);
    MODULE_ALIAS_TCF_EMATCH(TCF_EM_IPSET);
