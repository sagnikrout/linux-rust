//! Automatically rewritten from C to Rust
//! Source: drivers/phy/marvell/phy-mvebu-cp110-utmi.c
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
// Copyright (C) 2021 Marvell
//
// Authors:
// Konstantin Porotchkin <kostap@marvell.com>
//
// Marvell CP110 UTMI PHY driver
//

pub const UTMI_PHY_PORTS: c_int = 2;
// CP110 UTMI register macro definetions
pub const SYSCON_USB_CFG_REG: c_uint = 0x420;

pub const USB_CFG_DEVICE_MUX_OFFSET: c_int = 1;

pub const UTMI_PLL_CTRL_REG: c_uint = 0x0;
pub const PLL_REFDIV_OFFSET: c_int = 0;

pub const PLL_REFDIV_VAL: c_uint = 0x5;
pub const PLL_FBDIV_OFFSET: c_int = 16;

pub const PLL_FBDIV_VAL: c_uint = 0x60;

pub const UTMI_CAL_CTRL_REG: c_uint = 0x8;
pub const IMPCAL_VTH_OFFSET: c_int = 8;

pub const IMPCAL_VTH_VAL: c_uint = 0x7;

pub const UTMI_TX_CH_CTRL_REG: c_uint = 0xC;
pub const DRV_EN_LS_OFFSET: c_int = 12;

pub const IMP_SEL_LS_OFFSET: c_int = 16;

pub const TX_AMP_OFFSET: c_int = 20;

pub const TX_AMP_VAL: c_uint = 0x4;
pub const UTMI_RX_CH_CTRL0_REG: c_uint = 0x14;

pub const UTMI_RX_CH_CTRL1_REG: c_uint = 0x18;
pub const SQ_AMP_CAL_OFFSET: c_int = 0;

pub const SQ_AMP_CAL_VAL: c_int = 1;

pub const UTMI_DIG_CTRL1_REG: c_uint = 0x20;

pub const UTMI_CTRL_STATUS0_REG: c_uint = 0x24;

pub const UTMI_CHGDTC_CTRL_REG: c_uint = 0x38;
pub const VDAT_OFFSET: c_int = 8;

pub const VDAT_VAL: c_int = 1;
pub const VSRC_OFFSET: c_int = 10;

pub const VSRC_VAL: c_int = 1;
pub const PLL_LOCK_DELAY_US: c_int = 10000;
pub const PLL_LOCK_TIMEOUT_US: c_int = 1000000;

//
// struct mvebu_cp110_utmi - PHY driver data
//
// @regs: PHY registers
// @syscon: Regmap with system controller registers
// @dev: device driver handle
// @ops: phy ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvebu_cp110_utmi {
    pub regs: *mut void __iomem,
    pub syscon: *mut regmap,
    pub dev: *mut device,
    pub ops: *const phy_ops,
}

//
// struct mvebu_cp110_utmi_port - PHY port data
//
// @priv: PHY driver data
// @id: PHY port ID
// @dr_mode: PHY connection: USB_DR_MODE_HOST or USB_DR_MODE_PERIPHERAL
// @swap_dx: whether to swap d+/d- signals
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvebu_cp110_utmi_port {
    pub priv: *mut mvebu_cp110_utmi,
    pub id: u32,
    pub dr_mode: enum usb_dr_mode,
    pub swap_dx: bool,
}

#[no_mangle]
unsafe extern "C" fn mvebu_cp110_utmi_port_setup(port: *mut mvebu_cp110_utmi_port) {
    static void mvebu_cp110_utmi_port_setup(struct mvebu_cp110_utmi_port *port)
    {
    u32 reg;
//
// Setup PLL.
// The reference clock is the frequency of quartz resonator
// connected to pins REFCLK_XIN and REFCLK_XOUT of the SoC.
// Register init values are matching the 40MHz default clock.
// The crystal used for all platform boards is now 25MHz.
// See the functional specification for details.
//
    reg = readl(PORT_REGS(port) + UTMI_PLL_CTRL_REG);
    reg &= ~(PLL_REFDIV_MASK | PLL_FBDIV_MASK | PLL_SEL_LPFR_MASK);
    reg |= (PLL_REFDIV_VAL << PLL_REFDIV_OFFSET) |
    (PLL_FBDIV_VAL << PLL_FBDIV_OFFSET);
    writel(reg, PORT_REGS(port) + UTMI_PLL_CTRL_REG);
// Impedance Calibration Threshold Setting
    reg = readl(PORT_REGS(port) + UTMI_CAL_CTRL_REG);
    reg &= ~IMPCAL_VTH_MASK;
    reg |= IMPCAL_VTH_VAL << IMPCAL_VTH_OFFSET;
    writel(reg, PORT_REGS(port) + UTMI_CAL_CTRL_REG);
// Set LS TX driver strength coarse control
    reg = readl(PORT_REGS(port) + UTMI_TX_CH_CTRL_REG);
    reg &= ~TX_AMP_MASK;
    reg |= TX_AMP_VAL << TX_AMP_OFFSET;
    writel(reg, PORT_REGS(port) + UTMI_TX_CH_CTRL_REG);
// Disable SQ and enable analog squelch detect
    reg = readl(PORT_REGS(port) + UTMI_RX_CH_CTRL0_REG);
    reg &= ~SQ_DET_EN;
    reg |= SQ_ANA_DTC_SEL;
    writel(reg, PORT_REGS(port) + UTMI_RX_CH_CTRL0_REG);
//
// Set External squelch calibration number and
// enable the External squelch calibration
//
    reg = readl(PORT_REGS(port) + UTMI_RX_CH_CTRL1_REG);
    reg &= ~SQ_AMP_CAL_MASK;
    reg |= (SQ_AMP_CAL_VAL << SQ_AMP_CAL_OFFSET) | SQ_AMP_CAL_EN;
    writel(reg, PORT_REGS(port) + UTMI_RX_CH_CTRL1_REG);
//
// Set Control VDAT Reference Voltage - 0.325V and
// Control VSRC Reference Voltage - 0.6V
//
    reg = readl(PORT_REGS(port) + UTMI_CHGDTC_CTRL_REG);
    reg &= ~(VDAT_MASK | VSRC_MASK);
    reg |= (VDAT_VAL << VDAT_OFFSET) | (VSRC_VAL << VSRC_OFFSET);
    writel(reg, PORT_REGS(port) + UTMI_CHGDTC_CTRL_REG);
// Swap D+/D-
    reg = readl(PORT_REGS(port) + UTMI_DIG_CTRL1_REG);
    reg &= ~(SWAP_DPDM);
    if (port.swap_dx)
    reg |= SWAP_DPDM;
    writel(reg, PORT_REGS(port) + UTMI_DIG_CTRL1_REG);
    }
#[no_mangle]
unsafe extern "C" fn mvebu_cp110_utmi_phy_power_off(phy: *mut phy) -> c_int {
    static int mvebu_cp110_utmi_phy_power_off(struct phy *phy)
    {
    struct mvebu_cp110_utmi_port *port = phy_get_drvdata(phy);
    struct mvebu_cp110_utmi *utmi = port.priv;
    int i;
// Power down UTMI PHY port
    regmap_clear_bits(utmi.syscon, SYSCON_UTMI_CFG_REG(port.id),
    UTMI_PHY_CFG_PU_MASK);
    for (i = 0; i < UTMI_PHY_PORTS; i++) {
    int test = regmap_test_bits(utmi.syscon,
    SYSCON_UTMI_CFG_REG(i),
    UTMI_PHY_CFG_PU_MASK);
// skip PLL shutdown if there are active UTMI PHY ports
    if (test != 0)
    return 0;
    }
// PLL Power down if all UTMI PHYs are down
    regmap_clear_bits(utmi.syscon, SYSCON_USB_CFG_REG, USB_CFG_PLL_MASK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mvebu_cp110_utmi_phy_power_on(phy: *mut phy) -> c_int {
    static int mvebu_cp110_utmi_phy_power_on(struct phy *phy)
    {
    struct mvebu_cp110_utmi_port *port = phy_get_drvdata(phy);
    struct mvebu_cp110_utmi *utmi = port.priv;
    struct device *dev = &phy.dev;
    int ret;
    u32 reg;
// It is necessary to power off UTMI before configuration
    ret = mvebu_cp110_utmi_phy_power_off(phy);
    if (ret) {
    dev_err(dev, "UTMI power OFF before power ON failed\n");
    return ret;
    }
//
// If UTMI port is connected to USB Device controller,
// configure the USB MUX prior to UTMI PHY initialization.
// The single USB device controller can be connected
// to UTMI0 or to UTMI1 PHY port, but not to both.
//
    if (port.dr_mode == USB_DR_MODE_PERIPHERAL) {
    regmap_update_bits(utmi.syscon, SYSCON_USB_CFG_REG,
    USB_CFG_DEVICE_EN_MASK | USB_CFG_DEVICE_MUX_MASK,
    USB_CFG_DEVICE_EN_MASK |
    (port.id << USB_CFG_DEVICE_MUX_OFFSET));
    }
// Set Test suspendm mode and enable Test UTMI select
    reg = readl(PORT_REGS(port) + UTMI_CTRL_STATUS0_REG);
    reg |= SUSPENDM | TEST_SEL;
    writel(reg, PORT_REGS(port) + UTMI_CTRL_STATUS0_REG);
// Wait for UTMI power down
    mdelay(1);
// PHY port setup first
    mvebu_cp110_utmi_port_setup(port);
// Power UP UTMI PHY
    regmap_set_bits(utmi.syscon, SYSCON_UTMI_CFG_REG(port.id),
    UTMI_PHY_CFG_PU_MASK);
// Disable Test UTMI select
    reg = readl(PORT_REGS(port) + UTMI_CTRL_STATUS0_REG);
    reg &= ~TEST_SEL;
    writel(reg, PORT_REGS(port) + UTMI_CTRL_STATUS0_REG);
// Wait for impedance calibration
    ret = readl_poll_timeout(PORT_REGS(port) + UTMI_CAL_CTRL_REG, reg,
    reg & IMPCAL_DONE,
    PLL_LOCK_DELAY_US, PLL_LOCK_TIMEOUT_US);
    if (ret) {
    dev_err(dev, "Failed to end UTMI impedance calibration\n");
    return ret;
    }
// Wait for PLL calibration
    ret = readl_poll_timeout(PORT_REGS(port) + UTMI_CAL_CTRL_REG, reg,
    reg & PLLCAL_DONE,
    PLL_LOCK_DELAY_US, PLL_LOCK_TIMEOUT_US);
    if (ret) {
    dev_err(dev, "Failed to end UTMI PLL calibration\n");
    return ret;
    }
// Wait for PLL ready
    ret = readl_poll_timeout(PORT_REGS(port) + UTMI_PLL_CTRL_REG, reg,
    reg & PLL_RDY,
    PLL_LOCK_DELAY_US, PLL_LOCK_TIMEOUT_US);
    if (ret) {
    dev_err(dev, "PLL is not ready\n");
    return ret;
    }
// PLL Power up
    regmap_set_bits(utmi.syscon, SYSCON_USB_CFG_REG, USB_CFG_PLL_MASK);
    return 0;
    }
    static const struct phy_ops mvebu_cp110_utmi_phy_ops = {
    .power_on = mvebu_cp110_utmi_phy_power_on,
    .power_off = mvebu_cp110_utmi_phy_power_off,
    .owner = THIS_MODULE,
    };
    static const struct of_device_id mvebu_cp110_utmi_of_match[] = {
    { .compatible = "marvell,cp110-utmi-phy" },
    {},
    };
    MODULE_DEVICE_TABLE(of, mvebu_cp110_utmi_of_match);
#[no_mangle]
unsafe extern "C" fn mvebu_cp110_utmi_phy_probe(pdev: *mut platform_device) -> c_int {
    static int mvebu_cp110_utmi_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mvebu_cp110_utmi *utmi;
    struct phy_provider *provider;
    struct device_node *child;
    let mut usb_devices: u32 = 0;
    let mut swap_dx: u32 = 0;
    utmi = devm_kzalloc(dev, sizeof(*utmi), GFP_KERNEL);
    if (!utmi)
    return -ENOMEM;
    utmi.dev = dev;
// Get system controller region
    utmi.syscon = syscon_regmap_lookup_by_phandle(dev.of_node,
    "marvell,system-controller");
    if (IS_ERR(utmi.syscon)) {
    dev_err(dev, "Missing UTMI system controller\n");
    return PTR_ERR(utmi.syscon);
    }
// Get UTMI memory region
    utmi.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(utmi.regs))
    return PTR_ERR(utmi.regs);
    for_each_available_child_of_node(dev.of_node, child) {
    struct mvebu_cp110_utmi_port *port;
    struct phy *phy;
    int ret;
    u32 port_id;
    ret = of_property_read_u32(child, "reg", &port_id);
    if ((ret < 0) || (port_id >= UTMI_PHY_PORTS)) {
    dev_err(dev,
    "invalid 'reg' property on child %pOF\n",
    child);
    continue;
    }
    port = devm_kzalloc(dev, sizeof(*port), GFP_KERNEL);
    if (!port) {
    of_node_put(child);
    return -ENOMEM;
    }
    port.dr_mode = of_usb_get_dr_mode_by_phy(child, 0);
    if ((port.dr_mode != USB_DR_MODE_HOST) &&
    (port.dr_mode != USB_DR_MODE_PERIPHERAL)) {
    dev_err(&pdev.dev,
    "Missing dual role setting of the port%d, will use HOST mode\n",
    port_id);
    port.dr_mode = USB_DR_MODE_HOST;
    }
    if (port.dr_mode == USB_DR_MODE_PERIPHERAL) {
    usb_devices++;
    if (usb_devices > 1) {
    dev_err(dev,
    "Single USB device allowed! Port%d will use HOST mode\n",
    port_id);
    port.dr_mode = USB_DR_MODE_HOST;
    }
    }
    of_property_for_each_u32(dev.of_node, "swap-dx-lanes", swap_dx)
    if (swap_dx == port_id)
    port.swap_dx = 1;
// Retrieve PHY capabilities
    utmi.ops = &mvebu_cp110_utmi_phy_ops;
// Instantiate the PHY
    phy = devm_phy_create(dev, child, utmi.ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "Failed to create the UTMI PHY\n");
    of_node_put(child);
    return PTR_ERR(phy);
    }
    port.priv = utmi;
    port.id = port_id;
    phy_set_drvdata(phy, port);
// Ensure the PHY is powered off
    mvebu_cp110_utmi_phy_power_off(phy);
    }
    dev_set_drvdata(dev, utmi);
    provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(provider);
    }
    static struct platform_driver mvebu_cp110_utmi_driver = {
    .probe	= mvebu_cp110_utmi_phy_probe,
    .driver	= {
    .name		= "mvebu-cp110-utmi-phy",
    .of_match_table	= mvebu_cp110_utmi_of_match,
    },
    };
    module_platform_driver(mvebu_cp110_utmi_driver);
    MODULE_AUTHOR("Konstatin Porotchkin <kostap@marvell.com>");
    MODULE_DESCRIPTION("Marvell Armada CP110 UTMI PHY driver");
    MODULE_LICENSE("GPL v2");
