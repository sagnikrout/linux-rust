//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/sw/rxe/rxe_icrc.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2016 Mellanox Technologies Ltd. All rights reserved.
// Copyright (c) 2015 System Fabric Works, Inc. All rights reserved.
//

//
// rxe_crc32() - Compute cumulative crc32 for a contiguous segment
// @rxe: rdma_rxe device object
// @crc: starting crc32 value from previous segments
// @next: starting address of current segment
// @len: length of current segment
//
// Return: the cumulative crc32 checksum
//
#[no_mangle]
unsafe extern "C" fn rxe_crc32(rxe: *mut rxe_dev, crc: __be32, next: *mut c_void, len: usize) -> __be32 {
    static __be32 rxe_crc32(struct rxe_dev *rxe, __be32 crc, void *next, size_t len)
    {
    return ( __be32)crc32_le(( u32)crc, next, len);
    }
//
// rxe_icrc_hdr() - Compute the partial ICRC for the network and transport
// headers of a packet.
// @skb: packet buffer
// @pkt: packet information
//
// Return: the partial ICRC
//
#[no_mangle]
unsafe extern "C" fn rxe_icrc_hdr(skb: *mut sk_buff, pkt: *mut rxe_pkt_info) -> __be32 {
    static __be32 rxe_icrc_hdr(struct sk_buff *skb, struct rxe_pkt_info *pkt)
    {
    let mut bth_offset: c_uint = 0;
    struct iphdr *ip4h = core::ptr::null_mut();
    struct ipv6hdr *ip6h = core::ptr::null_mut();
    struct udphdr *udph;
    struct rxe_bth *bth;
    __be32 crc;
    int length;
    int hdr_size = sizeof(struct udphdr) +
    (skb.protocol == htons(ETH_P_IP) ?
    sizeof(struct iphdr) : sizeof(struct ipv6hdr));
// pseudo header buffer size is calculate using ipv6 header size since
// it is bigger than ipv4
//
    u8 pshdr[sizeof(struct udphdr) +
    sizeof(struct ipv6hdr) +
    RXE_BTH_BYTES];
// This seed is the result of computing a CRC with a seed of
// 0xfffffff and 8 bytes of 0xff representing a masked LRH.
//
    crc = ( __be32)0xdebb20e3;
    if (skb.protocol == htons(ETH_P_IP)) { /* IPv4 */
    memcpy(pshdr, ip_hdr(skb), hdr_size);
    ip4h = (struct iphdr *)pshdr;
    udph = (struct udphdr *)(ip4h + 1);
    ip4h.ttl = 0xff;
    ip4h.check = CSUM_MANGLED_0;
    ip4h.tos = 0xff;
    } else {				/* IPv6 */
    memcpy(pshdr, ipv6_hdr(skb), hdr_size);
    ip6h = (struct ipv6hdr *)pshdr;
    udph = (struct udphdr *)(ip6h + 1);
    memset(ip6h.flow_lbl, 0xff, sizeof(ip6h.flow_lbl));
    ip6h.priority = 0xf;
    ip6h.hop_limit = 0xff;
    }
    udph.check = CSUM_MANGLED_0;
    bth_offset += hdr_size;
    memcpy(&pshdr[bth_offset], pkt.hdr, RXE_BTH_BYTES);
    bth = (struct rxe_bth *)&pshdr[bth_offset];
// exclude bth.resv8a
    bth.qpn |= cpu_to_be32(~BTH_QPN_MASK);
    length = hdr_size + RXE_BTH_BYTES;
    crc = rxe_crc32(pkt.rxe, crc, pshdr, length);
// And finish to compute the CRC on the remainder of the headers.
    crc = rxe_crc32(pkt.rxe, crc, pkt.hdr + RXE_BTH_BYTES,
    rxe_opcode[pkt.opcode].length - RXE_BTH_BYTES);
    return crc;
    }
//
// rxe_icrc_check() - Compute ICRC for a packet and compare to the ICRC
// delivered in the packet.
// @skb: packet buffer
// @pkt: packet information
//
// Return: 0 if the values match else an error
//
#[no_mangle]
pub unsafe extern "C" fn rxe_icrc_check(skb: *mut sk_buff, pkt: *mut rxe_pkt_info) -> c_int {
    int rxe_icrc_check(struct sk_buff *skb, struct rxe_pkt_info *pkt)
    {
    __be32 *icrcp;
    __be32 pkt_icrc;
    __be32 icrc;
    icrcp = (__be32 *)(pkt.hdr + pkt.paylen - RXE_ICRC_SIZE);
    pkt_icrc = *icrcp;
    icrc = rxe_icrc_hdr(skb, pkt);
    icrc = rxe_crc32(pkt.rxe, icrc, (u8 *)payload_addr(pkt),
    payload_size(pkt) + bth_pad(pkt));
    icrc = ~icrc;
    if (unlikely(icrc != pkt_icrc))
    return -EINVAL;
    return 0;
    }
//
// rxe_icrc_generate() - compute ICRC for a packet.
// @skb: packet buffer
// @pkt: packet information
//
#[no_mangle]
pub unsafe extern "C" fn rxe_icrc_generate(skb: *mut sk_buff, pkt: *mut rxe_pkt_info) {
    void rxe_icrc_generate(struct sk_buff *skb, struct rxe_pkt_info *pkt)
    {
    __be32 *icrcp;
    __be32 icrc;
    icrcp = (__be32 *)(pkt.hdr + pkt.paylen - RXE_ICRC_SIZE);
    icrc = rxe_icrc_hdr(skb, pkt);
    icrc = rxe_crc32(pkt.rxe, icrc, (u8 *)payload_addr(pkt),
    payload_size(pkt) + bth_pad(pkt));
// icrcp = ~icrc;
    }
