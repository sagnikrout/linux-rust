//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/national.c
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
// drivers/net/phy/national.c
//
// Driver for National Semiconductor PHYs
//
// Author: Stuart Menefy <stuart.menefy@st.com>
// Maintainer: Giuseppe Cavallaro <peppe.cavallaro@st.com>
//
// Copyright (c) 2008 STMicroelectronics Limited
//

// DP83865 phy identifier values
pub const DP83865_PHY_ID: c_uint = 0x20005c7a;
pub const DP83865_INT_STATUS: c_uint = 0x14;
pub const DP83865_INT_MASK: c_uint = 0x15;
pub const DP83865_INT_CLEAR: c_uint = 0x17;
pub const DP83865_INT_REMOTE_FAULT: c_uint = 0x0008;
pub const DP83865_INT_ANE_COMPLETED: c_uint = 0x0010;
pub const DP83865_INT_LINK_CHANGE: c_uint = 0xe000;

    DP83865_INT_ANE_COMPLETED | \
    DP83865_INT_LINK_CHANGE)
// Advanced proprietary configuration
pub const NS_EXP_MEM_CTL: c_uint = 0x16;
pub const NS_EXP_MEM_DATA: c_uint = 0x1d;
pub const NS_EXP_MEM_ADD: c_uint = 0x1e;
pub const LED_CTRL_REG: c_uint = 0x13;
pub const AN_FALLBACK_AN: c_uint = 0x0001;
pub const AN_FALLBACK_CRC: c_uint = 0x0002;
pub const AN_FALLBACK_IE: c_uint = 0x0004;

    enum hdx_loopback {
    hdx_loopback_on = 0,
    hdx_loopback_off = 1,
    };
#[no_mangle]
unsafe extern "C" fn ns_exp_read(phydev: *mut phy_device, reg: u16) -> u8 {
    static u8 ns_exp_read(struct phy_device *phydev, u16 reg)
    {
    phy_write(phydev, NS_EXP_MEM_ADD, reg);
    return phy_read(phydev, NS_EXP_MEM_DATA);
    }
#[no_mangle]
unsafe extern "C" fn ns_exp_write(phydev: *mut phy_device, reg: u16, data: u8) {
    static void ns_exp_write(struct phy_device *phydev, u16 reg, u8 data)
    {
    phy_write(phydev, NS_EXP_MEM_ADD, reg);
    phy_write(phydev, NS_EXP_MEM_DATA, data);
    }
#[no_mangle]
unsafe extern "C" fn ns_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int ns_ack_interrupt(struct phy_device *phydev)
    {
    let mut ret: c_int = phy_read(phydev, DP83865_INT_STATUS);
    if (ret < 0)
    return ret;
// Clear the interrupt status bit by writing a “1”
// to the corresponding bit in INT_CLEAR (2:0 are reserved)
//
    ret = phy_write(phydev, DP83865_INT_CLEAR, ret & ~0x7);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ns_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t ns_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read(phydev, DP83865_INT_STATUS);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & DP83865_INT_MASK_DEFAULT))
    return IRQ_NONE;
// clear the interrupt
    phy_write(phydev, DP83865_INT_CLEAR, irq_status & ~0x7);
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ns_config_intr(phydev: *mut phy_device) -> c_int {
    static int ns_config_intr(struct phy_device *phydev)
    {
    int err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = ns_ack_interrupt(phydev);
    if (err)
    return err;
    err = phy_write(phydev, DP83865_INT_MASK,
    DP83865_INT_MASK_DEFAULT);
    } else {
    err = phy_write(phydev, DP83865_INT_MASK, 0);
    if (err)
    return err;
    err = ns_ack_interrupt(phydev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ns_giga_speed_fallback(phydev: *mut phy_device, mode: c_int) {
    static void ns_giga_speed_fallback(struct phy_device *phydev, int mode)
    {
    let mut bmcr: c_int = phy_read(phydev, MII_BMCR);
    phy_write(phydev, MII_BMCR, (bmcr | BMCR_PDOWN));
// Enable 8 bit expended memory read/write (no auto increment)
    phy_write(phydev, NS_EXP_MEM_CTL, 0);
    phy_write(phydev, NS_EXP_MEM_ADD, 0x1C0);
    phy_write(phydev, NS_EXP_MEM_DATA, 0x0008);
    phy_write(phydev, MII_BMCR, (bmcr & ~BMCR_PDOWN));
    phy_write(phydev, LED_CTRL_REG, mode);
    }
#[no_mangle]
unsafe extern "C" fn ns_10_base_t_hdx_loopack(phydev: *mut phy_device, disable: c_int) {
    static void ns_10_base_t_hdx_loopack(struct phy_device *phydev, int disable)
    {
    let mut lb_dis: u16 = BIT(1);
    if (disable)
    ns_exp_write(phydev, 0x1c0,
    ns_exp_read(phydev, 0x1c0) | lb_dis);
    else
    ns_exp_write(phydev, 0x1c0,
    ns_exp_read(phydev, 0x1c0) & ~lb_dis);
    pr_debug("10BASE-T HDX loopback %s\n",
    (ns_exp_read(phydev, 0x1c0) & lb_dis) ? "off" : "on");
    }
#[no_mangle]
unsafe extern "C" fn ns_config_init(phydev: *mut phy_device) -> c_int {
    static int ns_config_init(struct phy_device *phydev)
    {
    ns_giga_speed_fallback(phydev, ALL_FALLBACK_ON);
// In the latest MAC or switches design, the 10 Mbps loopback
// is desired to be turned off.
//
    ns_10_base_t_hdx_loopack(phydev, hdx_loopback_off);
    return ns_ack_interrupt(phydev);
    }
    static struct phy_driver dp83865_driver[] = { {
    .phy_id = DP83865_PHY_ID,
    .phy_id_mask = 0xfffffff0,
    .name = "NatSemi DP83865",
// PHY_GBIT_FEATURES
    .config_init = ns_config_init,
    .config_intr = ns_config_intr,
    .handle_interrupt = ns_handle_interrupt,
    } };
    module_phy_driver(dp83865_driver);
    MODULE_DESCRIPTION("NatSemi PHY driver");
    MODULE_AUTHOR("Stuart Menefy");
    MODULE_LICENSE("GPL");
    static const struct mdio_device_id __maybe_unused ns_tbl[] = {
    { DP83865_PHY_ID, 0xfffffff0 },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, ns_tbl);
