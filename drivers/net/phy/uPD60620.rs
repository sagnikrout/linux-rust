//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/uPD60620.c
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
// Driver for the Renesas PHY uPD60620.
//
// Copyright (C) 2015 Softing Industrial Automation GmbH
//

pub const UPD60620_PHY_ID: c_uint = 0xb8242824;
// Extended Registers and values
// PHY Special Control/Status
pub const PHY_PHYSCR: c_uint = 0x1F      /* PHY.31 */;
pub const PHY_PHYSCR_10MB: c_uint = 0x0004    /* PHY speed = 10mb */;
pub const PHY_PHYSCR_100MB: c_uint = 0x0008    /* PHY speed = 100mb */;
pub const PHY_PHYSCR_DUPLEX: c_uint = 0x0010    /* PHY Duplex */;
// PHY Special Modes
pub const PHY_SPM: c_uint = 0x12      /* PHY.18 */;
// Init PHY
#[no_mangle]
unsafe extern "C" fn upd60620_config_init(phydev: *mut phy_device) -> c_int {
    static int upd60620_config_init(struct phy_device *phydev)
    {
// Enable support for passive HUBs (could be a strap option)
// PHYMODE: All speeds, HD in parallel detect
    return phy_write(phydev, PHY_SPM, 0x0180 | phydev.mdio.addr);
    }
// Get PHY status from common registers
#[no_mangle]
unsafe extern "C" fn upd60620_read_status(phydev: *mut phy_device) -> c_int {
    static int upd60620_read_status(struct phy_device *phydev)
    {
    int phy_state;
// Read negotiated state
    phy_state = phy_read(phydev, MII_BMSR);
    if (phy_state < 0)
    return phy_state;
    phydev.link = 0;
    linkmode_zero(phydev.lp_advertising);
    phydev.pause = 0;
    phydev.asym_pause = 0;
    if (phy_state & (BMSR_ANEGCOMPLETE | BMSR_LSTATUS)) {
    phy_state = phy_read(phydev, PHY_PHYSCR);
    if (phy_state < 0)
    return phy_state;
    if (phy_state & (PHY_PHYSCR_10MB | PHY_PHYSCR_100MB)) {
    phydev.link = 1;
    phydev.speed = SPEED_10;
    phydev.duplex = DUPLEX_HALF;
    if (phy_state & PHY_PHYSCR_100MB)
    phydev.speed = SPEED_100;
    if (phy_state & PHY_PHYSCR_DUPLEX)
    phydev.duplex = DUPLEX_FULL;
    phy_state = phy_read(phydev, MII_LPA);
    if (phy_state < 0)
    return phy_state;
    mii_lpa_to_linkmode_lpa_t(phydev.lp_advertising,
    phy_state);
    phy_resolve_aneg_pause(phydev);
    }
    }
    return 0;
    }
    MODULE_DESCRIPTION("Renesas uPD60620 PHY driver");
    MODULE_AUTHOR("Bernd Edlinger <bernd.edlinger@hotmail.de>");
    MODULE_LICENSE("GPL");
    static struct phy_driver upd60620_driver[1] = { {
    .phy_id         = UPD60620_PHY_ID,
    .phy_id_mask    = 0xfffffffe,
    .name           = "Renesas uPD60620",
// PHY_BASIC_FEATURES
    .flags          = 0,
    .config_init    = upd60620_config_init,
    .read_status    = upd60620_read_status,
    } };
    module_phy_driver(upd60620_driver);
    static const struct mdio_device_id __maybe_unused upd60620_tbl[] = {
    { UPD60620_PHY_ID, 0xfffffffe },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, upd60620_tbl);
