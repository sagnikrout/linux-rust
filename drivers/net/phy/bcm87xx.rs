//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/bcm87xx.c
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
// Copyright (C) 2011 - 2012 Cavium, Inc.
//

pub const PHY_ID_BCM8706: c_uint = 0x0143bdc1;
pub const PHY_ID_BCM8727: c_uint = 0x0143bff0;
pub const BCM87XX_PMD_RX_SIGNAL_DETECT: c_uint = 0x000a;
pub const BCM87XX_10GBASER_PCS_STATUS: c_uint = 0x0020;
pub const BCM87XX_XGXS_LANE_STATUS: c_uint = 0x0018;
pub const BCM87XX_LASI_CONTROL: c_uint = 0x9002;
pub const BCM87XX_LASI_STATUS: c_uint = 0x9005;

// Set and/or override some configuration registers based on the
// broadcom,c45-reg-init property stored in the of_node for the phydev.
//
// broadcom,c45-reg-init = <devid reg mask value>,...;
//
// There may be one or more sets of <devid reg mask value>:
//
// devid: which sub-device to use.
// reg: the register.
// mask: if non-zero, ANDed with existing register value.
// value: ORed with the masked value and written to the regiser.
//
#[no_mangle]
unsafe extern "C" fn bcm87xx_of_reg_init(phydev: *mut phy_device) -> c_int {
    static int bcm87xx_of_reg_init(struct phy_device *phydev)
    {
    const __be32 *paddr;
    const __be32 *paddr_end;
    int len, ret;
    if (!phydev.mdio.dev.of_node)
    return 0;
    paddr = of_get_property(phydev.mdio.dev.of_node,
    "broadcom,c45-reg-init", &len);
    if (!paddr)
    return 0;
    paddr_end = paddr + (len /= sizeof(*paddr));
    ret = 0;
    while (paddr + 3 < paddr_end) {
    let mut devid: u16 = be32_to_cpup(paddr++);
    let mut reg: u16 = be32_to_cpup(paddr++);
    let mut mask: u16 = be32_to_cpup(paddr++);
    let mut val_bits: u16 = be32_to_cpup(paddr++);
    let mut val: c_int = 0;
    if (mask) {
    val = phy_read_mmd(phydev, devid, reg);
    if (val < 0) {
    ret = val;
    goto err;
    }
    val &= mask;
    }
    val |= val_bits;
    ret = phy_write_mmd(phydev, devid, reg, val);
    if (ret < 0)
    goto err;
    }
    err:
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn bcm87xx_of_reg_init(phydev: *mut phy_device) -> c_int {
    static int bcm87xx_of_reg_init(struct phy_device *phydev)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn bcm87xx_get_features(phydev: *mut phy_device) -> c_int {
    static int bcm87xx_get_features(struct phy_device *phydev)
    {
    linkmode_set_bit(ETHTOOL_LINK_MODE_10000baseR_FEC_BIT,
    phydev.supported);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm87xx_config_init(phydev: *mut phy_device) -> c_int {
    static int bcm87xx_config_init(struct phy_device *phydev)
    {
    return bcm87xx_of_reg_init(phydev);
    }
#[no_mangle]
unsafe extern "C" fn bcm87xx_config_aneg(phydev: *mut phy_device) -> c_int {
    static int bcm87xx_config_aneg(struct phy_device *phydev)
    {
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn bcm87xx_read_status(phydev: *mut phy_device) -> c_int {
    static int bcm87xx_read_status(struct phy_device *phydev)
    {
    int rx_signal_detect;
    int pcs_status;
    int xgxs_lane_status;
    rx_signal_detect = phy_read_mmd(phydev, MDIO_MMD_PMAPMD,
    BCM87XX_PMD_RX_SIGNAL_DETECT);
    if (rx_signal_detect < 0)
    return rx_signal_detect;
    if ((rx_signal_detect & 1) == 0)
    goto no_link;
    pcs_status = phy_read_mmd(phydev, MDIO_MMD_PCS,
    BCM87XX_10GBASER_PCS_STATUS);
    if (pcs_status < 0)
    return pcs_status;
    if ((pcs_status & 1) == 0)
    goto no_link;
    xgxs_lane_status = phy_read_mmd(phydev, MDIO_MMD_PHYXS,
    BCM87XX_XGXS_LANE_STATUS);
    if (xgxs_lane_status < 0)
    return xgxs_lane_status;
    if ((xgxs_lane_status & 0x1000) == 0)
    goto no_link;
    phydev.speed = 10000;
    phydev.link = 1;
    phydev.duplex = 1;
    return 0;
    no_link:
    phydev.link = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm87xx_config_intr(phydev: *mut phy_device) -> c_int {
    static int bcm87xx_config_intr(struct phy_device *phydev)
    {
    int reg, err;
    reg = phy_read_mmd(phydev, MDIO_MMD_PCS, BCM87XX_LASI_CONTROL);
    if (reg < 0)
    return reg;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = phy_read_mmd(phydev, MDIO_MMD_PCS, BCM87XX_LASI_STATUS);
    if (err)
    return err;
    reg |= 1;
    err = phy_write_mmd(phydev, MDIO_MMD_PCS,
    BCM87XX_LASI_CONTROL, reg);
    } else {
    reg &= ~1;
    err = phy_write_mmd(phydev, MDIO_MMD_PCS,
    BCM87XX_LASI_CONTROL, reg);
    if (err)
    return err;
    err = phy_read_mmd(phydev, MDIO_MMD_PCS, BCM87XX_LASI_STATUS);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bcm87xx_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t bcm87xx_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read(phydev, BCM87XX_LASI_STATUS);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (irq_status == 0)
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
    static int bcm87xx_match_phy_device(struct phy_device *phydev,
    const struct phy_driver *phydrv)
    {
    return phydev.c45_ids.device_ids[4] == phydrv.phy_id;
    }
    static struct phy_driver bcm87xx_driver[] = {
    {
    .phy_id		= PHY_ID_BCM8706,
    .phy_id_mask	= 0xffffffff,
    .name		= "Broadcom BCM8706",
    .get_features	= bcm87xx_get_features,
    .config_init	= bcm87xx_config_init,
    .config_aneg	= bcm87xx_config_aneg,
    .read_status	= bcm87xx_read_status,
    .config_intr	= bcm87xx_config_intr,
    .handle_interrupt = bcm87xx_handle_interrupt,
    .match_phy_device = bcm87xx_match_phy_device,
    }, {
    .phy_id		= PHY_ID_BCM8727,
    .phy_id_mask	= 0xffffffff,
    .name		= "Broadcom BCM8727",
    .get_features	= bcm87xx_get_features,
    .config_init	= bcm87xx_config_init,
    .config_aneg	= bcm87xx_config_aneg,
    .read_status	= bcm87xx_read_status,
    .config_intr	= bcm87xx_config_intr,
    .handle_interrupt = bcm87xx_handle_interrupt,
    .match_phy_device = bcm87xx_match_phy_device,
    } };
    module_phy_driver(bcm87xx_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Broadcom BCM87xx PHY driver");
