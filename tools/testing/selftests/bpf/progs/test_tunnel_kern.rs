//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tunnel_kern.c
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
// Copyright (c) 2016 VMware
// Copyright (c) 2016 Facebook
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//
// Macro flag: #define BPF_NO_KFUNC_PROTOTYPES

pub const VXLAN_UDP_PORT: c_int = 4789;
pub const ETH_P_IP: c_uint = 0x0800;
pub const PACKET_HOST: c_int = 0;

// Only IPv4 address assigned to veth1.
// 172.16.1.200
//
pub const ASSIGNED_ADDR_VETH1: c_uint = 0xac1001c8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_fou_encap___local {
    pub sport: __be16,
    pub dport: __be16,
    pub __attribute__((preserve_access_index)): },
    enum bpf_fou_encap_type___local {
    FOU_BPF_ENCAP_FOU___local,
    FOU_BPF_ENCAP_GUE___local,
}

    int bpf_skb_set_fou_encap(struct __sk_buff *skb_ctx,
    struct bpf_fou_encap___local *encap, int type) __ksym;
    int bpf_skb_get_fou_encap(struct __sk_buff *skb_ctx,
    struct bpf_fou_encap___local *encap) __ksym;
    struct xfrm_state *
    bpf_xdp_get_xfrm_state(struct xdp_md *ctx, struct bpf_xfrm_state_opts *opts,
    u32 opts__sz) __ksym;
    void bpf_xdp_xfrm_state_release(struct xfrm_state *x) __ksym;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);
    } local_ip_map SEC(".maps");
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn gre_set_tunnel(skb: *mut __sk_buff) -> c_int {
    int gre_set_tunnel(struct __sk_buff *skb)
    {
    int ret;
    struct bpf_tunnel_key key;
    __builtin_memset(&key, 0x0, sizeof(key));
    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_ZERO_CSUM_TX | BPF_F_SEQ_NUMBER);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn gre_set_tunnel_no_key(skb: *mut __sk_buff) -> c_int {
    int gre_set_tunnel_no_key(struct __sk_buff *skb)
    {
    int ret;
    struct bpf_tunnel_key key;
    __builtin_memset(&key, 0x0, sizeof(key));
    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    key.tunnel_ttl = 64;
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_ZERO_CSUM_TX | BPF_F_SEQ_NUMBER |
    BPF_F_NO_TUNNEL_KEY);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn gre_get_tunnel(skb: *mut __sk_buff) -> c_int {
    int gre_get_tunnel(struct __sk_buff *skb)
    {
    int ret;
    struct bpf_tunnel_key key;
    ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key), 0);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    bpf_printk("key %d remote ip 0x%x\n", key.tunnel_id, key.remote_ipv4);
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ip6gretap_set_tunnel(skb: *mut __sk_buff) -> c_int {
    int ip6gretap_set_tunnel(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    int ret;
    __builtin_memset(&key, 0x0, sizeof(key));
    key.remote_ipv6[3] = bpf_htonl(0x11); /* ::11 */
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;
    key.tunnel_label = 0xabcde;
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6 | BPF_F_ZERO_CSUM_TX |
    BPF_F_SEQ_NUMBER);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ip6gretap_get_tunnel(skb: *mut __sk_buff) -> c_int {
    int ip6gretap_get_tunnel(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    int ret;
    ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    bpf_printk("key %d remote ip6 ::%x label %x\n",
    key.tunnel_id, key.remote_ipv6[3], key.tunnel_label);
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn erspan_set_tunnel(skb: *mut __sk_buff) -> c_int {
    int erspan_set_tunnel(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    struct erspan_metadata md;
    int ret;
    __builtin_memset(&key, 0x0, sizeof(key));
    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_ZERO_CSUM_TX);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    __builtin_memset(&md, 0, sizeof(md));

    md.version = 1;
    md.u.index = bpf_htonl(123);

    let mut direction: __u8 = 1;
    let mut hwid: __u8 = 7;
    md.version = 2;
    BPF_CORE_WRITE_BITFIELD(&md.u.md2, dir, direction);
    BPF_CORE_WRITE_BITFIELD(&md.u.md2, hwid, (hwid & 0xf));
    BPF_CORE_WRITE_BITFIELD(&md.u.md2, hwid_upper, (hwid >> 4) & 0x3);

    ret = bpf_skb_set_tunnel_opt(skb, &md, sizeof(md));
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn erspan_get_tunnel(skb: *mut __sk_buff) -> c_int {
    int erspan_get_tunnel(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    struct erspan_metadata md;
    int ret;
    ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key), 0);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    ret = bpf_skb_get_tunnel_opt(skb, &md, sizeof(md));
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    bpf_printk("key %d remote ip 0x%x erspan version %d\n",
    key.tunnel_id, key.remote_ipv4, md.version);

    index = bpf_ntohl(md.u.index);
    bpf_printk("\tindex %x\n", index);

    bpf_printk("\tdirection %d hwid %x timestamp %u\n",
    BPF_CORE_READ_BITFIELD(&md.u.md2, dir),
    (BPF_CORE_READ_BITFIELD(&md.u.md2, hwid_upper) << 4) +
    BPF_CORE_READ_BITFIELD(&md.u.md2, hwid),
    bpf_ntohl(md.u.md2.timestamp));

    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ip4ip6erspan_set_tunnel(skb: *mut __sk_buff) -> c_int {
    int ip4ip6erspan_set_tunnel(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    struct erspan_metadata md;
    int ret;
    __builtin_memset(&key, 0x0, sizeof(key));
    key.remote_ipv6[3] = bpf_htonl(0x11);
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    __builtin_memset(&md, 0, sizeof(md));

    md.u.index = bpf_htonl(123);
    md.version = 1;

    let mut direction: __u8 = 0;
    let mut hwid: __u8 = 17;
    md.version = 2;
    BPF_CORE_WRITE_BITFIELD(&md.u.md2, dir, direction);
    BPF_CORE_WRITE_BITFIELD(&md.u.md2, hwid, (hwid & 0xf));
    BPF_CORE_WRITE_BITFIELD(&md.u.md2, hwid_upper, (hwid >> 4) & 0x3);

    ret = bpf_skb_set_tunnel_opt(skb, &md, sizeof(md));
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ip4ip6erspan_get_tunnel(skb: *mut __sk_buff) -> c_int {
    int ip4ip6erspan_get_tunnel(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    struct erspan_metadata md;
    int ret;
    ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    ret = bpf_skb_get_tunnel_opt(skb, &md, sizeof(md));
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    bpf_printk("ip6erspan get key %d remote ip6 ::%x erspan version %d\n",
    key.tunnel_id, key.remote_ipv4, md.version);

    index = bpf_ntohl(md.u.index);
    bpf_printk("\tindex %x\n", index);

    bpf_printk("\tdirection %d hwid %x timestamp %u\n",
    BPF_CORE_READ_BITFIELD(&md.u.md2, dir),
    (BPF_CORE_READ_BITFIELD(&md.u.md2, hwid_upper) << 4) +
    BPF_CORE_READ_BITFIELD(&md.u.md2, hwid),
    bpf_ntohl(md.u.md2.timestamp));

    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn vxlan_set_tunnel_dst(skb: *mut __sk_buff) -> c_int {
    int vxlan_set_tunnel_dst(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    struct vxlan_metadata md;
    let mut index: __u32 = 0;
    __u32 *local_ip = core::ptr::null_mut();
    let mut ret: c_int = 0;
    local_ip = bpf_map_lookup_elem(&local_ip_map, &index);
    if (!local_ip) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    __builtin_memset(&key, 0x0, sizeof(key));
    key.local_ipv4 = 0xac100164; /* 172.16.1.100 */
    key.remote_ipv4 = *local_ip;
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_ZERO_CSUM_TX);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    md.gbp = 0x800FF; /* Set VXLAN Group Policy extension */
    ret = bpf_skb_set_tunnel_opt(skb, &md, sizeof(md));
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn vxlan_set_tunnel_src(skb: *mut __sk_buff) -> c_int {
    int vxlan_set_tunnel_src(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    struct vxlan_metadata md;
    let mut index: __u32 = 0;
    __u32 *local_ip = core::ptr::null_mut();
    let mut ret: c_int = 0;
    local_ip = bpf_map_lookup_elem(&local_ip_map, &index);
    if (!local_ip) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    __builtin_memset(&key, 0x0, sizeof(key));
    key.local_ipv4 = *local_ip;
    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_ZERO_CSUM_TX);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    md.gbp = 0x800FF; /* Set VXLAN Group Policy extension */
    ret = bpf_skb_set_tunnel_opt(skb, &md, sizeof(md));
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn vxlan_get_tunnel_src(skb: *mut __sk_buff) -> c_int {
    int vxlan_get_tunnel_src(struct __sk_buff *skb)
    {
    int ret;
    struct bpf_tunnel_key key;
    struct vxlan_metadata md;
    ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_FLAGS);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    ret = bpf_skb_get_tunnel_opt(skb, &md, sizeof(md));
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    if (key.local_ipv4 != ASSIGNED_ADDR_VETH1 || md.gbp != 0x800FF ||
    !(key.tunnel_flags & TUNNEL_KEY) ||
    (key.tunnel_flags & TUNNEL_CSUM)) {
    bpf_printk("vxlan key %d local ip 0x%x remote ip 0x%x gbp 0x%x flags 0x%x\n",
    key.tunnel_id, key.local_ipv4,
    key.remote_ipv4, md.gbp,
    bpf_ntohs(key.tunnel_flags));
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn veth_set_outer_dst(skb: *mut __sk_buff) -> c_int {
    int veth_set_outer_dst(struct __sk_buff *skb)
    {
    struct ethhdr *eth = (struct ethhdr *)(long)skb.data;
    let mut assigned_ip: __u32 = bpf_htonl(ASSIGNED_ADDR_VETH1);
    void *data_end = (void *)(long)skb.data_end;
    struct udphdr *udph;
    struct iphdr *iph;
    let mut ret: c_int = 0;
    __s64 csum;
    if ((void *)eth + sizeof(*eth) > data_end) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    if (eth.h_proto != bpf_htons(ETH_P_IP))
    return TC_ACT_OK;
    iph = (struct iphdr *)(eth + 1);
    if ((void *)iph + sizeof(*iph) > data_end) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    if (iph.protocol != IPPROTO_UDP)
    return TC_ACT_OK;
    udph = (struct udphdr *)(iph + 1);
    if ((void *)udph + sizeof(*udph) > data_end) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    if (udph.dest != bpf_htons(VXLAN_UDP_PORT))
    return TC_ACT_OK;
    if (iph.daddr != assigned_ip) {
    csum = bpf_csum_diff(&iph.daddr, sizeof(__u32), &assigned_ip,
    sizeof(__u32), 0);
    if (bpf_skb_store_bytes(skb, ETH_HLEN + offsetof(struct iphdr, daddr),
    &assigned_ip, sizeof(__u32), 0) < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    if (bpf_l3_csum_replace(skb, ETH_HLEN + offsetof(struct iphdr, check),
    0, csum, 0) < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    bpf_skb_change_type(skb, PACKET_HOST);
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ip6vxlan_set_tunnel_dst(skb: *mut __sk_buff) -> c_int {
    int ip6vxlan_set_tunnel_dst(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    let mut index: __u32 = 0;
    __u32 *local_ip;
    let mut ret: c_int = 0;
    local_ip = bpf_map_lookup_elem(&local_ip_map, &index);
    if (!local_ip) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    __builtin_memset(&key, 0x0, sizeof(key));
    key.local_ipv6[3] = bpf_htonl(0x11); /* ::11 */
    key.remote_ipv6[3] = bpf_htonl(*local_ip);
    key.tunnel_id = 22;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ip6vxlan_set_tunnel_src(skb: *mut __sk_buff) -> c_int {
    int ip6vxlan_set_tunnel_src(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    let mut index: __u32 = 0;
    __u32 *local_ip;
    let mut ret: c_int = 0;
    local_ip = bpf_map_lookup_elem(&local_ip_map, &index);
    if (!local_ip) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    __builtin_memset(&key, 0x0, sizeof(key));
    key.local_ipv6[3] = bpf_htonl(*local_ip);
    key.remote_ipv6[3] = bpf_htonl(0x11); /* ::11 */
    key.tunnel_id = 22;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ip6vxlan_get_tunnel_src(skb: *mut __sk_buff) -> c_int {
    int ip6vxlan_get_tunnel_src(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    let mut index: __u32 = 0;
    __u32 *local_ip;
    let mut ret: c_int = 0;
    local_ip = bpf_map_lookup_elem(&local_ip_map, &index);
    if (!local_ip) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6 | BPF_F_TUNINFO_FLAGS);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    if (bpf_ntohl(key.local_ipv6[3]) != *local_ip ||
    !(key.tunnel_flags & TUNNEL_KEY) ||
    !(key.tunnel_flags & TUNNEL_CSUM)) {
    bpf_printk("ip6vxlan key %d local ip6 ::%x remote ip6 ::%x label 0x%x flags 0x%x\n",
    key.tunnel_id, bpf_ntohl(key.local_ipv6[3]),
    bpf_ntohl(key.remote_ipv6[3]), key.tunnel_label,
    bpf_ntohs(key.tunnel_flags));
    bpf_printk("local_ip 0x%x\n", *local_ip);
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct local_geneve_opt {
    pub gopt: geneve_opt,
    pub data: c_int,
}

    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn geneve_set_tunnel(skb: *mut __sk_buff) -> c_int {
    int geneve_set_tunnel(struct __sk_buff *skb)
    {
    int ret;
    struct bpf_tunnel_key key;
    struct local_geneve_opt local_gopt;
    struct geneve_opt *gopt = (struct geneve_opt *) &local_gopt;
    __builtin_memset(&key, 0x0, sizeof(key));
    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;
    __builtin_memset(gopt, 0x0, sizeof(local_gopt));
    gopt.opt_class = bpf_htons(0x102); /* Open Virtual Networking (OVN) */
    gopt.type = 0x08;
    gopt.r1 = 0;
    gopt.r2 = 0;
    gopt.r3 = 0;
    gopt.length = 2; /* 4-byte multiple */
// (int *) &gopt->opt_data = bpf_htonl(0xdeadbeef);
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_ZERO_CSUM_TX);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    ret = bpf_skb_set_tunnel_opt(skb, gopt, sizeof(local_gopt));
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn geneve_get_tunnel(skb: *mut __sk_buff) -> c_int {
    int geneve_get_tunnel(struct __sk_buff *skb)
    {
    int ret;
    struct bpf_tunnel_key key;
    struct geneve_opt gopt;
    ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key), 0);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    ret = bpf_skb_get_tunnel_opt(skb, &gopt, sizeof(gopt));
    if (ret < 0)
    gopt.opt_class = 0;
    bpf_printk("key %d remote ip 0x%x geneve class 0x%x\n",
    key.tunnel_id, key.remote_ipv4, gopt.opt_class);
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ip6geneve_set_tunnel(skb: *mut __sk_buff) -> c_int {
    int ip6geneve_set_tunnel(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    struct local_geneve_opt local_gopt;
    struct geneve_opt *gopt = (struct geneve_opt *) &local_gopt;
    int ret;
    __builtin_memset(&key, 0x0, sizeof(key));
    key.remote_ipv6[3] = bpf_htonl(0x11); /* ::11 */
    key.tunnel_id = 22;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    __builtin_memset(gopt, 0x0, sizeof(local_gopt));
    gopt.opt_class = bpf_htons(0x102); /* Open Virtual Networking (OVN) */
    gopt.type = 0x08;
    gopt.r1 = 0;
    gopt.r2 = 0;
    gopt.r3 = 0;
    gopt.length = 2; /* 4-byte multiple */
// (int *) &gopt->opt_data = bpf_htonl(0xfeedbeef);
    ret = bpf_skb_set_tunnel_opt(skb, gopt, sizeof(gopt));
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ip6geneve_get_tunnel(skb: *mut __sk_buff) -> c_int {
    int ip6geneve_get_tunnel(struct __sk_buff *skb)
    {
    struct bpf_tunnel_key key;
    struct geneve_opt gopt;
    int ret;
    ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    ret = bpf_skb_get_tunnel_opt(skb, &gopt, sizeof(gopt));
    if (ret < 0)
    gopt.opt_class = 0;
    bpf_printk("key %d remote ip 0x%x geneve class 0x%x\n",
    key.tunnel_id, key.remote_ipv4, gopt.opt_class);
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ipip_set_tunnel(skb: *mut __sk_buff) -> c_int {
    int ipip_set_tunnel(struct __sk_buff *skb)
    {
    let mut key: bpf_tunnel_key = {};
    void *data = (void *)(long)skb.data;
    struct iphdr *iph = data;
    void *data_end = (void *)(long)skb.data_end;
    int ret;
// single length check
    if (data + sizeof(*iph) > data_end) {
    log_err(1);
    return TC_ACT_SHOT;
    }
    key.tunnel_ttl = 64;
    if (iph.protocol == IPPROTO_ICMP) {
    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    }
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key), 0);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ipip_get_tunnel(skb: *mut __sk_buff) -> c_int {
    int ipip_get_tunnel(struct __sk_buff *skb)
    {
    int ret;
    struct bpf_tunnel_key key;
    ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key), 0);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    bpf_printk("remote ip 0x%x\n", key.remote_ipv4);
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ipip_gue_set_tunnel(skb: *mut __sk_buff) -> c_int {
    int ipip_gue_set_tunnel(struct __sk_buff *skb)
    {
    let mut key: bpf_tunnel_key = {};
    let mut encap: bpf_fou_encap___local = {};
    void *data = (void *)(long)skb.data;
    struct iphdr *iph = data;
    void *data_end = (void *)(long)skb.data_end;
    int ret;
    if (data + sizeof(*iph) > data_end) {
    log_err(1);
    return TC_ACT_SHOT;
    }
    key.tunnel_ttl = 64;
    if (iph.protocol == IPPROTO_ICMP)
    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key), 0);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    encap.sport = 0;
    encap.dport = bpf_htons(5555);
    ret = bpf_skb_set_fou_encap(skb, &encap,
    bpf_core_enum_value(enum bpf_fou_encap_type___local,
    FOU_BPF_ENCAP_GUE___local));
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ipip_fou_set_tunnel(skb: *mut __sk_buff) -> c_int {
    int ipip_fou_set_tunnel(struct __sk_buff *skb)
    {
    let mut key: bpf_tunnel_key = {};
    let mut encap: bpf_fou_encap___local = {};
    void *data = (void *)(long)skb.data;
    struct iphdr *iph = data;
    void *data_end = (void *)(long)skb.data_end;
    int ret;
    if (data + sizeof(*iph) > data_end) {
    log_err(1);
    return TC_ACT_SHOT;
    }
    key.tunnel_ttl = 64;
    if (iph.protocol == IPPROTO_ICMP)
    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key), 0);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    encap.sport = 0;
    encap.dport = bpf_htons(5555);
    ret = bpf_skb_set_fou_encap(skb, &encap,
    FOU_BPF_ENCAP_FOU___local);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ipip_encap_get_tunnel(skb: *mut __sk_buff) -> c_int {
    int ipip_encap_get_tunnel(struct __sk_buff *skb)
    {
    int ret;
    let mut key: bpf_tunnel_key = {};
    let mut encap: bpf_fou_encap___local = {};
    ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key), 0);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    ret = bpf_skb_get_fou_encap(skb, &encap);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    if (bpf_ntohs(encap.dport) != 5555)
    return TC_ACT_SHOT;
    bpf_printk("%d remote ip 0x%x, sport %d, dport %d\n", ret,
    key.remote_ipv4, bpf_ntohs(encap.sport),
    bpf_ntohs(encap.dport));
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ipip6_set_tunnel(skb: *mut __sk_buff) -> c_int {
    int ipip6_set_tunnel(struct __sk_buff *skb)
    {
    let mut key: bpf_tunnel_key = {};
    void *data = (void *)(long)skb.data;
    struct iphdr *iph = data;
    void *data_end = (void *)(long)skb.data_end;
    int ret;
// single length check
    if (data + sizeof(*iph) > data_end) {
    log_err(1);
    return TC_ACT_SHOT;
    }
    __builtin_memset(&key, 0x0, sizeof(key));
    key.tunnel_ttl = 64;
    if (iph.protocol == IPPROTO_ICMP) {
    key.remote_ipv6[3] = bpf_htonl(0x11); /* ::11 */
    }
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ipip6_get_tunnel(skb: *mut __sk_buff) -> c_int {
    int ipip6_get_tunnel(struct __sk_buff *skb)
    {
    int ret;
    struct bpf_tunnel_key key;
    ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    bpf_printk("remote ip6 %x::%x\n", bpf_htonl(key.remote_ipv6[0]),
    bpf_htonl(key.remote_ipv6[3]));
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ip6ip6_set_tunnel(skb: *mut __sk_buff) -> c_int {
    int ip6ip6_set_tunnel(struct __sk_buff *skb)
    {
    let mut key: bpf_tunnel_key = {};
    void *data = (void *)(long)skb.data;
    struct ipv6hdr *iph = data;
    void *data_end = (void *)(long)skb.data_end;
    int ret;
// single length check
    if (data + sizeof(*iph) > data_end) {
    log_err(1);
    return TC_ACT_SHOT;
    }
    key.tunnel_ttl = 64;
    if (iph.nexthdr == 58 /* NEXTHDR_ICMP */) {
    key.remote_ipv6[3] = bpf_htonl(0x11); /* ::11 */
    }
    ret = bpf_skb_set_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn ip6ip6_get_tunnel(skb: *mut __sk_buff) -> c_int {
    int ip6ip6_get_tunnel(struct __sk_buff *skb)
    {
    int ret;
    struct bpf_tunnel_key key;
    ret = bpf_skb_get_tunnel_key(skb, &key, sizeof(key),
    BPF_F_TUNINFO_IPV6);
    if (ret < 0) {
    log_err(ret);
    return TC_ACT_SHOT;
    }
    bpf_printk("remote ip6 %x::%x\n", bpf_htonl(key.remote_ipv6[0]),
    bpf_htonl(key.remote_ipv6[3]));
    return TC_ACT_OK;
    }
    let mut xfrm_reqid: volatile int = 0;
    let mut xfrm_spi: volatile int = 0;
    let mut xfrm_remote_ip: volatile int = 0;
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn xfrm_get_state(skb: *mut __sk_buff) -> c_int {
    int xfrm_get_state(struct __sk_buff *skb)
    {
    struct bpf_xfrm_state x;
    int ret;
    ret = bpf_skb_get_xfrm_state(skb, 0, &x, sizeof(x), 0);
    if (ret < 0)
    return TC_ACT_OK;
    xfrm_reqid = x.reqid;
    xfrm_spi = bpf_ntohl(x.spi);
    xfrm_remote_ip = bpf_ntohl(x.remote_ipv4);
    return TC_ACT_OK;
    }
    let mut xfrm_replay_window: volatile int = 0;
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xfrm_get_state_xdp(xdp: *mut xdp_md) -> c_int {
    int xfrm_get_state_xdp(struct xdp_md *xdp)
    {
    let mut opts: bpf_xfrm_state_opts = {};
    struct xfrm_state *x = core::ptr::null_mut();
    struct ip_esp_hdr *esph;
    struct bpf_dynptr ptr;
    u8 esph_buf[8] = {};
    u8 iph_buf[20] = {};
    struct iphdr *iph;
    u32 off;
    if (bpf_dynptr_from_xdp(xdp, 0, &ptr))
    goto out;
    off = sizeof(struct ethhdr);
    iph = bpf_dynptr_slice(&ptr, off, iph_buf, sizeof(iph_buf));
    if (!iph || iph.protocol != IPPROTO_ESP)
    goto out;
    off += sizeof(struct iphdr);
    esph = bpf_dynptr_slice(&ptr, off, esph_buf, sizeof(esph_buf));
    if (!esph)
    goto out;
    opts.netns_id = BPF_F_CURRENT_NETNS;
    opts.daddr.a4 = iph.daddr;
    opts.spi = esph.spi;
    opts.proto = IPPROTO_ESP;
    opts.family = AF_INET;
    x = bpf_xdp_get_xfrm_state(xdp, &opts, sizeof(opts));
    if (!x)
    goto out;
    if (!x.replay_esn)
    goto out;
    xfrm_replay_window = x.replay_esn.replay_window;
    out:
    if (x)
    bpf_xdp_xfrm_state_release(x);
    return XDP_PASS;
    }
    char _license[] SEC("license") = "GPL";
