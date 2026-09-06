//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_ip.c
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
// ebt_ip
//
// Authors:
// Bart De Schuymer <bdschuym@pandora.be>
//
// April, 2002
//
// Changes:
// added ip-sport and ip-dport
// Innominate Security Technologies AG <mhopf@innominate.com>
// September, 2002
//

    union pkthdr {
    struct {
    __be16 src;
    __be16 dst;
    } tcpudphdr;
    struct {
    u8 type;
    u8 code;
    } icmphdr;
    struct {
    u8 type;
    } igmphdr;
    };
    static bool
    ebt_ip_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct ebt_ip_info *info = par.matchinfo;
    const struct iphdr *ih;
    struct iphdr _iph;
    const union pkthdr *pptr;
    union pkthdr _pkthdr;
    ih = skb_header_pointer(skb, 0, sizeof(_iph), &_iph);
    if (ih == core::ptr::null_mut())
    return false;
    if ((info.bitmask & EBT_IP_TOS) &&
    NF_INVF(info, EBT_IP_TOS, info.tos != ih.tos))
    return false;
    if ((info.bitmask & EBT_IP_SOURCE) &&
    NF_INVF(info, EBT_IP_SOURCE,
    (ih.saddr & info.smsk) != info.saddr))
    return false;
    if ((info.bitmask & EBT_IP_DEST) &&
    NF_INVF(info, EBT_IP_DEST,
    (ih.daddr & info.dmsk) != info.daddr))
    return false;
    if (info.bitmask & EBT_IP_PROTO) {
    if (NF_INVF(info, EBT_IP_PROTO, info.protocol != ih.protocol))
    return false;
    if (!(info.bitmask & (EBT_IP_DPORT | EBT_IP_SPORT |
    EBT_IP_ICMP | EBT_IP_IGMP)))
    return true;
    if (ntohs(ih.frag_off) & IP_OFFSET)
    return false;
// min icmp/igmp headersize is 4, so sizeof(_pkthdr) is ok.
    pptr = skb_header_pointer(skb, ih.ihl*4,
    sizeof(_pkthdr), &_pkthdr);
    if (pptr == core::ptr::null_mut())
    return false;
    if (info.bitmask & EBT_IP_DPORT) {
    let mut dst: u32 = ntohs(pptr.tcpudphdr.dst);
    if (NF_INVF(info, EBT_IP_DPORT,
    dst < info.dport[0] ||
    dst > info.dport[1]))
    return false;
    }
    if (info.bitmask & EBT_IP_SPORT) {
    let mut src: u32 = ntohs(pptr.tcpudphdr.src);
    if (NF_INVF(info, EBT_IP_SPORT,
    src < info.sport[0] ||
    src > info.sport[1]))
    return false;
    }
    if ((info.bitmask & EBT_IP_ICMP) &&
    NF_INVF(info, EBT_IP_ICMP,
    pptr.icmphdr.type < info.icmp_type[0] ||
    pptr.icmphdr.type > info.icmp_type[1] ||
    pptr.icmphdr.code < info.icmp_code[0] ||
    pptr.icmphdr.code > info.icmp_code[1]))
    return false;
    if ((info.bitmask & EBT_IP_IGMP) &&
    NF_INVF(info, EBT_IP_IGMP,
    pptr.igmphdr.type < info.igmp_type[0] ||
    pptr.igmphdr.type > info.igmp_type[1]))
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ebt_ip_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int ebt_ip_mt_check(const struct xt_mtchk_param *par)
    {
    const struct ebt_ip_info *info = par.matchinfo;
    const struct ebt_entry *e = par.entryinfo;
    if (e.ethproto != htons(ETH_P_IP) ||
    e.invflags & EBT_IPROTO)
    return -EINVAL;
    if (info.bitmask & ~EBT_IP_MASK || info.invflags & ~EBT_IP_MASK)
    return -EINVAL;
    if (info.bitmask & (EBT_IP_DPORT | EBT_IP_SPORT)) {
    if (info.invflags & EBT_IP_PROTO)
    return -EINVAL;
    if (info.protocol != IPPROTO_TCP &&
    info.protocol != IPPROTO_UDP &&
    info.protocol != IPPROTO_UDPLITE &&
    info.protocol != IPPROTO_SCTP &&
    info.protocol != IPPROTO_DCCP)
    return -EINVAL;
    }
    if (info.bitmask & EBT_IP_DPORT && info.dport[0] > info.dport[1])
    return -EINVAL;
    if (info.bitmask & EBT_IP_SPORT && info.sport[0] > info.sport[1])
    return -EINVAL;
    if (info.bitmask & EBT_IP_ICMP) {
    if ((info.invflags & EBT_IP_PROTO) ||
    info.protocol != IPPROTO_ICMP)
    return -EINVAL;
    if (info.icmp_type[0] > info.icmp_type[1] ||
    info.icmp_code[0] > info.icmp_code[1])
    return -EINVAL;
    }
    if (info.bitmask & EBT_IP_IGMP) {
    if ((info.invflags & EBT_IP_PROTO) ||
    info.protocol != IPPROTO_IGMP)
    return -EINVAL;
    if (info.igmp_type[0] > info.igmp_type[1])
    return -EINVAL;
    }
    return 0;
    }
    static struct xt_match ebt_ip_mt_reg __read_mostly = {
    .name		= "ip",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .match		= ebt_ip_mt,
    .checkentry	= ebt_ip_mt_check,
    .matchsize	= sizeof(struct ebt_ip_info),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_ip_init() -> int __init {
    static int __init ebt_ip_init(void)
    {
    return xt_register_match(&ebt_ip_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_ip_fini() -> void __exit {
    static void __exit ebt_ip_fini(void)
    {
    xt_unregister_match(&ebt_ip_mt_reg);
    }
    module_init(ebt_ip_init);
    module_exit(ebt_ip_fini);
    MODULE_DESCRIPTION("Ebtables: IPv4 protocol packet match");
    MODULE_LICENSE("GPL");
