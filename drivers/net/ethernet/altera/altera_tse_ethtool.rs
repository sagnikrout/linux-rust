//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/altera/altera_tse_ethtool.c
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
// Ethtool support for Altera Triple-Speed Ethernet MAC driver
// Copyright (C) 2008-2014 Altera Corporation. All rights reserved
//
// Contributors:
// Dalon Westergreen
// Thomas Chou
// Ian Abbott
// Yuriy Kozlov
// Tobias Klauser
// Andriy Smolskyy
// Roman Bulgakov
// Dmytro Mytarchuk
//
// Original driver contributed by SLS.
// Major updates contributed by GlobalLogic
//

pub const TSE_STATS_LEN: c_int = 31;
pub const TSE_NUM_REGS: c_int = 128;
    static char const stat_gstrings[][ETH_GSTRING_LEN] = {
    "tx_packets",
    "rx_packets",
    "rx_crc_errors",
    "rx_align_errors",
    "tx_bytes",
    "rx_bytes",
    "tx_pause",
    "rx_pause",
    "rx_errors",
    "tx_errors",
    "rx_unicast",
    "rx_multicast",
    "rx_broadcast",
    "tx_discards",
    "tx_unicast",
    "tx_multicast",
    "tx_broadcast",
    "ether_drops",
    "rx_total_bytes",
    "rx_total_packets",
    "rx_undersize",
    "rx_oversize",
    "rx_64_bytes",
    "rx_65_127_bytes",
    "rx_128_255_bytes",
    "rx_256_511_bytes",
    "rx_512_1023_bytes",
    "rx_1024_1518_bytes",
    "rx_gte_1519_bytes",
    "rx_jabbers",
    "rx_runts",
    };
    static void tse_get_drvinfo(struct net_device *dev,
    struct ethtool_drvinfo *info)
    {
    struct altera_tse_private *priv = netdev_priv(dev);
    let mut rev: u32 = ioread32(&priv.mac_dev.megacore_revision);
    strcpy(info.driver, "altera_tse");
    snprintf(info.fw_version, ETHTOOL_FWVERS_LEN, "v%d.%d",
    rev & 0xFFFF, (rev & 0xFFFF0000) >> 16);
    sprintf(info.bus_info, "platform");
    }
// Fill in a buffer with the strings which correspond to the
// stats
//
#[no_mangle]
unsafe extern "C" fn tse_gstrings(dev: *mut net_device, stringset: u32, buf: *mut u8) {
    static void tse_gstrings(struct net_device *dev, u32 stringset, u8 *buf)
    {
    memcpy(buf, stat_gstrings, TSE_STATS_LEN * ETH_GSTRING_LEN);
    }
    static void tse_fill_stats(struct net_device *dev, struct ethtool_stats *dummy,
    u64 *buf)
    {
    struct altera_tse_private *priv = netdev_priv(dev);
    u64 ext;
    buf[0] = csrrd32(priv.mac_dev,
    tse_csroffs(frames_transmitted_ok));
    buf[1] = csrrd32(priv.mac_dev,
    tse_csroffs(frames_received_ok));
    buf[2] = csrrd32(priv.mac_dev,
    tse_csroffs(frames_check_sequence_errors));
    buf[3] = csrrd32(priv.mac_dev,
    tse_csroffs(alignment_errors));
// Extended aOctetsTransmittedOK counter
    ext = (u64) csrrd32(priv.mac_dev,
    tse_csroffs(msb_octets_transmitted_ok)) << 32;
    ext |= csrrd32(priv.mac_dev,
    tse_csroffs(octets_transmitted_ok));
    buf[4] = ext;
// Extended aOctetsReceivedOK counter
    ext = (u64) csrrd32(priv.mac_dev,
    tse_csroffs(msb_octets_received_ok)) << 32;
    ext |= csrrd32(priv.mac_dev,
    tse_csroffs(octets_received_ok));
    buf[5] = ext;
    buf[6] = csrrd32(priv.mac_dev,
    tse_csroffs(tx_pause_mac_ctrl_frames));
    buf[7] = csrrd32(priv.mac_dev,
    tse_csroffs(rx_pause_mac_ctrl_frames));
    buf[8] = csrrd32(priv.mac_dev,
    tse_csroffs(if_in_errors));
    buf[9] = csrrd32(priv.mac_dev,
    tse_csroffs(if_out_errors));
    buf[10] = csrrd32(priv.mac_dev,
    tse_csroffs(if_in_ucast_pkts));
    buf[11] = csrrd32(priv.mac_dev,
    tse_csroffs(if_in_multicast_pkts));
    buf[12] = csrrd32(priv.mac_dev,
    tse_csroffs(if_in_broadcast_pkts));
    buf[13] = csrrd32(priv.mac_dev,
    tse_csroffs(if_out_discards));
    buf[14] = csrrd32(priv.mac_dev,
    tse_csroffs(if_out_ucast_pkts));
    buf[15] = csrrd32(priv.mac_dev,
    tse_csroffs(if_out_multicast_pkts));
    buf[16] = csrrd32(priv.mac_dev,
    tse_csroffs(if_out_broadcast_pkts));
    buf[17] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_drop_events));
// Extended etherStatsOctets counter
    ext = (u64) csrrd32(priv.mac_dev,
    tse_csroffs(msb_ether_stats_octets)) << 32;
    ext |= csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_octets));
    buf[18] = ext;
    buf[19] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_pkts));
    buf[20] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_undersize_pkts));
    buf[21] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_oversize_pkts));
    buf[22] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_pkts_64_octets));
    buf[23] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_pkts_65to127_octets));
    buf[24] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_pkts_128to255_octets));
    buf[25] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_pkts_256to511_octets));
    buf[26] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_pkts_512to1023_octets));
    buf[27] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_pkts_1024to1518_octets));
    buf[28] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_pkts_1519tox_octets));
    buf[29] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_jabbers));
    buf[30] = csrrd32(priv.mac_dev,
    tse_csroffs(ether_stats_fragments));
    }
#[no_mangle]
unsafe extern "C" fn tse_sset_count(dev: *mut net_device, sset: c_int) -> c_int {
    static int tse_sset_count(struct net_device *dev, int sset)
    {
    switch (sset) {
    case ETH_SS_STATS:
    return TSE_STATS_LEN;
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn tse_get_msglevel(dev: *mut net_device) -> u32 {
    static u32 tse_get_msglevel(struct net_device *dev)
    {
    struct altera_tse_private *priv = netdev_priv(dev);
    return priv.msg_enable;
    }
#[no_mangle]
unsafe extern "C" fn tse_set_msglevel(dev: *mut net_device, data: u32) {
    static void tse_set_msglevel(struct net_device *dev, uint32_t data)
    {
    struct altera_tse_private *priv = netdev_priv(dev);
    priv.msg_enable = data;
    }
#[no_mangle]
unsafe extern "C" fn tse_reglen(dev: *mut net_device) -> c_int {
    static int tse_reglen(struct net_device *dev)
    {
    return TSE_NUM_REGS * sizeof(u32);
    }
    static void tse_get_regs(struct net_device *dev, struct ethtool_regs *regs,
    void *regbuf)
    {
    struct altera_tse_private *priv = netdev_priv(dev);
    u32 *buf = regbuf;
    int i;
// Set version to a known value, so ethtool knows
// how to do any special formatting of this data.
// This version number will need to change if and
// when this register table is changed.
//
// version[31:0] = 1: Dump the first 128 TSE Registers
// Upper bits are all 0 by default
//
// Upper 16-bits will indicate feature presence for
// Ethtool register decoding in future version.
//
    regs.version = 1;
    for (i = 0; i < TSE_NUM_REGS; i++)
    buf[i] = csrrd32(priv.mac_dev, i * 4);
    }
    static int tse_ethtool_set_link_ksettings(struct net_device *dev,
    const struct ethtool_link_ksettings *cmd)
    {
    struct altera_tse_private *priv = netdev_priv(dev);
    return phylink_ethtool_ksettings_set(priv.phylink, cmd);
    }
    static int tse_ethtool_get_link_ksettings(struct net_device *dev,
    struct ethtool_link_ksettings *cmd)
    {
    struct altera_tse_private *priv = netdev_priv(dev);
    return phylink_ethtool_ksettings_get(priv.phylink, cmd);
    }
    static const struct ethtool_ops tse_ethtool_ops = {
    .get_drvinfo = tse_get_drvinfo,
    .get_regs_len = tse_reglen,
    .get_regs = tse_get_regs,
    .get_link = ethtool_op_get_link,
    .get_strings = tse_gstrings,
    .get_sset_count = tse_sset_count,
    .get_ethtool_stats = tse_fill_stats,
    .get_msglevel = tse_get_msglevel,
    .set_msglevel = tse_set_msglevel,
    .get_link_ksettings = tse_ethtool_get_link_ksettings,
    .set_link_ksettings = tse_ethtool_set_link_ksettings,
    .get_ts_info = ethtool_op_get_ts_info,
    };
#[no_mangle]
pub unsafe extern "C" fn altera_tse_set_ethtool_ops(netdev: *mut net_device) {
    void altera_tse_set_ethtool_ops(struct net_device *netdev)
    {
    netdev.ethtool_ops = &tse_ethtool_ops;
    }
