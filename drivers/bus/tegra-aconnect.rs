//! Automatically rewritten from C to Rust
//! Source: drivers/bus/tegra-aconnect.c
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


//
// Tegra ACONNECT Bus Driver
//
// Copyright (C) 2016, NVIDIA CORPORATION.  All rights reserved.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_aconnect {
    pub ape_clk: *mut clk,
    pub apb2ape_clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn tegra_aconnect_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_aconnect_probe(struct platform_device *pdev)
    {
    struct tegra_aconnect *aconnect;
    if (!pdev.dev.of_node)
    return -EINVAL;
    aconnect = devm_kzalloc(&pdev.dev, sizeof(struct tegra_aconnect),
    GFP_KERNEL);
    if (!aconnect)
    return -ENOMEM;
    aconnect.ape_clk = devm_clk_get(&pdev.dev, "ape");
    if (IS_ERR(aconnect.ape_clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(aconnect.ape_clk),
    "can't retrieve ape clock\n");
    aconnect.apb2ape_clk = devm_clk_get(&pdev.dev, "apb2ape");
    if (IS_ERR(aconnect.apb2ape_clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(aconnect.apb2ape_clk),
    "can't retrieve apb2ape clock\n");
    dev_set_drvdata(&pdev.dev, aconnect);
    pm_runtime_enable(&pdev.dev);
    of_platform_populate(pdev.dev.of_node, core::ptr::null_mut(), core::ptr::null_mut(), &pdev.dev);
    dev_info(&pdev.dev, "Tegra ACONNECT bus registered\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_aconnect_remove(pdev: *mut platform_device) {
    static void tegra_aconnect_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn tegra_aconnect_runtime_resume(dev: *mut device) -> c_int {
    static int tegra_aconnect_runtime_resume(struct device *dev)
    {
    struct tegra_aconnect *aconnect = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(aconnect.ape_clk);
    if (ret) {
    dev_err(dev, "ape clk_enable failed: %d\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(aconnect.apb2ape_clk);
    if (ret) {
    clk_disable_unprepare(aconnect.ape_clk);
    dev_err(dev, "apb2ape clk_enable failed: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_aconnect_runtime_suspend(dev: *mut device) -> c_int {
    static int tegra_aconnect_runtime_suspend(struct device *dev)
    {
    struct tegra_aconnect *aconnect = dev_get_drvdata(dev);
    clk_disable_unprepare(aconnect.ape_clk);
    clk_disable_unprepare(aconnect.apb2ape_clk);
    return 0;
    }
    static const struct dev_pm_ops tegra_aconnect_pm_ops = {
    SET_RUNTIME_PM_OPS(tegra_aconnect_runtime_suspend,
    tegra_aconnect_runtime_resume, core::ptr::null_mut())
    SET_NOIRQ_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    };
    static const struct of_device_id tegra_aconnect_of_match[] = {
    { .compatible = "nvidia,tegra210-aconnect", },
    { }
    };
    MODULE_DEVICE_TABLE(of, tegra_aconnect_of_match);
    static struct platform_driver tegra_aconnect_driver = {
    .probe = tegra_aconnect_probe,
    .remove = tegra_aconnect_remove,
    .driver = {
    .name = "tegra-aconnect",
    .of_match_table = tegra_aconnect_of_match,
    .pm = &tegra_aconnect_pm_ops,
    },
    };
    module_platform_driver(tegra_aconnect_driver);
    MODULE_DESCRIPTION("NVIDIA Tegra ACONNECT Bus Driver");
    MODULE_AUTHOR("Jon Hunter <jonathanh@nvidia.com>");
    MODULE_LICENSE("GPL v2");
