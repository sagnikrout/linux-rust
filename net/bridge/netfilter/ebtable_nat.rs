//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebtable_nat.c
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
// ebtable_nat
//
// Authors:
// Bart De Schuymer <bdschuym@pandora.be>
//
// April, 2002
//

    (1 << NF_BR_POST_ROUTING))
    static struct ebt_entries initial_chains[] = {
    {
    .name	= "PREROUTING",
    .policy	= EBT_ACCEPT,
    },
    {
    .name	= "OUTPUT",
    .policy	= EBT_ACCEPT,
    },
    {
    .name	= "POSTROUTING",
    .policy	= EBT_ACCEPT,
    }
    };
    static struct ebt_replace_kernel initial_table = {
    .name		= "nat",
    .valid_hooks	= NAT_VALID_HOOKS,
    .entries_size	= 3 * sizeof(struct ebt_entries),
    .hook_entry	= {
    [NF_BR_PRE_ROUTING]	= &initial_chains[0],
    [NF_BR_LOCAL_OUT]	= &initial_chains[1],
    [NF_BR_POST_ROUTING]	= &initial_chains[2],
    },
    .entries	= (char *)initial_chains,
    };
    static const struct ebt_table frame_nat = {
    .name		= "nat",
    .table		= &initial_table,
    .valid_hooks	= NAT_VALID_HOOKS,
    .me		= THIS_MODULE,
    };
    static const struct nf_hook_ops ebt_ops_nat[] = {
    {
    .hook		= ebt_do_table,
    .pf		= NFPROTO_BRIDGE,
    .hooknum	= NF_BR_LOCAL_OUT,
    .priority	= NF_BR_PRI_NAT_DST_OTHER,
    },
    {
    .hook		= ebt_do_table,
    .pf		= NFPROTO_BRIDGE,
    .hooknum	= NF_BR_POST_ROUTING,
    .priority	= NF_BR_PRI_NAT_SRC,
    },
    {
    .hook		= ebt_do_table,
    .pf		= NFPROTO_BRIDGE,
    .hooknum	= NF_BR_PRE_ROUTING,
    .priority	= NF_BR_PRI_NAT_DST_BRIDGED,
    },
    };
#[no_mangle]
unsafe extern "C" fn frame_nat_table_init(net: *mut net) -> c_int {
    static int frame_nat_table_init(struct net *net)
    {
    return ebt_register_table(net, &frame_nat, ebt_ops_nat);
    }
#[no_mangle]
unsafe extern "C" fn frame_nat_net_pre_exit(net: *mut net) -> void __net_exit {
    static void __net_exit frame_nat_net_pre_exit(struct net *net)
    {
    ebt_unregister_table_pre_exit(net, "nat");
    }
#[no_mangle]
unsafe extern "C" fn frame_nat_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit frame_nat_net_exit(struct net *net)
    {
    ebt_unregister_table(net, "nat");
    }
    static struct pernet_operations frame_nat_net_ops = {
    .exit = frame_nat_net_exit,
    .pre_exit = frame_nat_net_pre_exit,
    };
#[no_mangle]
unsafe extern "C" fn ebtable_nat_init() -> int __init {
    static int __init ebtable_nat_init(void)
    {
    let mut ret: c_int = register_pernet_subsys(&frame_nat_net_ops);
    if (ret)
    return ret;
    ret = ebt_register_template(&frame_nat, frame_nat_table_init);
    if (ret)
    unregister_pernet_subsys(&frame_nat_net_ops);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ebtable_nat_fini() -> void __exit {
    static void __exit ebtable_nat_fini(void)
    {
    ebt_unregister_template(&frame_nat);
    unregister_pernet_subsys(&frame_nat_net_ops);
    }
    module_init(ebtable_nat_init);
    module_exit(ebtable_nat_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ebtables legacy stateless nat table");
