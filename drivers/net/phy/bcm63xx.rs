//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/bcm63xx.c
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
// Driver for Broadcom 63xx SOCs integrated PHYs
//

pub const MII_BCM63XX_IR: c_uint = 0x1a	/* interrupt register */;
pub const MII_BCM63XX_IR_EN: c_uint = 0x4000	/* global interrupt enable */;
pub const MII_BCM63XX_IR_DUPLEX: c_uint = 0x0800	/* duplex changed */;
pub const MII_BCM63XX_IR_SPEED: c_uint = 0x0400	/* speed changed */;
pub const MII_BCM63XX_IR_LINK: c_uint = 0x0200	/* link changed */;
pub const MII_BCM63XX_IR_GMASK: c_uint = 0x0100	/* global interrupt mask */;
    MODULE_DESCRIPTION("Broadcom 63xx internal PHY driver");
    MODULE_AUTHOR("Maxime Bizon <mbizon@freebox.fr>");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn bcm63xx_config_intr(phydev: *mut phy_device) -> c_int {
    static int bcm63xx_config_intr(struct phy_device *phydev)
    {
    int reg, err;
    reg = phy_read(phydev, MII_BCM63XX_IR);
    if (reg < 0)
    return reg;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = bcm_phy_ack_intr(phydev);
    if (err)
    return err;
    reg &= ~MII_BCM63XX_IR_GMASK;
    err = phy_write(phydev, MII_BCM63XX_IR, reg);
    } else {
    reg |= MII_BCM63XX_IR_GMASK;
    err = phy_write(phydev, MII_BCM63XX_IR, reg);
    if (err)
    return err;
    err = bcm_phy_ack_intr(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_config_init(phydev: *mut phy_device) -> c_int {
    static int bcm63xx_config_init(struct phy_device *phydev)
    {
    int reg, err;
// ASYM_PAUSE bit is marked RO in datasheet, so don't cheat
    linkmode_set_bit(ETHTOOL_LINK_MODE_Pause_BIT, phydev.supported);
    reg = phy_read(phydev, MII_BCM63XX_IR);
    if (reg < 0)
    return reg;
// Mask interrupts globally.
    reg |= MII_BCM63XX_IR_GMASK;
    err = phy_write(phydev, MII_BCM63XX_IR, reg);
    if (err < 0)
    return err;
// Unmask events we are interested in
    reg = ~(MII_BCM63XX_IR_DUPLEX |
    MII_BCM63XX_IR_SPEED |
    MII_BCM63XX_IR_LINK) |
    MII_BCM63XX_IR_EN;
    return phy_write(phydev, MII_BCM63XX_IR, reg);
    }
    static struct phy_driver bcm63xx_driver[] = {
    {
    .phy_id		= 0x00406000,
    .phy_id_mask	= 0xfffffc00,
    .name		= "Broadcom BCM63XX (1)",
// PHY_BASIC_FEATURES
    .flags		= PHY_IS_INTERNAL,
    .config_init	= bcm63xx_config_init,
    .config_intr	= bcm63xx_config_intr,
    .handle_interrupt = bcm_phy_handle_interrupt,
    }, {
// same phy as above, with just a different OUI
    .phy_id		= 0x002bdc00,
    .phy_id_mask	= 0xfffffc00,
    .name		= "Broadcom BCM63XX (2)",
// PHY_BASIC_FEATURES
    .flags		= PHY_IS_INTERNAL,
    .config_init	= bcm63xx_config_init,
    .config_intr	= bcm63xx_config_intr,
    .handle_interrupt = bcm_phy_handle_interrupt,
    } };
    module_phy_driver(bcm63xx_driver);
    static const struct mdio_device_id __maybe_unused bcm63xx_tbl[] = {
    { 0x00406000, 0xfffffc00 },
    { 0x002bdc00, 0xfffffc00 },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, bcm63xx_tbl);
