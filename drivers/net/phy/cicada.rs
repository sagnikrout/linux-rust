//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/cicada.c
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
// drivers/net/phy/cicada.c
//
// Driver for Cicada PHYs
//
// Author: Andy Fleming
//
// Copyright (c) 2004 Freescale Semiconductor, Inc.
//

// Cicada Extended Control Register 1
pub const MII_CIS8201_EXT_CON1: c_uint = 0x17;
pub const MII_CIS8201_EXTCON1_INIT: c_uint = 0x0000;
// Cicada Interrupt Mask Register
pub const MII_CIS8201_IMASK: c_uint = 0x19;
pub const MII_CIS8201_IMASK_IEN: c_uint = 0x8000;
pub const MII_CIS8201_IMASK_SPEED: c_uint = 0x4000;
pub const MII_CIS8201_IMASK_LINK: c_uint = 0x2000;
pub const MII_CIS8201_IMASK_DUPLEX: c_uint = 0x1000;
pub const MII_CIS8201_IMASK_MASK: c_uint = 0xf000;
// Cicada Interrupt Status Register
pub const MII_CIS8201_ISTAT: c_uint = 0x1a;
pub const MII_CIS8201_ISTAT_STATUS: c_uint = 0x8000;
pub const MII_CIS8201_ISTAT_SPEED: c_uint = 0x4000;
pub const MII_CIS8201_ISTAT_LINK: c_uint = 0x2000;
pub const MII_CIS8201_ISTAT_DUPLEX: c_uint = 0x1000;
// Cicada Auxiliary Control/Status Register
pub const MII_CIS8201_AUX_CONSTAT: c_uint = 0x1c;
pub const MII_CIS8201_AUXCONSTAT_INIT: c_uint = 0x0004;
pub const MII_CIS8201_AUXCONSTAT_DUPLEX: c_uint = 0x0020;
pub const MII_CIS8201_AUXCONSTAT_SPEED: c_uint = 0x0018;
pub const MII_CIS8201_AUXCONSTAT_GBIT: c_uint = 0x0010;
pub const MII_CIS8201_AUXCONSTAT_100: c_uint = 0x0008;
    MODULE_DESCRIPTION("Cicadia PHY driver");
    MODULE_AUTHOR("Andy Fleming");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn cis820x_config_init(phydev: *mut phy_device) -> c_int {
    static int cis820x_config_init(struct phy_device *phydev)
    {
    int err;
    err = phy_write(phydev, MII_CIS8201_AUX_CONSTAT,
    MII_CIS8201_AUXCONSTAT_INIT);
    if (err < 0)
    return err;
    err = phy_write(phydev, MII_CIS8201_EXT_CON1,
    MII_CIS8201_EXTCON1_INIT);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cis820x_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int cis820x_ack_interrupt(struct phy_device *phydev)
    {
    let mut err: c_int = phy_read(phydev, MII_CIS8201_ISTAT);
    return (err < 0) ? err : 0;
    }
#[no_mangle]
unsafe extern "C" fn cis820x_config_intr(phydev: *mut phy_device) -> c_int {
    static int cis820x_config_intr(struct phy_device *phydev)
    {
    int err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = cis820x_ack_interrupt(phydev);
    if (err)
    return err;
    err = phy_write(phydev, MII_CIS8201_IMASK,
    MII_CIS8201_IMASK_MASK);
    } else {
    err = phy_write(phydev, MII_CIS8201_IMASK, 0);
    if (err)
    return err;
    err = cis820x_ack_interrupt(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cis820x_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t cis820x_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read(phydev, MII_CIS8201_ISTAT);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & MII_CIS8201_IMASK_MASK))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
// Cicada 8201, a.k.a Vitesse VSC8201
    static struct phy_driver cis820x_driver[] = {
    {
    .phy_id		= 0x000fc410,
    .name		= "Cicada Cis8201",
    .phy_id_mask	= 0x000ffff0,
// PHY_GBIT_FEATURES
    .config_init	= &cis820x_config_init,
    .config_intr	= &cis820x_config_intr,
    .handle_interrupt = &cis820x_handle_interrupt,
    }, {
    .phy_id		= 0x000fc440,
    .name		= "Cicada Cis8204",
    .phy_id_mask	= 0x000fffc0,
// PHY_GBIT_FEATURES
    .config_init	= &cis820x_config_init,
    .config_intr	= &cis820x_config_intr,
    .handle_interrupt = &cis820x_handle_interrupt,
    } };
    module_phy_driver(cis820x_driver);
    static const struct mdio_device_id __maybe_unused cicada_tbl[] = {
    { 0x000fc410, 0x000ffff0 },
    { 0x000fc440, 0x000fffc0 },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, cicada_tbl);
