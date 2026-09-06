//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-bcm-iproc.c
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
// Copyright (C) 2015 Broadcom Corporation
//

pub const IPROC_GPHY_MDCDIV: c_uint = 0x1a;
pub const MII_CTRL_OFFSET: c_uint = 0x000;
pub const MII_CTRL_DIV_SHIFT: c_int = 0;
pub const MII_CTRL_PRE_SHIFT: c_int = 7;
pub const MII_CTRL_BUSY_SHIFT: c_int = 8;
pub const MII_DATA_OFFSET: c_uint = 0x004;
pub const MII_DATA_MASK: c_uint = 0xffff;
pub const MII_DATA_TA_SHIFT: c_int = 16;
pub const MII_DATA_TA_VAL: c_int = 2;
pub const MII_DATA_RA_SHIFT: c_int = 18;
pub const MII_DATA_PA_SHIFT: c_int = 23;
pub const MII_DATA_OP_SHIFT: c_int = 28;
pub const MII_DATA_OP_WRITE: c_int = 1;
pub const MII_DATA_OP_READ: c_int = 2;
pub const MII_DATA_SB_SHIFT: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_mdio_priv {
    pub mii_bus: *mut mii_bus,
    pub base: *mut void __iomem,
}

#[no_mangle]
pub unsafe extern "C" fn iproc_mdio_wait_for_idle(base: *mut void __iomem) -> c_int {
    static inline int iproc_mdio_wait_for_idle(void __iomem *base)
    {
    u32 val;
    unsigned int timeout = 1000; /* loop for 1s */
    do {
    val = readl(base + MII_CTRL_OFFSET);
    if ((val & BIT(MII_CTRL_BUSY_SHIFT)) == 0)
    return 0;
    usleep_range(1000, 2000);
    } while (timeout--);
    return -ETIMEDOUT;
    }
#[no_mangle]
pub unsafe extern "C" fn iproc_mdio_config_clk(base: *mut void __iomem) {
    static inline void iproc_mdio_config_clk(void __iomem *base)
    {
    u32 val;
    val = (IPROC_GPHY_MDCDIV << MII_CTRL_DIV_SHIFT) |
    BIT(MII_CTRL_PRE_SHIFT);
    writel(val, base + MII_CTRL_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn iproc_mdio_read(bus: *mut mii_bus, phy_id: c_int, reg: c_int) -> c_int {
    static int iproc_mdio_read(struct mii_bus *bus, int phy_id, int reg)
    {
    struct iproc_mdio_priv *priv = bus.priv;
    u32 cmd;
    int rc;
    rc = iproc_mdio_wait_for_idle(priv.base);
    if (rc)
    return rc;
// Prepare the read operation
    cmd = (MII_DATA_TA_VAL << MII_DATA_TA_SHIFT) |
    (reg << MII_DATA_RA_SHIFT) |
    (phy_id << MII_DATA_PA_SHIFT) |
    BIT(MII_DATA_SB_SHIFT) |
    (MII_DATA_OP_READ << MII_DATA_OP_SHIFT);
    writel(cmd, priv.base + MII_DATA_OFFSET);
    rc = iproc_mdio_wait_for_idle(priv.base);
    if (rc)
    return rc;
    cmd = readl(priv.base + MII_DATA_OFFSET) & MII_DATA_MASK;
    return cmd;
    }
    static int iproc_mdio_write(struct mii_bus *bus, int phy_id,
    int reg, u16 val)
    {
    struct iproc_mdio_priv *priv = bus.priv;
    u32 cmd;
    int rc;
    rc = iproc_mdio_wait_for_idle(priv.base);
    if (rc)
    return rc;
// Prepare the write operation
    cmd = (MII_DATA_TA_VAL << MII_DATA_TA_SHIFT) |
    (reg << MII_DATA_RA_SHIFT) |
    (phy_id << MII_DATA_PA_SHIFT) |
    BIT(MII_DATA_SB_SHIFT) |
    (MII_DATA_OP_WRITE << MII_DATA_OP_SHIFT) |
    ((u32)(val) & MII_DATA_MASK);
    writel(cmd, priv.base + MII_DATA_OFFSET);
    rc = iproc_mdio_wait_for_idle(priv.base);
    if (rc)
    return rc;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iproc_mdio_probe(pdev: *mut platform_device) -> c_int {
    static int iproc_mdio_probe(struct platform_device *pdev)
    {
    struct iproc_mdio_priv *priv;
    struct mii_bus *bus;
    int rc;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base)) {
    dev_err(&pdev.dev, "failed to ioremap register\n");
    return PTR_ERR(priv.base);
    }
    priv.mii_bus = mdiobus_alloc();
    if (!priv.mii_bus) {
    dev_err(&pdev.dev, "MDIO bus alloc failed\n");
    return -ENOMEM;
    }
    bus = priv.mii_bus;
    bus.priv = priv;
    bus.name = "iProc MDIO bus";
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s-%d", pdev.name, pdev.id);
    bus.parent = &pdev.dev;
    bus.read = iproc_mdio_read;
    bus.write = iproc_mdio_write;
    iproc_mdio_config_clk(priv.base);
    rc = of_mdiobus_register(bus, pdev.dev.of_node);
    if (rc) {
    dev_err(&pdev.dev, "MDIO bus registration failed\n");
    goto err_iproc_mdio;
    }
    platform_set_drvdata(pdev, priv);
    dev_info(&pdev.dev, "Broadcom iProc MDIO bus registered\n");
    return 0;
    err_iproc_mdio:
    mdiobus_free(bus);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn iproc_mdio_remove(pdev: *mut platform_device) {
    static void iproc_mdio_remove(struct platform_device *pdev)
    {
    struct iproc_mdio_priv *priv = platform_get_drvdata(pdev);
    mdiobus_unregister(priv.mii_bus);
    mdiobus_free(priv.mii_bus);
    }

#[no_mangle]
unsafe extern "C" fn iproc_mdio_resume(dev: *mut device) -> c_int {
    static int iproc_mdio_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct iproc_mdio_priv *priv = platform_get_drvdata(pdev);
// restore the mii clock configuration
    iproc_mdio_config_clk(priv.base);
    return 0;
    }
    static const struct dev_pm_ops iproc_mdio_pm_ops = {
    .resume = iproc_mdio_resume
    };

    static const struct of_device_id iproc_mdio_of_match[] = {
    { .compatible = "brcm,iproc-mdio", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, iproc_mdio_of_match);
    static struct platform_driver iproc_mdio_driver = {
    .driver = {
    .name = "iproc-mdio",
    .of_match_table = iproc_mdio_of_match,

    .pm = &iproc_mdio_pm_ops,

    },
    .probe = iproc_mdio_probe,
    .remove = iproc_mdio_remove,
    };
    module_platform_driver(iproc_mdio_driver);
    MODULE_AUTHOR("Broadcom Corporation");
    MODULE_DESCRIPTION("Broadcom iProc MDIO bus controller");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:iproc-mdio");
