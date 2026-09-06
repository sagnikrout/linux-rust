//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/iptable_nat.c
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
// (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
// (C) 2011 Patrick McHardy <kaber@trash.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iptable_nat_pernet {
    pub nf_nat_ops: *mut nf_hook_ops,
}

    static unsigned int iptable_nat_net_id __read_mostly;
    static const struct xt_table nf_nat_ipv4_table = {
    .name		= "nat",
    .valid_hooks	= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_LOCAL_IN),
    .me		= THIS_MODULE,
    .af		= NFPROTO_IPV4,
    };
    static const struct nf_hook_ops nf_nat_ipv4_ops[] = {
    {
    .hook		= ipt_do_table,
    .pf		= NFPROTO_IPV4,
    .hooknum	= NF_INET_PRE_ROUTING,
    .priority	= NF_IP_PRI_NAT_DST,
    },
    {
    .hook		= ipt_do_table,
    .pf		= NFPROTO_IPV4,
    .hooknum	= NF_INET_POST_ROUTING,
    .priority	= NF_IP_PRI_NAT_SRC,
    },
    {
    .hook		= ipt_do_table,
    .pf		= NFPROTO_IPV4,
    .hooknum	= NF_INET_LOCAL_OUT,
    .priority	= NF_IP_PRI_NAT_DST,
    },
    {
    .hook		= ipt_do_table,
    .pf		= NFPROTO_IPV4,
    .hooknum	= NF_INET_LOCAL_IN,
    .priority	= NF_IP_PRI_NAT_SRC,
    },
    };
#[no_mangle]
unsafe extern "C" fn ipt_nat_register_lookups(net: *mut net) -> c_int {
    static int ipt_nat_register_lookups(struct net *net)
    {
    struct iptable_nat_pernet *xt_nat_net;
    struct nf_hook_ops *ops;
    struct xt_table *table;
    int i, ret;
    xt_nat_net = net_generic(net, iptable_nat_net_id);
    table = xt_find_table(net, NFPROTO_IPV4, "nat");
    if (WARN_ON_ONCE(!table))
    return -ENOENT;
    ops = kmemdup(nf_nat_ipv4_ops, sizeof(nf_nat_ipv4_ops), GFP_KERNEL);
    if (!ops)
    return -ENOMEM;
    for (i = 0; i < ARRAY_SIZE(nf_nat_ipv4_ops); i++) {
    ops[i].priv = table;
    ret = nf_nat_ipv4_register_fn(net, &ops[i]);
    if (ret) {
    while (i)
    nf_nat_ipv4_unregister_fn(net, &ops[--i]);
    kfree_rcu(ops, rcu);
    return ret;
    }
    }
    xt_nat_net.nf_nat_ops = ops;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipt_nat_unregister_lookups(net: *mut net) {
    static void ipt_nat_unregister_lookups(struct net *net)
    {
    struct iptable_nat_pernet *xt_nat_net = net_generic(net, iptable_nat_net_id);
    struct nf_hook_ops *ops = xt_nat_net.nf_nat_ops;
    int i;
    if (!ops)
    return;
    for (i = 0; i < ARRAY_SIZE(nf_nat_ipv4_ops); i++)
    nf_nat_ipv4_unregister_fn(net, &ops[i]);
    kfree_rcu(ops, rcu);
    }
#[no_mangle]
unsafe extern "C" fn iptable_nat_table_init(net: *mut net) -> c_int {
    static int iptable_nat_table_init(struct net *net)
    {
    struct ipt_replace *repl;
    int ret;
    repl = ipt_alloc_initial_table(&nf_nat_ipv4_table);
    if (repl == core::ptr::null_mut())
    return -ENOMEM;
    ret = ipt_register_table(net, &nf_nat_ipv4_table, repl, core::ptr::null_mut());
    if (ret < 0) {
    kfree(repl);
    return ret;
    }
    ret = ipt_nat_register_lookups(net);
    if (ret < 0) {
    xt_unregister_table_pre_exit(net, NFPROTO_IPV4, "nat");
    synchronize_rcu();
    ipt_unregister_table_exit(net, "nat");
    }
    kfree(repl);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iptable_nat_net_pre_exit(net: *mut net) -> void __net_exit {
    static void __net_exit iptable_nat_net_pre_exit(struct net *net)
    {
    ipt_nat_unregister_lookups(net);
    xt_unregister_table_pre_exit(net, NFPROTO_IPV4, "nat");
    }
#[no_mangle]
unsafe extern "C" fn iptable_nat_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit iptable_nat_net_exit(struct net *net)
    {
    ipt_unregister_table_exit(net, "nat");
    }
    static struct pernet_operations iptable_nat_net_ops = {
    .pre_exit = iptable_nat_net_pre_exit,
    .exit	= iptable_nat_net_exit,
    .id	= &iptable_nat_net_id,
    .size	= sizeof(struct iptable_nat_pernet),
    };
#[no_mangle]
unsafe extern "C" fn iptable_nat_init() -> int __init {
    static int __init iptable_nat_init(void)
    {
    int ret;
// net->gen->ptr[iptable_nat_net_id] must be allocated
// before calling iptable_nat_table_init().
//
    ret = register_pernet_subsys(&iptable_nat_net_ops);
    if (ret < 0)
    return ret;
    ret = xt_register_template(&nf_nat_ipv4_table,
    iptable_nat_table_init);
    if (ret < 0)
    unregister_pernet_subsys(&iptable_nat_net_ops);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iptable_nat_exit() -> void __exit {
    static void __exit iptable_nat_exit(void)
    {
    xt_unregister_template(&nf_nat_ipv4_table);
    unregister_pernet_subsys(&iptable_nat_net_ops);
    }
    module_init(iptable_nat_init);
    module_exit(iptable_nat_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("iptables legacy nat table");
