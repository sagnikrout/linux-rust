//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_ipcomp.c
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
// Kernel module to match IPComp parameters for IPv4 and IPv6
//
// Copyright (C) 2013 WindRiver
//
// Author:
// Fan Du <fan.du@windriver.com>
//
// Based on:
// net/netfilter/xt_esp.c
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Fan Du <fan.du@windriver.com>");
    MODULE_DESCRIPTION("Xtables: IPv4/6 IPsec-IPComp SPI match");
    MODULE_ALIAS("ipt_ipcomp");
    MODULE_ALIAS("ip6t_ipcomp");
// Returns 1 if the spi is matched by the range, 0 otherwise
    static inline bool
    spi_match(u_int32_t min, u_int32_t max, u_int32_t spi, bool invert)
    {
    return (spi >= min && spi <= max) ^ invert;
    }
#[no_mangle]
unsafe extern "C" fn comp_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool comp_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    struct ip_comp_hdr _comphdr;
    const struct ip_comp_hdr *chdr;
    const struct xt_ipcomp *compinfo = par.matchinfo;
// Must not be a fragment.
    if (par.fragoff != 0)
    return false;
    chdr = skb_header_pointer(skb, par.thoff, sizeof(_comphdr), &_comphdr);
    if (chdr == core::ptr::null_mut()) {
// We've been asked to examine this packet, and we
// can't.  Hence, no choice but to drop.
//
    par.hotdrop = true;
    return false;
    }
    return spi_match(compinfo.spis[0], compinfo.spis[1],
    ntohs(chdr.cpi),
    !!(compinfo.invflags & XT_IPCOMP_INV_SPI));
    }
#[no_mangle]
unsafe extern "C" fn comp_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int comp_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_ipcomp *compinfo = par.matchinfo;
// Must specify no unknown invflags
    if (compinfo.invflags & ~XT_IPCOMP_INV_MASK) {
    pr_info_ratelimited("unknown flags %X\n", compinfo.invflags);
    return -EINVAL;
    }
    return 0;
    }
    static struct xt_match comp_mt_reg[] __read_mostly = {
    {
    .name		= "ipcomp",
    .family		= NFPROTO_IPV4,
    .match		= comp_mt,
    .matchsize	= sizeof(struct xt_ipcomp),
    .proto		= IPPROTO_COMP,
    .checkentry	= comp_mt_check,
    .me		= THIS_MODULE,
    },
    {
    .name		= "ipcomp",
    .family		= NFPROTO_IPV6,
    .match		= comp_mt,
    .matchsize	= sizeof(struct xt_ipcomp),
    .proto		= IPPROTO_COMP,
    .checkentry	= comp_mt_check,
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn comp_mt_init() -> int __init {
    static int __init comp_mt_init(void)
    {
    return xt_register_matches(comp_mt_reg, ARRAY_SIZE(comp_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn comp_mt_exit() -> void __exit {
    static void __exit comp_mt_exit(void)
    {
    xt_unregister_matches(comp_mt_reg, ARRAY_SIZE(comp_mt_reg));
    }
    module_init(comp_mt_init);
    module_exit(comp_mt_exit);
