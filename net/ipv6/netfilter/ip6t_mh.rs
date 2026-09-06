//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/ip6t_mh.c
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
// Copyright (C)2006 USAGI/WIDE Project
//
// Author:
// Masahide NAKAMURA @USAGI <masahide.nakamura.cz@hitachi.com>
//
// Based on net/netfilter/xt_tcpudp.c
//

    MODULE_DESCRIPTION("Xtables: IPv6 Mobility Header match");
    MODULE_LICENSE("GPL");
// Returns 1 if the type is matched by the range, 0 otherwise
    static inline bool
    type_match(u_int8_t min, u_int8_t max, u_int8_t type, bool invert)
    {
    return (type >= min && type <= max) ^ invert;
    }
#[no_mangle]
unsafe extern "C" fn mh_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool mh_mt6(const struct sk_buff *skb, struct xt_action_param *par)
    {
    struct ip6_mh _mh;
    const struct ip6_mh *mh;
    const struct ip6t_mh *mhinfo = par.matchinfo;
// Must not be a fragment.
    if (par.fragoff != 0)
    return false;
    mh = skb_header_pointer(skb, par.thoff, sizeof(_mh), &_mh);
    if (mh == core::ptr::null_mut()) {
// We've been asked to examine this packet, and we
    can't.  Hence, no choice but to drop. */
    par.hotdrop = true;
    return false;
    }
    if (mh.ip6mh_proto != IPPROTO_NONE) {
    par.hotdrop = true;
    return false;
    }
    return type_match(mhinfo.types[0], mhinfo.types[1], mh.ip6mh_type,
    !!(mhinfo.invflags & IP6T_MH_INV_TYPE));
    }
#[no_mangle]
unsafe extern "C" fn mh_mt6_check(par: *const xt_mtchk_param) -> c_int {
    static int mh_mt6_check(const struct xt_mtchk_param *par)
    {
    const struct ip6t_mh *mhinfo = par.matchinfo;
// Must specify no unknown invflags
    return (mhinfo.invflags & ~IP6T_MH_INV_MASK) ? -EINVAL : 0;
    }
    static struct xt_match mh_mt6_reg __read_mostly = {
    .name		= "mh",
    .family		= NFPROTO_IPV6,
    .checkentry	= mh_mt6_check,
    .match		= mh_mt6,
    .matchsize	= sizeof(struct ip6t_mh),
    .proto		= IPPROTO_MH,
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn mh_mt6_init() -> int __init {
    static int __init mh_mt6_init(void)
    {
    return xt_register_match(&mh_mt6_reg);
    }
#[no_mangle]
unsafe extern "C" fn mh_mt6_exit() -> void __exit {
    static void __exit mh_mt6_exit(void)
    {
    xt_unregister_match(&mh_mt6_reg);
    }
    module_init(mh_mt6_init);
    module_exit(mh_mt6_exit);
