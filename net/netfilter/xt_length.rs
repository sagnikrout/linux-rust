//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_length.c
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
// Kernel module to match packet length.
// (C) 1999-2001 James Morris <jmorros@intercode.com.au>
//

    MODULE_AUTHOR("James Morris <jmorris@intercode.com.au>");
    MODULE_DESCRIPTION("Xtables: Packet length (Layer3,4,5) match");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_length");
    MODULE_ALIAS("ip6t_length");
    static bool
    length_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_length_info *info = par.matchinfo;
    let mut pktlen: u32 = skb_ip_totlen(skb);
    return (pktlen >= info.min && pktlen <= info.max) ^ info.invert;
    }
    static bool
    length_mt6(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_length_info *info = par.matchinfo;
    let mut pktlen: u32 = skb.len;
    return (pktlen >= info.min && pktlen <= info.max) ^ info.invert;
    }
    static struct xt_match length_mt_reg[] __read_mostly = {
    {
    .name		= "length",
    .family		= NFPROTO_IPV4,
    .match		= length_mt,
    .matchsize	= sizeof(struct xt_length_info),
    .me		= THIS_MODULE,
    },
    {
    .name		= "length",
    .family		= NFPROTO_IPV6,
    .match		= length_mt6,
    .matchsize	= sizeof(struct xt_length_info),
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn length_mt_init() -> int __init {
    static int __init length_mt_init(void)
    {
    return xt_register_matches(length_mt_reg, ARRAY_SIZE(length_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn length_mt_exit() -> void __exit {
    static void __exit length_mt_exit(void)
    {
    xt_unregister_matches(length_mt_reg, ARRAY_SIZE(length_mt_reg));
    }
    module_init(length_mt_init);
    module_exit(length_mt_exit);
