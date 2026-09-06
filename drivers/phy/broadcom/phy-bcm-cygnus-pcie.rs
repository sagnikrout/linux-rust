//! Automatically rewritten from C to Rust
//! Source: drivers/phy/broadcom/phy-bcm-cygnus-pcie.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2015 Broadcom Corporation

pub const PCIE_CFG_OFFSET: c_uint = 0x00;
pub const PCIE1_PHY_IDDQ_SHIFT: c_int = 10;
pub const PCIE0_PHY_IDDQ_SHIFT: c_int = 2;
    enum cygnus_pcie_phy_id {
    CYGNUS_PHY_PCIE0 = 0,
    CYGNUS_PHY_PCIE1,
    MAX_NUM_PHYS,
    };
    struct cygnus_pcie_phy_core;
//
// struct cygnus_pcie_phy - Cygnus PCIe PHY device
// @core: pointer to the Cygnus PCIe PHY core control
// @id: internal ID to identify the Cygnus PCIe PHY
// @phy: pointer to the kernel PHY device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cygnus_pcie_phy {
    pub core: *mut cygnus_pcie_phy_core,
    pub id: enum cygnus_pcie_phy_id,
    pub phy: *mut phy,
}

//
// struct cygnus_pcie_phy_core - Cygnus PCIe PHY core control
// @dev: pointer to device
// @base: base register
// @lock: mutex to protect access to individual PHYs
// @phys: pointer to Cygnus PHY device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cygnus_pcie_phy_core {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub lock: mutex,
    pub phys: [cygnus_pcie_phy; MAX_NUM_PHYS],
}

#[no_mangle]
unsafe extern "C" fn cygnus_pcie_power_config(phy: *mut cygnus_pcie_phy, enable: bool) -> c_int {
    static int cygnus_pcie_power_config(struct cygnus_pcie_phy *phy, bool enable)
    {
    struct cygnus_pcie_phy_core *core = phy.core;
    unsigned shift;
    u32 val;
    mutex_lock(&core.lock);
    switch (phy.id) {
    case CYGNUS_PHY_PCIE0:
    shift = PCIE0_PHY_IDDQ_SHIFT;
    break;
    case CYGNUS_PHY_PCIE1:
    shift = PCIE1_PHY_IDDQ_SHIFT;
    break;
    default:
    mutex_unlock(&core.lock);
    dev_err(core.dev, "PCIe PHY %d invalid\n", phy.id);
    return -EINVAL;
    }
    if (enable) {
    val = readl(core.base + PCIE_CFG_OFFSET);
    val &= ~BIT(shift);
    writel(val, core.base + PCIE_CFG_OFFSET);
//
// Wait 50 ms for the PCIe Serdes to stabilize after the analog
// front end is brought up
//
    msleep(50);
    } else {
    val = readl(core.base + PCIE_CFG_OFFSET);
    val |= BIT(shift);
    writel(val, core.base + PCIE_CFG_OFFSET);
    }
    mutex_unlock(&core.lock);
    dev_dbg(core.dev, "PCIe PHY %d %s\n", phy.id,
    enable ? "enabled" : "disabled");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cygnus_pcie_phy_power_on(p: *mut phy) -> c_int {
    static int cygnus_pcie_phy_power_on(struct phy *p)
    {
    struct cygnus_pcie_phy *phy = phy_get_drvdata(p);
    return cygnus_pcie_power_config(phy, true);
    }
#[no_mangle]
unsafe extern "C" fn cygnus_pcie_phy_power_off(p: *mut phy) -> c_int {
    static int cygnus_pcie_phy_power_off(struct phy *p)
    {
    struct cygnus_pcie_phy *phy = phy_get_drvdata(p);
    return cygnus_pcie_power_config(phy, false);
    }
    static const struct phy_ops cygnus_pcie_phy_ops = {
    .power_on = cygnus_pcie_phy_power_on,
    .power_off = cygnus_pcie_phy_power_off,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn cygnus_pcie_phy_probe(pdev: *mut platform_device) -> c_int {
    static int cygnus_pcie_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct cygnus_pcie_phy_core *core;
    struct phy_provider *provider;
    let mut cnt: unsigned = 0;
    if (of_get_child_count(node) == 0) {
    dev_err(dev, "PHY no child node\n");
    return -ENODEV;
    }
    core = devm_kzalloc(dev, sizeof(*core), GFP_KERNEL);
    if (!core)
    return -ENOMEM;
    core.dev = dev;
    core.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(core.base))
    return PTR_ERR(core.base);
    mutex_init(&core.lock);
    for_each_available_child_of_node_scoped(node, child) {
    unsigned int id;
    struct cygnus_pcie_phy *p;
    if (of_property_read_u32(child, "reg", &id)) {
    dev_err(dev, "missing reg property for %pOFn\n",
    child);
    return -EINVAL;
    }
    if (id >= MAX_NUM_PHYS) {
    dev_err(dev, "invalid PHY id: %u\n", id);
    return -EINVAL;
    }
    if (core.phys[id].phy) {
    dev_err(dev, "duplicated PHY id: %u\n", id);
    return -EINVAL;
    }
    p = &core.phys[id];
    p.phy = devm_phy_create(dev, child, &cygnus_pcie_phy_ops);
    if (IS_ERR(p.phy)) {
    dev_err(dev, "failed to create PHY\n");
    return PTR_ERR(p.phy);
    }
    p.core = core;
    p.id = id;
    phy_set_drvdata(p.phy, p);
    cnt++;
    }
    dev_set_drvdata(dev, core);
    provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(provider)) {
    dev_err(dev, "failed to register PHY provider\n");
    return PTR_ERR(provider);
    }
    dev_dbg(dev, "registered %u PCIe PHY(s)\n", cnt);
    return 0;
    }
    static const struct of_device_id cygnus_pcie_phy_match_table[] = {
    { .compatible = "brcm,cygnus-pcie-phy" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, cygnus_pcie_phy_match_table);
    static struct platform_driver cygnus_pcie_phy_driver = {
    .driver = {
    .name = "cygnus-pcie-phy",
    .of_match_table = cygnus_pcie_phy_match_table,
    },
    .probe = cygnus_pcie_phy_probe,
    };
    module_platform_driver(cygnus_pcie_phy_driver);
    MODULE_AUTHOR("Ray Jui <rjui@broadcom.com>");
    MODULE_DESCRIPTION("Broadcom Cygnus PCIe PHY driver");
    MODULE_LICENSE("GPL v2");
