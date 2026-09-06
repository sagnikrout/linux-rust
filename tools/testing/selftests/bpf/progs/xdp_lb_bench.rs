//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/xdp_lb_bench.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

pub const IPPROTO_FRAGMENT: c_int = 44;

// jhash helpers
#[no_mangle]
pub unsafe extern "C" fn rol32(word: __u32, shift: c_uint) -> __u32 {
    static inline __u32 rol32(__u32 word, unsigned int shift)
    {
    return (word << shift) | (word >> ((-shift) & 31));
    }

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
#[no_mangle]
pub unsafe extern "C" fn __jhash_nwords(a: __u32, b: __u32, c: __u32, initval: __u32) -> __u32 {
    static inline __u32 __jhash_nwords(__u32 a, __u32 b, __u32 c, __u32 initval)
    {
    a += initval;
    b += initval;
    c += initval;
    __jhash_final(a, b, c);
    return c;
    }
#[no_mangle]
pub unsafe extern "C" fn jhash_2words(a: __u32, b: __u32, initval: __u32) -> __u32 {
    static inline __u32 jhash_2words(__u32 a, __u32 b, __u32 initval)
    {
    return __jhash_nwords(a, b, 0, initval + JHASH_INITVAL + (2 << 2));
    }
#[no_mangle]
pub unsafe extern "C" fn jhash2_4words(k: *const __u32, initval: __u32) -> __u32 {
    static inline __u32 jhash2_4words(const __u32 *k, __u32 initval)
    {
    __u32 a, b, c;
    a = b = c = JHASH_INITVAL + (4 << 2) + initval;
    a += k[0]; b += k[1]; c += k[2];
    __jhash_mix(a, b, c);
    a += k[3];
    __jhash_final(a, b, c);
    return c;
    }
#[no_mangle]
unsafe extern "C" fn ipv4_csum(iph: *mut iphdr) -> __always_inline void {
    static __always_inline void ipv4_csum(struct iphdr *iph)
    {
    __u16 *next_iph = (__u16 *)iph;
    let mut csum: __u32 = 0;
    int i;
    __pragma_loop_unroll_full
    for (i = 0; i < (int)(sizeof(*iph) >> 1); i++)
    csum += *next_iph++;
    csum = (csum & 0xffff) + (csum >> 16);
    csum = (csum & 0xffff) + (csum >> 16);
    iph.check = ~csum;
    }
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 64);
    __type(key, struct vip_definition);
    __type(value, struct vip_meta);
    } vip_map SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lru_inner_map {
    pub BPF_MAP_TYPE_LRU_HASH): __uint(type,,
    pub flow_key): __type(key, struct,
    pub real_pos_lru): __type(value, struct,
    pub DEFAULT_LRU_SIZE): __uint(max_entries,,
    pub SEC(".maps"): } lru_inner,
    struct {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub BENCH_NR_CPUS): __uint(max_entries,,
    pub lru_inner_map): __array(values, struct,
    pub SEC(".maps"): } lru_mapping,
    struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub CH_RINGS_SIZE): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } ch_rings,
    struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub MAX_REALS): __uint(max_entries,,
    pub __u32): __type(key,,
    pub real_definition): __type(value, struct,
    pub SEC(".maps"): } reals,
    struct {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub STATS_SIZE): __uint(max_entries,,
    pub __u32): __type(key,,
    pub lb_stats): __type(value, struct,
    pub SEC(".maps"): } stats,
    struct {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub MAX_REALS): __uint(max_entries,,
    pub __u32): __type(key,,
    pub lb_stats): __type(value, struct,
    pub SEC(".maps"): } reals_stats,
    struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 1): __uint(max_entries,,
    pub __u32): __type(key,,
    pub ctl_value): __type(value, struct,
    pub SEC(".maps"): } ctl_array,
    struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 1): __uint(max_entries,,
    pub __u32): __type(key,,
    pub vip_definition): __type(value, struct,
    pub SEC(".maps"): } vip_miss_stats,
    struct {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub MAX_REALS): __uint(max_entries,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } lru_miss_stats,
    pub flow_mask: volatile __u32,
    pub cold_lru: volatile __u32,
    pub batch_gen: __u32,
//
// old_eth MUST be read BEFORE writing the outer header because
// bpf_xdp_adjust_head makes them overlap.
//
    static __always_inline int encap_v4(struct xdp_md *xdp, __be32 saddr, __be32 daddr,
    __u16 payload_len, const __u8 *dst_mac)
    {
    pub old_eth: *mut *mut ethhdr new_eth,,
    pub data_end: *mut *mut void data,,
    pub iph: *mut iphdr,
    if (bpf_xdp_adjust_head(xdp, -(int)sizeof(struct iphdr)))
    pub -1: return,
    pub )(long)xdp->data: *mut data = (void,
    pub )(long)xdp->data_end: *mut data_end = (void,
    pub data: new_eth =,
    pub ethhdr): iph = data + sizeof(struct,
    pub iphdr): old_eth = data + sizeof(struct,
    if (new_eth + 1 > data_end || old_eth + 1 > data_end || iph + 1 > data_end)
    pub -1: return,
    pub sizeof(new_eth->h_source)): __builtin_memcpy(new_eth->h_source, old_eth->h_dest,,
    pub sizeof(new_eth->h_dest)): __builtin_memcpy(new_eth->h_dest, dst_mac,,
    pub bpf_htons(ETH_P_IP): new_eth->h_proto =,
    pub sizeof(*iph)): *mut __builtin_memset(iph, 0,,
    pub 4: iph->version =,
    pub 2: *mut *mut iph->ihl = sizeof(iph) >>,
    pub IPPROTO_IPIP: iph->protocol =,
    pub sizeof(*iph)): *mut iph->tot_len = bpf_htons(payload_len +,
    pub 64: iph->ttl =,
    pub saddr: iph->saddr =,
    pub daddr: iph->daddr =,
    pub 0: return,
    }
    static __always_inline int encap_v6(struct xdp_md *xdp, const __be32 saddr[4],
    const __be32 daddr[4], __u8 nexthdr, __u16 payload_len,
    const __u8 *dst_mac)
    {
    pub old_eth: *mut *mut ethhdr new_eth,,
    pub data_end: *mut *mut void data,,
    pub ip6h: *mut ipv6hdr,
    if (bpf_xdp_adjust_head(xdp, -(int)sizeof(struct ipv6hdr)))
    pub -1: return,
    pub )(long)xdp->data: *mut data = (void,
    pub )(long)xdp->data_end: *mut data_end = (void,
    pub data: new_eth =,
    pub ethhdr): ip6h = data + sizeof(struct,
    pub ipv6hdr): old_eth = data + sizeof(struct,
    if (new_eth + 1 > data_end || old_eth + 1 > data_end || ip6h + 1 > data_end)
    pub -1: return,
    pub sizeof(new_eth->h_source)): __builtin_memcpy(new_eth->h_source, old_eth->h_dest,,
    pub sizeof(new_eth->h_dest)): __builtin_memcpy(new_eth->h_dest, dst_mac,,
    pub bpf_htons(ETH_P_IPV6): new_eth->h_proto =,
    pub sizeof(*ip6h)): *mut __builtin_memset(ip6h, 0,,
    pub 6: ip6h->version =,
    pub nexthdr: ip6h->nexthdr =,
    pub bpf_htons(payload_len): ip6h->payload_len =,
    pub 64: ip6h->hop_limit =,
    pub sizeof(ip6h->saddr)): __builtin_memcpy(&ip6h->saddr, saddr,,
    pub sizeof(ip6h->daddr)): __builtin_memcpy(&ip6h->daddr, daddr,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn update_stats(map: *mut c_void, key: __u32, bytes: __u16) -> __always_inline void {
    static __always_inline void update_stats(void *map, __u32 key, __u16 bytes)
    {
    pub &key): *mut *mut lb_stats st = bpf_map_lookup_elem(map,,
    if (st) {
    pub 1: st->v1 +=,
    pub bytes: st->v2 +=,
    }
    }
#[no_mangle]
unsafe extern "C" fn count_action(action: c_int) -> __always_inline void {
    static __always_inline void count_action(int action)
    {
    pub st: *mut lb_stats,
    pub key: __u32,
    if (action == XDP_TX)
    pub STATS_XDP_TX: key =,
#[no_mangle]
pub unsafe extern "C" fn if(XDP_PASS: action ==) -> else {
    else if (action == XDP_PASS)
    pub STATS_XDP_PASS: key =,
    else
    pub STATS_XDP_DROP: key =,
    pub &key): st = bpf_map_lookup_elem(&stats,,
    if (st)
    pub 1: st->v1 +=,
    }
#[no_mangle]
unsafe extern "C" fn is_under_flood() -> __always_inline bool {
    static __always_inline bool is_under_flood(void)
    {
    pub STATS_NEW_CONN: __u32 key =,
    pub &key): *mut *mut lb_stats conn_st = bpf_map_lookup_elem(&stats,,
    pub cur_time: __u64,
    if (!conn_st)
    pub true: return,
    pub bpf_ktime_get_ns(): cur_time =,
    if ((cur_time - conn_st.v2) > ONE_SEC) {
    pub 1: conn_st->v1 =,
    pub cur_time: conn_st->v2 =,
    } else {
    pub 1: conn_st->v1 +=,
    if (conn_st.v1 > MAX_CONN_RATE)
    pub true: return,
    }
    pub false: return,
    }
    static __always_inline struct real_definition *connection_table_lookup(void *lru_map,
    struct flow_key *flow,
    __u32 *out_pos)
    {
    pub dst_lru: *mut real_pos_lru,
    pub real: *mut real_definition,
    pub key: __u32,
    pub flow): dst_lru = bpf_map_lookup_elem(lru_map,,
    if (!dst_lru)
    pub NULL: return,
// UDP connections use atime-based timeout instead of FIN/RST
    if (flow.proto == IPPROTO_UDP) {
    pub bpf_ktime_get_ns(): __u64 cur_time =,
    if (cur_time - dst_lru.atime > LRU_UDP_TIMEOUT)
    pub NULL: return,
    pub cur_time: dst_lru->atime =,
    }
    pub dst_lru->pos: key =,
// out_pos = key;
    pub &key): real = bpf_map_lookup_elem(&reals,,
    pub real: return,
    }
    static __always_inline bool get_packet_dst(struct real_definition **real, struct flow_key *flow,
    struct vip_meta *vip_info, bool is_v6, void *lru_map,
    bool is_rst, __u32 *out_pos)
    {
    pub under_flood: bool,
    pub ch_key: __u32 hash,,
    pub ch_val: *mut __u32,
    pub real_pos: __u32,
    pub is_under_flood(): under_flood =,
    if (is_v6) {
    pub MAX_VIPS): *mut *mut __u32 src_hash = jhash2_4words((__u32 )flow->srcv6,,
    pub CH_RING_SIZE): hash = jhash_2words(src_hash, flow->ports,,
    } else {
    pub CH_RING_SIZE): hash = jhash_2words(flow->src, flow->ports,,
    }
    pub CH_RING_SIZE: *mut *mut ch_key = CH_RING_SIZE  vip_info->vip_num + hash %,
    pub &ch_key): ch_val = bpf_map_lookup_elem(&ch_rings,,
    if (!ch_val)
    pub false: return,
    pub ch_val: *mut real_pos =,
// real = bpf_map_lookup_elem(&reals, &real_pos);
    if (!(*real))
    pub false: return,
    if (!(vip_info.flags & F_LRU_BYPASS) && !under_flood && !is_rst) {
    pub }: real_pos_lru new_lru = { .pos = real_pos,
    if (flow.proto == IPPROTO_UDP)
    pub bpf_ktime_get_ns(): new_lru.atime =,
    pub BPF_ANY): bpf_map_update_elem(lru_map, flow, &new_lru,,
    }
// out_pos = real_pos;
    pub true: return,
    }
    static __always_inline void update_vip_lru_miss_stats(struct vip_definition *vip, bool is_v6,
    __u32 real_idx)
    {
    pub miss_vip: *mut vip_definition,
    pub 0: __u32 key =,
    pub cnt: *mut __u32,
    pub &key): miss_vip = bpf_map_lookup_elem(&vip_miss_stats,,
    if (!miss_vip)
    if (is_v6) {
    if (miss_vip.vipv6[0] != vip.vipv6[0] || miss_vip.vipv6[1] != vip.vipv6[1] ||
    miss_vip.vipv6[2] != vip.vipv6[2] || miss_vip.vipv6[3] != vip.vipv6[3])
    } else {
    if (miss_vip.vip != vip.vip)
    }
    if (miss_vip.port != vip.port || miss_vip.proto != vip.proto)
    pub &real_idx): cnt = bpf_map_lookup_elem(&lru_miss_stats,,
    if (cnt)
// cnt += 1;
    }
#[no_mangle]
unsafe extern "C" fn process_packet(xdp: *mut xdp_md) -> __noinline int {
    static __noinline int process_packet(struct xdp_md *xdp)
    {
    pub )(long)xdp->data: *mut *mut void data = (void,
    pub )(long)xdp->data_end: *mut *mut void data_end = (void,
    pub data: *mut *mut ethhdr eth =,
    pub NULL: *mut *mut real_definition dst =,
    pub {}: vip_definition vip_def =,
    pub cval: *mut ctl_value,
    pub {}: flow_key flow =,
    pub vip_info: *mut vip_meta,
    pub data_stats: *mut lb_stats,
    pub uh: *mut udphdr,
    pub tnl_src: [__be32; 4],
    pub lru_map: *mut c_void,
    pub l4: *mut c_void,
    pub payload_len: __u16,
    pub key: __u32 real_pos = 0, cpu_num,,
    pub proto: __u8,
    pub XDP_DROP: int action =,
    pub false: bool is_v6, is_syn = false, is_rst =,
    if (eth + 1 > data_end)
    pub out: goto,
    if (eth.h_proto == bpf_htons(ETH_P_IPV6)) {
    pub true: is_v6 =,
    } else if (eth.h_proto == bpf_htons(ETH_P_IP)) {
    pub false: is_v6 =,
    } else {
    pub XDP_PASS: action =,
    pub out: goto,
    }
    if (is_v6) {
    pub 1): *mut *mut *mut ipv6hdr ip6h = (void )(eth +,
    if (ip6h + 1 > data_end)
    pub out: goto,
    if (ip6h.nexthdr == IPPROTO_FRAGMENT)
    pub out: goto,
    pub bpf_ntohs(ip6h->payload_len): payload_len = sizeof(struct ipv6hdr) +,
    pub ip6h->nexthdr: proto =,
    pub sizeof(flow.srcv6)): __builtin_memcpy(flow.srcv6, &ip6h->saddr,,
    pub sizeof(flow.dstv6)): __builtin_memcpy(flow.dstv6, &ip6h->daddr,,
    pub sizeof(vip_def.vipv6)): __builtin_memcpy(vip_def.vipv6, &ip6h->daddr,,
    pub 1): *mut *mut l4 = (void )(ip6h +,
    } else {
    pub 1): *mut *mut *mut iphdr iph = (void )(eth +,
    if (iph + 1 > data_end)
    pub out: goto,
    if (iph.ihl != 5)
    pub out: goto,
    if (iph.frag_off & bpf_htons(PCKT_FRAGMENTED))
    pub out: goto,
    pub bpf_ntohs(iph->tot_len): payload_len =,
    pub iph->protocol: proto =,
    pub iph->saddr: flow.src =,
    pub iph->daddr: flow.dst =,
    pub iph->daddr: vip_def.vip =,
    pub 1): *mut *mut l4 = (void )(iph +,
    }
// TCP and UDP share the same port layout at offset 0
    if (proto != IPPROTO_TCP && proto != IPPROTO_UDP) {
    pub XDP_PASS: action =,
    pub out: goto,
    }
    pub l4: uh =,
    if ((void *)(uh + 1) > data_end)
    pub out: goto,
    pub uh->source: flow.port16[0] =,
    pub uh->dest: flow.port16[1] =,
    if (proto == IPPROTO_TCP) {
    pub l4: *mut *mut tcphdr th =,
    if ((void *)(th + 1) > data_end)
    pub out: goto,
    pub th->syn: is_syn =,
    pub th->rst: is_rst =,
    }
    pub proto: flow.proto =,
    pub flow.port16[1]: vip_def.port =,
    pub proto: vip_def.proto =,
    pub &vip_def): vip_info = bpf_map_lookup_elem(&vip_map,,
    if (!vip_info) {
    pub XDP_PASS: action =,
    pub out: goto,
    }
    pub STATS_LRU: key =,
    pub &key): data_stats = bpf_map_lookup_elem(&stats,,
    if (!data_stats)
    pub out: goto,
    pub 1: data_stats->v1 +=,
    pub bpf_get_smp_processor_id(): cpu_num =,
    pub &cpu_num): lru_map = bpf_map_lookup_elem(&lru_mapping,,
    if (!lru_map)
    pub out: goto,
    if (!(vip_info.flags & F_LRU_BYPASS) && !is_syn)
    pub &real_pos): dst = connection_table_lookup(lru_map, &flow,,
    if (!dst) {
    if (flow.proto == IPPROTO_TCP) {
    pub miss_st: *mut lb_stats,
    pub STATS_LRU_MISS: key =,
    pub &key): miss_st = bpf_map_lookup_elem(&stats,,
    if (miss_st)
    pub 1: miss_st->v1 +=,
    }
    if (!get_packet_dst(&dst, &flow, vip_info, is_v6, lru_map, is_rst, &real_pos))
    pub out: goto,
    pub real_pos): update_vip_lru_miss_stats(&vip_def, is_v6,,
    pub 1: data_stats->v2 +=,
    }
    pub 0: key =,
    pub &key): cval = bpf_map_lookup_elem(&ctl_array,,
    if (!cval)
    pub out: goto,
    pub payload_len): update_stats(&stats, vip_info->vip_num,,
    pub payload_len): update_stats(&reals_stats, real_pos,,
    if (is_v6) {
    pub tnl_src): create_encap_ipv6_src(flow.port16[0], flow.srcv6[0],,
    if (encap_v6(xdp, tnl_src, dst.dstv6, IPPROTO_IPV6, payload_len, cval.mac))
    pub out: goto,
    } else if (dst.flags & F_IPV6) {
    pub tnl_src): create_encap_ipv6_src(flow.port16[0], flow.src,,
    if (encap_v6(xdp, tnl_src, dst.dstv6, IPPROTO_IPIP, payload_len, cval.mac))
    pub out: goto,
    } else {
    if (encap_v4(xdp, create_encap_ipv4_src(flow.port16[0], flow.src), dst.dst,
    payload_len, cval.mac))
    pub out: goto,
    }
    pub XDP_TX: action =,
    out:
    pub action: return,
    }
#[no_mangle]
unsafe extern "C" fn strip_encap(xdp: *mut xdp_md, saved_eth: *const ethhdr) -> __always_inline int {
    static __always_inline int strip_encap(struct xdp_md *xdp, const struct ethhdr *saved_eth)
    {
    pub )(long)xdp->data: *mut *mut void data = (void,
    pub )(long)xdp->data_end: *mut *mut void data_end = (void,
    pub data: *mut *mut ethhdr eth =,
    pub hdr_sz: c_int,
    if (eth + 1 > data_end)
    pub -1: return,
    hdr_sz = (eth.h_proto == bpf_htons(ETH_P_IPV6)) ? (int)sizeof(struct ipv6hdr)
    pub iphdr): : (int)sizeof(struct,
    if (bpf_xdp_adjust_head(xdp, hdr_sz))
    pub -1: return,
    pub )(long)xdp->data: *mut data = (void,
    pub )(long)xdp->data_end: *mut data_end = (void,
    pub data: eth =,
    if (eth + 1 > data_end)
    pub -1: return,
    pub sizeof(*saved_eth)): *mut __builtin_memcpy(eth, saved_eth,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn randomize_src(xdp: *mut xdp_md, saddr_off: c_int, rand_state: *mut __u32) -> __always_inline void {
    static __always_inline void randomize_src(struct xdp_md *xdp, int saddr_off, __u32 *rand_state)
    {
    pub )(long)xdp->data: *mut *mut void data = (void,
    pub )(long)xdp->data_end: *mut *mut void data_end = (void,
    pub saddr_off: *mut *mut __u32 saddr = data +,
// rand_state ^= *rand_state << 13;
// rand_state ^= *rand_state >> 17;
// rand_state ^= *rand_state << 5;
    if ((void *)(saddr + 1) <= data_end)
// saddr = *rand_state & flow_mask;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_lb_bench(xdp: *mut xdp_md) -> c_int {
    int xdp_lb_bench(struct xdp_md *xdp)
    {
    pub )(long)xdp->data: *mut *mut void data = (void,
    pub )(long)xdp->data_end: *mut *mut void data_end = (void,
    pub data: *mut *mut ethhdr eth =,
    pub saved_eth: ethhdr,
    pub 0: __u32 rand_state =,
    pub 0: __u32 batch_hash =,
    pub 0: int saddr_off =,
    pub is_v6: bool,
    if (eth + 1 > data_end)
    pub XDP_DROP: return,
    pub sizeof(saved_eth)): __builtin_memcpy(&saved_eth, eth,,
    pub bpf_htons(ETH_P_IPV6)): is_v6 = (saved_eth.h_proto ==,
    saddr_off = sizeof(struct ethhdr) + (is_v6 ? offsetof(struct ipv6hdr, saddr) :
    pub saddr)): offsetof(struct iphdr,,
    if (flow_mask)
    pub 1: rand_state = bpf_get_prandom_u32() |,
    if (cold_lru) {
    pub saddr_off: *mut *mut __u32 saddr = data +,
    pub KNUTH_HASH_MULT: *mut *mut batch_hash = (batch_gen + bpf_get_smp_processor_id()),
    if ((void *)(saddr + 1) <= data_end)
// saddr ^= batch_hash;
    }
    return BENCH_BPF_LOOP(
    process_packet(xdp),
    ({
    if (__bench_result == XDP_TX) {
    if (strip_encap(xdp, &saved_eth))
    pub XDP_DROP: return,
    if (rand_state)
    pub &rand_state): randomize_src(xdp, saddr_off,,
    }
    if (cold_lru) {
    pub )(long)xdp->data: *mut *mut void d = (void,
    pub )(long)xdp->data_end: *mut *mut void de = (void,
    pub saddr_off: *mut *mut __u32 __sa = d +,
    if ((void *)(__sa + 1) <= de)
// __sa ^= batch_hash;
    }
    })
    }
    pub "GPL": char _license[] SEC("license") =,
