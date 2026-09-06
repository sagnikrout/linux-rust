//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/rockchip/rockchip_canfd-ethtool.c
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
// Copyright (c) 2023, 2024 Pengutronix,
// Marc Kleine-Budde <kernel@pengutronix.de>
//

    enum rkcanfd_stats_type {
    RKCANFD_STATS_TYPE_RX_FIFO_EMPTY_ERRORS,
    RKCANFD_STATS_TYPE_TX_EXTENDED_AS_STANDARD_ERRORS,
    };
    static const char rkcanfd_stats_strings[][ETH_GSTRING_LEN] = {
    [RKCANFD_STATS_TYPE_RX_FIFO_EMPTY_ERRORS] = "rx_fifo_empty_errors",
    [RKCANFD_STATS_TYPE_TX_EXTENDED_AS_STANDARD_ERRORS] = "tx_extended_as_standard_errors",
    };
    static void
    rkcanfd_ethtool_get_strings(struct net_device *ndev, u32 stringset, u8 *buf)
    {
    switch (stringset) {
    case ETH_SS_STATS:
    memcpy(buf, rkcanfd_stats_strings,
    sizeof(rkcanfd_stats_strings));
    }
    }
#[no_mangle]
unsafe extern "C" fn rkcanfd_ethtool_get_sset_count(netdev: *mut net_device, sset: c_int) -> c_int {
    static int rkcanfd_ethtool_get_sset_count(struct net_device *netdev, int sset)
    {
    switch (sset) {
    case ETH_SS_STATS:
    return ARRAY_SIZE(rkcanfd_stats_strings);
    default:
    return -EOPNOTSUPP;
    }
    }
    static void
    rkcanfd_ethtool_get_ethtool_stats(struct net_device *ndev,
    struct ethtool_stats *stats, u64 *data)
    {
    struct rkcanfd_priv *priv = netdev_priv(ndev);
    struct rkcanfd_stats *rkcanfd_stats;
    unsigned int start;
    rkcanfd_stats = &priv.stats;
    do {
    start = u64_stats_fetch_begin(&rkcanfd_stats.syncp);
    data[RKCANFD_STATS_TYPE_RX_FIFO_EMPTY_ERRORS] =
    u64_stats_read(&rkcanfd_stats.rx_fifo_empty_errors);
    data[RKCANFD_STATS_TYPE_TX_EXTENDED_AS_STANDARD_ERRORS] =
    u64_stats_read(&rkcanfd_stats.tx_extended_as_standard_errors);
    } while (u64_stats_fetch_retry(&rkcanfd_stats.syncp, start));
    }
    static const struct ethtool_ops rkcanfd_ethtool_ops = {
    .get_ts_info = can_ethtool_op_get_ts_info_hwts,
    .get_strings = rkcanfd_ethtool_get_strings,
    .get_sset_count = rkcanfd_ethtool_get_sset_count,
    .get_ethtool_stats = rkcanfd_ethtool_get_ethtool_stats,
    };
#[no_mangle]
pub unsafe extern "C" fn rkcanfd_ethtool_init(priv: *mut rkcanfd_priv) {
    void rkcanfd_ethtool_init(struct rkcanfd_priv *priv)
    {
    priv.ndev.ethtool_ops = &rkcanfd_ethtool_ops;
    u64_stats_init(&priv.stats.syncp);
    }
