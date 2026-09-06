//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/nf_reject_ipv6.c
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

    static struct ipv6hdr *
    nf_reject_ip6hdr_put(struct sk_buff *nskb,
    const struct sk_buff *oldskb,
    __u8 protocol, int hoplimit);
    static void
    nf_reject_ip6_tcphdr_put(struct sk_buff *nskb,
    const struct sk_buff *oldskb,
    const struct tcphdr *oth, unsigned int otcplen);
    static const struct tcphdr *
    nf_reject_ip6_tcphdr_get(struct sk_buff *oldskb,
    struct tcphdr *otcph,
    unsigned int *otcplen, int hook);
#[no_mangle]
unsafe extern "C" fn nf_reject_v6_csum_ok(skb: *mut sk_buff, hook: c_int) -> bool {
    static bool nf_reject_v6_csum_ok(struct sk_buff *skb, int hook)
    {
    const struct ipv6hdr *ip6h = ipv6_hdr(skb);
    int thoff;
    __be16 fo;
    let mut proto: u8 = ip6h.nexthdr;
    if (skb_csum_unnecessary(skb))
    return true;
    if (ip6h.payload_len &&
    pskb_trim_rcsum(skb, ntohs(ip6h.payload_len) + sizeof(*ip6h)))
    return false;
    ip6h = ipv6_hdr(skb);
    thoff = ipv6_skip_exthdr(skb, ((u8*)(ip6h+1) - skb.data), &proto, &fo);
    if (thoff < 0 || thoff >= skb.len || (fo & htons(~0x7)) != 0)
    return false;
    if (!nf_reject_verify_csum(skb, thoff, proto))
    return true;
    return nf_ip6_checksum(skb, hook, thoff, proto) == 0;
    }
#[no_mangle]
unsafe extern "C" fn nf_reject_ip6hdr_validate(skb: *mut sk_buff) -> c_int {
    static int nf_reject_ip6hdr_validate(struct sk_buff *skb)
    {
    struct ipv6hdr *hdr;
    u32 pkt_len;
    if (!pskb_may_pull(skb, sizeof(struct ipv6hdr)))
    return 0;
    hdr = ipv6_hdr(skb);
    if (hdr.version != 6)
    return 0;
    pkt_len = ntohs(hdr.payload_len);
    if (pkt_len + sizeof(struct ipv6hdr) > skb.len)
    return 0;
    return 1;
    }
    struct sk_buff *nf_reject_skb_v6_tcp_reset(struct net *net,
    struct sk_buff *oldskb,
    const struct net_device *dev,
    int hook)
    {
    struct sk_buff *nskb;
    const struct tcphdr *oth;
    struct tcphdr _oth;
    unsigned int otcplen;
    struct ipv6hdr *nip6h;
    if (!nf_reject_ip6hdr_validate(oldskb))
    return core::ptr::null_mut();
    oth = nf_reject_ip6_tcphdr_get(oldskb, &_oth, &otcplen, hook);
    if (!oth)
    return core::ptr::null_mut();
    nskb = alloc_skb(sizeof(struct ipv6hdr) + sizeof(struct tcphdr) +
    LL_MAX_HEADER, GFP_ATOMIC);
    if (!nskb)
    return core::ptr::null_mut();
    nskb.dev = (struct net_device *)dev;
    skb_reserve(nskb, LL_MAX_HEADER);
    nip6h = nf_reject_ip6hdr_put(nskb, oldskb, IPPROTO_TCP,
    READ_ONCE(net.ipv6.devconf_all.hop_limit));
    nf_reject_ip6_tcphdr_put(nskb, oldskb, oth, otcplen);
    nip6h.payload_len = htons(nskb.len - sizeof(struct ipv6hdr));
    return nskb;
    }
    EXPORT_SYMBOL_GPL(nf_reject_skb_v6_tcp_reset);
#[no_mangle]
unsafe extern "C" fn nf_skb_is_icmp6_unreach(skb: *const sk_buff) -> bool {
    static bool nf_skb_is_icmp6_unreach(const struct sk_buff *skb)
    {
    const struct ipv6hdr *ip6h = ipv6_hdr(skb);
    let mut proto: u8 = ip6h.nexthdr;
    u8 _type, *tp;
    int thoff;
    __be16 fo;
    thoff = ipv6_skip_exthdr(skb, ((u8 *)(ip6h + 1) - skb.data), &proto, &fo);
    if (thoff < 0 || thoff >= skb.len || fo != 0)
    return false;
    if (proto != IPPROTO_ICMPV6)
    return false;
    tp = skb_header_pointer(skb,
    thoff + offsetof(struct icmp6hdr, icmp6_type),
    sizeof(_type), &_type);
    if (!tp)
    return false;
    return *tp == ICMPV6_DEST_UNREACH;
    }
    struct sk_buff *nf_reject_skb_v6_unreach(struct net *net,
    struct sk_buff *oldskb,
    const struct net_device *dev,
    int hook, u8 code)
    {
    struct sk_buff *nskb;
    struct ipv6hdr *nip6h;
    struct icmp6hdr *icmp6h;
    unsigned int len;
    if (!nf_reject_ip6hdr_validate(oldskb))
    return core::ptr::null_mut();
// Don't reply to ICMPV6_DEST_UNREACH with ICMPV6_DEST_UNREACH
    if (nf_skb_is_icmp6_unreach(oldskb))
    return core::ptr::null_mut();
// Include "As much of invoking packet as possible without the ICMPv6
// packet exceeding the minimum IPv6 MTU" in the ICMP payload.
//
    len = min_t(unsigned int, 1220, oldskb.len);
    if (!pskb_may_pull(oldskb, len))
    return core::ptr::null_mut();
    if (!nf_reject_v6_csum_ok(oldskb, hook))
    return core::ptr::null_mut();
    nskb = alloc_skb(sizeof(struct ipv6hdr) + sizeof(struct icmp6hdr) +
    LL_MAX_HEADER + len, GFP_ATOMIC);
    if (!nskb)
    return core::ptr::null_mut();
    nskb.dev = (struct net_device *)dev;
    skb_reserve(nskb, LL_MAX_HEADER);
    nip6h = nf_reject_ip6hdr_put(nskb, oldskb, IPPROTO_ICMPV6,
    READ_ONCE(net.ipv6.devconf_all.hop_limit));
    skb_reset_transport_header(nskb);
    icmp6h = skb_put_zero(nskb, sizeof(struct icmp6hdr));
    icmp6h.icmp6_type = ICMPV6_DEST_UNREACH;
    icmp6h.icmp6_code = code;
    skb_put_data(nskb, skb_network_header(oldskb), len);
    nip6h.payload_len = htons(nskb.len - sizeof(struct ipv6hdr));
    icmp6h.icmp6_cksum =
    csum_ipv6_magic(&nip6h.saddr, &nip6h.daddr,
    nskb.len - sizeof(struct ipv6hdr),
    IPPROTO_ICMPV6,
    csum_partial(icmp6h,
    nskb.len - sizeof(struct ipv6hdr),
    0));
    return nskb;
    }
    EXPORT_SYMBOL_GPL(nf_reject_skb_v6_unreach);
    static const struct tcphdr *
    nf_reject_ip6_tcphdr_get(struct sk_buff *oldskb,
    struct tcphdr *otcph,
    unsigned int *otcplen, int hook)
    {
    const struct ipv6hdr *oip6h = ipv6_hdr(oldskb);
    u8 proto;
    __be16 frag_off;
    int tcphoff;
    proto = oip6h.nexthdr;
    tcphoff = ipv6_skip_exthdr(oldskb, ((u8 *)(oip6h + 1) - oldskb.data),
    &proto, &frag_off);
    if ((tcphoff < 0) || (tcphoff > oldskb.len)) {
    pr_debug("Cannot get TCP header.\n");
    return core::ptr::null_mut();
    }
// otcplen = oldskb->len - tcphoff;
// IP header checks: fragment, too short.
    if (proto != IPPROTO_TCP || *otcplen < sizeof(struct tcphdr)) {
    pr_debug("proto(%d) != IPPROTO_TCP or too short (len = %d)\n",
    proto, *otcplen);
    return core::ptr::null_mut();
    }
    otcph = skb_header_pointer(oldskb, tcphoff, sizeof(struct tcphdr),
    otcph);
    if (otcph == core::ptr::null_mut())
    return core::ptr::null_mut();
// No RST for RST.
    if (otcph.rst) {
    pr_debug("RST is set\n");
    return core::ptr::null_mut();
    }
// Check checksum.
    if (nf_ip6_checksum(oldskb, hook, tcphoff, IPPROTO_TCP)) {
    pr_debug("TCP checksum is invalid\n");
    return core::ptr::null_mut();
    }
    return otcph;
    }
    static struct ipv6hdr *
    nf_reject_ip6hdr_put(struct sk_buff *nskb,
    const struct sk_buff *oldskb,
    __u8 protocol, int hoplimit)
    {
    struct ipv6hdr *ip6h;
    const struct ipv6hdr *oip6h = ipv6_hdr(oldskb);
pub const DEFAULT_TOS_VALUE: c_uint = 0x0U;
    let mut tclass: __u8 = DEFAULT_TOS_VALUE;
    skb_put(nskb, sizeof(struct ipv6hdr));
    skb_reset_network_header(nskb);
    ip6h = ipv6_hdr(nskb);
    ip6_flow_hdr(ip6h, tclass, 0);
    ip6h.hop_limit = hoplimit;
    ip6h.nexthdr = protocol;
    ip6h.saddr = oip6h.daddr;
    ip6h.daddr = oip6h.saddr;
    nskb.protocol = htons(ETH_P_IPV6);
    return ip6h;
    }
    static void
    nf_reject_ip6_tcphdr_put(struct sk_buff *nskb,
    const struct sk_buff *oldskb,
    const struct tcphdr *oth, unsigned int otcplen)
    {
    struct tcphdr *tcph;
    skb_reset_transport_header(nskb);
    tcph = skb_put_zero(nskb, sizeof(struct tcphdr));
// Truncate to length (no data)
    tcph.doff = sizeof(struct tcphdr)/4;
    tcph.source = oth.dest;
    tcph.dest = oth.source;
    if (oth.ack) {
    tcph.seq = oth.ack_seq;
    } else {
    tcph.ack_seq = htonl(ntohl(oth.seq) + oth.syn + oth.fin +
    otcplen - (oth.doff<<2));
    tcph.ack = 1;
    }
    tcph.rst = 1;
// Adjust TCP checksum
    tcph.check = csum_ipv6_magic(&ipv6_hdr(nskb).saddr,
    &ipv6_hdr(nskb).daddr,
    sizeof(struct tcphdr), IPPROTO_TCP,
    csum_partial(tcph,
    sizeof(struct tcphdr), 0));
    }
#[no_mangle]
unsafe extern "C" fn nf_reject6_fill_skb_dst(skb_in: *mut sk_buff) -> c_int {
    static int nf_reject6_fill_skb_dst(struct sk_buff *skb_in)
    {
    struct dst_entry *dst = core::ptr::null_mut();
    struct flowi fl;
    memset(&fl, 0, sizeof(struct flowi));
    fl.u.ip6.daddr = ipv6_hdr(skb_in).saddr;
    nf_ip6_route(dev_net(skb_in.dev), &dst, &fl, false);
    if (!dst)
    return -1;
    skb_dst_drop(skb_in);
    skb_dst_set(skb_in, dst);
    return 0;
    }
    void nf_send_reset6(struct net *net, struct sock *sk, struct sk_buff *oldskb,
    int hook)
    {
    const struct ipv6hdr *oip6h = ipv6_hdr(oldskb);
    struct dst_entry *dst = core::ptr::null_mut();
    const struct tcphdr *otcph;
    struct sk_buff *nskb;
    struct tcphdr _otcph;
    unsigned int otcplen;
    struct flowi6 fl6;
    if ((!(ipv6_addr_type(&oip6h.saddr) & IPV6_ADDR_UNICAST)) ||
    (!(ipv6_addr_type(&oip6h.daddr) & IPV6_ADDR_UNICAST))) {
    pr_debug("addr is not unicast.\n");
    return;
    }
    otcph = nf_reject_ip6_tcphdr_get(oldskb, &_otcph, &otcplen, hook);
    if (!otcph)
    return;
    memset(&fl6, 0, sizeof(fl6));
    fl6.flowi6_proto = IPPROTO_TCP;
    fl6.saddr = oip6h.daddr;
    fl6.daddr = oip6h.saddr;
    fl6.fl6_sport = otcph.dest;
    fl6.fl6_dport = otcph.source;
    if (!skb_valid_dst(oldskb)) {
    nf_ip6_route(net, &dst, flowi6_to_flowi(&fl6), false);
    if (!dst)
    return;
    skb_dst_drop(oldskb);
    skb_dst_set(oldskb, dst);
    }
    fl6.flowi6_oif = l3mdev_master_ifindex(skb_dst_dev(oldskb));
    fl6.flowi6_mark = IP6_REPLY_MARK(net, oldskb.mark);
    security_skb_classify_flow(oldskb, flowi6_to_flowi_common(&fl6));
    dst = ip6_route_output(net, core::ptr::null_mut(), &fl6);
    if (dst.error) {
    dst_release(dst);
    return;
    }
    dst = xfrm_lookup(net, dst, flowi6_to_flowi(&fl6), core::ptr::null_mut(), 0);
    if (IS_ERR(dst))
    return;
    nskb = alloc_skb(LL_MAX_HEADER + sizeof(struct ipv6hdr) +
    sizeof(struct tcphdr) + dst.trailer_len,
    GFP_ATOMIC);
    if (!nskb) {
    net_dbg_ratelimited("cannot alloc skb\n");
    dst_release(dst);
    return;
    }
    skb_dst_set(nskb, dst);
    nskb.mark = fl6.flowi6_mark;
    skb_reserve(nskb, LL_MAX_HEADER);
    nf_reject_ip6hdr_put(nskb, oldskb, IPPROTO_TCP, ip6_dst_hoplimit(dst));
    nf_reject_ip6_tcphdr_put(nskb, oldskb, otcph, otcplen);
    nf_ct_attach(nskb, oldskb);
    nf_ct_set_closing(skb_nfct(oldskb));

// If we use ip6_local_out for bridged traffic, the MAC source on
// the RST will be ours, instead of the destination's.  This confuses
// some routers/firewalls, and they drop the packet.  So we need to
// build the eth header using the original destination's MAC as the
// source, and send the RST packet directly.
//
    if (nf_bridge_info_exists(oldskb)) {
    struct ethhdr *oeth = eth_hdr(oldskb);
    struct ipv6hdr *ip6h = ipv6_hdr(nskb);
    struct net_device *br_indev;
    br_indev = nf_bridge_get_physindev(oldskb, net);
    if (!br_indev) {
    kfree_skb(nskb);
    return;
    }
    nskb.dev = br_indev;
    nskb.protocol = htons(ETH_P_IPV6);
    ip6h.payload_len = htons(sizeof(struct tcphdr));
    if (dev_hard_header(nskb, nskb.dev, ntohs(nskb.protocol),
    oeth.h_source, oeth.h_dest, nskb.len) < 0) {
    kfree_skb(nskb);
    return;
    }
    dev_queue_xmit(nskb);
    } else

    ip6_local_out(net, sk, nskb);
    }
    EXPORT_SYMBOL_GPL(nf_send_reset6);
#[no_mangle]
unsafe extern "C" fn reject6_csum_ok(skb: *mut sk_buff, hook: c_int) -> bool {
    static bool reject6_csum_ok(struct sk_buff *skb, int hook)
    {
    const struct ipv6hdr *ip6h = ipv6_hdr(skb);
    int thoff;
    __be16 fo;
    u8 proto;
    if (skb_csum_unnecessary(skb))
    return true;
    proto = ip6h.nexthdr;
    thoff = ipv6_skip_exthdr(skb, ((u8 *)(ip6h + 1) - skb.data), &proto, &fo);
    if (thoff < 0 || thoff >= skb.len || (fo & htons(~0x7)) != 0)
    return false;
    if (!nf_reject_verify_csum(skb, thoff, proto))
    return true;
    return nf_ip6_checksum(skb, hook, thoff, proto) == 0;
    }
    void nf_send_unreach6(struct net *net, struct sk_buff *skb_in,
    unsigned char code, unsigned int hooknum)
    {
    if (!reject6_csum_ok(skb_in, hooknum))
    return;
    if (hooknum == NF_INET_LOCAL_OUT && skb_in.dev == core::ptr::null_mut())
    skb_in.dev = net.loopback_dev;
    if (!skb_valid_dst(skb_in) && nf_reject6_fill_skb_dst(skb_in) < 0)
    return;
    icmpv6_send(skb_in, ICMPV6_DEST_UNREACH, code, 0);
    }
    EXPORT_SYMBOL_GPL(nf_send_unreach6);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("IPv6 packet rejection core");
