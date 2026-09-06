//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/nf_defrag_ipv4.c
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

    static DEFINE_MUTEX(defrag4_mutex);
    static int nf_ct_ipv4_gather_frags(struct net *net, struct sk_buff *skb,
    u_int32_t user)
    {
    int err;
    local_bh_disable();
    err = ip_defrag(net, skb, user);
    local_bh_enable();
    if (!err)
    skb.ignore_df = 1;
    return err;
    }
    static enum ip_defrag_users nf_ct_defrag_user(unsigned int hooknum,
    struct sk_buff *skb)
    {
    let mut zone_id: u16 = NF_CT_DEFAULT_ZONE_ID;

    if (skb_nfct(skb)) {
    enum ip_conntrack_info ctinfo;
    const struct nf_conn *ct = nf_ct_get(skb, &ctinfo);
    zone_id = nf_ct_zone_id(nf_ct_zone(ct), CTINFO2DIR(ctinfo));
    }

    if (nf_bridge_in_prerouting(skb))
    return IP_DEFRAG_CONNTRACK_BRIDGE_IN + zone_id;
    if (hooknum == NF_INET_PRE_ROUTING)
    return IP_DEFRAG_CONNTRACK_IN + zone_id;
    else
    return IP_DEFRAG_CONNTRACK_OUT + zone_id;
    }
    static unsigned int ipv4_conntrack_defrag(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct sock *sk = skb.sk;
    if (sk && sk_fullsock(sk) && (sk.sk_family == PF_INET) &&
    inet_test_bit(NODEFRAG, sk))
    return NF_ACCEPT;

// Previously seen (loopback)?  Ignore.  Do this before
    fragment check. */
    if (skb_nfct(skb) && !nf_ct_is_template((struct nf_conn *)skb_nfct(skb)))
    return NF_ACCEPT;

    if (skb._nfct == IP_CT_UNTRACKED)
    return NF_ACCEPT;

// Gather fragments.
    if (ip_is_fragment(ip_hdr(skb))) {
    enum ip_defrag_users user =
    nf_ct_defrag_user(state.hook, skb);
    if (nf_ct_ipv4_gather_frags(state.net, skb, user))
    return NF_STOLEN;
    }
    return NF_ACCEPT;
    }
    static const struct nf_hook_ops ipv4_defrag_ops[] = {
    {
    .hook		= ipv4_conntrack_defrag,
    .pf		= NFPROTO_IPV4,
    .hooknum	= NF_INET_PRE_ROUTING,
    .priority	= NF_IP_PRI_CONNTRACK_DEFRAG,
    },
    {
    .hook           = ipv4_conntrack_defrag,
    .pf             = NFPROTO_IPV4,
    .hooknum        = NF_INET_LOCAL_OUT,
    .priority       = NF_IP_PRI_CONNTRACK_DEFRAG,
    },
    };
#[no_mangle]
unsafe extern "C" fn defrag4_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit defrag4_net_exit(struct net *net)
    {
    if (net.nf.defrag_ipv4_users) {
    nf_unregister_net_hooks(net, ipv4_defrag_ops,
    ARRAY_SIZE(ipv4_defrag_ops));
    net.nf.defrag_ipv4_users = 0;
    }
    }
    static const struct nf_defrag_hook defrag_hook = {
    .owner = THIS_MODULE,
    .enable = nf_defrag_ipv4_enable,
    .disable = nf_defrag_ipv4_disable,
    };
    static struct pernet_operations defrag4_net_ops = {
    .exit = defrag4_net_exit,
    };
#[no_mangle]
unsafe extern "C" fn nf_defrag_init() -> int __init {
    static int __init nf_defrag_init(void)
    {
    int err;
    err = register_pernet_subsys(&defrag4_net_ops);
    if (err)
    return err;
    rcu_assign_pointer(nf_defrag_v4_hook, &defrag_hook);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nf_defrag_fini() -> void __exit {
    static void __exit nf_defrag_fini(void)
    {
    rcu_assign_pointer(nf_defrag_v4_hook, core::ptr::null_mut());
    unregister_pernet_subsys(&defrag4_net_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn nf_defrag_ipv4_enable(net: *mut net) -> c_int {
    int nf_defrag_ipv4_enable(struct net *net)
    {
    let mut err: c_int = 0;
    mutex_lock(&defrag4_mutex);
    if (net.nf.defrag_ipv4_users == UINT_MAX) {
    err = -EOVERFLOW;
    goto out_unlock;
    }
    if (net.nf.defrag_ipv4_users) {
    net.nf.defrag_ipv4_users++;
    goto out_unlock;
    }
    err = nf_register_net_hooks(net, ipv4_defrag_ops,
    ARRAY_SIZE(ipv4_defrag_ops));
    if (err == 0)
    net.nf.defrag_ipv4_users = 1;
    out_unlock:
    mutex_unlock(&defrag4_mutex);
    return err;
    }
    EXPORT_SYMBOL_GPL(nf_defrag_ipv4_enable);
#[no_mangle]
pub unsafe extern "C" fn nf_defrag_ipv4_disable(net: *mut net) {
    void nf_defrag_ipv4_disable(struct net *net)
    {
    mutex_lock(&defrag4_mutex);
    if (net.nf.defrag_ipv4_users) {
    net.nf.defrag_ipv4_users--;
    if (net.nf.defrag_ipv4_users == 0)
    nf_unregister_net_hooks(net, ipv4_defrag_ops,
    ARRAY_SIZE(ipv4_defrag_ops));
    }
    mutex_unlock(&defrag4_mutex);
    }
    EXPORT_SYMBOL_GPL(nf_defrag_ipv4_disable);
    module_init(nf_defrag_init);
    module_exit(nf_defrag_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("IPv4 defragmentation support");
