//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/iptable_security.c
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
// "security" table
//
// This is for use by Mandatory Access Control (MAC) security models,
// which need to be able to manage security policy in separate context
// to DAC.
//
// Based on iptable_mangle.c
//
// Copyright (C) 1999 Paul `Rusty' Russell & Michael J. Neuling
// Copyright (C) 2000-2004 Netfilter Core Team <coreteam <at> netfilter.org>
// Copyright (C) 2008 Red Hat, Inc., James Morris <jmorris <at> redhat.com>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("James Morris <jmorris <at> redhat.com>");
    MODULE_DESCRIPTION("iptables security table, for MAC rules");

    (1 << NF_INET_FORWARD) | \
    (1 << NF_INET_LOCAL_OUT)
    static const struct xt_table security_table = {
    .name		= "security",
    .valid_hooks	= SECURITY_VALID_HOOKS,
    .me		= THIS_MODULE,
    .af		= NFPROTO_IPV4,
    .priority	= NF_IP_PRI_SECURITY,
    };
    static struct nf_hook_ops *sectbl_ops __read_mostly;
#[no_mangle]
unsafe extern "C" fn iptable_security_table_init(net: *mut net) -> c_int {
    static int iptable_security_table_init(struct net *net)
    {
    struct ipt_replace *repl;
    int ret;
    repl = ipt_alloc_initial_table(&security_table);
    if (repl == core::ptr::null_mut())
    return -ENOMEM;
    ret = ipt_register_table(net, &security_table, repl, sectbl_ops);
    kfree(repl);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iptable_security_net_pre_exit(net: *mut net) -> void __net_exit {
    static void __net_exit iptable_security_net_pre_exit(struct net *net)
    {
    xt_unregister_table_pre_exit(net, NFPROTO_IPV4, "security");
    }
#[no_mangle]
unsafe extern "C" fn iptable_security_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit iptable_security_net_exit(struct net *net)
    {
    ipt_unregister_table_exit(net, "security");
    }
    static struct pernet_operations iptable_security_net_ops = {
    .pre_exit = iptable_security_net_pre_exit,
    .exit = iptable_security_net_exit,
    };
#[no_mangle]
unsafe extern "C" fn iptable_security_init() -> int __init {
    static int __init iptable_security_init(void)
    {
    int ret;
    sectbl_ops = xt_hook_ops_alloc(&security_table, ipt_do_table);
    if (IS_ERR(sectbl_ops))
    return PTR_ERR(sectbl_ops);
    ret = register_pernet_subsys(&iptable_security_net_ops);
    if (ret < 0)
    goto err_free;
    ret = xt_register_template(&security_table,
    iptable_security_table_init);
    if (ret < 0) {
    unregister_pernet_subsys(&iptable_security_net_ops);
    goto err_free;
    }
    return 0;
    err_free:
    kfree(sectbl_ops);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iptable_security_fini() -> void __exit {
    static void __exit iptable_security_fini(void)
    {
    xt_unregister_template(&security_table);
    unregister_pernet_subsys(&iptable_security_net_ops);
    kfree(sectbl_ops);
    }
    module_init(iptable_security_init);
    module_exit(iptable_security_fini);
