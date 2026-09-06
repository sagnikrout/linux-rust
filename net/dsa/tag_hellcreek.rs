//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_hellcreek.c
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// net/dsa/tag_hellcreek.c - Hirschmann Hellcreek switch tag format handling
//
// Copyright (C) 2019,2020 Linutronix GmbH
// Author Kurt Kanzenbach <kurt@linutronix.de>
//
// Based on tag_ksz.c.
//

pub const HELLCREEK_TAG_LEN: c_int = 1;
    static struct sk_buff *hellcreek_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    u8 *tag;
// Calculate checksums (if required) before adding the trailer tag to
// avoid including it in calculations. That would lead to wrong
// checksums after the switch strips the tag.
//
    if (skb.ip_summed == CHECKSUM_PARTIAL &&
    skb_checksum_help(skb)) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// Tag encoding
    tag  = skb_put(skb, HELLCREEK_TAG_LEN);
// tag = dsa_xmit_port_mask(skb, dev);
    return skb;
    }
    static struct sk_buff *hellcreek_rcv(struct sk_buff *skb,
    struct net_device *dev)
    {
// Tag decoding
    u8 *tag = skb_tail_pointer(skb) - HELLCREEK_TAG_LEN;
    let mut port: c_uint = tag[0] & 0x03;
    skb.dev = dsa_conduit_find_user(dev, 0, port);
    if (!skb.dev) {
    netdev_warn_once(dev, "Failed to get source port: %d\n", port);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    if (pskb_trim_rcsum(skb, skb.len - HELLCREEK_TAG_LEN)) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    dsa_default_offload_fwd_mark(skb);
    return skb;
    }
    static const struct dsa_device_ops hellcreek_netdev_ops = {
    .name	  = HELLCREEK_NAME,
    .proto	  = DSA_TAG_PROTO_HELLCREEK,
    .xmit	  = hellcreek_xmit,
    .rcv	  = hellcreek_rcv,
    .needed_tailroom = HELLCREEK_TAG_LEN,
    };
    MODULE_DESCRIPTION("DSA tag driver for Hirschmann Hellcreek TSN switches");
    MODULE_LICENSE("Dual MIT/GPL");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_HELLCREEK, HELLCREEK_NAME);
    module_dsa_tag_driver(hellcreek_netdev_ops);
