//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_log.c
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
// ebt_log
//
// Authors:
// Bart De Schuymer <bdschuym@pandora.be>
// Harald Welte <laforge@netfilter.org>
//
// April, 2002
//

    static DEFINE_SPINLOCK(ebt_log_lock);
#[no_mangle]
unsafe extern "C" fn ebt_log_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int ebt_log_tg_check(const struct xt_tgchk_param *par)
    {
    struct ebt_log_info *info = par.targinfo;
    if (info.bitmask & ~EBT_LOG_MASK)
    return -EINVAL;
    if (info.loglevel >= 8)
    return -EINVAL;
    info.prefix[EBT_LOG_PREFIX_SIZE - 1] = '\0';
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpudphdr {
    pub src: __be16,
    pub dst: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arppayload {
    pub mac_src: [c_uchar; ETH_ALEN],
    pub ip_src: [c_uchar; 4],
    pub mac_dst: [c_uchar; ETH_ALEN],
    pub ip_dst: [c_uchar; 4],
}

    static void
    print_ports(const struct sk_buff *skb, uint8_t protocol, int offset)
    {
    if (protocol == IPPROTO_TCP ||
    protocol == IPPROTO_UDP ||
    protocol == IPPROTO_UDPLITE ||
    protocol == IPPROTO_SCTP ||
    protocol == IPPROTO_DCCP) {
    const struct tcpudphdr *pptr;
    struct tcpudphdr _ports;
    pptr = skb_header_pointer(skb, offset,
    sizeof(_ports), &_ports);
    if (pptr == core::ptr::null_mut()) {
    pr_cont(" INCOMPLETE TCP/UDP header");
    return;
    }
    pr_cont(" SPT=%u DPT=%u", ntohs(pptr.src), ntohs(pptr.dst));
    }
    }
    static void
    ebt_log_packet(struct net *net, u_int8_t pf, unsigned int hooknum,
    const struct sk_buff *skb, const struct net_device *in,
    const struct net_device *out, const struct nf_loginfo *loginfo,
    const char *prefix)
    {
    unsigned int bitmask;
// FIXME: Disabled from containers until syslog ns is supported
    if (!net_eq(net, &init_net) && !sysctl_nf_log_all_netns)
    return;
    spin_lock_bh(&ebt_log_lock);
    printk(KERN_SOH "%c%s IN=%s OUT=%s MAC source = %pM MAC dest = %pM proto = 0x%04x",
    '0' + loginfo.u.log.level, prefix,
    in ? in.name : "", out ? out.name : "",
    eth_hdr(skb).h_source, eth_hdr(skb).h_dest,
    ntohs(eth_hdr(skb).h_proto));
    if (loginfo.type == NF_LOG_TYPE_LOG)
    bitmask = loginfo.u.log.logflags;
    else
    bitmask = NF_LOG_DEFAULT_MASK;
    if ((bitmask & EBT_LOG_IP) && eth_hdr(skb).h_proto ==
    htons(ETH_P_IP)) {
    const struct iphdr *ih;
    struct iphdr _iph;
    ih = skb_header_pointer(skb, 0, sizeof(_iph), &_iph);
    if (ih == core::ptr::null_mut()) {
    pr_cont(" INCOMPLETE IP header");
    goto out;
    }
    pr_cont(" IP SRC=%pI4 IP DST=%pI4, IP tos=0x%02X, IP proto=%d",
    &ih.saddr, &ih.daddr, ih.tos, ih.protocol);
    print_ports(skb, ih.protocol, ih.ihl*4);
    goto out;
    }

    if ((bitmask & EBT_LOG_IP6) && eth_hdr(skb).h_proto ==
    htons(ETH_P_IPV6)) {
    const struct ipv6hdr *ih;
    struct ipv6hdr _iph;
    uint8_t nexthdr;
    __be16 frag_off;
    int offset_ph;
    ih = skb_header_pointer(skb, 0, sizeof(_iph), &_iph);
    if (ih == core::ptr::null_mut()) {
    pr_cont(" INCOMPLETE IPv6 header");
    goto out;
    }
    pr_cont(" IPv6 SRC=%pI6 IPv6 DST=%pI6, IPv6 priority=0x%01X, Next Header=%d",
    &ih.saddr, &ih.daddr, ih.priority, ih.nexthdr);
    nexthdr = ih.nexthdr;
    offset_ph = ipv6_skip_exthdr(skb, sizeof(_iph), &nexthdr, &frag_off);
    if (offset_ph == -1)
    goto out;
    print_ports(skb, nexthdr, offset_ph);
    goto out;
    }

    if ((bitmask & EBT_LOG_ARP) &&
    ((eth_hdr(skb).h_proto == htons(ETH_P_ARP)) ||
    (eth_hdr(skb).h_proto == htons(ETH_P_RARP)))) {
    const struct arphdr *ah;
    struct arphdr _arph;
    ah = skb_header_pointer(skb, 0, sizeof(_arph), &_arph);
    if (ah == core::ptr::null_mut()) {
    pr_cont(" INCOMPLETE ARP header");
    goto out;
    }
    pr_cont(" ARP HTYPE=%d, PTYPE=0x%04x, OPCODE=%d",
    ntohs(ah.ar_hrd), ntohs(ah.ar_pro),
    ntohs(ah.ar_op));
// If it's for Ethernet and the lengths are OK,
// then log the ARP payload
//
    if (ah.ar_hrd == htons(1) &&
    ah.ar_hln == ETH_ALEN &&
    ah.ar_pln == sizeof(__be32)) {
    const struct arppayload *ap;
    struct arppayload _arpp;
    ap = skb_header_pointer(skb, sizeof(_arph),
    sizeof(_arpp), &_arpp);
    if (ap == core::ptr::null_mut()) {
    pr_cont(" INCOMPLETE ARP payload");
    goto out;
    }
    pr_cont(" ARP MAC SRC=%pM ARP IP SRC=%pI4 ARP MAC DST=%pM ARP IP DST=%pI4",
    ap.mac_src, ap.ip_src,
    ap.mac_dst, ap.ip_dst);
    }
    }
    out:
    pr_cont("\n");
    spin_unlock_bh(&ebt_log_lock);
    }
    static unsigned int
    ebt_log_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct ebt_log_info *info = par.targinfo;
    struct nf_loginfo li;
    struct net *net = xt_net(par);
    li.type = NF_LOG_TYPE_LOG;
    li.u.log.level = info.loglevel;
    li.u.log.logflags = info.bitmask;
// Remember that we have to use ebt_log_packet() not to break backward
// compatibility. We cannot use the default bridge packet logger via
// nf_log_packet() with NFT_LOG_TYPE_LOG here. --Pablo
//
    if (info.bitmask & EBT_LOG_NFLOG)
    nf_log_packet(net, NFPROTO_BRIDGE, xt_hooknum(par), skb,
    xt_in(par), xt_out(par), &li, "%s",
    info.prefix);
    else
    ebt_log_packet(net, NFPROTO_BRIDGE, xt_hooknum(par), skb,
    xt_in(par), xt_out(par), &li, info.prefix);
    return EBT_CONTINUE;
    }
    static struct xt_target ebt_log_tg_reg __read_mostly = {
    .name		= "log",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .target		= ebt_log_tg,
    .checkentry	= ebt_log_tg_check,
    .targetsize	= sizeof(struct ebt_log_info),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_log_init() -> int __init {
    static int __init ebt_log_init(void)
    {
    return xt_register_target(&ebt_log_tg_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_log_fini() -> void __exit {
    static void __exit ebt_log_fini(void)
    {
    xt_unregister_target(&ebt_log_tg_reg);
    }
    module_init(ebt_log_init);
    module_exit(ebt_log_fini);
    MODULE_DESCRIPTION("Ebtables: Packet logging to syslog");
    MODULE_LICENSE("GPL");
