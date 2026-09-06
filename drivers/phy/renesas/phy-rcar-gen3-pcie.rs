//! Automatically rewritten from C to Rust
//! Source: drivers/phy/renesas/phy-rcar-gen3-pcie.c
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
// Renesas R-Car Gen3 PCIe PHY driver
//
// Copyright (C) 2018 Cogent Embedded, Inc.
//

pub const PHY_CTRL: c_uint = 0x4000		/* R8A77980 only */;
// PHY control register (PHY_CTRL)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_gen3_phy {
    pub phy: *mut phy,
    pub lock: spinlock_t,
    pub base: *mut void __iomem,
}

    static void rcar_gen3_phy_pcie_modify_reg(struct phy *p, unsigned int reg,
    u32 clear, u32 set)
    {
    struct rcar_gen3_phy *phy = phy_get_drvdata(p);
    void __iomem *base = phy.base;
    unsigned long flags;
    u32 value;
    spin_lock_irqsave(&phy.lock, flags);
    value = readl(base + reg);
    value &= ~clear;
    value |= set;
    writel(value, base + reg);
    spin_unlock_irqrestore(&phy.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn r8a77980_phy_pcie_power_on(p: *mut phy) -> c_int {
    static int r8a77980_phy_pcie_power_on(struct phy *p)
    {
// Power on the PCIe PHY
    rcar_gen3_phy_pcie_modify_reg(p, PHY_CTRL, PHY_CTRL_PHY_PWDN, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn r8a77980_phy_pcie_power_off(p: *mut phy) -> c_int {
    static int r8a77980_phy_pcie_power_off(struct phy *p)
    {
// Power off the PCIe PHY
    rcar_gen3_phy_pcie_modify_reg(p, PHY_CTRL, 0, PHY_CTRL_PHY_PWDN);
    return 0;
    }
    static const struct phy_ops r8a77980_phy_pcie_ops = {
    .power_on	= r8a77980_phy_pcie_power_on,
    .power_off	= r8a77980_phy_pcie_power_off,
    .owner		= THIS_MODULE,
    };
    static const struct of_device_id rcar_gen3_phy_pcie_match_table[] = {
    { .compatible = "renesas,r8a77980-pcie-phy" },
    { }
    };
    MODULE_DEVICE_TABLE(of, rcar_gen3_phy_pcie_match_table);
#[no_mangle]
unsafe extern "C" fn rcar_gen3_phy_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int rcar_gen3_phy_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct phy_provider *provider;
    struct rcar_gen3_phy *phy;
    void __iomem *base;
    int error;
    if (!dev.of_node) {
    dev_err(dev,
    "This driver must only be instantiated from the device tree\n");
    return -EINVAL;
    }
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    phy = devm_kzalloc(dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    spin_lock_init(&phy.lock);
    phy.base = base;
//
// devm_phy_create() will call pm_runtime_enable(&phy->dev);
// And then, phy-core will manage runtime PM for this device.
//
    pm_runtime_enable(dev);
    phy.phy = devm_phy_create(dev, core::ptr::null_mut(), &r8a77980_phy_pcie_ops);
    if (IS_ERR(phy.phy)) {
    dev_err(dev, "Failed to create PCIe PHY\n");
    error = PTR_ERR(phy.phy);
    goto error;
    }
    phy_set_drvdata(phy.phy, phy);
    provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(provider)) {
    dev_err(dev, "Failed to register PHY provider\n");
    error = PTR_ERR(provider);
    goto error;
    }
    return 0;
    error:
    pm_runtime_disable(dev);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn rcar_gen3_phy_pcie_remove(pdev: *mut platform_device) {
    static void rcar_gen3_phy_pcie_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    }
    static struct platform_driver rcar_gen3_phy_driver = {
    .driver = {
    .name = "phy_rcar_gen3_pcie",
    .of_match_table = rcar_gen3_phy_pcie_match_table,
    },
    .probe = rcar_gen3_phy_pcie_probe,
    .remove = rcar_gen3_phy_pcie_remove,
    };
    module_platform_driver(rcar_gen3_phy_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Renesas R-Car Gen3 PCIe PHY");
    MODULE_AUTHOR("Sergei Shtylyov <sergei.shtylyov@cogentembedded.com>");
