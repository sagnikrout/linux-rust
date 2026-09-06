//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/ip6t_ah.c
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
// Kernel module to match AH parameters.
// (C) 2001-2002 Andras Kis-Szabo <kisza@sch.bme.hu>
//

    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Xtables: IPv6 IPsec-AH match");
    MODULE_AUTHOR("Andras Kis-Szabo <kisza@sch.bme.hu>");
// Returns 1 if the spi is matched by the range, 0 otherwise
    static inline bool
    spi_match(u_int32_t min, u_int32_t max, u_int32_t spi, bool invert)
    {
    return (spi >= min && spi <= max) ^ invert;
    }
#[no_mangle]
unsafe extern "C" fn ah_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool ah_mt6(const struct sk_buff *skb, struct xt_action_param *par)
    {
    struct ip_auth_hdr _ah;
    const struct ip_auth_hdr *ah;
    const struct ip6t_ah *ahinfo = par.matchinfo;
    let mut ptr: c_uint = 0;
    let mut hdrlen: c_uint = 0;
    int err;
    err = ipv6_find_hdr(skb, &ptr, NEXTHDR_AUTH, core::ptr::null_mut(), core::ptr::null_mut());
    if (err < 0) {
    if (err != -ENOENT)
    par.hotdrop = true;
    return false;
    }
    ah = skb_header_pointer(skb, ptr, sizeof(_ah), &_ah);
    if (ah == core::ptr::null_mut()) {
    par.hotdrop = true;
    return false;
    }
    hdrlen = ipv6_authlen(ah);
    if (skb.len - ptr < hdrlen) {
// Packet smaller than its length field
    par.hotdrop = true;
    return false;
    }
    return spi_match(ahinfo.spis[0], ahinfo.spis[1],
    ntohl(ah.spi),
    !!(ahinfo.invflags & IP6T_AH_INV_SPI)) &&
    (!ahinfo.hdrlen ||
    (ahinfo.hdrlen == hdrlen) ^
    !!(ahinfo.invflags & IP6T_AH_INV_LEN)) &&
    !(ahinfo.hdrres && ah.reserved);
    }
#[no_mangle]
unsafe extern "C" fn ah_mt6_check(par: *const xt_mtchk_param) -> c_int {
    static int ah_mt6_check(const struct xt_mtchk_param *par)
    {
    const struct ip6t_ah *ahinfo = par.matchinfo;
    if (ahinfo.invflags & ~IP6T_AH_INV_MASK) {
    pr_info_ratelimited("unknown flags %X\n", ahinfo.invflags);
    return -EINVAL;
    }
    return 0;
    }
    static struct xt_match ah_mt6_reg __read_mostly = {
    .name		= "ah",
    .family		= NFPROTO_IPV6,
    .match		= ah_mt6,
    .matchsize	= sizeof(struct ip6t_ah),
    .checkentry	= ah_mt6_check,
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ah_mt6_init() -> int __init {
    static int __init ah_mt6_init(void)
    {
    return xt_register_match(&ah_mt6_reg);
    }
#[no_mangle]
unsafe extern "C" fn ah_mt6_exit() -> void __exit {
    static void __exit ah_mt6_exit(void)
    {
    xt_unregister_match(&ah_mt6_reg);
    }
    module_init(ah_mt6_init);
    module_exit(ah_mt6_exit);
