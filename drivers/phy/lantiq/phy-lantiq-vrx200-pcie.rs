//! Automatically rewritten from C to Rust
//! Source: drivers/phy/lantiq/phy-lantiq-vrx200-pcie.c
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
// PCIe PHY driver for Lantiq VRX200 and ARX300 SoCs.
//
// Copyright (C) 2019 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
//
// Based on the BSP (called "UGW") driver:
// Copyright (C) 2009-2015 Lei Chuanhua <chuanhua.lei@lantiq.com>
// Copyright (C) 2016 Intel Corporation
//
// TODO: PHY modes other than 36MHz (without "SSC")
//

pub const PCIE_PHY_PLL_CTRL1: c_uint = 0x44;
pub const PCIE_PHY_PLL_CTRL2: c_uint = 0x46;

pub const PCIE_PHY_PLL_CTRL3: c_uint = 0x48;

pub const PCIE_PHY_PLL_CTRL4: c_uint = 0x4a;
pub const PCIE_PHY_PLL_CTRL5: c_uint = 0x4c;
pub const PCIE_PHY_PLL_CTRL6: c_uint = 0x4e;
pub const PCIE_PHY_PLL_CTRL7: c_uint = 0x50;
pub const PCIE_PHY_PLL_A_CTRL1: c_uint = 0x52;
pub const PCIE_PHY_PLL_A_CTRL2: c_uint = 0x54;

pub const PCIE_PHY_PLL_A_CTRL3: c_uint = 0x56;

pub const PCIE_PHY_PLL_STATUS: c_uint = 0x58;
pub const PCIE_PHY_TX1_CTRL1: c_uint = 0x60;

pub const PCIE_PHY_TX1_CTRL2: c_uint = 0x62;
pub const PCIE_PHY_TX1_CTRL3: c_uint = 0x64;
pub const PCIE_PHY_TX1_A_CTRL1: c_uint = 0x66;
pub const PCIE_PHY_TX1_A_CTRL2: c_uint = 0x68;
pub const PCIE_PHY_TX1_MOD1: c_uint = 0x6a;
pub const PCIE_PHY_TX1_MOD2: c_uint = 0x6c;
pub const PCIE_PHY_TX1_MOD3: c_uint = 0x6e;
pub const PCIE_PHY_TX2_CTRL1: c_uint = 0x70;

pub const PCIE_PHY_TX2_CTRL2: c_uint = 0x72;
pub const PCIE_PHY_TX2_A_CTRL1: c_uint = 0x76;
pub const PCIE_PHY_TX2_A_CTRL2: c_uint = 0x78;
pub const PCIE_PHY_TX2_MOD1: c_uint = 0x7a;
pub const PCIE_PHY_TX2_MOD2: c_uint = 0x7c;
pub const PCIE_PHY_TX2_MOD3: c_uint = 0x7e;
pub const PCIE_PHY_RX1_CTRL1: c_uint = 0xa0;

pub const PCIE_PHY_RX1_CTRL2: c_uint = 0xa2;
pub const PCIE_PHY_RX1_CDR: c_uint = 0xa4;
pub const PCIE_PHY_RX1_EI: c_uint = 0xa6;
pub const PCIE_PHY_RX1_A_CTRL: c_uint = 0xaa;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltq_vrx200_pcie_phy_priv {
    pub phy: *mut phy,
    pub mode: c_uint,
    pub dev: *mut device,
    pub phy_regmap: *mut regmap,
    pub rcu_regmap: *mut regmap,
    pub pdi_clk: *mut clk,
    pub phy_clk: *mut clk,
    pub phy_reset: *mut reset_control,
    pub pcie_reset: *mut reset_control,
    pub rcu_ahb_endian_offset: u32,
    pub rcu_ahb_endian_big_endian_mask: u32,
}

#[no_mangle]
unsafe extern "C" fn ltq_vrx200_pcie_phy_common_setup(phy: *mut phy) {
    static void ltq_vrx200_pcie_phy_common_setup(struct phy *phy)
    {
    struct ltq_vrx200_pcie_phy_priv *priv = phy_get_drvdata(phy);
// PLL Setting
    regmap_write(priv.phy_regmap, PCIE_PHY_PLL_A_CTRL1, 0x120e);
// increase the bias reference voltage
    regmap_write(priv.phy_regmap, PCIE_PHY_PLL_A_CTRL2, 0x39d7);
    regmap_write(priv.phy_regmap, PCIE_PHY_PLL_A_CTRL3, 0x0900);
// Endcnt
    regmap_write(priv.phy_regmap, PCIE_PHY_RX1_EI, 0x0004);
    regmap_write(priv.phy_regmap, PCIE_PHY_RX1_A_CTRL, 0x6803);
    regmap_update_bits(priv.phy_regmap, PCIE_PHY_TX1_CTRL1,
    PCIE_PHY_TX1_CTRL1_FORCE_EN,
    PCIE_PHY_TX1_CTRL1_FORCE_EN);
// predrv_ser_en
    regmap_write(priv.phy_regmap, PCIE_PHY_TX1_A_CTRL2, 0x0706);
// ctrl_lim
    regmap_write(priv.phy_regmap, PCIE_PHY_TX1_CTRL3, 0x1fff);
// ctrl
    regmap_write(priv.phy_regmap, PCIE_PHY_TX1_A_CTRL1, 0x0810);
// predrv_ser_en
    regmap_update_bits(priv.phy_regmap, PCIE_PHY_TX2_A_CTRL2, 0x7f00,
    0x4700);
// RTERM
    regmap_write(priv.phy_regmap, PCIE_PHY_TX1_CTRL2, 0x2e00);
// Improved 100MHz clock output
    regmap_write(priv.phy_regmap, PCIE_PHY_TX2_CTRL2, 0x3096);
    regmap_write(priv.phy_regmap, PCIE_PHY_TX2_A_CTRL2, 0x4707);
// Reduced CDR BW to avoid glitches
    regmap_write(priv.phy_regmap, PCIE_PHY_RX1_CDR, 0x0235);
    }
#[no_mangle]
unsafe extern "C" fn pcie_phy_36mhz_mode_setup(phy: *mut phy) {
    static void pcie_phy_36mhz_mode_setup(struct phy *phy)
    {
    struct ltq_vrx200_pcie_phy_priv *priv = phy_get_drvdata(phy);
    regmap_update_bits(priv.phy_regmap, PCIE_PHY_PLL_CTRL3,
    PCIE_PHY_PLL_CTRL3_EXT_MMD_DIV_RATIO_EN, 0x0000);
    regmap_update_bits(priv.phy_regmap, PCIE_PHY_PLL_CTRL3,
    PCIE_PHY_PLL_CTRL3_EXT_MMD_DIV_RATIO_MASK, 0x0000);
    regmap_update_bits(priv.phy_regmap, PCIE_PHY_PLL_CTRL2,
    PCIE_PHY_PLL_CTRL2_PLL_SDM_EN,
    PCIE_PHY_PLL_CTRL2_PLL_SDM_EN);
    regmap_update_bits(priv.phy_regmap, PCIE_PHY_PLL_CTRL2,
    PCIE_PHY_PLL_CTRL2_CONST_SDM_EN,
    PCIE_PHY_PLL_CTRL2_CONST_SDM_EN);
    regmap_update_bits(priv.phy_regmap, PCIE_PHY_PLL_A_CTRL3,
    PCIE_PHY_PLL_A_CTRL3_MMD_MASK,
    FIELD_PREP(PCIE_PHY_PLL_A_CTRL3_MMD_MASK, 0x1));
    regmap_update_bits(priv.phy_regmap, PCIE_PHY_PLL_A_CTRL2,
    PCIE_PHY_PLL_A_CTRL2_LF_MODE_EN, 0x0000);
// const_sdm
    regmap_write(priv.phy_regmap, PCIE_PHY_PLL_CTRL1, 0x38e4);
    regmap_update_bits(priv.phy_regmap, PCIE_PHY_PLL_CTRL2,
    PCIE_PHY_PLL_CTRL2_CONST_SDM_MASK,
    FIELD_PREP(PCIE_PHY_PLL_CTRL2_CONST_SDM_MASK,
    0xee));
// pllmod
    regmap_write(priv.phy_regmap, PCIE_PHY_PLL_CTRL7, 0x0002);
    regmap_write(priv.phy_regmap, PCIE_PHY_PLL_CTRL6, 0x3a04);
    regmap_write(priv.phy_regmap, PCIE_PHY_PLL_CTRL5, 0xfae3);
    regmap_write(priv.phy_regmap, PCIE_PHY_PLL_CTRL4, 0x1b72);
    }
#[no_mangle]
unsafe extern "C" fn ltq_vrx200_pcie_phy_wait_for_pll(phy: *mut phy) -> c_int {
    static int ltq_vrx200_pcie_phy_wait_for_pll(struct phy *phy)
    {
    struct ltq_vrx200_pcie_phy_priv *priv = phy_get_drvdata(phy);
    unsigned int tmp;
    int ret;
    ret = regmap_read_poll_timeout(priv.phy_regmap, PCIE_PHY_PLL_STATUS,
    tmp, ((tmp & 0x0070) == 0x0070), 10,
    10000);
    if (ret) {
    dev_err(priv.dev, "PLL Link timeout, PLL status = 0x%04x\n",
    tmp);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltq_vrx200_pcie_phy_apply_workarounds(phy: *mut phy) {
    static void ltq_vrx200_pcie_phy_apply_workarounds(struct phy *phy)
    {
    struct ltq_vrx200_pcie_phy_priv *priv = phy_get_drvdata(phy);
    static const struct reg_default slices[] =  {
    {
    .reg = PCIE_PHY_TX1_CTRL1,
    .def = PCIE_PHY_TX1_CTRL1_LOAD_EN,
    },
    {
    .reg = PCIE_PHY_TX2_CTRL1,
    .def = PCIE_PHY_TX2_CTRL1_LOAD_EN,
    },
    {
    .reg = PCIE_PHY_RX1_CTRL1,
    .def = PCIE_PHY_RX1_CTRL1_LOAD_EN,
    }
    };
    int i;
    for (i = 0; i < ARRAY_SIZE(slices); i++) {
// enable load_en
    regmap_update_bits(priv.phy_regmap, slices[i].reg,
    slices[i].def, slices[i].def);
    udelay(1);
// disable load_en
    regmap_update_bits(priv.phy_regmap, slices[i].reg,
    slices[i].def, 0x0);
    }
    for (i = 0; i < 5; i++) {
// TX2 modulation
    regmap_write(priv.phy_regmap, PCIE_PHY_TX2_MOD1, 0x1ffe);
    regmap_write(priv.phy_regmap, PCIE_PHY_TX2_MOD2, 0xfffe);
    regmap_write(priv.phy_regmap, PCIE_PHY_TX2_MOD3, 0x0601);
    usleep_range(1000, 2000);
    regmap_write(priv.phy_regmap, PCIE_PHY_TX2_MOD3, 0x0001);
// TX1 modulation
    regmap_write(priv.phy_regmap, PCIE_PHY_TX1_MOD1, 0x1ffe);
    regmap_write(priv.phy_regmap, PCIE_PHY_TX1_MOD2, 0xfffe);
    regmap_write(priv.phy_regmap, PCIE_PHY_TX1_MOD3, 0x0601);
    usleep_range(1000, 2000);
    regmap_write(priv.phy_regmap, PCIE_PHY_TX1_MOD3, 0x0001);
    }
    }
#[no_mangle]
unsafe extern "C" fn ltq_vrx200_pcie_phy_init(phy: *mut phy) -> c_int {
    static int ltq_vrx200_pcie_phy_init(struct phy *phy)
    {
    struct ltq_vrx200_pcie_phy_priv *priv = phy_get_drvdata(phy);
    int ret;
    if (of_device_is_big_endian(priv.dev.of_node))
    regmap_update_bits(priv.rcu_regmap,
    priv.rcu_ahb_endian_offset,
    priv.rcu_ahb_endian_big_endian_mask,
    priv.rcu_ahb_endian_big_endian_mask);
    else
    regmap_update_bits(priv.rcu_regmap,
    priv.rcu_ahb_endian_offset,
    priv.rcu_ahb_endian_big_endian_mask, 0x0);
    ret = reset_control_assert(priv.phy_reset);
    if (ret)
    goto err;
    udelay(1);
    ret = reset_control_deassert(priv.phy_reset);
    if (ret)
    goto err;
    udelay(1);
    ret = reset_control_deassert(priv.pcie_reset);
    if (ret)
    goto err_assert_phy_reset;
// Make sure PHY PLL is stable
    usleep_range(20, 40);
    return 0;
    err_assert_phy_reset:
    reset_control_assert(priv.phy_reset);
    err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ltq_vrx200_pcie_phy_exit(phy: *mut phy) -> c_int {
    static int ltq_vrx200_pcie_phy_exit(struct phy *phy)
    {
    struct ltq_vrx200_pcie_phy_priv *priv = phy_get_drvdata(phy);
    int ret;
    ret = reset_control_assert(priv.pcie_reset);
    if (ret)
    return ret;
    ret = reset_control_assert(priv.phy_reset);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltq_vrx200_pcie_phy_power_on(phy: *mut phy) -> c_int {
    static int ltq_vrx200_pcie_phy_power_on(struct phy *phy)
    {
    struct ltq_vrx200_pcie_phy_priv *priv = phy_get_drvdata(phy);
    int ret;
// Enable PDI to access PCIe PHY register
    ret = clk_prepare_enable(priv.pdi_clk);
    if (ret)
    goto err;
// Configure PLL and PHY clock
    ltq_vrx200_pcie_phy_common_setup(phy);
    pcie_phy_36mhz_mode_setup(phy);
// Enable the PCIe PHY and make PLL setting take effect
    ret = clk_prepare_enable(priv.phy_clk);
    if (ret)
    goto err_disable_pdi_clk;
// Check if we are in "startup ready" status
    ret = ltq_vrx200_pcie_phy_wait_for_pll(phy);
    if (ret)
    goto err_disable_phy_clk;
    ltq_vrx200_pcie_phy_apply_workarounds(phy);
    return 0;
    err_disable_phy_clk:
    clk_disable_unprepare(priv.phy_clk);
    err_disable_pdi_clk:
    clk_disable_unprepare(priv.pdi_clk);
    err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ltq_vrx200_pcie_phy_power_off(phy: *mut phy) -> c_int {
    static int ltq_vrx200_pcie_phy_power_off(struct phy *phy)
    {
    struct ltq_vrx200_pcie_phy_priv *priv = phy_get_drvdata(phy);
    clk_disable_unprepare(priv.phy_clk);
    clk_disable_unprepare(priv.pdi_clk);
    return 0;
    }
    static const struct phy_ops ltq_vrx200_pcie_phy_ops = {
    .init		= ltq_vrx200_pcie_phy_init,
    .exit		= ltq_vrx200_pcie_phy_exit,
    .power_on	= ltq_vrx200_pcie_phy_power_on,
    .power_off	= ltq_vrx200_pcie_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static struct phy *ltq_vrx200_pcie_phy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct ltq_vrx200_pcie_phy_priv *priv = dev_get_drvdata(dev);
    unsigned int mode;
    if (args.args_count != 1) {
    dev_err(dev, "invalid number of arguments\n");
    return ERR_PTR(-EINVAL);
    }
    mode = args.args[0];
    switch (mode) {
    case LANTIQ_PCIE_PHY_MODE_36MHZ:
    priv.mode = mode;
    break;
    case LANTIQ_PCIE_PHY_MODE_25MHZ:
    case LANTIQ_PCIE_PHY_MODE_25MHZ_SSC:
    case LANTIQ_PCIE_PHY_MODE_36MHZ_SSC:
    case LANTIQ_PCIE_PHY_MODE_100MHZ:
    case LANTIQ_PCIE_PHY_MODE_100MHZ_SSC:
    dev_err(dev, "PHY mode not implemented yet: %u\n", mode);
    return ERR_PTR(-EINVAL);
    default:
    dev_err(dev, "invalid PHY mode %u\n", mode);
    return ERR_PTR(-EINVAL);
    }
    return priv.phy;
    }
#[no_mangle]
unsafe extern "C" fn ltq_vrx200_pcie_phy_probe(pdev: *mut platform_device) -> c_int {
    static int ltq_vrx200_pcie_phy_probe(struct platform_device *pdev)
    {
    static const struct regmap_config regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .reg_stride = 2,
    .max_register = PCIE_PHY_RX1_A_CTRL,
    };
    struct ltq_vrx200_pcie_phy_priv *priv;
    struct device *dev = &pdev.dev;
    struct phy_provider *provider;
    void __iomem *base;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.phy_regmap = devm_regmap_init_mmio(dev, base, &regmap_config);
    if (IS_ERR(priv.phy_regmap))
    return PTR_ERR(priv.phy_regmap);
    priv.rcu_regmap = syscon_regmap_lookup_by_phandle(dev.of_node,
    "lantiq,rcu");
    if (IS_ERR(priv.rcu_regmap))
    return PTR_ERR(priv.rcu_regmap);
    ret = device_property_read_u32(dev, "lantiq,rcu-endian-offset",
    &priv.rcu_ahb_endian_offset);
    if (ret) {
    dev_err(dev,
    "failed to parse the 'lantiq,rcu-endian-offset' property\n");
    return ret;
    }
    ret = device_property_read_u32(dev, "lantiq,rcu-big-endian-mask",
    &priv.rcu_ahb_endian_big_endian_mask);
    if (ret) {
    dev_err(dev,
    "failed to parse the 'lantiq,rcu-big-endian-mask' property\n");
    return ret;
    }
    priv.pdi_clk = devm_clk_get(dev, "pdi");
    if (IS_ERR(priv.pdi_clk))
    return PTR_ERR(priv.pdi_clk);
    priv.phy_clk = devm_clk_get(dev, "phy");
    if (IS_ERR(priv.phy_clk))
    return PTR_ERR(priv.phy_clk);
    priv.phy_reset = devm_reset_control_get_exclusive(dev, "phy");
    if (IS_ERR(priv.phy_reset))
    return PTR_ERR(priv.phy_reset);
    priv.pcie_reset = devm_reset_control_get_shared(dev, "pcie");
    if (IS_ERR(priv.pcie_reset))
    return PTR_ERR(priv.pcie_reset);
    priv.dev = dev;
    priv.phy = devm_phy_create(dev, dev.of_node,
    &ltq_vrx200_pcie_phy_ops);
    if (IS_ERR(priv.phy)) {
    dev_err(dev, "failed to create PHY\n");
    return PTR_ERR(priv.phy);
    }
    phy_set_drvdata(priv.phy, priv);
    dev_set_drvdata(dev, priv);
    provider = devm_of_phy_provider_register(dev,
    ltq_vrx200_pcie_phy_xlate);
    return PTR_ERR_OR_ZERO(provider);
    }
    static const struct of_device_id ltq_vrx200_pcie_phy_of_match[] = {
    { .compatible = "lantiq,vrx200-pcie-phy", },
    { .compatible = "lantiq,arx300-pcie-phy", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, ltq_vrx200_pcie_phy_of_match);
    static struct platform_driver ltq_vrx200_pcie_phy_driver = {
    .probe	= ltq_vrx200_pcie_phy_probe,
    .driver = {
    .name	= "ltq-vrx200-pcie-phy",
    .of_match_table	= ltq_vrx200_pcie_phy_of_match,
    }
    };
    module_platform_driver(ltq_vrx200_pcie_phy_driver);
    MODULE_AUTHOR("Martin Blumenstingl <martin.blumenstingl@googlemail.com>");
    MODULE_DESCRIPTION("Lantiq VRX200 and ARX300 PCIe PHY driver");
    MODULE_LICENSE("GPL v2");
