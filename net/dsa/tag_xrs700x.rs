//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_xrs700x.c
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
// XRS700x tag format handling
// Copyright (c) 2008-2009 Marvell Semiconductor
// Copyright (c) 2020 NovaTech LLC
//

    static struct sk_buff *xrs700x_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    u8 *trailer;
    trailer = skb_put(skb, 1);
    trailer[0] = dsa_xmit_port_mask(skb, dev);
    return skb;
    }
    static struct sk_buff *xrs700x_rcv(struct sk_buff *skb, struct net_device *dev)
    {
    int source_port;
    u8 *trailer;
    trailer = skb_tail_pointer(skb) - 1;
    source_port = ffs((int)trailer[0]) - 1;
    if (source_port < 0) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    skb.dev = dsa_conduit_find_user(dev, 0, source_port);
    if (!skb.dev) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    if (pskb_trim_rcsum(skb, skb.len - 1)) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// Frame is forwarded by hardware, don't forward in software.
    dsa_default_offload_fwd_mark(skb);
    return skb;
    }
    static const struct dsa_device_ops xrs700x_netdev_ops = {
    .name	= XRS700X_NAME,
    .proto	= DSA_TAG_PROTO_XRS700X,
    .xmit	= xrs700x_xmit,
    .rcv	= xrs700x_rcv,
    .needed_tailroom = 1,
    };
    MODULE_DESCRIPTION("DSA tag driver for XRS700x switches");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_XRS700X, XRS700X_NAME);
    module_dsa_tag_driver(xrs700x_netdev_ops);
