//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_pkttype.c
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
// ebt_pkttype
//
// Authors:
// Bart De Schuymer <bdschuym@pandora.be>
//
// April, 2003
//

    static bool
    ebt_pkttype_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct ebt_pkttype_info *info = par.matchinfo;
    return (skb.pkt_type == info.pkt_type) ^ info.invert;
    }
#[no_mangle]
unsafe extern "C" fn ebt_pkttype_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int ebt_pkttype_mt_check(const struct xt_mtchk_param *par)
    {
    const struct ebt_pkttype_info *info = par.matchinfo;
    if (info.invert != 0 && info.invert != 1)
    return -EINVAL;
// Allow any pkt_type value
    return 0;
    }
    static struct xt_match ebt_pkttype_mt_reg __read_mostly = {
    .name		= "pkttype",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .match		= ebt_pkttype_mt,
    .checkentry	= ebt_pkttype_mt_check,
    .matchsize	= sizeof(struct ebt_pkttype_info),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_pkttype_init() -> int __init {
    static int __init ebt_pkttype_init(void)
    {
    return xt_register_match(&ebt_pkttype_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_pkttype_fini() -> void __exit {
    static void __exit ebt_pkttype_fini(void)
    {
    xt_unregister_match(&ebt_pkttype_mt_reg);
    }
    module_init(ebt_pkttype_init);
    module_exit(ebt_pkttype_fini);
    MODULE_DESCRIPTION("Ebtables: Link layer packet type match");
    MODULE_LICENSE("GPL");
