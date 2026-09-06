//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/dp83tc811.c
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
// Driver for the Texas Instruments DP83TC811 PHY
//
// Copyright (C) 2018 Texas Instruments Incorporated - http://www.ti.com
//

pub const DP83TC811_PHY_ID: c_uint = 0x2000a253;
pub const DP83811_DEVADDR: c_uint = 0x1f;
pub const MII_DP83811_SGMII_CTRL: c_uint = 0x09;
pub const MII_DP83811_INT_STAT1: c_uint = 0x12;
pub const MII_DP83811_INT_STAT2: c_uint = 0x13;
pub const MII_DP83811_INT_STAT3: c_uint = 0x18;
pub const MII_DP83811_RESET_CTRL: c_uint = 0x1f;

// INT_STAT1 bits

// INT_STAT2 bits

// INT_STAT3 bits

pub const MII_DP83811_RXSOP1: c_uint = 0x04a5;
pub const MII_DP83811_RXSOP2: c_uint = 0x04a6;
pub const MII_DP83811_RXSOP3: c_uint = 0x04a7;
// WoL Registers
pub const MII_DP83811_WOL_CFG: c_uint = 0x04a0;
pub const MII_DP83811_WOL_STAT: c_uint = 0x04a1;
pub const MII_DP83811_WOL_DA1: c_uint = 0x04a2;
pub const MII_DP83811_WOL_DA2: c_uint = 0x04a3;
pub const MII_DP83811_WOL_DA3: c_uint = 0x04a4;
// WoL bits

// SGMII CTRL bits

#[no_mangle]
unsafe extern "C" fn dp83811_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int dp83811_ack_interrupt(struct phy_device *phydev)
    {
    int err;
    err = phy_read(phydev, MII_DP83811_INT_STAT1);
    if (err < 0)
    return err;
    err = phy_read(phydev, MII_DP83811_INT_STAT2);
    if (err < 0)
    return err;
    err = phy_read(phydev, MII_DP83811_INT_STAT3);
    if (err < 0)
    return err;
    return 0;
    }
    static int dp83811_set_wol(struct phy_device *phydev,
    struct ethtool_wolinfo *wol)
    {
    struct net_device *ndev = phydev.attached_dev;
    const u8 *mac;
    u16 value;
    if (wol.wolopts & (WAKE_MAGIC | WAKE_MAGICSECURE)) {
    mac = (const u8 *)ndev.dev_addr;
    if (!is_valid_ether_addr(mac))
    return -EINVAL;
// MAC addresses start with byte 5, but stored in mac[0].
// 811 PHYs store bytes 4|5, 2|3, 0|1
//
    phy_write_mmd(phydev, DP83811_DEVADDR, MII_DP83811_WOL_DA1,
    (mac[1] << 8) | mac[0]);
    phy_write_mmd(phydev, DP83811_DEVADDR, MII_DP83811_WOL_DA2,
    (mac[3] << 8) | mac[2]);
    phy_write_mmd(phydev, DP83811_DEVADDR, MII_DP83811_WOL_DA3,
    (mac[5] << 8) | mac[4]);
    value = phy_read_mmd(phydev, DP83811_DEVADDR,
    MII_DP83811_WOL_CFG);
    if (wol.wolopts & WAKE_MAGIC)
    value |= DP83811_WOL_MAGIC_EN;
    else
    value &= ~DP83811_WOL_MAGIC_EN;
    if (wol.wolopts & WAKE_MAGICSECURE) {
    phy_write_mmd(phydev, DP83811_DEVADDR,
    MII_DP83811_RXSOP1,
    (wol.sopass[1] << 8) | wol.sopass[0]);
    phy_write_mmd(phydev, DP83811_DEVADDR,
    MII_DP83811_RXSOP2,
    (wol.sopass[3] << 8) | wol.sopass[2]);
    phy_write_mmd(phydev, DP83811_DEVADDR,
    MII_DP83811_RXSOP3,
    (wol.sopass[5] << 8) | wol.sopass[4]);
    value |= DP83811_WOL_SECURE_ON;
    } else {
    value &= ~DP83811_WOL_SECURE_ON;
    }
// Clear any pending WoL interrupt
    phy_read(phydev, MII_DP83811_INT_STAT1);
    value |= DP83811_WOL_EN | DP83811_WOL_INDICATION_SEL |
    DP83811_WOL_CLR_INDICATION;
    return phy_write_mmd(phydev, DP83811_DEVADDR,
    MII_DP83811_WOL_CFG, value);
    } else {
    return phy_clear_bits_mmd(phydev, DP83811_DEVADDR,
    MII_DP83811_WOL_CFG, DP83811_WOL_EN);
    }
    }
    static void dp83811_get_wol(struct phy_device *phydev,
    struct ethtool_wolinfo *wol)
    {
    u16 sopass_val;
    int value;
    wol.supported = (WAKE_MAGIC | WAKE_MAGICSECURE);
    wol.wolopts = 0;
    value = phy_read_mmd(phydev, DP83811_DEVADDR, MII_DP83811_WOL_CFG);
    if (value & DP83811_WOL_MAGIC_EN)
    wol.wolopts |= WAKE_MAGIC;
    if (value & DP83811_WOL_SECURE_ON) {
    sopass_val = phy_read_mmd(phydev, DP83811_DEVADDR,
    MII_DP83811_RXSOP1);
    wol.sopass[0] = (sopass_val & 0xff);
    wol.sopass[1] = (sopass_val >> 8);
    sopass_val = phy_read_mmd(phydev, DP83811_DEVADDR,
    MII_DP83811_RXSOP2);
    wol.sopass[2] = (sopass_val & 0xff);
    wol.sopass[3] = (sopass_val >> 8);
    sopass_val = phy_read_mmd(phydev, DP83811_DEVADDR,
    MII_DP83811_RXSOP3);
    wol.sopass[4] = (sopass_val & 0xff);
    wol.sopass[5] = (sopass_val >> 8);
    wol.wolopts |= WAKE_MAGICSECURE;
    }
// WoL is not enabled so set wolopts to 0
    if (!(value & DP83811_WOL_EN))
    wol.wolopts = 0;
    }
#[no_mangle]
unsafe extern "C" fn dp83811_config_intr(phydev: *mut phy_device) -> c_int {
    static int dp83811_config_intr(struct phy_device *phydev)
    {
    int misr_status, err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = dp83811_ack_interrupt(phydev);
    if (err)
    return err;
    misr_status = phy_read(phydev, MII_DP83811_INT_STAT1);
    if (misr_status < 0)
    return misr_status;
    misr_status |= (DP83811_RX_ERR_HF_INT_EN |
    DP83811_MS_TRAINING_INT_EN |
    DP83811_ANEG_COMPLETE_INT_EN |
    DP83811_ESD_EVENT_INT_EN |
    DP83811_WOL_INT_EN |
    DP83811_LINK_STAT_INT_EN |
    DP83811_ENERGY_DET_INT_EN |
    DP83811_LINK_QUAL_INT_EN);
    err = phy_write(phydev, MII_DP83811_INT_STAT1, misr_status);
    if (err < 0)
    return err;
    misr_status = phy_read(phydev, MII_DP83811_INT_STAT2);
    if (misr_status < 0)
    return misr_status;
    misr_status |= (DP83811_JABBER_DET_INT_EN |
    DP83811_POLARITY_INT_EN |
    DP83811_SLEEP_MODE_INT_EN |
    DP83811_OVERTEMP_INT_EN |
    DP83811_OVERVOLTAGE_INT_EN |
    DP83811_UNDERVOLTAGE_INT_EN);
    err = phy_write(phydev, MII_DP83811_INT_STAT2, misr_status);
    if (err < 0)
    return err;
    misr_status = phy_read(phydev, MII_DP83811_INT_STAT3);
    if (misr_status < 0)
    return misr_status;
    misr_status |= (DP83811_LPS_INT_EN |
    DP83811_NO_FRAME_INT_EN |
    DP83811_POR_DONE_INT_EN);
    err = phy_write(phydev, MII_DP83811_INT_STAT3, misr_status);
    } else {
    err = phy_write(phydev, MII_DP83811_INT_STAT1, 0);
    if (err < 0)
    return err;
    err = phy_write(phydev, MII_DP83811_INT_STAT2, 0);
    if (err < 0)
    return err;
    err = phy_write(phydev, MII_DP83811_INT_STAT3, 0);
    if (err < 0)
    return err;
    err = dp83811_ack_interrupt(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dp83811_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t dp83811_handle_interrupt(struct phy_device *phydev)
    {
    let mut trigger_machine: bool = false;
    int irq_status;
// The INT_STAT registers 1, 2 and 3 are holding the interrupt status
// in the upper half (15:8), while the lower half (7:0) is used for
// controlling the interrupt enable state of those individual interrupt
// sources. To determine the possible interrupt sources, just read the
// INT_STAT* register and use it directly to know which interrupts have
// been enabled previously or not.
//
    irq_status = phy_read(phydev, MII_DP83811_INT_STAT1);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (irq_status & ((irq_status & GENMASK(7, 0)) << 8))
    trigger_machine = true;
    irq_status = phy_read(phydev, MII_DP83811_INT_STAT2);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (irq_status & ((irq_status & GENMASK(7, 0)) << 8))
    trigger_machine = true;
    irq_status = phy_read(phydev, MII_DP83811_INT_STAT3);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (irq_status & ((irq_status & GENMASK(7, 0)) << 8))
    trigger_machine = true;
    if (!trigger_machine)
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dp83811_config_aneg(phydev: *mut phy_device) -> c_int {
    static int dp83811_config_aneg(struct phy_device *phydev)
    {
    int value, err;
    if (phydev.interface == PHY_INTERFACE_MODE_SGMII) {
    value = phy_read(phydev, MII_DP83811_SGMII_CTRL);
    if (phydev.autoneg == AUTONEG_ENABLE) {
    err = phy_write(phydev, MII_DP83811_SGMII_CTRL,
    (DP83811_SGMII_AUTO_NEG_EN | value));
    if (err < 0)
    return err;
    } else {
    err = phy_write(phydev, MII_DP83811_SGMII_CTRL,
    (~DP83811_SGMII_AUTO_NEG_EN & value));
    if (err < 0)
    return err;
    }
    }
    return genphy_config_aneg(phydev);
    }
#[no_mangle]
unsafe extern "C" fn dp83811_config_init(phydev: *mut phy_device) -> c_int {
    static int dp83811_config_init(struct phy_device *phydev)
    {
    int value, err;
    value = phy_read(phydev, MII_DP83811_SGMII_CTRL);
    if (phydev.interface == PHY_INTERFACE_MODE_SGMII) {
    err = phy_write(phydev, MII_DP83811_SGMII_CTRL,
    (DP83811_SGMII_EN | value));
    } else {
    err = phy_write(phydev, MII_DP83811_SGMII_CTRL,
    (~DP83811_SGMII_EN & value));
    }
    if (err < 0)
    return err;
    value = DP83811_WOL_MAGIC_EN | DP83811_WOL_SECURE_ON | DP83811_WOL_EN;
    return phy_clear_bits_mmd(phydev, DP83811_DEVADDR, MII_DP83811_WOL_CFG,
    value);
    }
#[no_mangle]
unsafe extern "C" fn dp83811_phy_reset(phydev: *mut phy_device) -> c_int {
    static int dp83811_phy_reset(struct phy_device *phydev)
    {
    int err;
    err = phy_write(phydev, MII_DP83811_RESET_CTRL, DP83811_HW_RESET);
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dp83811_suspend(phydev: *mut phy_device) -> c_int {
    static int dp83811_suspend(struct phy_device *phydev)
    {
    int value;
    value = phy_read_mmd(phydev, DP83811_DEVADDR, MII_DP83811_WOL_CFG);
    if (!(value & DP83811_WOL_EN))
    genphy_suspend(phydev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dp83811_resume(phydev: *mut phy_device) -> c_int {
    static int dp83811_resume(struct phy_device *phydev)
    {
    genphy_resume(phydev);
    phy_set_bits_mmd(phydev, DP83811_DEVADDR, MII_DP83811_WOL_CFG,
    DP83811_WOL_CLR_INDICATION);
    return 0;
    }
    static struct phy_driver dp83811_driver[] = {
    {
    .phy_id = DP83TC811_PHY_ID,
    .phy_id_mask = 0xfffffff0,
    .name = "TI DP83TC811",
// PHY_BASIC_FEATURES
    .config_init = dp83811_config_init,
    .config_aneg = dp83811_config_aneg,
    .soft_reset = dp83811_phy_reset,
    .get_features = genphy_c45_pma_read_ext_abilities,
    .get_wol = dp83811_get_wol,
    .set_wol = dp83811_set_wol,
    .config_intr = dp83811_config_intr,
    .handle_interrupt = dp83811_handle_interrupt,
    .suspend = dp83811_suspend,
    .resume = dp83811_resume,
    },
    };
    module_phy_driver(dp83811_driver);
    static const struct mdio_device_id __maybe_unused dp83811_tbl[] = {
    { DP83TC811_PHY_ID, 0xfffffff0 },
    { },
    };
    MODULE_DEVICE_TABLE(mdio, dp83811_tbl);
    MODULE_DESCRIPTION("Texas Instruments DP83TC811 PHY driver");
    MODULE_AUTHOR("Dan Murphy <dmurphy@ti.com");
    MODULE_LICENSE("GPL");
