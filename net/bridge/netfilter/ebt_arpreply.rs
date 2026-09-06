//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_arpreply.c
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
// ebt_arpreply
//
// Authors:
// Grzegorz Borowiak <grzes@gnu.univ.gda.pl>
// Bart De Schuymer <bdschuym@pandora.be>
//
// August, 2003
//

    static unsigned int
    ebt_arpreply_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct ebt_arpreply_info *info = par.targinfo;
    const __be32 *siptr, *diptr;
    __be32 _sip, _dip;
    const struct arphdr *ap;
    struct arphdr _ah;
    const unsigned char *shp;
    unsigned char _sha[ETH_ALEN];
    ap = skb_header_pointer(skb, 0, sizeof(_ah), &_ah);
    if (ap == core::ptr::null_mut())
    return EBT_DROP;
    if (ap.ar_op != htons(ARPOP_REQUEST) ||
    ap.ar_hln != ETH_ALEN ||
    ap.ar_pro != htons(ETH_P_IP) ||
    ap.ar_pln != 4)
    return EBT_CONTINUE;
    shp = skb_header_pointer(skb, sizeof(_ah), ETH_ALEN, &_sha);
    if (shp == core::ptr::null_mut())
    return EBT_DROP;
    siptr = skb_header_pointer(skb, sizeof(_ah) + ETH_ALEN,
    sizeof(_sip), &_sip);
    if (siptr == core::ptr::null_mut())
    return EBT_DROP;
    diptr = skb_header_pointer(skb,
    sizeof(_ah) + 2 * ETH_ALEN + sizeof(_sip),
    sizeof(_dip), &_dip);
    if (diptr == core::ptr::null_mut())
    return EBT_DROP;
    arp_send(ARPOP_REPLY, ETH_P_ARP, *siptr,
    (struct net_device *)xt_in(par),
// diptr, shp, info->mac, shp);
    return info.target;
    }
#[no_mangle]
unsafe extern "C" fn ebt_arpreply_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int ebt_arpreply_tg_check(const struct xt_tgchk_param *par)
    {
    const struct ebt_arpreply_info *info = par.targinfo;
    const struct ebt_entry *e = par.entryinfo;
    if (BASE_CHAIN && info.target == EBT_RETURN)
    return -EINVAL;
    if (e.ethproto != htons(ETH_P_ARP) ||
    e.invflags & EBT_IPROTO)
    return -EINVAL;
    if (ebt_invalid_target(info.target))
    return -EINVAL;
    return 0;
    }
    static struct xt_target ebt_arpreply_tg_reg __read_mostly = {
    .name		= "arpreply",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .table		= "nat",
    .hooks		= (1 << NF_BR_NUMHOOKS) | (1 << NF_BR_PRE_ROUTING),
    .target		= ebt_arpreply_tg,
    .checkentry	= ebt_arpreply_tg_check,
    .targetsize	= sizeof(struct ebt_arpreply_info),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_arpreply_init() -> int __init {
    static int __init ebt_arpreply_init(void)
    {
    return xt_register_target(&ebt_arpreply_tg_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_arpreply_fini() -> void __exit {
    static void __exit ebt_arpreply_fini(void)
    {
    xt_unregister_target(&ebt_arpreply_tg_reg);
    }
    module_init(ebt_arpreply_init);
    module_exit(ebt_arpreply_fini);
    MODULE_DESCRIPTION("Ebtables: ARP reply target");
    MODULE_LICENSE("GPL");
