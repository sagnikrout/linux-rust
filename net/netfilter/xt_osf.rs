//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_osf.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2003+ Evgeniy Polyakov <zbr@ioremap.net>
//

    static bool
    xt_osf_match_packet(const struct sk_buff *skb, struct xt_action_param *p)
    {
    if (p.fragoff)
    return false;
    return nf_osf_match(skb, xt_family(p), xt_hooknum(p), xt_in(p),
    xt_out(p), p.matchinfo, xt_net(p), nf_osf_fingers);
    }
    static struct xt_match xt_osf_match = {
    .name 		= "osf",
    .revision	= 0,
    .family		= NFPROTO_IPV4,
    .proto		= IPPROTO_TCP,
    .hooks      	= (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_FORWARD),
    .match 		= xt_osf_match_packet,
    .matchsize	= sizeof(struct xt_osf_info),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn xt_osf_init() -> int __init {
    static int __init xt_osf_init(void)
    {
    int err;
    err = xt_register_match(&xt_osf_match);
    if (err) {
    pr_err("Failed to register OS fingerprint "
    "matching module (%d)\n", err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xt_osf_fini() -> void __exit {
    static void __exit xt_osf_fini(void)
    {
    xt_unregister_match(&xt_osf_match);
    }
    module_init(xt_osf_init);
    module_exit(xt_osf_fini);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Evgeniy Polyakov <zbr@ioremap.net>");
    MODULE_DESCRIPTION("Passive OS fingerprint matching.");
    MODULE_ALIAS("ipt_osf");
    MODULE_ALIAS("ip6t_osf");
