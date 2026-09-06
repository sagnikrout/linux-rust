//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/adin1140-phy.c
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
//
// Driver for Analog Devices, Inc. ADIN1140 10BASE-T1S PHY
//
// Copyright 2026 Analog Devices Inc.
//

pub const ADIN1140_PHY_ID: c_uint = 0x0283be00;
pub const ADIN1140_PCS_CTRL: c_uint = 0x08f3;

#[no_mangle]
unsafe extern "C" fn adin1140_config_aneg(phydev: *mut phy_device) -> c_int {
    static int adin1140_config_aneg(struct phy_device *phydev)
    {
// phylib tries to clear BIT(12) in MDIO_CTRL1, since AN is disabled.
// However, on the ADIN1140, that field is non-standard, being used
// to control the reset status of the PHY (thus it needs to remain set).
//
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adin1140_loopback(phydev: *mut phy_device, enable: bool, speed: c_int) -> c_int {
    static int adin1140_loopback(struct phy_device *phydev, bool enable, int speed)
    {
    if (enable && speed)
    return -EOPNOTSUPP;
    return phy_modify_mmd(phydev, MDIO_MMD_PCS, ADIN1140_PCS_CTRL,
    ADIN1140_PCS_CTRL_LOOPBACK,
    enable ? ADIN1140_PCS_CTRL_LOOPBACK : 0);
    }
#[no_mangle]
unsafe extern "C" fn adin1140_read_status(phydev: *mut phy_device) -> c_int {
    static int adin1140_read_status(struct phy_device *phydev)
    {
    phydev.link = 1;
    phydev.duplex = DUPLEX_HALF;
    phydev.speed = SPEED_10;
    phydev.autoneg = AUTONEG_DISABLE;
    return 0;
    }
    static struct phy_driver adin1140_driver[] = {
    {
    PHY_ID_MATCH_EXACT(ADIN1140_PHY_ID),
    .name = "ADIN1140_PHY",
    .features = PHY_BASIC_T1S_P2MP_FEATURES,
    .read_status = adin1140_read_status,
    .config_aneg = adin1140_config_aneg,
    .set_loopback = adin1140_loopback,
    .read_mmd = genphy_read_mmd_c45,
    .write_mmd = genphy_write_mmd_c45,
    .get_plca_cfg = genphy_c45_plca_get_cfg,
    .set_plca_cfg = genphy_c45_plca_set_cfg,
    .get_plca_status = genphy_c45_plca_get_status,
    },
    };
    module_phy_driver(adin1140_driver);
    static const struct mdio_device_id __maybe_unused adin1140_tbl[] = {
    { PHY_ID_MATCH_EXACT(ADIN1140_PHY_ID) },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, adin1140_tbl);
    MODULE_DESCRIPTION("Analog Devices, Inc. ADIN1140 10BASE-T1S PHY");
    MODULE_AUTHOR("Ciprian Regus <ciprian.regus@analog.com>");
    MODULE_LICENSE("GPL");
