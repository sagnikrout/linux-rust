//! Automatically rewritten from C to Rust
//! Source: drivers/ufs/host/ti-j721e-ufs.c
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
// Copyright (C) 2019 Texas Instruments Incorporated - http://www.ti.com
//

pub const TI_UFS_SS_CTRL: c_uint = 0x4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_j721e_ufs {
    pub regbase: *mut void __iomem,
    pub reg: u32,
}

#[no_mangle]
unsafe extern "C" fn ti_j721e_ufs_probe(pdev: *mut platform_device) -> c_int {
    static int ti_j721e_ufs_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ti_j721e_ufs *ufs;
    unsigned long clk_rate;
    struct clk *clk;
    int ret;
    ufs = devm_kzalloc(dev, sizeof(*ufs), GFP_KERNEL);
    if (!ufs)
    return -ENOMEM;
    ufs.regbase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ufs.regbase))
    return PTR_ERR(ufs.regbase);
    pm_runtime_enable(dev);
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0)
    goto disable_pm;
// Select MPHY refclk frequency
    clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    dev_err(dev, "Cannot claim MPHY clock.\n");
    goto clk_err;
    }
    clk_rate = clk_get_rate(clk);
    if (clk_rate == 26000000)
    ufs.reg |= TI_UFS_SS_CLK_26MHZ;
    devm_clk_put(dev, clk);
// Take UFS slave device out of reset
    ufs.reg |= TI_UFS_SS_RST_N_PCS;
    writel(ufs.reg, ufs.regbase + TI_UFS_SS_CTRL);
    dev_set_drvdata(dev, ufs);
    ret = of_platform_populate(pdev.dev.of_node, core::ptr::null_mut(), core::ptr::null_mut(),
    dev);
    if (ret) {
    dev_err(dev, "failed to populate child nodes %d\n", ret);
    goto clk_err;
    }
    return ret;
    clk_err:
    pm_runtime_put_sync(dev);
    disable_pm:
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ti_j721e_ufs_remove(pdev: *mut platform_device) {
    static void ti_j721e_ufs_remove(struct platform_device *pdev)
    {
    of_platform_depopulate(&pdev.dev);
    pm_runtime_put_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn ti_j721e_ufs_resume(dev: *mut device) -> c_int {
    static int ti_j721e_ufs_resume(struct device *dev)
    {
    struct ti_j721e_ufs *ufs = dev_get_drvdata(dev);
    writel(ufs.reg, ufs.regbase + TI_UFS_SS_CTRL);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ti_j721e_ufs_pm_ops, core::ptr::null_mut(), ti_j721e_ufs_resume);
    static const struct of_device_id ti_j721e_ufs_of_match[] = {
    {
    .compatible = "ti,j721e-ufs",
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, ti_j721e_ufs_of_match);
    static struct platform_driver ti_j721e_ufs_driver = {
    .probe	= ti_j721e_ufs_probe,
    .remove = ti_j721e_ufs_remove,
    .driver	= {
    .name   = "ti-j721e-ufs",
    .of_match_table = ti_j721e_ufs_of_match,
    .pm = pm_sleep_ptr(&ti_j721e_ufs_pm_ops),
    },
    };
    module_platform_driver(ti_j721e_ufs_driver);
    MODULE_AUTHOR("Vignesh Raghavendra <vigneshr@ti.com>");
    MODULE_DESCRIPTION("TI UFS host controller glue driver");
    MODULE_LICENSE("GPL v2");
