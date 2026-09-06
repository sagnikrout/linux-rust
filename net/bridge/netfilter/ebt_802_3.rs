//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_802_3.c
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
// 802_3
//
// Author:
// Chris Vitale csv@bluetail.com
//
// May 2003
//

    static struct ebt_802_3_hdr *ebt_802_3_hdr(const struct sk_buff *skb)
    {
    return (struct ebt_802_3_hdr *)skb_mac_header(skb);
    }
    static bool
    ebt_802_3_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct ebt_802_3_info *info = par.matchinfo;
    const struct ebt_802_3_hdr *hdr = ebt_802_3_hdr(skb);
    let mut type: __be16 = hdr.llc.ui.ctrl & IS_UI ? hdr.llc.ui.type : hdr.llc.ni.type;
    if (info.bitmask & EBT_802_3_SAP) {
    if (NF_INVF(info, EBT_802_3_SAP, info.sap != hdr.llc.ui.ssap))
    return false;
    if (NF_INVF(info, EBT_802_3_SAP, info.sap != hdr.llc.ui.dsap))
    return false;
    }
    if (info.bitmask & EBT_802_3_TYPE) {
    if (!(hdr.llc.ui.dsap == CHECK_TYPE && hdr.llc.ui.ssap == CHECK_TYPE))
    return false;
    if (NF_INVF(info, EBT_802_3_TYPE, info.type != type))
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ebt_802_3_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int ebt_802_3_mt_check(const struct xt_mtchk_param *par)
    {
    const struct ebt_802_3_info *info = par.matchinfo;
    if (info.bitmask & ~EBT_802_3_MASK || info.invflags & ~EBT_802_3_MASK)
    return -EINVAL;
    return 0;
    }
    static struct xt_match ebt_802_3_mt_reg __read_mostly = {
    .name		= "802_3",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .match		= ebt_802_3_mt,
    .checkentry	= ebt_802_3_mt_check,
    .matchsize	= sizeof(struct ebt_802_3_info),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_802_3_init() -> int __init {
    static int __init ebt_802_3_init(void)
    {
    return xt_register_match(&ebt_802_3_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_802_3_fini() -> void __exit {
    static void __exit ebt_802_3_fini(void)
    {
    xt_unregister_match(&ebt_802_3_mt_reg);
    }
    module_init(ebt_802_3_init);
    module_exit(ebt_802_3_fini);
    MODULE_DESCRIPTION("Ebtables: DSAP/SSAP field and SNAP type matching");
    MODULE_LICENSE("GPL");
