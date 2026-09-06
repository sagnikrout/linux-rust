//! Automatically rewritten from C to Rust
//! Source: drivers/phy/freescale/phy-fsl-imx8m-pcie.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2021 NXP
//

pub const IMX8MM_PCIE_PHY_CMN_REG061: c_uint = 0x184;

pub const IMX8MM_PCIE_PHY_CMN_REG062: c_uint = 0x188;

pub const IMX8MM_PCIE_PHY_CMN_REG063: c_uint = 0x18C;

pub const IMX8MM_PCIE_PHY_CMN_REG064: c_uint = 0x190;

pub const IMX8MM_PCIE_PHY_CMN_REG065: c_uint = 0x194;

pub const IMX8MM_PCIE_PHY_CMN_REG075: c_uint = 0x1D4;
pub const ANA_PLL_DONE: c_uint = 0x3;
pub const PCIE_PHY_TRSV_REG5: c_uint = 0x414;
pub const PCIE_PHY_TRSV_REG6: c_uint = 0x418;

    enum imx8_pcie_phy_type {
    IMX8MM,
    IMX8MP,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8_pcie_phy_drvdata {
    pub gpr: *const c_char,
    pub variant: enum imx8_pcie_phy_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8_pcie_phy {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub phy: *mut phy,
    pub iomuxc_gpr: *mut regmap,
    pub perst: *mut reset_control,
    pub reset: *mut reset_control,
    pub refclk_pad_mode: u32,
    pub tx_deemph_gen1: u32,
    pub tx_deemph_gen2: u32,
    pub clkreq_unused: bool,
    pub drvdata: *const imx8_pcie_phy_drvdata,
}

#[no_mangle]
unsafe extern "C" fn imx8_pcie_phy_power_on(phy: *mut phy) -> c_int {
    static int imx8_pcie_phy_power_on(struct phy *phy)
    {
    int ret;
    u32 val, pad_mode;
    struct imx8_pcie_phy *imx8_phy = phy_get_drvdata(phy);
    pad_mode = imx8_phy.refclk_pad_mode;
    switch (imx8_phy.drvdata.variant) {
    case IMX8MM:
    reset_control_assert(imx8_phy.reset);
// Tune PHY de-emphasis setting to pass PCIe compliance.
    if (imx8_phy.tx_deemph_gen1)
    writel(imx8_phy.tx_deemph_gen1,
    imx8_phy.base + PCIE_PHY_TRSV_REG5);
    if (imx8_phy.tx_deemph_gen2)
    writel(imx8_phy.tx_deemph_gen2,
    imx8_phy.base + PCIE_PHY_TRSV_REG6);
    break;
    case IMX8MP:
    reset_control_assert(imx8_phy.reset);
    break;
    }
    if (pad_mode == IMX8_PCIE_REFCLK_PAD_INPUT ||
    pad_mode == IMX8_PCIE_REFCLK_PAD_UNUSED) {
// Configure the pad as input
    val = readl(imx8_phy.base + IMX8MM_PCIE_PHY_CMN_REG061);
    writel(val & ~ANA_PLL_CLK_OUT_TO_EXT_IO_EN,
    imx8_phy.base + IMX8MM_PCIE_PHY_CMN_REG061);
    } else {
// Configure the PHY to output the refclock via pad
    writel(ANA_PLL_CLK_OUT_TO_EXT_IO_EN,
    imx8_phy.base + IMX8MM_PCIE_PHY_CMN_REG061);
    }
    if (pad_mode == IMX8_PCIE_REFCLK_PAD_OUTPUT ||
    pad_mode == IMX8_PCIE_REFCLK_PAD_UNUSED) {
// Source clock from SoC internal PLL
    writel(ANA_PLL_CLK_OUT_TO_EXT_IO_SEL,
    imx8_phy.base + IMX8MM_PCIE_PHY_CMN_REG062);
    if (imx8_phy.drvdata.variant != IMX8MM) {
    writel(AUX_PLL_REFCLK_SEL_SYS_PLL,
    imx8_phy.base + IMX8MM_PCIE_PHY_CMN_REG063);
    }
    val = ANA_AUX_RX_TX_SEL_TX | ANA_AUX_TX_TERM;
    writel(val | ANA_AUX_RX_TERM_GND_EN,
    imx8_phy.base + IMX8MM_PCIE_PHY_CMN_REG064);
    writel(ANA_AUX_RX_TERM | ANA_AUX_TX_LVL,
    imx8_phy.base + IMX8MM_PCIE_PHY_CMN_REG065);
    }
// Set AUX_EN_OVERRIDE 1'b0, when the CLKREQ# isn't hooked
    regmap_update_bits(imx8_phy.iomuxc_gpr, IOMUXC_GPR14,
    IMX8MM_GPR_PCIE_AUX_EN_OVERRIDE,
    imx8_phy.clkreq_unused ?
    0 : IMX8MM_GPR_PCIE_AUX_EN_OVERRIDE);
    regmap_update_bits(imx8_phy.iomuxc_gpr, IOMUXC_GPR14,
    IMX8MM_GPR_PCIE_AUX_EN,
    IMX8MM_GPR_PCIE_AUX_EN);
    regmap_update_bits(imx8_phy.iomuxc_gpr, IOMUXC_GPR14,
    IMX8MM_GPR_PCIE_POWER_OFF, 0);
    regmap_update_bits(imx8_phy.iomuxc_gpr, IOMUXC_GPR14,
    IMX8MM_GPR_PCIE_SSC_EN, 0);
    regmap_update_bits(imx8_phy.iomuxc_gpr, IOMUXC_GPR14,
    IMX8MM_GPR_PCIE_REF_CLK_SEL,
    pad_mode == IMX8_PCIE_REFCLK_PAD_INPUT ?
    IMX8MM_GPR_PCIE_REF_CLK_EXT :
    IMX8MM_GPR_PCIE_REF_CLK_PLL);
    usleep_range(100, 200);
    reset_control_deassert(imx8_phy.perst);
    reset_control_deassert(imx8_phy.reset);
    usleep_range(200, 500);
// Do the PHY common block reset
    regmap_update_bits(imx8_phy.iomuxc_gpr, IOMUXC_GPR14,
    IMX8MM_GPR_PCIE_CMN_RST,
    IMX8MM_GPR_PCIE_CMN_RST);
// Polling to check the phy is ready or not.
    ret = readl_poll_timeout(imx8_phy.base + IMX8MM_PCIE_PHY_CMN_REG075,
    val, val == ANA_PLL_DONE, 10, 20000);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx8_pcie_phy_power_off(phy: *mut phy) -> c_int {
    static int imx8_pcie_phy_power_off(struct phy *phy)
    {
    struct imx8_pcie_phy *imx8_phy = phy_get_drvdata(phy);
    reset_control_assert(imx8_phy.reset);
    reset_control_assert(imx8_phy.perst);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8_pcie_phy_init(phy: *mut phy) -> c_int {
    static int imx8_pcie_phy_init(struct phy *phy)
    {
    struct imx8_pcie_phy *imx8_phy = phy_get_drvdata(phy);
    return clk_prepare_enable(imx8_phy.clk);
    }
#[no_mangle]
unsafe extern "C" fn imx8_pcie_phy_exit(phy: *mut phy) -> c_int {
    static int imx8_pcie_phy_exit(struct phy *phy)
    {
    struct imx8_pcie_phy *imx8_phy = phy_get_drvdata(phy);
    clk_disable_unprepare(imx8_phy.clk);
    return 0;
    }
    static const struct phy_ops imx8_pcie_phy_ops = {
    .init		= imx8_pcie_phy_init,
    .exit		= imx8_pcie_phy_exit,
    .power_on	= imx8_pcie_phy_power_on,
    .power_off	= imx8_pcie_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static const struct imx8_pcie_phy_drvdata imx8mm_drvdata = {
    .gpr = "fsl,imx8mm-iomuxc-gpr",
    .variant = IMX8MM,
    };
    static const struct imx8_pcie_phy_drvdata imx8mp_drvdata = {
    .gpr = "fsl,imx8mp-iomuxc-gpr",
    .variant = IMX8MP,
    };
    static const struct of_device_id imx8_pcie_phy_of_match[] = {
    {.compatible = "fsl,imx8mm-pcie-phy", .data = &imx8mm_drvdata, },
    {.compatible = "fsl,imx8mp-pcie-phy", .data = &imx8mp_drvdata, },
    { },
    };
    MODULE_DEVICE_TABLE(of, imx8_pcie_phy_of_match);
#[no_mangle]
unsafe extern "C" fn imx8_pcie_phy_probe(pdev: *mut platform_device) -> c_int {
    static int imx8_pcie_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct imx8_pcie_phy *imx8_phy;
    imx8_phy = devm_kzalloc(dev, sizeof(*imx8_phy), GFP_KERNEL);
    if (!imx8_phy)
    return -ENOMEM;
    imx8_phy.drvdata = of_device_get_match_data(dev);
// get PHY refclk pad mode
    of_property_read_u32(np, "fsl,refclk-pad-mode",
    &imx8_phy.refclk_pad_mode);
    if (of_property_read_u32(np, "fsl,tx-deemph-gen1",
    &imx8_phy.tx_deemph_gen1))
    imx8_phy.tx_deemph_gen1 = 0;
    if (of_property_read_u32(np, "fsl,tx-deemph-gen2",
    &imx8_phy.tx_deemph_gen2))
    imx8_phy.tx_deemph_gen2 = 0;
    if (of_property_read_bool(np, "fsl,clkreq-unsupported"))
    imx8_phy.clkreq_unused = true;
    else
    imx8_phy.clkreq_unused = false;
    imx8_phy.clk = devm_clk_get(dev, "ref");
    if (IS_ERR(imx8_phy.clk))
    return dev_err_probe(dev, PTR_ERR(imx8_phy.clk),
    "failed to get imx pcie phy clock\n");
// Grab GPR config register range
    imx8_phy.iomuxc_gpr =
    syscon_regmap_lookup_by_compatible(imx8_phy.drvdata.gpr);
    if (IS_ERR(imx8_phy.iomuxc_gpr))
    return dev_err_probe(dev, PTR_ERR(imx8_phy.iomuxc_gpr),
    "unable to find iomuxc registers\n");
    imx8_phy.reset = devm_reset_control_get_exclusive(dev, "pciephy");
    if (IS_ERR(imx8_phy.reset))
    return dev_err_probe(dev, PTR_ERR(imx8_phy.reset),
    "Failed to get PCIEPHY reset control\n");
    if (imx8_phy.drvdata.variant == IMX8MP) {
    imx8_phy.perst =
    devm_reset_control_get_exclusive(dev, "perst");
    if (IS_ERR(imx8_phy.perst))
    return dev_err_probe(dev, PTR_ERR(imx8_phy.perst),
    "Failed to get PCIE PHY PERST control\n");
    }
    imx8_phy.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(imx8_phy.base))
    return PTR_ERR(imx8_phy.base);
    imx8_phy.phy = devm_phy_create(dev, core::ptr::null_mut(), &imx8_pcie_phy_ops);
    if (IS_ERR(imx8_phy.phy))
    return PTR_ERR(imx8_phy.phy);
    phy_set_drvdata(imx8_phy.phy, imx8_phy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static struct platform_driver imx8_pcie_phy_driver = {
    .probe	= imx8_pcie_phy_probe,
    .driver = {
    .name	= "imx8-pcie-phy",
    .of_match_table	= imx8_pcie_phy_of_match,
    }
    };
    module_platform_driver(imx8_pcie_phy_driver);
    MODULE_DESCRIPTION("FSL IMX8 PCIE PHY driver");
    MODULE_LICENSE("GPL v2");
