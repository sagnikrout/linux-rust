//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_xdp_vlan.c
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
// Copyright(c) 2018 Jesper Dangaard Brouer.
//
// XDP/TC VLAN manipulation example
//
// GOTCHA: Remember to disable NIC hardware offloading of VLANs,
// else the VLAN tags are NOT inlined in the packet payload:
//
// # ethtool -K ixgbe2 rxvlan off
//
// Verify setting:
// # ethtool -k ixgbe2 | grep rx-vlan-offload
// rx-vlan-offload: off
//

// linux/if_vlan.h have not exposed this as UAPI, thus mirror some here
//
// struct vlan_hdr - vlan header
// @h_vlan_TCI: priority and VLAN ID
// @h_vlan_encapsulated_proto: packet type ID or len
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vlan_hdr {
    pub h_vlan_TCI: __be16,
    pub h_vlan_encapsulated_proto: __be16,
}

pub const VLAN_PRIO_MASK: c_uint = 0xe000 /* Priority Code Point */;
pub const VLAN_PRIO_SHIFT: c_int = 13;
pub const VLAN_CFI_MASK: c_uint = 0x1000 /* Canonical Format Indicator */;

pub const VLAN_VID_MASK: c_uint = 0x0fff /* VLAN Identifier */;
pub const VLAN_N_VID: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_pkt {
    pub l3_proto: __u16,
    pub l3_offset: __u16,
    pub vlan_outer: __u16,
    pub vlan_inner: __u16,
    pub vlan_outer_offset: __u8,
    pub vlan_inner_offset: __u8,
}

    char _license[] SEC("license") = "GPL";
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn parse_eth_frame(eth: *mut ethhdr, data_end: *mut c_void, pkt: *mut parse_pkt) -> bool {
    bool parse_eth_frame(struct ethhdr *eth, void *data_end, struct parse_pkt *pkt)
    {
    __u16 eth_type;
    __u8 offset;
    offset = sizeof(*eth);
// Make sure packet is large enough for parsing eth + 2 VLAN headers
    if ((void *)eth + offset + (2*sizeof(struct _vlan_hdr)) > data_end)
    return false;
    eth_type = eth.h_proto;
// Handle outer VLAN tag
    if (eth_type == bpf_htons(ETH_P_8021Q)
    || eth_type == bpf_htons(ETH_P_8021AD)) {
    struct _vlan_hdr *vlan_hdr;
    vlan_hdr = (void *)eth + offset;
    pkt.vlan_outer_offset = offset;
    pkt.vlan_outer = bpf_ntohs(vlan_hdr.h_vlan_TCI)
    & VLAN_VID_MASK;
    eth_type        = vlan_hdr.h_vlan_encapsulated_proto;
    offset += sizeof(*vlan_hdr);
    }
// Handle inner (double) VLAN tag
    if (eth_type == bpf_htons(ETH_P_8021Q)
    || eth_type == bpf_htons(ETH_P_8021AD)) {
    struct _vlan_hdr *vlan_hdr;
    vlan_hdr = (void *)eth + offset;
    pkt.vlan_inner_offset = offset;
    pkt.vlan_inner = bpf_ntohs(vlan_hdr.h_vlan_TCI)
    & VLAN_VID_MASK;
    eth_type        = vlan_hdr.h_vlan_encapsulated_proto;
    offset += sizeof(*vlan_hdr);
    }
    pkt.l3_proto = bpf_ntohs(eth_type); /* Convert to host-byte-order */
    pkt.l3_offset = offset;
    return true;
    }
// Hint, VLANs are chosen to hit network-byte-order issues

// #define TO_VLAN  4000 /* 0xFA0 (hint 0xOA0 = 160)
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_drop_vlan_4011(ctx: *mut xdp_md) -> c_int {
    int xdp_drop_vlan_4011(struct xdp_md *ctx)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data     = (void *)(long)ctx.data;
    let mut pkt: parse_pkt = { 0 };
    if (!parse_eth_frame(data, data_end, &pkt))
    return XDP_ABORTED;
// Drop specific VLAN ID example
    if (pkt.vlan_outer == TESTVLAN)
    return XDP_ABORTED;
//
// Using XDP_ABORTED makes it possible to record this event,
// via tracepoint xdp:xdp_exception like:
// # perf record -a -e xdp:xdp_exception
// # perf script
//
    return XDP_PASS;
    }
//
    Commands to setup VLAN on Linux to test packets gets dropped:
    export ROOTDEV=ixgbe2
    export VLANID=4011
    ip link add link $ROOTDEV name $ROOTDEV.$VLANID type vlan id $VLANID
    ip link set dev  $ROOTDEV.$VLANID up
    ip link set dev $ROOTDEV mtu 1508
    ip addr add 100.64.40.11/24 dev $ROOTDEV.$VLANID
    Load prog with ip tool:
    ip link set $ROOTDEV xdp off
    ip link set $ROOTDEV xdp object xdp_vlan01_kern.o section xdp_drop_vlan_4011
//
// Changing VLAN to zero, have same practical effect as removing the VLAN.
pub const TO_VLAN: c_int = 0;
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_vlan_change(ctx: *mut xdp_md) -> c_int {
    int xdp_vlan_change(struct xdp_md *ctx)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data     = (void *)(long)ctx.data;
    let mut pkt: parse_pkt = { 0 };
    if (!parse_eth_frame(data, data_end, &pkt))
    return XDP_ABORTED;
// Change specific VLAN ID
    if (pkt.vlan_outer == TESTVLAN) {
    struct _vlan_hdr *vlan_hdr = data + pkt.vlan_outer_offset;
// Modifying VLAN, preserve top 4 bits
    vlan_hdr.h_vlan_TCI =
    bpf_htons((bpf_ntohs(vlan_hdr.h_vlan_TCI) & 0xf000U)
    | TO_VLAN);
    }
    return XDP_PASS;
    }
//
// Show XDP+TC can cooperate, on creating a VLAN rewriter.
// 1. Create a XDP prog that can "pop"/remove a VLAN header.
// 2. Create a TC-bpf prog that egress can add a VLAN header.
//

    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_vlan_remove_outer(ctx: *mut xdp_md) -> c_int {
    int xdp_vlan_remove_outer(struct xdp_md *ctx)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data     = (void *)(long)ctx.data;
    let mut pkt: parse_pkt = { 0 };
    char *dest;
    if (!parse_eth_frame(data, data_end, &pkt))
    return XDP_ABORTED;
// Skip packet if no outer VLAN was detected
    if (pkt.vlan_outer_offset == 0)
    return XDP_PASS;
// Moving Ethernet header, dest overlap with src, memmove handle this
    dest = data;
    dest += VLAN_HDR_SZ;
//
// Notice: Taking over vlan_hdr->h_vlan_encapsulated_proto, by
// only moving two MAC addrs (12 bytes), not overwriting last 2 bytes
//
    __builtin_memmove(dest, data, ETH_ALEN * 2);
// Note: LLVM built-in memmove inlining require size to be constant
// Move start of packet header seen by Linux kernel stack
    bpf_xdp_adjust_head(ctx, VLAN_HDR_SZ);
    return XDP_PASS;
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn shift_mac_4bytes_32bit(data: *mut c_void) {
    void shift_mac_4bytes_32bit(void *data)
    {
    __u32 *p = data;
// Assuming VLAN hdr present. The 4 bytes in p[3] that gets
// overwritten, is ethhdr->h_proto and vlan_hdr->h_vlan_TCI.
// The vlan_hdr->h_vlan_encapsulated_proto take over role as
// ethhdr->h_proto.
//
    p[3] = p[2];
    p[2] = p[1];
    p[1] = p[0];
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_vlan_remove_outer2(ctx: *mut xdp_md) -> c_int {
    int xdp_vlan_remove_outer2(struct xdp_md *ctx)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data     = (void *)(long)ctx.data;
    struct ethhdr *orig_eth = data;
    let mut pkt: parse_pkt = { 0 };
    if (!parse_eth_frame(orig_eth, data_end, &pkt))
    return XDP_ABORTED;
// Skip packet if no outer VLAN was detected
    if (pkt.vlan_outer_offset == 0)
    return XDP_PASS;
// Simply shift down MAC addrs 4 bytes, overwrite h_proto + TCI
    shift_mac_4bytes_32bit(data);
// Move start of packet header seen by Linux kernel stack
    bpf_xdp_adjust_head(ctx, VLAN_HDR_SZ);
    return XDP_PASS;
    }
// =====================================
// BELOW: TC-hook based ebpf programs
// ====================================
// The TC-clsact eBPF programs (currently) need to be attach via TC commands
//
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_vlan_push(ctx: *mut __sk_buff) -> c_int {
    int tc_vlan_push(struct __sk_buff *ctx)
    {
    bpf_skb_vlan_push(ctx, bpf_htons(ETH_P_8021Q), TESTVLAN);
    return TC_ACT_OK;
    }
//
    Commands to setup TC to use above bpf prog:
    export ROOTDEV=ixgbe2
    export FILE=xdp_vlan01_kern.o

    tc qdisc del dev $ROOTDEV clsact 2> /dev/null ;\
    tc qdisc add dev $ROOTDEV clsact

    tc filter add dev $ROOTDEV egress \
    prio 1 handle 1 bpf da obj $FILE sec tc_vlan_push
    tc filter show dev $ROOTDEV egress
//
