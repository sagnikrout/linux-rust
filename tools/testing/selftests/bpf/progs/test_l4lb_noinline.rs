//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_l4lb_noinline.c
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
// Copyright (c) 2017 Facebook

#[no_mangle]
unsafe extern "C" fn rol32(word: __u32, shift: c_uint) -> __always_inline __u32 {
    static __always_inline __u32 rol32(__u32 word, unsigned int shift)
    {
    return (word << shift) | (word >> ((-shift) & 31));
    }
// copy paste of jhash from kernel sources to make sure llvm
// can compile it into valid sequence of bpf instructions
//

    {						\
    a -= c;  a ^= rol32(c, 4);  c += b;	\
    b -= a;  b ^= rol32(a, 6);  a += c;	\
    c -= b;  c ^= rol32(b, 8);  b += a;	\
    a -= c;  a ^= rol32(c, 16); c += b;	\
    b -= a;  b ^= rol32(a, 19); a += c;	\
    c -= b;  c ^= rol32(b, 4);  b += a;	\
    }

    {						\
    c ^= b; c -= rol32(b, 14);		\
    a ^= c; a -= rol32(c, 11);		\
    b ^= a; b -= rol32(a, 25);		\
    c ^= b; c -= rol32(b, 16);		\
    a ^= c; a -= rol32(c, 4);		\
    b ^= a; b -= rol32(a, 14);		\
    c ^= b; c -= rol32(b, 24);		\
    }
pub const JHASH_INITVAL: c_uint = 0xdeadbeef;
    typedef unsigned int u32;
#[no_mangle]
unsafe extern "C" fn jhash(key: *const c_void, length: u32, initval: u32) -> __noinline u32 {
    static __noinline u32 jhash(const void *key, u32 length, u32 initval)
    {
    u32 a, b, c;
    const unsigned char *k = key;
    a = b = c = JHASH_INITVAL + length + initval;
    while (length > 12) {
    a += *(u32 *)(k);
    b += *(u32 *)(k + 4);
    c += *(u32 *)(k + 8);
    __jhash_mix(a, b, c);
    length -= 12;
    k += 12;
    }
    switch (length) {
    case 12: c += (u32)k[11]<<24;
    case 11: c += (u32)k[10]<<16;
    case 10: c += (u32)k[9]<<8;
    case 9:  c += k[8];
    case 8:  b += (u32)k[7]<<24;
    case 7:  b += (u32)k[6]<<16;
    case 6:  b += (u32)k[5]<<8;
    case 5:  b += k[4];
    case 4:  a += (u32)k[3]<<24;
    case 3:  a += (u32)k[2]<<16;
    case 2:  a += (u32)k[1]<<8;
    case 1:  a += k[0];
    __jhash_final(a, b, c);
    case 0: /* Nothing left to add */
    break;
    }
    return c;
    }
#[no_mangle]
unsafe extern "C" fn __jhash_nwords(a: u32, b: u32, c: u32, initval: u32) -> __noinline u32 {
    static __noinline u32 __jhash_nwords(u32 a, u32 b, u32 c, u32 initval)
    {
    a += initval;
    b += initval;
    c += initval;
    __jhash_final(a, b, c);
    return c;
    }
#[no_mangle]
unsafe extern "C" fn jhash_2words(a: u32, b: u32, initval: u32) -> __noinline u32 {
    static __noinline u32 jhash_2words(u32 a, u32 b, u32 initval)
    {
    return __jhash_nwords(a, b, 0, initval + JHASH_INITVAL + (2 << 2));
    }
pub const PCKT_FRAGMENTED: c_int = 65343;
pub const IPV4_HDR_LEN_NO_OPT: c_int = 20;
pub const IPV4_PLUS_ICMP_HDR: c_int = 28;
pub const IPV6_PLUS_ICMP_HDR: c_int = 48;
pub const RING_SIZE: c_int = 2;
pub const MAX_VIPS: c_int = 12;
pub const MAX_REALS: c_int = 5;
pub const CTL_MAP_SIZE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_description {
    union {
    pub src: __be32,
    pub srcv6: [__be32; 4],
}

    union {
    __be32 dst;
    __be32 dstv6[4];
    };
    union {
    __u32 ports;
    __u16 port16[2];
    };
    __u8 proto;
    __u8 flags;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_value {
    union {
    pub value: __u64,
    pub ifindex: __u32,
    pub mac: [__u8; 6],
}

    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_meta {
    pub flags: __u32,
    pub vip_num: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct real_definition {
    union {
    pub dst: __be32,
    pub dstv6: [__be32; 4],
}

    __u8 flags;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_stats {
    pub bytes: __u64,
    pub pkts: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_hdr {
    pub eth_dest: [c_uchar; ETH_ALEN],
    pub eth_source: [c_uchar; ETH_ALEN],
    pub eth_proto: c_ushort,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, MAX_VIPS);
    __type(key, struct vip);
    __type(value, struct vip_meta);
    } vip_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, CH_RINGS_SIZE);
    __type(key, __u32);
    __type(value, __u32);
    } ch_rings SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, MAX_REALS);
    __type(key, __u32);
    __type(value, struct real_definition);
    } reals SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, MAX_VIPS);
    __type(key, __u32);
    __type(value, struct vip_stats);
    } stats SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, CTL_MAP_SIZE);
    __type(key, __u32);
    __type(value, struct ctl_value);
    } ctl_array SEC(".maps");
#[no_mangle]
unsafe extern "C" fn get_packet_hash(pckt: *mut packet_description, ipv6: bool) -> __noinline __u32 {
    static __noinline __u32 get_packet_hash(struct packet_description *pckt, bool ipv6)
    {
    if (ipv6)
    return jhash_2words(jhash(pckt.srcv6, 16, MAX_VIPS),
    pckt.ports, CH_RINGS_SIZE);
    else
    return jhash_2words(pckt.src, pckt.ports, CH_RINGS_SIZE);
    }
    static __noinline bool get_packet_dst(struct real_definition **real,
    struct packet_description *pckt,
    struct vip_meta *vip_info,
    bool is_ipv6)
    {
    let mut hash: __u32 = get_packet_hash(pckt, is_ipv6);
    let mut key: __u32 = RING_SIZE * vip_info.vip_num + hash % RING_SIZE;
    __u32 *real_pos;
    if (hash != 0x358459b7 /* jhash of ipv4 packet */  &&
    hash != 0x2f4bc6bb /* jhash of ipv6 packet */)
    return false;
    real_pos = bpf_map_lookup_elem(&ch_rings, &key);
    if (!real_pos)
    return false;
    key = *real_pos;
// real = bpf_map_lookup_elem(&reals, &key);
    if (!(*real))
    return false;
    return true;
    }
    static __noinline int parse_icmpv6(void *data, void *data_end, __u64 off,
    struct packet_description *pckt)
    {
    struct icmp6hdr *icmp_hdr;
    struct ipv6hdr *ip6h;
    icmp_hdr = data + off;
    if (icmp_hdr + 1 > data_end)
    return TC_ACT_SHOT;
    if (icmp_hdr.icmp6_type != ICMPV6_PKT_TOOBIG)
    return TC_ACT_OK;
    off += sizeof(struct icmp6hdr);
    ip6h = data + off;
    if (ip6h + 1 > data_end)
    return TC_ACT_SHOT;
    pckt.proto = ip6h.nexthdr;
    pckt.flags |= F_ICMP;
    memcpy(pckt.srcv6, ip6h.daddr.s6_addr32, 16);
    memcpy(pckt.dstv6, ip6h.saddr.s6_addr32, 16);
    return TC_ACT_UNSPEC;
    }
    static __noinline int parse_icmp(void *data, void *data_end, __u64 off,
    struct packet_description *pckt)
    {
    struct icmphdr *icmp_hdr;
    struct iphdr *iph;
    icmp_hdr = data + off;
    if (icmp_hdr + 1 > data_end)
    return TC_ACT_SHOT;
    if (icmp_hdr.type != ICMP_DEST_UNREACH ||
    icmp_hdr.code != ICMP_FRAG_NEEDED)
    return TC_ACT_OK;
    off += sizeof(struct icmphdr);
    iph = data + off;
    if (iph + 1 > data_end)
    return TC_ACT_SHOT;
    if (iph.ihl != 5)
    return TC_ACT_SHOT;
    pckt.proto = iph.protocol;
    pckt.flags |= F_ICMP;
    pckt.src = iph.daddr;
    pckt.dst = iph.saddr;
    return TC_ACT_UNSPEC;
    }
    static __noinline bool parse_udp(void *data, __u64 off, void *data_end,
    struct packet_description *pckt)
    {
    struct udphdr *udp;
    udp = data + off;
    if (udp + 1 > data_end)
    return false;
    if (!(pckt.flags & F_ICMP)) {
    pckt.port16[0] = udp.source;
    pckt.port16[1] = udp.dest;
    } else {
    pckt.port16[0] = udp.dest;
    pckt.port16[1] = udp.source;
    }
    return true;
    }
    static __noinline bool parse_tcp(void *data, __u64 off, void *data_end,
    struct packet_description *pckt)
    {
    struct tcphdr *tcp;
    tcp = data + off;
    if (tcp + 1 > data_end)
    return false;
    if (tcp.syn)
    pckt.flags |= F_SYN_SET;
    if (!(pckt.flags & F_ICMP)) {
    pckt.port16[0] = tcp.source;
    pckt.port16[1] = tcp.dest;
    } else {
    pckt.port16[0] = tcp.dest;
    pckt.port16[1] = tcp.source;
    }
    return true;
    }
    static __noinline int process_packet(void *data, __u64 off, void *data_end,
    bool is_ipv6, struct __sk_buff *skb)
    {
    void *pkt_start = (void *)(long)skb.data;
    let mut pckt: packet_description = {};
    struct eth_hdr *eth = pkt_start;
    let mut tkey: bpf_tunnel_key = {};
    struct vip_stats *data_stats;
    struct real_definition *dst;
    struct vip_meta *vip_info;
    struct ctl_value *cval;
    let mut v4_intf_pos: __u32 = 1;
    let mut v6_intf_pos: __u32 = 2;
    struct ipv6hdr *ip6h;
    let mut vip: vip = {};
    struct iphdr *iph;
    let mut tun_flag: c_int = 0;
    __u16 pkt_bytes;
    __u64 iph_len;
    __u32 ifindex;
    __u8 protocol;
    __u32 vip_num;
    int action;
    tkey.tunnel_ttl = 64;
    if (is_ipv6) {
    ip6h = data + off;
    if (ip6h + 1 > data_end)
    return TC_ACT_SHOT;
    iph_len = sizeof(struct ipv6hdr);
    protocol = ip6h.nexthdr;
    pckt.proto = protocol;
    pkt_bytes = bpf_ntohs(ip6h.payload_len);
    off += iph_len;
    if (protocol == IPPROTO_FRAGMENT) {
    return TC_ACT_SHOT;
    } else if (protocol == IPPROTO_ICMPV6) {
    action = parse_icmpv6(data, data_end, off, &pckt);
    if (action >= 0)
    return action;
    off += IPV6_PLUS_ICMP_HDR;
    } else {
    memcpy(pckt.srcv6, ip6h.saddr.s6_addr32, 16);
    memcpy(pckt.dstv6, ip6h.daddr.s6_addr32, 16);
    }
    } else {
    iph = data + off;
    if (iph + 1 > data_end)
    return TC_ACT_SHOT;
    if (iph.ihl != 5)
    return TC_ACT_SHOT;
    protocol = iph.protocol;
    pckt.proto = protocol;
    pkt_bytes = bpf_ntohs(iph.tot_len);
    off += IPV4_HDR_LEN_NO_OPT;
    if (iph.frag_off & PCKT_FRAGMENTED)
    return TC_ACT_SHOT;
    if (protocol == IPPROTO_ICMP) {
    action = parse_icmp(data, data_end, off, &pckt);
    if (action >= 0)
    return action;
    off += IPV4_PLUS_ICMP_HDR;
    } else {
    pckt.src = iph.saddr;
    pckt.dst = iph.daddr;
    }
    }
    protocol = pckt.proto;
    if (protocol == IPPROTO_TCP) {
    if (!parse_tcp(data, off, data_end, &pckt))
    return TC_ACT_SHOT;
    } else if (protocol == IPPROTO_UDP) {
    if (!parse_udp(data, off, data_end, &pckt))
    return TC_ACT_SHOT;
    } else {
    return TC_ACT_SHOT;
    }
    if (is_ipv6)
    memcpy(vip.daddr.v6, pckt.dstv6, 16);
    else
    vip.daddr.v4 = pckt.dst;
    vip.dport = pckt.port16[1];
    vip.protocol = pckt.proto;
    vip_info = bpf_map_lookup_elem(&vip_map, &vip);
    if (!vip_info) {
    vip.dport = 0;
    vip_info = bpf_map_lookup_elem(&vip_map, &vip);
    if (!vip_info)
    return TC_ACT_SHOT;
    pckt.port16[1] = 0;
    }
    if (vip_info.flags & F_HASH_NO_SRC_PORT)
    pckt.port16[0] = 0;
    if (!get_packet_dst(&dst, &pckt, vip_info, is_ipv6))
    return TC_ACT_SHOT;
    if (dst.flags & F_IPV6) {
    cval = bpf_map_lookup_elem(&ctl_array, &v6_intf_pos);
    if (!cval)
    return TC_ACT_SHOT;
    ifindex = cval.ifindex;
    memcpy(tkey.remote_ipv6, dst.dstv6, 16);
    tun_flag = BPF_F_TUNINFO_IPV6;
    } else {
    cval = bpf_map_lookup_elem(&ctl_array, &v4_intf_pos);
    if (!cval)
    return TC_ACT_SHOT;
    ifindex = cval.ifindex;
    tkey.remote_ipv4 = dst.dst;
    }
    vip_num = vip_info.vip_num;
    data_stats = bpf_map_lookup_elem(&stats, &vip_num);
    if (!data_stats)
    return TC_ACT_SHOT;
    data_stats.pkts++;
    data_stats.bytes += pkt_bytes;
    bpf_skb_set_tunnel_key(skb, &tkey, sizeof(tkey), tun_flag);
// (u32 *)eth->eth_dest = tkey.remote_ipv4;
    return bpf_redirect(ifindex, 0);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn balancer_ingress(ctx: *mut __sk_buff) -> c_int {
    int balancer_ingress(struct __sk_buff *ctx)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    struct eth_hdr *eth = data;
    __u32 eth_proto;
    __u32 nh_off;
    nh_off = sizeof(struct eth_hdr);
    if (data + nh_off > data_end)
    return TC_ACT_SHOT;
    eth_proto = eth.eth_proto;
    if (eth_proto == bpf_htons(ETH_P_IP))
    return process_packet(data, nh_off, data_end, false, ctx);
#[no_mangle]
pub unsafe extern "C" fn if(bpf_htons(ETH_P_IPV6): eth_proto ==) -> else {
    else if (eth_proto == bpf_htons(ETH_P_IPV6))
    return process_packet(data, nh_off, data_end, true, ctx);
    else
    return TC_ACT_SHOT;
    }
    char _license[] SEC("license") = "GPL";
