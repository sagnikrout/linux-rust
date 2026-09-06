//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_qca.c
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
// Copyright (c) 2015, The Linux Foundation. All rights reserved.
//

    static struct sk_buff *qca_tag_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    __be16 *phdr;
    u16 hdr;
    skb_push(skb, QCA_HDR_LEN);
    dsa_alloc_etype_header(skb, QCA_HDR_LEN);
    phdr = dsa_etype_header_pos_tx(skb);
// Set the version field, and set destination port information
    hdr = FIELD_PREP(QCA_HDR_XMIT_VERSION, QCA_HDR_VERSION);
    hdr |= QCA_HDR_XMIT_FROM_CPU;
    hdr |= FIELD_PREP(QCA_HDR_XMIT_DP_BIT, dsa_xmit_port_mask(skb, dev));
// phdr = htons(hdr);
    return skb;
    }
    static struct sk_buff *qca_tag_rcv(struct sk_buff *skb, struct net_device *dev)
    {
    struct qca_tagger_data *tagger_data;
    struct dsa_port *dp = dev.dsa_ptr;
    struct dsa_switch *ds = dp.ds;
    u8 ver, pk_type;
    __be16 *phdr;
    int port;
    u16 hdr;
    BUILD_BUG_ON(sizeof(struct qca_mgmt_ethhdr) != QCA_HDR_MGMT_HEADER_LEN + QCA_HDR_LEN);
    tagger_data = ds.tagger_data;
    if (unlikely(!pskb_may_pull(skb, QCA_HDR_LEN))) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    phdr = dsa_etype_header_pos_rx(skb);
    hdr = ntohs(*phdr);
// Make sure the version is correct
    ver = FIELD_GET(QCA_HDR_RECV_VERSION, hdr);
    if (unlikely(ver != QCA_HDR_VERSION)) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// Get pk type
    pk_type = FIELD_GET(QCA_HDR_RECV_TYPE, hdr);
// Ethernet mgmt read/write packet
    if (pk_type == QCA_HDR_RECV_TYPE_RW_REG_ACK) {
    if (likely(tagger_data.rw_reg_ack_handler))
    tagger_data.rw_reg_ack_handler(ds, skb);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// Ethernet MIB counter packet
    if (pk_type == QCA_HDR_RECV_TYPE_MIB) {
    if (likely(tagger_data.mib_autocast_handler))
    tagger_data.mib_autocast_handler(ds, skb);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// Get source port information
    port = FIELD_GET(QCA_HDR_RECV_SOURCE_PORT, hdr);
    skb.dev = dsa_conduit_find_user(dev, 0, port);
    if (!skb.dev) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// Remove QCA tag and recalculate checksum
    skb_pull_rcsum(skb, QCA_HDR_LEN);
    dsa_strip_etype_header(skb, QCA_HDR_LEN);
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn qca_tag_connect(ds: *mut dsa_switch) -> c_int {
    static int qca_tag_connect(struct dsa_switch *ds)
    {
    struct qca_tagger_data *tagger_data;
    tagger_data = kzalloc_obj(*tagger_data);
    if (!tagger_data)
    return -ENOMEM;
    ds.tagger_data = tagger_data;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qca_tag_disconnect(ds: *mut dsa_switch) {
    static void qca_tag_disconnect(struct dsa_switch *ds)
    {
    kfree(ds.tagger_data);
    ds.tagger_data = core::ptr::null_mut();
    }
    static const struct dsa_device_ops qca_netdev_ops = {
    .name	= QCA_NAME,
    .proto	= DSA_TAG_PROTO_QCA,
    .connect = qca_tag_connect,
    .disconnect = qca_tag_disconnect,
    .xmit	= qca_tag_xmit,
    .rcv	= qca_tag_rcv,
    .needed_headroom = QCA_HDR_LEN,
    .promisc_on_conduit = true,
    };
    MODULE_DESCRIPTION("DSA tag driver for Qualcomm Atheros QCA8K switches");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_QCA, QCA_NAME);
    module_dsa_tag_driver(qca_netdev_ops);
