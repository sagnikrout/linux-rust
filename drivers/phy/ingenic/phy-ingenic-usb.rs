//! Automatically rewritten from C to Rust
//! Source: drivers/phy/ingenic/phy-ingenic-usb.c
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
// Ingenic SoCs USB PHY driver
// Copyright (c) Paul Cercueil <paul@crapouillou.net>
// Copyright (c) 漆鹏振 (Qi Pengzhen) <aric.pzqi@ingenic.com>
// Copyright (c) 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
//

// OTGPHY register offsets
pub const REG_USBPCR_OFFSET: c_uint = 0x00;
pub const REG_USBRDT_OFFSET: c_uint = 0x04;
pub const REG_USBVBFIL_OFFSET: c_uint = 0x08;
pub const REG_USBPCR1_OFFSET: c_uint = 0x0c;
// bits within the USBPCR register

pub const USBPCR_IDPULLUP_ALWAYS: c_uint = 0x2;
pub const USBPCR_IDPULLUP_SUSPEND: c_uint = 0x1;
pub const USBPCR_IDPULLUP_OTG: c_uint = 0x0;

pub const USBPCR_COMPDISTUNE_DFT: c_uint = 0x4;

pub const USBPCR_OTGTUNE_DFT: c_uint = 0x4;

pub const USBPCR_SQRXTUNE_DCR_20PCT: c_uint = 0x7;
pub const USBPCR_SQRXTUNE_DFT: c_uint = 0x3;

pub const USBPCR_TXFSLSTUNE_DCR_50PPT: c_uint = 0xf;
pub const USBPCR_TXFSLSTUNE_DCR_25PPT: c_uint = 0x7;
pub const USBPCR_TXFSLSTUNE_DFT: c_uint = 0x3;
pub const USBPCR_TXFSLSTUNE_INC_25PPT: c_uint = 0x1;
pub const USBPCR_TXFSLSTUNE_INC_50PPT: c_uint = 0x0;

pub const USBPCR_TXHSXVTUNE_DFT: c_uint = 0x3;
pub const USBPCR_TXHSXVTUNE_DCR_15MV: c_uint = 0x1;

pub const USBPCR_TXRISETUNE_DFT: c_uint = 0x3;

pub const USBPCR_TXVREFTUNE_INC_75PPT: c_uint = 0xb;
pub const USBPCR_TXVREFTUNE_INC_25PPT: c_uint = 0x7;
pub const USBPCR_TXVREFTUNE_DFT: c_uint = 0x5;
// bits within the USBRDTR register

// bits within the USBPCR1 register

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_soc_info {
    pub phy): *mut *mut void (usb_phy_init)(struct phy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_usb_phy {
    pub soc_info: *const ingenic_soc_info,
    pub phy: *mut phy,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub vcc_supply: *mut regulator,
}

#[no_mangle]
unsafe extern "C" fn ingenic_usb_phy_init(phy: *mut phy) -> c_int {
    static int ingenic_usb_phy_init(struct phy *phy)
    {
    struct ingenic_usb_phy *priv = phy_get_drvdata(phy);
    int err;
    u32 reg;
    err = clk_prepare_enable(priv.clk);
    if (err) {
    dev_err(&phy.dev, "Unable to start clock: %d\n", err);
    return err;
    }
    priv.soc_info.usb_phy_init(phy);
// Wait for PHY to reset
    usleep_range(30, 300);
    reg = readl(priv.base + REG_USBPCR_OFFSET);
    writel(reg & ~USBPCR_POR, priv.base + REG_USBPCR_OFFSET);
    usleep_range(300, 1000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_usb_phy_exit(phy: *mut phy) -> c_int {
    static int ingenic_usb_phy_exit(struct phy *phy)
    {
    struct ingenic_usb_phy *priv = phy_get_drvdata(phy);
    clk_disable_unprepare(priv.clk);
    regulator_disable(priv.vcc_supply);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_usb_phy_power_on(phy: *mut phy) -> c_int {
    static int ingenic_usb_phy_power_on(struct phy *phy)
    {
    struct ingenic_usb_phy *priv = phy_get_drvdata(phy);
    int err;
    err = regulator_enable(priv.vcc_supply);
    if (err) {
    dev_err(&phy.dev, "Unable to enable VCC: %d\n", err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_usb_phy_power_off(phy: *mut phy) -> c_int {
    static int ingenic_usb_phy_power_off(struct phy *phy)
    {
    struct ingenic_usb_phy *priv = phy_get_drvdata(phy);
    regulator_disable(priv.vcc_supply);
    return 0;
    }
    static int ingenic_usb_phy_set_mode(struct phy *phy,
    enum phy_mode mode, int submode)
    {
    struct ingenic_usb_phy *priv = phy_get_drvdata(phy);
    u32 reg;
    switch (mode) {
    case PHY_MODE_USB_HOST:
    reg = readl(priv.base + REG_USBPCR_OFFSET);
    u32p_replace_bits(&reg, 1, USBPCR_USB_MODE);
    u32p_replace_bits(&reg, 0, USBPCR_VBUSVLDEXT);
    u32p_replace_bits(&reg, 0, USBPCR_VBUSVLDEXTSEL);
    u32p_replace_bits(&reg, 0, USBPCR_OTG_DISABLE);
    writel(reg, priv.base + REG_USBPCR_OFFSET);
    break;
    case PHY_MODE_USB_DEVICE:
    reg = readl(priv.base + REG_USBPCR_OFFSET);
    u32p_replace_bits(&reg, 0, USBPCR_USB_MODE);
    u32p_replace_bits(&reg, 1, USBPCR_VBUSVLDEXT);
    u32p_replace_bits(&reg, 1, USBPCR_VBUSVLDEXTSEL);
    u32p_replace_bits(&reg, 1, USBPCR_OTG_DISABLE);
    writel(reg, priv.base + REG_USBPCR_OFFSET);
    break;
    case PHY_MODE_USB_OTG:
    reg = readl(priv.base + REG_USBPCR_OFFSET);
    u32p_replace_bits(&reg, 1, USBPCR_USB_MODE);
    u32p_replace_bits(&reg, 1, USBPCR_VBUSVLDEXT);
    u32p_replace_bits(&reg, 1, USBPCR_VBUSVLDEXTSEL);
    u32p_replace_bits(&reg, 0, USBPCR_OTG_DISABLE);
    writel(reg, priv.base + REG_USBPCR_OFFSET);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct phy_ops ingenic_usb_phy_ops = {
    .init		= ingenic_usb_phy_init,
    .exit		= ingenic_usb_phy_exit,
    .power_on	= ingenic_usb_phy_power_on,
    .power_off	= ingenic_usb_phy_power_off,
    .set_mode	= ingenic_usb_phy_set_mode,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn jz4770_usb_phy_init(phy: *mut phy) {
    static void jz4770_usb_phy_init(struct phy *phy)
    {
    struct ingenic_usb_phy *priv = phy_get_drvdata(phy);
    u32 reg;
    reg = USBPCR_AVLD_REG | USBPCR_COMMONONN | USBPCR_POR |
    FIELD_PREP(USBPCR_IDPULLUP_MASK, USBPCR_IDPULLUP_ALWAYS) |
    FIELD_PREP(USBPCR_COMPDISTUNE_MASK, USBPCR_COMPDISTUNE_DFT) |
    FIELD_PREP(USBPCR_OTGTUNE_MASK, USBPCR_OTGTUNE_DFT) |
    FIELD_PREP(USBPCR_SQRXTUNE_MASK, USBPCR_SQRXTUNE_DFT) |
    FIELD_PREP(USBPCR_TXFSLSTUNE_MASK, USBPCR_TXFSLSTUNE_DFT) |
    FIELD_PREP(USBPCR_TXRISETUNE_MASK, USBPCR_TXRISETUNE_DFT) |
    FIELD_PREP(USBPCR_TXVREFTUNE_MASK, USBPCR_TXVREFTUNE_DFT);
    writel(reg, priv.base + REG_USBPCR_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn jz4775_usb_phy_init(phy: *mut phy) {
    static void jz4775_usb_phy_init(struct phy *phy)
    {
    struct ingenic_usb_phy *priv = phy_get_drvdata(phy);
    u32 reg;
    reg = readl(priv.base + REG_USBPCR1_OFFSET) | USBPCR1_USB_SEL |
    USBPCR1_WORD_IF_16BIT;
    writel(reg, priv.base + REG_USBPCR1_OFFSET);
    reg = USBPCR_COMMONONN | USBPCR_POR |
    FIELD_PREP(USBPCR_TXVREFTUNE_MASK, USBPCR_TXVREFTUNE_INC_75PPT);
    writel(reg, priv.base + REG_USBPCR_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn jz4780_usb_phy_init(phy: *mut phy) {
    static void jz4780_usb_phy_init(struct phy *phy)
    {
    struct ingenic_usb_phy *priv = phy_get_drvdata(phy);
    u32 reg;
    reg = readl(priv.base + REG_USBPCR1_OFFSET) | USBPCR1_USB_SEL |
    USBPCR1_WORD_IF_16BIT;
    writel(reg, priv.base + REG_USBPCR1_OFFSET);
    reg = USBPCR_TXPREEMPHTUNE | USBPCR_COMMONONN | USBPCR_POR;
    writel(reg, priv.base + REG_USBPCR_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn x1000_usb_phy_init(phy: *mut phy) {
    static void x1000_usb_phy_init(struct phy *phy)
    {
    struct ingenic_usb_phy *priv = phy_get_drvdata(phy);
    u32 reg;
    reg = readl(priv.base + REG_USBPCR1_OFFSET) | USBPCR1_WORD_IF_16BIT;
    writel(reg, priv.base + REG_USBPCR1_OFFSET);
    reg = USBPCR_TXPREEMPHTUNE | USBPCR_COMMONONN | USBPCR_POR |
    FIELD_PREP(USBPCR_SQRXTUNE_MASK, USBPCR_SQRXTUNE_DCR_20PCT) |
    FIELD_PREP(USBPCR_TXHSXVTUNE_MASK, USBPCR_TXHSXVTUNE_DCR_15MV) |
    FIELD_PREP(USBPCR_TXVREFTUNE_MASK, USBPCR_TXVREFTUNE_INC_25PPT);
    writel(reg, priv.base + REG_USBPCR_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn x1830_usb_phy_init(phy: *mut phy) {
    static void x1830_usb_phy_init(struct phy *phy)
    {
    struct ingenic_usb_phy *priv = phy_get_drvdata(phy);
    u32 reg;
// rdt
    writel(USBRDT_VBFIL_EN | USBRDT_UTMI_RST, priv.base + REG_USBRDT_OFFSET);
    reg = readl(priv.base + REG_USBPCR1_OFFSET) | USBPCR1_WORD_IF_16BIT |
    USBPCR1_DMPD | USBPCR1_DPPD;
    writel(reg, priv.base + REG_USBPCR1_OFFSET);
    reg = USBPCR_VBUSVLDEXT | USBPCR_TXPREEMPHTUNE | USBPCR_COMMONONN | USBPCR_POR |
    FIELD_PREP(USBPCR_IDPULLUP_MASK, USBPCR_IDPULLUP_OTG);
    writel(reg, priv.base + REG_USBPCR_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn x2000_usb_phy_init(phy: *mut phy) {
    static void x2000_usb_phy_init(struct phy *phy)
    {
    struct ingenic_usb_phy *priv = phy_get_drvdata(phy);
    u32 reg;
    reg = readl(priv.base + REG_USBPCR1_OFFSET) | USBPCR1_DPPD | USBPCR1_DMPD;
    writel(reg & ~USBPCR1_PORT_RST, priv.base + REG_USBPCR1_OFFSET);
    reg = USBPCR_POR | FIELD_PREP(USBPCR_IDPULLUP_MASK, USBPCR_IDPULLUP_OTG);
    writel(reg, priv.base + REG_USBPCR_OFFSET);
    }
    static const struct ingenic_soc_info jz4770_soc_info = {
    .usb_phy_init = jz4770_usb_phy_init,
    };
    static const struct ingenic_soc_info jz4775_soc_info = {
    .usb_phy_init = jz4775_usb_phy_init,
    };
    static const struct ingenic_soc_info jz4780_soc_info = {
    .usb_phy_init = jz4780_usb_phy_init,
    };
    static const struct ingenic_soc_info x1000_soc_info = {
    .usb_phy_init = x1000_usb_phy_init,
    };
    static const struct ingenic_soc_info x1830_soc_info = {
    .usb_phy_init = x1830_usb_phy_init,
    };
    static const struct ingenic_soc_info x2000_soc_info = {
    .usb_phy_init = x2000_usb_phy_init,
    };
#[no_mangle]
unsafe extern "C" fn ingenic_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int ingenic_usb_phy_probe(struct platform_device *pdev)
    {
    struct ingenic_usb_phy *priv;
    struct phy_provider *provider;
    struct device *dev = &pdev.dev;
    int err;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.soc_info = device_get_match_data(dev);
    if (!priv.soc_info) {
    dev_err(dev, "Error: No device match found\n");
    return -ENODEV;
    }
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base)) {
    dev_err(dev, "Failed to map registers\n");
    return PTR_ERR(priv.base);
    }
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk)) {
    err = PTR_ERR(priv.clk);
    return dev_err_probe(dev, err, "Failed to get clock\n");
    }
    priv.vcc_supply = devm_regulator_get(dev, "vcc");
    if (IS_ERR(priv.vcc_supply)) {
    err = PTR_ERR(priv.vcc_supply);
    return dev_err_probe(dev, err, "Failed to get regulator\n");
    }
    priv.phy = devm_phy_create(dev, core::ptr::null_mut(), &ingenic_usb_phy_ops);
    if (IS_ERR(priv.phy))
    return PTR_ERR(priv.phy);
    phy_set_drvdata(priv.phy, priv);
    provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(provider);
    }
    static const struct of_device_id ingenic_usb_phy_of_matches[] = {
    { .compatible = "ingenic,jz4770-phy", .data = &jz4770_soc_info },
    { .compatible = "ingenic,jz4775-phy", .data = &jz4775_soc_info },
    { .compatible = "ingenic,jz4780-phy", .data = &jz4780_soc_info },
    { .compatible = "ingenic,x1000-phy", .data = &x1000_soc_info },
    { .compatible = "ingenic,x1830-phy", .data = &x1830_soc_info },
    { .compatible = "ingenic,x2000-phy", .data = &x2000_soc_info },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ingenic_usb_phy_of_matches);
    static struct platform_driver ingenic_usb_phy_driver = {
    .probe		= ingenic_usb_phy_probe,
    .driver		= {
    .name	= "ingenic-usb-phy",
    .of_match_table = ingenic_usb_phy_of_matches,
    },
    };
    module_platform_driver(ingenic_usb_phy_driver);
    MODULE_AUTHOR("周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>");
    MODULE_AUTHOR("漆鹏振 (Qi Pengzhen) <aric.pzqi@ingenic.com>");
    MODULE_AUTHOR("Paul Cercueil <paul@crapouillou.net>");
    MODULE_DESCRIPTION("Ingenic SoCs USB PHY driver");
    MODULE_LICENSE("GPL");
