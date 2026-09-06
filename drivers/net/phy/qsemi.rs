//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/qsemi.c
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
// drivers/net/phy/qsemi.c
//
// Driver for Quality Semiconductor PHYs
//
// Author: Andy Fleming
//
// Copyright (c) 2004 Freescale Semiconductor, Inc.
//

// -------------------------------------------------------------------------
// The Quality Semiconductor QS6612 is used on the RPX CLLF
// register definitions

pub const MII_QS6612_IMR_INIT: c_uint = 0x003a;

pub const QS6612_PCR_AN_COMPLETE: c_uint = 0x1000;
pub const QS6612_PCR_RLBEN: c_uint = 0x0200;
pub const QS6612_PCR_DCREN: c_uint = 0x0100;
pub const QS6612_PCR_4B5BEN: c_uint = 0x0040;
pub const QS6612_PCR_TX_ISOLATE: c_uint = 0x0020;
pub const QS6612_PCR_MLT3_DIS: c_uint = 0x0002;
pub const QS6612_PCR_SCRM_DESCRM: c_uint = 0x0001;
    MODULE_DESCRIPTION("Quality Semiconductor PHY driver");
    MODULE_AUTHOR("Andy Fleming");
    MODULE_LICENSE("GPL");
// Returns 0, unless there's a write error
#[no_mangle]
unsafe extern "C" fn qs6612_config_init(phydev: *mut phy_device) -> c_int {
    static int qs6612_config_init(struct phy_device *phydev)
    {
// The PHY powers up isolated on the RPX,
// so send a command to allow operation.
// XXX - My docs indicate this should be 0x0940
// ...or something.  The current value sets three
// reserved bits, bit 11, which specifies it should be
// set to one, bit 10, which specifies it should be set
// to 0, and bit 7, which doesn't specify.  However, my
// docs are preliminary, and I will leave it like this
// until someone more knowledgable corrects me or it.
// -- Andy Fleming
//
    return phy_write(phydev, MII_QS6612_PCR, 0x0dc0);
    }
#[no_mangle]
unsafe extern "C" fn qs6612_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int qs6612_ack_interrupt(struct phy_device *phydev)
    {
    int err;
// The Interrupt Source register is not self-clearing, bits 4 and 5 are
// cleared when MII_BMSR is read and bits 1 and 3 are cleared when
// MII_EXPANSION is read
//
    err = phy_read(phydev, MII_QS6612_ISR);
    if (err < 0)
    return err;
    err = phy_read(phydev, MII_BMSR);
    if (err < 0)
    return err;
    err = phy_read(phydev, MII_EXPANSION);
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qs6612_config_intr(phydev: *mut phy_device) -> c_int {
    static int qs6612_config_intr(struct phy_device *phydev)
    {
    int err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
// clear any interrupts before enabling them
    err = qs6612_ack_interrupt(phydev);
    if (err)
    return err;
    err = phy_write(phydev, MII_QS6612_IMR,
    MII_QS6612_IMR_INIT);
    } else {
    err = phy_write(phydev, MII_QS6612_IMR, 0);
    if (err)
    return err;
// clear any leftover interrupts
    err = qs6612_ack_interrupt(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn qs6612_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t qs6612_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read(phydev, MII_QS6612_ISR);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & MII_QS6612_IMR_INIT))
    return IRQ_NONE;
// the interrupt source register is not self-clearing
    qs6612_ack_interrupt(phydev);
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
    static struct phy_driver qs6612_driver[] = { {
    .phy_id		= 0x00181440,
    .name		= "QS6612",
    .phy_id_mask	= 0xfffffff0,
// PHY_BASIC_FEATURES
    .config_init	= qs6612_config_init,
    .config_intr	= qs6612_config_intr,
    .handle_interrupt = qs6612_handle_interrupt,
    } };
    module_phy_driver(qs6612_driver);
    static const struct mdio_device_id __maybe_unused qs6612_tbl[] = {
    { 0x00181440, 0xfffffff0 },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, qs6612_tbl);
