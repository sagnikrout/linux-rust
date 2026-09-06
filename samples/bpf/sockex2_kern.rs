//! Automatically rewritten from C to Rust
//! Source: samples/bpf/sockex2_kern.c
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


pub const IP_MF: c_uint = 0x2000;
pub const IP_OFFSET: c_uint = 0x1FFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_hdr {
    pub h_vlan_TCI: __be16,
    pub h_vlan_encapsulated_proto: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_key_record {
    pub src: __be32,
    pub dst: __be32,
    union {
    pub ports: __be32,
    pub port16: [__be16; 2],
}

    __u16 thoff;
    __u8 ip_proto;
    };
#[no_mangle]
pub unsafe extern "C" fn proto_ports_offset(proto: __u64) -> c_int {
    static inline int proto_ports_offset(__u64 proto)
    {
    switch (proto) {
    case IPPROTO_TCP:
    case IPPROTO_UDP:
    case IPPROTO_ESP:
    case IPPROTO_SCTP:
    case IPPROTO_UDPLITE:
    return 0;
    case IPPROTO_AH:
    return 4;
    default:
    return 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ip_is_fragment(ctx: *mut __sk_buff, nhoff: __u64) -> c_int {
    static inline int ip_is_fragment(struct __sk_buff *ctx, __u64 nhoff)
    {
#[no_mangle]
pub unsafe extern "C" fn load_half(_arg: ctx, iphdr: nhoff + offsetof(struct, _arg: frag_off)) -> return {
    return load_half(ctx, nhoff + offsetof(struct iphdr, frag_off))
    & (IP_MF | IP_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn ipv6_addr_hash(ctx: *mut __sk_buff, off: __u64) -> __u32 {
    static inline __u32 ipv6_addr_hash(struct __sk_buff *ctx, __u64 off)
    {
    let mut w0: __u64 = load_word(ctx, off);
    let mut w1: __u64 = load_word(ctx, off + 4);
    let mut w2: __u64 = load_word(ctx, off + 8);
    let mut w3: __u64 = load_word(ctx, off + 12);
    return (__u32)(w0 ^ w1 ^ w2 ^ w3);
    }
    static inline __u64 parse_ip(struct __sk_buff *skb, __u64 nhoff, __u64 *ip_proto,
    struct flow_key_record *flow)
    {
    __u64 verlen;
    if (unlikely(ip_is_fragment(skb, nhoff)))
// ip_proto = 0;
    else
// ip_proto = load_byte(skb, nhoff + offsetof(struct iphdr, protocol));
    if (*ip_proto != IPPROTO_GRE) {
    flow.src = load_word(skb, nhoff + offsetof(struct iphdr, saddr));
    flow.dst = load_word(skb, nhoff + offsetof(struct iphdr, daddr));
    }
    verlen = load_byte(skb, nhoff + 0/*offsetof(struct iphdr, ihl)*/);
    if (likely(verlen == 0x45))
    nhoff += 20;
    else
    nhoff += (verlen & 0xF) << 2;
    return nhoff;
    }
    static inline __u64 parse_ipv6(struct __sk_buff *skb, __u64 nhoff, __u64 *ip_proto,
    struct flow_key_record *flow)
    {
// ip_proto = load_byte(skb,
    nhoff + offsetof(struct ipv6hdr, nexthdr));
    flow.src = ipv6_addr_hash(skb,
    nhoff + offsetof(struct ipv6hdr, saddr));
    flow.dst = ipv6_addr_hash(skb,
    nhoff + offsetof(struct ipv6hdr, daddr));
    nhoff += sizeof(struct ipv6hdr);
    return nhoff;
    }
    static inline bool flow_dissector(struct __sk_buff *skb,
    struct flow_key_record *flow)
    {
    let mut nhoff: __u64 = ETH_HLEN;
    __u64 ip_proto;
    let mut proto: __u64 = load_half(skb, 12);
    int poff;
    if (proto == ETH_P_8021AD) {
    proto = load_half(skb, nhoff + offsetof(struct vlan_hdr,
    h_vlan_encapsulated_proto));
    nhoff += sizeof(struct vlan_hdr);
    }
    if (proto == ETH_P_8021Q) {
    proto = load_half(skb, nhoff + offsetof(struct vlan_hdr,
    h_vlan_encapsulated_proto));
    nhoff += sizeof(struct vlan_hdr);
    }
    if (likely(proto == ETH_P_IP))
    nhoff = parse_ip(skb, nhoff, &ip_proto, flow);
#[no_mangle]
pub unsafe extern "C" fn if(ETH_P_IPV6: proto ==) -> else {
    else if (proto == ETH_P_IPV6)
    nhoff = parse_ipv6(skb, nhoff, &ip_proto, flow);
    else
    return false;
    switch (ip_proto) {
    case IPPROTO_GRE: {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gre_hdr {
    pub flags: __be16,
    pub proto: __be16,
}

    __u64 gre_flags = load_half(skb,
    nhoff + offsetof(struct gre_hdr, flags));
    __u64 gre_proto = load_half(skb,
    nhoff + offsetof(struct gre_hdr, proto));
    if (gre_flags & (GRE_VERSION|GRE_ROUTING))
    break;
    proto = gre_proto;
    nhoff += 4;
    if (gre_flags & GRE_CSUM)
    nhoff += 4;
    if (gre_flags & GRE_KEY)
    nhoff += 4;
    if (gre_flags & GRE_SEQ)
    nhoff += 4;
    if (proto == ETH_P_8021Q) {
    proto = load_half(skb,
    nhoff + offsetof(struct vlan_hdr,
    h_vlan_encapsulated_proto));
    nhoff += sizeof(struct vlan_hdr);
    }
    if (proto == ETH_P_IP)
    nhoff = parse_ip(skb, nhoff, &ip_proto, flow);
#[no_mangle]
pub unsafe extern "C" fn if(ETH_P_IPV6: proto ==) -> else {
    else if (proto == ETH_P_IPV6)
    nhoff = parse_ipv6(skb, nhoff, &ip_proto, flow);
    else
    return false;
    break;
    }
    case IPPROTO_IPIP:
    nhoff = parse_ip(skb, nhoff, &ip_proto, flow);
    break;
    case IPPROTO_IPV6:
    nhoff = parse_ipv6(skb, nhoff, &ip_proto, flow);
    break;
    default:
    break;
    }
    flow.ip_proto = ip_proto;
    poff = proto_ports_offset(ip_proto);
    if (poff >= 0) {
    nhoff += poff;
    flow.ports = load_word(skb, nhoff);
    }
    flow.thoff = (__u16) nhoff;
    return true;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pair {
    pub packets: c_long,
    pub bytes: c_long,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, __be32);
    __type(value, struct pair);
    __uint(max_entries, 1024);
    } hash_map SEC(".maps");
    SEC("socket2")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog2(skb: *mut __sk_buff) -> c_int {
    int bpf_prog2(struct __sk_buff *skb)
    {
    let mut flow: flow_key_record = {};
    struct pair *value;
    u32 key;
    if (!flow_dissector(skb, &flow))
    return 0;
    key = flow.dst;
    value = bpf_map_lookup_elem(&hash_map, &key);
    if (value) {
    __sync_fetch_and_add(&value.packets, 1);
    __sync_fetch_and_add(&value.bytes, skb.len);
    } else {
    let mut val: pair = {1, skb.len};
    bpf_map_update_elem(&hash_map, &key, &val, BPF_ANY);
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
