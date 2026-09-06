//! Automatically rewritten from C to Rust
//! Source: drivers/usb/cdns3/cdns3-starfive.c
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
// cdns3-starfive.c - StarFive specific Glue layer for Cadence USB Controller
//
// Copyright (C) 2023 StarFive Technology Co., Ltd.
//
// Author:	Minda Chen <minda.chen@starfivetech.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_starfive {
    pub dev: *mut device,
    pub stg_syscon: *mut regmap,
    pub resets: *mut reset_control,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub stg_usb_mode: u32,
}

    static void cdns_mode_init(struct platform_device *pdev,
    struct cdns_starfive *data)
    {
    enum usb_dr_mode mode;
    regmap_update_bits(data.stg_syscon, data.stg_usb_mode,
    USB_MISC_CFG_MASK,
    USB_SUSPENDM_BYPS | USB_PLL_EN | USB_REFCLK_MODE);
// dr mode setting
    mode = usb_get_dr_mode(&pdev.dev);
    switch (mode) {
    case USB_DR_MODE_HOST:
    regmap_update_bits(data.stg_syscon,
    data.stg_usb_mode,
    USB_STRAP_MASK,
    USB_STRAP_HOST);
    regmap_update_bits(data.stg_syscon,
    data.stg_usb_mode,
    USB_SUSPENDM_MASK,
    USB_SUSPENDM_HOST);
    break;
    case USB_DR_MODE_PERIPHERAL:
    regmap_update_bits(data.stg_syscon, data.stg_usb_mode,
    USB_STRAP_MASK, USB_STRAP_DEVICE);
    regmap_update_bits(data.stg_syscon, data.stg_usb_mode,
    USB_SUSPENDM_MASK, 0);
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn cdns_clk_rst_init(data: *mut cdns_starfive) -> c_int {
    static int cdns_clk_rst_init(struct cdns_starfive *data)
    {
    int ret;
    ret = clk_bulk_prepare_enable(data.num_clks, data.clks);
    if (ret)
    return dev_err_probe(data.dev, ret,
    "failed to enable clocks\n");
    ret = reset_control_deassert(data.resets);
    if (ret) {
    dev_err(data.dev, "failed to reset clocks\n");
    goto err_clk_init;
    }
    return ret;
    err_clk_init:
    clk_bulk_disable_unprepare(data.num_clks, data.clks);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_clk_rst_deinit(data: *mut cdns_starfive) {
    static void cdns_clk_rst_deinit(struct cdns_starfive *data)
    {
    reset_control_assert(data.resets);
    clk_bulk_disable_unprepare(data.num_clks, data.clks);
    }
#[no_mangle]
unsafe extern "C" fn cdns_starfive_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_starfive_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cdns_starfive *data;
    unsigned int args;
    int ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.dev = dev;
    data.stg_syscon =
    syscon_regmap_lookup_by_phandle_args(pdev.dev.of_node,
    "starfive,stg-syscon", 1, &args);
    if (IS_ERR(data.stg_syscon))
    return dev_err_probe(dev, PTR_ERR(data.stg_syscon),
    "Failed to parse starfive,stg-syscon\n");
    data.stg_usb_mode = args;
    data.num_clks = devm_clk_bulk_get_all(data.dev, &data.clks);
    if (data.num_clks < 0)
    return dev_err_probe(data.dev, -ENODEV,
    "Failed to get clocks\n");
    data.resets = devm_reset_control_array_get_exclusive(data.dev);
    if (IS_ERR(data.resets))
    return dev_err_probe(data.dev, PTR_ERR(data.resets),
    "Failed to get resets");
    cdns_mode_init(pdev, data);
    ret = cdns_clk_rst_init(data);
    if (ret)
    return ret;
    ret = of_platform_populate(dev.of_node, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    if (ret) {
    dev_err(dev, "Failed to create children\n");
    cdns_clk_rst_deinit(data);
    return ret;
    }
    device_set_wakeup_capable(dev, true);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    platform_set_drvdata(pdev, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_starfive_remove_core(dev: *mut device, c: *mut c_void) -> c_int {
    static int cdns_starfive_remove_core(struct device *dev, void *c)
    {
    struct platform_device *pdev = to_platform_device(dev);
    platform_device_unregister(pdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_starfive_remove(pdev: *mut platform_device) {
    static void cdns_starfive_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cdns_starfive *data = dev_get_drvdata(dev);
    pm_runtime_get_sync(dev);
    device_for_each_child(dev, core::ptr::null_mut(), cdns_starfive_remove_core);
    pm_runtime_disable(dev);
    pm_runtime_put_noidle(dev);
    cdns_clk_rst_deinit(data);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    }

#[no_mangle]
unsafe extern "C" fn cdns_starfive_runtime_resume(dev: *mut device) -> c_int {
    static int cdns_starfive_runtime_resume(struct device *dev)
    {
    struct cdns_starfive *data = dev_get_drvdata(dev);
    return clk_bulk_prepare_enable(data.num_clks, data.clks);
    }
#[no_mangle]
unsafe extern "C" fn cdns_starfive_runtime_suspend(dev: *mut device) -> c_int {
    static int cdns_starfive_runtime_suspend(struct device *dev)
    {
    struct cdns_starfive *data = dev_get_drvdata(dev);
    clk_bulk_disable_unprepare(data.num_clks, data.clks);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn cdns_starfive_resume(dev: *mut device) -> c_int {
    static int cdns_starfive_resume(struct device *dev)
    {
    struct cdns_starfive *data = dev_get_drvdata(dev);
    return cdns_clk_rst_init(data);
    }
#[no_mangle]
unsafe extern "C" fn cdns_starfive_suspend(dev: *mut device) -> c_int {
    static int cdns_starfive_suspend(struct device *dev)
    {
    struct cdns_starfive *data = dev_get_drvdata(dev);
    cdns_clk_rst_deinit(data);
    return 0;
    }

    static const struct dev_pm_ops cdns_starfive_pm_ops = {
    SET_RUNTIME_PM_OPS(cdns_starfive_runtime_suspend,
    cdns_starfive_runtime_resume, core::ptr::null_mut())
    SET_SYSTEM_SLEEP_PM_OPS(cdns_starfive_suspend, cdns_starfive_resume)
    };
    static const struct of_device_id cdns_starfive_of_match[] = {
    { .compatible = "starfive,jh7110-usb", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, cdns_starfive_of_match);
    static struct platform_driver cdns_starfive_driver = {
    .probe		= cdns_starfive_probe,
    .remove		= cdns_starfive_remove,
    .driver		= {
    .name	= "cdns3-starfive",
    .of_match_table	= cdns_starfive_of_match,
    .pm	= &cdns_starfive_pm_ops,
    },
    };
    module_platform_driver(cdns_starfive_driver);
    MODULE_ALIAS("platform:cdns3-starfive");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Cadence USB3 StarFive Glue Layer");
