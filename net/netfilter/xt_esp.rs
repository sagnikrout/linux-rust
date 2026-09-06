//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_esp.c
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
// Kernel module to match ESP parameters.
// (C) 1999-2000 Yon Uriarte <yon@astaro.de>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Yon Uriarte <yon@astaro.de>");
    MODULE_DESCRIPTION("Xtables: IPsec-ESP packet match");
    MODULE_ALIAS("ipt_esp");
    MODULE_ALIAS("ip6t_esp");
// Returns 1 if the spi is matched by the range, 0 otherwise
    static inline bool
    spi_match(u_int32_t min, u_int32_t max, u_int32_t spi, bool invert)
    {
    return (spi >= min && spi <= max) ^ invert;
    }
#[no_mangle]
unsafe extern "C" fn esp_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool esp_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct ip_esp_hdr *eh;
    struct ip_esp_hdr _esp;
    const struct xt_esp *espinfo = par.matchinfo;
// Must not be a fragment.
    if (par.fragoff != 0)
    return false;
    eh = skb_header_pointer(skb, par.thoff, sizeof(_esp), &_esp);
    if (eh == core::ptr::null_mut()) {
// We've been asked to examine this packet, and we
// can't.  Hence, no choice but to drop.
//
    par.hotdrop = true;
    return false;
    }
    return spi_match(espinfo.spis[0], espinfo.spis[1], ntohl(eh.spi),
    !!(espinfo.invflags & XT_ESP_INV_SPI));
    }
#[no_mangle]
unsafe extern "C" fn esp_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int esp_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_esp *espinfo = par.matchinfo;
    if (espinfo.invflags & ~XT_ESP_INV_MASK) {
    pr_info_ratelimited("unknown flags %X\n", espinfo.invflags);
    return -EINVAL;
    }
    return 0;
    }
    static struct xt_match esp_mt_reg[] __read_mostly = {
    {
    .name		= "esp",
    .family		= NFPROTO_IPV4,
    .checkentry	= esp_mt_check,
    .match		= esp_mt,
    .matchsize	= sizeof(struct xt_esp),
    .proto		= IPPROTO_ESP,
    .me		= THIS_MODULE,
    },
    {
    .name		= "esp",
    .family		= NFPROTO_IPV6,
    .checkentry	= esp_mt_check,
    .match		= esp_mt,
    .matchsize	= sizeof(struct xt_esp),
    .proto		= IPPROTO_ESP,
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn esp_mt_init() -> int __init {
    static int __init esp_mt_init(void)
    {
    return xt_register_matches(esp_mt_reg, ARRAY_SIZE(esp_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn esp_mt_exit() -> void __exit {
    static void __exit esp_mt_exit(void)
    {
    xt_unregister_matches(esp_mt_reg, ARRAY_SIZE(esp_mt_reg));
    }
    module_init(esp_mt_init);
    module_exit(esp_mt_exit);
