//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/spi/mcp251xfd/mcp251xfd-ethtool.c
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
// mcp251xfd - Microchip MCP251xFD Family CAN controller driver
//
// Copyright (c) 2021, 2022 Pengutronix,
// Marc Kleine-Budde <kernel@pengutronix.de>
//

    static void
    mcp251xfd_ring_get_ringparam(struct net_device *ndev,
    struct ethtool_ringparam *ring,
    struct kernel_ethtool_ringparam *kernel_ring,
    struct netlink_ext_ack *extack)
    {
    const struct mcp251xfd_priv *priv = netdev_priv(ndev);
    let mut fd_mode: bool = mcp251xfd_is_fd_mode(priv);
    struct can_ram_layout layout;
    can_ram_get_layout(&layout, &mcp251xfd_ram_config, core::ptr::null_mut(), core::ptr::null_mut(), fd_mode);
    ring.rx_max_pending = layout.max_rx;
    ring.tx_max_pending = layout.max_tx;
    ring.rx_pending = priv.rx_obj_num;
    ring.tx_pending = priv.tx.obj_num;
    }
    static int
    mcp251xfd_ring_set_ringparam(struct net_device *ndev,
    struct ethtool_ringparam *ring,
    struct kernel_ethtool_ringparam *kernel_ring,
    struct netlink_ext_ack *extack)
    {
    struct mcp251xfd_priv *priv = netdev_priv(ndev);
    let mut fd_mode: bool = mcp251xfd_is_fd_mode(priv);
    struct can_ram_layout layout;
    can_ram_get_layout(&layout, &mcp251xfd_ram_config, ring, core::ptr::null_mut(), fd_mode);
    if ((layout.cur_rx != priv.rx_obj_num ||
    layout.cur_tx != priv.tx.obj_num) &&
    netif_running(ndev))
    return -EBUSY;
    priv.rx_obj_num = layout.cur_rx;
    priv.rx_obj_num_coalesce_irq = layout.rx_coalesce;
    priv.tx.obj_num = layout.cur_tx;
    priv.tx_obj_num_coalesce_irq = layout.tx_coalesce;
    return 0;
    }
    static int mcp251xfd_ring_get_coalesce(struct net_device *ndev,
    struct ethtool_coalesce *ec,
    struct kernel_ethtool_coalesce *kec,
    struct netlink_ext_ack *ext_ack)
    {
    struct mcp251xfd_priv *priv = netdev_priv(ndev);
    u32 rx_max_frames, tx_max_frames;
// The ethtool doc says:
// To disable coalescing, set usecs = 0 and max_frames = 1.
//
    if (priv.rx_obj_num_coalesce_irq == 0)
    rx_max_frames = 1;
    else
    rx_max_frames = priv.rx_obj_num_coalesce_irq;
    ec.rx_max_coalesced_frames_irq = rx_max_frames;
    ec.rx_coalesce_usecs_irq = priv.rx_coalesce_usecs_irq;
    if (priv.tx_obj_num_coalesce_irq == 0)
    tx_max_frames = 1;
    else
    tx_max_frames = priv.tx_obj_num_coalesce_irq;
    ec.tx_max_coalesced_frames_irq = tx_max_frames;
    ec.tx_coalesce_usecs_irq = priv.tx_coalesce_usecs_irq;
    return 0;
    }
    static int mcp251xfd_ring_set_coalesce(struct net_device *ndev,
    struct ethtool_coalesce *ec,
    struct kernel_ethtool_coalesce *kec,
    struct netlink_ext_ack *ext_ack)
    {
    struct mcp251xfd_priv *priv = netdev_priv(ndev);
    let mut fd_mode: bool = mcp251xfd_is_fd_mode(priv);
    const struct ethtool_ringparam ring = {
    .rx_pending = priv.rx_obj_num,
    .tx_pending = priv.tx.obj_num,
    };
    struct can_ram_layout layout;
    can_ram_get_layout(&layout, &mcp251xfd_ram_config, &ring, ec, fd_mode);
    if ((layout.rx_coalesce != priv.rx_obj_num_coalesce_irq ||
    ec.rx_coalesce_usecs_irq != priv.rx_coalesce_usecs_irq ||
    layout.tx_coalesce != priv.tx_obj_num_coalesce_irq ||
    ec.tx_coalesce_usecs_irq != priv.tx_coalesce_usecs_irq) &&
    netif_running(ndev))
    return -EBUSY;
    priv.rx_obj_num = layout.cur_rx;
    priv.rx_obj_num_coalesce_irq = layout.rx_coalesce;
    priv.rx_coalesce_usecs_irq = ec.rx_coalesce_usecs_irq;
    priv.tx.obj_num = layout.cur_tx;
    priv.tx_obj_num_coalesce_irq = layout.tx_coalesce;
    priv.tx_coalesce_usecs_irq = ec.tx_coalesce_usecs_irq;
    return 0;
    }
    static const struct ethtool_ops mcp251xfd_ethtool_ops = {
    .supported_coalesce_params = ETHTOOL_COALESCE_RX_USECS_IRQ |
    ETHTOOL_COALESCE_RX_MAX_FRAMES_IRQ |
    ETHTOOL_COALESCE_TX_USECS_IRQ |
    ETHTOOL_COALESCE_TX_MAX_FRAMES_IRQ,
    .get_ringparam = mcp251xfd_ring_get_ringparam,
    .set_ringparam = mcp251xfd_ring_set_ringparam,
    .get_coalesce = mcp251xfd_ring_get_coalesce,
    .set_coalesce = mcp251xfd_ring_set_coalesce,
    .get_ts_info = can_ethtool_op_get_ts_info_hwts,
    };
#[no_mangle]
pub unsafe extern "C" fn mcp251xfd_ethtool_init(priv: *mut mcp251xfd_priv) {
    void mcp251xfd_ethtool_init(struct mcp251xfd_priv *priv)
    {
    struct can_ram_layout layout;
    priv.ndev.ethtool_ops = &mcp251xfd_ethtool_ops;
    can_ram_get_layout(&layout, &mcp251xfd_ram_config, core::ptr::null_mut(), core::ptr::null_mut(), false);
    priv.rx_obj_num = layout.default_rx;
    priv.tx.obj_num = layout.default_tx;
    priv.rx_obj_num_coalesce_irq = 0;
    priv.tx_obj_num_coalesce_irq = 0;
    priv.rx_coalesce_usecs_irq = 0;
    priv.tx_coalesce_usecs_irq = 0;
    }
