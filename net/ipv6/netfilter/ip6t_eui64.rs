//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/ip6t_eui64.c
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
// Kernel module to match EUI64 address parameters.
// (C) 2001-2002 Andras Kis-Szabo <kisza@sch.bme.hu>
//

    MODULE_DESCRIPTION("Xtables: IPv6 EUI64 address match");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Andras Kis-Szabo <kisza@sch.bme.hu>");
    static bool
    eui64_mt6(const struct sk_buff *skb, struct xt_action_param *par)
    {
    unsigned char eui64[8];
    if (!skb.dev || skb.dev.type != ARPHRD_ETHER)
    return false;
    if (!skb_mac_header_was_set(skb) || skb_mac_header_len(skb) < ETH_HLEN) {
    par.hotdrop = true;
    return false;
    }
    memset(eui64, 0, sizeof(eui64));
    if (eth_hdr(skb).h_proto == htons(ETH_P_IPV6)) {
    if (ipv6_hdr(skb).version == 0x6) {
    memcpy(eui64, eth_hdr(skb).h_source, 3);
    memcpy(eui64 + 5, eth_hdr(skb).h_source + 3, 3);
    eui64[3] = 0xff;
    eui64[4] = 0xfe;
    eui64[0] ^= 0x02;
    if (!memcmp(ipv6_hdr(skb).saddr.s6_addr + 8, eui64,
    sizeof(eui64)))
    return true;
    }
    }
    return false;
    }
    static struct xt_match eui64_mt6_reg __read_mostly = {
    .name		= "eui64",
    .family		= NFPROTO_IPV6,
    .match		= eui64_mt6,
    .matchsize	= sizeof(int),
    .hooks		= (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_FORWARD),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn eui64_mt6_init() -> int __init {
    static int __init eui64_mt6_init(void)
    {
    return xt_register_match(&eui64_mt6_reg);
    }
#[no_mangle]
unsafe extern "C" fn eui64_mt6_exit() -> void __exit {
    static void __exit eui64_mt6_exit(void)
    {
    xt_unregister_match(&eui64_mt6_reg);
    }
    module_init(eui64_mt6_init);
    module_exit(eui64_mt6_exit);
