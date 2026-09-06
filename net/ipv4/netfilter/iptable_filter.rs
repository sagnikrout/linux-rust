//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/iptable_filter.c
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
    MODULE_DESCRIPTION("iptables filter table");

    (1 << NF_INET_FORWARD) | \
    (1 << NF_INET_LOCAL_OUT))
    static const struct xt_table packet_filter = {
    .name		= "filter",
    .valid_hooks	= FILTER_VALID_HOOKS,
    .me		= THIS_MODULE,
    .af		= NFPROTO_IPV4,
    .priority	= NF_IP_PRI_FILTER,
    };
    static struct nf_hook_ops *filter_ops __read_mostly;
// Default to forward because I got too much mail already.
    let mut __read_mostly: static bool forward = true;
    module_param(forward, bool, 0000);
#[no_mangle]
unsafe extern "C" fn iptable_filter_table_init(net: *mut net) -> c_int {
    static int iptable_filter_table_init(struct net *net)
    {
    struct ipt_replace *repl;
    int err;
    repl = ipt_alloc_initial_table(&packet_filter);
    if (repl == core::ptr::null_mut())
    return -ENOMEM;
// Entry 1 is the FORWARD hook
    ((struct ipt_standard *)repl.entries)[1].target.verdict =
    forward ? -NF_ACCEPT - 1 : NF_DROP - 1;
    err = ipt_register_table(net, &packet_filter, repl, filter_ops);
    kfree(repl);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn iptable_filter_net_init(net: *mut net) -> int __net_init {
    static int __net_init iptable_filter_net_init(struct net *net)
    {
    if (!forward)
    return iptable_filter_table_init(net);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iptable_filter_net_pre_exit(net: *mut net) -> void __net_exit {
    static void __net_exit iptable_filter_net_pre_exit(struct net *net)
    {
    xt_unregister_table_pre_exit(net, NFPROTO_IPV4, "filter");
    }
#[no_mangle]
unsafe extern "C" fn iptable_filter_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit iptable_filter_net_exit(struct net *net)
    {
    ipt_unregister_table_exit(net, "filter");
    }
    static struct pernet_operations iptable_filter_net_ops = {
    .init = iptable_filter_net_init,
    .pre_exit = iptable_filter_net_pre_exit,
    .exit = iptable_filter_net_exit,
    };
#[no_mangle]
unsafe extern "C" fn iptable_filter_init() -> int __init {
    static int __init iptable_filter_init(void)
    {
    int ret;
    filter_ops = xt_hook_ops_alloc(&packet_filter, ipt_do_table);
    if (IS_ERR(filter_ops))
    return PTR_ERR(filter_ops);
    ret = register_pernet_subsys(&iptable_filter_net_ops);
    if (ret < 0)
    goto err_free;
    ret = xt_register_template(&packet_filter,
    iptable_filter_table_init);
    if (ret < 0) {
    unregister_pernet_subsys(&iptable_filter_net_ops);
    goto err_free;
    }
    return 0;
    err_free:
    kfree(filter_ops);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iptable_filter_fini() -> void __exit {
    static void __exit iptable_filter_fini(void)
    {
    xt_unregister_template(&packet_filter);
    unregister_pernet_subsys(&iptable_filter_net_ops);
    kfree(filter_ops);
    }
    module_init(iptable_filter_init);
    module_exit(iptable_filter_fini);
