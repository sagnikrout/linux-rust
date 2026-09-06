//! Automatically rewritten from C to Rust
//! Source: drivers/usb/chipidea/ci_hdrc_usb2.c
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
// Copyright (C) 2014 Marvell Technology Group Ltd.
//
// Antoine Tenart <antoine.tenart@free-electrons.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_hdrc_usb2_priv {
    pub ci_pdev: *mut platform_device,
    pub clk: *mut clk,
}

    static const struct ci_hdrc_platform_data ci_default_pdata = {
    .capoffset	= DEF_CAPOFFSET,
    .flags		= CI_HDRC_DISABLE_STREAMING,
    };
    static const struct ci_hdrc_platform_data ci_zynq_pdata = {
    .capoffset	= DEF_CAPOFFSET,
    .flags          = CI_HDRC_PHY_VBUS_CONTROL,
    };
    static const struct ci_hdrc_platform_data ci_zevio_pdata = {
    .capoffset	= DEF_CAPOFFSET,
    .flags		= CI_HDRC_REGS_SHARED | CI_HDRC_FORCE_FULLSPEED,
    };
    static const struct of_device_id ci_hdrc_usb2_of_match[] = {
    { .compatible = "chipidea,usb2" },
    { .compatible = "xlnx,zynq-usb-2.20a", .data = &ci_zynq_pdata },
    { .compatible = "lsi,zevio-usb", .data = &ci_zevio_pdata },
    { }
    };
    MODULE_DEVICE_TABLE(of, ci_hdrc_usb2_of_match);
#[no_mangle]
unsafe extern "C" fn ci_hdrc_usb2_probe(pdev: *mut platform_device) -> c_int {
    static int ci_hdrc_usb2_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ci_hdrc_usb2_priv *priv;
    struct ci_hdrc_platform_data *ci_pdata = dev_get_platdata(dev);
    const struct ci_hdrc_platform_data *data;
    int ret;
    if (!ci_pdata) {
    ci_pdata = devm_kmalloc(dev, sizeof(*ci_pdata), GFP_KERNEL);
    if (!ci_pdata)
    return -ENOMEM;
// ci_pdata = ci_default_pdata;	/* struct copy
    }
    data = device_get_match_data(&pdev.dev);
    if (data)
// struct copy
// ci_pdata = *data;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.clk = devm_clk_get_optional(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    ret = clk_prepare_enable(priv.clk);
    if (ret) {
    dev_err(dev, "failed to enable the clock: %d\n", ret);
    return ret;
    }
    ci_pdata.name = dev_name(dev);
    priv.ci_pdev = ci_hdrc_add_device(dev, pdev.resource,
    pdev.num_resources, ci_pdata);
    if (IS_ERR(priv.ci_pdev)) {
    ret = PTR_ERR(priv.ci_pdev);
    if (ret != -EPROBE_DEFER)
    dev_err(dev,
    "failed to register ci_hdrc platform device: %d\n",
    ret);
    goto clk_err;
    }
    platform_set_drvdata(pdev, priv);
    pm_runtime_no_callbacks(dev);
    pm_runtime_enable(dev);
    return 0;
    clk_err:
    clk_disable_unprepare(priv.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ci_hdrc_usb2_remove(pdev: *mut platform_device) {
    static void ci_hdrc_usb2_remove(struct platform_device *pdev)
    {
    struct ci_hdrc_usb2_priv *priv = platform_get_drvdata(pdev);
    pm_runtime_disable(&pdev.dev);
    ci_hdrc_remove_device(priv.ci_pdev);
    clk_disable_unprepare(priv.clk);
    }
    static struct platform_driver ci_hdrc_usb2_driver = {
    .probe	= ci_hdrc_usb2_probe,
    .remove = ci_hdrc_usb2_remove,
    .driver	= {
    .name		= "chipidea-usb2",
    .of_match_table	= ci_hdrc_usb2_of_match,
    },
    };
    module_platform_driver(ci_hdrc_usb2_driver);
    MODULE_DESCRIPTION("ChipIdea HDRC USB2 binding for ci13xxx");
    MODULE_AUTHOR("Antoine Tenart <antoine.tenart@free-electrons.com>");
    MODULE_LICENSE("GPL");
