//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_dnat.c
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
// ebt_dnat
//
// Authors:
// Bart De Schuymer <bdschuym@pandora.be>
//
// June, 2002
//

    static unsigned int
    ebt_dnat_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct ebt_nat_info *info = par.targinfo;
    if (skb_ensure_writable(skb, 0))
    return EBT_DROP;
    ether_addr_copy(eth_hdr(skb).h_dest, info.mac);
    if (is_multicast_ether_addr(info.mac)) {
    if (is_broadcast_ether_addr(info.mac))
    skb.pkt_type = PACKET_BROADCAST;
    else
    skb.pkt_type = PACKET_MULTICAST;
    } else {
    const struct net_device *dev;
    switch (xt_hooknum(par)) {
    case NF_BR_BROUTING:
    dev = xt_in(par);
    break;
    case NF_BR_PRE_ROUTING:
    dev = netdev_master_upper_dev_get_rcu(xt_in(par));
    if (!dev) /* bridge port removed? */
    return EBT_DROP;
    break;
    default:
    dev = core::ptr::null_mut();
    break;
    }
    if (!dev) /* NF_BR_LOCAL_OUT */
    return info.target;
    if (ether_addr_equal(info.mac, dev.dev_addr))
    skb.pkt_type = PACKET_HOST;
    else
    skb.pkt_type = PACKET_OTHERHOST;
    }
    return info.target;
    }
#[no_mangle]
unsafe extern "C" fn ebt_dnat_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int ebt_dnat_tg_check(const struct xt_tgchk_param *par)
    {
    const struct ebt_nat_info *info = par.targinfo;
    unsigned int hook_mask;
    if (BASE_CHAIN && info.target == EBT_RETURN)
    return -EINVAL;
    hook_mask = par.hook_mask & ~(1 << NF_BR_NUMHOOKS);
    if ((strcmp(par.table, "nat") != 0 ||
    (hook_mask & ~((1 << NF_BR_PRE_ROUTING) |
    (1 << NF_BR_LOCAL_OUT)))) &&
    (strcmp(par.table, "broute") != 0 ||
    hook_mask & ~(1 << NF_BR_BROUTING)))
    return -EINVAL;
    if (ebt_invalid_target(info.target))
    return -EINVAL;
    return 0;
    }
    static struct xt_target ebt_dnat_tg_reg __read_mostly = {
    .name		= "dnat",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .hooks		= (1 << NF_BR_NUMHOOKS) | (1 << NF_BR_PRE_ROUTING) |
    (1 << NF_BR_LOCAL_OUT) | (1 << NF_BR_BROUTING),
    .target		= ebt_dnat_tg,
    .checkentry	= ebt_dnat_tg_check,
    .targetsize	= sizeof(struct ebt_nat_info),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_dnat_init() -> int __init {
    static int __init ebt_dnat_init(void)
    {
    return xt_register_target(&ebt_dnat_tg_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_dnat_fini() -> void __exit {
    static void __exit ebt_dnat_fini(void)
    {
    xt_unregister_target(&ebt_dnat_tg_reg);
    }
    module_init(ebt_dnat_init);
    module_exit(ebt_dnat_fini);
    MODULE_DESCRIPTION("Ebtables: Destination MAC address translation");
    MODULE_LICENSE("GPL");
