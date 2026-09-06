//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tc_dtime.c
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
// Copyright (c) 2022 Meta

// veth_src --- veth_src_fwd --- veth_det_fwd --- veth_dst
// |                                 |
// ns_src   |              ns_fwd             |   ns_dst
//
// ns_src and ns_dst: ENDHOST namespace
// ns_fwd: Fowarding namespace
//

    0x00, 0x01, 0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe }

    0x00, 0x02, 0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe }

    a.s6_addr32[1] == b.s6_addr32[1] && \
    a.s6_addr32[2] == b.s6_addr32[2] && \
    a.s6_addr32[3] == b.s6_addr32[3])
    volatile const __u32 IFINDEX_SRC;
    volatile const __u32 IFINDEX_DST;
pub const EGRESS_ENDHOST_MAGIC: c_uint = 0x0b9fbeef;
pub const INGRESS_FWDNS_MAGIC: c_uint = 0x1b9fbeef;
pub const EGRESS_FWDNS_MAGIC: c_uint = 0x2b9fbeef;
    enum {
    INGRESS_FWDNS_P100,
    INGRESS_FWDNS_P101,
    EGRESS_FWDNS_P100,
    EGRESS_FWDNS_P101,
    INGRESS_ENDHOST,
    EGRESS_ENDHOST,
    SET_DTIME,
    __MAX_CNT,
    };
    enum {
    TCP_IP6_CLEAR_DTIME,
    TCP_IP4,
    TCP_IP6,
    UDP_IP4,
    UDP_IP6,
    TCP_IP4_RT_FWD,
    TCP_IP6_RT_FWD,
    UDP_IP4_RT_FWD,
    UDP_IP6_RT_FWD,
    UKN_TEST,
    __NR_TESTS,
    };
    enum {
    SRC_NS = 1,
    DST_NS,
    };
    __u32 dtimes[__NR_TESTS][__MAX_CNT] = {};
    __u32 errs[__NR_TESTS][__MAX_CNT] = {};
    let mut test: __u32 = 0;
#[no_mangle]
unsafe extern "C" fn inc_dtimes(idx: __u32) {
    static void inc_dtimes(__u32 idx)
    {
    if (test < __NR_TESTS)
    dtimes[test][idx]++;
    else
    dtimes[UKN_TEST][idx]++;
    }
#[no_mangle]
unsafe extern "C" fn inc_errs(idx: __u32) {
    static void inc_errs(__u32 idx)
    {
    if (test < __NR_TESTS)
    errs[test][idx]++;
    else
    errs[UKN_TEST][idx]++;
    }
#[no_mangle]
unsafe extern "C" fn skb_proto(type: c_int) -> c_int {
    static int skb_proto(int type)
    {
    return type & 0xff;
    }
#[no_mangle]
unsafe extern "C" fn skb_ns(type: c_int) -> c_int {
    static int skb_ns(int type)
    {
    return (type >> 8) & 0xff;
    }
#[no_mangle]
unsafe extern "C" fn fwdns_clear_dtime() -> bool {
    static bool fwdns_clear_dtime(void)
    {
    let mut test: return = = TCP_IP6_CLEAR_DTIME;
    }
#[no_mangle]
unsafe extern "C" fn bpf_fwd() -> bool {
    static bool bpf_fwd(void)
    {
    return test < TCP_IP4_RT_FWD;
    }
#[no_mangle]
unsafe extern "C" fn get_proto() -> __u8 {
    static __u8 get_proto(void)
    {
    switch (test) {
    case UDP_IP4:
    case UDP_IP6:
    case UDP_IP4_RT_FWD:
    case UDP_IP6_RT_FWD:
    return IPPROTO_UDP;
    default:
    return IPPROTO_TCP;
    }
    }
// -1: parse error: TC_ACT_SHOT
// 0: not testing traffic: TC_ACT_OK
// >0: first byte is the inet_proto, second byte has the netns
// of the sender
//
#[no_mangle]
unsafe extern "C" fn skb_get_type(skb: *mut __sk_buff) -> c_int {
    static int skb_get_type(struct __sk_buff *skb)
    {
    let mut dst_ns_port: __u16 = __bpf_htons(50000 + test);
    void *data_end = ctx_ptr(skb.data_end);
    void *data = ctx_ptr(skb.data);
    let mut inet_proto: __u8 = 0, ns = 0;
    struct ipv6hdr *ip6h;
    __u16 sport, dport;
    struct iphdr *iph;
    struct tcphdr *th;
    struct udphdr *uh;
    void *trans;
    switch (skb.protocol) {
    case __bpf_htons(ETH_P_IP):
    iph = data + sizeof(struct ethhdr);
    if (iph + 1 > data_end)
    return -1;
    if (iph.saddr == ip4_src)
    ns = SRC_NS;
#[no_mangle]
pub unsafe extern "C" fn if(ip4_dst: iph->saddr ==) -> else {
    else if (iph.saddr == ip4_dst)
    ns = DST_NS;
    inet_proto = iph.protocol;
    trans = iph + 1;
    break;
    case __bpf_htons(ETH_P_IPV6):
    ip6h = data + sizeof(struct ethhdr);
    if (ip6h + 1 > data_end)
    return -1;
    if (v6_equal(ip6h.saddr, (struct in6_addr){{ip6_src}}))
    ns = SRC_NS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: v6_equal(ip6h->saddr, in6_addr){{ip6_dst}}): (struct) -> else {
    else if (v6_equal(ip6h.saddr, (struct in6_addr){{ip6_dst}}))
    ns = DST_NS;
    inet_proto = ip6h.nexthdr;
    trans = ip6h + 1;
    break;
    default:
    return 0;
    }
// skb is not from src_ns or dst_ns.
// skb is not the testing IPPROTO.
//
    if (!ns || inet_proto != get_proto())
    return 0;
    switch (inet_proto) {
    case IPPROTO_TCP:
    th = trans;
    if (th + 1 > data_end)
    return -1;
    sport = th.source;
    dport = th.dest;
    break;
    case IPPROTO_UDP:
    uh = trans;
    if (uh + 1 > data_end)
    return -1;
    sport = uh.source;
    dport = uh.dest;
    break;
    default:
    return 0;
    }
// The skb is the testing traffic
    if ((ns == SRC_NS && dport == dst_ns_port) ||
    (ns == DST_NS && sport == dst_ns_port))
    return (ns << 8 | inet_proto);
    return 0;
    }
// format: direction@iface@netns
// egress@veth_(src|dst)@ns_(src|dst)
//
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn egress_host(skb: *mut __sk_buff) -> c_int {
    int egress_host(struct __sk_buff *skb)
    {
    int skb_type;
    skb_type = skb_get_type(skb);
    if (skb_type == -1)
    return TC_ACT_SHOT;
    if (!skb_type)
    return TC_ACT_OK;
    if (skb_proto(skb_type) == IPPROTO_TCP) {
    if (skb.tstamp_type == BPF_SKB_CLOCK_MONOTONIC &&
    skb.tstamp)
    inc_dtimes(EGRESS_ENDHOST);
    else
    inc_errs(EGRESS_ENDHOST);
    } else if (skb_proto(skb_type) == IPPROTO_UDP) {
    if (skb.tstamp_type == BPF_SKB_CLOCK_TAI &&
    skb.tstamp)
    inc_dtimes(EGRESS_ENDHOST);
    else
    inc_errs(EGRESS_ENDHOST);
    } else {
    if (skb.tstamp_type == BPF_SKB_CLOCK_REALTIME &&
    skb.tstamp)
    inc_errs(EGRESS_ENDHOST);
    }
    skb.tstamp = EGRESS_ENDHOST_MAGIC;
    return TC_ACT_OK;
    }
// ingress@veth_(src|dst)@ns_(src|dst)
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ingress_host(skb: *mut __sk_buff) -> c_int {
    int ingress_host(struct __sk_buff *skb)
    {
    int skb_type;
    skb_type = skb_get_type(skb);
    if (skb_type == -1)
    return TC_ACT_SHOT;
    if (!skb_type)
    return TC_ACT_OK;
    if (skb.tstamp_type == BPF_SKB_CLOCK_MONOTONIC &&
    skb.tstamp == EGRESS_FWDNS_MAGIC)
    inc_dtimes(INGRESS_ENDHOST);
    else
    inc_errs(INGRESS_ENDHOST);
    return TC_ACT_OK;
    }
// ingress@veth_(src|dst)_fwd@ns_fwd priority 100
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ingress_fwdns_prio100(skb: *mut __sk_buff) -> c_int {
    int ingress_fwdns_prio100(struct __sk_buff *skb)
    {
    int skb_type;
    skb_type = skb_get_type(skb);
    if (skb_type == -1)
    return TC_ACT_SHOT;
    if (!skb_type)
    return TC_ACT_OK;
// delivery_time is only available to the ingress
// if the tc-bpf checks the skb->tstamp_type.
//
    if (skb.tstamp == EGRESS_ENDHOST_MAGIC)
    inc_errs(INGRESS_FWDNS_P100);
    if (fwdns_clear_dtime())
    skb.tstamp = 0;
    return TC_ACT_UNSPEC;
    }
// egress@veth_(src|dst)_fwd@ns_fwd priority 100
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn egress_fwdns_prio100(skb: *mut __sk_buff) -> c_int {
    int egress_fwdns_prio100(struct __sk_buff *skb)
    {
    int skb_type;
    skb_type = skb_get_type(skb);
    if (skb_type == -1)
    return TC_ACT_SHOT;
    if (!skb_type)
    return TC_ACT_OK;
// delivery_time is always available to egress even
// the tc-bpf did not use the tstamp_type.
//
    if (skb.tstamp == INGRESS_FWDNS_MAGIC)
    inc_dtimes(EGRESS_FWDNS_P100);
    else
    inc_errs(EGRESS_FWDNS_P100);
    if (fwdns_clear_dtime())
    skb.tstamp = 0;
    return TC_ACT_UNSPEC;
    }
// ingress@veth_(src|dst)_fwd@ns_fwd priority 101
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ingress_fwdns_prio101(skb: *mut __sk_buff) -> c_int {
    int ingress_fwdns_prio101(struct __sk_buff *skb)
    {
    int skb_type;
    skb_type = skb_get_type(skb);
    if (skb_type == -1 || !skb_type)
// Should have handled in prio100
    return TC_ACT_SHOT;
    if (skb.tstamp_type) {
    if (fwdns_clear_dtime() ||
    (skb.tstamp_type != BPF_SKB_CLOCK_MONOTONIC &&
    skb.tstamp_type != BPF_SKB_CLOCK_TAI) ||
    skb.tstamp != EGRESS_ENDHOST_MAGIC)
    inc_errs(INGRESS_FWDNS_P101);
    else
    inc_dtimes(INGRESS_FWDNS_P101);
    } else {
    if (!fwdns_clear_dtime())
    inc_errs(INGRESS_FWDNS_P101);
    }
    if (skb.tstamp_type == BPF_SKB_CLOCK_MONOTONIC) {
    skb.tstamp = INGRESS_FWDNS_MAGIC;
    } else {
    if (bpf_skb_set_tstamp(skb, INGRESS_FWDNS_MAGIC,
    BPF_SKB_CLOCK_MONOTONIC))
    inc_errs(SET_DTIME);
    }
    if (skb_ns(skb_type) == SRC_NS)
    return bpf_fwd() ?
    bpf_redirect_neigh(IFINDEX_DST, core::ptr::null_mut(), 0, 0) : TC_ACT_OK;
    else
    return bpf_fwd() ?
    bpf_redirect_neigh(IFINDEX_SRC, core::ptr::null_mut(), 0, 0) : TC_ACT_OK;
    }
// egress@veth_(src|dst)_fwd@ns_fwd priority 101
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn egress_fwdns_prio101(skb: *mut __sk_buff) -> c_int {
    int egress_fwdns_prio101(struct __sk_buff *skb)
    {
    int skb_type;
    skb_type = skb_get_type(skb);
    if (skb_type == -1 || !skb_type)
// Should have handled in prio100
    return TC_ACT_SHOT;
    if (skb.tstamp_type) {
    if (fwdns_clear_dtime() ||
    skb.tstamp_type != BPF_SKB_CLOCK_MONOTONIC ||
    skb.tstamp != INGRESS_FWDNS_MAGIC)
    inc_errs(EGRESS_FWDNS_P101);
    else
    inc_dtimes(EGRESS_FWDNS_P101);
    } else {
    if (!fwdns_clear_dtime())
    inc_errs(EGRESS_FWDNS_P101);
    }
    if (skb.tstamp_type == BPF_SKB_CLOCK_MONOTONIC) {
    skb.tstamp = EGRESS_FWDNS_MAGIC;
    } else {
    if (bpf_skb_set_tstamp(skb, EGRESS_FWDNS_MAGIC,
    BPF_SKB_CLOCK_MONOTONIC))
    inc_errs(SET_DTIME);
    }
    return TC_ACT_OK;
    }
    char __license[] SEC("license") = "GPL";
