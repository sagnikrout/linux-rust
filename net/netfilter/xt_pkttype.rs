//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_pkttype.c
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
// (C) 1999-2001 Michal Ludvig <michal@logix.cz>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Michal Ludvig <michal@logix.cz>");
    MODULE_DESCRIPTION("Xtables: link layer packet type match");
    MODULE_ALIAS("ipt_pkttype");
    MODULE_ALIAS("ip6t_pkttype");
    static bool
    pkttype_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_pkttype_info *info = par.matchinfo;
    u_int8_t type;
    if (skb.pkt_type != PACKET_LOOPBACK)
    type = skb.pkt_type;
    else if (xt_family(par) == NFPROTO_IPV4 &&
    ipv4_is_multicast(ip_hdr(skb).daddr))
    type = PACKET_MULTICAST;
#[no_mangle]
pub unsafe extern "C" fn if(NFPROTO_IPV6: xt_family(par) ==) -> else {
    else if (xt_family(par) == NFPROTO_IPV6)
    type = PACKET_MULTICAST;
    else
    type = PACKET_BROADCAST;
    return (type == info.pkttype) ^ info.invert;
    }
    static struct xt_match pkttype_mt_reg __read_mostly = {
    .name      = "pkttype",
    .revision  = 0,
    .family    = NFPROTO_UNSPEC,
    .match     = pkttype_mt,
    .matchsize = sizeof(struct xt_pkttype_info),
    .me        = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn pkttype_mt_init() -> int __init {
    static int __init pkttype_mt_init(void)
    {
    return xt_register_match(&pkttype_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn pkttype_mt_exit() -> void __exit {
    static void __exit pkttype_mt_exit(void)
    {
    xt_unregister_match(&pkttype_mt_reg);
    }
    module_init(pkttype_mt_init);
    module_exit(pkttype_mt_exit);
