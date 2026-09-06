//! Automatically rewritten from C to Rust
//! Source: drivers/bus/imx-aipstz.c
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
// Copyright 2025 NXP
//

pub const IMX_AIPSTZ_MPR0: c_uint = 0x0;
pub const IMX_AIPSTZ_OPACR0: c_uint = 0x40;
pub const IMX_AIPSTZ_OPACR1: c_uint = 0x44;
pub const IMX_AIPSTZ_OPACR2: c_uint = 0x48;
pub const IMX_AIPSTZ_OPACR3: c_uint = 0x4c;
pub const IMX_AIPSTZ_OPACR4: c_uint = 0x50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_aipstz_config {
    pub mpr0: u32,
    pub opacr0: u32,
    pub opacr1: u32,
    pub opacr2: u32,
    pub opacr3: u32,
    pub opacr4: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_aipstz_data {
    pub base: *mut void __iomem,
    pub default_cfg: *const imx_aipstz_config,
}

#[no_mangle]
unsafe extern "C" fn imx_aipstz_apply_default(data: *mut imx_aipstz_data) {
    static void imx_aipstz_apply_default(struct imx_aipstz_data *data)
    {
    writel(data.default_cfg.mpr0, data.base + IMX_AIPSTZ_MPR0);
    writel(data.default_cfg.opacr0, data.base + IMX_AIPSTZ_OPACR0);
    writel(data.default_cfg.opacr1, data.base + IMX_AIPSTZ_OPACR1);
    writel(data.default_cfg.opacr2, data.base + IMX_AIPSTZ_OPACR2);
    writel(data.default_cfg.opacr3, data.base + IMX_AIPSTZ_OPACR3);
    writel(data.default_cfg.opacr4, data.base + IMX_AIPSTZ_OPACR4);
    }
    static const struct of_device_id imx_aipstz_match_table[] = {
    { .compatible = "simple-bus", },
    { }
    };
#[no_mangle]
unsafe extern "C" fn imx_aipstz_probe(pdev: *mut platform_device) -> c_int {
    static int imx_aipstz_probe(struct platform_device *pdev)
    {
    struct imx_aipstz_data *data;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return dev_err_probe(&pdev.dev, -ENOMEM,
    "failed to allocate data memory\n");
    data.base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(data.base))
    return dev_err_probe(&pdev.dev, -ENOMEM,
    "failed to get/ioremap AC memory\n");
    data.default_cfg = of_device_get_match_data(&pdev.dev);
    imx_aipstz_apply_default(data);
    dev_set_drvdata(&pdev.dev, data);
    pm_runtime_set_active(&pdev.dev);
    devm_pm_runtime_enable(&pdev.dev);
    return of_platform_populate(pdev.dev.of_node, imx_aipstz_match_table,
    core::ptr::null_mut(), &pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn imx_aipstz_remove(pdev: *mut platform_device) {
    static void imx_aipstz_remove(struct platform_device *pdev)
    {
    of_platform_depopulate(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn imx_aipstz_runtime_resume(dev: *mut device) -> c_int {
    static int imx_aipstz_runtime_resume(struct device *dev)
    {
    struct imx_aipstz_data *data = dev_get_drvdata(dev);
// restore potentially lost configuration during domain power-off
    imx_aipstz_apply_default(data);
    return 0;
    }
    static const struct dev_pm_ops imx_aipstz_pm_ops = {
    RUNTIME_PM_OPS(core::ptr::null_mut(), imx_aipstz_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
    };
//
// following configuration is equivalent to:
// masters 0-7 => trusted for R/W + use AHB's HPROT[1] to det. privilege
//
    static const struct imx_aipstz_config imx8mp_aipstz_default_cfg = {
    .mpr0 = 0x77777777,
    };
    static const struct of_device_id imx_aipstz_of_ids[] = {
    { .compatible = "fsl,imx8mp-aipstz", .data = &imx8mp_aipstz_default_cfg },
    { }
    };
    MODULE_DEVICE_TABLE(of, imx_aipstz_of_ids);
    static struct platform_driver imx_aipstz_of_driver = {
    .probe = imx_aipstz_probe,
    .remove = imx_aipstz_remove,
    .driver = {
    .name = "imx-aipstz",
    .of_match_table = imx_aipstz_of_ids,
    .pm = pm_ptr(&imx_aipstz_pm_ops),
    },
    };
    module_platform_driver(imx_aipstz_of_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("IMX secure AHB to IP Slave bus (AIPSTZ) bridge driver");
    MODULE_AUTHOR("Laurentiu Mihalcea <laurentiu.mihalcea@nxp.com>");
