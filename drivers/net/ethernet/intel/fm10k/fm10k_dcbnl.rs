//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/fm10k/fm10k_dcbnl.c
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
// Copyright(c) 2013 - 2019 Intel Corporation.

//
// fm10k_dcbnl_ieee_getets - get the ETS configuration for the device
// @dev: netdev interface for the device
// @ets: ETS structure to push configuration to
//
#[no_mangle]
unsafe extern "C" fn fm10k_dcbnl_ieee_getets(dev: *mut net_device, ets: *mut ieee_ets) -> c_int {
    static int fm10k_dcbnl_ieee_getets(struct net_device *dev, struct ieee_ets *ets)
    {
    int i;
// we support 8 TCs in all modes
    ets.ets_cap = IEEE_8021QAZ_MAX_TCS;
    ets.cbs = 0;
// we only support strict priority and cannot do traffic shaping
    memset(ets.tc_tx_bw, 0, sizeof(ets.tc_tx_bw));
    memset(ets.tc_rx_bw, 0, sizeof(ets.tc_rx_bw));
    memset(ets.tc_tsa, IEEE_8021QAZ_TSA_STRICT, sizeof(ets.tc_tsa));
// populate the prio map based on the netdev
    for (i = 0; i < IEEE_8021QAZ_MAX_TCS; i++)
    ets.prio_tc[i] = netdev_get_prio_tc_map(dev, i);
    return 0;
    }
//
// fm10k_dcbnl_ieee_setets - set the ETS configuration for the device
// @dev: netdev interface for the device
// @ets: ETS structure to pull configuration from
//
#[no_mangle]
unsafe extern "C" fn fm10k_dcbnl_ieee_setets(dev: *mut net_device, ets: *mut ieee_ets) -> c_int {
    static int fm10k_dcbnl_ieee_setets(struct net_device *dev, struct ieee_ets *ets)
    {
    let mut num_tc: u8 = 0;
    int i;
// verify type and determine num_tcs needed
    for (i = 0; i < IEEE_8021QAZ_MAX_TCS; i++) {
    if (ets.tc_tx_bw[i] || ets.tc_rx_bw[i])
    return -EINVAL;
    if (ets.tc_tsa[i] != IEEE_8021QAZ_TSA_STRICT)
    return -EINVAL;
    if (ets.prio_tc[i] > num_tc)
    num_tc = ets.prio_tc[i];
    }
// if requested TC is greater than 0 then num_tcs is max + 1
    if (num_tc)
    num_tc++;
    if (num_tc > IEEE_8021QAZ_MAX_TCS)
    return -EINVAL;
// update TC hardware mapping if necessary
    if (num_tc != netdev_get_num_tc(dev)) {
    let mut err: c_int = fm10k_setup_tc(dev, num_tc);
    if (err)
    return err;
    }
// update priority mapping
    for (i = 0; i < IEEE_8021QAZ_MAX_TCS; i++)
    netdev_set_prio_tc_map(dev, i, ets.prio_tc[i]);
    return 0;
    }
//
// fm10k_dcbnl_ieee_getpfc - get the PFC configuration for the device
// @dev: netdev interface for the device
// @pfc: PFC structure to push configuration to
//
#[no_mangle]
unsafe extern "C" fn fm10k_dcbnl_ieee_getpfc(dev: *mut net_device, pfc: *mut ieee_pfc) -> c_int {
    static int fm10k_dcbnl_ieee_getpfc(struct net_device *dev, struct ieee_pfc *pfc)
    {
    struct fm10k_intfc *interface = netdev_priv(dev);
// record flow control max count and state of TCs
    pfc.pfc_cap = IEEE_8021QAZ_MAX_TCS;
    pfc.pfc_en = interface.pfc_en;
    return 0;
    }
//
// fm10k_dcbnl_ieee_setpfc - set the PFC configuration for the device
// @dev: netdev interface for the device
// @pfc: PFC structure to pull configuration from
//
#[no_mangle]
unsafe extern "C" fn fm10k_dcbnl_ieee_setpfc(dev: *mut net_device, pfc: *mut ieee_pfc) -> c_int {
    static int fm10k_dcbnl_ieee_setpfc(struct net_device *dev, struct ieee_pfc *pfc)
    {
    struct fm10k_intfc *interface = netdev_priv(dev);
// record PFC configuration to interface
    interface.pfc_en = pfc.pfc_en;
// if we are running update the drop_en state for all queues
    if (netif_running(dev))
    fm10k_update_rx_drop_en(interface);
    return 0;
    }
//
// fm10k_dcbnl_getdcbx - get the DCBX configuration for the device
// @dev: netdev interface for the device
//
// Returns that we support only IEEE DCB for this interface
//
#[no_mangle]
unsafe extern "C" fn fm10k_dcbnl_getdcbx(dev: *mut net_device __always_unused) -> u8 {
    static u8 fm10k_dcbnl_getdcbx(struct net_device __always_unused *dev)
    {
    return DCB_CAP_DCBX_HOST | DCB_CAP_DCBX_VER_IEEE;
    }
//
// fm10k_dcbnl_setdcbx - get the DCBX configuration for the device
// @dev: netdev interface for the device
// @mode: new mode for this device
//
// Returns error on attempt to enable anything but IEEE DCB for this interface
//
#[no_mangle]
unsafe extern "C" fn fm10k_dcbnl_setdcbx(dev: *mut net_device __always_unused, mode: u8) -> u8 {
    static u8 fm10k_dcbnl_setdcbx(struct net_device __always_unused *dev, u8 mode)
    {
    return (mode != (DCB_CAP_DCBX_HOST | DCB_CAP_DCBX_VER_IEEE)) ? 1 : 0;
    }
    static const struct dcbnl_rtnl_ops fm10k_dcbnl_ops = {
    .ieee_getets	= fm10k_dcbnl_ieee_getets,
    .ieee_setets	= fm10k_dcbnl_ieee_setets,
    .ieee_getpfc	= fm10k_dcbnl_ieee_getpfc,
    .ieee_setpfc	= fm10k_dcbnl_ieee_setpfc,
    .getdcbx	= fm10k_dcbnl_getdcbx,
    .setdcbx	= fm10k_dcbnl_setdcbx,
    };
//
// fm10k_dcbnl_set_ops - Configures dcbnl ops pointer for netdev
// @dev: netdev interface for the device
//
// Enables PF for DCB by assigning DCBNL ops pointer.
//
#[no_mangle]
pub unsafe extern "C" fn fm10k_dcbnl_set_ops(dev: *mut net_device) {
    void fm10k_dcbnl_set_ops(struct net_device *dev)
    {
    struct fm10k_intfc *interface = netdev_priv(dev);
    struct fm10k_hw *hw = &interface.hw;
    if (hw.mac.type == fm10k_mac_pf)
    dev.dcbnl_ops = &fm10k_dcbnl_ops;
    }
