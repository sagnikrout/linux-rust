//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/davicom.c
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
// drivers/net/phy/davicom.c
//
// Driver for Davicom PHYs
//
// Author: Andy Fleming
//
// Copyright (c) 2004 Freescale Semiconductor, Inc.
//

pub const MII_DM9161_SCR: c_uint = 0x10;
pub const MII_DM9161_SCR_INIT: c_uint = 0x0610;
pub const MII_DM9161_SCR_RMII: c_uint = 0x0100;
// DM9161 Interrupt Register
pub const MII_DM9161_INTR: c_uint = 0x15;
pub const MII_DM9161_INTR_PEND: c_uint = 0x8000;
pub const MII_DM9161_INTR_DPLX_MASK: c_uint = 0x0800;
pub const MII_DM9161_INTR_SPD_MASK: c_uint = 0x0400;
pub const MII_DM9161_INTR_LINK_MASK: c_uint = 0x0200;
pub const MII_DM9161_INTR_MASK: c_uint = 0x0100;
pub const MII_DM9161_INTR_DPLX_CHANGE: c_uint = 0x0010;
pub const MII_DM9161_INTR_SPD_CHANGE: c_uint = 0x0008;
pub const MII_DM9161_INTR_LINK_CHANGE: c_uint = 0x0004;
pub const MII_DM9161_INTR_INIT: c_uint = 0x0000;

    (MII_DM9161_INTR_DPLX_MASK | MII_DM9161_INTR_SPD_MASK |	\
    MII_DM9161_INTR_LINK_MASK | MII_DM9161_INTR_MASK)

    (MII_DM9161_INTR_DPLX_CHANGE | \
    MII_DM9161_INTR_SPD_CHANGE | \
    MII_DM9161_INTR_LINK_CHANGE)
// DM9161 10BT Configuration/Status
pub const MII_DM9161_10BTCSR: c_uint = 0x12;
pub const MII_DM9161_10BTCSR_INIT: c_uint = 0x7800;
    MODULE_DESCRIPTION("Davicom PHY driver");
    MODULE_AUTHOR("Andy Fleming");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn dm9161_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int dm9161_ack_interrupt(struct phy_device *phydev)
    {
    let mut err: c_int = phy_read(phydev, MII_DM9161_INTR);
    return (err < 0) ? err : 0;
    }
pub const DM9161_DELAY: c_int = 1;
#[no_mangle]
unsafe extern "C" fn dm9161_config_intr(phydev: *mut phy_device) -> c_int {
    static int dm9161_config_intr(struct phy_device *phydev)
    {
    int temp, err;
    temp = phy_read(phydev, MII_DM9161_INTR);
    if (temp < 0)
    return temp;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = dm9161_ack_interrupt(phydev);
    if (err)
    return err;
    temp &= ~(MII_DM9161_INTR_STOP);
    err = phy_write(phydev, MII_DM9161_INTR, temp);
    } else {
    temp |= MII_DM9161_INTR_STOP;
    err = phy_write(phydev, MII_DM9161_INTR, temp);
    if (err)
    return err;
    err = dm9161_ack_interrupt(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dm9161_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t dm9161_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read(phydev, MII_DM9161_INTR);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & MII_DM9161_INTR_CHANGE))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dm9161_config_aneg(phydev: *mut phy_device) -> c_int {
    static int dm9161_config_aneg(struct phy_device *phydev)
    {
    int err;
// Isolate the PHY
    err = phy_write(phydev, MII_BMCR, BMCR_ISOLATE);
    if (err < 0)
    return err;
// Configure the new settings
    err = genphy_config_aneg(phydev);
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dm9161_config_init(phydev: *mut phy_device) -> c_int {
    static int dm9161_config_init(struct phy_device *phydev)
    {
    int err, temp;
// Isolate the PHY
    err = phy_write(phydev, MII_BMCR, BMCR_ISOLATE);
    if (err < 0)
    return err;
    switch (phydev.interface) {
    case PHY_INTERFACE_MODE_MII:
    temp = MII_DM9161_SCR_INIT;
    break;
    case PHY_INTERFACE_MODE_RMII:
    temp =  MII_DM9161_SCR_INIT | MII_DM9161_SCR_RMII;
    break;
    default:
    return -EINVAL;
    }
// Do not bypass the scrambler/descrambler
    err = phy_write(phydev, MII_DM9161_SCR, temp);
    if (err < 0)
    return err;
// Clear 10BTCSR to default
    err = phy_write(phydev, MII_DM9161_10BTCSR, MII_DM9161_10BTCSR_INIT);
    if (err < 0)
    return err;
// Reconnect the PHY, and enable Autonegotiation
    return phy_write(phydev, MII_BMCR, BMCR_ANENABLE);
    }
    static struct phy_driver dm91xx_driver[] = {
    {
    .phy_id		= 0x0181b880,
    .name		= "Davicom DM9161E",
    .phy_id_mask	= 0x0ffffff0,
// PHY_BASIC_FEATURES
    .config_init	= dm9161_config_init,
    .config_aneg	= dm9161_config_aneg,
    .config_intr	= dm9161_config_intr,
    .handle_interrupt = dm9161_handle_interrupt,
    }, {
    .phy_id		= 0x0181b8b0,
    .name		= "Davicom DM9161B/C",
    .phy_id_mask	= 0x0ffffff0,
// PHY_BASIC_FEATURES
    .config_init	= dm9161_config_init,
    .config_aneg	= dm9161_config_aneg,
    .config_intr	= dm9161_config_intr,
    .handle_interrupt = dm9161_handle_interrupt,
    }, {
    .phy_id		= 0x0181b8a0,
    .name		= "Davicom DM9161A",
    .phy_id_mask	= 0x0ffffff0,
// PHY_BASIC_FEATURES
    .config_init	= dm9161_config_init,
    .config_aneg	= dm9161_config_aneg,
    .config_intr	= dm9161_config_intr,
    .handle_interrupt = dm9161_handle_interrupt,
    }, {
    .phy_id		= 0x00181b80,
    .name		= "Davicom DM9131",
    .phy_id_mask	= 0x0ffffff0,
// PHY_BASIC_FEATURES
    .config_intr	= dm9161_config_intr,
    .handle_interrupt = dm9161_handle_interrupt,
    } };
    module_phy_driver(dm91xx_driver);
    static const struct mdio_device_id __maybe_unused davicom_tbl[] = {
    { 0x0181b880, 0x0ffffff0 },
    { 0x0181b8b0, 0x0ffffff0 },
    { 0x0181b8a0, 0x0ffffff0 },
    { 0x00181b80, 0x0ffffff0 },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, davicom_tbl);
