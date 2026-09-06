//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-gic-pm.c
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
// Copyright (C) 2016 NVIDIA CORPORATION, All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gic_clk_data {
    pub num_clocks: c_uint,
    pub clocks: *const *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gic_chip_pm {
    pub chip_data: *mut gic_chip_data,
    pub clk_data: *const gic_clk_data,
    pub clks: *mut clk_bulk_data,
}

#[no_mangle]
unsafe extern "C" fn gic_runtime_resume(dev: *mut device) -> c_int {
    static int gic_runtime_resume(struct device *dev)
    {
    struct gic_chip_pm *chip_pm = dev_get_drvdata(dev);
    struct gic_chip_data *gic = chip_pm.chip_data;
    const struct gic_clk_data *data = chip_pm.clk_data;
    int ret;
    ret = clk_bulk_prepare_enable(data.num_clocks, chip_pm.clks);
    if (ret)
    return ret;
//
// On the very first resume, the pointer to chip_pm->chip_data
// will be NULL and this is intentional, because we do not
// want to restore the GIC on the very first resume. So if
// the pointer is not valid just return.
//
    if (!gic)
    return 0;
    gic_dist_restore(gic);
    gic_cpu_restore(gic);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gic_runtime_suspend(dev: *mut device) -> c_int {
    static int gic_runtime_suspend(struct device *dev)
    {
    struct gic_chip_pm *chip_pm = dev_get_drvdata(dev);
    struct gic_chip_data *gic = chip_pm.chip_data;
    const struct gic_clk_data *data = chip_pm.clk_data;
    gic_dist_save(gic);
    gic_cpu_save(gic);
    clk_bulk_disable_unprepare(data.num_clocks, chip_pm.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gic_probe(pdev: *mut platform_device) -> c_int {
    static int gic_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct gic_clk_data *data;
    struct gic_chip_pm *chip_pm;
    int ret, irq, i;
    data = of_device_get_match_data(&pdev.dev);
    if (!data) {
    dev_err(&pdev.dev, "no device match found\n");
    return -ENODEV;
    }
    chip_pm = devm_kzalloc(dev, sizeof(*chip_pm), GFP_KERNEL);
    if (!chip_pm)
    return -ENOMEM;
    irq = irq_of_parse_and_map(dev.of_node, 0);
    if (!irq) {
    dev_err(dev, "no parent interrupt found!\n");
    return -EINVAL;
    }
    chip_pm.clks = devm_kcalloc(dev, data.num_clocks,
    sizeof(*chip_pm.clks), GFP_KERNEL);
    if (!chip_pm.clks)
    return -ENOMEM;
    for (i = 0; i < data.num_clocks; i++)
    chip_pm.clks[i].id = data.clocks[i];
    ret = devm_clk_bulk_get(dev, data.num_clocks, chip_pm.clks);
    if (ret)
    goto irq_dispose;
    chip_pm.clk_data = data;
    dev_set_drvdata(dev, chip_pm);
    pm_runtime_enable(dev);
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0)
    goto rpm_disable;
    ret = gic_of_init_child(dev, &chip_pm.chip_data, irq);
    if (ret)
    goto rpm_put;
    pm_runtime_put(dev);
    dev_info(dev, "GIC IRQ controller registered\n");
    return 0;
    rpm_put:
    pm_runtime_put_sync(dev);
    rpm_disable:
    pm_runtime_disable(dev);
    irq_dispose:
    irq_dispose_mapping(irq);
    return ret;
    }
    static const struct dev_pm_ops gic_pm_ops = {
    SET_RUNTIME_PM_OPS(gic_runtime_suspend,
    gic_runtime_resume, core::ptr::null_mut())
    SET_LATE_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    };
    static const char * const gic400_clocks[] = {
    "clk",
    };
    static const struct gic_clk_data gic400_data = {
    .num_clocks = ARRAY_SIZE(gic400_clocks),
    .clocks = gic400_clocks,
    };
    static const struct of_device_id gic_match[] = {
    { .compatible = "nvidia,tegra210-agic",	.data = &gic400_data },
    {},
    };
    MODULE_DEVICE_TABLE(of, gic_match);
    static struct platform_driver gic_driver = {
    .probe		= gic_probe,
    .driver		= {
    .name	= "gic",
    .of_match_table	= gic_match,
    .pm	= &gic_pm_ops,
    }
    };
    builtin_platform_driver(gic_driver);
