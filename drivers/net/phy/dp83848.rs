//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/dp83848.c
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
// Driver for the Texas Instruments DP83848 PHY
//
// Copyright (C) 2015-2016 Texas Instruments Incorporated - http://www.ti.com
//

pub const TI_DP83848C_PHY_ID: c_uint = 0x20005ca0;
pub const TI_DP83620_PHY_ID: c_uint = 0x20005ce0;
pub const NS_DP83848C_PHY_ID: c_uint = 0x20005c90;
pub const TLK10X_PHY_ID: c_uint = 0x2000a210;
// Registers
pub const DP83848_MICR: c_uint = 0x11 /* MII Interrupt Control Register */;
pub const DP83848_MISR: c_uint = 0x12 /* MII Interrupt Status Register */;
// MICR Register Fields

// MISR Register Fields

    (DP83848_MISR_ANC_INT_EN |	\
    DP83848_MISR_DUP_INT_EN |	\
    DP83848_MISR_SPD_INT_EN |	\
    DP83848_MISR_LINK_INT_EN)

    (DP83848_MISR_ANC_INT |	\
    DP83848_MISR_DUP_INT |	\
    DP83848_MISR_SPD_INT |	\
    DP83848_MISR_LINK_INT)
#[no_mangle]
unsafe extern "C" fn dp83848_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int dp83848_ack_interrupt(struct phy_device *phydev)
    {
    let mut err: c_int = phy_read(phydev, DP83848_MISR);
    return err < 0 ? err : 0;
    }
#[no_mangle]
unsafe extern "C" fn dp83848_config_intr(phydev: *mut phy_device) -> c_int {
    static int dp83848_config_intr(struct phy_device *phydev)
    {
    int control, ret;
    control = phy_read(phydev, DP83848_MICR);
    if (control < 0)
    return control;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    ret = dp83848_ack_interrupt(phydev);
    if (ret)
    return ret;
    control |= DP83848_MICR_INT_OE;
    control |= DP83848_MICR_INTEN;
    ret = phy_write(phydev, DP83848_MISR, DP83848_INT_EN_MASK);
    if (ret < 0)
    return ret;
    ret = phy_write(phydev, DP83848_MICR, control);
    } else {
    control &= ~DP83848_MICR_INTEN;
    ret = phy_write(phydev, DP83848_MICR, control);
    if (ret)
    return ret;
    ret = dp83848_ack_interrupt(phydev);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dp83848_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t dp83848_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read(phydev, DP83848_MISR);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & DP83848_INT_MASK))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dp83848_config_init(phydev: *mut phy_device) -> c_int {
    static int dp83848_config_init(struct phy_device *phydev)
    {
    int val;
// DP83620 always reports Auto Negotiation Ability on BMSR. Instead,
// we check initial value of BMCR Auto negotiation enable bit
//
    val = phy_read(phydev, MII_BMCR);
    if (!(val & BMCR_ANENABLE))
    phydev.autoneg = AUTONEG_DISABLE;
    return 0;
    }
    static const struct mdio_device_id __maybe_unused dp83848_tbl[] = {
    { TI_DP83848C_PHY_ID, 0xfffffff0 },
    { NS_DP83848C_PHY_ID, 0xfffffff0 },
    { TI_DP83620_PHY_ID, 0xfffffff0 },
    { TLK10X_PHY_ID, 0xfffffff0 },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, dp83848_tbl);

    {							\
    .phy_id		= _id,				\
    .phy_id_mask	= 0xfffffff0,			\
    .name		= _name,			\
// PHY_BASIC_FEATURES */			\
    \
    .soft_reset	= genphy_soft_reset,		\
    .config_init	= _config_init,			\
    .suspend	= genphy_suspend,		\
    .resume		= genphy_resume,		\
    \
// IRQ related */				\
    .config_intr	= dp83848_config_intr,		\
    .handle_interrupt = dp83848_handle_interrupt,	\
    \
    .flags		= PHY_RST_AFTER_CLK_EN,		\
    }
    static struct phy_driver dp83848_driver[] = {
    DP83848_PHY_DRIVER(TI_DP83848C_PHY_ID, "TI DP83848C 10/100 Mbps PHY",
    core::ptr::null_mut()),
    DP83848_PHY_DRIVER(NS_DP83848C_PHY_ID, "NS DP83848C 10/100 Mbps PHY",
    core::ptr::null_mut()),
    DP83848_PHY_DRIVER(TI_DP83620_PHY_ID, "TI DP83620 10/100 Mbps PHY",
    dp83848_config_init),
    DP83848_PHY_DRIVER(TLK10X_PHY_ID, "TI TLK10X 10/100 Mbps PHY",
    core::ptr::null_mut()),
    };
    module_phy_driver(dp83848_driver);
    MODULE_DESCRIPTION("Texas Instruments DP83848 PHY driver");
    MODULE_AUTHOR("Andrew F. Davis <afd@ti.com>");
    MODULE_LICENSE("GPL v2");
