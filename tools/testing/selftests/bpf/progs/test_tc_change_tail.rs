//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tc_change_tail.c
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

    let mut change_tail_ret: c_long = 1;
    static __always_inline struct iphdr *parse_ip_header(struct __sk_buff *skb, int *ip_proto)
    {
    void *data_end = (void *)(long)skb.data_end;
    void *data = (void *)(long)skb.data;
    struct ethhdr *eth = data;
    struct iphdr *iph;
// Verify Ethernet header
    if ((void *)(data + sizeof(*eth)) > data_end)
    return core::ptr::null_mut();
// Skip Ethernet header to get to IP header
    iph = (void *)(data + sizeof(struct ethhdr));
// Verify IP header
    if ((void *)(data + sizeof(struct ethhdr) + sizeof(*iph)) > data_end)
    return core::ptr::null_mut();
// Basic IP header validation
    if (iph.version != 4)  /* Only support IPv4 */
    return core::ptr::null_mut();
    if (iph.ihl < 5)  /* Minimum IP header length */
    return core::ptr::null_mut();
// ip_proto = iph->protocol;
    return iph;
    }
    static __always_inline struct udphdr *parse_udp_header(struct __sk_buff *skb, struct iphdr *iph)
    {
    void *data_end = (void *)(long)skb.data_end;
    void *hdr = (void *)iph;
    struct udphdr *udp;
// Calculate UDP header position
    udp = hdr + (iph.ihl * 4);
    hdr = (void *)udp;
// Verify UDP header bounds
    if ((void *)(hdr + sizeof(*udp)) > data_end)
    return core::ptr::null_mut();
    return udp;
    }
    SEC("tc/ingress")
#[no_mangle]
pub unsafe extern "C" fn change_tail(skb: *mut __sk_buff) -> c_int {
    int change_tail(struct __sk_buff *skb)
    {
    let mut len: c_int = skb.len;
    struct udphdr *udp;
    struct iphdr *iph;
    void *data_end;
    char *payload;
    int ip_proto;
    bpf_skb_pull_data(skb, len);
    data_end = (void *)(long)skb.data_end;
    iph = parse_ip_header(skb, &ip_proto);
    if (!iph)
    return TCX_PASS;
    if (ip_proto != IPPROTO_UDP)
    return TCX_PASS;
    udp = parse_udp_header(skb, iph);
    if (!udp)
    return TCX_PASS;
    payload = (char *)udp + (sizeof(struct udphdr));
    if (payload + 1 > (char *)data_end)
    return TCX_PASS;
    if (payload[0] == 'T') { /* Trim the packet */
    change_tail_ret = bpf_skb_change_tail(skb, len - 1, 0);
    if (!change_tail_ret)
    bpf_skb_change_tail(skb, len, 0);
    return TCX_PASS;
    } else if (payload[0] == 'G') { /* Grow the packet */
    change_tail_ret = bpf_skb_change_tail(skb, len + 1, 0);
    if (!change_tail_ret)
    bpf_skb_change_tail(skb, len, 0);
    return TCX_PASS;
    } else if (payload[0] == 'E') { /* Error */
    change_tail_ret = bpf_skb_change_tail(skb, BPF_SKB_MAX_LEN, 0);
    return TCX_PASS;
    } else if (payload[0] == 'Z') { /* Zero */
    change_tail_ret = bpf_skb_change_tail(skb, 0, 0);
    return TCX_PASS;
    }
    return TCX_DROP;
    }
    char _license[] SEC("license") = "GPL";
