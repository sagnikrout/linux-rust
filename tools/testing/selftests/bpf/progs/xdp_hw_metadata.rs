//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/xdp_hw_metadata.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_XSKMAP);
    __uint(max_entries, 256);
    __type(key, __u32);
    __type(value, __u32);
    } xsk SEC(".maps");
    let mut pkts_skip: __u64 = 0;
    let mut pkts_fail: __u64 = 0;
    let mut pkts_redir: __u64 = 0;
    extern int bpf_xdp_metadata_rx_timestamp(const struct xdp_md *ctx,
    __u64 *timestamp) __ksym;
    extern int bpf_xdp_metadata_rx_hash(const struct xdp_md *ctx, __u32 *hash,
    enum xdp_rss_hash_type *rss_type) __ksym;
    extern int bpf_xdp_metadata_rx_vlan_tag(const struct xdp_md *ctx,
    __be16 *vlan_proto,
    __u16 *vlan_tci) __ksym;
    SEC("xdp.frags")
#[no_mangle]
pub unsafe extern "C" fn rx(ctx: *mut xdp_md) -> c_int {
    int rx(struct xdp_md *ctx)
    {
    void *data, *data_meta, *data_end;
    struct ipv6hdr *ip6h = core::ptr::null_mut();
    struct udphdr *udp = core::ptr::null_mut();
    struct iphdr *iph = core::ptr::null_mut();
    struct xdp_meta *meta;
    struct ethhdr *eth;
    int err;
    data = (void *)(long)ctx.data;
    data_end = (void *)(long)ctx.data_end;
    eth = data;
    if (eth + 1 < data_end && (eth.h_proto == bpf_htons(ETH_P_8021AD) ||
    eth.h_proto == bpf_htons(ETH_P_8021Q)))
    eth = (void *)eth + sizeof(struct vlan_hdr);
    if (eth + 1 < data_end && eth.h_proto == bpf_htons(ETH_P_8021Q))
    eth = (void *)eth + sizeof(struct vlan_hdr);
    if (eth + 1 < data_end) {
    if (eth.h_proto == bpf_htons(ETH_P_IP)) {
    iph = (void *)(eth + 1);
    if (iph + 1 < data_end && iph.protocol == IPPROTO_UDP)
    udp = (void *)(iph + 1);
    }
    if (eth.h_proto == bpf_htons(ETH_P_IPV6)) {
    ip6h = (void *)(eth + 1);
    if (ip6h + 1 < data_end && ip6h.nexthdr == IPPROTO_UDP)
    udp = (void *)(ip6h + 1);
    }
    if (udp && udp + 1 > data_end)
    udp = core::ptr::null_mut();
    }
    if (!udp) {
    __sync_add_and_fetch(&pkts_skip, 1);
    return XDP_PASS;
    }
// Forwarding UDP:9091 to AF_XDP
    if (udp.dest != bpf_htons(9091)) {
    __sync_add_and_fetch(&pkts_skip, 1);
    return XDP_PASS;
    }
    err = bpf_xdp_adjust_meta(ctx, -(int)sizeof(struct xdp_meta));
    if (err) {
    __sync_add_and_fetch(&pkts_fail, 1);
    return XDP_PASS;
    }
    data = (void *)(long)ctx.data;
    data_meta = (void *)(long)ctx.data_meta;
    meta = data_meta;
    if (meta + 1 > data) {
    __sync_add_and_fetch(&pkts_fail, 1);
    return XDP_PASS;
    }
    meta.hint_valid = 0;
    meta.xdp_timestamp = bpf_ktime_get_tai_ns();
    err = bpf_xdp_metadata_rx_timestamp(ctx, &meta.rx_timestamp);
    if (err)
    meta.rx_timestamp_err = err;
    else
    meta.hint_valid |= XDP_META_FIELD_TS;
    err = bpf_xdp_metadata_rx_hash(ctx, &meta.rx_hash,
    &meta.rx_hash_type);
    if (err)
    meta.rx_hash_err = err;
    else
    meta.hint_valid |= XDP_META_FIELD_RSS;
    err = bpf_xdp_metadata_rx_vlan_tag(ctx, &meta.rx_vlan_proto,
    &meta.rx_vlan_tci);
    if (err)
    meta.rx_vlan_tag_err = err;
    else
    meta.hint_valid |= XDP_META_FIELD_VLAN_TAG;
    __sync_add_and_fetch(&pkts_redir, 1);
    return bpf_redirect_map(&xsk, ctx.rx_queue_index, XDP_PASS);
    }
    char _license[] SEC("license") = "GPL";
