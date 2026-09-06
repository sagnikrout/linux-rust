//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_vsc73xx_8021q.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
// Copyright (C) 2024 Pawel Dembicki <paweldembicki@gmail.com>
//

    static struct sk_buff *
    vsc73xx_xmit(struct sk_buff *skb, struct net_device *netdev)
    {
    struct dsa_port *dp = dsa_user_to_port(netdev);
    let mut queue_mapping: u16 = skb_get_queue_mapping(skb);
    let mut tx_vid: u16 = dsa_tag_8021q_standalone_vid(dp);
    u8 pcp;
    if (skb.offload_fwd_mark) {
    let mut bridge_num: c_uint = dsa_port_bridge_num_get(dp);
    struct net_device *br = dsa_port_bridge_dev_get(dp);
    if (br_vlan_enabled(br))
    return skb;
    tx_vid = dsa_tag_8021q_bridge_vid(bridge_num);
    }
    pcp = netdev_txq_to_tc(netdev, queue_mapping);
    return dsa_8021q_xmit(skb, netdev, ETH_P_8021Q,
    ((pcp << VLAN_PRIO_SHIFT) | tx_vid));
    }
    static struct sk_buff *
    vsc73xx_rcv(struct sk_buff *skb, struct net_device *netdev)
    {
    let mut src_port: c_int = -1, switch_id = -1, vbid = -1, vid = -1;
    dsa_8021q_rcv(skb, &src_port, &switch_id, &vbid, &vid);
    skb.dev = dsa_tag_8021q_find_user(netdev, src_port, switch_id,
    vid, vbid);
    if (!skb.dev) {
    dev_warn_ratelimited(&netdev.dev,
    "Couldn't decode source port\n");
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    dsa_default_offload_fwd_mark(skb);
    return skb;
    }
    static const struct dsa_device_ops vsc73xx_8021q_netdev_ops = {
    .name			= VSC73XX_8021Q_NAME,
    .proto			= DSA_TAG_PROTO_VSC73XX_8021Q,
    .xmit			= vsc73xx_xmit,
    .rcv			= vsc73xx_rcv,
    .needed_headroom	= VLAN_HLEN,
    .promisc_on_conduit	= true,
    };
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("DSA tag driver for VSC73XX family of switches, using VLAN");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_VSC73XX_8021Q, VSC73XX_8021Q_NAME);
    module_dsa_tag_driver(vsc73xx_8021q_netdev_ops);
