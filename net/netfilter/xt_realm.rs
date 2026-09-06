//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_realm.c
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
// IP tables module for matching the routing realm
//
// (C) 2003 by Sampsa Ranta <sampsa@netsonic.fi>
//

    MODULE_AUTHOR("Sampsa Ranta <sampsa@netsonic.fi>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Xtables: Routing realm match");
    MODULE_ALIAS("ipt_realm");
    static bool
    realm_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_realm_info *info = par.matchinfo;
    const struct dst_entry *dst = skb_dst(skb);
    return (info.id == (dst.tclassid & info.mask)) ^ info.invert;
    }
    static struct xt_match realm_mt_reg __read_mostly = {
    .name		= "realm",
    .match		= realm_mt,
    .matchsize	= sizeof(struct xt_realm_info),
    .hooks		= (1 << NF_INET_POST_ROUTING) | (1 << NF_INET_FORWARD) |
    (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_LOCAL_IN),
    .family		= NFPROTO_IPV4,
    .me		= THIS_MODULE
    };
#[no_mangle]
unsafe extern "C" fn realm_mt_init() -> int __init {
    static int __init realm_mt_init(void)
    {
    return xt_register_match(&realm_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn realm_mt_exit() -> void __exit {
    static void __exit realm_mt_exit(void)
    {
    xt_unregister_match(&realm_mt_reg);
    }
    module_init(realm_mt_init);
    module_exit(realm_mt_exit);
