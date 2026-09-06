//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/xdp_redirect_map.c
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
    __uint(type, BPF_MAP_TYPE_DEVMAP);
    __uint(max_entries, 8);
    __uint(key_size, sizeof(int));
    __uint(value_size, sizeof(int));
    } tx_port SEC(".maps");
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_redirect_map_0(xdp: *mut xdp_md) -> c_int {
    int xdp_redirect_map_0(struct xdp_md *xdp)
    {
    return bpf_redirect_map(&tx_port, 0, 0);
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_redirect_map_1(xdp: *mut xdp_md) -> c_int {
    int xdp_redirect_map_1(struct xdp_md *xdp)
    {
    return bpf_redirect_map(&tx_port, 1, 0);
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_redirect_map_2(xdp: *mut xdp_md) -> c_int {
    int xdp_redirect_map_2(struct xdp_md *xdp)
    {
    return bpf_redirect_map(&tx_port, 2, 0);
    }
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 3);
    __type(key, __u32);
    __type(value, __u64);
    } rxcnt SEC(".maps");
#[no_mangle]
unsafe extern "C" fn xdp_count(xdp: *mut xdp_md, key: __u32) -> c_int {
    static int xdp_count(struct xdp_md *xdp, __u32 key)
    {
    void *data_end = (void *)(long)xdp.data_end;
    void *data = (void *)(long)xdp.data;
    struct ethhdr *eth = data;
    __u64 *count;
    if (data + sizeof(*eth) > data_end)
    return XDP_DROP;
    if (bpf_htons(eth.h_proto) == ETH_P_IP) {
// We only count IPv4 packets
    count = bpf_map_lookup_elem(&rxcnt, &key);
    if (count)
// count += 1;
    }
    return XDP_PASS;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_count_0(xdp: *mut xdp_md) -> c_int {
    int xdp_count_0(struct xdp_md *xdp)
    {
    return xdp_count(xdp, 0);
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_count_1(xdp: *mut xdp_md) -> c_int {
    int xdp_count_1(struct xdp_md *xdp)
    {
    return xdp_count(xdp, 1);
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_count_2(xdp: *mut xdp_md) -> c_int {
    int xdp_count_2(struct xdp_md *xdp)
    {
    return xdp_count(xdp, 2);
    }
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 2);
    __type(key, __u32);
    __type(value, __be64);
    } rx_mac SEC(".maps");
#[no_mangle]
unsafe extern "C" fn store_mac(xdp: *mut xdp_md, id: __u32) -> c_int {
    static int store_mac(struct xdp_md *xdp, __u32 id)
    {
    void *data_end = (void *)(long)xdp.data_end;
    void *data = (void *)(long)xdp.data;
    struct ethhdr *eth = data;
    let mut key: __u32 = id;
    let mut mac: __be64 = 0;
    if (data + sizeof(*eth) > data_end)
    return XDP_DROP;
// Only store IPv4 MAC to avoid being polluted by IPv6 packets
    if (eth.h_proto == bpf_htons(ETH_P_IP)) {
    __builtin_memcpy(&mac, eth.h_source, ETH_ALEN);
    bpf_map_update_elem(&rx_mac, &key, &mac, 0);
    bpf_printk("%s - %x", __func__, mac);
    }
    return XDP_PASS;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn store_mac_1(xdp: *mut xdp_md) -> c_int {
    int store_mac_1(struct xdp_md *xdp)
    {
    return store_mac(xdp, 0);
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn store_mac_2(xdp: *mut xdp_md) -> c_int {
    int store_mac_2(struct xdp_md *xdp)
    {
    return store_mac(xdp, 1);
    }
    char _license[] SEC("license") = "GPL";
