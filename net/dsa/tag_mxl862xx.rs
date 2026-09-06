//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_mxl862xx.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// DSA Special Tag for MaxLinear 862xx switch chips
//
// Copyright (C) 2025 Daniel Golle <daniel@makrotopia.org>
// Copyright (C) 2024 MaxLinear Inc.
//

pub const MXL862_HEADER_LEN: c_int = 8;
// Word 0 -> EtherType
// Word 2

// Word 3

    static struct sk_buff *mxl862_tag_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct dsa_port *dp = dsa_user_to_port(dev);
    struct dsa_port *cpu_dp = dp.cpu_dp;
    unsigned int cpu_port, sub_interface;
    __be16 *mxl862_tag;
    cpu_port = cpu_dp.index;
// target port sub-interface ID relative to the CPU port
    sub_interface = dp.index + 16 - cpu_port;
// provide additional space 'MXL862_HEADER_LEN' bytes
    skb_push(skb, MXL862_HEADER_LEN);
// shift MAC address to the beginning of the enlarged buffer,
// releasing the space required for DSA tag (between MAC address and
// Ethertype)
//
    dsa_alloc_etype_header(skb, MXL862_HEADER_LEN);
// special tag ingress (from the perspective of the switch)
    mxl862_tag = dsa_etype_header_pos_tx(skb);
    mxl862_tag[0] = htons(ETH_P_MXLGSW);
    mxl862_tag[1] = 0;
    mxl862_tag[2] = htons(FIELD_PREP(MXL862_SUBIF_ID, sub_interface));
    mxl862_tag[3] = htons(FIELD_PREP(MXL862_IGP_EGP, cpu_port));
    return skb;
    }
    static struct sk_buff *mxl862_tag_rcv(struct sk_buff *skb,
    struct net_device *dev)
    {
    __be16 *mxl862_tag;
    int port;
    if (unlikely(!pskb_may_pull(skb, MXL862_HEADER_LEN))) {
    dev_warn_ratelimited(&dev.dev, "Cannot pull SKB, packet dropped\n");
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    mxl862_tag = dsa_etype_header_pos_rx(skb);
    if (unlikely(mxl862_tag[0] != htons(ETH_P_MXLGSW))) {
    dev_warn_ratelimited(&dev.dev,
    "Invalid special tag marker, packet dropped, tag: %8ph\n",
    mxl862_tag);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// Get source port information
    port = FIELD_GET(MXL862_IGP_EGP, ntohs(mxl862_tag[3]));
    skb.dev = dsa_conduit_find_user(dev, 0, port);
    if (unlikely(!skb.dev)) {
    dev_warn_ratelimited(&dev.dev,
    "Invalid source port, packet dropped, tag: %8ph\n",
    mxl862_tag);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    if (likely(!is_link_local_ether_addr(eth_hdr(skb).h_dest)))
    dsa_default_offload_fwd_mark(skb);
// remove the MxL862xx special tag between the MAC addresses and the
// current ethertype field.
//
    skb_pull_rcsum(skb, MXL862_HEADER_LEN);
    dsa_strip_etype_header(skb, MXL862_HEADER_LEN);
    return skb;
    }
    static const struct dsa_device_ops mxl862_netdev_ops = {
    .name = MXL862_NAME,
    .proto = DSA_TAG_PROTO_MXL862,
    .xmit = mxl862_tag_xmit,
    .rcv = mxl862_tag_rcv,
    .needed_headroom = MXL862_HEADER_LEN,
    };
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_MXL862, MXL862_NAME);
    MODULE_DESCRIPTION("DSA tag driver for MaxLinear MxL862xx switches");
    MODULE_LICENSE("GPL");
    module_dsa_tag_driver(mxl862_netdev_ops);
