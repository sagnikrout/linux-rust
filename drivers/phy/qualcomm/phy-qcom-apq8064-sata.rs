//! Automatically rewritten from C to Rust
//! Source: drivers/phy/qualcomm/phy-qcom-apq8064-sata.c
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
// Copyright (c) 2014, The Linux Foundation. All rights reserved.
//

// PHY registers
pub const UNIPHY_PLL_REFCLK_CFG: c_uint = 0x000;
pub const UNIPHY_PLL_PWRGEN_CFG: c_uint = 0x014;
pub const UNIPHY_PLL_GLB_CFG: c_uint = 0x020;
pub const UNIPHY_PLL_SDM_CFG0: c_uint = 0x038;
pub const UNIPHY_PLL_SDM_CFG1: c_uint = 0x03C;
pub const UNIPHY_PLL_SDM_CFG2: c_uint = 0x040;
pub const UNIPHY_PLL_SDM_CFG3: c_uint = 0x044;
pub const UNIPHY_PLL_SDM_CFG4: c_uint = 0x048;
pub const UNIPHY_PLL_SSC_CFG0: c_uint = 0x04C;
pub const UNIPHY_PLL_SSC_CFG1: c_uint = 0x050;
pub const UNIPHY_PLL_SSC_CFG2: c_uint = 0x054;
pub const UNIPHY_PLL_SSC_CFG3: c_uint = 0x058;
pub const UNIPHY_PLL_LKDET_CFG0: c_uint = 0x05C;
pub const UNIPHY_PLL_LKDET_CFG1: c_uint = 0x060;
pub const UNIPHY_PLL_LKDET_CFG2: c_uint = 0x064;
pub const UNIPHY_PLL_CAL_CFG0: c_uint = 0x06C;
pub const UNIPHY_PLL_CAL_CFG8: c_uint = 0x08C;
pub const UNIPHY_PLL_CAL_CFG9: c_uint = 0x090;
pub const UNIPHY_PLL_CAL_CFG10: c_uint = 0x094;
pub const UNIPHY_PLL_CAL_CFG11: c_uint = 0x098;
pub const UNIPHY_PLL_STATUS: c_uint = 0x0C0;
pub const SATA_PHY_SER_CTRL: c_uint = 0x100;
pub const SATA_PHY_TX_DRIV_CTRL0: c_uint = 0x104;
pub const SATA_PHY_TX_DRIV_CTRL1: c_uint = 0x108;
pub const SATA_PHY_TX_IMCAL0: c_uint = 0x11C;
pub const SATA_PHY_TX_IMCAL2: c_uint = 0x124;
pub const SATA_PHY_RX_IMCAL0: c_uint = 0x128;
pub const SATA_PHY_EQUAL: c_uint = 0x13C;
pub const SATA_PHY_OOB_TERM: c_uint = 0x144;
pub const SATA_PHY_CDR_CTRL0: c_uint = 0x148;
pub const SATA_PHY_CDR_CTRL1: c_uint = 0x14C;
pub const SATA_PHY_CDR_CTRL2: c_uint = 0x150;
pub const SATA_PHY_CDR_CTRL3: c_uint = 0x154;
pub const SATA_PHY_PI_CTRL0: c_uint = 0x168;
pub const SATA_PHY_POW_DWN_CTRL0: c_uint = 0x180;
pub const SATA_PHY_POW_DWN_CTRL1: c_uint = 0x184;
pub const SATA_PHY_TX_DATA_CTRL: c_uint = 0x188;
pub const SATA_PHY_ALIGNP: c_uint = 0x1A4;
pub const SATA_PHY_TX_IMCAL_STAT: c_uint = 0x1E4;
pub const SATA_PHY_RX_IMCAL_STAT: c_uint = 0x1E8;

// default timeout set to 1 sec
pub const TIMEOUT_MS: c_int = 10000;
pub const DELAY_INTERVAL_US: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_apq8064_sata_phy {
    pub mmio: *mut void __iomem,
    pub cfg_clk: *mut clk,
    pub dev: *mut device,
}

// Helper function to do poll and timeout
#[no_mangle]
unsafe extern "C" fn poll_timeout(addr: *mut void __iomem, mask: u32) -> c_int {
    static int poll_timeout(void __iomem *addr, u32 mask)
    {
    u32 val;
    return readl_relaxed_poll_timeout(addr, val, (val & mask),
    DELAY_INTERVAL_US, TIMEOUT_MS * 1000);
    }
#[no_mangle]
unsafe extern "C" fn qcom_apq8064_sata_phy_init(generic_phy: *mut phy) -> c_int {
    static int qcom_apq8064_sata_phy_init(struct phy *generic_phy)
    {
    struct qcom_apq8064_sata_phy *phy = phy_get_drvdata(generic_phy);
    void __iomem *base = phy.mmio;
    let mut ret: c_int = 0;
// SATA phy initialization
    writel_relaxed(0x01, base + SATA_PHY_SER_CTRL);
    writel_relaxed(0xB1, base + SATA_PHY_POW_DWN_CTRL0);
// Make sure the power down happens before power up
    mb();
    usleep_range(10, 60);
    writel_relaxed(0x01, base + SATA_PHY_POW_DWN_CTRL0);
    writel_relaxed(0x3E, base + SATA_PHY_POW_DWN_CTRL1);
    writel_relaxed(0x01, base + SATA_PHY_RX_IMCAL0);
    writel_relaxed(0x01, base + SATA_PHY_TX_IMCAL0);
    writel_relaxed(0x02, base + SATA_PHY_TX_IMCAL2);
// Write UNIPHYPLL registers to configure PLL
    writel_relaxed(0x04, base + UNIPHY_PLL_REFCLK_CFG);
    writel_relaxed(0x00, base + UNIPHY_PLL_PWRGEN_CFG);
    writel_relaxed(0x0A, base + UNIPHY_PLL_CAL_CFG0);
    writel_relaxed(0xF3, base + UNIPHY_PLL_CAL_CFG8);
    writel_relaxed(0x01, base + UNIPHY_PLL_CAL_CFG9);
    writel_relaxed(0xED, base + UNIPHY_PLL_CAL_CFG10);
    writel_relaxed(0x02, base + UNIPHY_PLL_CAL_CFG11);
    writel_relaxed(0x36, base + UNIPHY_PLL_SDM_CFG0);
    writel_relaxed(0x0D, base + UNIPHY_PLL_SDM_CFG1);
    writel_relaxed(0xA3, base + UNIPHY_PLL_SDM_CFG2);
    writel_relaxed(0xF0, base + UNIPHY_PLL_SDM_CFG3);
    writel_relaxed(0x00, base + UNIPHY_PLL_SDM_CFG4);
    writel_relaxed(0x19, base + UNIPHY_PLL_SSC_CFG0);
    writel_relaxed(0xE1, base + UNIPHY_PLL_SSC_CFG1);
    writel_relaxed(0x00, base + UNIPHY_PLL_SSC_CFG2);
    writel_relaxed(0x11, base + UNIPHY_PLL_SSC_CFG3);
    writel_relaxed(0x04, base + UNIPHY_PLL_LKDET_CFG0);
    writel_relaxed(0xFF, base + UNIPHY_PLL_LKDET_CFG1);
    writel_relaxed(0x02, base + UNIPHY_PLL_GLB_CFG);
// make sure global config LDO power down happens before power up
    mb();
    writel_relaxed(0x03, base + UNIPHY_PLL_GLB_CFG);
    writel_relaxed(0x05, base + UNIPHY_PLL_LKDET_CFG2);
// PLL Lock wait
    ret = poll_timeout(base + UNIPHY_PLL_STATUS, UNIPHY_PLL_LOCK);
    if (ret) {
    dev_err(phy.dev, "poll timeout UNIPHY_PLL_STATUS\n");
    return ret;
    }
// TX Calibration
    ret = poll_timeout(base + SATA_PHY_TX_IMCAL_STAT, SATA_PHY_TX_CAL);
    if (ret) {
    dev_err(phy.dev, "poll timeout SATA_PHY_TX_IMCAL_STAT\n");
    return ret;
    }
// RX Calibration
    ret = poll_timeout(base + SATA_PHY_RX_IMCAL_STAT, SATA_PHY_RX_CAL);
    if (ret) {
    dev_err(phy.dev, "poll timeout SATA_PHY_RX_IMCAL_STAT\n");
    return ret;
    }
// SATA phy calibrated successfully, power up to functional mode
    writel_relaxed(0x3E, base + SATA_PHY_POW_DWN_CTRL1);
    writel_relaxed(0x01, base + SATA_PHY_RX_IMCAL0);
    writel_relaxed(0x01, base + SATA_PHY_TX_IMCAL0);
    writel_relaxed(0x00, base + SATA_PHY_POW_DWN_CTRL1);
    writel_relaxed(0x59, base + SATA_PHY_CDR_CTRL0);
    writel_relaxed(0x04, base + SATA_PHY_CDR_CTRL1);
    writel_relaxed(0x00, base + SATA_PHY_CDR_CTRL2);
    writel_relaxed(0x00, base + SATA_PHY_PI_CTRL0);
    writel_relaxed(0x00, base + SATA_PHY_CDR_CTRL3);
    writel_relaxed(0x01, base + SATA_PHY_POW_DWN_CTRL0);
    writel_relaxed(0x11, base + SATA_PHY_TX_DATA_CTRL);
    writel_relaxed(0x43, base + SATA_PHY_ALIGNP);
    writel_relaxed(0x04, base + SATA_PHY_OOB_TERM);
    writel_relaxed(0x01, base + SATA_PHY_EQUAL);
    writel_relaxed(0x09, base + SATA_PHY_TX_DRIV_CTRL0);
    writel_relaxed(0x09, base + SATA_PHY_TX_DRIV_CTRL1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_apq8064_sata_phy_exit(generic_phy: *mut phy) -> c_int {
    static int qcom_apq8064_sata_phy_exit(struct phy *generic_phy)
    {
    struct qcom_apq8064_sata_phy *phy = phy_get_drvdata(generic_phy);
    void __iomem *base = phy.mmio;
// Power down PHY
    writel_relaxed(0xF8, base + SATA_PHY_POW_DWN_CTRL0);
    writel_relaxed(0xFE, base + SATA_PHY_POW_DWN_CTRL1);
// Power down PLL block
    writel_relaxed(0x00, base + UNIPHY_PLL_GLB_CFG);
    return 0;
    }
    static const struct phy_ops qcom_apq8064_sata_phy_ops = {
    .init		= qcom_apq8064_sata_phy_init,
    .exit		= qcom_apq8064_sata_phy_exit,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn qcom_apq8064_sata_phy_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_apq8064_sata_phy_probe(struct platform_device *pdev)
    {
    struct qcom_apq8064_sata_phy *phy;
    struct device *dev = &pdev.dev;
    struct phy_provider *phy_provider;
    struct phy *generic_phy;
    int ret;
    phy = devm_kzalloc(dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.mmio = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(phy.mmio))
    return PTR_ERR(phy.mmio);
    generic_phy = devm_phy_create(dev, core::ptr::null_mut(), &qcom_apq8064_sata_phy_ops);
    if (IS_ERR(generic_phy)) {
    dev_err(dev, "%s: failed to create phy\n", __func__);
    return PTR_ERR(generic_phy);
    }
    phy.dev = dev;
    phy_set_drvdata(generic_phy, phy);
    platform_set_drvdata(pdev, phy);
    phy.cfg_clk = devm_clk_get(dev, "cfg");
    if (IS_ERR(phy.cfg_clk)) {
    dev_err(dev, "Failed to get sata cfg clock\n");
    return PTR_ERR(phy.cfg_clk);
    }
    ret = clk_prepare_enable(phy.cfg_clk);
    if (ret)
    return ret;
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider)) {
    clk_disable_unprepare(phy.cfg_clk);
    dev_err(dev, "%s: failed to register phy\n", __func__);
    return PTR_ERR(phy_provider);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_apq8064_sata_phy_remove(pdev: *mut platform_device) {
    static void qcom_apq8064_sata_phy_remove(struct platform_device *pdev)
    {
    struct qcom_apq8064_sata_phy *phy = platform_get_drvdata(pdev);
    clk_disable_unprepare(phy.cfg_clk);
    }
    static const struct of_device_id qcom_apq8064_sata_phy_of_match[] = {
    { .compatible = "qcom,apq8064-sata-phy" },
    { },
    };
    MODULE_DEVICE_TABLE(of, qcom_apq8064_sata_phy_of_match);
    static struct platform_driver qcom_apq8064_sata_phy_driver = {
    .probe = qcom_apq8064_sata_phy_probe,
    .remove = qcom_apq8064_sata_phy_remove,
    .driver = {
    .name = "qcom-apq8064-sata-phy",
    .of_match_table = qcom_apq8064_sata_phy_of_match,
    },
    };
    module_platform_driver(qcom_apq8064_sata_phy_driver);
    MODULE_DESCRIPTION("QCOM apq8064 SATA PHY driver");
    MODULE_LICENSE("GPL v2");
