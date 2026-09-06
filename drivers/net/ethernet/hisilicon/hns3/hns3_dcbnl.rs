//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3_dcbnl.c
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
// Copyright (c) 2016-2017 Hisilicon Limited.

#[no_mangle]
unsafe extern "C" fn hns3_dcbnl_ieee_getets(ndev: *mut net_device, ets: *mut ieee_ets) -> c_int {
    static int hns3_dcbnl_ieee_getets(struct net_device *ndev, struct ieee_ets *ets)
    {
    struct hnae3_handle *h = hns3_get_handle(ndev);
    if (hns3_nic_resetting(ndev))
    return -EBUSY;
    if (h.kinfo.dcb_ops.ieee_getets)
    return h.kinfo.dcb_ops.ieee_getets(h, ets);
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn hns3_dcbnl_ieee_setets(ndev: *mut net_device, ets: *mut ieee_ets) -> c_int {
    static int hns3_dcbnl_ieee_setets(struct net_device *ndev, struct ieee_ets *ets)
    {
    struct hnae3_handle *h = hns3_get_handle(ndev);
    if (hns3_nic_resetting(ndev))
    return -EBUSY;
    if (h.kinfo.dcb_ops.ieee_setets)
    return h.kinfo.dcb_ops.ieee_setets(h, ets);
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn hns3_dcbnl_ieee_getpfc(ndev: *mut net_device, pfc: *mut ieee_pfc) -> c_int {
    static int hns3_dcbnl_ieee_getpfc(struct net_device *ndev, struct ieee_pfc *pfc)
    {
    struct hnae3_handle *h = hns3_get_handle(ndev);
    if (hns3_nic_resetting(ndev))
    return -EBUSY;
    if (h.kinfo.dcb_ops.ieee_getpfc)
    return h.kinfo.dcb_ops.ieee_getpfc(h, pfc);
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn hns3_dcbnl_ieee_setpfc(ndev: *mut net_device, pfc: *mut ieee_pfc) -> c_int {
    static int hns3_dcbnl_ieee_setpfc(struct net_device *ndev, struct ieee_pfc *pfc)
    {
    struct hnae3_handle *h = hns3_get_handle(ndev);
    if (hns3_nic_resetting(ndev))
    return -EBUSY;
    if (h.kinfo.dcb_ops.ieee_setpfc)
    return h.kinfo.dcb_ops.ieee_setpfc(h, pfc);
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn hns3_dcbnl_ieee_setapp(ndev: *mut net_device, app: *mut dcb_app) -> c_int {
    static int hns3_dcbnl_ieee_setapp(struct net_device *ndev, struct dcb_app *app)
    {
    struct hnae3_handle *h = hns3_get_handle(ndev);
    if (hns3_nic_resetting(ndev))
    return -EBUSY;
    if (h.kinfo.dcb_ops.ieee_setapp)
    return h.kinfo.dcb_ops.ieee_setapp(h, app);
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn hns3_dcbnl_ieee_delapp(ndev: *mut net_device, app: *mut dcb_app) -> c_int {
    static int hns3_dcbnl_ieee_delapp(struct net_device *ndev, struct dcb_app *app)
    {
    struct hnae3_handle *h = hns3_get_handle(ndev);
    if (hns3_nic_resetting(ndev))
    return -EBUSY;
    if (h.kinfo.dcb_ops.ieee_delapp)
    return h.kinfo.dcb_ops.ieee_delapp(h, app);
    return -EOPNOTSUPP;
    }
// DCBX configuration
#[no_mangle]
unsafe extern "C" fn hns3_dcbnl_getdcbx(ndev: *mut net_device) -> u8 {
    static u8 hns3_dcbnl_getdcbx(struct net_device *ndev)
    {
    struct hnae3_handle *h = hns3_get_handle(ndev);
    if (h.kinfo.dcb_ops.getdcbx)
    return h.kinfo.dcb_ops.getdcbx(h);
    return 0;
    }
// return 0 if successful, otherwise fail
#[no_mangle]
unsafe extern "C" fn hns3_dcbnl_setdcbx(ndev: *mut net_device, mode: u8) -> u8 {
    static u8 hns3_dcbnl_setdcbx(struct net_device *ndev, u8 mode)
    {
    struct hnae3_handle *h = hns3_get_handle(ndev);
    if (h.kinfo.dcb_ops.setdcbx)
    return h.kinfo.dcb_ops.setdcbx(h, mode);
    return 1;
    }
    static const struct dcbnl_rtnl_ops hns3_dcbnl_ops = {
    .ieee_getets	= hns3_dcbnl_ieee_getets,
    .ieee_setets	= hns3_dcbnl_ieee_setets,
    .ieee_getpfc	= hns3_dcbnl_ieee_getpfc,
    .ieee_setpfc	= hns3_dcbnl_ieee_setpfc,
    .ieee_setapp    = hns3_dcbnl_ieee_setapp,
    .ieee_delapp    = hns3_dcbnl_ieee_delapp,
    .getdcbx	= hns3_dcbnl_getdcbx,
    .setdcbx	= hns3_dcbnl_setdcbx,
    };
// hclge_dcbnl_setup - DCBNL setup
// @handle: the corresponding vport handle
// Set up DCBNL
//
#[no_mangle]
pub unsafe extern "C" fn hns3_dcbnl_setup(handle: *mut hnae3_handle) {
    void hns3_dcbnl_setup(struct hnae3_handle *handle)
    {
    struct net_device *dev = handle.kinfo.netdev;
    if ((!handle.kinfo.dcb_ops) || (handle.flags & HNAE3_SUPPORT_VF))
    return;
    dev.dcbnl_ops = &hns3_dcbnl_ops;
    }
