//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/xdp_redirect_multi_kern.c
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

// One map use devmap, another one use devmap_hash for testing
    struct {
    __uint(type, BPF_MAP_TYPE_DEVMAP);
    __uint(key_size, sizeof(int));
    __uint(value_size, sizeof(int));
    __uint(max_entries, 1024);
    } map_all SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_DEVMAP_HASH);
    __uint(key_size, sizeof(int));
    __uint(value_size, sizeof(struct bpf_devmap_val));
    __uint(max_entries, 128);
    } map_egress SEC(".maps");
// map to store egress interfaces mac addresses
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, __u32);
    __type(value, __be64);
    __uint(max_entries, 128);
    } mac_map SEC(".maps");
// map to store redirect flags for each protocol
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, __u16);
    __type(value, __u64);
    __uint(max_entries, 16);
    } redirect_flags SEC(".maps");
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_redirect_map_multi_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_redirect_map_multi_prog(struct xdp_md *ctx)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    let mut if_index: c_int = ctx.ingress_ifindex;
    struct ethhdr *eth = data;
    __u64 *flags_from_map;
    __u16 h_proto;
    __u64 nh_off;
    __u64 flags;
    nh_off = sizeof(*eth);
    if (data + nh_off > data_end)
    return XDP_DROP;
    h_proto = bpf_htons(eth.h_proto);
    flags_from_map = bpf_map_lookup_elem(&redirect_flags, &h_proto);
// Default flags for IPv4 : (BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS)
    if (h_proto == ETH_P_IP) {
    flags = flags_from_map ? *flags_from_map : BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS;
    return bpf_redirect_map(&map_all, 0, flags);
    }
// Default flags for IPv6 : 0
    if (h_proto == ETH_P_IPV6) {
    flags = flags_from_map ? *flags_from_map : 0;
    return bpf_redirect_map(&map_all, if_index, flags);
    }
// Default flags for others BPF_F_BROADCAST : 0
    else {
    flags = flags_from_map ? *flags_from_map : BPF_F_BROADCAST;
    return bpf_redirect_map(&map_all, 0, flags);
    }
    }
// The following 2 progs are for 2nd devmap prog testing
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_redirect_map_all_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_redirect_map_all_prog(struct xdp_md *ctx)
    {
    return bpf_redirect_map(&map_egress, 0,
    BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS);
    }
    SEC("xdp/devmap")
#[no_mangle]
pub unsafe extern "C" fn xdp_devmap_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_devmap_prog(struct xdp_md *ctx)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    let mut key: __u32 = ctx.egress_ifindex;
    struct ethhdr *eth = data;
    __u64 nh_off;
    __be64 *mac;
    nh_off = sizeof(*eth);
    if (data + nh_off > data_end)
    return XDP_DROP;
    mac = bpf_map_lookup_elem(&mac_map, &key);
    if (mac)
    __builtin_memcpy(eth.h_source, mac, ETH_ALEN);
    return XDP_PASS;
    }
    char _license[] SEC("license") = "GPL";
