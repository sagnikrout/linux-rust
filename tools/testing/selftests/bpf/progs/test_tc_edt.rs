//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tc_edt.c
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

// the maximum delay we are willing to add (drop packets beyond that)

pub const NS_PER_SEC: c_int = 1000000000;
pub const ECN_HORIZON_NS: c_int = 5000000;
// flow_key => last_tstamp timestamp used
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, uint32_t);
    __type(value, uint64_t);
    __uint(max_entries, 1);
    } flow_map SEC(".maps");
    __uint64_t target_rate;
#[no_mangle]
pub unsafe extern "C" fn throttle_flow(skb: *mut __sk_buff) -> c_int {
    static inline int throttle_flow(struct __sk_buff *skb)
    {
    let mut key: c_int = 0;
    uint64_t *last_tstamp = bpf_map_lookup_elem(&flow_map, &key);
    let mut delay_ns: u64 = ((uint64_t)skb.len) * NS_PER_SEC / target_rate;
    let mut now: u64 = bpf_ktime_get_ns();
    uint64_t tstamp, next_tstamp = 0;
    if (last_tstamp)
    next_tstamp = *last_tstamp + delay_ns;
    tstamp = skb.tstamp;
    if (tstamp < now)
    tstamp = now;
// should we throttle?
    if (next_tstamp <= tstamp) {
    if (bpf_map_update_elem(&flow_map, &key, &tstamp, BPF_ANY))
    return TC_ACT_SHOT;
    return TC_ACT_OK;
    }
// do not queue past the time horizon
    if (next_tstamp - now >= TIME_HORIZON_NS)
    return TC_ACT_SHOT;
// set ecn bit, if needed
    if (next_tstamp - now >= ECN_HORIZON_NS)
    bpf_skb_ecn_set_ce(skb);
    if (bpf_map_update_elem(&flow_map, &key, &next_tstamp, BPF_EXIST))
    return TC_ACT_SHOT;
    skb.tstamp = next_tstamp;
    return TC_ACT_OK;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_tcp(skb: *mut __sk_buff, tcp: *mut tcphdr) -> c_int {
    static inline int handle_tcp(struct __sk_buff *skb, struct tcphdr *tcp)
    {
    void *data_end = (void *)(long)skb.data_end;
// drop malformed packets
    if ((void *)(tcp + 1) > data_end)
    return TC_ACT_SHOT;
    if (tcp.source == bpf_htons(9000))
    return throttle_flow(skb);
    return TC_ACT_OK;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_ipv4(skb: *mut __sk_buff) -> c_int {
    static inline int handle_ipv4(struct __sk_buff *skb)
    {
    void *data_end = (void *)(long)skb.data_end;
    void *data = (void *)(long)skb.data;
    struct iphdr *iph;
    uint32_t ihl;
// drop malformed packets
    if (data + sizeof(struct ethhdr) > data_end)
    return TC_ACT_SHOT;
    iph = (struct iphdr *)(data + sizeof(struct ethhdr));
    if ((void *)(iph + 1) > data_end)
    return TC_ACT_SHOT;
    ihl = iph.ihl * 4;
    if (((void *)iph) + ihl > data_end)
    return TC_ACT_SHOT;
    if (iph.protocol == IPPROTO_TCP)
    return handle_tcp(skb, (struct tcphdr *)(((void *)iph) + ihl));
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_prog(skb: *mut __sk_buff) -> c_int {
    int tc_prog(struct __sk_buff *skb)
    {
    if (skb.protocol == bpf_htons(ETH_P_IP))
    return handle_ipv4(skb);
    return TC_ACT_OK;
    }
    char __license[] SEC("license") = "GPL";
