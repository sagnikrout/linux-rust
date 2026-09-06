//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_mxl-gsw1xx.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// DSA driver Special Tag support for MaxLinear GSW1xx switch chips
//
// Copyright (C) 2025 Daniel Golle <daniel@makrotopia.org>
// Copyright (C) 2023 - 2024 MaxLinear Inc.
//

// To define the outgoing port and to discover the incoming port a special
// tag is used by the GSW1xx.
//
// Dest MAC       Src MAC    special TAG        EtherType
// ...| 1 2 3 4 5 6 | 1 2 3 4 5 6 | 1 2 3 4 5 6 7 8 | 1 2 |...
// |<--------------->|
//

// special tag header length (RX and TX)
pub const GSW1XX_HEADER_LEN: c_int = 8;
// Word 0 = Ethertype -> 0x88C3
// Word 1

// special tag in RX path header
// Word 2

    static struct sk_buff *gsw1xx_tag_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    __be16 *gsw1xx_tag;
    u16 tag;
// provide additional space 'GSW1XX_HEADER_LEN' bytes
    skb_push(skb, GSW1XX_HEADER_LEN);
// add space between MAC address and Ethertype
    dsa_alloc_etype_header(skb, GSW1XX_HEADER_LEN);
// special tag ingress
    gsw1xx_tag = dsa_etype_header_pos_tx(skb);
    gsw1xx_tag[0] = htons(ETH_P_MXLGSW);
    tag = FIELD_PREP(GSW1XX_TX_PORT_MAP, dsa_xmit_port_mask(skb, dev)) |
    GSW1XX_TX_PORT_MAP_EN | GSW1XX_TX_LRN_DIS;
    gsw1xx_tag[1] = htons(tag);
    gsw1xx_tag[2] = 0;
    gsw1xx_tag[3] = 0;
    return skb;
    }
    static struct sk_buff *gsw1xx_tag_rcv(struct sk_buff *skb,
    struct net_device *dev)
    {
    int port;
    __be16 *gsw1xx_tag;
    if (unlikely(!pskb_may_pull(skb, GSW1XX_HEADER_LEN))) {
    dev_warn_ratelimited(&dev.dev, "Dropping packet, cannot pull SKB\n");
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    gsw1xx_tag = dsa_etype_header_pos_rx(skb);
    if (unlikely(ntohs(gsw1xx_tag[0]) != ETH_P_MXLGSW)) {
    dev_warn_ratelimited(&dev.dev, "Dropping packet due to invalid special tag\n");
    dev_warn_ratelimited(&dev.dev, "Tag: %8ph\n", gsw1xx_tag);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// Get source port information
    port = FIELD_GET(GSW1XX_RX_PORT_MAP, ntohs(gsw1xx_tag[1]));
    skb.dev = dsa_conduit_find_user(dev, 0, port);
    if (!skb.dev) {
    dev_warn_ratelimited(&dev.dev, "Dropping packet due to invalid source port\n");
    dev_warn_ratelimited(&dev.dev, "Tag: %8ph\n", gsw1xx_tag);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// remove the GSW1xx special tag between MAC addresses and the current
// ethertype field.
//
    skb_pull_rcsum(skb, GSW1XX_HEADER_LEN);
    dsa_strip_etype_header(skb, GSW1XX_HEADER_LEN);
    return skb;
    }
    static const struct dsa_device_ops gsw1xx_netdev_ops = {
    .name			= GSW1XX_TAG_NAME,
    .proto			= DSA_TAG_PROTO_MXL_GSW1XX,
    .xmit			= gsw1xx_tag_xmit,
    .rcv			= gsw1xx_tag_rcv,
    .needed_headroom	= GSW1XX_HEADER_LEN,
    };
    MODULE_DESCRIPTION("DSA tag driver for MaxLinear GSW1xx 8 byte protocol");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_MXL_GSW1XX, GSW1XX_TAG_NAME);
    module_dsa_tag_driver(gsw1xx_netdev_ops);
