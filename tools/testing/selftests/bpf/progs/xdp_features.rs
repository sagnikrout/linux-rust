//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/xdp_features.c
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

    (a).s6_addr32[1] == (b).s6_addr32[1] &&	\
    (a).s6_addr32[2] == (b).s6_addr32[2] &&	\
    (a).s6_addr32[3] == (b).s6_addr32[3])
    struct net_device;
    struct bpf_prog;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_cpumap_stats {
    pub redirect: c_uint,
    pub pass: c_uint,
    pub drop: c_uint,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, __u32);
    __type(value, __u32);
    __uint(max_entries, 1);
    } stats SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, __u32);
    __type(value, __u32);
    __uint(max_entries, 1);
    } dut_stats SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_CPUMAP);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bpf_cpumap_val));
    __uint(max_entries, 1);
    } cpu_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_DEVMAP);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bpf_devmap_val));
    __uint(max_entries, 1);
    } dev_map SEC(".maps");
    const volatile struct in6_addr tester_addr;
    const volatile struct in6_addr dut_addr;
    static __always_inline int
    xdp_process_echo_packet(struct xdp_md *xdp, bool dut)
    {
    void *data_end = (void *)(long)xdp.data_end;
    void *data = (void *)(long)xdp.data;
    struct ethhdr *eh = data;
    struct tlv_hdr *tlv;
    struct udphdr *uh;
    __be16 port;
    if (eh + 1 > (struct ethhdr *)data_end)
    return -EINVAL;
    if (eh.h_proto == bpf_htons(ETH_P_IP)) {
    struct iphdr *ih = (struct iphdr *)(eh + 1);
    __be32 saddr = dut ? tester_addr.s6_addr32[3]
    : dut_addr.s6_addr32[3];
    __be32 daddr = dut ? dut_addr.s6_addr32[3]
    : tester_addr.s6_addr32[3];
    ih = (struct iphdr *)(eh + 1);
    if (ih + 1 > (struct iphdr *)data_end)
    return -EINVAL;
    if (saddr != ih.saddr)
    return -EINVAL;
    if (daddr != ih.daddr)
    return -EINVAL;
    if (ih.protocol != IPPROTO_UDP)
    return -EINVAL;
    uh = (struct udphdr *)(ih + 1);
    } else if (eh.h_proto == bpf_htons(ETH_P_IPV6)) {
    let mut saddr: in6_addr = dut ? tester_addr : dut_addr;
    let mut daddr: in6_addr = dut ? dut_addr : tester_addr;
    struct ipv6hdr *ih6 = (struct ipv6hdr *)(eh + 1);
    if (ih6 + 1 > (struct ipv6hdr *)data_end)
    return -EINVAL;
    if (!ipv6_addr_equal(saddr, ih6.saddr))
    return -EINVAL;
    if (!ipv6_addr_equal(daddr, ih6.daddr))
    return -EINVAL;
    if (ih6.nexthdr != IPPROTO_UDP)
    return -EINVAL;
    uh = (struct udphdr *)(ih6 + 1);
    } else {
    return -EINVAL;
    }
    if (uh + 1 > (struct udphdr *)data_end)
    return -EINVAL;
    port = dut ? uh.dest : uh.source;
    if (port != bpf_htons(DUT_ECHO_PORT))
    return -EINVAL;
    tlv = (struct tlv_hdr *)(uh + 1);
    if (tlv + 1 > data_end)
    return -EINVAL;
    return bpf_htons(tlv.type) == CMD_ECHO ? 0 : -EINVAL;
    }
    static __always_inline int
    xdp_update_stats(struct xdp_md *xdp, bool tx, bool dut)
    {
    __u32 *val, key = 0;
    if (xdp_process_echo_packet(xdp, tx))
    return -EINVAL;
    if (dut)
    val = bpf_map_lookup_elem(&dut_stats, &key);
    else
    val = bpf_map_lookup_elem(&stats, &key);
    if (val)
    __sync_add_and_fetch(val, 1);
    return 0;
    }
// Tester
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_tester_check_tx(xdp: *mut xdp_md) -> c_int {
    int xdp_tester_check_tx(struct xdp_md *xdp)
    {
    xdp_update_stats(xdp, true, false);
    return XDP_PASS;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_tester_check_rx(xdp: *mut xdp_md) -> c_int {
    int xdp_tester_check_rx(struct xdp_md *xdp)
    {
    xdp_update_stats(xdp, false, false);
    return XDP_PASS;
    }
// DUT
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_do_pass(xdp: *mut xdp_md) -> c_int {
    int xdp_do_pass(struct xdp_md *xdp)
    {
    xdp_update_stats(xdp, true, true);
    return XDP_PASS;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_do_drop(xdp: *mut xdp_md) -> c_int {
    int xdp_do_drop(struct xdp_md *xdp)
    {
    if (xdp_update_stats(xdp, true, true))
    return XDP_PASS;
    return XDP_DROP;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_do_aborted(xdp: *mut xdp_md) -> c_int {
    int xdp_do_aborted(struct xdp_md *xdp)
    {
    if (xdp_process_echo_packet(xdp, true))
    return XDP_PASS;
    return XDP_ABORTED;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_do_tx(xdp: *mut xdp_md) -> c_int {
    int xdp_do_tx(struct xdp_md *xdp)
    {
    void *data = (void *)(long)xdp.data;
    struct ethhdr *eh = data;
    __u8 tmp_mac[ETH_ALEN];
    if (xdp_update_stats(xdp, true, true))
    return XDP_PASS;
    __builtin_memcpy(tmp_mac, eh.h_source, ETH_ALEN);
    __builtin_memcpy(eh.h_source, eh.h_dest, ETH_ALEN);
    __builtin_memcpy(eh.h_dest, tmp_mac, ETH_ALEN);
    return XDP_TX;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_do_redirect(xdp: *mut xdp_md) -> c_int {
    int xdp_do_redirect(struct xdp_md *xdp)
    {
    if (xdp_process_echo_packet(xdp, true))
    return XDP_PASS;
    return bpf_redirect_map(&cpu_map, 0, 0);
    }
    SEC("tp_btf/xdp_exception")
    int BPF_PROG(xdp_exception, const struct net_device *dev,
    const struct bpf_prog *xdp, __u32 act)
    {
    __u32 *val, key = 0;
    val = bpf_map_lookup_elem(&dut_stats, &key);
    if (val)
    __sync_add_and_fetch(val, 1);
    return 0;
    }
    SEC("tp_btf/xdp_cpumap_kthread")
    int BPF_PROG(tp_xdp_cpumap_kthread, int map_id, unsigned int processed,
    unsigned int drops, int sched, struct xdp_cpumap_stats *xdp_stats)
    {
    __u32 *val, key = 0;
    val = bpf_map_lookup_elem(&dut_stats, &key);
    if (val)
    __sync_add_and_fetch(val, 1);
    return 0;
    }
    SEC("xdp/cpumap")
#[no_mangle]
pub unsafe extern "C" fn xdp_do_redirect_cpumap(xdp: *mut xdp_md) -> c_int {
    int xdp_do_redirect_cpumap(struct xdp_md *xdp)
    {
    void *data = (void *)(long)xdp.data;
    struct ethhdr *eh = data;
    __u8 tmp_mac[ETH_ALEN];
    if (xdp_process_echo_packet(xdp, true))
    return XDP_PASS;
    __builtin_memcpy(tmp_mac, eh.h_source, ETH_ALEN);
    __builtin_memcpy(eh.h_source, eh.h_dest, ETH_ALEN);
    __builtin_memcpy(eh.h_dest, tmp_mac, ETH_ALEN);
    return bpf_redirect_map(&dev_map, 0, 0);
    }
    char _license[] SEC("license") = "GPL";
