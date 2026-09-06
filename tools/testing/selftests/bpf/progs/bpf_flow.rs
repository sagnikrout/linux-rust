//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_flow.c
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

pub const FLOW_CONTINUE_SADDR: c_uint = 0x7f00007f /* 127.0.0.127 */;
// These are the identifiers of the BPF programs that will be used in tail
// calls. Name is limited to 16 characters, with the terminating character and
// bpf_func_ above, we have only 6 to work with, anything after will be cropped.
//
pub const IP: c_int = 0;
pub const IPV6: c_int = 1;

pub const MPLS: c_int = 4;
pub const VLAN: c_int = 5;
pub const MAX_PROG: c_int = 6;
pub const IP_MF: c_uint = 0x2000;
pub const IP_OFFSET: c_uint = 0x1FFF;
pub const IP6_MF: c_uint = 0x0001;
pub const IP6_OFFSET: c_uint = 0xFFF8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_hdr {
    pub h_vlan_TCI: __be16,
    pub h_vlan_encapsulated_proto: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gre_hdr {
    pub flags: __be16,
    pub proto: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag_hdr {
    pub nexthdr: __u8,
    pub reserved: __u8,
    pub frag_off: __be16,
    pub identification: __be32,
}

    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, MAX_PROG);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    } jmp_table SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1024);
    __type(key, __u32);
    __type(value, struct bpf_flow_keys);
    } last_dissection SEC(".maps");
    static __always_inline int export_flow_keys(struct bpf_flow_keys *keys,
    int ret)
    {
    let mut key: __u32 = (__u32)(keys.sport) << 16 | keys.dport;
    struct bpf_flow_keys val;
    memcpy(&val, keys, sizeof(val));
    bpf_map_update_elem(&last_dissection, &key, &val, BPF_ANY);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn ip6_flowlabel(hdr: *const ipv6hdr) -> __be32 {
    static inline __be32 ip6_flowlabel(const struct ipv6hdr *hdr)
    {
    return *(__be32 *)hdr & IPV6_FLOWLABEL_MASK;
    }
    static __always_inline void *bpf_flow_dissect_get_header(struct __sk_buff *skb,
    __u16 hdr_size,
    void *buffer)
    {
    void *data_end = (void *)(long)skb.data_end;
    void *data = (void *)(long)skb.data;
    let mut thoff: __u16 = skb.flow_keys.thoff;
    __u8 *hdr;
// Verifies this variable offset does not overflow
    if (thoff > (USHRT_MAX - hdr_size))
    return core::ptr::null_mut();
    hdr = data + thoff;
    if (hdr + hdr_size <= data_end)
    return hdr;
    if (bpf_skb_load_bytes(skb, thoff, buffer, hdr_size))
    return core::ptr::null_mut();
    return buffer;
    }
// Dispatches on ETHERTYPE
#[no_mangle]
unsafe extern "C" fn parse_eth_proto(skb: *mut __sk_buff, proto: __be16) -> __always_inline int {
    static __always_inline int parse_eth_proto(struct __sk_buff *skb, __be16 proto)
    {
    struct bpf_flow_keys *keys = skb.flow_keys;
    switch (proto) {
    case bpf_htons(ETH_P_IP):
    bpf_tail_call_static(skb, &jmp_table, IP);
    break;
    case bpf_htons(ETH_P_IPV6):
    bpf_tail_call_static(skb, &jmp_table, IPV6);
    break;
    case bpf_htons(ETH_P_MPLS_MC):
    case bpf_htons(ETH_P_MPLS_UC):
    bpf_tail_call_static(skb, &jmp_table, MPLS);
    break;
    case bpf_htons(ETH_P_8021Q):
    case bpf_htons(ETH_P_8021AD):
    bpf_tail_call_static(skb, &jmp_table, VLAN);
    break;
    default:
// Protocol not supported
    return export_flow_keys(keys, BPF_DROP);
    }
    return export_flow_keys(keys, BPF_DROP);
    }
    SEC("flow_dissector")
#[no_mangle]
pub unsafe extern "C" fn _dissect(skb: *mut __sk_buff) -> c_int {
    int _dissect(struct __sk_buff *skb)
    {
    struct bpf_flow_keys *keys = skb.flow_keys;
    if (keys.n_proto == bpf_htons(ETH_P_IP)) {
// IP traffic from FLOW_CONTINUE_SADDR falls-back to
// standard dissector
//
    struct iphdr *iph, _iph;
    iph = bpf_flow_dissect_get_header(skb, sizeof(*iph), &_iph);
    if (iph && iph.ihl == 5 &&
    iph.saddr == bpf_htonl(FLOW_CONTINUE_SADDR)) {
    return BPF_FLOW_DISSECTOR_CONTINUE;
    }
    }
    return parse_eth_proto(skb, keys.n_proto);
    }
// Parses on IPPROTO_*
#[no_mangle]
unsafe extern "C" fn parse_ip_proto(skb: *mut __sk_buff, proto: __u8) -> __always_inline int {
    static __always_inline int parse_ip_proto(struct __sk_buff *skb, __u8 proto)
    {
    struct bpf_flow_keys *keys = skb.flow_keys;
    void *data_end = (void *)(long)skb.data_end;
    struct icmphdr *icmp, _icmp;
    struct gre_hdr *gre, _gre;
    struct ethhdr *eth, _eth;
    struct tcphdr *tcp, _tcp;
    struct udphdr *udp, _udp;
    switch (proto) {
    case IPPROTO_ICMP:
    icmp = bpf_flow_dissect_get_header(skb, sizeof(*icmp), &_icmp);
    if (!icmp)
    return export_flow_keys(keys, BPF_DROP);
    return export_flow_keys(keys, BPF_OK);
    case IPPROTO_IPIP:
    keys.is_encap = true;
    if (keys.flags & BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP)
    return export_flow_keys(keys, BPF_OK);
    return parse_eth_proto(skb, bpf_htons(ETH_P_IP));
    case IPPROTO_IPV6:
    keys.is_encap = true;
    if (keys.flags & BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP)
    return export_flow_keys(keys, BPF_OK);
    return parse_eth_proto(skb, bpf_htons(ETH_P_IPV6));
    case IPPROTO_GRE:
    gre = bpf_flow_dissect_get_header(skb, sizeof(*gre), &_gre);
    if (!gre)
    return export_flow_keys(keys, BPF_DROP);
    if (bpf_htons(gre.flags & GRE_VERSION))
// Only inspect standard GRE packets with version 0
    return export_flow_keys(keys, BPF_OK);
    keys.thoff += sizeof(*gre); /* Step over GRE Flags and Proto */
    if (GRE_IS_CSUM(gre.flags))
    keys.thoff += 4; /* Step over chksum and Padding */
    if (GRE_IS_KEY(gre.flags))
    keys.thoff += 4; /* Step over key */
    if (GRE_IS_SEQ(gre.flags))
    keys.thoff += 4; /* Step over sequence number */
    keys.is_encap = true;
    if (keys.flags & BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP)
    return export_flow_keys(keys, BPF_OK);
    if (gre.proto == bpf_htons(ETH_P_TEB)) {
    eth = bpf_flow_dissect_get_header(skb, sizeof(*eth),
    &_eth);
    if (!eth)
    return export_flow_keys(keys, BPF_DROP);
    keys.thoff += sizeof(*eth);
    return parse_eth_proto(skb, eth.h_proto);
    } else {
    return parse_eth_proto(skb, gre.proto);
    }
    case IPPROTO_TCP:
    tcp = bpf_flow_dissect_get_header(skb, sizeof(*tcp), &_tcp);
    if (!tcp)
    return export_flow_keys(keys, BPF_DROP);
    if (tcp.doff < 5)
    return export_flow_keys(keys, BPF_DROP);
    if ((__u8 *)tcp + (tcp.doff << 2) > data_end)
    return export_flow_keys(keys, BPF_DROP);
    keys.sport = tcp.source;
    keys.dport = tcp.dest;
    return export_flow_keys(keys, BPF_OK);
    case IPPROTO_UDP:
    case IPPROTO_UDPLITE:
    udp = bpf_flow_dissect_get_header(skb, sizeof(*udp), &_udp);
    if (!udp)
    return export_flow_keys(keys, BPF_DROP);
    keys.sport = udp.source;
    keys.dport = udp.dest;
    return export_flow_keys(keys, BPF_OK);
    default:
    return export_flow_keys(keys, BPF_DROP);
    }
    return export_flow_keys(keys, BPF_DROP);
    }
#[no_mangle]
unsafe extern "C" fn parse_ipv6_proto(skb: *mut __sk_buff, nexthdr: __u8) -> __always_inline int {
    static __always_inline int parse_ipv6_proto(struct __sk_buff *skb, __u8 nexthdr)
    {
    struct bpf_flow_keys *keys = skb.flow_keys;
    switch (nexthdr) {
    case IPPROTO_HOPOPTS:
    case IPPROTO_DSTOPTS:
    bpf_tail_call_static(skb, &jmp_table, IPV6OP);
    break;
    case IPPROTO_FRAGMENT:
    bpf_tail_call_static(skb, &jmp_table, IPV6FR);
    break;
    default:
    return parse_ip_proto(skb, nexthdr);
    }
    return export_flow_keys(keys, BPF_DROP);
    }
    PROG(IP)(struct __sk_buff *skb)
    {
    void *data_end = (void *)(long)skb.data_end;
    struct bpf_flow_keys *keys = skb.flow_keys;
    void *data = (void *)(long)skb.data;
    struct iphdr *iph, _iph;
    let mut done: bool = false;
    iph = bpf_flow_dissect_get_header(skb, sizeof(*iph), &_iph);
    if (!iph)
    return export_flow_keys(keys, BPF_DROP);
// IP header cannot be smaller than 20 bytes
    if (iph.ihl < 5)
    return export_flow_keys(keys, BPF_DROP);
    keys.addr_proto = ETH_P_IP;
    keys.ipv4_src = iph.saddr;
    keys.ipv4_dst = iph.daddr;
    keys.ip_proto = iph.protocol;
    keys.thoff += iph.ihl << 2;
    if (data + keys.thoff > data_end)
    return export_flow_keys(keys, BPF_DROP);
    if (iph.frag_off & bpf_htons(IP_MF | IP_OFFSET)) {
    keys.is_frag = true;
    if (iph.frag_off & bpf_htons(IP_OFFSET)) {
// From second fragment on, packets do not have headers
// we can parse.
//
    done = true;
    } else {
    keys.is_first_frag = true;
// No need to parse fragmented packet unless
// explicitly asked for.
//
    if (!(keys.flags &
    BPF_FLOW_DISSECTOR_F_PARSE_1ST_FRAG))
    done = true;
    }
    }
    if (done)
    return export_flow_keys(keys, BPF_OK);
    return parse_ip_proto(skb, iph.protocol);
    }
    PROG(IPV6)(struct __sk_buff *skb)
    {
    struct bpf_flow_keys *keys = skb.flow_keys;
    struct ipv6hdr *ip6h, _ip6h;
    ip6h = bpf_flow_dissect_get_header(skb, sizeof(*ip6h), &_ip6h);
    if (!ip6h)
    return export_flow_keys(keys, BPF_DROP);
    keys.addr_proto = ETH_P_IPV6;
    memcpy(&keys.ipv6_src, &ip6h.saddr, 2*sizeof(ip6h.saddr));
    keys.thoff += sizeof(struct ipv6hdr);
    keys.ip_proto = ip6h.nexthdr;
    keys.flow_label = ip6_flowlabel(ip6h);
    if (keys.flow_label && keys.flags & BPF_FLOW_DISSECTOR_F_STOP_AT_FLOW_LABEL)
    return export_flow_keys(keys, BPF_OK);
    return parse_ipv6_proto(skb, ip6h.nexthdr);
    }
    PROG(IPV6OP)(struct __sk_buff *skb)
    {
    struct bpf_flow_keys *keys = skb.flow_keys;
    struct ipv6_opt_hdr *ip6h, _ip6h;
    ip6h = bpf_flow_dissect_get_header(skb, sizeof(*ip6h), &_ip6h);
    if (!ip6h)
    return export_flow_keys(keys, BPF_DROP);
// hlen is in 8-octets and does not include the first 8 bytes
// of the header
//
    keys.thoff += (1 + ip6h.hdrlen) << 3;
    keys.ip_proto = ip6h.nexthdr;
    return parse_ipv6_proto(skb, ip6h.nexthdr);
    }
    PROG(IPV6FR)(struct __sk_buff *skb)
    {
    struct bpf_flow_keys *keys = skb.flow_keys;
    struct frag_hdr *fragh, _fragh;
    fragh = bpf_flow_dissect_get_header(skb, sizeof(*fragh), &_fragh);
    if (!fragh)
    return export_flow_keys(keys, BPF_DROP);
    keys.thoff += sizeof(*fragh);
    keys.is_frag = true;
    keys.ip_proto = fragh.nexthdr;
    if (!(fragh.frag_off & bpf_htons(IP6_OFFSET))) {
    keys.is_first_frag = true;
// No need to parse fragmented packet unless
// explicitly asked for.
//
    if (!(keys.flags & BPF_FLOW_DISSECTOR_F_PARSE_1ST_FRAG))
    return export_flow_keys(keys, BPF_OK);
    } else {
    return export_flow_keys(keys, BPF_OK);
    }
    return parse_ipv6_proto(skb, fragh.nexthdr);
    }
    PROG(MPLS)(struct __sk_buff *skb)
    {
    struct bpf_flow_keys *keys = skb.flow_keys;
    struct mpls_label *mpls, _mpls;
    mpls = bpf_flow_dissect_get_header(skb, sizeof(*mpls), &_mpls);
    if (!mpls)
    return export_flow_keys(keys, BPF_DROP);
    return export_flow_keys(keys, BPF_OK);
    }
    PROG(VLAN)(struct __sk_buff *skb)
    {
    struct bpf_flow_keys *keys = skb.flow_keys;
    struct vlan_hdr *vlan, _vlan;
// Account for double-tagging
    if (keys.n_proto == bpf_htons(ETH_P_8021AD)) {
    vlan = bpf_flow_dissect_get_header(skb, sizeof(*vlan), &_vlan);
    if (!vlan)
    return export_flow_keys(keys, BPF_DROP);
    if (vlan.h_vlan_encapsulated_proto != bpf_htons(ETH_P_8021Q))
    return export_flow_keys(keys, BPF_DROP);
    keys.nhoff += sizeof(*vlan);
    keys.thoff += sizeof(*vlan);
    }
    vlan = bpf_flow_dissect_get_header(skb, sizeof(*vlan), &_vlan);
    if (!vlan)
    return export_flow_keys(keys, BPF_DROP);
    keys.nhoff += sizeof(*vlan);
    keys.thoff += sizeof(*vlan);
// Only allow 8021AD + 8021Q double tagging and no triple tagging.
    if (vlan.h_vlan_encapsulated_proto == bpf_htons(ETH_P_8021AD) ||
    vlan.h_vlan_encapsulated_proto == bpf_htons(ETH_P_8021Q))
    return export_flow_keys(keys, BPF_DROP);
    keys.n_proto = vlan.h_vlan_encapsulated_proto;
    return parse_eth_proto(skb, vlan.h_vlan_encapsulated_proto);
    }
    char __license[] SEC("license") = "GPL";
