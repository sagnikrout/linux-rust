//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/iptable_mangle.c
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
// This is the 1999 rewrite of IP Firewalling, aiming for kernel 2.3.x.
//
// Copyright (C) 1999 Paul `Rusty' Russell & Michael J. Neuling
// Copyright (C) 2000-2004 Netfilter Core Team <coreteam@netfilter.org>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Netfilter Core Team <coreteam@netfilter.org>");
    MODULE_DESCRIPTION("iptables mangle table");

    (1 << NF_INET_LOCAL_IN) | \
    (1 << NF_INET_FORWARD) | \
    (1 << NF_INET_LOCAL_OUT) | \
    (1 << NF_INET_POST_ROUTING))
    static const struct xt_table packet_mangler = {
    .name		= "mangle",
    .valid_hooks	= MANGLE_VALID_HOOKS,
    .me		= THIS_MODULE,
    .af		= NFPROTO_IPV4,
    .priority	= NF_IP_PRI_MANGLE,
    };
    static unsigned int
    ipt_mangle_out(void *priv, struct sk_buff *skb, const struct nf_hook_state *state)
    {
    unsigned int ret, verdict;
    const struct iphdr *iph;
    __be32 saddr, daddr;
    u32 mark;
    int err;
    u8 tos;
// Save things which could affect route
    mark = skb.mark;
    iph = ip_hdr(skb);
    saddr = iph.saddr;
    daddr = iph.daddr;
    tos = iph.tos;
    ret = ipt_do_table(priv, skb, state);
    verdict = ret & NF_VERDICT_MASK;
// Reroute for ANY change.
    if (verdict != NF_DROP && verdict != NF_STOLEN) {
    iph = ip_hdr(skb);
    if (iph.saddr != saddr ||
    iph.daddr != daddr ||
    skb.mark != mark ||
    iph.tos != tos) {
    err = ip_route_me_harder(state.net, state.sk, skb, RTN_UNSPEC);
    if (err < 0)
    ret = NF_DROP_ERR(err);
    }
    }
    return ret;
    }
// The work comes in here from netfilter.c.
    static unsigned int
    iptable_mangle_hook(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    if (state.hook == NF_INET_LOCAL_OUT)
    return ipt_mangle_out(priv, skb, state);
    return ipt_do_table(priv, skb, state);
    }
    static struct nf_hook_ops *mangle_ops __read_mostly;
#[no_mangle]
unsafe extern "C" fn iptable_mangle_table_init(net: *mut net) -> c_int {
    static int iptable_mangle_table_init(struct net *net)
    {
    struct ipt_replace *repl;
    int ret;
    repl = ipt_alloc_initial_table(&packet_mangler);
    if (repl == core::ptr::null_mut())
    return -ENOMEM;
    ret = ipt_register_table(net, &packet_mangler, repl, mangle_ops);
    kfree(repl);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iptable_mangle_net_pre_exit(net: *mut net) -> void __net_exit {
    static void __net_exit iptable_mangle_net_pre_exit(struct net *net)
    {
    xt_unregister_table_pre_exit(net, NFPROTO_IPV4, "mangle");
    }
#[no_mangle]
unsafe extern "C" fn iptable_mangle_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit iptable_mangle_net_exit(struct net *net)
    {
    ipt_unregister_table_exit(net, "mangle");
    }
    static struct pernet_operations iptable_mangle_net_ops = {
    .pre_exit = iptable_mangle_net_pre_exit,
    .exit = iptable_mangle_net_exit,
    };
#[no_mangle]
unsafe extern "C" fn iptable_mangle_init() -> int __init {
    static int __init iptable_mangle_init(void)
    {
    int ret;
    mangle_ops = xt_hook_ops_alloc(&packet_mangler, iptable_mangle_hook);
    if (IS_ERR(mangle_ops))
    return PTR_ERR(mangle_ops);
    ret = register_pernet_subsys(&iptable_mangle_net_ops);
    if (ret < 0)
    goto err_free;
    ret = xt_register_template(&packet_mangler,
    iptable_mangle_table_init);
    if (ret < 0) {
    unregister_pernet_subsys(&iptable_mangle_net_ops);
    goto err_free;
    }
    return 0;
    err_free:
    kfree(mangle_ops);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iptable_mangle_fini() -> void __exit {
    static void __exit iptable_mangle_fini(void)
    {
    xt_unregister_template(&packet_mangler);
    unregister_pernet_subsys(&iptable_mangle_net_ops);
    kfree(mangle_ops);
    }
    module_init(iptable_mangle_init);
    module_exit(iptable_mangle_fini);
