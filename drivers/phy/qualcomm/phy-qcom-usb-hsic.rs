//! Automatically rewritten from C to Rust
//! Source: drivers/phy/qualcomm/phy-qcom-usb-hsic.c
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
// Copyright (C) 2016 Linaro Ltd
//

pub const ULPI_HSIC_CFG: c_uint = 0x30;
pub const ULPI_HSIC_IO_CAL: c_uint = 0x33;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_usb_hsic_phy {
    pub ulpi: *mut ulpi,
    pub phy: *mut phy,
    pub pctl: *mut pinctrl,
    pub phy_clk: *mut clk,
    pub cal_clk: *mut clk,
    pub cal_sleep_clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn qcom_usb_hsic_phy_power_on(phy: *mut phy) -> c_int {
    static int qcom_usb_hsic_phy_power_on(struct phy *phy)
    {
    struct qcom_usb_hsic_phy *uphy = phy_get_drvdata(phy);
    struct ulpi *ulpi = uphy.ulpi;
    struct pinctrl_state *pins_default;
    int ret;
    ret = clk_prepare_enable(uphy.phy_clk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(uphy.cal_clk);
    if (ret)
    goto err_cal;
    ret = clk_prepare_enable(uphy.cal_sleep_clk);
    if (ret)
    goto err_sleep;
// Set periodic calibration interval to ~2.048sec in HSIC_IO_CAL_REG
    ret = ulpi_write(ulpi, ULPI_HSIC_IO_CAL, 0xff);
    if (ret)
    goto err_ulpi;
// Enable periodic IO calibration in HSIC_CFG register
    ret = ulpi_write(ulpi, ULPI_HSIC_CFG, 0xa8);
    if (ret)
    goto err_ulpi;
// Configure pins for HSIC functionality
    pins_default = pinctrl_lookup_state(uphy.pctl, PINCTRL_STATE_DEFAULT);
    if (IS_ERR(pins_default)) {
    ret = PTR_ERR(pins_default);
    goto err_ulpi;
    }
    ret = pinctrl_select_state(uphy.pctl, pins_default);
    if (ret)
    goto err_ulpi;
// Enable HSIC mode in HSIC_CFG register
    ret = ulpi_write(ulpi, ULPI_SET(ULPI_HSIC_CFG), 0x01);
    if (ret)
    goto err_ulpi;
// Disable auto-resume
    ret = ulpi_write(ulpi, ULPI_CLR(ULPI_IFC_CTRL),
    ULPI_IFC_CTRL_AUTORESUME);
    if (ret)
    goto err_ulpi;
    return ret;
    err_ulpi:
    clk_disable_unprepare(uphy.cal_sleep_clk);
    err_sleep:
    clk_disable_unprepare(uphy.cal_clk);
    err_cal:
    clk_disable_unprepare(uphy.phy_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_usb_hsic_phy_power_off(phy: *mut phy) -> c_int {
    static int qcom_usb_hsic_phy_power_off(struct phy *phy)
    {
    struct qcom_usb_hsic_phy *uphy = phy_get_drvdata(phy);
    clk_disable_unprepare(uphy.cal_sleep_clk);
    clk_disable_unprepare(uphy.cal_clk);
    clk_disable_unprepare(uphy.phy_clk);
    return 0;
    }
    static const struct phy_ops qcom_usb_hsic_phy_ops = {
    .power_on = qcom_usb_hsic_phy_power_on,
    .power_off = qcom_usb_hsic_phy_power_off,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn qcom_usb_hsic_phy_probe(ulpi: *mut ulpi) -> c_int {
    static int qcom_usb_hsic_phy_probe(struct ulpi *ulpi)
    {
    struct qcom_usb_hsic_phy *uphy;
    struct phy_provider *p;
    struct clk *clk;
    uphy = devm_kzalloc(&ulpi.dev, sizeof(*uphy), GFP_KERNEL);
    if (!uphy)
    return -ENOMEM;
    ulpi_set_drvdata(ulpi, uphy);
    uphy.ulpi = ulpi;
    uphy.pctl = devm_pinctrl_get(&ulpi.dev);
    if (IS_ERR(uphy.pctl))
    return PTR_ERR(uphy.pctl);
    uphy.phy_clk = clk = devm_clk_get(&ulpi.dev, "phy");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    uphy.cal_clk = clk = devm_clk_get(&ulpi.dev, "cal");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    uphy.cal_sleep_clk = clk = devm_clk_get(&ulpi.dev, "cal_sleep");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    uphy.phy = devm_phy_create(&ulpi.dev, ulpi.dev.of_node,
    &qcom_usb_hsic_phy_ops);
    if (IS_ERR(uphy.phy))
    return PTR_ERR(uphy.phy);
    phy_set_drvdata(uphy.phy, uphy);
    p = devm_of_phy_provider_register(&ulpi.dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(p);
    }
    static const struct of_device_id qcom_usb_hsic_phy_match[] = {
    { .compatible = "qcom,usb-hsic-phy", },
    { }
    };
    MODULE_DEVICE_TABLE(of, qcom_usb_hsic_phy_match);
    static struct ulpi_driver qcom_usb_hsic_phy_driver = {
    .probe = qcom_usb_hsic_phy_probe,
    .driver = {
    .name = "qcom_usb_hsic_phy",
    .of_match_table = qcom_usb_hsic_phy_match,
    },
    };
    module_ulpi_driver(qcom_usb_hsic_phy_driver);
    MODULE_DESCRIPTION("Qualcomm USB HSIC phy");
    MODULE_LICENSE("GPL v2");
