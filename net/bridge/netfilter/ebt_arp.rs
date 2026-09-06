//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_arp.c
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
// ebt_arp
//
// Authors:
// Bart De Schuymer <bdschuym@pandora.be>
// Tim Gardner <timg@tpi.com>
//
// April, 2002
//

    static bool
    ebt_arp_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct ebt_arp_info *info = par.matchinfo;
    const struct arphdr *ah;
    struct arphdr _arph;
    ah = skb_header_pointer(skb, 0, sizeof(_arph), &_arph);
    if (ah == core::ptr::null_mut())
    return false;
    if ((info.bitmask & EBT_ARP_OPCODE) &&
    NF_INVF(info, EBT_ARP_OPCODE, info.opcode != ah.ar_op))
    return false;
    if ((info.bitmask & EBT_ARP_HTYPE) &&
    NF_INVF(info, EBT_ARP_HTYPE, info.htype != ah.ar_hrd))
    return false;
    if ((info.bitmask & EBT_ARP_PTYPE) &&
    NF_INVF(info, EBT_ARP_PTYPE, info.ptype != ah.ar_pro))
    return false;
    if (info.bitmask & (EBT_ARP_SRC_IP | EBT_ARP_DST_IP | EBT_ARP_GRAT)) {
    const __be32 *sap, *dap;
    __be32 saddr, daddr;
    if (ah.ar_pln != sizeof(__be32) || ah.ar_pro != htons(ETH_P_IP))
    return false;
    sap = skb_header_pointer(skb, sizeof(struct arphdr) +
    ah.ar_hln, sizeof(saddr),
    &saddr);
    if (sap == core::ptr::null_mut())
    return false;
    dap = skb_header_pointer(skb, sizeof(struct arphdr) +
    2*ah.ar_hln+sizeof(saddr),
    sizeof(daddr), &daddr);
    if (dap == core::ptr::null_mut())
    return false;
    if ((info.bitmask & EBT_ARP_SRC_IP) &&
    NF_INVF(info, EBT_ARP_SRC_IP,
    info.saddr != (*sap & info.smsk)))
    return false;
    if ((info.bitmask & EBT_ARP_DST_IP) &&
    NF_INVF(info, EBT_ARP_DST_IP,
    info.daddr != (*dap & info.dmsk)))
    return false;
    if ((info.bitmask & EBT_ARP_GRAT) &&
    NF_INVF(info, EBT_ARP_GRAT, *dap != *sap))
    return false;
    }
    if (info.bitmask & (EBT_ARP_SRC_MAC | EBT_ARP_DST_MAC)) {
    const unsigned char *mp;
    unsigned char _mac[ETH_ALEN];
    if (ah.ar_hln != ETH_ALEN || ah.ar_hrd != htons(ARPHRD_ETHER))
    return false;
    if (info.bitmask & EBT_ARP_SRC_MAC) {
    mp = skb_header_pointer(skb, sizeof(struct arphdr),
    sizeof(_mac), &_mac);
    if (mp == core::ptr::null_mut())
    return false;
    if (NF_INVF(info, EBT_ARP_SRC_MAC,
    !ether_addr_equal_masked(mp, info.smaddr,
    info.smmsk)))
    return false;
    }
    if (info.bitmask & EBT_ARP_DST_MAC) {
    mp = skb_header_pointer(skb, sizeof(struct arphdr) +
    ah.ar_hln + ah.ar_pln,
    sizeof(_mac), &_mac);
    if (mp == core::ptr::null_mut())
    return false;
    if (NF_INVF(info, EBT_ARP_DST_MAC,
    !ether_addr_equal_masked(mp, info.dmaddr,
    info.dmmsk)))
    return false;
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ebt_arp_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int ebt_arp_mt_check(const struct xt_mtchk_param *par)
    {
    const struct ebt_arp_info *info = par.matchinfo;
    const struct ebt_entry *e = par.entryinfo;
    if ((e.ethproto != htons(ETH_P_ARP) &&
    e.ethproto != htons(ETH_P_RARP)) ||
    e.invflags & EBT_IPROTO)
    return -EINVAL;
    if (info.bitmask & ~EBT_ARP_MASK || info.invflags & ~EBT_ARP_MASK)
    return -EINVAL;
    return 0;
    }
    static struct xt_match ebt_arp_mt_reg __read_mostly = {
    .name		= "arp",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .match		= ebt_arp_mt,
    .checkentry	= ebt_arp_mt_check,
    .matchsize	= sizeof(struct ebt_arp_info),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_arp_init() -> int __init {
    static int __init ebt_arp_init(void)
    {
    return xt_register_match(&ebt_arp_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_arp_fini() -> void __exit {
    static void __exit ebt_arp_fini(void)
    {
    xt_unregister_match(&ebt_arp_mt_reg);
    }
    module_init(ebt_arp_init);
    module_exit(ebt_arp_fini);
    MODULE_DESCRIPTION("Ebtables: ARP protocol packet match");
    MODULE_LICENSE("GPL");
