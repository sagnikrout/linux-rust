//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/nf_defrag_ipv6_hooks.c
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
// (C) 1999-2001 Paul `Rusty' Russell
// (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
//

    static DEFINE_MUTEX(defrag6_mutex);
    static enum ip6_defrag_users nf_ct6_defrag_user(unsigned int hooknum,
    struct sk_buff *skb)
    {
    let mut zone_id: u16 = NF_CT_DEFAULT_ZONE_ID;

    if (skb_nfct(skb)) {
    enum ip_conntrack_info ctinfo;
    const struct nf_conn *ct = nf_ct_get(skb, &ctinfo);
    zone_id = nf_ct_zone_id(nf_ct_zone(ct), CTINFO2DIR(ctinfo));
    }

    if (nf_bridge_in_prerouting(skb))
    return IP6_DEFRAG_CONNTRACK_BRIDGE_IN + zone_id;
    if (hooknum == NF_INET_PRE_ROUTING)
    return IP6_DEFRAG_CONNTRACK_IN + zone_id;
    else
    return IP6_DEFRAG_CONNTRACK_OUT + zone_id;
    }
    static unsigned int ipv6_defrag(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    int err;

// Previously seen (loopback)?
    if (skb_nfct(skb) && !nf_ct_is_template((struct nf_conn *)skb_nfct(skb)))
    return NF_ACCEPT;
    if (skb._nfct == IP_CT_UNTRACKED)
    return NF_ACCEPT;

    err = nf_ct_frag6_gather(state.net, skb,
    nf_ct6_defrag_user(state.hook, skb));
// queued
    if (err == -EINPROGRESS)
    return NF_STOLEN;
    let mut err: return = = 0 ? NF_ACCEPT : NF_DROP;
    }
    static const struct nf_hook_ops ipv6_defrag_ops[] = {
    {
    .hook		= ipv6_defrag,
    .pf		= NFPROTO_IPV6,
    .hooknum	= NF_INET_PRE_ROUTING,
    .priority	= NF_IP6_PRI_CONNTRACK_DEFRAG,
    },
    {
    .hook		= ipv6_defrag,
    .pf		= NFPROTO_IPV6,
    .hooknum	= NF_INET_LOCAL_OUT,
    .priority	= NF_IP6_PRI_CONNTRACK_DEFRAG,
    },
    };
#[no_mangle]
unsafe extern "C" fn defrag6_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit defrag6_net_exit(struct net *net)
    {
    if (net.nf.defrag_ipv6_users) {
    nf_unregister_net_hooks(net, ipv6_defrag_ops,
    ARRAY_SIZE(ipv6_defrag_ops));
    net.nf.defrag_ipv6_users = 0;
    }
    }
    static const struct nf_defrag_hook defrag_hook = {
    .owner = THIS_MODULE,
    .enable = nf_defrag_ipv6_enable,
    .disable = nf_defrag_ipv6_disable,
    };
    static struct pernet_operations defrag6_net_ops = {
    .exit = defrag6_net_exit,
    };
#[no_mangle]
unsafe extern "C" fn nf_defrag_init() -> int __init {
    static int __init nf_defrag_init(void)
    {
    let mut ret: c_int = 0;
    ret = nf_ct_frag6_init();
    if (ret < 0) {
    pr_err("nf_defrag_ipv6: can't initialize frag6.\n");
    return ret;
    }
    ret = register_pernet_subsys(&defrag6_net_ops);
    if (ret < 0) {
    pr_err("nf_defrag_ipv6: can't register pernet ops\n");
    goto cleanup_frag6;
    }
    rcu_assign_pointer(nf_defrag_v6_hook, &defrag_hook);
    return ret;
    cleanup_frag6:
    nf_ct_frag6_cleanup();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nf_defrag_fini() -> void __exit {
    static void __exit nf_defrag_fini(void)
    {
    rcu_assign_pointer(nf_defrag_v6_hook, core::ptr::null_mut());
    unregister_pernet_subsys(&defrag6_net_ops);
    nf_ct_frag6_cleanup();
    }
#[no_mangle]
pub unsafe extern "C" fn nf_defrag_ipv6_enable(net: *mut net) -> c_int {
    int nf_defrag_ipv6_enable(struct net *net)
    {
    let mut err: c_int = 0;
    mutex_lock(&defrag6_mutex);
    if (net.nf.defrag_ipv6_users == UINT_MAX) {
    err = -EOVERFLOW;
    goto out_unlock;
    }
    if (net.nf.defrag_ipv6_users) {
    net.nf.defrag_ipv6_users++;
    goto out_unlock;
    }
    err = nf_register_net_hooks(net, ipv6_defrag_ops,
    ARRAY_SIZE(ipv6_defrag_ops));
    if (err == 0)
    net.nf.defrag_ipv6_users = 1;
    out_unlock:
    mutex_unlock(&defrag6_mutex);
    return err;
    }
    EXPORT_SYMBOL_GPL(nf_defrag_ipv6_enable);
#[no_mangle]
pub unsafe extern "C" fn nf_defrag_ipv6_disable(net: *mut net) {
    void nf_defrag_ipv6_disable(struct net *net)
    {
    mutex_lock(&defrag6_mutex);
    if (net.nf.defrag_ipv6_users) {
    net.nf.defrag_ipv6_users--;
    if (net.nf.defrag_ipv6_users == 0)
    nf_unregister_net_hooks(net, ipv6_defrag_ops,
    ARRAY_SIZE(ipv6_defrag_ops));
    }
    mutex_unlock(&defrag6_mutex);
    }
    EXPORT_SYMBOL_GPL(nf_defrag_ipv6_disable);
    module_init(nf_defrag_init);
    module_exit(nf_defrag_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("IPv6 defragmentation support");
