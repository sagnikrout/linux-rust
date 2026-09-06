//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_lan9303.c
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
// Copyright (C) 2017 Pengutronix, Juergen Borleis <jbe@pengutronix.de>
//

// To define the outgoing port and to discover the incoming port a regular
// VLAN tag is used by the LAN9303. But its VID meaning is 'special':
//
// Dest MAC       Src MAC        TAG    Type
// ...| 1 2 3 4 5 6 | 1 2 3 4 5 6 | 1 2 3 4 | 1 2 |...
// |<------->|
// TAG:
// |<------------->|
// |  1  2 | 3  4  |
// TPID    VID
// 0x8100
//
// VID bit 3 indicates a request for an ALR lookup.
//
// If VID bit 3 is zero, then bits 0 and 1 specify the destination port
// (0, 1, 2) or broadcast (3) or the source port (1, 2).
//
// VID bit 4 is used to specify if the STP port state should be overridden.
// Required when no forwarding between the external ports should happen.
//

pub const LAN9303_TAG_LEN: c_int = 4;

    LAN9303_TAG_RX_STP)
// Decide whether to transmit using ALR lookup, or transmit directly to
// port using tag. ALR learning is performed only when using ALR lookup.
// If the two external ports are bridged and the frame is unicast,
// then use ALR lookup to allow ALR learning on CPU port.
// Otherwise transmit directly to port with STP state override.
// See also: lan9303_separate_ports() and lan9303.pdf 6.4.10.1
//
#[no_mangle]
unsafe extern "C" fn lan9303_xmit_use_arl(dp: *mut dsa_port, dest_addr: *mut u8) -> c_int {
    static int lan9303_xmit_use_arl(struct dsa_port *dp, u8 *dest_addr)
    {
    struct lan9303 *chip = dp.ds.priv;
    return chip.is_bridged && !is_multicast_ether_addr(dest_addr);
    }
    static struct sk_buff *lan9303_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct dsa_port *dp = dsa_user_to_port(dev);
    __be16 *lan9303_tag;
    u16 tag;
// provide 'LAN9303_TAG_LEN' bytes additional space
    skb_push(skb, LAN9303_TAG_LEN);
// make room between MACs and Ether-Type
    dsa_alloc_etype_header(skb, LAN9303_TAG_LEN);
    lan9303_tag = dsa_etype_header_pos_tx(skb);
    tag = lan9303_xmit_use_arl(dp, skb.data) ?
    LAN9303_TAG_TX_USE_ALR :
    dp.index | LAN9303_TAG_TX_STP_OVERRIDE;
    lan9303_tag[0] = htons(ETH_P_8021Q);
    lan9303_tag[1] = htons(tag);
    return skb;
    }
    static struct sk_buff *lan9303_rcv(struct sk_buff *skb, struct net_device *dev)
    {
    u16 lan9303_tag1;
    unsigned int source_port;
    if (unlikely(!pskb_may_pull(skb, LAN9303_TAG_LEN))) {
    dev_warn_ratelimited(&dev.dev,
    "Dropping packet, cannot pull\n");
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    if (skb_vlan_tag_present(skb)) {
    lan9303_tag1 = skb_vlan_tag_get(skb);
    __vlan_hwaccel_clear_tag(skb);
    } else {
    skb_push_rcsum(skb, ETH_HLEN);
    __skb_vlan_pop(skb, &lan9303_tag1);
    skb_pull_rcsum(skb, ETH_HLEN);
    }
    source_port = lan9303_tag1 & 0x3;
    skb.dev = dsa_conduit_find_user(dev, 0, source_port);
    if (!skb.dev) {
    dev_warn_ratelimited(&dev.dev, "Dropping packet due to invalid source port\n");
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    if (!(lan9303_tag1 & LAN9303_TAG_RX_TRAPPED_TO_CPU))
    dsa_default_offload_fwd_mark(skb);
    return skb;
    }
    static const struct dsa_device_ops lan9303_netdev_ops = {
    .name = LAN9303_NAME,
    .proto	= DSA_TAG_PROTO_LAN9303,
    .xmit = lan9303_xmit,
    .rcv = lan9303_rcv,
    .needed_headroom = LAN9303_TAG_LEN,
    };
    MODULE_DESCRIPTION("DSA tag driver for SMSC/Microchip LAN9303 family of switches");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_LAN9303, LAN9303_NAME);
    module_dsa_tag_driver(lan9303_netdev_ops);
