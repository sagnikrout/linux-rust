//! Automatically rewritten from C to Rust
//! Source: net/8021q/vlan_gvrp.c
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
// IEEE 802.1Q GARP VLAN Registration Protocol (GVRP)
//
// Copyright (c) 2008 Patrick McHardy <kaber@trash.net>
//

    enum gvrp_attributes {
    GVRP_ATTR_INVALID,
    GVRP_ATTR_VID,
    __GVRP_ATTR_MAX
    };

    static struct garp_application vlan_gvrp_app __read_mostly = {
    .proto.group_address	= GARP_GVRP_ADDRESS,
    .maxattr		= GVRP_ATTR_MAX,
    .type			= GARP_APPLICATION_GVRP,
    };
#[no_mangle]
pub unsafe extern "C" fn vlan_gvrp_request_join(dev: *const net_device) -> c_int {
    int vlan_gvrp_request_join(const struct net_device *dev)
    {
    const struct vlan_dev_priv *vlan = vlan_dev_priv(dev);
    let mut vlan_id: __be16 = htons(vlan.vlan_id);
    if (vlan.vlan_proto != htons(ETH_P_8021Q))
    return 0;
    return garp_request_join(vlan.real_dev, &vlan_gvrp_app,
    &vlan_id, sizeof(vlan_id), GVRP_ATTR_VID);
    }
#[no_mangle]
pub unsafe extern "C" fn vlan_gvrp_request_leave(dev: *const net_device) {
    void vlan_gvrp_request_leave(const struct net_device *dev)
    {
    const struct vlan_dev_priv *vlan = vlan_dev_priv(dev);
    let mut vlan_id: __be16 = htons(vlan.vlan_id);
    if (vlan.vlan_proto != htons(ETH_P_8021Q))
    return;
    garp_request_leave(vlan.real_dev, &vlan_gvrp_app,
    &vlan_id, sizeof(vlan_id), GVRP_ATTR_VID);
    }
#[no_mangle]
pub unsafe extern "C" fn vlan_gvrp_init_applicant(dev: *mut net_device) -> c_int {
    int vlan_gvrp_init_applicant(struct net_device *dev)
    {
    return garp_init_applicant(dev, &vlan_gvrp_app);
    }
#[no_mangle]
pub unsafe extern "C" fn vlan_gvrp_uninit_applicant(dev: *mut net_device) {
    void vlan_gvrp_uninit_applicant(struct net_device *dev)
    {
    garp_uninit_applicant(dev, &vlan_gvrp_app);
    }
#[no_mangle]
pub unsafe extern "C" fn vlan_gvrp_init() -> int __init {
    int __init vlan_gvrp_init(void)
    {
    return garp_register_application(&vlan_gvrp_app);
    }
#[no_mangle]
pub unsafe extern "C" fn vlan_gvrp_uninit() {
    void vlan_gvrp_uninit(void)
    {
    garp_unregister_application(&vlan_gvrp_app);
    }
