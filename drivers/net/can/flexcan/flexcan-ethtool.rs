//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/flexcan/flexcan-ethtool.c
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
// Copyright (c) 2022 Pengutronix, Marc Kleine-Budde <kernel@pengutronix.de>
//

    static const char flexcan_priv_flags_strings[][ETH_GSTRING_LEN] = {

    "rx-rtr",
    };
    static void
    flexcan_get_ringparam(struct net_device *ndev, struct ethtool_ringparam *ring,
    struct kernel_ethtool_ringparam *kernel_ring,
    struct netlink_ext_ack *ext_ack)
    {
    const struct flexcan_priv *priv = netdev_priv(ndev);
    ring.rx_max_pending = priv.mb_count;
    ring.tx_max_pending = priv.mb_count;
    if (priv.devtype_data.quirks & FLEXCAN_QUIRK_USE_RX_MAILBOX)
    ring.rx_pending = priv.offload.mb_last -
    priv.offload.mb_first + 1;
    else
    ring.rx_pending = 6;	/* RX-FIFO depth is fixed */
// the drive currently supports only on TX buffer
    ring.tx_pending = 1;
    }
    static void
    flexcan_get_strings(struct net_device *ndev, u32 stringset, u8 *data)
    {
    switch (stringset) {
    case ETH_SS_PRIV_FLAGS:
    memcpy(data, flexcan_priv_flags_strings,
    sizeof(flexcan_priv_flags_strings));
    }
    }
#[no_mangle]
unsafe extern "C" fn flexcan_get_priv_flags(ndev: *mut net_device) -> u32 {
    static u32 flexcan_get_priv_flags(struct net_device *ndev)
    {
    const struct flexcan_priv *priv = netdev_priv(ndev);
    let mut priv_flags: u32 = 0;
    if (flexcan_active_rx_rtr(priv))
    priv_flags |= FLEXCAN_PRIV_FLAGS_RX_RTR;
    return priv_flags;
    }
#[no_mangle]
unsafe extern "C" fn flexcan_set_priv_flags(ndev: *mut net_device, priv_flags: u32) -> c_int {
    static int flexcan_set_priv_flags(struct net_device *ndev, u32 priv_flags)
    {
    struct flexcan_priv *priv = netdev_priv(ndev);
    let mut quirks: u32 = priv.devtype_data.quirks;
    if (priv_flags & FLEXCAN_PRIV_FLAGS_RX_RTR) {
    if (flexcan_supports_rx_mailbox_rtr(priv))
    quirks |= FLEXCAN_QUIRK_USE_RX_MAILBOX;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: flexcan_supports_rx_fifo(priv)) -> else {
    else if (flexcan_supports_rx_fifo(priv))
    quirks &= ~FLEXCAN_QUIRK_USE_RX_MAILBOX;
    else
    quirks |= FLEXCAN_QUIRK_USE_RX_MAILBOX;
    } else {
    if (flexcan_supports_rx_mailbox(priv))
    quirks |= FLEXCAN_QUIRK_USE_RX_MAILBOX;
    else
    quirks &= ~FLEXCAN_QUIRK_USE_RX_MAILBOX;
    }
    if (quirks != priv.devtype_data.quirks && netif_running(ndev))
    return -EBUSY;
    priv.devtype_data.quirks = quirks;
    if (!(priv_flags & FLEXCAN_PRIV_FLAGS_RX_RTR) &&
    !flexcan_active_rx_rtr(priv))
    netdev_info(ndev,
    "Activating RX mailbox mode, cannot receive RTR frames.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flexcan_get_sset_count(netdev: *mut net_device, sset: c_int) -> c_int {
    static int flexcan_get_sset_count(struct net_device *netdev, int sset)
    {
    switch (sset) {
    case ETH_SS_PRIV_FLAGS:
    return ARRAY_SIZE(flexcan_priv_flags_strings);
    default:
    return -EOPNOTSUPP;
    }
    }
    const struct ethtool_ops flexcan_ethtool_ops = {
    .get_ringparam = flexcan_get_ringparam,
    .get_strings = flexcan_get_strings,
    .get_priv_flags = flexcan_get_priv_flags,
    .set_priv_flags = flexcan_set_priv_flags,
    .get_sset_count = flexcan_get_sset_count,
    .get_ts_info = ethtool_op_get_ts_info,
    };
