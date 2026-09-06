//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/et1011c.c
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
// drivers/net/phy/et1011c.c
//
// Driver for LSI ET1011C PHYs
//
// Author: Chaithrika U S
//
// Copyright (c) 2008 Texas Instruments
//

    MODULE_DESCRIPTION("LSI ET1011C PHY driver");
    MODULE_AUTHOR("Chaithrika U S");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn et1011c_config_aneg(phydev: *mut phy_device) -> c_int {
    static int et1011c_config_aneg(struct phy_device *phydev)
    {
    let mut ctl: c_int = phy_read(phydev, MII_BMCR);
    if (ctl < 0)
    return ctl;
    ctl &= ~(BMCR_FULLDPLX | BMCR_SPEED100 | BMCR_SPEED1000 |
    BMCR_ANENABLE);
// First clear the PHY
    phy_write(phydev, MII_BMCR, ctl | BMCR_RESET);
    return genphy_config_aneg(phydev);
    }
#[no_mangle]
unsafe extern "C" fn et1011c_read_status(phydev: *mut phy_device) -> c_int {
    static int et1011c_read_status(struct phy_device *phydev)
    {
    static int speed;
    int ret;
    u32 val;
    ret = genphy_read_status(phydev);
    if (speed != phydev.speed) {
    speed = phydev.speed;
    val = phy_read(phydev, ET1011C_STATUS_REG);
    if ((val & ET1011C_SPEED_MASK) ==
    ET1011C_GIGABIT_SPEED) {
    val = phy_read(phydev, ET1011C_CONFIG_REG);
    val &= ~ET1011C_TX_FIFO_MASK;
    phy_write(phydev, ET1011C_CONFIG_REG, val |
    ET1011C_GMII_INTERFACE |
    ET1011C_SYS_CLK_EN |
    ET1011C_TX_FIFO_DEPTH_16);
    }
    }
    return ret;
    }
    static struct phy_driver et1011c_driver[] = { {
    .phy_id		= 0x0282f014,
    .name		= "ET1011C",
    .phy_id_mask	= 0xfffffff0,
// PHY_GBIT_FEATURES
    .config_aneg	= et1011c_config_aneg,
    .read_status	= et1011c_read_status,
    } };
    module_phy_driver(et1011c_driver);
    static const struct mdio_device_id __maybe_unused et1011c_tbl[] = {
    { 0x0282f014, 0xfffffff0 },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, et1011c_tbl);
