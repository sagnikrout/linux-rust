//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_redirect.c
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
// ebt_redirect
//
// Authors:
// Bart De Schuymer <bdschuym@pandora.be>
//
// April, 2002
//

    static unsigned int
    ebt_redirect_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct ebt_redirect_info *info = par.targinfo;
    if (skb_ensure_writable(skb, 0))
    return EBT_DROP;
    if (xt_hooknum(par) != NF_BR_BROUTING) {
    const struct net_device *dev;
    dev = netdev_master_upper_dev_get_rcu(xt_in(par));
    if (!dev)
    return EBT_DROP;
    ether_addr_copy(eth_hdr(skb).h_dest, dev.dev_addr);
    } else {
    ether_addr_copy(eth_hdr(skb).h_dest, xt_in(par).dev_addr);
    }
    skb.pkt_type = PACKET_HOST;
    return info.target;
    }
#[no_mangle]
unsafe extern "C" fn ebt_redirect_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int ebt_redirect_tg_check(const struct xt_tgchk_param *par)
    {
    const struct ebt_redirect_info *info = par.targinfo;
    unsigned int hook_mask;
    if (BASE_CHAIN && info.target == EBT_RETURN)
    return -EINVAL;
    hook_mask = par.hook_mask & ~(1 << NF_BR_NUMHOOKS);
    if ((strcmp(par.table, "nat") != 0 ||
    hook_mask & ~(1 << NF_BR_PRE_ROUTING)) &&
    (strcmp(par.table, "broute") != 0 ||
    hook_mask & ~(1 << NF_BR_BROUTING)))
    return -EINVAL;
    if (ebt_invalid_target(info.target))
    return -EINVAL;
    return 0;
    }
    static struct xt_target ebt_redirect_tg_reg __read_mostly = {
    .name		= "redirect",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .hooks		= (1 << NF_BR_NUMHOOKS) | (1 << NF_BR_PRE_ROUTING) |
    (1 << NF_BR_BROUTING),
    .target		= ebt_redirect_tg,
    .checkentry	= ebt_redirect_tg_check,
    .targetsize	= sizeof(struct ebt_redirect_info),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_redirect_init() -> int __init {
    static int __init ebt_redirect_init(void)
    {
    return xt_register_target(&ebt_redirect_tg_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_redirect_fini() -> void __exit {
    static void __exit ebt_redirect_fini(void)
    {
    xt_unregister_target(&ebt_redirect_tg_reg);
    }
    module_init(ebt_redirect_init);
    module_exit(ebt_redirect_fini);
    MODULE_DESCRIPTION("Ebtables: Packet redirection to localhost");
    MODULE_LICENSE("GPL");
