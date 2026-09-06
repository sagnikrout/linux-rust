//! Automatically rewritten from C to Rust
//! Source: net/802/stp.c
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
// STP SAP demux
//
// Copyright (c) 2008 Patrick McHardy <kaber@trash.net>
//

// 01:80:c2:00:00:20 - 01:80:c2:00:00:2F
pub const GARP_ADDR_MIN: c_uint = 0x20;
pub const GARP_ADDR_MAX: c_uint = 0x2F;

    static const struct stp_proto __rcu *garp_protos[GARP_ADDR_RANGE + 1] __read_mostly;
    static const struct stp_proto __rcu *stp_proto __read_mostly;
    static struct llc_sap *sap __read_mostly;
    static unsigned int sap_registered;
    static DEFINE_MUTEX(stp_proto_mutex);
// Called under rcu_read_lock from LLC
    static int stp_pdu_rcv(struct sk_buff *skb, struct net_device *dev,
    struct packet_type *pt, struct net_device *orig_dev)
    {
    const struct ethhdr *eh = eth_hdr(skb);
    const struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    const struct stp_proto *proto;
    if (pdu.ssap != LLC_SAP_BSPAN ||
    pdu.dsap != LLC_SAP_BSPAN ||
    pdu.ctrl_1 != LLC_PDU_TYPE_U)
    goto err;
    if (eh.h_dest[5] >= GARP_ADDR_MIN && eh.h_dest[5] <= GARP_ADDR_MAX) {
    proto = rcu_dereference(garp_protos[eh.h_dest[5] -
    GARP_ADDR_MIN]);
    if (proto &&
    !ether_addr_equal(eh.h_dest, proto.group_address))
    goto err;
    } else
    proto = rcu_dereference(stp_proto);
    if (!proto)
    goto err;
    proto.rcv(proto, skb, dev);
    return 0;
    err:
    kfree_skb(skb);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn stp_proto_register(proto: *const stp_proto) -> c_int {
    int stp_proto_register(const struct stp_proto *proto)
    {
    let mut err: c_int = 0;
    mutex_lock(&stp_proto_mutex);
    if (sap_registered++ == 0) {
    sap = llc_sap_open(LLC_SAP_BSPAN, stp_pdu_rcv);
    if (!sap) {
    err = -ENOMEM;
    goto out;
    }
    }
    if (is_zero_ether_addr(proto.group_address))
    rcu_assign_pointer(stp_proto, proto);
    else
    rcu_assign_pointer(garp_protos[proto.group_address[5] -
    GARP_ADDR_MIN], proto);
    out:
    mutex_unlock(&stp_proto_mutex);
    return err;
    }
    EXPORT_SYMBOL_GPL(stp_proto_register);
#[no_mangle]
pub unsafe extern "C" fn stp_proto_unregister(proto: *const stp_proto) {
    void stp_proto_unregister(const struct stp_proto *proto)
    {
    mutex_lock(&stp_proto_mutex);
    if (is_zero_ether_addr(proto.group_address))
    RCU_INIT_POINTER(stp_proto, core::ptr::null_mut());
    else
    RCU_INIT_POINTER(garp_protos[proto.group_address[5] -
    GARP_ADDR_MIN], core::ptr::null_mut());
    synchronize_rcu();
    if (--sap_registered == 0)
    llc_sap_put(sap);
    mutex_unlock(&stp_proto_mutex);
    }
    EXPORT_SYMBOL_GPL(stp_proto_unregister);
    MODULE_DESCRIPTION("SAP demux for IEEE 802.1D Spanning Tree Protocol (STP)");
    MODULE_LICENSE("GPL");
