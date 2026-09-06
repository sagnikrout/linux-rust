//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/ax88796b.c
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
// Driver for Asix PHYs
//
// Author: Michael Schmitz <schmitzmic@gmail.com>
//

pub const PHY_ID_ASIX_AX88772A: c_uint = 0x003b1861;
pub const PHY_ID_ASIX_AX88772C: c_uint = 0x003b1881;
pub const PHY_ID_ASIX_AX88796B: c_uint = 0x003b1841;
    MODULE_DESCRIPTION("Asix PHY driver");
    MODULE_AUTHOR("Michael Schmitz <schmitzmic@gmail.com>");
    MODULE_LICENSE("GPL");
//
// asix_soft_reset - software reset the PHY via BMCR_RESET bit
// @phydev: target phy_device struct
//
// Description: Perform a software PHY reset using the standard
// BMCR_RESET bit and poll for the reset bit to be cleared.
// Toggle BMCR_RESET bit off to accommodate broken AX8796B PHY implementation
// such as used on the Individual Computers' X-Surf 100 Zorro card.
//
// Returns: 0 on success, < 0 on failure
//
#[no_mangle]
unsafe extern "C" fn asix_soft_reset(phydev: *mut phy_device) -> c_int {
    static int asix_soft_reset(struct phy_device *phydev)
    {
    int ret;
// Asix PHY won't reset unless reset bit toggles
    ret = phy_write(phydev, MII_BMCR, 0);
    if (ret < 0)
    return ret;
    return genphy_soft_reset(phydev);
    }
// AX88772A is not working properly with some old switches (NETGEAR EN 108TP):
// after autoneg is done and the link status is reported as active, the MII_LPA
// register is 0. This issue is not reproducible on AX88772C.
//
#[no_mangle]
unsafe extern "C" fn asix_ax88772a_read_status(phydev: *mut phy_device) -> c_int {
    static int asix_ax88772a_read_status(struct phy_device *phydev)
    {
    int ret, val;
    ret = genphy_update_link(phydev);
    if (ret)
    return ret;
    if (!phydev.link)
    return 0;
// If MII_LPA is 0, phy_resolve_aneg_linkmode() will fail to resolve
// linkmode so use MII_BMCR as default values.
//
    val = phy_read(phydev, MII_BMCR);
    if (val < 0)
    return val;
    if (val & BMCR_SPEED100)
    phydev.speed = SPEED_100;
    else
    phydev.speed = SPEED_10;
    if (val & BMCR_FULLDPLX)
    phydev.duplex = DUPLEX_FULL;
    else
    phydev.duplex = DUPLEX_HALF;
    ret = genphy_read_lpa(phydev);
    if (ret < 0)
    return ret;
    if (phydev.autoneg == AUTONEG_ENABLE && phydev.autoneg_complete)
    phy_resolve_aneg_linkmode(phydev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asix_ax88772a_link_change_notify(phydev: *mut phy_device) {
    static void asix_ax88772a_link_change_notify(struct phy_device *phydev)
    {
// Reset PHY, otherwise MII_LPA will provide outdated information.
// This issue is reproducible only with some link partner PHYs
//
    if (phydev.state == PHY_NOLINK) {
    phy_init_hw(phydev);
    _phy_start_aneg(phydev);
    }
    }
    static struct phy_driver asix_driver[] = {
    {
    PHY_ID_MATCH_EXACT(PHY_ID_ASIX_AX88772A),
    .name		= "Asix Electronics AX88772A",
    .flags		= PHY_IS_INTERNAL,
    .read_status	= asix_ax88772a_read_status,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    .soft_reset	= asix_soft_reset,
    .link_change_notify	= asix_ax88772a_link_change_notify,
    }, {
    PHY_ID_MATCH_EXACT(PHY_ID_ASIX_AX88772C),
    .name		= "Asix Electronics AX88772C",
    .flags		= PHY_IS_INTERNAL,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    .soft_reset	= asix_soft_reset,
    }, {
    PHY_ID_MATCH_MODEL(PHY_ID_ASIX_AX88796B),
    .name		= "Asix Electronics AX88796B",
// PHY_BASIC_FEATURES
    .soft_reset	= asix_soft_reset,
    } };
    module_phy_driver(asix_driver);
    static const struct mdio_device_id __maybe_unused asix_tbl[] = {
    { PHY_ID_MATCH_EXACT(PHY_ID_ASIX_AX88772A) },
    { PHY_ID_MATCH_EXACT(PHY_ID_ASIX_AX88772C) },
    { PHY_ID_MATCH_MODEL(PHY_ID_ASIX_AX88796B) },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, asix_tbl);
