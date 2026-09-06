//! Automatically rewritten from C to Rust
//! Source: net/ipv6/udp_offload.c
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
// IPV6 GSO/GRO offload support
// Linux INET6 implementation
//
// UDPv6 GSO support
//

    static struct sk_buff *udp6_ufo_fragment(struct sk_buff *skb,
    netdev_features_t features)
    {
    struct sk_buff *segs = ERR_PTR(-EINVAL);
    unsigned int mss;
    unsigned int unfrag_ip6hlen, unfrag_len;
    struct frag_hdr *fptr;
    u8 *packet_start, *prevhdr;
    u8 nexthdr;
    let mut frag_hdr_sz: u8 = sizeof(struct frag_hdr);
    __wsum csum;
    int tnl_hlen;
    int err;
    if (skb.encapsulation && skb_shinfo(skb).gso_type &
    (SKB_GSO_UDP_TUNNEL|SKB_GSO_UDP_TUNNEL_CSUM))
    segs = skb_udp_tunnel_segment(skb, features, true);
    else {
    const struct ipv6hdr *ipv6h;
    struct udphdr *uh;
    if (!(skb_shinfo(skb).gso_type & (SKB_GSO_UDP | SKB_GSO_UDP_L4)))
    goto out;
    if (!pskb_may_pull(skb, sizeof(struct udphdr)))
    goto out;
    if (skb_shinfo(skb).gso_type & SKB_GSO_UDP_L4)
    return __udp_gso_segment(skb, features, true);
    mss = skb_shinfo(skb).gso_size;
    if (unlikely(skb.len <= mss))
    goto out;
// Do software UFO. Complete and fill in the UDP checksum as HW cannot
// do checksum of UDP packets sent as multiple IP fragments.
//
    uh = udp_hdr(skb);
    ipv6h = ipv6_hdr(skb);
    uh.check = 0;
    csum = skb_checksum(skb, 0, skb.len, 0);
    uh.check = udp_v6_check(skb.len, &ipv6h.saddr,
    &ipv6h.daddr, csum);
    if (uh.check == 0)
    uh.check = CSUM_MANGLED_0;
    skb.ip_summed = CHECKSUM_UNNECESSARY;
// If there is no outer header we can fake a checksum offload
// due to the fact that we have already done the checksum in
// software prior to segmenting the frame.
//
    if (!skb.encap_hdr_csum)
    features |= NETIF_F_HW_CSUM;
// Check if there is enough headroom to insert fragment header.
    tnl_hlen = skb_tnl_header_len(skb);
    if (skb.mac_header < (tnl_hlen + frag_hdr_sz)) {
    if (gso_pskb_expand_head(skb, tnl_hlen + frag_hdr_sz))
    goto out;
    }
// Find the unfragmentable header and shift it left by frag_hdr_sz
// bytes to insert fragment header.
//
    err = ip6_find_1stfragopt(skb, &prevhdr);
    if (err < 0)
    return ERR_PTR(err);
    unfrag_ip6hlen = err;
    nexthdr = *prevhdr;
// prevhdr = NEXTHDR_FRAGMENT;
    unfrag_len = (skb_network_header(skb) - skb_mac_header(skb)) +
    unfrag_ip6hlen + tnl_hlen;
    packet_start = (u8 *) skb.head + SKB_GSO_CB(skb).mac_offset;
    memmove(packet_start-frag_hdr_sz, packet_start, unfrag_len);
    SKB_GSO_CB(skb).mac_offset -= frag_hdr_sz;
    skb.mac_header -= frag_hdr_sz;
    skb.network_header -= frag_hdr_sz;
    fptr = (struct frag_hdr *)(skb_network_header(skb) + unfrag_ip6hlen);
    fptr.nexthdr = nexthdr;
    fptr.reserved = 0;
    fptr.identification = ipv6_proxy_select_ident(dev_net(skb.dev), skb);
// Fragment the skb. ipv6 header and the remaining fields of the
// fragment header are updated in ipv6_gso_segment()
//
    segs = skb_segment(skb, features);
    }
    out:
    return segs;
    }
    static struct sock *udp6_gro_lookup_skb(struct sk_buff *skb, __be16 sport,
    __be16 dport)
    {
    const struct ipv6hdr *iph = skb_gro_network_header(skb);
    struct net *net = dev_net_rcu(skb.dev);
    struct sock *sk;
    int iif, sdif;
    sk = udp_tunnel_sk(net, true);
    if (sk && dport == htons(sk.sk_num))
    return sk;
    inet6_get_iif_sdif(skb, &iif, &sdif);
    return __udp6_lib_lookup(net, &iph.saddr, sport,
    &iph.daddr, dport, iif, sdif, core::ptr::null_mut());
    }
    struct sk_buff *udp6_gro_receive(struct list_head *head, struct sk_buff *skb)
    {
    struct udphdr *uh = udp_gro_udphdr(skb);
    struct sock *sk = core::ptr::null_mut();
    struct sk_buff *pp;
    if (unlikely(!uh))
    goto flush;
// Don't bother verifying checksum if we're going to flush anyway.
    if (NAPI_GRO_CB(skb).flush)
    goto skip;
    if (skb_gro_checksum_validate_zero_check(skb, IPPROTO_UDP, uh.check,
    ip6_gro_compute_pseudo))
    goto flush;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: uh->check) -> else {
    else if (uh.check)
    skb_gro_checksum_try_convert(skb, IPPROTO_UDP,
    ip6_gro_compute_pseudo);
    skip:
    if (static_branch_unlikely(&udpv6_encap_needed_key))
    sk = udp6_gro_lookup_skb(skb, uh.source, uh.dest);
    pp = udp_gro_receive(head, skb, uh, sk);
    return pp;
    flush:
    NAPI_GRO_CB(skb).flush = 1;
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn udp6_gro_complete(skb: *mut sk_buff, nhoff: c_int) -> c_int {
    int udp6_gro_complete(struct sk_buff *skb, int nhoff)
    {
    let mut offset: u16 = NAPI_GRO_CB(skb).network_offsets[skb.encapsulation];
    const struct ipv6hdr *ipv6h = (struct ipv6hdr *)(skb.data + offset);
    struct udphdr *uh = (struct udphdr *)(skb.data + nhoff);
// do fraglist only if there is no outer UDP encap (or we already processed it)
    if (NAPI_GRO_CB(skb).is_flist && !NAPI_GRO_CB(skb).encap_mark) {
    udp_set_len(uh, skb.len - nhoff);
    skb_shinfo(skb).gso_type |= (SKB_GSO_FRAGLIST|SKB_GSO_UDP_L4);
    skb_shinfo(skb).gso_segs = NAPI_GRO_CB(skb).count;
    __skb_incr_checksum_unnecessary(skb);
    return 0;
    }
    if (uh.check)
    uh.check = ~udp_v6_check(skb.len - nhoff, &ipv6h.saddr,
    &ipv6h.daddr, 0);
    return udp_gro_complete(skb, nhoff, udp6_lib_lookup_skb);
    }
#[no_mangle]
pub unsafe extern "C" fn udpv6_offload_init() -> int __init {
    int __init udpv6_offload_init(void)
    {
    net_hotdata.udpv6_offload = (struct net_offload) {
    .callbacks = {
    .gso_segment	=	udp6_ufo_fragment,
    .gro_receive	=	udp6_gro_receive,
    .gro_complete	=	udp6_gro_complete,
    },
    };
    return inet6_add_offload(&net_hotdata.udpv6_offload, IPPROTO_UDP);
    }
#[no_mangle]
pub unsafe extern "C" fn udpv6_offload_exit() -> c_int {
    int udpv6_offload_exit(void)
    {
    return inet6_del_offload(&net_hotdata.udpv6_offload, IPPROTO_UDP);
    }
