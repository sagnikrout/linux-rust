//! Automatically rewritten from C to Rust
//! Source: net/dsa/tag_ocelot_8021q.c
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
// Copyright 2020-2021 NXP
//
// An implementation of the software-defined tag_8021q.c tagger format, which
// also preserves full functionality under a vlan_filtering bridge. It does
// this by using the TCAM engines for:
// - pushing the RX VLAN as a second, outer tag, on egress towards the CPU port
// - redirecting towards the correct front port based on TX VLAN and popping
// that on egress
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_8021q_tagger_private {
    pub /: *mut *mut ocelot_8021q_tagger_data data; / Must be first,
    pub xmit_worker: *mut kthread_worker,
}

    static struct sk_buff *ocelot_defer_xmit(struct dsa_port *dp,
    struct sk_buff *skb)
    {
    struct ocelot_8021q_tagger_private *priv = dp.ds.tagger_data;
    struct ocelot_8021q_tagger_data *data = &priv.data;
    void (*xmit_work_fn)(struct kthread_work *work);
    struct felix_deferred_xmit_work *xmit_work;
    struct kthread_worker *xmit_worker;
    xmit_work_fn = data.xmit_work_fn;
    xmit_worker = priv.xmit_worker;
    if (!xmit_work_fn || !xmit_worker) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// PTP over IP packets need UDP checksumming. We may have inherited
// NETIF_F_HW_CSUM from the DSA conduit, but these packets are not sent
// through the DSA conduit, so calculate the checksum here.
//
    if (skb.ip_summed == CHECKSUM_PARTIAL && skb_checksum_help(skb)) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    xmit_work = kzalloc_obj(*xmit_work, GFP_ATOMIC);
    if (!xmit_work) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
// Calls felix_port_deferred_xmit in felix.c
    kthread_init_work(&xmit_work.work, xmit_work_fn);
    xmit_work.dp = dp;
    xmit_work.skb = skb_get(skb);
    kthread_queue_work(xmit_worker, &xmit_work.work);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    static struct sk_buff *ocelot_xmit(struct sk_buff *skb,
    struct net_device *netdev)
    {
    struct dsa_port *dp = dsa_user_to_port(netdev);
    let mut queue_mapping: u16 = skb_get_queue_mapping(skb);
    let mut pcp: u8 = netdev_txq_to_tc(netdev, queue_mapping);
    let mut tx_vid: u16 = dsa_tag_8021q_standalone_vid(dp);
    struct ethhdr *hdr = eth_hdr(skb);
    if (ocelot_ptp_rew_op(skb) || is_link_local_ether_addr(hdr.h_dest))
    return ocelot_defer_xmit(dp, skb);
    return dsa_8021q_xmit(skb, netdev, ETH_P_8021Q,
    ((pcp << VLAN_PRIO_SHIFT) | tx_vid));
    }
    static struct sk_buff *ocelot_rcv(struct sk_buff *skb,
    struct net_device *netdev)
    {
    let mut src_port: c_int = -1, switch_id = -1;
    dsa_8021q_rcv(skb, &src_port, &switch_id, core::ptr::null_mut(), core::ptr::null_mut());
    skb.dev = dsa_conduit_find_user(netdev, switch_id, src_port);
    if (!skb.dev) {
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    dsa_default_offload_fwd_mark(skb);
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn ocelot_disconnect(ds: *mut dsa_switch) {
    static void ocelot_disconnect(struct dsa_switch *ds)
    {
    struct ocelot_8021q_tagger_private *priv = ds.tagger_data;
    kthread_destroy_worker(priv.xmit_worker);
    kfree(priv);
    ds.tagger_data = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn ocelot_connect(ds: *mut dsa_switch) -> c_int {
    static int ocelot_connect(struct dsa_switch *ds)
    {
    struct ocelot_8021q_tagger_private *priv;
    int err;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    priv.xmit_worker = kthread_run_worker(0, "felix_xmit");
    if (IS_ERR(priv.xmit_worker)) {
    err = PTR_ERR(priv.xmit_worker);
    kfree(priv);
    return err;
    }
    ds.tagger_data = priv;
    return 0;
    }
    static const struct dsa_device_ops ocelot_8021q_netdev_ops = {
    .name			= OCELOT_8021Q_NAME,
    .proto			= DSA_TAG_PROTO_OCELOT_8021Q,
    .xmit			= ocelot_xmit,
    .rcv			= ocelot_rcv,
    .connect		= ocelot_connect,
    .disconnect		= ocelot_disconnect,
    .needed_headroom	= VLAN_HLEN,
    .promisc_on_conduit	= true,
    };
    MODULE_DESCRIPTION("DSA tag driver for Ocelot family of switches, using VLAN");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_OCELOT_8021Q, OCELOT_8021Q_NAME);
    module_dsa_tag_driver(ocelot_8021q_netdev_ops);
