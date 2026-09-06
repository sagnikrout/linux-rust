//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/nxp/nxpwifi/ethtool.c
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
// nxpwifi: ethtool
//
// Copyright 2011-2024 NXP
//

    static void nxpwifi_ethtool_get_wol(struct net_device *dev,
    struct ethtool_wolinfo *wol)
    {
    struct nxpwifi_private *priv = nxpwifi_netdev_get_priv(dev);
    let mut conditions: u32 = le32_to_cpu(priv.adapter.hs_cfg.conditions);
    wol.supported = WAKE_UCAST | WAKE_MCAST | WAKE_BCAST | WAKE_PHY;
    if (conditions == HS_CFG_COND_DEF)
    return;
    if (conditions & HS_CFG_COND_UNICAST_DATA)
    wol.wolopts |= WAKE_UCAST;
    if (conditions & HS_CFG_COND_MULTICAST_DATA)
    wol.wolopts |= WAKE_MCAST;
    if (conditions & HS_CFG_COND_BROADCAST_DATA)
    wol.wolopts |= WAKE_BCAST;
    if (conditions & HS_CFG_COND_MAC_EVENT)
    wol.wolopts |= WAKE_PHY;
    }
    static int nxpwifi_ethtool_set_wol(struct net_device *dev,
    struct ethtool_wolinfo *wol)
    {
    struct nxpwifi_private *priv = nxpwifi_netdev_get_priv(dev);
    let mut conditions: u32 = 0;
    if (wol.wolopts & ~(WAKE_UCAST | WAKE_MCAST | WAKE_BCAST | WAKE_PHY))
    return -EOPNOTSUPP;
    if (wol.wolopts & WAKE_UCAST)
    conditions |= HS_CFG_COND_UNICAST_DATA;
    if (wol.wolopts & WAKE_MCAST)
    conditions |= HS_CFG_COND_MULTICAST_DATA;
    if (wol.wolopts & WAKE_BCAST)
    conditions |= HS_CFG_COND_BROADCAST_DATA;
    if (wol.wolopts & WAKE_PHY)
    conditions |= HS_CFG_COND_MAC_EVENT;
    if (wol.wolopts == 0)
    conditions |= HS_CFG_COND_DEF;
    priv.adapter.hs_cfg.conditions = cpu_to_le32(conditions);
    return 0;
    }
    const struct ethtool_ops nxpwifi_ethtool_ops = {
    .get_wol = nxpwifi_ethtool_get_wol,
    .set_wol = nxpwifi_ethtool_set_wol,
    };
