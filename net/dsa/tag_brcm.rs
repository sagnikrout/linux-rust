//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_brcm.c
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
// Broadcom tag support
//
// Copyright (C) 2014 Broadcom Corporation
//

// Legacy Broadcom tag (6 bytes)
pub const BRCM_LEG_TAG_LEN: c_int = 6;
// Type fields
// 1st byte in the tag
pub const BRCM_LEG_TYPE_HI: c_uint = 0x88;
// 2nd byte in the tag
pub const BRCM_LEG_TYPE_LO: c_uint = 0x74;
// Tag fields
// 3rd byte in the tag

// 4th byte in the tag

// 6th byte in the tag

// Newer Broadcom tag (4 bytes)
pub const BRCM_TAG_LEN: c_int = 4;
// Tag is constructed and deconstructed using byte by byte access
// because the tag is placed after the MAC Source Address, which does
// not make it 4-bytes aligned, so this might cause unaligned accesses
// on most systems where this is used.
//
// Ingress and egress opcodes
pub const BRCM_OPCODE_SHIFT: c_int = 5;
pub const BRCM_OPCODE_MASK: c_uint = 0x7;
// Ingress fields
// 1st byte in the tag
pub const BRCM_IG_TC_SHIFT: c_int = 2;
pub const BRCM_IG_TC_MASK: c_uint = 0x7;
// 2nd byte in the tag
pub const BRCM_IG_TE_MASK: c_uint = 0x3;
pub const BRCM_IG_TS_SHIFT: c_int = 7;
// 3rd byte in the tag
pub const BRCM_IG_DSTMAP2_MASK: c_int = 1;
pub const BRCM_IG_DSTMAP1_MASK: c_uint = 0xff;
// Egress fields
// 2nd byte in the tag
pub const BRCM_EG_CID_MASK: c_uint = 0xff;
// 3rd byte in the tag
pub const BRCM_EG_RC_MASK: c_uint = 0xff;

pub const BRCM_EG_TC_SHIFT: c_int = 5;
pub const BRCM_EG_TC_MASK: c_uint = 0x7;
pub const BRCM_EG_PID_MASK: c_uint = 0x1f;

    IS_ENABLED(CONFIG_NET_DSA_TAG_BRCM_PREPEND)
    static struct sk_buff *brcm_tag_xmit_ll(struct sk_buff *skb,
    struct net_device *dev,
    unsigned int offset)
    {
    struct dsa_port *dp = dsa_user_to_port(dev);
    let mut queue: u16 = skb_get_queue_mapping(skb);
    u16 port_mask;
    u8 *brcm_tag;
// The Ethernet switch we are interfaced with needs packets to be at
// least 64 bytes (including FCS) otherwise they will be discarded when
// they enter the switch port logic. When Broadcom tags are enabled, we
// need to make sure that packets are at least 68 bytes
// (including FCS and tag) because the length verification is done after
// the Broadcom tag is stripped off the ingress packet.
//
// Free the SKB on error.
//
    if (skb_put_padto(skb, ETH_ZLEN + BRCM_TAG_LEN))
    return core::ptr::null_mut();
    skb_push(skb, BRCM_TAG_LEN);
    if (offset)
    dsa_alloc_etype_header(skb, BRCM_TAG_LEN);
    brcm_tag = skb.data + offset;
// Set the ingress opcode, traffic class, tag enforcement is
// deprecated
//
    brcm_tag[0] = (1 << BRCM_OPCODE_SHIFT) |
    ((queue & BRCM_IG_TC_MASK) << BRCM_IG_TC_SHIFT);
    brcm_tag[1] = 0;
    port_mask = dsa_xmit_port_mask(skb, dev);
    brcm_tag[2] = (port_mask >> 8) & BRCM_IG_DSTMAP2_MASK;
    brcm_tag[3] = port_mask & BRCM_IG_DSTMAP1_MASK;
// Now tell the conduit network device about the desired output queue
// as well
//
    skb_set_queue_mapping(skb, BRCM_TAG_SET_PORT_QUEUE(dp.index, queue));
    return skb;
    }
// Frames with this tag have one of these two layouts:
// -----------------------------------
// | MAC DA | MAC SA | 4b tag | Type | DSA_TAG_PROTO_BRCM
// -----------------------------------
// | 4b tag | MAC DA | MAC SA | Type | DSA_TAG_PROTO_BRCM_PREPEND
// -----------------------------------
// In both cases, at receive time, skb->data points 2 bytes before the actual
// Ethernet type field and we have an offset of 4bytes between where skb->data
// and where the payload starts. So the same low-level receive function can be
// used.
//
    static struct sk_buff *brcm_tag_rcv_ll(struct sk_buff *skb,
    struct net_device *dev,
    unsigned int offset)
    {
    int source_port;
    u8 *brcm_tag;
    if (unlikely(!pskb_may_pull(skb, BRCM_TAG_LEN))) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    brcm_tag = skb.data - offset;
// The opcode should never be different than 0b000
    if (unlikely((brcm_tag[0] >> BRCM_OPCODE_SHIFT) & BRCM_OPCODE_MASK)) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// We should never see a reserved reason code without knowing how to
// handle it
//
    if (unlikely(brcm_tag[2] & BRCM_EG_RC_RSVD)) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// Locate which port this is coming from
    source_port = brcm_tag[3] & BRCM_EG_PID_MASK;
    skb.dev = dsa_conduit_find_user(dev, 0, source_port);
    if (!skb.dev) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// Remove Broadcom tag and update checksum
    skb_pull_rcsum(skb, BRCM_TAG_LEN);
    if (likely(!is_link_local_ether_addr(eth_hdr(skb).h_dest)))
    dsa_default_offload_fwd_mark(skb);
    return skb;
    }

    static struct sk_buff *brcm_tag_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
// Build the tag after the MAC Source Address
    return brcm_tag_xmit_ll(skb, dev, 2 * ETH_ALEN);
    }
    static struct sk_buff *brcm_tag_rcv(struct sk_buff *skb, struct net_device *dev)
    {
    struct sk_buff *nskb;
// skb->data points to the EtherType, the tag is right before it
    nskb = brcm_tag_rcv_ll(skb, dev, 2);
    if (!nskb)
    return nskb;
    dsa_strip_etype_header(skb, BRCM_TAG_LEN);
    return nskb;
    }
    static const struct dsa_device_ops brcm_netdev_ops = {
    .name	= BRCM_NAME,
    .proto	= DSA_TAG_PROTO_BRCM,
    .xmit	= brcm_tag_xmit,
    .rcv	= brcm_tag_rcv,
    .needed_headroom = BRCM_TAG_LEN,
    };
    DSA_TAG_DRIVER(brcm_netdev_ops);
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_BRCM, BRCM_NAME);

    IS_ENABLED(CONFIG_NET_DSA_TAG_BRCM_LEGACY_FCS)
    static struct sk_buff *brcm_leg_tag_rcv(struct sk_buff *skb,
    struct net_device *dev)
    {
    let mut len: c_int = BRCM_LEG_TAG_LEN;
    int source_port;
    __be16 *proto;
    u8 *brcm_tag;
    if (unlikely(!pskb_may_pull(skb, BRCM_LEG_TAG_LEN + VLAN_HLEN))) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    brcm_tag = dsa_etype_header_pos_rx(skb);
    proto = (__be16 *)(brcm_tag + BRCM_LEG_TAG_LEN);
    source_port = brcm_tag[5] & BRCM_LEG_PORT_ID;
    skb.dev = dsa_conduit_find_user(dev, 0, source_port);
    if (!skb.dev) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// The internal switch in BCM63XX SoCs always tags on egress on the CPU
// port. We use VID 0 internally for untagged traffic, so strip the tag
// if the TCI field is all 0, and keep it otherwise to also retain
// e.g. 802.1p tagged packets.
//
    if (proto[0] == htons(ETH_P_8021Q) && proto[1] == 0)
    len += VLAN_HLEN;
// Remove Broadcom tag and update checksum
    skb_pull_rcsum(skb, len);
    if (likely(!is_link_local_ether_addr(eth_hdr(skb).h_dest)))
    dsa_default_offload_fwd_mark(skb);
    dsa_strip_etype_header(skb, len);
    return skb;
    }

    static struct sk_buff *brcm_leg_tag_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct dsa_port *dp = dsa_user_to_port(dev);
    u8 *brcm_tag;
// The Ethernet switch we are interfaced with needs packets to be at
// least 64 bytes (including FCS) otherwise they will be discarded when
// they enter the switch port logic. When Broadcom tags are enabled, we
// need to make sure that packets are at least 70 bytes
// (including FCS and tag) because the length verification is done after
// the Broadcom tag is stripped off the ingress packet.
//
    if (skb_put_padto(skb, ETH_ZLEN + BRCM_LEG_TAG_LEN))
    return core::ptr::null_mut();
    skb_push(skb, BRCM_LEG_TAG_LEN);
    dsa_alloc_etype_header(skb, BRCM_LEG_TAG_LEN);
    brcm_tag = skb.data + 2 * ETH_ALEN;
// Broadcom tag type
    brcm_tag[0] = BRCM_LEG_TYPE_HI;
    brcm_tag[1] = BRCM_LEG_TYPE_LO;
// Broadcom tag value
    brcm_tag[2] = BRCM_LEG_EGRESS;
    brcm_tag[3] = 0;
    brcm_tag[4] = 0;
    brcm_tag[5] = dp.index & BRCM_LEG_PORT_ID;
    return skb;
    }
    static const struct dsa_device_ops brcm_legacy_netdev_ops = {
    .name = BRCM_LEGACY_NAME,
    .proto = DSA_TAG_PROTO_BRCM_LEGACY,
    .xmit = brcm_leg_tag_xmit,
    .rcv = brcm_leg_tag_rcv,
    .needed_headroom = BRCM_LEG_TAG_LEN,
    };
    DSA_TAG_DRIVER(brcm_legacy_netdev_ops);
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_BRCM_LEGACY, BRCM_LEGACY_NAME);

    static struct sk_buff *brcm_leg_fcs_tag_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct dsa_port *dp = dsa_user_to_port(dev);
    unsigned int fcs_len;
    __le32 fcs_val;
    u8 *brcm_tag;
// The Ethernet switch we are interfaced with needs packets to be at
// least 64 bytes (including FCS) otherwise they will be discarded when
// they enter the switch port logic. When Broadcom tags are enabled, we
// need to make sure that packets are at least 70 bytes (including FCS
// and tag) because the length verification is done after the Broadcom
// tag is stripped off the ingress packet.
//
    if (skb_put_padto(skb, ETH_ZLEN + BRCM_LEG_TAG_LEN))
    return core::ptr::null_mut();
    fcs_len = skb.len;
    fcs_val = cpu_to_le32(crc32_le(~0, skb.data, fcs_len) ^ ~0);
    skb_push(skb, BRCM_LEG_TAG_LEN);
    dsa_alloc_etype_header(skb, BRCM_LEG_TAG_LEN);
    brcm_tag = skb.data + 2 * ETH_ALEN;
// Broadcom tag type
    brcm_tag[0] = BRCM_LEG_TYPE_HI;
    brcm_tag[1] = BRCM_LEG_TYPE_LO;
// Broadcom tag value
    brcm_tag[2] = BRCM_LEG_EGRESS | BRCM_LEG_LEN_HI(fcs_len);
    brcm_tag[3] = BRCM_LEG_LEN_LO(fcs_len);
    brcm_tag[4] = 0;
    brcm_tag[5] = dp.index & BRCM_LEG_PORT_ID;
// Original FCS value
    if (skb_pad(skb, ETH_FCS_LEN))
    return core::ptr::null_mut();
    skb_put_data(skb, &fcs_val, ETH_FCS_LEN);
    return skb;
    }
    static const struct dsa_device_ops brcm_legacy_fcs_netdev_ops = {
    .name = BRCM_LEGACY_FCS_NAME,
    .proto = DSA_TAG_PROTO_BRCM_LEGACY_FCS,
    .xmit = brcm_leg_fcs_tag_xmit,
    .rcv = brcm_leg_tag_rcv,
    .needed_headroom = BRCM_LEG_TAG_LEN,
    };
    DSA_TAG_DRIVER(brcm_legacy_fcs_netdev_ops);
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_BRCM_LEGACY_FCS, BRCM_LEGACY_FCS_NAME);

    static struct sk_buff *brcm_tag_xmit_prepend(struct sk_buff *skb,
    struct net_device *dev)
    {
// tag is prepended to the packet
    return brcm_tag_xmit_ll(skb, dev, 0);
    }
    static struct sk_buff *brcm_tag_rcv_prepend(struct sk_buff *skb,
    struct net_device *dev)
    {
// tag is prepended to the packet
    return brcm_tag_rcv_ll(skb, dev, ETH_HLEN);
    }
    static const struct dsa_device_ops brcm_prepend_netdev_ops = {
    .name	= BRCM_PREPEND_NAME,
    .proto	= DSA_TAG_PROTO_BRCM_PREPEND,
    .xmit	= brcm_tag_xmit_prepend,
    .rcv	= brcm_tag_rcv_prepend,
    .needed_headroom = BRCM_TAG_LEN,
    };
    DSA_TAG_DRIVER(brcm_prepend_netdev_ops);
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_BRCM_PREPEND, BRCM_PREPEND_NAME);

    static struct dsa_tag_driver *dsa_tag_driver_array[] =	{

    &DSA_TAG_DRIVER_NAME(brcm_netdev_ops),

    &DSA_TAG_DRIVER_NAME(brcm_legacy_netdev_ops),

    &DSA_TAG_DRIVER_NAME(brcm_legacy_fcs_netdev_ops),

    &DSA_TAG_DRIVER_NAME(brcm_prepend_netdev_ops),

    };
    module_dsa_tag_drivers(dsa_tag_driver_array);
    MODULE_DESCRIPTION("DSA tag driver for Broadcom switches using in-frame headers");
    MODULE_LICENSE("GPL");
