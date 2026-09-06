//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_mtk.c
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
// Mediatek DSA Tag support
// Copyright (C) 2017 Landen Chao <landen.chao@mediatek.com>
// Sean Wang <sean.wang@mediatek.com>
//

pub const MTK_HDR_LEN: c_int = 4;
pub const MTK_HDR_XMIT_UNTAGGED: c_int = 0;
pub const MTK_HDR_XMIT_TAGGED_TPID_8100: c_int = 1;
pub const MTK_HDR_XMIT_TAGGED_TPID_88A8: c_int = 2;

    static struct sk_buff *mtk_tag_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct dsa_port *dp = dsa_user_to_port(dev);
    u8 xmit_tpid;
    u8 *mtk_tag;
    skb_set_queue_mapping(skb, dp.index);
// Build the special tag after the MAC Source Address. If VLAN header
// is present, it's required that VLAN header and special tag is
// being combined. Only in this way we can allow the switch can parse
// the both special and VLAN tag at the same time and then look up VLAN
// table with VID.
//
    switch (skb.protocol) {
    case htons(ETH_P_8021Q):
    xmit_tpid = MTK_HDR_XMIT_TAGGED_TPID_8100;
    break;
    case htons(ETH_P_8021AD):
    xmit_tpid = MTK_HDR_XMIT_TAGGED_TPID_88A8;
    break;
    default:
    xmit_tpid = MTK_HDR_XMIT_UNTAGGED;
    skb_push(skb, MTK_HDR_LEN);
    dsa_alloc_etype_header(skb, MTK_HDR_LEN);
    }
    mtk_tag = dsa_etype_header_pos_tx(skb);
// Mark tag attribute on special tag insertion to notify hardware
// whether that's a combined special tag with 802.1Q header.
//
    mtk_tag[0] = xmit_tpid;
    mtk_tag[1] = FIELD_PREP(MTK_HDR_XMIT_DP_BIT_MASK,
    dsa_xmit_port_mask(skb, dev));
// Tag control information is kept for 802.1Q
    if (xmit_tpid == MTK_HDR_XMIT_UNTAGGED) {
    mtk_tag[2] = 0;
    mtk_tag[3] = 0;
    }
    return skb;
    }
    static struct sk_buff *mtk_tag_rcv(struct sk_buff *skb, struct net_device *dev)
    {
    u16 hdr;
    int port;
    __be16 *phdr;
    if (unlikely(!pskb_may_pull(skb, MTK_HDR_LEN))) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    phdr = dsa_etype_header_pos_rx(skb);
    hdr = ntohs(*phdr);
// Remove MTK tag and recalculate checksum.
    skb_pull_rcsum(skb, MTK_HDR_LEN);
    dsa_strip_etype_header(skb, MTK_HDR_LEN);
// Get source port information
    port = (hdr & MTK_HDR_RECV_SOURCE_PORT_MASK);
    skb.dev = dsa_conduit_find_user(dev, 0, port);
    if (!skb.dev) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    dsa_default_offload_fwd_mark(skb);
    return skb;
    }
    static const struct dsa_device_ops mtk_netdev_ops = {
    .name		= MTK_NAME,
    .proto		= DSA_TAG_PROTO_MTK,
    .xmit		= mtk_tag_xmit,
    .rcv		= mtk_tag_rcv,
    .needed_headroom = MTK_HDR_LEN,
    };
    MODULE_DESCRIPTION("DSA tag driver for Mediatek switches");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_MTK, MTK_NAME);
    module_dsa_tag_driver(mtk_netdev_ops);
