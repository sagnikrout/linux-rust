//! Automatically rewritten from C to Rust
//! Source: drivers/phy/amlogic/phy-meson8b-usb2.c
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
// Meson8, Meson8b and GXBB USB2 PHY driver
//
// Copyright (C) 2016 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
//

pub const REG_CONFIG: c_uint = 0x00;

pub const REG_CTRL: c_uint = 0x04;

pub const REG_ENDP_INTR: c_uint = 0x08;
// bits [31:26], [24:21] and [15:3] seem to be read-only
pub const REG_ADP_BC: c_uint = 0x0c;

pub const REG_DBG_UART: c_uint = 0x10;

pub const REG_TEST: c_uint = 0x14;

pub const REG_TUNE: c_uint = 0x18;

pub const RESET_COMPLETE_TIME: c_int = 500;
pub const ACA_ENABLE_COMPLETE_TIME: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_meson8b_usb2_match_data {
    pub host_enable_aca: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_meson8b_usb2_priv {
    pub regmap: *mut regmap,
    pub dr_mode: enum usb_dr_mode,
    pub clk_usb_general: *mut clk,
    pub clk_usb: *mut clk,
    pub reset: *mut reset_control,
    pub match: *const phy_meson8b_usb2_match_data,
}

    static const struct regmap_config phy_meson8b_usb2_regmap_conf = {
    .reg_bits = 8,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = REG_TUNE,
    };
#[no_mangle]
unsafe extern "C" fn phy_meson8b_usb2_power_on(phy: *mut phy) -> c_int {
    static int phy_meson8b_usb2_power_on(struct phy *phy)
    {
    struct phy_meson8b_usb2_priv *priv = phy_get_drvdata(phy);
    u32 reg;
    int ret;
    if (!IS_ERR_OR_NULL(priv.reset)) {
    ret = reset_control_reset(priv.reset);
    if (ret) {
    dev_err(&phy.dev, "Failed to trigger USB reset\n");
    return ret;
    }
    }
    ret = clk_prepare_enable(priv.clk_usb_general);
    if (ret) {
    dev_err(&phy.dev, "Failed to enable USB general clock\n");
    reset_control_rearm(priv.reset);
    return ret;
    }
    ret = clk_prepare_enable(priv.clk_usb);
    if (ret) {
    dev_err(&phy.dev, "Failed to enable USB DDR clock\n");
    clk_disable_unprepare(priv.clk_usb_general);
    reset_control_rearm(priv.reset);
    return ret;
    }
    regmap_set_bits(priv.regmap, REG_CONFIG, REG_CONFIG_CLK_32k_ALTSEL);
    regmap_update_bits(priv.regmap, REG_CTRL, REG_CTRL_REF_CLK_SEL_MASK,
    FIELD_PREP(REG_CTRL_REF_CLK_SEL_MASK, 0x2));
    regmap_update_bits(priv.regmap, REG_CTRL, REG_CTRL_FSEL_MASK,
    FIELD_PREP(REG_CTRL_FSEL_MASK, 0x5));
// reset the PHY
    regmap_set_bits(priv.regmap, REG_CTRL, REG_CTRL_POWER_ON_RESET);
    udelay(RESET_COMPLETE_TIME);
    regmap_clear_bits(priv.regmap, REG_CTRL, REG_CTRL_POWER_ON_RESET);
    udelay(RESET_COMPLETE_TIME);
    regmap_set_bits(priv.regmap, REG_CTRL, REG_CTRL_SOF_TOGGLE_OUT);
    if (priv.dr_mode == USB_DR_MODE_HOST) {
    regmap_clear_bits(priv.regmap, REG_DBG_UART,
    REG_DBG_UART_SET_IDDQ);
    if (priv.match.host_enable_aca) {
    regmap_set_bits(priv.regmap, REG_ADP_BC,
    REG_ADP_BC_ACA_ENABLE);
    udelay(ACA_ENABLE_COMPLETE_TIME);
    regmap_read(priv.regmap, REG_ADP_BC, &reg);
    if (reg & REG_ADP_BC_ACA_PIN_FLOAT) {
    dev_warn(&phy.dev, "USB ID detect failed!\n");
    clk_disable_unprepare(priv.clk_usb);
    clk_disable_unprepare(priv.clk_usb_general);
    reset_control_rearm(priv.reset);
    return -EINVAL;
    }
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phy_meson8b_usb2_power_off(phy: *mut phy) -> c_int {
    static int phy_meson8b_usb2_power_off(struct phy *phy)
    {
    struct phy_meson8b_usb2_priv *priv = phy_get_drvdata(phy);
    if (priv.dr_mode == USB_DR_MODE_HOST)
    regmap_set_bits(priv.regmap, REG_DBG_UART,
    REG_DBG_UART_SET_IDDQ);
    clk_disable_unprepare(priv.clk_usb);
    clk_disable_unprepare(priv.clk_usb_general);
    reset_control_rearm(priv.reset);
// power off the PHY by putting it into reset mode
    regmap_set_bits(priv.regmap, REG_CTRL, REG_CTRL_POWER_ON_RESET);
    return 0;
    }
    static const struct phy_ops phy_meson8b_usb2_ops = {
    .power_on	= phy_meson8b_usb2_power_on,
    .power_off	= phy_meson8b_usb2_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn phy_meson8b_usb2_probe(pdev: *mut platform_device) -> c_int {
    static int phy_meson8b_usb2_probe(struct platform_device *pdev)
    {
    struct phy_meson8b_usb2_priv *priv;
    struct phy *phy;
    struct phy_provider *phy_provider;
    void __iomem *base;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.match = device_get_match_data(&pdev.dev);
    if (!priv.match)
    return -ENODEV;
    priv.regmap = devm_regmap_init_mmio(&pdev.dev, base,
    &phy_meson8b_usb2_regmap_conf);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    priv.clk_usb_general = devm_clk_get(&pdev.dev, "usb_general");
    if (IS_ERR(priv.clk_usb_general))
    return PTR_ERR(priv.clk_usb_general);
    priv.clk_usb = devm_clk_get(&pdev.dev, "usb");
    if (IS_ERR(priv.clk_usb))
    return PTR_ERR(priv.clk_usb);
    priv.reset = devm_reset_control_get_optional_shared(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(priv.reset))
    return dev_err_probe(&pdev.dev, PTR_ERR(priv.reset),
    "Failed to get the reset line");
    priv.dr_mode = of_usb_get_dr_mode_by_phy(pdev.dev.of_node, -1);
    if (priv.dr_mode == USB_DR_MODE_UNKNOWN) {
    dev_err(&pdev.dev,
    "missing dual role configuration of the controller\n");
    return -EINVAL;
    }
    phy = devm_phy_create(&pdev.dev, core::ptr::null_mut(), &phy_meson8b_usb2_ops);
    if (IS_ERR(phy)) {
    return dev_err_probe(&pdev.dev, PTR_ERR(phy),
    "failed to create PHY\n");
    }
    phy_set_drvdata(phy, priv);
    phy_provider =
    devm_of_phy_provider_register(&pdev.dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct phy_meson8b_usb2_match_data phy_meson8_usb2_match_data = {
    .host_enable_aca = false,
    };
    static const struct phy_meson8b_usb2_match_data phy_meson8b_usb2_match_data = {
    .host_enable_aca = true,
    };
    static const struct of_device_id phy_meson8b_usb2_of_match[] = {
    {
    .compatible = "amlogic,meson8-usb2-phy",
    .data = &phy_meson8_usb2_match_data
    },
    {
    .compatible = "amlogic,meson8b-usb2-phy",
    .data = &phy_meson8b_usb2_match_data
    },
    {
    .compatible = "amlogic,meson8m2-usb2-phy",
    .data = &phy_meson8b_usb2_match_data
    },
    {
    .compatible = "amlogic,meson-gxbb-usb2-phy",
    .data = &phy_meson8b_usb2_match_data
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, phy_meson8b_usb2_of_match);
    static struct platform_driver phy_meson8b_usb2_driver = {
    .probe	= phy_meson8b_usb2_probe,
    .driver	= {
    .name		= "phy-meson-usb2",
    .of_match_table	= phy_meson8b_usb2_of_match,
    },
    };
    module_platform_driver(phy_meson8b_usb2_driver);
    MODULE_AUTHOR("Martin Blumenstingl <martin.blumenstingl@googlemail.com>");
    MODULE_DESCRIPTION("Meson8, Meson8b, Meson8m2 and GXBB USB2 PHY driver");
    MODULE_LICENSE("GPL");
