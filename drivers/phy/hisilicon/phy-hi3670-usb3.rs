//! Automatically rewritten from C to Rust
//! Source: drivers/phy/hisilicon/phy-hi3670-usb3.c
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
// Phy provider for USB 3.1 controller on HiSilicon Kirin970 platform
//
// Copyright (C) 2017-2020 Hilisicon Electronics Co., Ltd.
// http://www.huawei.com
//
// Authors: Yu Chen <chenyu56@huawei.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi3670_priv {
    pub dev: *mut device,
    pub peri_crg: *mut regmap,
    pub pctrl: *mut regmap,
    pub sctrl: *mut regmap,
    pub usb31misc: *mut regmap,
    pub eye_diagram_param: u32,
    pub tx_vboost_lvl: u32,
    pub peri_crg_offset: u32,
    pub pctrl_offset: u32,
    pub usb31misc_offset: u32,
}

#[no_mangle]
unsafe extern "C" fn hi3670_phy_cr_clk(usb31misc: *mut regmap) -> c_int {
    static int hi3670_phy_cr_clk(struct regmap *usb31misc)
    {
    int ret;
// Clock up
    ret = regmap_update_bits(usb31misc, USB_MISC_CFG54,
    CFG54_USB31PHY_CR_CLK, CFG54_USB31PHY_CR_CLK);
    if (ret)
    return ret;
// Clock down
    return regmap_update_bits(usb31misc, USB_MISC_CFG54,
    CFG54_USB31PHY_CR_CLK, 0);
    }
#[no_mangle]
unsafe extern "C" fn hi3670_phy_cr_set_sel(usb31misc: *mut regmap) -> c_int {
    static int hi3670_phy_cr_set_sel(struct regmap *usb31misc)
    {
    return regmap_update_bits(usb31misc, USB_MISC_CFG54,
    CFG54_USB31PHY_CR_SEL, CFG54_USB31PHY_CR_SEL);
    }
#[no_mangle]
unsafe extern "C" fn hi3670_phy_cr_start(usb31misc: *mut regmap, direction: c_int) -> c_int {
    static int hi3670_phy_cr_start(struct regmap *usb31misc, int direction)
    {
    int ret, reg;
    if (direction)
    reg = CFG54_USB31PHY_CR_WR_EN;
    else
    reg = CFG54_USB31PHY_CR_RD_EN;
    ret = regmap_update_bits(usb31misc, USB_MISC_CFG54, reg, reg);
    if (ret)
    return ret;
    ret = hi3670_phy_cr_clk(usb31misc);
    if (ret)
    return ret;
    return regmap_update_bits(usb31misc, USB_MISC_CFG54,
    CFG54_USB31PHY_CR_RD_EN | CFG54_USB31PHY_CR_WR_EN, 0);
    }
#[no_mangle]
unsafe extern "C" fn hi3670_phy_cr_wait_ack(usb31misc: *mut regmap) -> c_int {
    static int hi3670_phy_cr_wait_ack(struct regmap *usb31misc)
    {
    u32 reg;
    let mut retry: c_int = 10;
    int ret;
    while (retry-- > 0) {
    ret = regmap_read(usb31misc, USB_MISC_CFG54, &reg);
    if (ret)
    return ret;
    if ((reg & CFG54_USB31PHY_CR_ACK) == CFG54_USB31PHY_CR_ACK)
    return 0;
    ret = hi3670_phy_cr_clk(usb31misc);
    if (ret)
    return ret;
    usleep_range(10, 20);
    }
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn hi3670_phy_cr_set_addr(usb31misc: *mut regmap, addr: u32) -> c_int {
    static int hi3670_phy_cr_set_addr(struct regmap *usb31misc, u32 addr)
    {
    u32 reg;
    int ret;
    ret = regmap_read(usb31misc, USB_MISC_CFG54, &reg);
    if (ret)
    return ret;
    reg = FIELD_PREP(CFG54_USB31PHY_CR_ADDR_MASK, addr);
    return regmap_update_bits(usb31misc, USB_MISC_CFG54,
    CFG54_USB31PHY_CR_ADDR_MASK, reg);
    }
#[no_mangle]
unsafe extern "C" fn hi3670_phy_cr_read(usb31misc: *mut regmap, addr: u32, val: *mut u32) -> c_int {
    static int hi3670_phy_cr_read(struct regmap *usb31misc, u32 addr, u32 *val)
    {
    int reg, i, ret;
    for (i = 0; i < 100; i++) {
    ret = hi3670_phy_cr_clk(usb31misc);
    if (ret)
    return ret;
    }
    ret = hi3670_phy_cr_set_sel(usb31misc);
    if (ret)
    return ret;
    ret = hi3670_phy_cr_set_addr(usb31misc, addr);
    if (ret)
    return ret;
    ret = hi3670_phy_cr_start(usb31misc, 0);
    if (ret)
    return ret;
    ret = hi3670_phy_cr_wait_ack(usb31misc);
    if (ret)
    return ret;
    ret = regmap_read(usb31misc, USB_MISC_CFG58, &reg);
    if (ret)
    return ret;
// val = FIELD_GET(CFG58_USB31PHY_CR_DATA_MASK, reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3670_phy_cr_write(usb31misc: *mut regmap, addr: u32, val: u32) -> c_int {
    static int hi3670_phy_cr_write(struct regmap *usb31misc, u32 addr, u32 val)
    {
    int i;
    int ret;
    for (i = 0; i < 100; i++) {
    ret = hi3670_phy_cr_clk(usb31misc);
    if (ret)
    return ret;
    }
    ret = hi3670_phy_cr_set_sel(usb31misc);
    if (ret)
    return ret;
    ret = hi3670_phy_cr_set_addr(usb31misc, addr);
    if (ret)
    return ret;
    ret = regmap_write(usb31misc, USB_MISC_CFG58,
    FIELD_PREP(CFG58_USB31PHY_CR_DATA_MASK, val));
    if (ret)
    return ret;
    ret = hi3670_phy_cr_start(usb31misc, 1);
    if (ret)
    return ret;
    return hi3670_phy_cr_wait_ack(usb31misc);
    }
#[no_mangle]
unsafe extern "C" fn hi3670_phy_set_params(priv: *mut hi3670_priv) -> c_int {
    static int hi3670_phy_set_params(struct hi3670_priv *priv)
    {
    u32 reg;
    int ret;
    let mut retry: c_int = 3;
    ret = regmap_write(priv.usb31misc, USB3OTG_CTRL4,
    priv.eye_diagram_param);
    if (ret) {
    dev_err(priv.dev, "set USB3OTG_CTRL4 failed\n");
    return ret;
    }
    while (retry-- > 0) {
    ret = hi3670_phy_cr_read(priv.usb31misc,
    TX_VBOOST_LVL_REG, &reg);
    if (!ret)
    break;
    if (ret != -ETIMEDOUT) {
    dev_err(priv.dev, "read TX_VBOOST_LVL_REG failed\n");
    return ret;
    }
    }
    if (ret)
    return ret;
    reg |= (TX_VBOOST_LVL_ENABLE | (priv.tx_vboost_lvl << TX_VBOOST_LVL_START));
    ret = hi3670_phy_cr_write(priv.usb31misc, TX_VBOOST_LVL_REG, reg);
    if (ret)
    dev_err(priv.dev, "write TX_VBOOST_LVL_REG failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hi3670_is_abbclk_selected(priv: *mut hi3670_priv) -> bool {
    static bool hi3670_is_abbclk_selected(struct hi3670_priv *priv)
    {
    u32 reg;
    if (!priv.sctrl) {
    dev_err(priv.dev, "priv.sctrl is null!\n");
    return false;
    }
    if (regmap_read(priv.sctrl, SCTRL_SCDEEPSLEEPED, &reg)) {
    dev_err(priv.dev, "SCTRL_SCDEEPSLEEPED read failed!\n");
    return false;
    }
    if ((reg & USB_CLK_SELECTED) == 0)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn hi3670_config_phy_clock(priv: *mut hi3670_priv) -> c_int {
    static int hi3670_config_phy_clock(struct hi3670_priv *priv)
    {
    u32 val, mask;
    int ret;
    if (!hi3670_is_abbclk_selected(priv)) {
// usb refclk iso disable
    ret = regmap_write(priv.peri_crg, PERI_CRG_ISODIS,
    USB_REFCLK_ISO_EN);
    if (ret)
    goto out;
// enable usb_tcxo_en
    ret = regmap_write(priv.pctrl, PCTRL_PERI_CTRL3,
    USB_TCXO_EN |
    (USB_TCXO_EN << PCTRL_PERI_CTRL3_MSK_START));
// select usbphy clk from abb
    mask = SC_CLK_USB3PHY_3MUX1_SEL;
    ret = regmap_update_bits(priv.pctrl,
    PCTRL_PERI_CTRL24, mask, 0);
    if (ret)
    goto out;
    ret = regmap_update_bits(priv.usb31misc, USB_MISC_CFGA0,
    CFGA0_USB2PHY_REFCLK_SELECT, 0);
    if (ret)
    goto out;
    ret = regmap_read(priv.usb31misc, USB3OTG_CTRL7, &val);
    if (ret)
    goto out;
    val &= ~CTRL7_USB2_REFCLKSEL_MASK;
    val |= CTRL7_USB2_REFCLKSEL_ABB;
    ret = regmap_write(priv.usb31misc, USB3OTG_CTRL7, val);
    if (ret)
    goto out;
    return 0;
    }
    ret = regmap_update_bits(priv.usb31misc, USB_MISC_CFG54,
    CFG54_USB3PHY_REF_USE_PAD,
    CFG54_USB3PHY_REF_USE_PAD);
    if (ret)
    goto out;
    ret = regmap_update_bits(priv.usb31misc, USB_MISC_CFGA0,
    CFGA0_USB2PHY_REFCLK_SELECT,
    CFGA0_USB2PHY_REFCLK_SELECT);
    if (ret)
    goto out;
    ret = regmap_read(priv.usb31misc, USB3OTG_CTRL7, &val);
    if (ret)
    goto out;
    val &= ~CTRL7_USB2_REFCLKSEL_MASK;
    val |= CTRL7_USB2_REFCLKSEL_PAD;
    ret = regmap_write(priv.usb31misc, USB3OTG_CTRL7, val);
    if (ret)
    goto out;
    ret = regmap_write(priv.peri_crg,
    PERI_CRG_PEREN6, GT_CLK_USB2PHY_REF);
    if (ret)
    goto out;
    return 0;
    out:
    dev_err(priv.dev, "failed to config phy clock ret: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hi3670_config_tca(priv: *mut hi3670_priv) -> c_int {
    static int hi3670_config_tca(struct hi3670_priv *priv)
    {
    u32 val, mask;
    int ret;
    ret = regmap_write(priv.usb31misc, TCA_INTR_STS, 0xffff);
    if (ret)
    goto out;
    ret = regmap_write(priv.usb31misc, TCA_INTR_EN,
    INTR_EN_XA_TIMEOUT_EVT_EN | INTR_EN_XA_ACK_EVT_EN);
    if (ret)
    goto out;
    mask = CLK_RST_TCA_REF_CLK_EN | CLK_RST_SUSPEND_CLK_EN;
    ret = regmap_update_bits(priv.usb31misc, TCA_CLK_RST, mask, 0);
    if (ret)
    goto out;
    ret = regmap_update_bits(priv.usb31misc, TCA_GCFG,
    GCFG_ROLE_HSTDEV | GCFG_OP_MODE,
    GCFG_ROLE_HSTDEV | GCFG_OP_MODE_CTRL_SYNC_MODE);
    if (ret)
    goto out;
    ret = regmap_update_bits(priv.usb31misc, TCA_SYSMODE_CFG,
    SYSMODE_CFG_TYPEC_DISABLE, 0);
    if (ret)
    goto out;
    ret = regmap_read(priv.usb31misc, TCA_TCPC, &val);
    if (ret)
    goto out;
    val &= ~(TCPC_VALID | TCPC_LOW_POWER_EN | TCPC_MUX_CONTROL_MASK);
    val |= (TCPC_VALID | TCPC_MUX_CONTROL_USB31);
    ret = regmap_write(priv.usb31misc, TCA_TCPC, val);
    if (ret)
    goto out;
    ret = regmap_write(priv.usb31misc, TCA_VBUS_CTRL,
    VBUS_CTRL_POWERPRESENT_OVERRD | VBUS_CTRL_VBUSVALID_OVERRD);
    if (ret)
    goto out;
    return 0;
    out:
    dev_err(priv.dev, "failed to config phy clock ret: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hi3670_phy_init(phy: *mut phy) -> c_int {
    static int hi3670_phy_init(struct phy *phy)
    {
    struct hi3670_priv *priv = phy_get_drvdata(phy);
    u32 val;
    int ret;
// assert controller
    val = CFGA0_VAUX_RESET | CFGA0_USB31C_RESET |
    CFGA0_USB3PHY_RESET | CFGA0_USB2PHY_POR;
    ret = regmap_update_bits(priv.usb31misc, USB_MISC_CFGA0, val, 0);
    if (ret)
    goto out;
    ret = hi3670_config_phy_clock(priv);
    if (ret)
    goto out;
// Exit from IDDQ mode
    ret = regmap_update_bits(priv.usb31misc, USB3OTG_CTRL5,
    CTRL5_USB2_SIDDQ, 0);
    if (ret)
    goto out;
// Release USB31 PHY out of TestPowerDown mode
    ret = regmap_update_bits(priv.usb31misc, USB_MISC_CFG50,
    CFG50_USB3_PHY_TEST_POWERDOWN, 0);
    if (ret)
    goto out;
// Deassert phy
    val = CFGA0_USB3PHY_RESET | CFGA0_USB2PHY_POR;
    ret = regmap_update_bits(priv.usb31misc, USB_MISC_CFGA0, val, val);
    if (ret)
    goto out;
    usleep_range(100, 120);
// Tell the PHY power is stable
    val = CFG54_USB3_PHY0_ANA_PWR_EN | CFG54_PHY0_PCS_PWR_STABLE |
    CFG54_PHY0_PMA_PWR_STABLE;
    ret = regmap_update_bits(priv.usb31misc, USB_MISC_CFG54,
    val, val);
    if (ret)
    goto out;
    ret = hi3670_config_tca(priv);
    if (ret)
    goto out;
// Enable SSC
    ret = regmap_update_bits(priv.usb31misc, USB_MISC_CFG5C,
    CFG5C_USB3_PHY0_SS_MPLLA_SSC_EN,
    CFG5C_USB3_PHY0_SS_MPLLA_SSC_EN);
    if (ret)
    goto out;
// Deassert controller
    val = CFGA0_VAUX_RESET | CFGA0_USB31C_RESET;
    ret = regmap_update_bits(priv.usb31misc, USB_MISC_CFGA0, val, val);
    if (ret)
    goto out;
    usleep_range(100, 120);
// Set fake vbus valid signal
    val = CTRL0_USB3_VBUSVLD | CTRL0_USB3_VBUSVLD_SEL;
    ret = regmap_update_bits(priv.usb31misc, USB3OTG_CTRL0, val, val);
    if (ret)
    goto out;
    val = CTRL3_USB2_VBUSVLDEXT0 | CTRL3_USB2_VBUSVLDEXTSEL0;
    ret = regmap_update_bits(priv.usb31misc, USB3OTG_CTRL3, val, val);
    if (ret)
    goto out;
    usleep_range(100, 120);
    ret = hi3670_phy_set_params(priv);
    if (ret)
    goto out;
    return 0;
    out:
    dev_err(priv.dev, "failed to init phy ret: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hi3670_phy_exit(phy: *mut phy) -> c_int {
    static int hi3670_phy_exit(struct phy *phy)
    {
    struct hi3670_priv *priv = phy_get_drvdata(phy);
    u32 mask;
    int ret;
// Assert phy
    mask = CFGA0_USB3PHY_RESET | CFGA0_USB2PHY_POR;
    ret = regmap_update_bits(priv.usb31misc, USB_MISC_CFGA0, mask, 0);
    if (ret)
    goto out;
    if (!hi3670_is_abbclk_selected(priv)) {
// disable usb_tcxo_en
    ret = regmap_write(priv.pctrl, PCTRL_PERI_CTRL3,
    USB_TCXO_EN << PCTRL_PERI_CTRL3_MSK_START);
    } else {
    ret = regmap_write(priv.peri_crg, PERI_CRG_PERDIS6,
    GT_CLK_USB2PHY_REF);
    if (ret)
    goto out;
    }
    return 0;
    out:
    dev_err(priv.dev, "failed to exit phy ret: %d\n", ret);
    return ret;
    }
    static const struct phy_ops hi3670_phy_ops = {
    .init		= hi3670_phy_init,
    .exit		= hi3670_phy_exit,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn hi3670_phy_probe(pdev: *mut platform_device) -> c_int {
    static int hi3670_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct phy *phy;
    struct hi3670_priv *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.peri_crg = syscon_regmap_lookup_by_phandle(dev.of_node,
    "hisilicon,pericrg-syscon");
    if (IS_ERR(priv.peri_crg)) {
    dev_err(dev, "no hisilicon,pericrg-syscon\n");
    return PTR_ERR(priv.peri_crg);
    }
    priv.pctrl = syscon_regmap_lookup_by_phandle(dev.of_node,
    "hisilicon,pctrl-syscon");
    if (IS_ERR(priv.pctrl)) {
    dev_err(dev, "no hisilicon,pctrl-syscon\n");
    return PTR_ERR(priv.pctrl);
    }
    priv.sctrl = syscon_regmap_lookup_by_phandle(dev.of_node,
    "hisilicon,sctrl-syscon");
    if (IS_ERR(priv.sctrl)) {
    dev_err(dev, "no hisilicon,sctrl-syscon\n");
    return PTR_ERR(priv.sctrl);
    }
// node of hi3670 phy is a sub-node of usb3_otg_bc
    priv.usb31misc = syscon_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(priv.usb31misc)) {
    dev_err(dev, "no hisilicon,usb3-otg-bc-syscon\n");
    return PTR_ERR(priv.usb31misc);
    }
    if (of_property_read_u32(dev.of_node, "hisilicon,eye-diagram-param",
    &priv.eye_diagram_param))
    priv.eye_diagram_param = KIRIN970_USB_DEFAULT_PHY_PARAM;
    if (of_property_read_u32(dev.of_node, "hisilicon,tx-vboost-lvl",
    &priv.tx_vboost_lvl))
    priv.tx_vboost_lvl = KIRIN970_USB_DEFAULT_PHY_VBOOST;
    phy = devm_phy_create(dev, core::ptr::null_mut(), &hi3670_phy_ops);
    if (IS_ERR(phy))
    return PTR_ERR(phy);
    phy_set_drvdata(phy, priv);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id hi3670_phy_of_match[] = {
    { .compatible = "hisilicon,hi3670-usb-phy" },
    { },
    };
    MODULE_DEVICE_TABLE(of, hi3670_phy_of_match);
    static struct platform_driver hi3670_phy_driver = {
    .probe	= hi3670_phy_probe,
    .driver = {
    .name	= "hi3670-usb-phy",
    .of_match_table	= hi3670_phy_of_match,
    }
    };
    module_platform_driver(hi3670_phy_driver);
    MODULE_AUTHOR("Yu Chen <chenyu56@huawei.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Hilisicon Kirin970 USB31 PHY Driver");
