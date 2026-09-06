//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_rtl4_a.c
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
//
// Handler for Realtek 4 byte DSA switch tags
// Currently only supports protocol "A" found in RTL8366RB
// Copyright (c) 2020 Linus Walleij <linus.walleij@linaro.org>
//
// This "proprietary tag" header looks like so:
//
// -------------------------------------------------
// | MAC DA | MAC SA | 0x8899 | 2 bytes tag | Type |
// -------------------------------------------------
//
// The 2 bytes tag form a 16 bit big endian word. The exact
// meaning has been guessed from packet dumps from ingress
// frames.
//

pub const RTL4_A_HDR_LEN: c_int = 4;
pub const RTL4_A_PROTOCOL_SHIFT: c_int = 12;
//
// 0x1 = Realtek Remote Control protocol (RRCP)
// 0x2/0x3 seems to be used for loopback testing
// 0x9 = RTL8306 DSA protocol
// 0xa = RTL8366RB DSA protocol
//
pub const RTL4_A_PROTOCOL_RTL8366RB: c_uint = 0xa;
    static struct sk_buff *rtl4a_tag_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct dsa_port *dp = dsa_user_to_port(dev);
    __be16 *p;
    u8 *tag;
    u16 out;
// Pad out to at least 60 bytes
    if (unlikely(eth_skb_pad(skb)))
    return core::ptr::null_mut();
    netdev_dbg(dev, "add realtek tag to package to port %d\n",
    dp.index);
    skb_push(skb, RTL4_A_HDR_LEN);
    dsa_alloc_etype_header(skb, RTL4_A_HDR_LEN);
    tag = dsa_etype_header_pos_tx(skb);
// Set Ethertype
    p = (__be16 *)tag;
// p = htons(ETH_P_REALTEK);
    out = (RTL4_A_PROTOCOL_RTL8366RB << RTL4_A_PROTOCOL_SHIFT);
// The lower bits indicate the port number
    out |= dsa_xmit_port_mask(skb, dev);
    p = (__be16 *)(tag + 2);
// p = htons(out);
    return skb;
    }
    static struct sk_buff *rtl4a_tag_rcv(struct sk_buff *skb,
    struct net_device *dev)
    {
    u16 protport;
    __be16 *p;
    u16 etype;
    u8 *tag;
    u8 prot;
    u8 port;
    if (unlikely(!pskb_may_pull(skb, RTL4_A_HDR_LEN))) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    tag = dsa_etype_header_pos_rx(skb);
    p = (__be16 *)tag;
    etype = ntohs(*p);
    if (etype != ETH_P_REALTEK) {
// Not custom, just pass through
    netdev_dbg(dev, "non-realtek ethertype 0x%04x\n", etype);
    return skb;
    }
    p = (__be16 *)(tag + 2);
    protport = ntohs(*p);
// The 4 upper bits are the protocol
    prot = (protport >> RTL4_A_PROTOCOL_SHIFT) & 0x0f;
    if (prot != RTL4_A_PROTOCOL_RTL8366RB) {
    netdev_err(dev, "unknown realtek protocol 0x%01x\n", prot);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    port = protport & 0xff;
    skb.dev = dsa_conduit_find_user(dev, 0, port);
    if (!skb.dev) {
    netdev_dbg(dev, "could not find user for port %d\n", port);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// Remove RTL4 tag and recalculate checksum
    skb_pull_rcsum(skb, RTL4_A_HDR_LEN);
    dsa_strip_etype_header(skb, RTL4_A_HDR_LEN);
    dsa_default_offload_fwd_mark(skb);
    return skb;
    }
    static const struct dsa_device_ops rtl4a_netdev_ops = {
    .name	= RTL4_A_NAME,
    .proto	= DSA_TAG_PROTO_RTL4_A,
    .xmit	= rtl4a_tag_xmit,
    .rcv	= rtl4a_tag_rcv,
    .needed_headroom = RTL4_A_HDR_LEN,
    };
    module_dsa_tag_driver(rtl4a_netdev_ops);
    MODULE_DESCRIPTION("DSA tag driver for Realtek 4 byte protocol A tags");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_RTL4_A, RTL4_A_NAME);
