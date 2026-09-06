//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_snat.c
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
// ebt_snat
//
// Authors:
// Bart De Schuymer <bdschuym@pandora.be>
//
// June, 2002
//

    static unsigned int
    ebt_snat_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct ebt_nat_info *info = par.targinfo;
    if (skb_ensure_writable(skb, 0))
    return EBT_DROP;
    ether_addr_copy(eth_hdr(skb).h_source, info.mac);
    if (!(info.target & NAT_ARP_BIT) &&
    eth_hdr(skb).h_proto == htons(ETH_P_ARP)) {
    const struct arphdr *ap;
    struct arphdr _ah;
    if (skb_ensure_writable(skb, sizeof(_ah) + ETH_ALEN))
    return EBT_DROP;
    ap = skb_header_pointer(skb, 0, sizeof(_ah), &_ah);
    if (ap == core::ptr::null_mut())
    return EBT_DROP;
    if (ap.ar_hln != ETH_ALEN)
    goto out;
    if (skb_store_bits(skb, sizeof(_ah), info.mac, ETH_ALEN))
    return EBT_DROP;
    }
    out:
    return info.target | ~EBT_VERDICT_BITS;
    }
#[no_mangle]
unsafe extern "C" fn ebt_snat_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int ebt_snat_tg_check(const struct xt_tgchk_param *par)
    {
    const struct ebt_nat_info *info = par.targinfo;
    int tmp;
    tmp = info.target | ~EBT_VERDICT_BITS;
    if (BASE_CHAIN && tmp == EBT_RETURN)
    return -EINVAL;
    if (ebt_invalid_target(tmp))
    return -EINVAL;
    tmp = info.target | EBT_VERDICT_BITS;
    if ((tmp & ~NAT_ARP_BIT) != ~NAT_ARP_BIT)
    return -EINVAL;
    return 0;
    }
    static struct xt_target ebt_snat_tg_reg __read_mostly = {
    .name		= "snat",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .table		= "nat",
    .hooks		= (1 << NF_BR_NUMHOOKS) | (1 << NF_BR_POST_ROUTING),
    .target		= ebt_snat_tg,
    .checkentry	= ebt_snat_tg_check,
    .targetsize	= sizeof(struct ebt_nat_info),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_snat_init() -> int __init {
    static int __init ebt_snat_init(void)
    {
    return xt_register_target(&ebt_snat_tg_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_snat_fini() -> void __exit {
    static void __exit ebt_snat_fini(void)
    {
    xt_unregister_target(&ebt_snat_tg_reg);
    }
    module_init(ebt_snat_init);
    module_exit(ebt_snat_fini);
    MODULE_DESCRIPTION("Ebtables: Source MAC address translation");
    MODULE_LICENSE("GPL");
