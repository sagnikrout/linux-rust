//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/ccp/sp-platform.c
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
// AMD Secure Processor device driver
//
// Copyright (C) 2014,2018 Advanced Micro Devices, Inc.
//
// Author: Tom Lendacky <thomas.lendacky@amd.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_platform {
    pub coherent: c_int,
    pub irq_count: c_uint,
}

    static const struct sp_dev_vdata dev_vdata[] = {
    {
    .bar = 0,

    .ccp_vdata = &ccpv3_platform,

    },
    };
    static const struct acpi_device_id sp_acpi_match[] = {
    { "AMDI0C00", (kernel_ulong_t)&dev_vdata[0] },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, sp_acpi_match);
    static const struct of_device_id sp_of_match[] = {
    { .compatible = "amd,ccp-seattle-v1a",
    .data = (const void *)&dev_vdata[0] },
    { },
    };
    MODULE_DEVICE_TABLE(of, sp_of_match);
    static const struct sp_dev_vdata *sp_get_acpi_version(struct platform_device *pdev)
    {
    const struct acpi_device_id *match;
    match = acpi_match_device(sp_acpi_match, &pdev.dev);
    if (match && match.driver_data)
    return (const struct sp_dev_vdata *)match.driver_data;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sp_get_irqs(sp: *mut sp_device) -> c_int {
    static int sp_get_irqs(struct sp_device *sp)
    {
    struct sp_platform *sp_platform = sp.dev_specific;
    struct device *dev = sp.dev;
    struct platform_device *pdev = to_platform_device(dev);
    int ret;
    sp_platform.irq_count = platform_irq_count(pdev);
    ret = platform_get_irq(pdev, 0);
    if (ret < 0) {
    dev_notice(dev, "unable to get IRQ (%d)\n", ret);
    return ret;
    }
    sp.psp_irq = ret;
    if (sp_platform.irq_count == 1) {
    sp.ccp_irq = ret;
    } else {
    ret = platform_get_irq(pdev, 1);
    if (ret < 0) {
    dev_notice(dev, "unable to get IRQ (%d)\n", ret);
    return ret;
    }
    sp.ccp_irq = ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_platform_probe(pdev: *mut platform_device) -> c_int {
    static int sp_platform_probe(struct platform_device *pdev)
    {
    struct sp_device *sp;
    struct sp_platform *sp_platform;
    struct device *dev = &pdev.dev;
    enum dev_dma_attr attr;
    int ret;
    ret = -ENOMEM;
    sp = sp_alloc_struct(dev);
    if (!sp)
    goto e_err;
    sp_platform = devm_kzalloc(dev, sizeof(*sp_platform), GFP_KERNEL);
    if (!sp_platform)
    goto e_err;
    sp.dev_specific = sp_platform;
    sp.dev_vdata = pdev.dev.of_node ? of_device_get_match_data(&pdev.dev)
    : sp_get_acpi_version(pdev);
    if (!sp.dev_vdata) {
    ret = -ENODEV;
    dev_err(dev, "missing driver data\n");
    goto e_err;
    }
    sp.io_map = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(sp.io_map)) {
    ret = PTR_ERR(sp.io_map);
    goto e_err;
    }
    attr = device_get_dma_attr(dev);
    if (attr == DEV_DMA_NOT_SUPPORTED) {
    dev_err(dev, "DMA is not supported");
    goto e_err;
    }
    sp_platform.coherent = (attr == DEV_DMA_COHERENT);
    if (sp_platform.coherent)
    sp.axcache = CACHE_WB_NO_ALLOC;
    else
    sp.axcache = CACHE_NONE;
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(48));
    if (ret) {
    dev_err(dev, "dma_set_mask_and_coherent failed (%d)\n", ret);
    goto e_err;
    }
    ret = sp_get_irqs(sp);
    if (ret)
    goto e_err;
    dev_set_drvdata(dev, sp);
    ret = sp_init(sp);
    if (ret)
    goto e_err;
    dev_notice(dev, "enabled\n");
    return 0;
    e_err:
    dev_notice(dev, "initialization failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sp_platform_remove(pdev: *mut platform_device) {
    static void sp_platform_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sp_device *sp = dev_get_drvdata(dev);
    sp_destroy(sp);
    dev_notice(dev, "disabled\n");
    }

    static int sp_platform_suspend(struct platform_device *pdev,
    pm_message_t state)
    {
    struct device *dev = &pdev.dev;
    struct sp_device *sp = dev_get_drvdata(dev);
    return sp_suspend(sp);
    }
#[no_mangle]
unsafe extern "C" fn sp_platform_resume(pdev: *mut platform_device) -> c_int {
    static int sp_platform_resume(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sp_device *sp = dev_get_drvdata(dev);
    return sp_resume(sp);
    }

    static struct platform_driver sp_platform_driver = {
    .driver = {
    .name = "ccp",
    .acpi_match_table = sp_acpi_match,
    .of_match_table = sp_of_match,
    },
    .probe = sp_platform_probe,
    .remove = sp_platform_remove,

    .suspend = sp_platform_suspend,
    .resume = sp_platform_resume,

    };
#[no_mangle]
pub unsafe extern "C" fn sp_platform_init() -> c_int {
    int sp_platform_init(void)
    {
    return platform_driver_register(&sp_platform_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn sp_platform_exit() {
    void sp_platform_exit(void)
    {
    platform_driver_unregister(&sp_platform_driver);
    }
