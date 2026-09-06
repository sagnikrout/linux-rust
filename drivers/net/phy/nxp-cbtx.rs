//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/nxp-cbtx.c
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
// Driver for 100BASE-TX PHY embedded into NXP SJA1110 switch
//
// Copyright 2022-2023 NXP
//

pub const PHY_ID_CBTX_SJA1110: c_uint = 0x001bb020;
// Registers
pub const CBTX_MODE_CTRL_STAT: c_uint = 0x11;
pub const CBTX_PDOWN_CTRL: c_uint = 0x18;
pub const CBTX_RX_ERR_COUNTER: c_uint = 0x1a;
pub const CBTX_IRQ_STAT: c_uint = 0x1d;
pub const CBTX_IRQ_ENABLE: c_uint = 0x1e;
// Fields

#[no_mangle]
unsafe extern "C" fn cbtx_soft_reset(phydev: *mut phy_device) -> c_int {
    static int cbtx_soft_reset(struct phy_device *phydev)
    {
    int ret;
// Can't soft reset unless we remove PHY from true power down mode
    ret = phy_clear_bits(phydev, CBTX_PDOWN_CTRL,
    CBTX_PDOWN_CTL_TRUE_PDOWN);
    if (ret)
    return ret;
    return genphy_soft_reset(phydev);
    }
#[no_mangle]
unsafe extern "C" fn cbtx_config_init(phydev: *mut phy_device) -> c_int {
    static int cbtx_config_init(struct phy_device *phydev)
    {
// Wait for cbtx_config_aneg() to kick in and apply this
    phydev.mdix_ctrl = ETH_TP_MDI_AUTO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cbtx_mdix_status(phydev: *mut phy_device) -> c_int {
    static int cbtx_mdix_status(struct phy_device *phydev)
    {
    int ret;
    ret = phy_read(phydev, CBTX_MODE_CTRL_STAT);
    if (ret < 0)
    return ret;
    if (ret & CBTX_MODE_CTRL_STAT_MDIX_MODE)
    phydev.mdix = ETH_TP_MDI_X;
    else
    phydev.mdix = ETH_TP_MDI;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cbtx_read_status(phydev: *mut phy_device) -> c_int {
    static int cbtx_read_status(struct phy_device *phydev)
    {
    int ret;
    ret = cbtx_mdix_status(phydev);
    if (ret)
    return ret;
    return genphy_read_status(phydev);
    }
#[no_mangle]
unsafe extern "C" fn cbtx_mdix_config(phydev: *mut phy_device) -> c_int {
    static int cbtx_mdix_config(struct phy_device *phydev)
    {
    int ret;
    switch (phydev.mdix_ctrl) {
    case ETH_TP_MDI_AUTO:
    return phy_set_bits(phydev, CBTX_MODE_CTRL_STAT,
    CBTX_MODE_CTRL_STAT_AUTO_MDIX_EN);
    case ETH_TP_MDI:
    ret = phy_clear_bits(phydev, CBTX_MODE_CTRL_STAT,
    CBTX_MODE_CTRL_STAT_AUTO_MDIX_EN);
    if (ret)
    return ret;
    return phy_clear_bits(phydev, CBTX_MODE_CTRL_STAT,
    CBTX_MODE_CTRL_STAT_MDIX_MODE);
    case ETH_TP_MDI_X:
    ret = phy_clear_bits(phydev, CBTX_MODE_CTRL_STAT,
    CBTX_MODE_CTRL_STAT_AUTO_MDIX_EN);
    if (ret)
    return ret;
    return phy_set_bits(phydev, CBTX_MODE_CTRL_STAT,
    CBTX_MODE_CTRL_STAT_MDIX_MODE);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cbtx_config_aneg(phydev: *mut phy_device) -> c_int {
    static int cbtx_config_aneg(struct phy_device *phydev)
    {
    int ret;
    ret = cbtx_mdix_config(phydev);
    if (ret)
    return ret;
    return genphy_config_aneg(phydev);
    }
#[no_mangle]
unsafe extern "C" fn cbtx_ack_interrupts(phydev: *mut phy_device) -> c_int {
    static int cbtx_ack_interrupts(struct phy_device *phydev)
    {
    return phy_read(phydev, CBTX_IRQ_STAT);
    }
#[no_mangle]
unsafe extern "C" fn cbtx_config_intr(phydev: *mut phy_device) -> c_int {
    static int cbtx_config_intr(struct phy_device *phydev)
    {
    int ret;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    ret = cbtx_ack_interrupts(phydev);
    if (ret < 0)
    return ret;
    ret = phy_write(phydev, CBTX_IRQ_ENABLE, CBTX_IRQ_LINK_DOWN |
    CBTX_IRQ_AN_COMPLETE | CBTX_IRQ_ENERGYON);
    if (ret)
    return ret;
    } else {
    ret = phy_write(phydev, CBTX_IRQ_ENABLE, 0);
    if (ret)
    return ret;
    ret = cbtx_ack_interrupts(phydev);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cbtx_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t cbtx_handle_interrupt(struct phy_device *phydev)
    {
    int irq_stat, irq_enabled;
    irq_stat = cbtx_ack_interrupts(phydev);
    if (irq_stat < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    irq_enabled = phy_read(phydev, CBTX_IRQ_ENABLE);
    if (irq_enabled < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_enabled & irq_stat))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cbtx_get_sset_count(phydev: *mut phy_device) -> c_int {
    static int cbtx_get_sset_count(struct phy_device *phydev)
    {
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn cbtx_get_strings(phydev: *mut phy_device, data: *mut u8) {
    static void cbtx_get_strings(struct phy_device *phydev, u8 *data)
    {
    ethtool_puts(&data, "100btx_rx_err");
    }
    static void cbtx_get_stats(struct phy_device *phydev,
    struct ethtool_stats *stats, u64 *data)
    {
    int ret;
    ret = phy_read(phydev, CBTX_RX_ERR_COUNTER);
    data[0] = (ret < 0) ? U64_MAX : ret;
    }
    static struct phy_driver cbtx_driver[] = {
    {
    PHY_ID_MATCH_MODEL(PHY_ID_CBTX_SJA1110),
    .name			= "NXP CBTX (SJA1110)",
// PHY_BASIC_FEATURES
    .soft_reset		= cbtx_soft_reset,
    .config_init		= cbtx_config_init,
    .suspend		= genphy_suspend,
    .resume			= genphy_resume,
    .config_intr		= cbtx_config_intr,
    .handle_interrupt	= cbtx_handle_interrupt,
    .read_status		= cbtx_read_status,
    .config_aneg		= cbtx_config_aneg,
    .get_sset_count		= cbtx_get_sset_count,
    .get_strings		= cbtx_get_strings,
    .get_stats		= cbtx_get_stats,
    },
    };
    module_phy_driver(cbtx_driver);
    static const struct mdio_device_id __maybe_unused cbtx_tbl[] = {
    { PHY_ID_MATCH_MODEL(PHY_ID_CBTX_SJA1110) },
    { },
    };
    MODULE_DEVICE_TABLE(mdio, cbtx_tbl);
    MODULE_AUTHOR("Vladimir Oltean <vladimir.oltean@nxp.com>");
    MODULE_DESCRIPTION("NXP CBTX PHY driver");
    MODULE_LICENSE("GPL");
