//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_stp.c
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
// ebt_stp
//
// Authors:
// Bart De Schuymer <bdschuym@pandora.be>
// Stephen Hemminger <shemminger@osdl.org>
//
// July, 2003
//

pub const BPDU_TYPE_CONFIG: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stp_header {
    pub dsap: u8,
    pub ssap: u8,
    pub ctrl: u8,
    pub pid: u8,
    pub vers: u8,
    pub type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stp_config_pdu {
    pub flags: u8,
    pub root: [u8; 8],
    pub root_cost: [u8; 4],
    pub sender: [u8; 8],
    pub port: [u8; 2],
    pub msg_age: [u8; 2],
    pub max_age: [u8; 2],
    pub hello_time: [u8; 2],
    pub forward_delay: [u8; 2],
}

    static bool ebt_filter_config(const struct ebt_stp_info *info,
    const struct stp_config_pdu *stpc)
    {
    const struct ebt_stp_config_info *c;
    u16 v16;
    u32 v32;
    c = &info.config;
    if ((info.bitmask & EBT_STP_FLAGS) &&
    NF_INVF(info, EBT_STP_FLAGS, c.flags != stpc.flags))
    return false;
    if (info.bitmask & EBT_STP_ROOTPRIO) {
    v16 = NR16(stpc.root);
    if (NF_INVF(info, EBT_STP_ROOTPRIO,
    v16 < c.root_priol || v16 > c.root_priou))
    return false;
    }
    if (info.bitmask & EBT_STP_ROOTADDR) {
    if (NF_INVF(info, EBT_STP_ROOTADDR,
    !ether_addr_equal_masked(&stpc.root[2],
    c.root_addr,
    c.root_addrmsk)))
    return false;
    }
    if (info.bitmask & EBT_STP_ROOTCOST) {
    v32 = NR32(stpc.root_cost);
    if (NF_INVF(info, EBT_STP_ROOTCOST,
    v32 < c.root_costl || v32 > c.root_costu))
    return false;
    }
    if (info.bitmask & EBT_STP_SENDERPRIO) {
    v16 = NR16(stpc.sender);
    if (NF_INVF(info, EBT_STP_SENDERPRIO,
    v16 < c.sender_priol || v16 > c.sender_priou))
    return false;
    }
    if (info.bitmask & EBT_STP_SENDERADDR) {
    if (NF_INVF(info, EBT_STP_SENDERADDR,
    !ether_addr_equal_masked(&stpc.sender[2],
    c.sender_addr,
    c.sender_addrmsk)))
    return false;
    }
    if (info.bitmask & EBT_STP_PORT) {
    v16 = NR16(stpc.port);
    if (NF_INVF(info, EBT_STP_PORT,
    v16 < c.portl || v16 > c.portu))
    return false;
    }
    if (info.bitmask & EBT_STP_MSGAGE) {
    v16 = NR16(stpc.msg_age);
    if (NF_INVF(info, EBT_STP_MSGAGE,
    v16 < c.msg_agel || v16 > c.msg_ageu))
    return false;
    }
    if (info.bitmask & EBT_STP_MAXAGE) {
    v16 = NR16(stpc.max_age);
    if (NF_INVF(info, EBT_STP_MAXAGE,
    v16 < c.max_agel || v16 > c.max_ageu))
    return false;
    }
    if (info.bitmask & EBT_STP_HELLOTIME) {
    v16 = NR16(stpc.hello_time);
    if (NF_INVF(info, EBT_STP_HELLOTIME,
    v16 < c.hello_timel || v16 > c.hello_timeu))
    return false;
    }
    if (info.bitmask & EBT_STP_FWDD) {
    v16 = NR16(stpc.forward_delay);
    if (NF_INVF(info, EBT_STP_FWDD,
    v16 < c.forward_delayl || v16 > c.forward_delayu))
    return false;
    }
    return true;
    }
    static bool
    ebt_stp_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct ebt_stp_info *info = par.matchinfo;
    const struct stp_header *sp;
    struct stp_header _stph;
    const u8 header[6] = {0x42, 0x42, 0x03, 0x00, 0x00, 0x00};
    sp = skb_header_pointer(skb, 0, sizeof(_stph), &_stph);
    if (sp == core::ptr::null_mut())
    return false;
// The stp code only considers these
    if (memcmp(sp, header, sizeof(header)))
    return false;
    if ((info.bitmask & EBT_STP_TYPE) &&
    NF_INVF(info, EBT_STP_TYPE, info.type != sp.type))
    return false;
    if (sp.type == BPDU_TYPE_CONFIG &&
    info.bitmask & EBT_STP_CONFIG_MASK) {
    const struct stp_config_pdu *st;
    struct stp_config_pdu _stpc;
    st = skb_header_pointer(skb, sizeof(_stph),
    sizeof(_stpc), &_stpc);
    if (st == core::ptr::null_mut())
    return false;
    return ebt_filter_config(info, st);
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ebt_stp_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int ebt_stp_mt_check(const struct xt_mtchk_param *par)
    {
    const struct ebt_stp_info *info = par.matchinfo;
    const struct ebt_entry *e = par.entryinfo;
    if (info.bitmask & ~EBT_STP_MASK || info.invflags & ~EBT_STP_MASK ||
    !(info.bitmask & EBT_STP_MASK))
    return -EINVAL;
// Make sure the match only receives stp frames
    if (!par.nft_compat &&
    (!ether_addr_equal(e.destmac, eth_stp_addr) ||
    !(e.bitmask & EBT_DESTMAC) ||
    !is_broadcast_ether_addr(e.destmsk)))
    return -EINVAL;
    return 0;
    }
    static struct xt_match ebt_stp_mt_reg __read_mostly = {
    .name		= "stp",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .match		= ebt_stp_mt,
    .checkentry	= ebt_stp_mt_check,
    .matchsize	= sizeof(struct ebt_stp_info),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_stp_init() -> int __init {
    static int __init ebt_stp_init(void)
    {
    return xt_register_match(&ebt_stp_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_stp_fini() -> void __exit {
    static void __exit ebt_stp_fini(void)
    {
    xt_unregister_match(&ebt_stp_mt_reg);
    }
    module_init(ebt_stp_init);
    module_exit(ebt_stp_fini);
    MODULE_DESCRIPTION("Ebtables: Spanning Tree Protocol packet match");
    MODULE_LICENSE("GPL");
