//! Automatically rewritten from C to Rust
//! Source: drivers/clk/samsung/clk-exynos-clkout.c
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
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Author: Tomasz Figa <t.figa@samsung.com>
//
// Clock driver for Exynos clock output
//

pub const EXYNOS_CLKOUT_NR_CLKS: c_int = 1;
pub const EXYNOS_CLKOUT_PARENTS: c_int = 32;
pub const EXYNOS_PMU_DEBUG_REG: c_uint = 0xa00;
pub const EXYNOS_CLKOUT_DISABLE_SHIFT: c_int = 0;
pub const EXYNOS_CLKOUT_MUX_SHIFT: c_int = 8;
pub const EXYNOS4_CLKOUT_MUX_MASK: c_uint = 0xf;
pub const EXYNOS5_CLKOUT_MUX_MASK: c_uint = 0x1f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_clkout {
    pub gate: clk_gate,
    pub mux: clk_mux,
    pub slock: spinlock_t,
    pub reg: *mut void __iomem,
    pub np: *mut device_node,
    pub pmu_debug_save: u32,
    pub data: clk_hw_onecell_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_clkout_variant {
    pub mux_mask: u32,
}

    static const struct exynos_clkout_variant exynos_clkout_exynos4 = {
    .mux_mask	= EXYNOS4_CLKOUT_MUX_MASK,
    };
    static const struct exynos_clkout_variant exynos_clkout_exynos5 = {
    .mux_mask	= EXYNOS5_CLKOUT_MUX_MASK,
    };
    static const struct of_device_id exynos_clkout_ids[] = {
    {
    .compatible = "samsung,exynos3250-pmu",
    .data = &exynos_clkout_exynos4,
    }, {
    .compatible = "samsung,exynos4210-pmu",
    .data = &exynos_clkout_exynos4,
    }, {
    .compatible = "samsung,exynos4212-pmu",
    .data = &exynos_clkout_exynos4,
    }, {
    .compatible = "samsung,exynos4412-pmu",
    .data = &exynos_clkout_exynos4,
    }, {
    .compatible = "samsung,exynos5250-pmu",
    .data = &exynos_clkout_exynos5,
    }, {
    .compatible = "samsung,exynos5410-pmu",
    .data = &exynos_clkout_exynos5,
    }, {
    .compatible = "samsung,exynos5420-pmu",
    .data = &exynos_clkout_exynos5,
    }, {
    .compatible = "samsung,exynos5433-pmu",
    .data = &exynos_clkout_exynos5,
    }, { }
    };
//
// Device will be instantiated as child of PMU device without its own
// device node.  Therefore match compatibles against parent.
//
#[no_mangle]
unsafe extern "C" fn exynos_clkout_match_parent_dev(dev: *mut device, mux_mask: *mut u32) -> c_int {
    static int exynos_clkout_match_parent_dev(struct device *dev, u32 *mux_mask)
    {
    const struct exynos_clkout_variant *variant;
    const struct of_device_id *match;
    if (!dev.parent) {
    dev_err(dev, "not instantiated from MFD\n");
    return -EINVAL;
    }
//
// 'exynos_clkout_ids' arrays is not the ids array matched by
// the dev->parent driver, so of_device_get_match_data() or
// device_get_match_data() cannot be used here.
//
    match = of_match_device(exynos_clkout_ids, dev.parent);
    if (!match) {
    dev_err(dev, "cannot match parent device\n");
    return -EINVAL;
    }
    variant = match.data;
// mux_mask = variant->mux_mask;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_clkout_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_clkout_probe(struct platform_device *pdev)
    {
    const char *parent_names[EXYNOS_CLKOUT_PARENTS];
    struct clk *parents[EXYNOS_CLKOUT_PARENTS];
    struct exynos_clkout *clkout;
    int parent_count, ret, i;
    u32 mux_mask;
    clkout = devm_kzalloc(&pdev.dev,
    struct_size(clkout, data.hws, EXYNOS_CLKOUT_NR_CLKS),
    GFP_KERNEL);
    if (!clkout)
    return -ENOMEM;
    ret = exynos_clkout_match_parent_dev(&pdev.dev, &mux_mask);
    if (ret)
    return ret;
    clkout.np = pdev.dev.of_node;
    if (!clkout.np) {
//
// pdev->dev.parent was checked by exynos_clkout_match_parent_dev()
// so it is not NULL.
//
    clkout.np = pdev.dev.parent.of_node;
    }
    platform_set_drvdata(pdev, clkout);
    spin_lock_init(&clkout.slock);
    parent_count = 0;
    for (i = 0; i < EXYNOS_CLKOUT_PARENTS; ++i) {
    char name[] = "clkoutXX";
    snprintf(name, sizeof(name), "clkout%d", i);
    parents[i] = of_clk_get_by_name(clkout.np, name);
    if (IS_ERR(parents[i])) {
    parent_names[i] = "none";
    continue;
    }
    parent_names[i] = __clk_get_name(parents[i]);
    parent_count = i + 1;
    }
    if (!parent_count)
    return -EINVAL;
    clkout.reg = of_iomap(clkout.np, 0);
    if (!clkout.reg) {
    ret = -ENODEV;
    goto clks_put;
    }
    clkout.gate.reg = clkout.reg + EXYNOS_PMU_DEBUG_REG;
    clkout.gate.bit_idx = EXYNOS_CLKOUT_DISABLE_SHIFT;
    clkout.gate.flags = CLK_GATE_SET_TO_DISABLE;
    clkout.gate.lock = &clkout.slock;
    clkout.mux.reg = clkout.reg + EXYNOS_PMU_DEBUG_REG;
    clkout.mux.mask = mux_mask;
    clkout.mux.shift = EXYNOS_CLKOUT_MUX_SHIFT;
    clkout.mux.lock = &clkout.slock;
    clkout.data.num = EXYNOS_CLKOUT_NR_CLKS;
    clkout.data.hws[0] = clk_hw_register_composite(core::ptr::null_mut(), "clkout",
    parent_names, parent_count, &clkout.mux.hw,
    &clk_mux_ops, core::ptr::null_mut(), core::ptr::null_mut(), &clkout.gate.hw,
    &clk_gate_ops, CLK_SET_RATE_PARENT
    | CLK_SET_RATE_NO_REPARENT);
    if (IS_ERR(clkout.data.hws[0])) {
    ret = PTR_ERR(clkout.data.hws[0]);
    goto err_unmap;
    }
    ret = of_clk_add_hw_provider(clkout.np, of_clk_hw_onecell_get, &clkout.data);
    if (ret)
    goto err_clk_unreg;
    return 0;
    err_clk_unreg:
    clk_hw_unregister(clkout.data.hws[0]);
    err_unmap:
    iounmap(clkout.reg);
    clks_put:
    for (i = 0; i < EXYNOS_CLKOUT_PARENTS; ++i)
    if (!IS_ERR(parents[i]))
    clk_put(parents[i]);
    dev_err(&pdev.dev, "failed to register clkout clock\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos_clkout_remove(pdev: *mut platform_device) {
    static void exynos_clkout_remove(struct platform_device *pdev)
    {
    struct exynos_clkout *clkout = platform_get_drvdata(pdev);
    of_clk_del_provider(clkout.np);
    clk_hw_unregister(clkout.data.hws[0]);
    iounmap(clkout.reg);
    }
#[no_mangle]
unsafe extern "C" fn exynos_clkout_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused exynos_clkout_suspend(struct device *dev)
    {
    struct exynos_clkout *clkout = dev_get_drvdata(dev);
    clkout.pmu_debug_save = readl(clkout.reg + EXYNOS_PMU_DEBUG_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_clkout_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused exynos_clkout_resume(struct device *dev)
    {
    struct exynos_clkout *clkout = dev_get_drvdata(dev);
    writel(clkout.pmu_debug_save, clkout.reg + EXYNOS_PMU_DEBUG_REG);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(exynos_clkout_pm_ops, exynos_clkout_suspend,
    exynos_clkout_resume);
    static struct platform_driver exynos_clkout_driver = {
    .driver = {
    .name = DRV_NAME,
    .pm = &exynos_clkout_pm_ops,
    },
    .probe = exynos_clkout_probe,
    .remove = exynos_clkout_remove,
    };
    module_platform_driver(exynos_clkout_driver);
    MODULE_AUTHOR("Krzysztof Kozlowski <krzk@kernel.org>");
    MODULE_AUTHOR("Tomasz Figa <tomasz.figa@gmail.com>");
    MODULE_DESCRIPTION("Samsung Exynos clock output driver");
    MODULE_ALIAS("platform:" DRV_NAME);
    MODULE_LICENSE("GPL");
