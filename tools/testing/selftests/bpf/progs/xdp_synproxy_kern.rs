//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/xdp_synproxy_kern.c
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
// Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// Macro flag: #define BPF_NO_KFUNC_PROTOTYPES

pub const TC_ACT_OK: c_int = 0;
pub const TC_ACT_SHOT: c_int = 2;

pub const ETH_ALEN: c_int = 6;
pub const ETH_P_IP: c_uint = 0x0800;
pub const ETH_P_IPV6: c_uint = 0x86DD;

pub const IP_MF: c_uint = 0x2000;
pub const IP_OFFSET: c_uint = 0x1fff;
pub const NEXTHDR_TCP: c_int = 6;
pub const TCPOPT_NOP: c_int = 1;
pub const TCPOPT_EOL: c_int = 0;
pub const TCPOPT_MSS: c_int = 2;
pub const TCPOPT_WINDOW: c_int = 3;
pub const TCPOPT_SACK_PERM: c_int = 4;
pub const TCPOPT_TIMESTAMP: c_int = 8;
pub const TCPOLEN_MSS: c_int = 4;
pub const TCPOLEN_WINDOW: c_int = 3;
pub const TCPOLEN_SACK_PERM: c_int = 2;
pub const TCPOLEN_TIMESTAMP: c_int = 10;
pub const TCP_TS_HZ: c_int = 1000;
pub const TS_OPT_WSCALE_MASK: c_uint = 0xf;

pub const TSBITS: c_int = 6;

pub const IPV4_MAXLEN: c_int = 60;
pub const TCP_MAXLEN: c_int = 60;
pub const DEFAULT_MSS4: c_int = 1460;
pub const DEFAULT_MSS6: c_int = 1440;
pub const DEFAULT_WSCALE: c_int = 7;
pub const DEFAULT_TTL: c_int = 64;
pub const MAX_ALLOWED_PORTS: c_int = 8;
pub const MAX_PACKET_OFF: c_uint = 0xffff;

    do { typeof(a) __tmp = (a); (a) = (b); (b) = __tmp; } while (0)

    const struct { type x; } __attribute__((__packed__)) *__pptr = (typeof(__pptr))(ptr); \
    __pptr.x;								\
    })

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, __u32);
    __type(value, __u64);
    __uint(max_entries, 2);
    } values SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, __u32);
    __type(value, __u16);
    __uint(max_entries, MAX_ALLOWED_PORTS);
    } allowed_ports SEC(".maps");
// Some symbols defined in net/netfilter/nf_conntrack_bpf.c are unavailable in
// vmlinux.h if CONFIG_NF_CONNTRACK=m, so they are redefined locally.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ct_opts___local {
    pub netns_id: i32,
    pub error: i32,
    pub l4proto: u8,
    pub dir: u8,
    pub reserved: [u8; 2],
    pub __attribute__((preserve_access_index)): },

    extern struct nf_conn *bpf_xdp_ct_lookup(struct xdp_md *xdp_ctx,
    struct bpf_sock_tuple *bpf_tuple,
    __u32 len_tuple,
    struct bpf_ct_opts___local *opts,
    pub __ksym: __u32 len_opts),
    extern struct nf_conn *bpf_skb_ct_lookup(struct __sk_buff *skb_ctx,
    struct bpf_sock_tuple *bpf_tuple,
    u32 len_tuple,
    struct bpf_ct_opts___local *opts,
    pub __ksym: u32 len_opts),
    pub __ksym: *mut *mut extern void bpf_ct_release(struct nf_conn ct),
#[no_mangle]
unsafe extern "C" fn swap_eth_addr(a: *mut __u8, b: *mut __u8) -> __always_inline void {
    static __always_inline void swap_eth_addr(__u8 *a, __u8 *b)
    {
    pub tmp: [__u8; ETH_ALEN],
    pub ETH_ALEN): __builtin_memcpy(tmp, a,,
    pub ETH_ALEN): __builtin_memcpy(a, b,,
    pub ETH_ALEN): __builtin_memcpy(b, tmp,,
    }
#[no_mangle]
unsafe extern "C" fn csum_fold(csum: __u32) -> __always_inline __u16 {
    static __always_inline __u16 csum_fold(__u32 csum)
    {
    pub 16): csum = (csum & 0xffff) + (csum >>,
    pub 16): csum = (csum & 0xffff) + (csum >>,
    pub (__u16)~csum: return,
    }
    static __always_inline __u16 csum_tcpudp_magic(__be32 saddr, __be32 daddr,
    __u32 len, __u8 proto,
    __u32 csum)
    {
    pub csum: __u64 s =,
    pub (__u32)saddr: s +=,
    pub (__u32)daddr: s +=,

    pub len: s += proto +,

    pub 8: s += (proto + len) <<,

    pub 32): s = (s & 0xffffffff) + (s >>,
    pub 32): s = (s & 0xffffffff) + (s >>,
    pub csum_fold((__u32)s): return,
    }
    static __always_inline __u16 csum_ipv6_magic(const struct in6_addr *saddr,
    const struct in6_addr *daddr,
    __u32 len, __u8 proto, __u32 csum)
    {
    pub csum: __u64 sum =,
    pub i: c_int,
    __pragma_loop_unroll
    pub i++): for (i = 0; i < 4;,
    pub (__u32)saddr->in6_u.u6_addr32[i]: sum +=,
    __pragma_loop_unroll
    pub i++): for (i = 0; i < 4;,
    pub (__u32)daddr->in6_u.u6_addr32[i]: sum +=,
// Don't combine additions to avoid 32-bit overflow.
    pub bpf_htonl(len): sum +=,
    pub bpf_htonl(proto): sum +=,
    pub 32): sum = (sum & 0xffffffff) + (sum >>,
    pub 32): sum = (sum & 0xffffffff) + (sum >>,
    pub csum_fold((__u32)sum): return,
    }
#[no_mangle]
unsafe extern "C" fn tcp_clock_ns() -> __always_inline __u64 {
    static __always_inline __u64 tcp_clock_ns(void)
    {
    pub bpf_ktime_get_ns(): return,
    }
#[no_mangle]
unsafe extern "C" fn tcp_ns_to_ts(ns: __u64) -> __always_inline __u32 {
    static __always_inline __u32 tcp_ns_to_ts(__u64 ns)
    {
    pub TCP_TS_HZ): return ns / (NSEC_PER_SEC /,
    }
#[no_mangle]
unsafe extern "C" fn tcp_clock_ms() -> __always_inline __u32 {
    static __always_inline __u32 tcp_clock_ms(void)
    {
    pub tcp_ns_to_ts(tcp_clock_ns()): return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpopt_context {
    pub data: *mut c_void,
    pub data_end: *mut c_void,
    pub tsecr: *mut __be32,
    pub wscale: __u8,
    pub option_timestamp: bool,
    pub option_sack: bool,
    pub off: __u32,
}

    static __always_inline u8 *next(struct tcpopt_context *ctx, __u32 sz)
    {
    let mut off: __u64 = ctx.off;
    __u8 *data;
// Verifier forbids access to packet when offset exceeds MAX_PACKET_OFF
    if (off > MAX_PACKET_OFF - sz)
    return core::ptr::null_mut();
    data = ctx.data + off;
    barrier_var(data);
    if (data + sz >= ctx.data_end)
    return core::ptr::null_mut();
    ctx.off += sz;
    return data;
    }
#[no_mangle]
unsafe extern "C" fn tscookie_tcpopt_parse(ctx: *mut tcpopt_context) -> c_int {
    static int tscookie_tcpopt_parse(struct tcpopt_context *ctx)
    {
    __u8 *opcode, *opsize, *wscale, *tsecr;
    let mut off: __u32 = ctx.off;
    opcode = next(ctx, 1);
    if (!opcode)
    return 1;
    if (*opcode == TCPOPT_EOL)
    return 1;
    if (*opcode == TCPOPT_NOP)
    return 0;
    opsize = next(ctx, 1);
    if (!opsize || *opsize < 2)
    return 1;
    switch (*opcode) {
    case TCPOPT_WINDOW:
    wscale = next(ctx, 1);
    if (!wscale)
    return 1;
    if (*opsize == TCPOLEN_WINDOW)
    ctx.wscale = *wscale < TCP_MAX_WSCALE ? *wscale : TCP_MAX_WSCALE;
    break;
    case TCPOPT_TIMESTAMP:
    tsecr = next(ctx, 4);
    if (!tsecr)
    return 1;
    if (*opsize == TCPOLEN_TIMESTAMP) {
    ctx.option_timestamp = true;
// Client's tsval becomes our tsecr.
// ctx->tsecr = get_unaligned((__be32 *)tsecr);
    }
    break;
    case TCPOPT_SACK_PERM:
    if (*opsize == TCPOLEN_SACK_PERM)
    ctx.option_sack = true;
    break;
    }
    ctx.off = off + *opsize;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tscookie_tcpopt_parse_batch(index: __u32, context: *mut c_void) -> c_int {
    static int tscookie_tcpopt_parse_batch(__u32 index, void *context)
    {
    int i;
    for (i = 0; i < 7; i++)
    if (tscookie_tcpopt_parse(context))
    return 1;
    return 0;
    }
    static __always_inline bool tscookie_init(struct tcphdr *tcp_header,
    __u16 tcp_len, __be32 *tsval,
    __be32 *tsecr, void *data, void *data_end)
    {
    struct tcpopt_context loop_ctx = {
    .data = data,
    .data_end = data_end,
    .tsecr = tsecr,
    .wscale = TS_OPT_WSCALE_MASK,
    .option_timestamp = false,
    .option_sack = false,
// Note: currently verifier would track .off as unbound scalar.
// In case if verifier would at some point get smarter and
// compute bounded value for this var, beware that it might
// hinder bpf_loop() convergence validation.
//
    .off = (__u8 *)(tcp_header + 1) - (__u8 *)data,
    };
    u32 cookie;
    bpf_loop(6, tscookie_tcpopt_parse_batch, &loop_ctx, 0);
    if (!loop_ctx.option_timestamp)
    return false;
    cookie = tcp_clock_ms() & ~TSMASK;
    cookie |= loop_ctx.wscale & TS_OPT_WSCALE_MASK;
    if (loop_ctx.option_sack)
    cookie |= TS_OPT_SACK;
    if (tcp_header.ece && tcp_header.cwr)
    cookie |= TS_OPT_ECN;
// tsval = bpf_htonl(cookie);
    return true;
    }
    static __always_inline void values_get_tcpipopts(__u16 *mss, __u8 *wscale,
    __u8 *ttl, bool ipv6)
    {
    let mut key: __u32 = 0;
    __u64 *value;
    value = bpf_map_lookup_elem(&values, &key);
    if (value && *value != 0) {
    if (ipv6)
// mss = (*value >> 32) & 0xffff;
    else
// mss = *value & 0xffff;
// wscale = (*value >> 16) & 0xf;
// ttl = (*value >> 24) & 0xff;
    return;
    }
// mss = ipv6 ? DEFAULT_MSS6 : DEFAULT_MSS4;
// wscale = DEFAULT_WSCALE;
// ttl = DEFAULT_TTL;
    }
#[no_mangle]
unsafe extern "C" fn values_inc_synacks() -> __always_inline void {
    static __always_inline void values_inc_synacks(void)
    {
    let mut key: __u32 = 1;
    __u64 *value;
    value = bpf_map_lookup_elem(&values, &key);
    if (value)
    __sync_fetch_and_add(value, 1);
    }
#[no_mangle]
unsafe extern "C" fn check_port_allowed(port: __u16) -> __always_inline bool {
    static __always_inline bool check_port_allowed(__u16 port)
    {
    __u32 i;
    for (i = 0; i < MAX_ALLOWED_PORTS; i++) {
    let mut key: __u32 = i;
    __u16 *value;
    value = bpf_map_lookup_elem(&allowed_ports, &key);
    if (!value)
    break;
// 0 is a terminator value. Check it first to avoid matching on
// a forbidden port == 0 and returning true.
//
    if (*value == 0)
    break;
    if (*value == port)
    return true;
    }
    return false;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct header_pointers {
    pub eth: *mut ethhdr,
    pub ipv4: *mut iphdr,
    pub ipv6: *mut ipv6hdr,
    pub tcp: *mut tcphdr,
    pub tcp_len: __u16,
}

    static __always_inline int tcp_dissect(void *data, void *data_end,
    struct header_pointers *hdr)
    {
    hdr.eth = data;
    if (hdr.eth + 1 > data_end)
    return XDP_DROP;
    switch (bpf_ntohs(hdr.eth.h_proto)) {
    case ETH_P_IP:
    hdr.ipv6 = core::ptr::null_mut();
    hdr.ipv4 = (void *)hdr.eth + sizeof(*hdr.eth);
    if (hdr.ipv4 + 1 > data_end)
    return XDP_DROP;
    if (hdr.ipv4.ihl * 4 < sizeof(*hdr.ipv4))
    return XDP_DROP;
    if (hdr.ipv4.version != 4)
    return XDP_DROP;
    if (hdr.ipv4.protocol != IPPROTO_TCP)
    return XDP_PASS;
    hdr.tcp = (void *)hdr.ipv4 + hdr.ipv4.ihl * 4;
    break;
    case ETH_P_IPV6:
    hdr.ipv4 = core::ptr::null_mut();
    hdr.ipv6 = (void *)hdr.eth + sizeof(*hdr.eth);
    if (hdr.ipv6 + 1 > data_end)
    return XDP_DROP;
    if (hdr.ipv6.version != 6)
    return XDP_DROP;
// XXX: Extension headers are not supported and could circumvent
// XDP SYN flood protection.
//
    if (hdr.ipv6.nexthdr != NEXTHDR_TCP)
    return XDP_PASS;
    hdr.tcp = (void *)hdr.ipv6 + sizeof(*hdr.ipv6);
    break;
    default:
// XXX: VLANs will circumvent XDP SYN flood protection.
    return XDP_PASS;
    }
    if (hdr.tcp + 1 > data_end)
    return XDP_DROP;
    hdr.tcp_len = hdr.tcp.doff * 4;
    if (hdr.tcp_len < sizeof(*hdr.tcp))
    return XDP_DROP;
    return XDP_TX;
    }
#[no_mangle]
unsafe extern "C" fn tcp_lookup(ctx: *mut c_void, hdr: *mut header_pointers, xdp: bool) -> __always_inline int {
    static __always_inline int tcp_lookup(void *ctx, struct header_pointers *hdr, bool xdp)
    {
    struct bpf_ct_opts___local ct_lookup_opts = {
    .netns_id = BPF_F_CURRENT_NETNS,
    .l4proto = IPPROTO_TCP,
    };
    let mut tup: bpf_sock_tuple = {};
    struct nf_conn *ct;
    __u32 tup_size;
    if (hdr.ipv4) {
// TCP doesn't normally use fragments, and XDP can't reassemble
// them.
//
    if ((hdr.ipv4.frag_off & bpf_htons(IP_MF | IP_OFFSET)) != 0)
    return XDP_DROP;
    tup.ipv4.saddr = hdr.ipv4.saddr;
    tup.ipv4.daddr = hdr.ipv4.daddr;
    tup.ipv4.sport = hdr.tcp.source;
    tup.ipv4.dport = hdr.tcp.dest;
    tup_size = sizeof(tup.ipv4);
    } else if (hdr.ipv6) {
    __builtin_memcpy(tup.ipv6.saddr, &hdr.ipv6.saddr, sizeof(tup.ipv6.saddr));
    __builtin_memcpy(tup.ipv6.daddr, &hdr.ipv6.daddr, sizeof(tup.ipv6.daddr));
    tup.ipv6.sport = hdr.tcp.source;
    tup.ipv6.dport = hdr.tcp.dest;
    tup_size = sizeof(tup.ipv6);
    } else {
// The verifier can't track that either ipv4 or ipv6 is not
// NULL.
//
    return XDP_ABORTED;
    }
    if (xdp)
    ct = bpf_xdp_ct_lookup(ctx, &tup, tup_size, &ct_lookup_opts, sizeof(ct_lookup_opts));
    else
    ct = bpf_skb_ct_lookup(ctx, &tup, tup_size, &ct_lookup_opts, sizeof(ct_lookup_opts));
    if (ct) {
    let mut status: c_ulong = ct.status;
    bpf_ct_release(ct);
    if (status & IPS_CONFIRMED)
    return XDP_PASS;
    } else if (ct_lookup_opts.error != -ENOENT) {
    return XDP_ABORTED;
    }
// error == -ENOENT || !(status & IPS_CONFIRMED)
    return XDP_TX;
    }
    static __always_inline __u8 tcp_mkoptions(__be32 *buf, __be32 *tsopt, __u16 mss,
    __u8 wscale)
    {
    __be32 *start = buf;
// buf++ = bpf_htonl((TCPOPT_MSS << 24) | (TCPOLEN_MSS << 16) | mss);
    if (!tsopt)
    return buf - start;
    if (tsopt[0] & bpf_htonl(1 << 4))
// buf++ = bpf_htonl((TCPOPT_SACK_PERM << 24) |
    (TCPOLEN_SACK_PERM << 16) |
    (TCPOPT_TIMESTAMP << 8) |
    TCPOLEN_TIMESTAMP);
    else
// buf++ = bpf_htonl((TCPOPT_NOP << 24) |
    (TCPOPT_NOP << 16) |
    (TCPOPT_TIMESTAMP << 8) |
    TCPOLEN_TIMESTAMP);
// buf++ = tsopt[0];
// buf++ = tsopt[1];
    if ((tsopt[0] & bpf_htonl(0xf)) != bpf_htonl(0xf))
// buf++ = bpf_htonl((TCPOPT_NOP << 24) |
    (TCPOPT_WINDOW << 16) |
    (TCPOLEN_WINDOW << 8) |
    wscale);
    return buf - start;
    }
    static __always_inline void tcp_gen_synack(struct tcphdr *tcp_header,
    __u32 cookie, __be32 *tsopt,
    __u16 mss, __u8 wscale)
    {
    void *tcp_options;
    tcp_flag_word(tcp_header) = TCP_FLAG_SYN | TCP_FLAG_ACK;
    if (tsopt && (tsopt[0] & bpf_htonl(1 << 5)))
    tcp_flag_word(tcp_header) |= TCP_FLAG_ECE;
    tcp_header.doff = 5; /* doff is part of tcp_flag_word. */
    swap(tcp_header.source, tcp_header.dest);
    tcp_header.ack_seq = bpf_htonl(bpf_ntohl(tcp_header.seq) + 1);
    tcp_header.seq = bpf_htonl(cookie);
    tcp_header.window = 0;
    tcp_header.urg_ptr = 0;
    tcp_header.check = 0; /* Calculate checksum later. */
    tcp_options = (void *)(tcp_header + 1);
    tcp_header.doff += tcp_mkoptions(tcp_options, tsopt, mss, wscale);
    }
    static __always_inline void tcpv4_gen_synack(struct header_pointers *hdr,
    __u32 cookie, __be32 *tsopt)
    {
    __u8 wscale;
    __u16 mss;
    __u8 ttl;
    values_get_tcpipopts(&mss, &wscale, &ttl, false);
    swap_eth_addr(hdr.eth.h_source, hdr.eth.h_dest);
    swap(hdr.ipv4.saddr, hdr.ipv4.daddr);
    hdr.ipv4.check = 0; /* Calculate checksum later. */
    hdr.ipv4.tos = 0;
    hdr.ipv4.id = 0;
    hdr.ipv4.ttl = ttl;
    tcp_gen_synack(hdr.tcp, cookie, tsopt, mss, wscale);
    hdr.tcp_len = hdr.tcp.doff * 4;
    hdr.ipv4.tot_len = bpf_htons(sizeof(*hdr.ipv4) + hdr.tcp_len);
    }
    static __always_inline void tcpv6_gen_synack(struct header_pointers *hdr,
    __u32 cookie, __be32 *tsopt)
    {
    __u8 wscale;
    __u16 mss;
    __u8 ttl;
    values_get_tcpipopts(&mss, &wscale, &ttl, true);
    swap_eth_addr(hdr.eth.h_source, hdr.eth.h_dest);
    swap(hdr.ipv6.saddr, hdr.ipv6.daddr);
// (__be32 *)hdr->ipv6 = bpf_htonl(0x60000000);
    hdr.ipv6.hop_limit = ttl;
    tcp_gen_synack(hdr.tcp, cookie, tsopt, mss, wscale);
    hdr.tcp_len = hdr.tcp.doff * 4;
    hdr.ipv6.payload_len = bpf_htons(hdr.tcp_len);
    }
    static __always_inline int syncookie_handle_syn(struct header_pointers *hdr,
    void *ctx,
    void *data, void *data_end,
    bool xdp)
    {
    __u32 old_pkt_size, new_pkt_size;
// Unlike clang 10, clang 11 and 12 generate code that doesn't pass the
// BPF verifier if tsopt is not volatile. Volatile forces it to store
// the pointer value and use it directly, otherwise tcp_mkoptions is
// (mis)compiled like this:
// if (!tsopt)
// return buf - start;
// reg = stored_return_value_of_tscookie_init;
// if (reg)
// tsopt = tsopt_buf;
// else
// tsopt = NULL;
// ...
// *buf++ = tsopt[1];
// It creates a dead branch where tsopt is assigned NULL, but the
// verifier can't prove it's dead and blocks the program.
//
    let mut tsopt: *mut __be32  volatile = core::ptr::null_mut();
    __be32 tsopt_buf[2] = {};
    __u16 ip_len;
    __u32 cookie;
    __s64 value;
// Checksum is not yet verified, but both checksum failure and TCP
// header checks return XDP_DROP, so the order doesn't matter.
//
    if (hdr.tcp.fin || hdr.tcp.rst)
    return XDP_DROP;
// Issue SYN cookies on allowed ports, drop SYN packets on blocked
// ports.
//
    if (!check_port_allowed(bpf_ntohs(hdr.tcp.dest)))
    return XDP_DROP;
    if (hdr.ipv4) {
// Check the IPv4 and TCP checksums before creating a SYNACK.
    value = bpf_csum_diff(0, 0, (void *)hdr.ipv4, hdr.ipv4.ihl * 4, 0);
    if (value < 0)
    return XDP_ABORTED;
    if (csum_fold(value) != 0)
    return XDP_DROP; /* Bad IPv4 checksum. */
    value = bpf_csum_diff(0, 0, (void *)hdr.tcp, hdr.tcp_len, 0);
    if (value < 0)
    return XDP_ABORTED;
    if (csum_tcpudp_magic(hdr.ipv4.saddr, hdr.ipv4.daddr,
    hdr.tcp_len, IPPROTO_TCP, value) != 0)
    return XDP_DROP; /* Bad TCP checksum. */
    ip_len = sizeof(*hdr.ipv4);
    value = bpf_tcp_raw_gen_syncookie_ipv4(hdr.ipv4, hdr.tcp,
    hdr.tcp_len);
    } else if (hdr.ipv6) {
// Check the TCP checksum before creating a SYNACK.
    value = bpf_csum_diff(0, 0, (void *)hdr.tcp, hdr.tcp_len, 0);
    if (value < 0)
    return XDP_ABORTED;
    if (csum_ipv6_magic(&hdr.ipv6.saddr, &hdr.ipv6.daddr,
    hdr.tcp_len, IPPROTO_TCP, value) != 0)
    return XDP_DROP; /* Bad TCP checksum. */
    ip_len = sizeof(*hdr.ipv6);
    value = bpf_tcp_raw_gen_syncookie_ipv6(hdr.ipv6, hdr.tcp,
    hdr.tcp_len);
    } else {
    return XDP_ABORTED;
    }
    if (value < 0)
    return XDP_ABORTED;
    cookie = (__u32)value;
    if (tscookie_init((void *)hdr.tcp, hdr.tcp_len,
    &tsopt_buf[0], &tsopt_buf[1], data, data_end))
    tsopt = tsopt_buf;
// Check that there is enough space for a SYNACK. It also covers
// the check that the destination of the __builtin_memmove below
// doesn't overflow.
//
    if (data + sizeof(*hdr.eth) + ip_len + TCP_MAXLEN > data_end)
    return XDP_ABORTED;
    if (hdr.ipv4) {
    if (hdr.ipv4.ihl * 4 > sizeof(*hdr.ipv4)) {
    struct tcphdr *new_tcp_header;
    new_tcp_header = data + sizeof(*hdr.eth) + sizeof(*hdr.ipv4);
    __builtin_memmove(new_tcp_header, hdr.tcp, sizeof(*hdr.tcp));
    hdr.tcp = new_tcp_header;
    hdr.ipv4.ihl = sizeof(*hdr.ipv4) / 4;
    }
    tcpv4_gen_synack(hdr, cookie, tsopt);
    } else if (hdr.ipv6) {
    tcpv6_gen_synack(hdr, cookie, tsopt);
    } else {
    return XDP_ABORTED;
    }
// Recalculate checksums.
    hdr.tcp.check = 0;
    value = bpf_csum_diff(0, 0, (void *)hdr.tcp, hdr.tcp_len, 0);
    if (value < 0)
    return XDP_ABORTED;
    if (hdr.ipv4) {
    hdr.tcp.check = csum_tcpudp_magic(hdr.ipv4.saddr,
    hdr.ipv4.daddr,
    hdr.tcp_len,
    IPPROTO_TCP,
    value);
    hdr.ipv4.check = 0;
    value = bpf_csum_diff(0, 0, (void *)hdr.ipv4, sizeof(*hdr.ipv4), 0);
    if (value < 0)
    return XDP_ABORTED;
    hdr.ipv4.check = csum_fold(value);
    } else if (hdr.ipv6) {
    hdr.tcp.check = csum_ipv6_magic(&hdr.ipv6.saddr,
    &hdr.ipv6.daddr,
    hdr.tcp_len,
    IPPROTO_TCP,
    value);
    } else {
    return XDP_ABORTED;
    }
// Set the new packet size.
    old_pkt_size = data_end - data;
    new_pkt_size = sizeof(*hdr.eth) + ip_len + hdr.tcp.doff * 4;
    if (xdp) {
    if (bpf_xdp_adjust_tail(ctx, new_pkt_size - old_pkt_size))
    return XDP_ABORTED;
    } else {
    if (bpf_skb_change_tail(ctx, new_pkt_size, 0))
    return XDP_ABORTED;
    }
    values_inc_synacks();
    return XDP_TX;
    }
#[no_mangle]
unsafe extern "C" fn syncookie_handle_ack(hdr: *mut header_pointers) -> __always_inline int {
    static __always_inline int syncookie_handle_ack(struct header_pointers *hdr)
    {
    int err;
    if (hdr.tcp.rst)
    return XDP_DROP;
    if (hdr.ipv4)
    err = bpf_tcp_raw_check_syncookie_ipv4(hdr.ipv4, hdr.tcp);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: hdr->ipv6) -> else {
    else if (hdr.ipv6)
    err = bpf_tcp_raw_check_syncookie_ipv6(hdr.ipv6, hdr.tcp);
    else
    return XDP_ABORTED;
    if (err)
    return XDP_DROP;
    return XDP_PASS;
    }
    static __always_inline int syncookie_part1(void *ctx, void *data, void *data_end,
    struct header_pointers *hdr, bool xdp)
    {
    int ret;
    ret = tcp_dissect(data, data_end, hdr);
    if (ret != XDP_TX)
    return ret;
    ret = tcp_lookup(ctx, hdr, xdp);
    if (ret != XDP_TX)
    return ret;
// Packet is TCP and doesn't belong to an established connection.
    if ((hdr.tcp.syn ^ hdr.tcp.ack) != 1)
    return XDP_DROP;
// Grow the TCP header to TCP_MAXLEN to be able to pass any hdr->tcp_len
// to bpf_tcp_raw_gen_syncookie_ipv{4,6} and pass the verifier.
//
    if (xdp) {
    if (bpf_xdp_adjust_tail(ctx, TCP_MAXLEN - hdr.tcp_len))
    return XDP_ABORTED;
    } else {
// Without volatile the verifier throws this error:
// R9 32-bit pointer arithmetic prohibited
//
    let mut old_len: volatile u64 = data_end - data;
    if (bpf_skb_change_tail(ctx, old_len + TCP_MAXLEN - hdr.tcp_len, 0))
    return XDP_ABORTED;
    }
    return XDP_TX;
    }
    static __always_inline int syncookie_part2(void *ctx, void *data, void *data_end,
    struct header_pointers *hdr, bool xdp)
    {
    if (hdr.ipv4) {
    hdr.eth = data;
    hdr.ipv4 = (void *)hdr.eth + sizeof(*hdr.eth);
// IPV4_MAXLEN is needed when calculating checksum.
// At least sizeof(struct iphdr) is needed here to access ihl.
//
    if ((void *)hdr.ipv4 + IPV4_MAXLEN > data_end)
    return XDP_ABORTED;
    hdr.tcp = (void *)hdr.ipv4 + hdr.ipv4.ihl * 4;
    } else if (hdr.ipv6) {
    hdr.eth = data;
    hdr.ipv6 = (void *)hdr.eth + sizeof(*hdr.eth);
    hdr.tcp = (void *)hdr.ipv6 + sizeof(*hdr.ipv6);
    } else {
    return XDP_ABORTED;
    }
    if ((void *)hdr.tcp + TCP_MAXLEN > data_end)
    return XDP_ABORTED;
// We run out of registers, tcp_len gets spilled to the stack, and the
// verifier forgets its min and max values checked above in tcp_dissect.
//
    hdr.tcp_len = hdr.tcp.doff * 4;
    if (hdr.tcp_len < sizeof(*hdr.tcp))
    return XDP_ABORTED;
    return hdr.tcp.syn ? syncookie_handle_syn(hdr, ctx, data, data_end, xdp) :
    syncookie_handle_ack(hdr);
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn syncookie_xdp(ctx: *mut xdp_md) -> c_int {
    int syncookie_xdp(struct xdp_md *ctx)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    struct header_pointers hdr;
    int ret;
    ret = syncookie_part1(ctx, data, data_end, &hdr, true);
    if (ret != XDP_TX)
    return ret;
    data_end = (void *)(long)ctx.data_end;
    data = (void *)(long)ctx.data;
    return syncookie_part2(ctx, data, data_end, &hdr, true);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn syncookie_tc(skb: *mut __sk_buff) -> c_int {
    int syncookie_tc(struct __sk_buff *skb)
    {
    void *data_end = (void *)(long)skb.data_end;
    void *data = (void *)(long)skb.data;
    struct header_pointers hdr;
    int ret;
    ret = syncookie_part1(skb, data, data_end, &hdr, false);
    if (ret != XDP_TX)
    let mut ret: return = = XDP_PASS ? TC_ACT_OK : TC_ACT_SHOT;
    data_end = (void *)(long)skb.data_end;
    data = (void *)(long)skb.data;
    ret = syncookie_part2(skb, data, data_end, &hdr, false);
    switch (ret) {
    case XDP_PASS:
    return TC_ACT_OK;
    case XDP_TX:
    return bpf_redirect(skb.ifindex, 0);
    default:
    return TC_ACT_SHOT;
    }
    }
    char _license[] SEC("license") = "GPL";
