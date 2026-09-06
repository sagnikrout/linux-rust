//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/arptable_filter.c
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
// Filtering ARP tables module.
//
// Copyright (C) 2002 David S. Miller (davem@redhat.com)
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David S. Miller <davem@redhat.com>");
    MODULE_DESCRIPTION("arptables filter table");

    (1 << NF_ARP_FORWARD))
    static const struct xt_table packet_filter = {
    .name		= "filter",
    .valid_hooks	= FILTER_VALID_HOOKS,
    .me		= THIS_MODULE,
    .af		= NFPROTO_ARP,
    .priority	= NF_IP_PRI_FILTER,
    };
    static struct nf_hook_ops *arpfilter_ops __read_mostly;
#[no_mangle]
unsafe extern "C" fn arptable_filter_table_init(net: *mut net) -> c_int {
    static int arptable_filter_table_init(struct net *net)
    {
    struct arpt_replace *repl;
    int err;
    repl = arpt_alloc_initial_table(&packet_filter);
    if (repl == core::ptr::null_mut())
    return -ENOMEM;
    err = arpt_register_table(net, &packet_filter, repl, arpfilter_ops);
    kfree(repl);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn arptable_filter_net_pre_exit(net: *mut net) -> void __net_exit {
    static void __net_exit arptable_filter_net_pre_exit(struct net *net)
    {
    xt_unregister_table_pre_exit(net, NFPROTO_ARP, "filter");
    }
#[no_mangle]
unsafe extern "C" fn arptable_filter_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit arptable_filter_net_exit(struct net *net)
    {
    arpt_unregister_table(net, "filter");
    }
    static struct pernet_operations arptable_filter_net_ops = {
    .exit = arptable_filter_net_exit,
    .pre_exit = arptable_filter_net_pre_exit,
    };
#[no_mangle]
unsafe extern "C" fn arptable_filter_init() -> int __init {
    static int __init arptable_filter_init(void)
    {
    int ret;
    arpfilter_ops = xt_hook_ops_alloc(&packet_filter, arpt_do_table);
    if (IS_ERR(arpfilter_ops))
    return PTR_ERR(arpfilter_ops);
    ret = register_pernet_subsys(&arptable_filter_net_ops);
    if (ret < 0)
    goto err_free;
    ret = xt_register_template(&packet_filter,
    arptable_filter_table_init);
    if (ret < 0) {
    unregister_pernet_subsys(&arptable_filter_net_ops);
    goto err_free;
    }
    return 0;
    err_free:
    kfree(arpfilter_ops);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn arptable_filter_fini() -> void __exit {
    static void __exit arptable_filter_fini(void)
    {
    xt_unregister_template(&packet_filter);
    unregister_pernet_subsys(&arptable_filter_net_ops);
    kfree(arpfilter_ops);
    }
    module_init(arptable_filter_init);
    module_exit(arptable_filter_fini);
