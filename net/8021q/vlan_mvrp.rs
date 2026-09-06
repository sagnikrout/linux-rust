//! Automatically rewritten from C to Rust
//! Source: net/8021q/vlan_mvrp.c
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
// IEEE 802.1Q Multiple VLAN Registration Protocol (MVRP)
//
// Copyright (c) 2012 Massachusetts Institute of Technology
//
// Adapted from code in net/8021q/vlan_gvrp.c
// Copyright (c) 2008 Patrick McHardy <kaber@trash.net>
//

    enum mvrp_attributes {
    MVRP_ATTR_INVALID,
    MVRP_ATTR_VID,
    __MVRP_ATTR_MAX
    };

    static struct mrp_application vlan_mrp_app __read_mostly = {
    .type		= MRP_APPLICATION_MVRP,
    .maxattr	= MVRP_ATTR_MAX,
    .pkttype.type	= htons(ETH_P_MVRP),
    .group_address	= MRP_MVRP_ADDRESS,
    .version	= 0,
    };
#[no_mangle]
pub unsafe extern "C" fn vlan_mvrp_request_join(dev: *const net_device) -> c_int {
    int vlan_mvrp_request_join(const struct net_device *dev)
    {
    const struct vlan_dev_priv *vlan = vlan_dev_priv(dev);
    let mut vlan_id: __be16 = htons(vlan.vlan_id);
    if (vlan.vlan_proto != htons(ETH_P_8021Q))
    return 0;
    return mrp_request_join(vlan.real_dev, &vlan_mrp_app,
    &vlan_id, sizeof(vlan_id), MVRP_ATTR_VID);
    }
#[no_mangle]
pub unsafe extern "C" fn vlan_mvrp_request_leave(dev: *const net_device) {
    void vlan_mvrp_request_leave(const struct net_device *dev)
    {
    const struct vlan_dev_priv *vlan = vlan_dev_priv(dev);
    let mut vlan_id: __be16 = htons(vlan.vlan_id);
    if (vlan.vlan_proto != htons(ETH_P_8021Q))
    return;
    mrp_request_leave(vlan.real_dev, &vlan_mrp_app,
    &vlan_id, sizeof(vlan_id), MVRP_ATTR_VID);
    }
#[no_mangle]
pub unsafe extern "C" fn vlan_mvrp_init_applicant(dev: *mut net_device) -> c_int {
    int vlan_mvrp_init_applicant(struct net_device *dev)
    {
    return mrp_init_applicant(dev, &vlan_mrp_app);
    }
#[no_mangle]
pub unsafe extern "C" fn vlan_mvrp_uninit_applicant(dev: *mut net_device) {
    void vlan_mvrp_uninit_applicant(struct net_device *dev)
    {
    mrp_uninit_applicant(dev, &vlan_mrp_app);
    }
#[no_mangle]
pub unsafe extern "C" fn vlan_mvrp_init() -> int __init {
    int __init vlan_mvrp_init(void)
    {
    return mrp_register_application(&vlan_mrp_app);
    }
#[no_mangle]
pub unsafe extern "C" fn vlan_mvrp_uninit() {
    void vlan_mvrp_uninit(void)
    {
    mrp_unregister_application(&vlan_mrp_app);
    }
