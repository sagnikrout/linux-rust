//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_netc.c
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
// Copyright 2025-2026 NXP
//

// Forward NXP switch tag
pub const NETC_TAG_FORWARD: c_int = 0;
// To_Port NXP switch tag
pub const NETC_TAG_TO_PORT: c_int = 1;
// SubType0: No request to perform timestamping
pub const NETC_TAG_TP_SUBTYPE0: c_int = 0;
// To_Host NXP switch tag
pub const NETC_TAG_TO_HOST: c_int = 2;
// SubType0: frames redirected or copied to CPU port
pub const NETC_TAG_TH_SUBTYPE0: c_int = 0;
// SubType1: frames redirected or copied to CPU port with timestamp
pub const NETC_TAG_TH_SUBTYPE1: c_int = 1;
// SubType2: Transmit timestamp response (two-step timestamping)
pub const NETC_TAG_TH_SUBTYPE2: c_int = 2;
// NETC switch tag lengths
pub const NETC_TAG_FORWARD_LEN: c_int = 6;
pub const NETC_TAG_TP_SUBTYPE0_LEN: c_int = 6;
pub const NETC_TAG_TH_SUBTYPE0_LEN: c_int = 6;
pub const NETC_TAG_TH_SUBTYPE1_LEN: c_int = 14;
pub const NETC_TAG_TH_SUBTYPE2_LEN: c_int = 14;
pub const NETC_TAG_CMN_LEN: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_tag_cmn {
    pub tpid: __be16,
    pub type: u8,
    pub qos: u8,
    pub switch_port: u8,
    pub __packed: },
    static void netc_fill_common_tag(struct netc_tag_cmn *tag, u8 type,
    u8 subtype, u8 sw_id, u8 port, u8 ipv)
    {
    pub htons(ETH_P_NXP_NETC): tag->tpid =,
    tag.type = FIELD_PREP(NETC_TAG_TYPE, type) |
    pub subtype): FIELD_PREP(NETC_TAG_SUBTYPE,,
    pub ipv): tag->qos = NETC_TAG_QV | FIELD_PREP(NETC_TAG_IPV,,
    tag.switch_port = FIELD_PREP(NETC_TAG_SWITCH, sw_id) |
    pub port): FIELD_PREP(NETC_TAG_PORT,,
    }
    static void *netc_fill_common_tp_tag(struct sk_buff *skb,
    struct net_device *ndev,
    u8 subtype, int tag_len)
    {
    pub dsa_user_to_port(ndev): *mut *mut dsa_port dp =,
    pub skb_get_queue_mapping(skb): u16 queue =,
    pub queue): s8 ipv = netdev_txq_to_tc(ndev,,
    pub tag: *mut c_void,
    if (unlikely(ipv < 0))
    pub 0: ipv =,
    pub tag_len): skb_push(skb,,
    pub tag_len): dsa_alloc_etype_header(skb,,
    pub dsa_etype_header_pos_tx(skb): tag =,
    pub NETC_TAG_CMN_LEN): memset(tag + NETC_TAG_CMN_LEN, 0, tag_len -,
// As 'dsa,member' is a required property for NETC switch, the member
// is used to specify the switch ID (thus the hardware switch ID and
// the software switch ID are consistent), its range is 1 ~ 7. The
// NETC switch driver will check this value, and if it is invalid,
// the switch driver will fail the probe.
// In addition, according to the nxp,netc-switch.yaml doc, the port
// index will not be greater than 0xf.
//
    netc_fill_common_tag(tag, NETC_TAG_TO_PORT, subtype,
    pub ipv): dp->ds->index, dp->index,,
    pub tag: return,
    }
    static void netc_fill_tp_tag_subtype0(struct sk_buff *skb,
    struct net_device *ndev)
    {
    netc_fill_common_tp_tag(skb, ndev, NETC_TAG_TP_SUBTYPE0,
    }
// Currently only support To_Port tag, subtype 0
    static struct sk_buff *netc_xmit(struct sk_buff *skb,
    struct net_device *ndev)
    {
    pub ndev): netc_fill_tp_tag_subtype0(skb,,
    pub skb: return,
    }
#[no_mangle]
unsafe extern "C" fn netc_get_rx_tag_len(type: c_int, subtype: c_int) -> c_int {
    static int netc_get_rx_tag_len(int type, int subtype)
    {
// Only NETC_TAG_TO_HOST and NETC_TAG_FORWARD are expected in RX,
// NETC_TAG_TO_PORT is a TX switch tag that does not exist in RX.
//
    if (type == NETC_TAG_TO_HOST) {
    if (subtype == NETC_TAG_TH_SUBTYPE1)
    pub NETC_TAG_TH_SUBTYPE1_LEN: return,
#[no_mangle]
pub unsafe extern "C" fn if(NETC_TAG_TH_SUBTYPE2: subtype ==) -> else {
    else if (subtype == NETC_TAG_TH_SUBTYPE2)
    pub NETC_TAG_TH_SUBTYPE2_LEN: return,
    else
    pub NETC_TAG_TH_SUBTYPE0_LEN: return,
    }
    pub NETC_TAG_FORWARD_LEN: return,
    }
    static struct sk_buff *netc_rcv(struct sk_buff *skb,
    struct net_device *ndev)
    {
    pub tag_cmn: *mut netc_tag_cmn,
    pub port: int tag_len, sw_id,,
    pub subtype: int type,,
    if (unlikely(!pskb_may_pull(skb, NETC_TAG_MAX_LEN)))
    pub err_free_skb: goto,
    pub dsa_etype_header_pos_rx(skb): tag_cmn =,
    if (ntohs(tag_cmn.tpid) != ETH_P_NXP_NETC) {
    dev_warn_ratelimited(&ndev.dev, "Unknown TPID 0x%04x\n",
    pub err_free_skb: goto,
    }
    if (tag_cmn.qos & NETC_TAG_QV)
    pub tag_cmn->qos): skb->priority = FIELD_GET(NETC_TAG_IPV,,
    pub tag_cmn->switch_port): sw_id = FIELD_GET(NETC_TAG_SWITCH,,
// ENETC VEPA switch ID (0) is not supported yet
    if (!sw_id) {
    dev_warn_ratelimited(&ndev.dev,
    pub yet\n"): "VEPA switch ID is not supported,
    pub err_free_skb: goto,
    }
    pub tag_cmn->switch_port): port = FIELD_GET(NETC_TAG_PORT,,
    pub port): skb->dev = dsa_conduit_find_user(ndev, sw_id,,
    if (!skb.dev)
    pub err_free_skb: goto,
    pub tag_cmn->type): type = FIELD_GET(NETC_TAG_TYPE,,
    pub tag_cmn->type): subtype = FIELD_GET(NETC_TAG_SUBTYPE,,
    if (type == NETC_TAG_FORWARD) {
    } else if (type == NETC_TAG_TO_HOST) {
// Currently only subtype0 supported
    if (subtype != NETC_TAG_TH_SUBTYPE0)
    pub err_free_skb: goto,
    } else {
    dev_warn_ratelimited(&ndev.dev,
    pub type): "Unexpected tag type %d\n",,
    pub err_free_skb: goto,
    }
// Remove Switch tag from the frame
    pub subtype): tag_len = netc_get_rx_tag_len(type,,
    pub tag_len): skb_pull_rcsum(skb,,
    pub tag_len): dsa_strip_etype_header(skb,,
    pub skb: return,
    err_free_skb:
    pub NULL: return,
    }
    static void netc_flow_dissect(const struct sk_buff *skb, __be16 *proto,
    int *offset)
    {
    pub 2): *mut *mut *mut netc_tag_cmn tag_cmn = (netc_tag_cmn )(skb->data -,
    pub tag_cmn->type): int subtype = FIELD_GET(NETC_TAG_SUBTYPE,,
    pub tag_cmn->type): int type = FIELD_GET(NETC_TAG_TYPE,,
    pub subtype): int tag_len = netc_get_rx_tag_len(type,,
// The RX minimum frame length of the NETC switch port is 64 bytes,
// and the frame is received by the ENETC driver. From the hardware
// perspective, the receive buffer of RX BD is at least 128 bytes,
// so the switch tag header is guaranteed to be in the linear region
// of the skb.
//
// offset = tag_len;
// proto = ((__be16 *)skb->data)[(tag_len / 2) - 1];
    }
    static const struct dsa_device_ops netc_netdev_ops = {
    .name			= NETC_NAME,
    .proto			= DSA_TAG_PROTO_NETC,
    .xmit			= netc_xmit,
    .rcv			= netc_rcv,
    .needed_headroom	= NETC_TAG_MAX_LEN,
    .flow_dissect		= netc_flow_dissect,
}

    MODULE_DESCRIPTION("DSA tag driver for NXP NETC switch family");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_NETC, NETC_NAME);
    module_dsa_tag_driver(netc_netdev_ops);
