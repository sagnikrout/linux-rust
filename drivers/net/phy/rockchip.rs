//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/rockchip.c
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
// drivers/net/phy/rockchip.c
//
// Driver for ROCKCHIP Ethernet PHYs
//
// Copyright (c) 2017, Fuzhou Rockchip Electronics Co., Ltd
//
// David Wu <david.wu@rock-chips.com>
//

pub const INTERNAL_EPHY_ID: c_uint = 0x1234d400;
pub const MII_INTERNAL_CTRL_STATUS: c_int = 17;
pub const SMI_ADDR_TSTCNTL: c_int = 20;
pub const SMI_ADDR_TSTREAD1: c_int = 21;
pub const SMI_ADDR_TSTREAD2: c_int = 22;
pub const SMI_ADDR_TSTWRITE: c_int = 23;
pub const MII_SPECIAL_CONTROL_STATUS: c_int = 31;

pub const TSTMODE_ENABLE: c_uint = 0x400;
pub const TSTMODE_DISABLE: c_uint = 0x0;
pub const WR_ADDR_A7CFG: c_uint = 0x18;
#[no_mangle]
unsafe extern "C" fn rockchip_init_tstmode(phydev: *mut phy_device) -> c_int {
    static int rockchip_init_tstmode(struct phy_device *phydev)
    {
    int ret;
// Enable access to Analog and DSP register banks
    ret = phy_write(phydev, SMI_ADDR_TSTCNTL, TSTMODE_ENABLE);
    if (ret)
    return ret;
    ret = phy_write(phydev, SMI_ADDR_TSTCNTL, TSTMODE_DISABLE);
    if (ret)
    return ret;
    return phy_write(phydev, SMI_ADDR_TSTCNTL, TSTMODE_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_close_tstmode(phydev: *mut phy_device) -> c_int {
    static int rockchip_close_tstmode(struct phy_device *phydev)
    {
// Back to basic register bank
    return phy_write(phydev, SMI_ADDR_TSTCNTL, TSTMODE_DISABLE);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_integrated_phy_analog_init(phydev: *mut phy_device) -> c_int {
    static int rockchip_integrated_phy_analog_init(struct phy_device *phydev)
    {
    int ret;
    ret = rockchip_init_tstmode(phydev);
    if (ret)
    return ret;
//
// Adjust tx amplitude to make sginal better,
// the default value is 0x8.
//
    ret = phy_write(phydev, SMI_ADDR_TSTWRITE, 0xB);
    if (ret)
    return ret;
    ret = phy_write(phydev, SMI_ADDR_TSTCNTL, TSTCNTL_WR | WR_ADDR_A7CFG);
    if (ret)
    return ret;
    return rockchip_close_tstmode(phydev);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_integrated_phy_config_init(phydev: *mut phy_device) -> c_int {
    static int rockchip_integrated_phy_config_init(struct phy_device *phydev)
    {
    int val, ret;
//
// The auto MIDX has linked problem on some board,
// workround to disable auto MDIX.
//
    val = phy_read(phydev, MII_INTERNAL_CTRL_STATUS);
    if (val < 0)
    return val;
    val &= ~MII_AUTO_MDIX_EN;
    ret = phy_write(phydev, MII_INTERNAL_CTRL_STATUS, val);
    if (ret)
    return ret;
    return rockchip_integrated_phy_analog_init(phydev);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_link_change_notify(phydev: *mut phy_device) {
    static void rockchip_link_change_notify(struct phy_device *phydev)
    {
//
// If mode switch happens from 10BT to 100BT, all DSP/AFE
// registers are set to default values. So any AFE/DSP
// registers have to be re-initialized in this case.
//
    if (phydev.state == PHY_RUNNING && phydev.speed == SPEED_100) {
    let mut ret: c_int = rockchip_integrated_phy_analog_init(phydev);
    if (ret)
    phydev_err(phydev, "rockchip_integrated_phy_analog_init err: %d.\n",
    ret);
    }
    }
#[no_mangle]
unsafe extern "C" fn rockchip_set_polarity(phydev: *mut phy_device, polarity: c_int) -> c_int {
    static int rockchip_set_polarity(struct phy_device *phydev, int polarity)
    {
    int reg, err, val;
// get the current settings
    reg = phy_read(phydev, MII_INTERNAL_CTRL_STATUS);
    if (reg < 0)
    return reg;
    reg &= ~MII_AUTO_MDIX_EN;
    val = reg;
    switch (polarity) {
    case ETH_TP_MDI:
    val &= ~MII_MDIX_EN;
    break;
    case ETH_TP_MDI_X:
    val |= MII_MDIX_EN;
    break;
    case ETH_TP_MDI_AUTO:
    case ETH_TP_MDI_INVALID:
    default:
    return 0;
    }
    if (val != reg) {
// Set the new polarity value in the register
    err = phy_write(phydev, MII_INTERNAL_CTRL_STATUS, val);
    if (err)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_config_aneg(phydev: *mut phy_device) -> c_int {
    static int rockchip_config_aneg(struct phy_device *phydev)
    {
    int err;
    err = rockchip_set_polarity(phydev, phydev.mdix);
    if (err < 0)
    return err;
    return genphy_config_aneg(phydev);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_phy_resume(phydev: *mut phy_device) -> c_int {
    static int rockchip_phy_resume(struct phy_device *phydev)
    {
    genphy_resume(phydev);
    return rockchip_integrated_phy_config_init(phydev);
    }
    static struct phy_driver rockchip_phy_driver[] = {
    {
    .phy_id			= INTERNAL_EPHY_ID,
    .phy_id_mask		= 0xfffffff0,
    .name			= "Rockchip integrated EPHY",
// PHY_BASIC_FEATURES
    .flags			= 0,
    .link_change_notify	= rockchip_link_change_notify,
    .soft_reset		= genphy_soft_reset,
    .config_init		= rockchip_integrated_phy_config_init,
    .config_aneg		= rockchip_config_aneg,
    .suspend		= genphy_suspend,
    .resume			= rockchip_phy_resume,
    },
    };
    module_phy_driver(rockchip_phy_driver);
    static const struct mdio_device_id __maybe_unused rockchip_phy_tbl[] = {
    { INTERNAL_EPHY_ID, 0xfffffff0 },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, rockchip_phy_tbl);
    MODULE_AUTHOR("David Wu <david.wu@rock-chips.com>");
    MODULE_DESCRIPTION("Rockchip Ethernet PHY driver");
    MODULE_LICENSE("GPL");
