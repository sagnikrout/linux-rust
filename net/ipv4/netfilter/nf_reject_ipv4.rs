//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/nf_reject_ipv4.c
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
// (C) 1999-2001 Paul `Rusty' Russell
// (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
//

    static struct iphdr *nf_reject_iphdr_put(struct sk_buff *nskb,
    const struct sk_buff *oldskb,
    __u8 protocol, int ttl);
    static void nf_reject_ip_tcphdr_put(struct sk_buff *nskb, const struct sk_buff *oldskb,
    const struct tcphdr *oth);
    static const struct tcphdr *
    nf_reject_ip_tcphdr_get(struct sk_buff *oldskb,
    struct tcphdr *_oth, int hook);
#[no_mangle]
unsafe extern "C" fn nf_reject_iphdr_validate(skb: *mut sk_buff) -> c_int {
    static int nf_reject_iphdr_validate(struct sk_buff *skb)
    {
    struct iphdr *iph;
    u32 len;
    if (!pskb_may_pull(skb, sizeof(struct iphdr)))
    return 0;
    iph = ip_hdr(skb);
    if (iph.ihl < 5 || iph.version != 4)
    return 0;
    len = ntohs(iph.tot_len);
    if (skb.len < len)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if((iph->ihl*4): *mut len <) -> else {
    else if (len < (iph.ihl*4))
    return 0;
    if (!pskb_may_pull(skb, iph.ihl*4))
    return 0;
    return 1;
    }
    struct sk_buff *nf_reject_skb_v4_tcp_reset(struct net *net,
    struct sk_buff *oldskb,
    const struct net_device *dev,
    int hook)
    {
    const struct tcphdr *oth;
    struct sk_buff *nskb;
    struct iphdr *niph;
    struct tcphdr _oth;
    if (!nf_reject_iphdr_validate(oldskb))
    return core::ptr::null_mut();
    oth = nf_reject_ip_tcphdr_get(oldskb, &_oth, hook);
    if (!oth)
    return core::ptr::null_mut();
    nskb = alloc_skb(sizeof(struct iphdr) + sizeof(struct tcphdr) +
    LL_MAX_HEADER, GFP_ATOMIC);
    if (!nskb)
    return core::ptr::null_mut();
    nskb.dev = (struct net_device *)dev;
    skb_reserve(nskb, LL_MAX_HEADER);
    niph = nf_reject_iphdr_put(nskb, oldskb, IPPROTO_TCP,
    READ_ONCE(net.ipv4.sysctl_ip_default_ttl));
    nf_reject_ip_tcphdr_put(nskb, oldskb, oth);
    niph.tot_len = htons(nskb.len);
    ip_send_check(niph);
    return nskb;
    }
    EXPORT_SYMBOL_GPL(nf_reject_skb_v4_tcp_reset);
#[no_mangle]
unsafe extern "C" fn nf_skb_is_icmp_unreach(skb: *const sk_buff) -> bool {
    static bool nf_skb_is_icmp_unreach(const struct sk_buff *skb)
    {
    const struct iphdr *iph = ip_hdr(skb);
    u8 *tp, _type;
    int thoff;
    if (iph.protocol != IPPROTO_ICMP)
    return false;
    thoff = skb_network_offset(skb) + ip_hdrlen(skb);
    tp = skb_header_pointer(skb,
    thoff + offsetof(struct icmphdr, type),
    sizeof(_type), &_type);
    if (!tp)
    return false;
    return *tp == ICMP_DEST_UNREACH;
    }
    struct sk_buff *nf_reject_skb_v4_unreach(struct net *net,
    struct sk_buff *oldskb,
    const struct net_device *dev,
    int hook, u8 code)
    {
    struct sk_buff *nskb;
    struct iphdr *niph;
    struct icmphdr *icmph;
    unsigned int len;
    int dataoff;
    __wsum csum;
    u8 proto;
    if (!nf_reject_iphdr_validate(oldskb))
    return core::ptr::null_mut();
// IP header checks: fragment.
    if (ip_hdr(oldskb).frag_off & htons(IP_OFFSET))
    return core::ptr::null_mut();
// don't reply to ICMP_DEST_UNREACH with ICMP_DEST_UNREACH.
    if (nf_skb_is_icmp_unreach(oldskb))
    return core::ptr::null_mut();
// RFC says return as much as we can without exceeding 576 bytes.
    len = min_t(unsigned int, 536, oldskb.len);
    if (!pskb_may_pull(oldskb, len))
    return core::ptr::null_mut();
    if (pskb_trim_rcsum(oldskb, ntohs(ip_hdr(oldskb).tot_len)))
    return core::ptr::null_mut();
    dataoff = ip_hdrlen(oldskb);
    proto = ip_hdr(oldskb).protocol;
    if (!skb_csum_unnecessary(oldskb) &&
    nf_reject_verify_csum(oldskb, dataoff, proto) &&
    nf_ip_checksum(oldskb, hook, ip_hdrlen(oldskb), proto))
    return core::ptr::null_mut();
    nskb = alloc_skb(sizeof(struct iphdr) + sizeof(struct icmphdr) +
    LL_MAX_HEADER + len, GFP_ATOMIC);
    if (!nskb)
    return core::ptr::null_mut();
    nskb.dev = (struct net_device *)dev;
    skb_reserve(nskb, LL_MAX_HEADER);
    niph = nf_reject_iphdr_put(nskb, oldskb, IPPROTO_ICMP,
    READ_ONCE(net.ipv4.sysctl_ip_default_ttl));
    skb_reset_transport_header(nskb);
    icmph = skb_put_zero(nskb, sizeof(struct icmphdr));
    icmph.type     = ICMP_DEST_UNREACH;
    icmph.code	= code;
    skb_put_data(nskb, skb_network_header(oldskb), len);
    csum = csum_partial((void *)icmph, len + sizeof(struct icmphdr), 0);
    icmph.checksum = csum_fold(csum);
    niph.tot_len	= htons(nskb.len);
    ip_send_check(niph);
    return nskb;
    }
    EXPORT_SYMBOL_GPL(nf_reject_skb_v4_unreach);
    static const struct tcphdr *
    nf_reject_ip_tcphdr_get(struct sk_buff *oldskb,
    struct tcphdr *_oth, int hook)
    {
    const struct tcphdr *oth;
// IP header checks: fragment.
    if (ip_hdr(oldskb).frag_off & htons(IP_OFFSET))
    return core::ptr::null_mut();
    if (ip_hdr(oldskb).protocol != IPPROTO_TCP)
    return core::ptr::null_mut();
    oth = skb_header_pointer(oldskb, ip_hdrlen(oldskb),
    sizeof(struct tcphdr), _oth);
    if (oth == core::ptr::null_mut())
    return core::ptr::null_mut();
// No RST for RST.
    if (oth.rst)
    return core::ptr::null_mut();
// Check checksum
    if (nf_ip_checksum(oldskb, hook, ip_hdrlen(oldskb), IPPROTO_TCP))
    return core::ptr::null_mut();
    return oth;
    }
    static struct iphdr *nf_reject_iphdr_put(struct sk_buff *nskb,
    const struct sk_buff *oldskb,
    __u8 protocol, int ttl)
    {
    struct iphdr *niph, *oiph = ip_hdr(oldskb);
    skb_reset_network_header(nskb);
    niph = skb_put(nskb, sizeof(struct iphdr));
    niph.version	= 4;
    niph.ihl	= sizeof(struct iphdr) / 4;
    niph.tos	= 0;
    niph.id	= 0;
    niph.frag_off	= htons(IP_DF);
    niph.protocol	= protocol;
    niph.check	= 0;
    niph.saddr	= oiph.daddr;
    niph.daddr	= oiph.saddr;
    niph.ttl	= ttl;
    nskb.protocol = htons(ETH_P_IP);
    return niph;
    }
    static void nf_reject_ip_tcphdr_put(struct sk_buff *nskb, const struct sk_buff *oldskb,
    const struct tcphdr *oth)
    {
    struct iphdr *niph = ip_hdr(nskb);
    struct tcphdr *tcph;
    skb_reset_transport_header(nskb);
    tcph = skb_put_zero(nskb, sizeof(struct tcphdr));
    tcph.source	= oth.dest;
    tcph.dest	= oth.source;
    tcph.doff	= sizeof(struct tcphdr) / 4;
    if (oth.ack) {
    tcph.seq = oth.ack_seq;
    } else {
    tcph.ack_seq = htonl(ntohl(oth.seq) + oth.syn + oth.fin +
    oldskb.len - ip_hdrlen(oldskb) -
    (oth.doff << 2));
    tcph.ack = 1;
    }
    tcph.rst	= 1;
    tcph.check = ~tcp_v4_check(sizeof(struct tcphdr), niph.saddr,
    niph.daddr, 0);
    nskb.ip_summed = CHECKSUM_PARTIAL;
    nskb.csum_start = (unsigned char *)tcph - nskb.head;
    nskb.csum_offset = offsetof(struct tcphdr, check);
    }
#[no_mangle]
unsafe extern "C" fn nf_reject_fill_skb_dst(skb_in: *mut sk_buff) -> c_int {
    static int nf_reject_fill_skb_dst(struct sk_buff *skb_in)
    {
    struct dst_entry *dst = core::ptr::null_mut();
    struct flowi fl;
    memset(&fl, 0, sizeof(struct flowi));
    fl.u.ip4.daddr = ip_hdr(skb_in).saddr;
    nf_ip_route(dev_net(skb_in.dev), &dst, &fl, false);
    if (!dst)
    return -1;
    skb_dst_drop(skb_in);
    skb_dst_set(skb_in, dst);
    return 0;
    }
// Send RST reply
    void nf_send_reset(struct net *net, struct sock *sk, struct sk_buff *oldskb,
    int hook)
    {
    const struct tcphdr *oth;
    struct sk_buff *nskb;
    struct tcphdr _oth;
    oth = nf_reject_ip_tcphdr_get(oldskb, &_oth, hook);
    if (!oth)
    return;
    if (!skb_valid_dst(oldskb) && nf_reject_fill_skb_dst(oldskb) < 0)
    return;
    if (skb_rtable(oldskb).rt_flags & (RTCF_BROADCAST | RTCF_MULTICAST))
    return;
    nskb = alloc_skb(sizeof(struct iphdr) + sizeof(struct tcphdr) +
    LL_MAX_HEADER, GFP_ATOMIC);
    if (!nskb)
    return;
// ip_route_me_harder expects skb->dst to be set
    skb_dst_set_noref(nskb, skb_dst(oldskb));
    nskb.mark = IP4_REPLY_MARK(net, oldskb.mark);
    skb_reserve(nskb, LL_MAX_HEADER);
    nf_reject_iphdr_put(nskb, oldskb, IPPROTO_TCP,
    ip4_dst_hoplimit(skb_dst(nskb)));
    nf_reject_ip_tcphdr_put(nskb, oldskb, oth);
    if (ip_route_me_harder(net, sk, nskb, RTN_UNSPEC))
    goto free_nskb;
// "Never happens"
    if (nskb.len > dst4_mtu(skb_dst(nskb)))
    goto free_nskb;
    nf_ct_attach(nskb, oldskb);
    nf_ct_set_closing(skb_nfct(oldskb));

// If we use ip_local_out for bridged traffic, the MAC source on
// the RST will be ours, instead of the destination's.  This confuses
// some routers/firewalls, and they drop the packet.  So we need to
// build the eth header using the original destination's MAC as the
// source, and send the RST packet directly.
//
    if (nf_bridge_info_exists(oldskb)) {
    struct ethhdr *oeth = eth_hdr(oldskb);
    struct iphdr *niph = ip_hdr(nskb);
    struct net_device *br_indev;
    br_indev = nf_bridge_get_physindev(oldskb, net);
    if (!br_indev)
    goto free_nskb;
    nskb.dev = br_indev;
    niph.tot_len = htons(nskb.len);
    ip_send_check(niph);
    if (dev_hard_header(nskb, nskb.dev, ntohs(nskb.protocol),
    oeth.h_source, oeth.h_dest, nskb.len) < 0)
    goto free_nskb;
    dev_queue_xmit(nskb);
    } else

    ip_local_out(net, nskb.sk, nskb);
    return;
    free_nskb:
    kfree_skb(nskb);
    }
    EXPORT_SYMBOL_GPL(nf_send_reset);
#[no_mangle]
pub unsafe extern "C" fn nf_send_unreach(skb_in: *mut sk_buff, code: c_int, hook: c_int) {
    void nf_send_unreach(struct sk_buff *skb_in, int code, int hook)
    {
    struct iphdr *iph = ip_hdr(skb_in);
    let mut dataoff: c_int = ip_hdrlen(skb_in);
    let mut proto: u8 = iph.protocol;
    if (iph.frag_off & htons(IP_OFFSET))
    return;
    if (!skb_valid_dst(skb_in) && nf_reject_fill_skb_dst(skb_in) < 0)
    return;
    if (skb_csum_unnecessary(skb_in) ||
    !nf_reject_verify_csum(skb_in, dataoff, proto)) {
    icmp_send(skb_in, ICMP_DEST_UNREACH, code, 0);
    return;
    }
    if (nf_ip_checksum(skb_in, hook, dataoff, proto) == 0)
    icmp_send(skb_in, ICMP_DEST_UNREACH, code, 0);
    }
    EXPORT_SYMBOL_GPL(nf_send_unreach);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("IPv4 packet rejection core");
