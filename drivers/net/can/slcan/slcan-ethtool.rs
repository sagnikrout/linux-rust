//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/slcan/slcan-ethtool.c
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
// Copyright (c) 2022 Amarula Solutions, Dario Binacchi <dario.binacchi@amarulasolutions.com>
//

    static const char slcan_priv_flags_strings[][ETH_GSTRING_LEN] = {

    "err-rst-on-open",
    };
#[no_mangle]
unsafe extern "C" fn slcan_get_strings(ndev: *mut net_device, stringset: u32, data: *mut u8) {
    static void slcan_get_strings(struct net_device *ndev, u32 stringset, u8 *data)
    {
    switch (stringset) {
    case ETH_SS_PRIV_FLAGS:
    memcpy(data, slcan_priv_flags_strings,
    sizeof(slcan_priv_flags_strings));
    }
    }
#[no_mangle]
unsafe extern "C" fn slcan_get_priv_flags(ndev: *mut net_device) -> u32 {
    static u32 slcan_get_priv_flags(struct net_device *ndev)
    {
    let mut flags: u32 = 0;
    if (slcan_err_rst_on_open(ndev))
    flags |= SLCAN_PRIV_FLAGS_ERR_RST_ON_OPEN;
    return flags;
    }
#[no_mangle]
unsafe extern "C" fn slcan_set_priv_flags(ndev: *mut net_device, flags: u32) -> c_int {
    static int slcan_set_priv_flags(struct net_device *ndev, u32 flags)
    {
    let mut err_rst_op_open: bool = !!(flags & SLCAN_PRIV_FLAGS_ERR_RST_ON_OPEN);
    return slcan_enable_err_rst_on_open(ndev, err_rst_op_open);
    }
#[no_mangle]
unsafe extern "C" fn slcan_get_sset_count(netdev: *mut net_device, sset: c_int) -> c_int {
    static int slcan_get_sset_count(struct net_device *netdev, int sset)
    {
    switch (sset) {
    case ETH_SS_PRIV_FLAGS:
    return ARRAY_SIZE(slcan_priv_flags_strings);
    default:
    return -EOPNOTSUPP;
    }
    }
    const struct ethtool_ops slcan_ethtool_ops = {
    .get_strings = slcan_get_strings,
    .get_priv_flags = slcan_get_priv_flags,
    .set_priv_flags = slcan_set_priv_flags,
    .get_sset_count = slcan_get_sset_count,
    .get_ts_info = ethtool_op_get_ts_info,
    };
