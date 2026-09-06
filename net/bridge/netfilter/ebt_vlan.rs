//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_vlan.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Description: EBTables 802.1Q match extension kernelspace module.
// Authors: Nick Fedchik <nick@fedchik.org.ua>
// Bart De Schuymer <bdschuym@pandora.be>
//

    MODULE_AUTHOR("Nick Fedchik <nick@fedchik.org.ua>");
    MODULE_DESCRIPTION("Ebtables: 802.1Q VLAN tag match");
    MODULE_LICENSE("GPL");

    static bool
    ebt_vlan_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct ebt_vlan_info *info = par.matchinfo;
    unsigned short TCI;	/* Whole TCI, given from parsed frame */
    unsigned short id;	/* VLAN ID, given from frame TCI */
    unsigned char prio;	/* user_priority, given from frame TCI */
// VLAN encapsulated Type/Length field, given from orig frame
    __be16 encap;
    if (skb_vlan_tag_present(skb)) {
    TCI = skb_vlan_tag_get(skb);
    encap = skb.protocol;
    } else {
    const struct vlan_hdr *fp;
    struct vlan_hdr _frame;
    fp = skb_header_pointer(skb, 0, sizeof(_frame), &_frame);
    if (fp == core::ptr::null_mut())
    return false;
    TCI = ntohs(fp.h_vlan_TCI);
    encap = fp.h_vlan_encapsulated_proto;
    }
// Tag Control Information (TCI) consists of the following elements:
// - User_priority. The user_priority field is three bits in length,
// interpreted as a binary number.
// - Canonical Format Indicator (CFI). The Canonical Format Indicator
// (CFI) is a single bit flag value. Currently ignored.
// - VLAN Identifier (VID). The VID is encoded as
// an unsigned binary number.
//
    id = TCI & VLAN_VID_MASK;
    prio = (TCI >> 13) & 0x7;
// Checking VLAN Identifier (VID)
    if (GET_BITMASK(EBT_VLAN_ID))
    EXIT_ON_MISMATCH(id, EBT_VLAN_ID);
// Checking user_priority
    if (GET_BITMASK(EBT_VLAN_PRIO))
    EXIT_ON_MISMATCH(prio, EBT_VLAN_PRIO);
// Checking Encapsulated Proto (Length/Type) field
    if (GET_BITMASK(EBT_VLAN_ENCAP))
    EXIT_ON_MISMATCH(encap, EBT_VLAN_ENCAP);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ebt_vlan_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int ebt_vlan_mt_check(const struct xt_mtchk_param *par)
    {
    struct ebt_vlan_info *info = par.matchinfo;
    const struct ebt_entry *e = par.entryinfo;
// Is it 802.1Q frame checked?
    if (e.ethproto != htons(ETH_P_8021Q)) {
    pr_debug("passed entry proto %2.4X is not 802.1Q (8100)\n",
    ntohs(e.ethproto));
    return -EINVAL;
    }
// Check for bitmask range
// True if even one bit is out of mask
//
    if (info.bitmask & ~EBT_VLAN_MASK) {
    pr_debug("bitmask %2X is out of mask (%2X)\n",
    info.bitmask, EBT_VLAN_MASK);
    return -EINVAL;
    }
// Check for inversion flags range
    if (info.invflags & ~EBT_VLAN_MASK) {
    pr_debug("inversion flags %2X is out of mask (%2X)\n",
    info.invflags, EBT_VLAN_MASK);
    return -EINVAL;
    }
// Reserved VLAN ID (VID) values
// -----------------------------
// 0 - The null VLAN ID.
// 1 - The default Port VID (PVID)
// 0x0FFF - Reserved for implementation use.
// if_vlan.h: VLAN_N_VID 4096.
//
    if (GET_BITMASK(EBT_VLAN_ID)) {
    if (!!info.id) { /* if id!=0 => check vid range */
    if (info.id > VLAN_N_VID) {
    pr_debug("id %d is out of range (1-4096)\n",
    info.id);
    return -EINVAL;
    }
// Note: This is valid VLAN-tagged frame point.
// Any value of user_priority are acceptable,
// but should be ignored according to 802.1Q Std.
// So we just drop the prio flag.
//
    info.bitmask &= ~EBT_VLAN_PRIO;
    }
// Else, id=0 (null VLAN ID)  => user_priority range (any?)
    }
    if (GET_BITMASK(EBT_VLAN_PRIO)) {
    if ((unsigned char) info.prio > 7) {
    pr_debug("prio %d is out of range (0-7)\n",
    info.prio);
    return -EINVAL;
    }
    }
// Check for encapsulated proto range - it is possible to be
// any value for u_short range.
// if_ether.h:  ETH_ZLEN        60   -  Min. octets in frame sans FCS
//
    if (GET_BITMASK(EBT_VLAN_ENCAP)) {
    if ((unsigned short) ntohs(info.encap) < ETH_ZLEN) {
    pr_debug("encap frame length %d is less than "
    "minimal\n", ntohs(info.encap));
    return -EINVAL;
    }
    }
    return 0;
    }
    static struct xt_match ebt_vlan_mt_reg __read_mostly = {
    .name		= "vlan",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .match		= ebt_vlan_mt,
    .checkentry	= ebt_vlan_mt_check,
    .matchsize	= sizeof(struct ebt_vlan_info),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_vlan_init() -> int __init {
    static int __init ebt_vlan_init(void)
    {
    pr_debug("ebtables 802.1Q extension module v" MODULE_VERS "\n");
    return xt_register_match(&ebt_vlan_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_vlan_fini() -> void __exit {
    static void __exit ebt_vlan_fini(void)
    {
    xt_unregister_match(&ebt_vlan_mt_reg);
    }
    module_init(ebt_vlan_init);
    module_exit(ebt_vlan_fini);
