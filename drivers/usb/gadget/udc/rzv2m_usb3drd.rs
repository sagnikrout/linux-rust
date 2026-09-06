//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/udc/rzv2m_usb3drd.c
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
// Renesas RZ/V2M USB3DRD driver
//
// Copyright (C) 2022 Renesas Electronics Corporation
//

pub const USB_PERI_DRD_CON: c_uint = 0x000;

    static void rzv2m_usb3drd_set_bit(struct rzv2m_usb3drd *usb3, u32 bits,
    u32 offs)
    {
    let mut val: u32 = readl(usb3.reg + offs);
    val |= bits;
    writel(val, usb3.reg + offs);
    }
    static void rzv2m_usb3drd_clear_bit(struct rzv2m_usb3drd *usb3, u32 bits,
    u32 offs)
    {
    let mut val: u32 = readl(usb3.reg + offs);
    val &= ~bits;
    writel(val, usb3.reg + offs);
    }
#[no_mangle]
pub unsafe extern "C" fn rzv2m_usb3drd_reset(dev: *mut device, host: bool) {
    void rzv2m_usb3drd_reset(struct device *dev, bool host)
    {
    struct rzv2m_usb3drd *usb3 = dev_get_drvdata(dev);
    if (host) {
    rzv2m_usb3drd_clear_bit(usb3, USB_PERI_DRD_CON_PERI_CON,
    USB_PERI_DRD_CON);
    rzv2m_usb3drd_clear_bit(usb3, USB_PERI_DRD_CON_HOST_RST,
    USB_PERI_DRD_CON);
    rzv2m_usb3drd_set_bit(usb3, USB_PERI_DRD_CON_PERI_RST,
    USB_PERI_DRD_CON);
    } else {
    rzv2m_usb3drd_set_bit(usb3, USB_PERI_DRD_CON_PERI_CON,
    USB_PERI_DRD_CON);
    rzv2m_usb3drd_set_bit(usb3, USB_PERI_DRD_CON_HOST_RST,
    USB_PERI_DRD_CON);
    rzv2m_usb3drd_clear_bit(usb3, USB_PERI_DRD_CON_PERI_RST,
    USB_PERI_DRD_CON);
    }
    }
    EXPORT_SYMBOL_GPL(rzv2m_usb3drd_reset);
#[no_mangle]
unsafe extern "C" fn rzv2m_usb3drd_remove(pdev: *mut platform_device) {
    static void rzv2m_usb3drd_remove(struct platform_device *pdev)
    {
    struct rzv2m_usb3drd *usb3 = platform_get_drvdata(pdev);
    of_platform_depopulate(usb3.dev);
    pm_runtime_put(usb3.dev);
    pm_runtime_disable(&pdev.dev);
    reset_control_assert(usb3.drd_rstc);
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_usb3drd_probe(pdev: *mut platform_device) -> c_int {
    static int rzv2m_usb3drd_probe(struct platform_device *pdev)
    {
    struct rzv2m_usb3drd *usb3;
    int ret;
    usb3 = devm_kzalloc(&pdev.dev, sizeof(*usb3), GFP_KERNEL);
    if (!usb3)
    return -ENOMEM;
    usb3.dev = &pdev.dev;
    usb3.drd_irq = platform_get_irq_byname(pdev, "drd");
    if (usb3.drd_irq < 0)
    return usb3.drd_irq;
    usb3.reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(usb3.reg))
    return PTR_ERR(usb3.reg);
    platform_set_drvdata(pdev, usb3);
    usb3.drd_rstc = devm_reset_control_get_exclusive(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(usb3.drd_rstc))
    return dev_err_probe(&pdev.dev, PTR_ERR(usb3.drd_rstc),
    "failed to get drd reset");
    reset_control_deassert(usb3.drd_rstc);
    pm_runtime_enable(&pdev.dev);
    ret = pm_runtime_resume_and_get(usb3.dev);
    if (ret)
    goto err_rst;
    ret = of_platform_populate(usb3.dev.of_node, core::ptr::null_mut(), core::ptr::null_mut(), usb3.dev);
    if (ret)
    goto err_pm;
    return 0;
    err_pm:
    pm_runtime_put(usb3.dev);
    err_rst:
    pm_runtime_disable(&pdev.dev);
    reset_control_assert(usb3.drd_rstc);
    return ret;
    }
    static const struct of_device_id rzv2m_usb3drd_of_match[] = {
    { .compatible = "renesas,rzv2m-usb3drd", },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rzv2m_usb3drd_of_match);
    static struct platform_driver rzv2m_usb3drd_driver = {
    .driver = {
    .name = "rzv2m-usb3drd",
    .of_match_table = rzv2m_usb3drd_of_match,
    },
    .probe = rzv2m_usb3drd_probe,
    .remove = rzv2m_usb3drd_remove,
    };
    module_platform_driver(rzv2m_usb3drd_driver);
    MODULE_AUTHOR("Biju Das <biju.das.jz@bp.renesas.com>");
    MODULE_DESCRIPTION("Renesas RZ/V2M USB3DRD driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:rzv2m_usb3drd");
