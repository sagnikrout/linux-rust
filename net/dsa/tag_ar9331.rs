//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_ar9331.c
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
// Copyright (c) 2019 Pengutronix, Oleksij Rempel <kernel@pengutronix.de>
//

pub const AR9331_HDR_LEN: c_int = 2;
pub const AR9331_HDR_VERSION: c_int = 1;

// AR9331_HDR_RESERVED - not used or may be version field.
// According to the AR8216 doc it should 0b10. On AR9331 it is 0b11 on RX path
// and should be set to 0b11 to make it work.
//

    static struct sk_buff *ar9331_tag_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct dsa_port *dp = dsa_user_to_port(dev);
    __le16 *phdr;
    u16 hdr;
    phdr = skb_push(skb, AR9331_HDR_LEN);
    hdr = FIELD_PREP(AR9331_HDR_VERSION_MASK, AR9331_HDR_VERSION);
    hdr |= AR9331_HDR_FROM_CPU | dp.index;
// 0b10 for AR8216 and 0b11 for AR9331
    hdr |= AR9331_HDR_RESERVED_MASK;
    phdr[0] = cpu_to_le16(hdr);
    return skb;
    }
    static struct sk_buff *ar9331_tag_rcv(struct sk_buff *skb,
    struct net_device *ndev)
    {
    u8 ver, port;
    u16 hdr;
    if (unlikely(!pskb_may_pull(skb, AR9331_HDR_LEN))) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    hdr = le16_to_cpu(*(__le16 *)skb_mac_header(skb));
    ver = FIELD_GET(AR9331_HDR_VERSION_MASK, hdr);
    if (unlikely(ver != AR9331_HDR_VERSION)) {
    netdev_warn_once(ndev, "%s:%i wrong header version 0x%2x\n",
    __func__, __LINE__, hdr);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    if (unlikely(hdr & AR9331_HDR_FROM_CPU)) {
    netdev_warn_once(ndev, "%s:%i packet should not be from cpu 0x%2x\n",
    __func__, __LINE__, hdr);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    skb_pull_rcsum(skb, AR9331_HDR_LEN);
// Get source port information
    port = FIELD_GET(AR9331_HDR_PORT_NUM_MASK, hdr);
    skb.dev = dsa_conduit_find_user(ndev, 0, port);
    if (!skb.dev) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    return skb;
    }
    static const struct dsa_device_ops ar9331_netdev_ops = {
    .name	= AR9331_NAME,
    .proto	= DSA_TAG_PROTO_AR9331,
    .xmit	= ar9331_tag_xmit,
    .rcv	= ar9331_tag_rcv,
    .needed_headroom = AR9331_HDR_LEN,
    };
    MODULE_DESCRIPTION("DSA tag driver for Atheros AR9331 SoC with built-in switch");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_AR9331, AR9331_NAME);
    module_dsa_tag_driver(ar9331_netdev_ops);
