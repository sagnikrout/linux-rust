//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/ste10Xp.c
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
// drivers/net/phy/ste10Xp.c
//
// Driver for STMicroelectronics STe10Xp PHYs
//
// Author: Giuseppe Cavallaro <peppe.cavallaro@st.com>
//
// Copyright (c) 2008 STMicroelectronics Limited
//

pub const MII_XCIIS: c_uint = 0x11	/* Configuration Info IRQ & Status Reg */;
pub const MII_XIE: c_uint = 0x12	/* Interrupt Enable Register */;
pub const MII_XIE_DEFAULT_MASK: c_uint = 0x0070 /* ANE complete, Remote Fault, Link Down */;
pub const STE101P_PHY_ID: c_uint = 0x00061c50;
pub const STE100P_PHY_ID: c_uint = 0x1c040011;
#[no_mangle]
unsafe extern "C" fn ste10Xp_config_init(phydev: *mut phy_device) -> c_int {
    static int ste10Xp_config_init(struct phy_device *phydev)
    {
    int value, err;
// Software Reset PHY
    value = phy_read(phydev, MII_BMCR);
    if (value < 0)
    return value;
    value |= BMCR_RESET;
    err = phy_write(phydev, MII_BMCR, value);
    if (err < 0)
    return err;
    do {
    value = phy_read(phydev, MII_BMCR);
    } while (value & BMCR_RESET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ste10Xp_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int ste10Xp_ack_interrupt(struct phy_device *phydev)
    {
    let mut err: c_int = phy_read(phydev, MII_XCIIS);
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ste10Xp_config_intr(phydev: *mut phy_device) -> c_int {
    static int ste10Xp_config_intr(struct phy_device *phydev)
    {
    int err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
// clear any pending interrupts
    err = ste10Xp_ack_interrupt(phydev);
    if (err)
    return err;
// Enable all STe101P interrupts (PR12)
    err = phy_write(phydev, MII_XIE, MII_XIE_DEFAULT_MASK);
    } else {
    err = phy_write(phydev, MII_XIE, 0);
    if (err)
    return err;
    err = ste10Xp_ack_interrupt(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ste10Xp_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t ste10Xp_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read(phydev, MII_XCIIS);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & MII_XIE_DEFAULT_MASK))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
    static struct phy_driver ste10xp_pdriver[] = {
    {
    .phy_id = STE101P_PHY_ID,
    .phy_id_mask = 0xfffffff0,
    .name = "STe101p",
// PHY_BASIC_FEATURES
    .config_init = ste10Xp_config_init,
    .config_intr = ste10Xp_config_intr,
    .handle_interrupt = ste10Xp_handle_interrupt,
    .suspend = genphy_suspend,
    .resume = genphy_resume,
    }, {
    .phy_id = STE100P_PHY_ID,
    .phy_id_mask = 0xffffffff,
    .name = "STe100p",
// PHY_BASIC_FEATURES
    .config_init = ste10Xp_config_init,
    .config_intr = ste10Xp_config_intr,
    .handle_interrupt = ste10Xp_handle_interrupt,
    .suspend = genphy_suspend,
    .resume = genphy_resume,
    } };
    module_phy_driver(ste10xp_pdriver);
    static const struct mdio_device_id __maybe_unused ste10Xp_tbl[] = {
    { STE101P_PHY_ID, 0xfffffff0 },
    { STE100P_PHY_ID, 0xffffffff },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, ste10Xp_tbl);
    MODULE_DESCRIPTION("STMicroelectronics STe10Xp PHY driver");
    MODULE_AUTHOR("Giuseppe Cavallaro <peppe.cavallaro@st.com>");
    MODULE_LICENSE("GPL");
