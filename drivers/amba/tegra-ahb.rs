//! Automatically rewritten from C to Rust
//! Source: drivers/amba/tegra-ahb.c
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
//
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
// Copyright (C) 2011 Google, Inc.
//
// Author:
// Jay Cheng <jacheng@nvidia.com>
// James Wylder <james.wylder@motorola.com>
// Benoit Goby <benoit@android.com>
// Colin Cross <ccross@android.com>
// Hiroshi DOYU <hdoyu@nvidia.com>
//

pub const AHB_ARBITRATION_DISABLE: c_uint = 0x04;
pub const AHB_ARBITRATION_PRIORITY_CTRL: c_uint = 0x08;

pub const AHB_GIZMO_AHB_MEM: c_uint = 0x10;

pub const AHB_GIZMO_APB_DMA: c_uint = 0x14;
pub const AHB_GIZMO_IDE: c_uint = 0x1c;
pub const AHB_GIZMO_USB: c_uint = 0x20;
pub const AHB_GIZMO_AHB_XBAR_BRIDGE: c_uint = 0x24;
pub const AHB_GIZMO_CPU_AHB_BRIDGE: c_uint = 0x28;
pub const AHB_GIZMO_COP_AHB_BRIDGE: c_uint = 0x2c;
pub const AHB_GIZMO_XBAR_APB_CTLR: c_uint = 0x30;
pub const AHB_GIZMO_VCP_AHB_BRIDGE: c_uint = 0x34;
pub const AHB_GIZMO_NAND: c_uint = 0x40;
pub const AHB_GIZMO_SDMMC4: c_uint = 0x48;
pub const AHB_GIZMO_XIO: c_uint = 0x4c;
pub const AHB_GIZMO_BSEV: c_uint = 0x64;
pub const AHB_GIZMO_BSEA: c_uint = 0x74;
pub const AHB_GIZMO_NOR: c_uint = 0x78;
pub const AHB_GIZMO_USB2: c_uint = 0x7c;
pub const AHB_GIZMO_USB3: c_uint = 0x80;

pub const AHB_GIZMO_SDMMC1: c_uint = 0x84;
pub const AHB_GIZMO_SDMMC2: c_uint = 0x88;
pub const AHB_GIZMO_SDMMC3: c_uint = 0x8c;
pub const AHB_MEM_PREFETCH_CFG_X: c_uint = 0xdc;
pub const AHB_ARBITRATION_XBAR_CTRL: c_uint = 0xe0;
pub const AHB_MEM_PREFETCH_CFG3: c_uint = 0xe4;
pub const AHB_MEM_PREFETCH_CFG4: c_uint = 0xe8;
pub const AHB_MEM_PREFETCH_CFG1: c_uint = 0xf0;
pub const AHB_MEM_PREFETCH_CFG2: c_uint = 0xf4;

pub const AHB_ARBITRATION_AHB_MEM_WRQUE_MST_ID: c_uint = 0xfc;

//
// INCORRECT_BASE_ADDR_LOW_BYTE: Legacy kernel DT files for Tegra SoCs
// prior to Tegra124 generally use a physical base address ending in
// 0x4 for the AHB IP block.  According to the TRM, the low byte
// should be 0x0.  During device probing, this macro is used to detect
// whether the passed-in physical address is incorrect, and if so, to
// correct it.
//
pub const INCORRECT_BASE_ADDR_LOW_BYTE: c_uint = 0x4;
    static struct platform_driver tegra_ahb_driver;
    static const u32 tegra_ahb_gizmo[] = {
    AHB_ARBITRATION_DISABLE,
    AHB_ARBITRATION_PRIORITY_CTRL,
    AHB_GIZMO_AHB_MEM,
    AHB_GIZMO_APB_DMA,
    AHB_GIZMO_IDE,
    AHB_GIZMO_USB,
    AHB_GIZMO_AHB_XBAR_BRIDGE,
    AHB_GIZMO_CPU_AHB_BRIDGE,
    AHB_GIZMO_COP_AHB_BRIDGE,
    AHB_GIZMO_XBAR_APB_CTLR,
    AHB_GIZMO_VCP_AHB_BRIDGE,
    AHB_GIZMO_NAND,
    AHB_GIZMO_SDMMC4,
    AHB_GIZMO_XIO,
    AHB_GIZMO_BSEV,
    AHB_GIZMO_BSEA,
    AHB_GIZMO_NOR,
    AHB_GIZMO_USB2,
    AHB_GIZMO_USB3,
    AHB_GIZMO_SDMMC1,
    AHB_GIZMO_SDMMC2,
    AHB_GIZMO_SDMMC3,
    AHB_MEM_PREFETCH_CFG_X,
    AHB_ARBITRATION_XBAR_CTRL,
    AHB_MEM_PREFETCH_CFG3,
    AHB_MEM_PREFETCH_CFG4,
    AHB_MEM_PREFETCH_CFG1,
    AHB_MEM_PREFETCH_CFG2,
    AHB_ARBITRATION_AHB_MEM_WRQUE_MST_ID,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_ahb {
    pub regs: *mut void __iomem,
    pub dev: *mut device,
    pub ctx: [u32; ],
}

#[no_mangle]
pub unsafe extern "C" fn gizmo_readl(ahb: *mut tegra_ahb, offset: u32) -> u32 {
    static inline u32 gizmo_readl(struct tegra_ahb *ahb, u32 offset)
    {
    return readl(ahb.regs + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn gizmo_writel(ahb: *mut tegra_ahb, value: u32, offset: u32) {
    static inline void gizmo_writel(struct tegra_ahb *ahb, u32 value, u32 offset)
    {
    writel(value, ahb.regs + offset);
    }

#[no_mangle]
pub unsafe extern "C" fn tegra_ahb_enable_smmu(dn: *mut device_node) -> c_int {
    int tegra_ahb_enable_smmu(struct device_node *dn)
    {
    struct device *dev;
    u32 val;
    struct tegra_ahb *ahb;
    dev = driver_find_device_by_of_node(&tegra_ahb_driver.driver, dn);
    if (!dev)
    return -EPROBE_DEFER;
    ahb = dev_get_drvdata(dev);
    put_device(dev);
    val = gizmo_readl(ahb, AHB_ARBITRATION_XBAR_CTRL);
    val |= AHB_ARBITRATION_XBAR_CTRL_SMMU_INIT_DONE;
    gizmo_writel(ahb, val, AHB_ARBITRATION_XBAR_CTRL);
    return 0;
    }
    EXPORT_SYMBOL(tegra_ahb_enable_smmu);

#[no_mangle]
unsafe extern "C" fn tegra_ahb_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra_ahb_suspend(struct device *dev)
    {
    int i;
    struct tegra_ahb *ahb = dev_get_drvdata(dev);
    for (i = 0; i < ARRAY_SIZE(tegra_ahb_gizmo); i++)
    ahb.ctx[i] = gizmo_readl(ahb, tegra_ahb_gizmo[i]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_ahb_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra_ahb_resume(struct device *dev)
    {
    int i;
    struct tegra_ahb *ahb = dev_get_drvdata(dev);
    for (i = 0; i < ARRAY_SIZE(tegra_ahb_gizmo); i++)
    gizmo_writel(ahb, ahb.ctx[i], tegra_ahb_gizmo[i]);
    return 0;
    }
    static UNIVERSAL_DEV_PM_OPS(tegra_ahb_pm,
    tegra_ahb_suspend,
    tegra_ahb_resume, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn tegra_ahb_gizmo_init(ahb: *mut tegra_ahb) {
    static void tegra_ahb_gizmo_init(struct tegra_ahb *ahb)
    {
    u32 val;
    val = gizmo_readl(ahb, AHB_GIZMO_AHB_MEM);
    val |= ENB_FAST_REARBITRATE | IMMEDIATE | DONT_SPLIT_AHB_WR;
    gizmo_writel(ahb, val, AHB_GIZMO_AHB_MEM);
    val = gizmo_readl(ahb, AHB_GIZMO_USB);
    val |= IMMEDIATE;
    gizmo_writel(ahb, val, AHB_GIZMO_USB);
    val = gizmo_readl(ahb, AHB_GIZMO_USB2);
    val |= IMMEDIATE;
    gizmo_writel(ahb, val, AHB_GIZMO_USB2);
    val = gizmo_readl(ahb, AHB_GIZMO_USB3);
    val |= IMMEDIATE;
    gizmo_writel(ahb, val, AHB_GIZMO_USB3);
    val = gizmo_readl(ahb, AHB_ARBITRATION_PRIORITY_CTRL);
    val |= PRIORITY_SELECT_USB |
    PRIORITY_SELECT_USB2 |
    PRIORITY_SELECT_USB3 |
    AHB_PRIORITY_WEIGHT(7);
    gizmo_writel(ahb, val, AHB_ARBITRATION_PRIORITY_CTRL);
    val = gizmo_readl(ahb, AHB_MEM_PREFETCH_CFG1);
    val &= ~MST_ID(~0);
    val |= PREFETCH_ENB |
    AHBDMA_MST_ID |
    ADDR_BNDRY(0xc) |
    INACTIVITY_TIMEOUT(0x1000);
    gizmo_writel(ahb, val, AHB_MEM_PREFETCH_CFG1);
    val = gizmo_readl(ahb, AHB_MEM_PREFETCH_CFG2);
    val &= ~MST_ID(~0);
    val |= PREFETCH_ENB |
    USB_MST_ID |
    ADDR_BNDRY(0xc) |
    INACTIVITY_TIMEOUT(0x1000);
    gizmo_writel(ahb, val, AHB_MEM_PREFETCH_CFG2);
    val = gizmo_readl(ahb, AHB_MEM_PREFETCH_CFG3);
    val &= ~MST_ID(~0);
    val |= PREFETCH_ENB |
    USB3_MST_ID |
    ADDR_BNDRY(0xc) |
    INACTIVITY_TIMEOUT(0x1000);
    gizmo_writel(ahb, val, AHB_MEM_PREFETCH_CFG3);
    val = gizmo_readl(ahb, AHB_MEM_PREFETCH_CFG4);
    val &= ~MST_ID(~0);
    val |= PREFETCH_ENB |
    USB2_MST_ID |
    ADDR_BNDRY(0xc) |
    INACTIVITY_TIMEOUT(0x1000);
    gizmo_writel(ahb, val, AHB_MEM_PREFETCH_CFG4);
    }
#[no_mangle]
unsafe extern "C" fn tegra_ahb_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_ahb_probe(struct platform_device *pdev)
    {
    struct resource *res;
    struct tegra_ahb *ahb;
    size_t bytes;
    bytes = sizeof(*ahb) + sizeof(u32) * ARRAY_SIZE(tegra_ahb_gizmo);
    ahb = devm_kzalloc(&pdev.dev, bytes, GFP_KERNEL);
    if (!ahb)
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
// Correct the IP block base address if necessary
    if (res &&
    (res.start & INCORRECT_BASE_ADDR_LOW_BYTE) ==
    INCORRECT_BASE_ADDR_LOW_BYTE) {
    dev_warn(&pdev.dev, "incorrect AHB base address in DT data - enabling workaround\n");
    res.start -= INCORRECT_BASE_ADDR_LOW_BYTE;
    }
    ahb.regs = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(ahb.regs))
    return PTR_ERR(ahb.regs);
    ahb.dev = &pdev.dev;
    platform_set_drvdata(pdev, ahb);
    tegra_ahb_gizmo_init(ahb);
    return 0;
    }
    static const struct of_device_id tegra_ahb_of_match[] = {
    { .compatible = "nvidia,tegra30-ahb", },
    { .compatible = "nvidia,tegra20-ahb", },
    {},
    };
    static struct platform_driver tegra_ahb_driver = {
    .probe = tegra_ahb_probe,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = tegra_ahb_of_match,
    .pm = &tegra_ahb_pm,
    },
    };
    module_platform_driver(tegra_ahb_driver);
    MODULE_AUTHOR("Hiroshi DOYU <hdoyu@nvidia.com>");
    MODULE_DESCRIPTION("Tegra AHB driver");
    MODULE_ALIAS("platform:" DRV_NAME);
