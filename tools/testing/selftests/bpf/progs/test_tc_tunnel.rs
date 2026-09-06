//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tc_tunnel.c
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
// In-place tunneling

    let mut cfg_port: static int = 8000;
    let mut cfg_udp_src: static int = 20000;
pub const ETH_P_MPLS_UC: c_uint = 0x8847;
pub const ETH_P_TEB: c_uint = 0x6558;
pub const MPLS_LS_S_MASK: c_uint = 0x00000100;

    (((__u64)len & BPF_ADJ_ROOM_ENCAP_L2_MASK)	\
    << BPF_ADJ_ROOM_ENCAP_L2_SHIFT)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlanhdr___local {
    pub vx_flags: __be32,
    pub vx_vni: __be32,
}

pub const UDP_PORT: c_int = 5555;
pub const MPLS_OVER_UDP_PORT: c_int = 6635;
pub const ETH_OVER_UDP_PORT: c_int = 7777;
pub const VXLAN_UDP_PORT: c_int = 8472;
pub const EXTPROTO_VXLAN: c_uint = 0x1;

    SKB_GSO_UDP_TUNNEL_CSUM)

    SKB_GSO_GRE |				\
    SKB_GSO_GRE_CSUM |			\
    SKB_GSO_IPXIP4 |			\
    SKB_GSO_IPXIP6 |			\
    SKB_GSO_ESP)

    BPF_F_ADJ_ROOM_DECAP_L4_GRE)

    BPF_F_ADJ_ROOM_DECAP_IPXIP6)

pub const VNI_ID: c_int = 1;

pub const NEXTHDR_DEST: c_int = 60;

// MPLS label 1000 with S bit (last label) set and ttl of 255.
    static const __u32 mpls_label = __bpf_constant_htonl(1000 << 12 |
    MPLS_LS_S_MASK | 0xff);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gre_hdr {
    pub flags: __be16,
    pub protocol: __be16,
    pub __attribute__((packed)): },
    union l4hdr {
    pub udp: udphdr,
    pub gre: gre_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4hdr {
    pub ip: iphdr,
    pub l4hdr: union l4hdr,
    pub /: *mut *mut __u8 pad[L2_PAD_SZ]; / space for L2 header / vxlan header ...,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v6hdr {
    pub ip: ipv6hdr,
    pub l4hdr: union l4hdr,
    pub /: *mut *mut __u8 pad[L2_PAD_SZ]; / space for L2 header / vxlan header ...,
    pub __attribute__((packed)): },
#[no_mangle]
unsafe extern "C" fn set_ipv4_csum(iph: *mut iphdr) -> __always_inline void {
    static __always_inline void set_ipv4_csum(struct iphdr *iph)
    {
    pub )iph: *mut *mut __u16 iph16 = (__u16,
    pub csum: __u32,
    pub i: c_int,
    pub 0: iph->check =,
    __pragma_loop_unroll_full
    pub i++): *mut *mut for (i = 0, csum = 0; i < sizeof(iph) >> 1;,
    pub iph16++: *mut csum +=,
    pub 16)): iph->check = ~((csum & 0xffff) + (csum >>,
    }
    static __always_inline int __encap_ipv4(struct __sk_buff *skb, __u8 encap_proto,
    __u16 l2_proto, __u16 ext_proto)
    {
    pub {0}: iphdr iph_inner =,
    pub UDP_PORT: __u16 udp_dst =,
    pub h_outer: v4hdr,
    pub tcph: tcphdr,
    pub l2_len: int olen,,
    pub NULL: *mut *mut __u8 l2_hdr =,
    pub tcp_off: c_int,
    pub flags: __u64,
// Most tests encapsulate a packet into a tunnel with the same
// network protocol, and derive the outer header fields from
// the inner header.
//
// The 6in4 case tests different inner and outer protocols. As
// the inner is ipv6, but the outer expects an ipv4 header as
// input, manually build a struct iphdr based on the ipv6hdr.
//
    if (encap_proto == IPPROTO_IPV6) {
    pub 1: __u32 saddr = (192 << 24) | (168 << 16) | (1 << 8) |,
    pub 2: __u32 daddr = (192 << 24) | (168 << 16) | (1 << 8) |,
    pub iph6_inner: ipv6hdr,
// Read the IPv6 header
    if (bpf_skb_load_bytes(skb, ETH_HLEN, &iph6_inner,
    sizeof(iph6_inner)) < 0)
    pub TC_ACT_OK: return,
// Derive the IPv4 header fields from the IPv6 header
    pub 4: iph_inner.version =,
    pub 5: iph_inner.ihl =,
    iph_inner.tot_len = bpf_htons(sizeof(iph6_inner) +
    pub 1: iph_inner.ttl = iph6_inner.hop_limit -,
    pub iph6_inner.nexthdr: iph_inner.protocol =,
    pub __bpf_constant_htonl(saddr): iph_inner.saddr =,
    pub __bpf_constant_htonl(daddr): iph_inner.daddr =,
    pub sizeof(iph6_inner): tcp_off =,
    } else {
    if (bpf_skb_load_bytes(skb, ETH_HLEN, &iph_inner,
    sizeof(iph_inner)) < 0)
    pub TC_ACT_OK: return,
    pub sizeof(iph_inner): tcp_off =,
    }
// filter only packets we want
    if (iph_inner.ihl != 5 || iph_inner.protocol != IPPROTO_TCP)
    pub TC_ACT_OK: return,
    if (bpf_skb_load_bytes(skb, ETH_HLEN + tcp_off,
    &tcph, sizeof(tcph)) < 0)
    pub TC_ACT_OK: return,
    if (tcph.dest != __bpf_constant_htons(cfg_port))
    pub TC_ACT_OK: return,
    pub sizeof(h_outer.ip): olen =,
    pub 0: l2_len =,
    pub BPF_F_ADJ_ROOM_ENCAP_L3_IPV4: flags = BPF_F_ADJ_ROOM_FIXED_GSO |,
    switch (l2_proto) {
    case ETH_P_MPLS_UC:
    pub sizeof(mpls_label): l2_len =,
    pub MPLS_OVER_UDP_PORT: udp_dst =,
    case ETH_P_TEB:
    pub ETH_HLEN: l2_len =,
    if (ext_proto & EXTPROTO_VXLAN) {
    pub VXLAN_UDP_PORT: udp_dst =,
    pub vxlanhdr___local): l2_len += sizeof(struct,
    } else
    pub ETH_OVER_UDP_PORT: udp_dst =,
    }
    pub BPF_F_ADJ_ROOM_ENCAP_L2(l2_len): flags |=,
    switch (encap_proto) {
    case IPPROTO_GRE:
    pub BPF_F_ADJ_ROOM_ENCAP_L4_GRE: flags |=,
    pub sizeof(h_outer.l4hdr.gre): olen +=,
    pub bpf_htons(l2_proto): h_outer.l4hdr.gre.protocol =,
    pub 0: h_outer.l4hdr.gre.flags =,
    case IPPROTO_UDP:
    pub BPF_F_ADJ_ROOM_ENCAP_L4_UDP: flags |=,
    pub sizeof(h_outer.l4hdr.udp): olen +=,
    pub __bpf_constant_htons(cfg_udp_src): h_outer.l4hdr.udp.source =,
    pub bpf_htons(udp_dst): h_outer.l4hdr.udp.dest =,
    pub 0: h_outer.l4hdr.udp.check =,
    h_outer.l4hdr.udp.len = bpf_htons(bpf_ntohs(iph_inner.tot_len) +
    sizeof(h_outer.l4hdr.udp) +
    case IPPROTO_IPIP:
    case IPPROTO_IPV6:
    default:
    pub TC_ACT_OK: return,
    }
// add L2 encap (if specified)
    pub olen: *mut *mut l2_hdr = (__u8 )&h_outer +,
    switch (l2_proto) {
    case ETH_P_MPLS_UC:
// (__u32 *)l2_hdr = mpls_label;
    case ETH_P_TEB:
    pub BPF_F_ADJ_ROOM_ENCAP_L2_ETH: flags |=,
    if (ext_proto & EXTPROTO_VXLAN) {
    pub )l2_hdr: *mut *mut vxlanhdr___local vxlan_hdr = (vxlanhdr___local,
    pub VXLAN_FLAGS: vxlan_hdr->vx_flags =,
    pub VXLAN_VNI: vxlan_hdr->vx_vni =,
    pub vxlanhdr___local): l2_hdr += sizeof(struct,
    }
    if (bpf_skb_load_bytes(skb, 0, l2_hdr, ETH_HLEN))
    pub TC_ACT_SHOT: return,
    }
    pub l2_len: olen +=,
// add room between mac and network header
    if (bpf_skb_adjust_room(skb, olen, BPF_ADJ_ROOM_MAC, flags))
    pub TC_ACT_SHOT: return,
// prepare new outer network header
    pub iph_inner: h_outer.ip =,
    h_outer.ip.tot_len = bpf_htons(olen +
    pub encap_proto: h_outer.ip.protocol =,
    pub )&h_outer.ip): *mut set_ipv4_csum((void,
// store new outer network header
    if (bpf_skb_store_bytes(skb, ETH_HLEN, &h_outer, olen,
    BPF_F_INVALIDATE_HASH) < 0)
    pub TC_ACT_SHOT: return,
// if changing outer proto type, update eth->h_proto
    if (encap_proto == IPPROTO_IPV6) {
    pub eth: ethhdr,
    if (bpf_skb_load_bytes(skb, 0, &eth, sizeof(eth)) < 0)
    pub TC_ACT_SHOT: return,
    pub bpf_htons(ETH_P_IP): eth.h_proto =,
    if (bpf_skb_store_bytes(skb, 0, &eth, sizeof(eth), 0) < 0)
    pub TC_ACT_SHOT: return,
    }
    pub TC_ACT_OK: return,
    }
    static __always_inline int encap_ipv4(struct __sk_buff *skb, __u8 encap_proto,
    __u16 l2_proto)
    {
    pub 0): return __encap_ipv4(skb, encap_proto, l2_proto,,
    }
    static __always_inline int __encap_ipv6(struct __sk_buff *skb, __u8 encap_proto,
    __u16 l2_proto, __u16 ext_proto)
    {
    pub UDP_PORT: __u16 udp_dst =,
    pub iph_inner: ipv6hdr,
    pub h_outer: v6hdr,
    pub tcph: tcphdr,
    pub l2_len: int olen,,
    pub NULL: *mut *mut __u8 l2_hdr =,
    pub tot_len: __u16,
    pub flags: __u64,
    if (bpf_skb_load_bytes(skb, ETH_HLEN, &iph_inner,
    sizeof(iph_inner)) < 0)
    pub TC_ACT_OK: return,
// filter only packets we want
    if (bpf_skb_load_bytes(skb, ETH_HLEN + sizeof(iph_inner),
    &tcph, sizeof(tcph)) < 0)
    pub TC_ACT_OK: return,
    if (tcph.dest != __bpf_constant_htons(cfg_port))
    pub TC_ACT_OK: return,
    pub sizeof(h_outer.ip): olen =,
    pub 0: l2_len =,
    pub BPF_F_ADJ_ROOM_ENCAP_L3_IPV6: flags = BPF_F_ADJ_ROOM_FIXED_GSO |,
    switch (l2_proto) {
    case ETH_P_MPLS_UC:
    pub sizeof(mpls_label): l2_len =,
    pub MPLS_OVER_UDP_PORT: udp_dst =,
    case ETH_P_TEB:
    pub ETH_HLEN: l2_len =,
    if (ext_proto & EXTPROTO_VXLAN) {
    pub VXLAN_UDP_PORT: udp_dst =,
    pub vxlanhdr___local): l2_len += sizeof(struct,
    } else
    pub ETH_OVER_UDP_PORT: udp_dst =,
    }
    pub BPF_F_ADJ_ROOM_ENCAP_L2(l2_len): flags |=,
    switch (encap_proto) {
    case IPPROTO_GRE:
    pub BPF_F_ADJ_ROOM_ENCAP_L4_GRE: flags |=,
    pub sizeof(h_outer.l4hdr.gre): olen +=,
    pub bpf_htons(l2_proto): h_outer.l4hdr.gre.protocol =,
    pub 0: h_outer.l4hdr.gre.flags =,
    case IPPROTO_UDP:
    pub BPF_F_ADJ_ROOM_ENCAP_L4_UDP: flags |=,
    pub sizeof(h_outer.l4hdr.udp): olen +=,
    pub __bpf_constant_htons(cfg_udp_src): h_outer.l4hdr.udp.source =,
    pub bpf_htons(udp_dst): h_outer.l4hdr.udp.dest =,
    tot_len = bpf_ntohs(iph_inner.payload_len) + sizeof(iph_inner) +
    pub l2_len: sizeof(h_outer.l4hdr.udp) +,
    pub 0: h_outer.l4hdr.udp.check =,
    pub bpf_htons(tot_len): h_outer.l4hdr.udp.len =,
    case IPPROTO_IPV6:
    default:
    pub TC_ACT_OK: return,
    }
// add L2 encap (if specified)
    pub olen: *mut *mut l2_hdr = (__u8 )&h_outer +,
    switch (l2_proto) {
    case ETH_P_MPLS_UC:
// (__u32 *)l2_hdr = mpls_label;
    case ETH_P_TEB:
    pub BPF_F_ADJ_ROOM_ENCAP_L2_ETH: flags |=,
    if (ext_proto & EXTPROTO_VXLAN) {
    pub )l2_hdr: *mut *mut vxlanhdr___local vxlan_hdr = (vxlanhdr___local,
    pub VXLAN_FLAGS: vxlan_hdr->vx_flags =,
    pub VXLAN_VNI: vxlan_hdr->vx_vni =,
    pub vxlanhdr___local): l2_hdr += sizeof(struct,
    }
    if (bpf_skb_load_bytes(skb, 0, l2_hdr, ETH_HLEN))
    pub TC_ACT_SHOT: return,
    }
    pub l2_len: olen +=,
// add room between mac and network header
    if (bpf_skb_adjust_room(skb, olen, BPF_ADJ_ROOM_MAC, flags))
    pub TC_ACT_SHOT: return,
// prepare new outer network header
    pub iph_inner: h_outer.ip =,
    h_outer.ip.payload_len = bpf_htons(olen +
    pub encap_proto: h_outer.ip.nexthdr =,
// store new outer network header
    if (bpf_skb_store_bytes(skb, ETH_HLEN, &h_outer, olen,
    BPF_F_INVALIDATE_HASH) < 0)
    pub TC_ACT_SHOT: return,
    pub TC_ACT_OK: return,
    }
#[no_mangle]
unsafe extern "C" fn encap_ipv6_ipip6(skb: *mut __sk_buff) -> c_int {
    static int encap_ipv6_ipip6(struct __sk_buff *skb)
    {
    pub {0}: v6hdr h_outer =,
    pub iph_inner: iphdr,
    pub tcph: tcphdr,
    pub eth: ethhdr,
    pub flags: __u64,
    pub olen: c_int,
    if (bpf_skb_load_bytes(skb, ETH_HLEN, &iph_inner,
    sizeof(iph_inner)) < 0)
    pub TC_ACT_OK: return,
// filter only packets we want
    if (bpf_skb_load_bytes(skb, ETH_HLEN + (iph_inner.ihl << 2),
    &tcph, sizeof(tcph)) < 0)
    pub TC_ACT_OK: return,
    if (tcph.dest != __bpf_constant_htons(cfg_port))
    pub TC_ACT_OK: return,
    pub sizeof(h_outer.ip): olen =,
    pub BPF_F_ADJ_ROOM_ENCAP_L3_IPV6: flags = BPF_F_ADJ_ROOM_FIXED_GSO |,
// add room between mac and network header
    if (bpf_skb_adjust_room(skb, olen, BPF_ADJ_ROOM_MAC, flags))
    pub TC_ACT_SHOT: return,
// prepare new outer network header
    pub 6: h_outer.ip.version =,
    pub iph_inner.ttl: h_outer.ip.hop_limit =,
    pub 0xfd: h_outer.ip.saddr.in6_u.u6_addr8[1] =,
    pub 1: h_outer.ip.saddr.in6_u.u6_addr8[15] =,
    pub 0xfd: h_outer.ip.daddr.in6_u.u6_addr8[1] =,
    pub 2: h_outer.ip.daddr.in6_u.u6_addr8[15] =,
    pub iph_inner.tot_len: h_outer.ip.payload_len =,
    pub IPPROTO_IPIP: h_outer.ip.nexthdr =,
// store new outer network header
    if (bpf_skb_store_bytes(skb, ETH_HLEN, &h_outer, olen,
    BPF_F_INVALIDATE_HASH) < 0)
    pub TC_ACT_SHOT: return,
// update eth->h_proto
    if (bpf_skb_load_bytes(skb, 0, &eth, sizeof(eth)) < 0)
    pub TC_ACT_SHOT: return,
    pub bpf_htons(ETH_P_IPV6): eth.h_proto =,
    if (bpf_skb_store_bytes(skb, 0, &eth, sizeof(eth), 0) < 0)
    pub TC_ACT_SHOT: return,
    pub TC_ACT_OK: return,
    }
    static __always_inline int encap_ipv6(struct __sk_buff *skb, __u8 encap_proto,
    __u16 l2_proto)
    {
    pub 0): return __encap_ipv6(skb, encap_proto, l2_proto,,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_ipip_none(skb: *mut __sk_buff) -> c_int {
    int __encap_ipip_none(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IP))
    pub ETH_P_IP): return encap_ipv4(skb, IPPROTO_IPIP,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_gre_none(skb: *mut __sk_buff) -> c_int {
    int __encap_gre_none(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IP))
    pub ETH_P_IP): return encap_ipv4(skb, IPPROTO_GRE,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_gre_mpls(skb: *mut __sk_buff) -> c_int {
    int __encap_gre_mpls(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IP))
    pub ETH_P_MPLS_UC): return encap_ipv4(skb, IPPROTO_GRE,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_gre_eth(skb: *mut __sk_buff) -> c_int {
    int __encap_gre_eth(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IP))
    pub ETH_P_TEB): return encap_ipv4(skb, IPPROTO_GRE,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_udp_none(skb: *mut __sk_buff) -> c_int {
    int __encap_udp_none(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IP))
    pub ETH_P_IP): return encap_ipv4(skb, IPPROTO_UDP,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_udp_mpls(skb: *mut __sk_buff) -> c_int {
    int __encap_udp_mpls(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IP))
    pub ETH_P_MPLS_UC): return encap_ipv4(skb, IPPROTO_UDP,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_udp_eth(skb: *mut __sk_buff) -> c_int {
    int __encap_udp_eth(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IP))
    pub ETH_P_TEB): return encap_ipv4(skb, IPPROTO_UDP,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_vxlan_eth(skb: *mut __sk_buff) -> c_int {
    int __encap_vxlan_eth(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IP))
    return __encap_ipv4(skb, IPPROTO_UDP,
    ETH_P_TEB,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_sit_none(skb: *mut __sk_buff) -> c_int {
    int __encap_sit_none(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IPV6))
    pub ETH_P_IP): return encap_ipv4(skb, IPPROTO_IPV6,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_ip6tnl_none(skb: *mut __sk_buff) -> c_int {
    int __encap_ip6tnl_none(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IPV6))
    pub ETH_P_IPV6): return encap_ipv6(skb, IPPROTO_IPV6,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_ipip6_none(skb: *mut __sk_buff) -> c_int {
    int __encap_ipip6_none(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IP))
    pub encap_ipv6_ipip6(skb): return,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_ip6gre_none(skb: *mut __sk_buff) -> c_int {
    int __encap_ip6gre_none(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IPV6))
    pub ETH_P_IPV6): return encap_ipv6(skb, IPPROTO_GRE,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_ip6gre_mpls(skb: *mut __sk_buff) -> c_int {
    int __encap_ip6gre_mpls(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IPV6))
    pub ETH_P_MPLS_UC): return encap_ipv6(skb, IPPROTO_GRE,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_ip6gre_eth(skb: *mut __sk_buff) -> c_int {
    int __encap_ip6gre_eth(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IPV6))
    pub ETH_P_TEB): return encap_ipv6(skb, IPPROTO_GRE,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_ip6udp_none(skb: *mut __sk_buff) -> c_int {
    int __encap_ip6udp_none(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IPV6))
    pub ETH_P_IPV6): return encap_ipv6(skb, IPPROTO_UDP,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_ip6udp_mpls(skb: *mut __sk_buff) -> c_int {
    int __encap_ip6udp_mpls(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IPV6))
    pub ETH_P_MPLS_UC): return encap_ipv6(skb, IPPROTO_UDP,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_ip6udp_eth(skb: *mut __sk_buff) -> c_int {
    int __encap_ip6udp_eth(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IPV6))
    pub ETH_P_TEB): return encap_ipv6(skb, IPPROTO_UDP,,
    else
    pub TC_ACT_OK: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __encap_ip6vxlan_eth(skb: *mut __sk_buff) -> c_int {
    int __encap_ip6vxlan_eth(struct __sk_buff *skb)
    {
    if (skb.protocol == __bpf_constant_htons(ETH_P_IPV6))
    return __encap_ipv6(skb, IPPROTO_UDP,
    ETH_P_TEB,
    else
    pub TC_ACT_OK: return,
    }
    static int decap_internal(struct __sk_buff *skb, int off, int len, char proto,
    __u64 ipxip_flag)
    {
    pub BPF_F_ADJ_ROOM_FIXED_GSO: __u64 flags =,
    pub kskb: *mut sk_buff,
    pub shinfo: *mut skb_shared_info,
    pub ip6_opt_hdr: ipv6_opt_hdr,
    pub greh: gre_hdr,
    pub udph: udphdr,
    pub len: int olen =,
    switch (proto) {
    case IPPROTO_IPIP:
    flags |= BPF_F_ADJ_ROOM_DECAP_L3_IPV4 |
    case IPPROTO_IPV6:
    flags |= BPF_F_ADJ_ROOM_DECAP_L3_IPV6 |
    case NEXTHDR_DEST:
    if (bpf_skb_load_bytes(skb, off + len, &ip6_opt_hdr,
    sizeof(ip6_opt_hdr)) < 0)
    pub TC_ACT_OK: return,
    switch (ip6_opt_hdr.nexthdr) {
    case IPPROTO_IPIP:
    flags |= BPF_F_ADJ_ROOM_DECAP_L3_IPV4 |
    case IPPROTO_IPV6:
    flags |= BPF_F_ADJ_ROOM_DECAP_L3_IPV6 |
    default:
    pub TC_ACT_OK: return,
    }
    case IPPROTO_GRE:
    pub gre_hdr): olen += sizeof(struct,
    if (!bpf_core_enum_value_exists(enum bpf_adj_room_flags,
    BPF_F_ADJ_ROOM_DECAP_L4_GRE))
    pub TC_ACT_SHOT: return,
    pub BPF_F_ADJ_ROOM_DECAP_L4_GRE: flags |=,
    if (bpf_skb_load_bytes(skb, off + len, &greh, sizeof(greh)) < 0)
    pub TC_ACT_OK: return,
    switch (bpf_ntohs(greh.protocol)) {
    case ETH_P_MPLS_UC:
    pub sizeof(mpls_label): olen +=,
    case ETH_P_TEB:
    pub ETH_HLEN: olen +=,
    }
    case IPPROTO_UDP:
    pub udphdr): olen += sizeof(struct,
    if (!bpf_core_enum_value_exists(enum bpf_adj_room_flags,
    BPF_F_ADJ_ROOM_DECAP_L4_UDP))
    pub TC_ACT_SHOT: return,
    pub BPF_F_ADJ_ROOM_DECAP_L4_UDP: flags |=,
    if (bpf_skb_load_bytes(skb, off + len, &udph, sizeof(udph)) < 0)
    pub TC_ACT_OK: return,
    switch (bpf_ntohs(udph.dest)) {
    case MPLS_OVER_UDP_PORT:
    pub sizeof(mpls_label): olen +=,
    case ETH_OVER_UDP_PORT:
    pub ETH_HLEN: olen +=,
    case VXLAN_UDP_PORT:
    pub vxlanhdr___local): olen += ETH_HLEN + sizeof(struct,
    }
    default:
    pub TC_ACT_OK: return,
    }
    if (bpf_skb_adjust_room(skb, -olen, BPF_ADJ_ROOM_MAC, flags))
    pub TC_ACT_SHOT: return,
    pub bpf_cast_to_kern_ctx(skb): kskb =,
    pub skb_shared_info): shinfo = bpf_core_cast(kskb->head + kskb->end, struct,
    if (shinfo.gso_size) {
    if ((flags & BPF_F_ADJ_ROOM_DECAP_L4_UDP) &&
    (shinfo.gso_type & SKB_GSO_UDP_TUNNEL_MASK))
    pub TC_ACT_SHOT: return,
    if ((flags & BPF_F_ADJ_ROOM_DECAP_L4_GRE) &&
    (shinfo.gso_type & (SKB_GSO_GRE | SKB_GSO_GRE_CSUM)))
    pub TC_ACT_SHOT: return,
    if ((flags & BPF_F_ADJ_ROOM_DECAP_IPXIP4) &&
    (shinfo.gso_type & SKB_GSO_IPXIP4))
    pub TC_ACT_SHOT: return,
    if ((flags & BPF_F_ADJ_ROOM_DECAP_IPXIP6) &&
    (shinfo.gso_type & SKB_GSO_IPXIP6))
    pub TC_ACT_SHOT: return,
    if (flags & (BPF_F_ADJ_ROOM_DECAP_L4_MASK |
    BPF_F_ADJ_ROOM_DECAP_IPXIP_MASK)) {
    if ((shinfo.gso_type & SKB_GSO_TUNNEL_MASK) &&
    !kskb.encapsulation)
    pub TC_ACT_SHOT: return,
    if (!(shinfo.gso_type & SKB_GSO_TUNNEL_MASK) &&
    kskb.encapsulation)
    pub TC_ACT_SHOT: return,
    }
    } else if ((flags & (BPF_F_ADJ_ROOM_DECAP_L4_MASK |
    BPF_F_ADJ_ROOM_DECAP_IPXIP_MASK)) &&
    kskb.encapsulation) {
    pub TC_ACT_SHOT: return,
    }
    pub TC_ACT_OK: return,
    }
#[no_mangle]
unsafe extern "C" fn decap_ipv4(skb: *mut __sk_buff) -> c_int {
    static int decap_ipv4(struct __sk_buff *skb)
    {
    pub iph_outer: iphdr,
    if (!bpf_core_enum_value_exists(enum bpf_adj_room_flags,
    BPF_F_ADJ_ROOM_DECAP_IPXIP4))
    pub TC_ACT_SHOT: return,
    if (bpf_skb_load_bytes(skb, ETH_HLEN, &iph_outer,
    sizeof(iph_outer)) < 0)
    pub TC_ACT_OK: return,
    if (iph_outer.ihl != 5)
    pub TC_ACT_OK: return,
    return decap_internal(skb, ETH_HLEN, sizeof(iph_outer),
    iph_outer.protocol,
    }
#[no_mangle]
unsafe extern "C" fn decap_ipv6(skb: *mut __sk_buff) -> c_int {
    static int decap_ipv6(struct __sk_buff *skb)
    {
    pub iph_outer: ipv6hdr,
    if (!bpf_core_enum_value_exists(enum bpf_adj_room_flags,
    BPF_F_ADJ_ROOM_DECAP_IPXIP6))
    pub TC_ACT_SHOT: return,
    if (bpf_skb_load_bytes(skb, ETH_HLEN, &iph_outer,
    sizeof(iph_outer)) < 0)
    pub TC_ACT_OK: return,
    return decap_internal(skb, ETH_HLEN, sizeof(iph_outer),
    iph_outer.nexthdr,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn decap_f(skb: *mut __sk_buff) -> c_int {
    int decap_f(struct __sk_buff *skb)
    {
    switch (skb.protocol) {
    case __bpf_constant_htons(ETH_P_IP):
    pub decap_ipv4(skb): return,
    case __bpf_constant_htons(ETH_P_IPV6):
    pub decap_ipv6(skb): return,
    default:
// does not match, ignore
    pub TC_ACT_OK: return,
    }
    }
    pub "GPL": char __license[] SEC("license") =,
