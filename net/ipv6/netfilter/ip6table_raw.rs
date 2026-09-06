//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/ip6table_raw.c
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
// IPv6 raw table, a port of the IPv4 raw table to IPv6
//
// Copyright (C) 2003 Jozsef Kadlecsik <kadlec@netfilter.org>
//

    static bool raw_before_defrag __read_mostly;
    MODULE_PARM_DESC(raw_before_defrag, "Enable raw table before defrag");
    module_param(raw_before_defrag, bool, 0000);
    static const struct xt_table packet_raw = {
    .name = "raw",
    .valid_hooks = RAW_VALID_HOOKS,
    .me = THIS_MODULE,
    .af = NFPROTO_IPV6,
    .priority = NF_IP6_PRI_RAW,
    };
    static const struct xt_table packet_raw_before_defrag = {
    .name = "raw",
    .valid_hooks = RAW_VALID_HOOKS,
    .me = THIS_MODULE,
    .af = NFPROTO_IPV6,
    .priority = NF_IP6_PRI_RAW_BEFORE_DEFRAG,
    };
    static struct nf_hook_ops *rawtable_ops __read_mostly;
#[no_mangle]
unsafe extern "C" fn ip6table_raw_table_init(net: *mut net) -> c_int {
    static int ip6table_raw_table_init(struct net *net)
    {
    struct ip6t_replace *repl;
    const struct xt_table *table = &packet_raw;
    int ret;
    if (raw_before_defrag)
    table = &packet_raw_before_defrag;
    repl = ip6t_alloc_initial_table(table);
    if (repl == core::ptr::null_mut())
    return -ENOMEM;
    ret = ip6t_register_table(net, table, repl, rawtable_ops);
    kfree(repl);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ip6table_raw_net_pre_exit(net: *mut net) -> void __net_exit {
    static void __net_exit ip6table_raw_net_pre_exit(struct net *net)
    {
    xt_unregister_table_pre_exit(net, NFPROTO_IPV6, "raw");
    }
#[no_mangle]
unsafe extern "C" fn ip6table_raw_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit ip6table_raw_net_exit(struct net *net)
    {
    ip6t_unregister_table_exit(net, "raw");
    }
    static struct pernet_operations ip6table_raw_net_ops = {
    .pre_exit = ip6table_raw_net_pre_exit,
    .exit = ip6table_raw_net_exit,
    };
#[no_mangle]
unsafe extern "C" fn ip6table_raw_init() -> int __init {
    static int __init ip6table_raw_init(void)
    {
    const struct xt_table *table = &packet_raw;
    int ret;
    if (raw_before_defrag) {
    table = &packet_raw_before_defrag;
    pr_info("Enabling raw table before defrag\n");
    }
// Register hooks
    rawtable_ops = xt_hook_ops_alloc(table, ip6t_do_table);
    if (IS_ERR(rawtable_ops))
    return PTR_ERR(rawtable_ops);
    ret = register_pernet_subsys(&ip6table_raw_net_ops);
    if (ret < 0)
    goto err_free;
    ret = xt_register_template(table, ip6table_raw_table_init);
    if (ret < 0) {
    unregister_pernet_subsys(&ip6table_raw_net_ops);
    goto err_free;
    }
    return 0;
    err_free:
    kfree(rawtable_ops);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ip6table_raw_fini() -> void __exit {
    static void __exit ip6table_raw_fini(void)
    {
    xt_unregister_template(&packet_raw);
    unregister_pernet_subsys(&ip6table_raw_net_ops);
    kfree(rawtable_ops);
    }
    module_init(ip6table_raw_init);
    module_exit(ip6table_raw_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Ip6tables legacy raw table");
