//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_rzn1_a5psw.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2022 Schneider Electric
//
// Clément Léger <clement.leger@bootlin.com>
//

// To define the outgoing port and to discover the incoming port a TAG is
// inserted after Src MAC :
//
// Dest MAC       Src MAC           TAG         Type
// ...| 1 2 3 4 5 6 | 1 2 3 4 5 6 | 1 2 3 4 5 6 7 8 | 1 2 |...
// |<--------------->|
//
// See struct a5psw_tag for layout
//

pub const ETH_P_DSA_A5PSW: c_uint = 0xE001;
pub const A5PSW_TAG_LEN: c_int = 8;

// This is both used for xmit tag and rcv tagging

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a5psw_tag {
    pub ctrl_tag: __be16,
    pub ctrl_data: __be16,
    pub ctrl_data2_hi: __be16,
    pub ctrl_data2_lo: __be16,
}

    static struct sk_buff *a5psw_tag_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct a5psw_tag *ptag;
    u32 data2_val;
    BUILD_BUG_ON(sizeof(*ptag) != A5PSW_TAG_LEN);
// The Ethernet switch we are interfaced with needs packets to be at
// least 60 bytes otherwise they will be discarded when they enter the
// switch port logic.
//
    if (eth_skb_pad(skb))
    return core::ptr::null_mut();
// provide 'A5PSW_TAG_LEN' bytes additional space
    skb_push(skb, A5PSW_TAG_LEN);
// make room between MACs and Ether-Type to insert tag
    dsa_alloc_etype_header(skb, A5PSW_TAG_LEN);
    ptag = dsa_etype_header_pos_tx(skb);
    data2_val = FIELD_PREP(A5PSW_CTRL_DATA_PORT, dsa_xmit_port_mask(skb, dev));
    ptag.ctrl_tag = htons(ETH_P_DSA_A5PSW);
    ptag.ctrl_data = htons(A5PSW_CTRL_DATA_FORCE_FORWARD);
    ptag.ctrl_data2_lo = htons(data2_val);
    ptag.ctrl_data2_hi = 0;
    return skb;
    }
    static struct sk_buff *a5psw_tag_rcv(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct a5psw_tag *tag;
    int port;
    if (unlikely(!pskb_may_pull(skb, A5PSW_TAG_LEN))) {
    dev_warn_ratelimited(&dev.dev,
    "Dropping packet, cannot pull\n");
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    tag = dsa_etype_header_pos_rx(skb);
    if (tag.ctrl_tag != htons(ETH_P_DSA_A5PSW)) {
    dev_warn_ratelimited(&dev.dev, "Dropping packet due to invalid TAG marker\n");
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    port = FIELD_GET(A5PSW_CTRL_DATA_PORT, ntohs(tag.ctrl_data));
    skb.dev = dsa_conduit_find_user(dev, 0, port);
    if (!skb.dev) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    skb_pull_rcsum(skb, A5PSW_TAG_LEN);
    dsa_strip_etype_header(skb, A5PSW_TAG_LEN);
    dsa_default_offload_fwd_mark(skb);
    return skb;
    }
    static const struct dsa_device_ops a5psw_netdev_ops = {
    .name	= A5PSW_NAME,
    .proto	= DSA_TAG_PROTO_RZN1_A5PSW,
    .xmit	= a5psw_tag_xmit,
    .rcv	= a5psw_tag_rcv,
    .needed_headroom = A5PSW_TAG_LEN,
    };
    MODULE_DESCRIPTION("DSA tag driver for Renesas RZ/N1 A5PSW switch");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_A5PSW, A5PSW_NAME);
    module_dsa_tag_driver(a5psw_netdev_ops);
