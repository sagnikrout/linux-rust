//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/marvell/libertas/ethtool.c
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

    static void lbs_ethtool_get_drvinfo(struct net_device *dev,
    struct ethtool_drvinfo *info)
    {
    struct lbs_private *priv = dev.ml_priv;
    snprintf(info.fw_version, sizeof(info.fw_version),
    "%u.%u.%u.p%u",
    priv.fwrelease >> 24 & 0xff,
    priv.fwrelease >> 16 & 0xff,
    priv.fwrelease >>  8 & 0xff,
    priv.fwrelease       & 0xff);
    strscpy(info.driver, "libertas", sizeof(info.driver));
    strscpy(info.version, lbs_driver_version, sizeof(info.version));
    }
//
// All 8388 parts have 16KiB EEPROM size at the time of writing.
// In case that changes this needs fixing.
//
pub const LBS_EEPROM_LEN: c_int = 16384;
#[no_mangle]
unsafe extern "C" fn lbs_ethtool_get_eeprom_len(dev: *mut net_device) -> c_int {
    static int lbs_ethtool_get_eeprom_len(struct net_device *dev)
    {
    return LBS_EEPROM_LEN;
    }
    static int lbs_ethtool_get_eeprom(struct net_device *dev,
    struct ethtool_eeprom *eeprom, u8 * bytes)
    {
    struct lbs_private *priv = dev.ml_priv;
    struct cmd_ds_802_11_eeprom_access cmd;
    int ret;
    if (eeprom.offset + eeprom.len > LBS_EEPROM_LEN ||
    eeprom.len > LBS_EEPROM_READ_LEN)
    return -EINVAL;
    cmd.hdr.size = cpu_to_le16(sizeof(struct cmd_ds_802_11_eeprom_access) -
    LBS_EEPROM_READ_LEN + eeprom.len);
    cmd.action = cpu_to_le16(CMD_ACT_GET);
    cmd.offset = cpu_to_le16(eeprom.offset);
    cmd.len    = cpu_to_le16(eeprom.len);
    ret = lbs_cmd_with_response(priv, CMD_802_11_EEPROM_ACCESS, &cmd);
    if (!ret)
    memcpy(bytes, cmd.value, eeprom.len);
    return ret;
    }
    static void lbs_ethtool_get_wol(struct net_device *dev,
    struct ethtool_wolinfo *wol)
    {
    struct lbs_private *priv = dev.ml_priv;
    wol.supported = WAKE_UCAST|WAKE_MCAST|WAKE_BCAST|WAKE_PHY;
    if (priv.wol_criteria == EHS_REMOVE_WAKEUP)
    return;
    if (priv.wol_criteria & EHS_WAKE_ON_UNICAST_DATA)
    wol.wolopts |= WAKE_UCAST;
    if (priv.wol_criteria & EHS_WAKE_ON_MULTICAST_DATA)
    wol.wolopts |= WAKE_MCAST;
    if (priv.wol_criteria & EHS_WAKE_ON_BROADCAST_DATA)
    wol.wolopts |= WAKE_BCAST;
    if (priv.wol_criteria & EHS_WAKE_ON_MAC_EVENT)
    wol.wolopts |= WAKE_PHY;
    }
    static int lbs_ethtool_set_wol(struct net_device *dev,
    struct ethtool_wolinfo *wol)
    {
    struct lbs_private *priv = dev.ml_priv;
    if (wol.wolopts & ~(WAKE_UCAST|WAKE_MCAST|WAKE_BCAST|WAKE_PHY))
    return -EOPNOTSUPP;
    priv.wol_criteria = 0;
    if (wol.wolopts & WAKE_UCAST)
    priv.wol_criteria |= EHS_WAKE_ON_UNICAST_DATA;
    if (wol.wolopts & WAKE_MCAST)
    priv.wol_criteria |= EHS_WAKE_ON_MULTICAST_DATA;
    if (wol.wolopts & WAKE_BCAST)
    priv.wol_criteria |= EHS_WAKE_ON_BROADCAST_DATA;
    if (wol.wolopts & WAKE_PHY)
    priv.wol_criteria |= EHS_WAKE_ON_MAC_EVENT;
    if (wol.wolopts == 0)
    priv.wol_criteria |= EHS_REMOVE_WAKEUP;
    return 0;
    }
    const struct ethtool_ops lbs_ethtool_ops = {
    .get_drvinfo = lbs_ethtool_get_drvinfo,
    .get_eeprom =  lbs_ethtool_get_eeprom,
    .get_eeprom_len = lbs_ethtool_get_eeprom_len,

    .get_sset_count = lbs_mesh_ethtool_get_sset_count,
    .get_ethtool_stats = lbs_mesh_ethtool_get_stats,
    .get_strings = lbs_mesh_ethtool_get_strings,

    .get_wol = lbs_ethtool_get_wol,
    .set_wol = lbs_ethtool_set_wol,
    };
