//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_trailer.c
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
// net/dsa/tag_trailer.c - Trailer tag format handling
// Copyright (c) 2008-2009 Marvell Semiconductor
//

    static struct sk_buff *trailer_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    u8 *trailer;
    trailer = skb_put(skb, 4);
    trailer[0] = 0x80;
    trailer[1] = dsa_xmit_port_mask(skb, dev);
    trailer[2] = 0x10;
    trailer[3] = 0x00;
    return skb;
    }
    static struct sk_buff *trailer_rcv(struct sk_buff *skb, struct net_device *dev)
    {
    u8 *trailer;
    int source_port;
    if (skb_linearize(skb)) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    trailer = skb_tail_pointer(skb) - 4;
    if (trailer[0] != 0x80 || (trailer[1] & 0xf8) != 0x00 ||
    (trailer[2] & 0xef) != 0x00 || trailer[3] != 0x00) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    source_port = trailer[1] & 7;
    skb.dev = dsa_conduit_find_user(dev, 0, source_port);
    if (!skb.dev) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    if (pskb_trim_rcsum(skb, skb.len - 4)) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    return skb;
    }
    static const struct dsa_device_ops trailer_netdev_ops = {
    .name	= TRAILER_NAME,
    .proto	= DSA_TAG_PROTO_TRAILER,
    .xmit	= trailer_xmit,
    .rcv	= trailer_rcv,
    .needed_tailroom = 4,
    };
    MODULE_DESCRIPTION("DSA tag driver for switches using a trailer tag");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_TRAILER, TRAILER_NAME);
    module_dsa_tag_driver(trailer_netdev_ops);
