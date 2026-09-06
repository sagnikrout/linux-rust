//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/freescale/xgmac_mdio.c
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


//
// QorIQ 10G MDIO Controller
//
// Copyright 2012 Freescale Semiconductor, Inc.
// Copyright 2021 NXP
//
// Authors: Andy Fleming <afleming@freescale.com>
// Timur Tabi <timur@freescale.com>
//
// This file is licensed under the terms of the GNU General Public License
// version 2.  This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

// Number of microseconds to wait for a register to respond
pub const TIMEOUT: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tgec_mdio_controller {
    pub reserved: [__be32; 12],
    pub /: *mut *mut __be32 mdio_stat; / MDIO configuration and status,
    pub /: *mut *mut __be32 mdio_ctl; / MDIO control,
    pub /: *mut *mut __be32 mdio_data; / MDIO data,
    pub /: *mut *mut __be32 mdio_addr; / MDIO address,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_fsl_priv {
    pub mdio_base: *mut tgec_mdio_controller __iomem,
    pub enet_clk: *mut clk,
    pub mdc_freq: u32,
    pub is_little_endian: bool,
    pub has_a009885: bool,
    pub has_a011043: bool,
}

    static u32 xgmac_read32(void __iomem *regs,
    bool is_little_endian)
    {
    if (is_little_endian)
    return ioread32(regs);
    else
    return ioread32be(regs);
    }
    static void xgmac_write32(u32 value,
    void __iomem *regs,
    bool is_little_endian)
    {
    if (is_little_endian)
    iowrite32(value, regs);
    else
    iowrite32be(value, regs);
    }
//
// Wait until the MDIO bus is free
//
    static int xgmac_wait_until_free(struct device *dev,
    struct tgec_mdio_controller __iomem *regs,
    bool is_little_endian)
    {
    unsigned int timeout;
// Wait till the bus is free
    timeout = TIMEOUT;
    while ((xgmac_read32(&regs.mdio_stat, is_little_endian) &
    MDIO_STAT_BSY) && timeout) {
    cpu_relax();
    timeout--;
    }
    if (!timeout) {
    dev_err(dev, "timeout waiting for bus to be free\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
//
// Wait till the MDIO read or write operation is complete
//
    static int xgmac_wait_until_done(struct device *dev,
    struct tgec_mdio_controller __iomem *regs,
    bool is_little_endian)
    {
    unsigned int timeout;
// Wait till the MDIO write is complete
    timeout = TIMEOUT;
    while ((xgmac_read32(&regs.mdio_stat, is_little_endian) &
    MDIO_STAT_BSY) && timeout) {
    cpu_relax();
    timeout--;
    }
    if (!timeout) {
    dev_err(dev, "timeout waiting for operation to complete\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
    static int xgmac_mdio_write_c22(struct mii_bus *bus, int phy_id, int regnum,
    u16 value)
    {
    struct mdio_fsl_priv *priv = (struct mdio_fsl_priv *)bus.priv;
    struct tgec_mdio_controller __iomem *regs = priv.mdio_base;
    let mut endian: bool = priv.is_little_endian;
    let mut dev_addr: u16 = regnum & 0x1f;
    u32 mdio_ctl, mdio_stat;
    int ret;
    mdio_stat = xgmac_read32(&regs.mdio_stat, endian);
    mdio_stat &= ~MDIO_STAT_ENC;
    xgmac_write32(mdio_stat, &regs.mdio_stat, endian);
    ret = xgmac_wait_until_free(&bus.dev, regs, endian);
    if (ret)
    return ret;
// Set the port and dev addr
    mdio_ctl = MDIO_CTL_PORT_ADDR(phy_id) | MDIO_CTL_DEV_ADDR(dev_addr);
    xgmac_write32(mdio_ctl, &regs.mdio_ctl, endian);
// Write the value to the register
    xgmac_write32(MDIO_DATA(value), &regs.mdio_data, endian);
    ret = xgmac_wait_until_done(&bus.dev, regs, endian);
    if (ret)
    return ret;
    return 0;
    }
    static int xgmac_mdio_write_c45(struct mii_bus *bus, int phy_id, int dev_addr,
    int regnum, u16 value)
    {
    struct mdio_fsl_priv *priv = (struct mdio_fsl_priv *)bus.priv;
    struct tgec_mdio_controller __iomem *regs = priv.mdio_base;
    let mut endian: bool = priv.is_little_endian;
    u32 mdio_ctl, mdio_stat;
    int ret;
    mdio_stat = xgmac_read32(&regs.mdio_stat, endian);
    mdio_stat |= MDIO_STAT_ENC;
    xgmac_write32(mdio_stat, &regs.mdio_stat, endian);
    ret = xgmac_wait_until_free(&bus.dev, regs, endian);
    if (ret)
    return ret;
// Set the port and dev addr
    mdio_ctl = MDIO_CTL_PORT_ADDR(phy_id) | MDIO_CTL_DEV_ADDR(dev_addr);
    xgmac_write32(mdio_ctl, &regs.mdio_ctl, endian);
// Set the register address
    xgmac_write32(regnum & 0xffff, &regs.mdio_addr, endian);
    ret = xgmac_wait_until_free(&bus.dev, regs, endian);
    if (ret)
    return ret;
// Write the value to the register
    xgmac_write32(MDIO_DATA(value), &regs.mdio_data, endian);
    ret = xgmac_wait_until_done(&bus.dev, regs, endian);
    if (ret)
    return ret;
    return 0;
    }
// Reads from register regnum in the PHY for device dev, returning the value.
// Clears miimcom first.  All PHY configuration has to be done through the
// TSEC1 MIIM regs.
//
#[no_mangle]
unsafe extern "C" fn xgmac_mdio_read_c22(bus: *mut mii_bus, phy_id: c_int, regnum: c_int) -> c_int {
    static int xgmac_mdio_read_c22(struct mii_bus *bus, int phy_id, int regnum)
    {
    struct mdio_fsl_priv *priv = (struct mdio_fsl_priv *)bus.priv;
    struct tgec_mdio_controller __iomem *regs = priv.mdio_base;
    let mut endian: bool = priv.is_little_endian;
    let mut dev_addr: u16 = regnum & 0x1f;
    unsigned long flags;
    uint32_t mdio_stat;
    uint32_t mdio_ctl;
    int ret;
    mdio_stat = xgmac_read32(&regs.mdio_stat, endian);
    mdio_stat &= ~MDIO_STAT_ENC;
    xgmac_write32(mdio_stat, &regs.mdio_stat, endian);
    ret = xgmac_wait_until_free(&bus.dev, regs, endian);
    if (ret)
    return ret;
// Set the Port and Device Addrs
    mdio_ctl = MDIO_CTL_PORT_ADDR(phy_id) | MDIO_CTL_DEV_ADDR(dev_addr);
    xgmac_write32(mdio_ctl, &regs.mdio_ctl, endian);
    if (priv.has_a009885)
// Once the operation completes, i.e. MDIO_STAT_BSY clears, we
// must read back the data register within 16 MDC cycles.
//
    local_irq_save(flags);
// Initiate the read
    xgmac_write32(mdio_ctl | MDIO_CTL_READ, &regs.mdio_ctl, endian);
    ret = xgmac_wait_until_done(&bus.dev, regs, endian);
    if (ret)
    goto irq_restore;
// Return all Fs if nothing was there
    if ((xgmac_read32(&regs.mdio_stat, endian) & MDIO_STAT_RD_ER) &&
    !priv.has_a011043) {
    dev_dbg(&bus.dev,
    "Error while reading PHY%d reg at %d.%d\n",
    phy_id, dev_addr, regnum);
    ret = 0xffff;
    } else {
    ret = xgmac_read32(&regs.mdio_data, endian) & 0xffff;
    dev_dbg(&bus.dev, "read %04x\n", ret);
    }
    irq_restore:
    if (priv.has_a009885)
    local_irq_restore(flags);
    return ret;
    }
// Reads from register regnum in the PHY for device dev, returning the value.
// Clears miimcom first.  All PHY configuration has to be done through the
// TSEC1 MIIM regs.
//
    static int xgmac_mdio_read_c45(struct mii_bus *bus, int phy_id, int dev_addr,
    int regnum)
    {
    struct mdio_fsl_priv *priv = (struct mdio_fsl_priv *)bus.priv;
    struct tgec_mdio_controller __iomem *regs = priv.mdio_base;
    let mut endian: bool = priv.is_little_endian;
    u32 mdio_stat, mdio_ctl;
    unsigned long flags;
    int ret;
    mdio_stat = xgmac_read32(&regs.mdio_stat, endian);
    mdio_stat |= MDIO_STAT_ENC;
    xgmac_write32(mdio_stat, &regs.mdio_stat, endian);
    ret = xgmac_wait_until_free(&bus.dev, regs, endian);
    if (ret)
    return ret;
// Set the Port and Device Addrs
    mdio_ctl = MDIO_CTL_PORT_ADDR(phy_id) | MDIO_CTL_DEV_ADDR(dev_addr);
    xgmac_write32(mdio_ctl, &regs.mdio_ctl, endian);
// Set the register address
    xgmac_write32(regnum & 0xffff, &regs.mdio_addr, endian);
    ret = xgmac_wait_until_free(&bus.dev, regs, endian);
    if (ret)
    return ret;
    if (priv.has_a009885)
// Once the operation completes, i.e. MDIO_STAT_BSY clears, we
// must read back the data register within 16 MDC cycles.
//
    local_irq_save(flags);
// Initiate the read
    xgmac_write32(mdio_ctl | MDIO_CTL_READ, &regs.mdio_ctl, endian);
    ret = xgmac_wait_until_done(&bus.dev, regs, endian);
    if (ret)
    goto irq_restore;
// Return all Fs if nothing was there
    if ((xgmac_read32(&regs.mdio_stat, endian) & MDIO_STAT_RD_ER) &&
    !priv.has_a011043) {
    dev_dbg(&bus.dev,
    "Error while reading PHY%d reg at %d.%d\n",
    phy_id, dev_addr, regnum);
    ret = 0xffff;
    } else {
    ret = xgmac_read32(&regs.mdio_data, endian) & 0xffff;
    dev_dbg(&bus.dev, "read %04x\n", ret);
    }
    irq_restore:
    if (priv.has_a009885)
    local_irq_restore(flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xgmac_mdio_set_mdc_freq(bus: *mut mii_bus) -> c_int {
    static int xgmac_mdio_set_mdc_freq(struct mii_bus *bus)
    {
    struct mdio_fsl_priv *priv = (struct mdio_fsl_priv *)bus.priv;
    struct tgec_mdio_controller __iomem *regs = priv.mdio_base;
    struct device *dev = bus.parent;
    u32 mdio_stat, div;
    if (device_property_read_u32(dev, "clock-frequency", &priv.mdc_freq))
    return 0;
    priv.enet_clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.enet_clk)) {
    dev_err(dev, "Input clock unknown, not changing MDC frequency");
    return PTR_ERR(priv.enet_clk);
    }
    div = ((clk_get_rate(priv.enet_clk) / priv.mdc_freq) - 1) / 2;
    if (div < 5 || div > 0x1ff) {
    dev_err(dev, "Requested MDC frequency is out of range, ignoring");
    return -EINVAL;
    }
    mdio_stat = xgmac_read32(&regs.mdio_stat, priv.is_little_endian);
    mdio_stat &= ~MDIO_STAT_CLKDIV(0x1ff);
    mdio_stat |= MDIO_STAT_CLKDIV(div);
    xgmac_write32(mdio_stat, &regs.mdio_stat, priv.is_little_endian);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgmac_mdio_set_suppress_preamble(bus: *mut mii_bus) {
    static void xgmac_mdio_set_suppress_preamble(struct mii_bus *bus)
    {
    struct mdio_fsl_priv *priv = (struct mdio_fsl_priv *)bus.priv;
    struct tgec_mdio_controller __iomem *regs = priv.mdio_base;
    struct device *dev = bus.parent;
    u32 mdio_stat;
    if (!device_property_read_bool(dev, "suppress-preamble"))
    return;
    mdio_stat = xgmac_read32(&regs.mdio_stat, priv.is_little_endian);
    mdio_stat |= MDIO_STAT_PRE_DIS;
    xgmac_write32(mdio_stat, &regs.mdio_stat, priv.is_little_endian);
    }
#[no_mangle]
unsafe extern "C" fn xgmac_mdio_probe(pdev: *mut platform_device) -> c_int {
    static int xgmac_mdio_probe(struct platform_device *pdev)
    {
    struct fwnode_handle *fwnode;
    struct mdio_fsl_priv *priv;
    struct resource *res;
    struct mii_bus *bus;
    int ret;
// In DPAA-1, MDIO is one of the many FMan sub-devices. The FMan
// defines a register space that spans a large area, covering all the
// subdevice areas. Therefore, MDIO cannot claim exclusive access to
// this register area.
//
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(&pdev.dev, "could not obtain address\n");
    return -EINVAL;
    }
    bus = devm_mdiobus_alloc_size(&pdev.dev, sizeof(struct mdio_fsl_priv));
    if (!bus)
    return -ENOMEM;
    bus.name = "Freescale XGMAC MDIO Bus";
    bus.read = xgmac_mdio_read_c22;
    bus.write = xgmac_mdio_write_c22;
    bus.read_c45 = xgmac_mdio_read_c45;
    bus.write_c45 = xgmac_mdio_write_c45;
    bus.parent = &pdev.dev;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%pa", &res.start);
    priv = bus.priv;
    priv.mdio_base = devm_ioremap(&pdev.dev, res.start,
    resource_size(res));
    if (!priv.mdio_base)
    return -ENOMEM;
// For both ACPI and DT cases, endianness of MDIO controller
// needs to be specified using "little-endian" property.
//
    priv.is_little_endian = device_property_read_bool(&pdev.dev,
    "little-endian");
    priv.has_a009885 = device_property_read_bool(&pdev.dev,
    "fsl,erratum-a009885");
    priv.has_a011043 = device_property_read_bool(&pdev.dev,
    "fsl,erratum-a011043");
    xgmac_mdio_set_suppress_preamble(bus);
    ret = xgmac_mdio_set_mdc_freq(bus);
    if (ret)
    return ret;
    fwnode = dev_fwnode(&pdev.dev);
    if (is_of_node(fwnode))
    ret = of_mdiobus_register(bus, to_of_node(fwnode));
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_acpi_node(fwnode)) -> else {
    else if (is_acpi_node(fwnode))
    ret = acpi_mdiobus_register(bus, fwnode);
    else
    ret = -EINVAL;
    if (ret) {
    dev_err(&pdev.dev, "cannot register MDIO bus\n");
    return ret;
    }
    platform_set_drvdata(pdev, bus);
    return 0;
    }
    static const struct of_device_id xgmac_mdio_match[] = {
    {
    .compatible = "fsl,fman-xmdio",
    },
    {
    .compatible = "fsl,fman-memac-mdio",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, xgmac_mdio_match);
    static const struct acpi_device_id xgmac_acpi_match[] = {
    { "NXP0006" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, xgmac_acpi_match);
    static struct platform_driver xgmac_mdio_driver = {
    .driver = {
    .name = "fsl-fman_xmdio",
    .of_match_table = xgmac_mdio_match,
    .acpi_match_table = xgmac_acpi_match,
    },
    .probe = xgmac_mdio_probe,
    };
    module_platform_driver(xgmac_mdio_driver);
    MODULE_DESCRIPTION("Freescale QorIQ 10G MDIO Controller");
    MODULE_LICENSE("GPL v2");
