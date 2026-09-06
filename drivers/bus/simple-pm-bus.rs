//! Automatically rewritten from C to Rust
//! Source: drivers/bus/simple-pm-bus.c
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
// Simple Power-Managed Bus Driver
//
// Copyright (C) 2014-2015 Glider bvba
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_pm_bus {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
}

#[no_mangle]
unsafe extern "C" fn simple_pm_bus_probe(pdev: *mut platform_device) -> c_int {
    static int simple_pm_bus_probe(struct platform_device *pdev)
    {
    const struct device *dev = &pdev.dev;
    const struct of_dev_auxdata *lookup = dev_get_platdata(dev);
    struct device_node *np = dev.of_node;
    const struct of_device_id *match;
    struct simple_pm_bus *bus;
//
// Allow user to use driver_override to bind this driver to a
// transparent bus device which has a different compatible string
// that's not listed in simple_pm_bus_of_match. We don't want to do any
// of the simple-pm-bus tasks for these devices, so return early.
//
    if (device_has_driver_override(&pdev.dev))
    return 0;
    match = of_match_device(dev.driver.of_match_table, dev);
//
// These are transparent bus devices (not simple-pm-bus matches) that
// have their child nodes populated automatically.  So, don't need to
// do anything more. We only match with the device if this driver is
// the most specific match because we don't want to incorrectly bind to
// a device that has a more specific driver.
//
    if (match && match.data) {
    if (of_property_match_string(np, "compatible", match.compatible) == 0)
    return 0;
    else
    return -ENODEV;
    }
    bus = devm_kzalloc(&pdev.dev, sizeof(*bus), GFP_KERNEL);
    if (!bus)
    return -ENOMEM;
    bus.num_clks = devm_clk_bulk_get_all(&pdev.dev, &bus.clks);
    if (bus.num_clks < 0)
    return dev_err_probe(&pdev.dev, bus.num_clks, "failed to get clocks\n");
    dev_set_drvdata(&pdev.dev, bus);
    dev_dbg(&pdev.dev, "%s\n", __func__);
    pm_runtime_enable(&pdev.dev);
    if (np)
    of_platform_populate(np, core::ptr::null_mut(), lookup, &pdev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn simple_pm_bus_remove(pdev: *mut platform_device) {
    static void simple_pm_bus_remove(struct platform_device *pdev)
    {
    const void *data = of_device_get_match_data(&pdev.dev);
    if (device_has_driver_override(&pdev.dev) || data)
    return;
    dev_dbg(&pdev.dev, "%s\n", __func__);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn simple_pm_bus_runtime_suspend(dev: *mut device) -> c_int {
    static int simple_pm_bus_runtime_suspend(struct device *dev)
    {
    struct simple_pm_bus *bus = dev_get_drvdata(dev);
    clk_bulk_disable_unprepare(bus.num_clks, bus.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn simple_pm_bus_runtime_resume(dev: *mut device) -> c_int {
    static int simple_pm_bus_runtime_resume(struct device *dev)
    {
    struct simple_pm_bus *bus = dev_get_drvdata(dev);
    int ret;
    ret = clk_bulk_prepare_enable(bus.num_clks, bus.clks);
    if (ret) {
    dev_err(dev, "failed to enable clocks: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn simple_pm_bus_suspend(dev: *mut device) -> c_int {
    static int simple_pm_bus_suspend(struct device *dev)
    {
    struct simple_pm_bus *bus = dev_get_drvdata(dev);
    if (!bus)
    return 0;
    return pm_runtime_force_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn simple_pm_bus_resume(dev: *mut device) -> c_int {
    static int simple_pm_bus_resume(struct device *dev)
    {
    struct simple_pm_bus *bus = dev_get_drvdata(dev);
    if (!bus)
    return 0;
    return pm_runtime_force_resume(dev);
    }
    static const struct dev_pm_ops simple_pm_bus_pm_ops = {
    RUNTIME_PM_OPS(simple_pm_bus_runtime_suspend, simple_pm_bus_runtime_resume, core::ptr::null_mut())
    NOIRQ_SYSTEM_SLEEP_PM_OPS(simple_pm_bus_suspend, simple_pm_bus_resume)
    };

    static const struct of_device_id simple_pm_bus_of_match[] = {
    { .compatible = "simple-pm-bus", },
    { .compatible = "simple-bus",	.data = ONLY_BUS },
    { .compatible = "simple-mfd",	.data = ONLY_BUS },
    { .compatible = "isa",		.data = ONLY_BUS },
    { .compatible = "arm,amba-bus",	.data = ONLY_BUS },
    { .compatible = "fsl,ls1021a-scfg", },
    { .compatible = "fsl,ls1043a-scfg", },
    { .compatible = "fsl,ls1046a-scfg", },
    { .compatible = "fsl,ls1088a-isc", },
    { .compatible = "fsl,ls2080a-isc", },
    { .compatible = "fsl,lx2160a-isc", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, simple_pm_bus_of_match);
    static struct platform_driver simple_pm_bus_driver = {
    .probe = simple_pm_bus_probe,
    .remove = simple_pm_bus_remove,
    .driver = {
    .name = "simple-pm-bus",
    .of_match_table = simple_pm_bus_of_match,
    .pm = pm_ptr(&simple_pm_bus_pm_ops),
    },
    };
    module_platform_driver(simple_pm_bus_driver);
    MODULE_DESCRIPTION("Simple Power-Managed Bus Driver");
    MODULE_AUTHOR("Geert Uytterhoeven <geert+renesas@glider.be>");
