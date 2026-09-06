//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_gswip.c
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
// Intel / Lantiq GSWIP V2.0 PMAC tag support
//
// Copyright (C) 2017 - 2018 Hauke Mehrtens <hauke@hauke-m.de>
//

pub const GSWIP_TX_HEADER_LEN: c_int = 4;
// special tag in TX path header
// Byte 0

pub const GSWIP_TX_SLPID_CPU: c_int = 2;
pub const GSWIP_TX_SLPID_APP1: c_int = 3;
pub const GSWIP_TX_SLPID_APP2: c_int = 4;
pub const GSWIP_TX_SLPID_APP3: c_int = 5;
pub const GSWIP_TX_SLPID_APP4: c_int = 6;
pub const GSWIP_TX_SLPID_APP5: c_int = 7;
// Byte 1

pub const GSWIP_TX_DPID_ELAN: c_int = 0;
pub const GSWIP_TX_DPID_EWAN: c_int = 1;
pub const GSWIP_TX_DPID_CPU: c_int = 2;
pub const GSWIP_TX_DPID_APP1: c_int = 3;
pub const GSWIP_TX_DPID_APP2: c_int = 4;
pub const GSWIP_TX_DPID_APP3: c_int = 5;
pub const GSWIP_TX_DPID_APP4: c_int = 6;
pub const GSWIP_TX_DPID_APP5: c_int = 7;
// Byte 2

pub const GSWIP_TX_CLASS_SHIFT: c_int = 0;

// Byte 3

pub const GSWIP_RX_HEADER_LEN: c_int = 8;
// special tag in RX path header
// Byte 7
pub const GSWIP_RX_SPPID_SHIFT: c_int = 4;

    static struct sk_buff *gswip_tag_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    u8 *gswip_tag;
    skb_push(skb, GSWIP_TX_HEADER_LEN);
    gswip_tag = skb.data;
    gswip_tag[0] = GSWIP_TX_SLPID_CPU;
    gswip_tag[1] = GSWIP_TX_DPID_ELAN;
    gswip_tag[2] = GSWIP_TX_PORT_MAP_EN | GSWIP_TX_PORT_MAP_SEL;
    gswip_tag[3] = FIELD_PREP(GSWIP_TX_PORT_MAP, dsa_xmit_port_mask(skb, dev));
    gswip_tag[3] |= GSWIP_TX_DPID_EN;
    return skb;
    }
    static struct sk_buff *gswip_tag_rcv(struct sk_buff *skb,
    struct net_device *dev)
    {
    int port;
    u8 *gswip_tag;
    if (unlikely(!pskb_may_pull(skb, GSWIP_RX_HEADER_LEN))) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    gswip_tag = skb.data - ETH_HLEN;
// Get source port information
    port = (gswip_tag[7] & GSWIP_RX_SPPID_MASK) >> GSWIP_RX_SPPID_SHIFT;
    skb.dev = dsa_conduit_find_user(dev, 0, port);
    if (!skb.dev) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// remove GSWIP tag
    skb_pull_rcsum(skb, GSWIP_RX_HEADER_LEN);
    return skb;
    }
    static const struct dsa_device_ops gswip_netdev_ops = {
    .name = GSWIP_NAME,
    .proto	= DSA_TAG_PROTO_GSWIP,
    .xmit = gswip_tag_xmit,
    .rcv = gswip_tag_rcv,
    .needed_headroom = GSWIP_RX_HEADER_LEN,
    };
    MODULE_DESCRIPTION("DSA tag driver for Lantiq / Intel GSWIP switches");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_GSWIP, GSWIP_NAME);
    module_dsa_tag_driver(gswip_netdev_ops);
