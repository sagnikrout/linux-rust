//! Automatically rewritten from C to Rust
//! Source: drivers/phy/samsung/phy-exynos5250-sata.c
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
// Samsung SATA SerDes(PHY) driver
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// Authors: Girish K S <ks.giri@samsung.com>
// Yuvaraj Kumar C D <yuvaraj.cd@samsung.com>
//

pub const SATAPHY_CONTROL_OFFSET: c_uint = 0x0724;

pub const EXYNOS5_SATA_RESET: c_uint = 0x4;

pub const LINK_RESET: c_uint = 0xf0000;
pub const EXYNOS5_SATA_MODE0: c_uint = 0x10;

pub const EXYNOS5_SATA_CTRL0: c_uint = 0x14;

pub const EXYNOS5_SATA_PHSATA_CTRLM: c_uint = 0xe0;

pub const EXYNOS5_SATA_PHSATA_STATM: c_uint = 0xf0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_sata_phy {
    pub phy: *mut phy,
    pub phyclk: *mut clk,
    pub regs: *mut void __iomem,
    pub pmureg: *mut regmap,
    pub client: *mut i2c_client,
}

    static int wait_for_reg_status(void __iomem *base, u32 reg, u32 checkbit,
    u32 status)
    {
    let mut timeout: c_ulong = jiffies + PHY_PLL_TIMEOUT;
    while (time_before(jiffies, timeout)) {
    if ((readl(base + reg) & checkbit) == status)
    return 0;
    }
    return -EFAULT;
    }
#[no_mangle]
unsafe extern "C" fn exynos_sata_phy_power_on(phy: *mut phy) -> c_int {
    static int exynos_sata_phy_power_on(struct phy *phy)
    {
    struct exynos_sata_phy *sata_phy = phy_get_drvdata(phy);
    return regmap_update_bits(sata_phy.pmureg, SATAPHY_CONTROL_OFFSET,
    EXYNOS5_SATAPHY_PMU_ENABLE, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos_sata_phy_power_off(phy: *mut phy) -> c_int {
    static int exynos_sata_phy_power_off(struct phy *phy)
    {
    struct exynos_sata_phy *sata_phy = phy_get_drvdata(phy);
    return regmap_update_bits(sata_phy.pmureg, SATAPHY_CONTROL_OFFSET,
    EXYNOS5_SATAPHY_PMU_ENABLE, false);
    }
#[no_mangle]
unsafe extern "C" fn exynos_sata_phy_init(phy: *mut phy) -> c_int {
    static int exynos_sata_phy_init(struct phy *phy)
    {
    let mut val: u32 = 0;
    let mut ret: c_int = 0;
    u8 buf[] = { 0x3a, 0x0b };
    struct exynos_sata_phy *sata_phy = phy_get_drvdata(phy);
    ret = regmap_update_bits(sata_phy.pmureg, SATAPHY_CONTROL_OFFSET,
    EXYNOS5_SATAPHY_PMU_ENABLE, true);
    if (ret != 0)
    dev_err(&sata_phy.phy.dev, "phy init failed\n");
    writel(val, sata_phy.regs + EXYNOS5_SATA_RESET);
    val = readl(sata_phy.regs + EXYNOS5_SATA_RESET);
    val |= RESET_GLOBAL_RST_N | RESET_CMN_RST_N | RESET_CMN_BLOCK_RST_N
    | RESET_CMN_I2C_RST_N | RESET_TX_RX_PIPE_RST_N
    | RESET_TX_RX_BLOCK_RST_N | RESET_TX_RX_I2C_RST_N;
    writel(val, sata_phy.regs + EXYNOS5_SATA_RESET);
    val = readl(sata_phy.regs + EXYNOS5_SATA_RESET);
    val |= LINK_RESET;
    writel(val, sata_phy.regs + EXYNOS5_SATA_RESET);
    val = readl(sata_phy.regs + EXYNOS5_SATA_RESET);
    val |= RESET_CMN_RST_N;
    writel(val, sata_phy.regs + EXYNOS5_SATA_RESET);
    val = readl(sata_phy.regs + EXYNOS5_SATA_PHSATA_CTRLM);
    val &= ~PHCTRLM_REF_RATE;
    writel(val, sata_phy.regs + EXYNOS5_SATA_PHSATA_CTRLM);
// High speed enable for Gen3
    val = readl(sata_phy.regs + EXYNOS5_SATA_PHSATA_CTRLM);
    val |= PHCTRLM_HIGH_SPEED;
    writel(val, sata_phy.regs + EXYNOS5_SATA_PHSATA_CTRLM);
    val = readl(sata_phy.regs + EXYNOS5_SATA_CTRL0);
    val |= CTRL0_P0_PHY_CALIBRATED_SEL | CTRL0_P0_PHY_CALIBRATED;
    writel(val, sata_phy.regs + EXYNOS5_SATA_CTRL0);
    val = readl(sata_phy.regs + EXYNOS5_SATA_MODE0);
    val |= SATA_SPD_GEN3;
    writel(val, sata_phy.regs + EXYNOS5_SATA_MODE0);
    ret = i2c_master_send(sata_phy.client, buf, sizeof(buf));
    if (ret < 0)
    return ret;
// release cmu reset
    val = readl(sata_phy.regs + EXYNOS5_SATA_RESET);
    val &= ~RESET_CMN_RST_N;
    writel(val, sata_phy.regs + EXYNOS5_SATA_RESET);
    val = readl(sata_phy.regs + EXYNOS5_SATA_RESET);
    val |= RESET_CMN_RST_N;
    writel(val, sata_phy.regs + EXYNOS5_SATA_RESET);
    ret = wait_for_reg_status(sata_phy.regs,
    EXYNOS5_SATA_PHSATA_STATM,
    PHSTATM_PLL_LOCKED, 1);
    if (ret < 0)
    dev_err(&sata_phy.phy.dev,
    "PHY PLL locking failed\n");
    return ret;
    }
    static const struct phy_ops exynos_sata_phy_ops = {
    .init		= exynos_sata_phy_init,
    .power_on	= exynos_sata_phy_power_on,
    .power_off	= exynos_sata_phy_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn exynos_sata_phy_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_sata_phy_probe(struct platform_device *pdev)
    {
    struct exynos_sata_phy *sata_phy;
    struct device *dev = &pdev.dev;
    struct phy_provider *phy_provider;
    struct device_node *node;
    let mut ret: c_int = 0;
    sata_phy = devm_kzalloc(dev, sizeof(*sata_phy), GFP_KERNEL);
    if (!sata_phy)
    return -ENOMEM;
    sata_phy.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(sata_phy.regs))
    return PTR_ERR(sata_phy.regs);
    sata_phy.pmureg = syscon_regmap_lookup_by_phandle(dev.of_node,
    "samsung,syscon-phandle");
    if (IS_ERR(sata_phy.pmureg)) {
    dev_err(dev, "syscon regmap lookup failed.\n");
    return PTR_ERR(sata_phy.pmureg);
    }
    node = of_parse_phandle(dev.of_node,
    "samsung,exynos-sataphy-i2c-phandle", 0);
    if (!node)
    return -EINVAL;
    sata_phy.client = of_find_i2c_device_by_node(node);
    of_node_put(node);
    if (!sata_phy.client)
    return -EPROBE_DEFER;
    dev_set_drvdata(dev, sata_phy);
    sata_phy.phyclk = devm_clk_get(dev, "sata_phyctrl");
    if (IS_ERR(sata_phy.phyclk)) {
    dev_err(dev, "failed to get clk for PHY\n");
    ret = PTR_ERR(sata_phy.phyclk);
    goto put_dev;
    }
    ret = clk_prepare_enable(sata_phy.phyclk);
    if (ret < 0) {
    dev_err(dev, "failed to enable source clk\n");
    goto put_dev;
    }
    sata_phy.phy = devm_phy_create(dev, core::ptr::null_mut(), &exynos_sata_phy_ops);
    if (IS_ERR(sata_phy.phy)) {
    dev_err(dev, "failed to create PHY\n");
    ret = PTR_ERR(sata_phy.phy);
    goto clk_disable;
    }
    phy_set_drvdata(sata_phy.phy, sata_phy);
    phy_provider = devm_of_phy_provider_register(dev,
    of_phy_simple_xlate);
    if (IS_ERR(phy_provider)) {
    ret = PTR_ERR(phy_provider);
    goto clk_disable;
    }
    return 0;
    clk_disable:
    clk_disable_unprepare(sata_phy.phyclk);
    put_dev:
    put_device(&sata_phy.client.dev);
    return ret;
    }
    static const struct of_device_id exynos_sata_phy_of_match[] = {
    { .compatible = "samsung,exynos5250-sata-phy" },
    { },
    };
    MODULE_DEVICE_TABLE(of, exynos_sata_phy_of_match);
    static struct platform_driver exynos_sata_phy_driver = {
    .probe	= exynos_sata_phy_probe,
    .driver = {
    .of_match_table	= exynos_sata_phy_of_match,
    .name  = "samsung,sata-phy",
    .suppress_bind_attrs = true,
    }
    };
    module_platform_driver(exynos_sata_phy_driver);
    MODULE_DESCRIPTION("Samsung SerDes PHY driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Girish K S <ks.giri@samsung.com>");
    MODULE_AUTHOR("Yuvaraj C D <yuvaraj.cd@samsung.com>");
