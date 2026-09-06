//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/amd.c
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
// Driver for AMD am79c PHYs
//
// Author: Heiko Schocher <hs@denx.de>
//
// Copyright (c) 2011 DENX Software Engineering GmbH
//

pub const PHY_ID_AC101L: c_uint = 0x00225520;
pub const PHY_ID_AM79C874: c_uint = 0x0022561b;

pub const MII_AM79C_IR_EN_LINK: c_uint = 0x0400	/* IR enable Linkstate */;
pub const MII_AM79C_IR_EN_ANEG: c_uint = 0x0100	/* IR enable Aneg Complete */;

    MODULE_DESCRIPTION("AMD PHY driver");
    MODULE_AUTHOR("Heiko Schocher <hs@denx.de>");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn am79c_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int am79c_ack_interrupt(struct phy_device *phydev)
    {
    int err;
    err = phy_read(phydev, MII_BMSR);
    if (err < 0)
    return err;
    err = phy_read(phydev, MII_AM79C_IR);
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn am79c_config_init(phydev: *mut phy_device) -> c_int {
    static int am79c_config_init(struct phy_device *phydev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn am79c_config_intr(phydev: *mut phy_device) -> c_int {
    static int am79c_config_intr(struct phy_device *phydev)
    {
    int err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = am79c_ack_interrupt(phydev);
    if (err)
    return err;
    err = phy_write(phydev, MII_AM79C_IR, MII_AM79C_IR_IMASK_INIT);
    } else {
    err = phy_write(phydev, MII_AM79C_IR, 0);
    if (err)
    return err;
    err = am79c_ack_interrupt(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn am79c_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t am79c_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read(phydev, MII_AM79C_IR);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & MII_AM79C_IR_IMASK_STAT))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
    static struct phy_driver am79c_drivers[] = {
    {
    .phy_id		= PHY_ID_AM79C874,
    .name		= "AM79C874",
    .phy_id_mask	= 0xfffffff0,
// PHY_BASIC_FEATURES
    .config_init	= am79c_config_init,
    .config_intr	= am79c_config_intr,
    .handle_interrupt = am79c_handle_interrupt,
    },
    {
    .phy_id		= PHY_ID_AC101L,
    .name		= "AC101L",
    .phy_id_mask	= 0xfffffff0,
// PHY_BASIC_FEATURES
    .config_init	= am79c_config_init,
    .config_intr	= am79c_config_intr,
    .handle_interrupt = am79c_handle_interrupt,
    },
    };
    module_phy_driver(am79c_drivers);
    static const struct mdio_device_id __maybe_unused amd_tbl[] = {
    { PHY_ID_AC101L, 0xfffffff0 },
    { PHY_ID_AM79C874, 0xfffffff0 },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, amd_tbl);
