//! Automatically rewritten from C to Rust
//! Source: net/netfilter/utils.c
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


// SPDX-License-Identifier: GPL-2.0

    __sum16 nf_ip_checksum(struct sk_buff *skb, unsigned int hook,
    unsigned int dataoff, u8 protocol)
    {
    const struct iphdr *iph = ip_hdr(skb);
    let mut csum: __sum16 = 0;
    switch (skb.ip_summed) {
    case CHECKSUM_COMPLETE:
    if (hook != NF_INET_PRE_ROUTING && hook != NF_INET_LOCAL_IN)
    break;
    if ((protocol != IPPROTO_TCP && protocol != IPPROTO_UDP &&
    !csum_fold(skb.csum)) ||
    !csum_tcpudp_magic(iph.saddr, iph.daddr,
    skb.len - dataoff, protocol,
    skb.csum)) {
    skb.ip_summed = CHECKSUM_UNNECESSARY;
    break;
    }
    fallthrough;
    case CHECKSUM_NONE:
    if (protocol != IPPROTO_TCP && protocol != IPPROTO_UDP)
    skb.csum = 0;
    else
    skb.csum = csum_tcpudp_nofold(iph.saddr, iph.daddr,
    skb.len - dataoff,
    protocol, 0);
    csum = __skb_checksum_complete(skb);
    }
    return csum;
    }
    EXPORT_SYMBOL(nf_ip_checksum);

    static __sum16 nf_ip_checksum_partial(struct sk_buff *skb, unsigned int hook,
    unsigned int dataoff, unsigned int len,
    u8 protocol)
    {
    const struct iphdr *iph = ip_hdr(skb);
    let mut csum: __sum16 = 0;
    switch (skb.ip_summed) {
    case CHECKSUM_COMPLETE:
    if (len == skb.len - dataoff)
    return nf_ip_checksum(skb, hook, dataoff, protocol);
    fallthrough;
    case CHECKSUM_NONE:
    skb.csum = csum_tcpudp_nofold(iph.saddr, iph.daddr, protocol,
    skb.len - dataoff, 0);
    skb.ip_summed = CHECKSUM_NONE;
    return __skb_checksum_complete_head(skb, dataoff + len);
    }
    return csum;
    }
    __sum16 nf_ip6_checksum(struct sk_buff *skb, unsigned int hook,
    unsigned int dataoff, u8 protocol)
    {
    const struct ipv6hdr *ip6h = ipv6_hdr(skb);
    let mut csum: __sum16 = 0;
    switch (skb.ip_summed) {
    case CHECKSUM_COMPLETE:
    if (hook != NF_INET_PRE_ROUTING && hook != NF_INET_LOCAL_IN)
    break;
    if (!csum_ipv6_magic(&ip6h.saddr, &ip6h.daddr,
    skb.len - dataoff, protocol,
    csum_sub(skb.csum,
    skb_checksum(skb, 0,
    dataoff, 0)))) {
    skb.ip_summed = CHECKSUM_UNNECESSARY;
    break;
    }
    fallthrough;
    case CHECKSUM_NONE:
    skb.csum = ~csum_unfold(
    csum_ipv6_magic(&ip6h.saddr, &ip6h.daddr,
    skb.len - dataoff,
    protocol,
    csum_sub(0,
    skb_checksum(skb, 0,
    dataoff, 0))));
    csum = __skb_checksum_complete(skb);
    }
    return csum;
    }
    EXPORT_SYMBOL(nf_ip6_checksum);
    static __sum16 nf_ip6_checksum_partial(struct sk_buff *skb, unsigned int hook,
    unsigned int dataoff, unsigned int len,
    u8 protocol)
    {
    const struct ipv6hdr *ip6h = ipv6_hdr(skb);
    __wsum hsum;
    let mut csum: __sum16 = 0;
    switch (skb.ip_summed) {
    case CHECKSUM_COMPLETE:
    if (len == skb.len - dataoff)
    return nf_ip6_checksum(skb, hook, dataoff, protocol);
    fallthrough;
    case CHECKSUM_NONE:
    hsum = skb_checksum(skb, 0, dataoff, 0);
    skb.csum = ~csum_unfold(csum_ipv6_magic(&ip6h.saddr,
    &ip6h.daddr,
    skb.len - dataoff,
    protocol,
    csum_sub(0, hsum)));
    skb.ip_summed = CHECKSUM_NONE;
    return __skb_checksum_complete_head(skb, dataoff + len);
    }
    return csum;
    };
    __sum16 nf_checksum(struct sk_buff *skb, unsigned int hook,
    unsigned int dataoff, u8 protocol,
    unsigned short family)
    {
    let mut csum: __sum16 = 0;
    switch (family) {
    case AF_INET:
    csum = nf_ip_checksum(skb, hook, dataoff, protocol);
    break;
    case AF_INET6:
    csum = nf_ip6_checksum(skb, hook, dataoff, protocol);
    break;
    }
    return csum;
    }
    EXPORT_SYMBOL_GPL(nf_checksum);
    __sum16 nf_checksum_partial(struct sk_buff *skb, unsigned int hook,
    unsigned int dataoff, unsigned int len,
    u8 protocol, unsigned short family)
    {
    let mut csum: __sum16 = 0;
    switch (family) {
    case AF_INET:
    csum = nf_ip_checksum_partial(skb, hook, dataoff, len,
    protocol);
    break;
    case AF_INET6:
    csum = nf_ip6_checksum_partial(skb, hook, dataoff, len,
    protocol);
    break;
    }
    return csum;
    }
    EXPORT_SYMBOL_GPL(nf_checksum_partial);
    int nf_route(struct net *net, struct dst_entry **dst, struct flowi *fl,
    bool strict, unsigned short family)
    {
    let mut ret: c_int = 0;
    switch (family) {
    case AF_INET:
    ret = nf_ip_route(net, dst, fl, strict);
    break;
    case AF_INET6:
    ret = nf_ip6_route(net, dst, fl, strict);
    break;
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(nf_route);
// Only get and check the lengths, not do any hop-by-hop stuff.
#[no_mangle]
pub unsafe extern "C" fn nf_ip6_check_hbh_len(skb: *mut sk_buff, plen: *mut u32) -> c_int {
    int nf_ip6_check_hbh_len(struct sk_buff *skb, u32 *plen)
    {
    int len, off = sizeof(struct ipv6hdr);
    unsigned char *nh;
    if (!pskb_may_pull(skb, off + 8))
    return -ENOMEM;
    nh = (unsigned char *)(ipv6_hdr(skb) + 1);
    len = (nh[1] + 1) << 3;
    if (!pskb_may_pull(skb, off + len))
    return -ENOMEM;
    nh = skb_network_header(skb);
    off += 2;
    len -= 2;
    while (len > 0) {
    int optlen;
    if (nh[off] == IPV6_TLV_PAD1) {
    off++;
    len--;
    continue;
    }
    if (len < 2)
    return -EBADMSG;
    optlen = nh[off + 1] + 2;
    if (optlen > len)
    return -EBADMSG;
    if (nh[off] == IPV6_TLV_JUMBO) {
    u32 pkt_len;
    if (nh[off + 1] != 4 || (off & 3) != 2)
    return -EBADMSG;
    pkt_len = ntohl(*(__be32 *)(nh + off + 2));
    if (pkt_len <= IPV6_MAXPLEN ||
    ipv6_hdr(skb).payload_len)
    return -EBADMSG;
    if (pkt_len > skb.len - sizeof(struct ipv6hdr))
    return -EBADMSG;
// plen = pkt_len;
    }
    off += optlen;
    len -= optlen;
    }
    return len ? -EBADMSG : 0;
    }
    EXPORT_SYMBOL_GPL(nf_ip6_check_hbh_len);
