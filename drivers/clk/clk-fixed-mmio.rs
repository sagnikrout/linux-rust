//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-fixed-mmio.c
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
// Memory Mapped IO Fixed clock driver
//
// Copyright (C) 2018 Cadence Design Systems, Inc.
//
// Authors:
// Jan Kotas <jank@cadence.com>
//

    static struct clk_hw *fixed_mmio_clk_setup(struct device_node *node)
    {
    struct clk_hw *clk;
    const char *clk_name = node.name;
    void __iomem *base;
    u32 freq;
    int ret;
    base = of_iomap(node, 0);
    if (!base) {
    pr_err("%pOFn: failed to map address\n", node);
    return ERR_PTR(-EIO);
    }
    freq = readl(base);
    iounmap(base);
    of_property_read_string(node, "clock-output-names", &clk_name);
    clk = clk_hw_register_fixed_rate(core::ptr::null_mut(), clk_name, core::ptr::null_mut(), 0, freq);
    if (IS_ERR(clk)) {
    pr_err("%pOFn: failed to register fixed rate clock\n", node);
    return clk;
    }
    ret = of_clk_add_hw_provider(node, of_clk_hw_simple_get, clk);
    if (ret) {
    pr_err("%pOFn: failed to add clock provider\n", node);
    clk_hw_unregister(clk);
    clk = ERR_PTR(ret);
    }
    return clk;
    }
#[no_mangle]
unsafe extern "C" fn of_fixed_mmio_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_fixed_mmio_clk_setup(struct device_node *node)
    {
    fixed_mmio_clk_setup(node);
    }
    CLK_OF_DECLARE(fixed_mmio_clk, "fixed-mmio-clock", of_fixed_mmio_clk_setup);
//
// This is not executed when of_fixed_mmio_clk_setup succeeded.
//
#[no_mangle]
unsafe extern "C" fn of_fixed_mmio_clk_probe(pdev: *mut platform_device) -> c_int {
    static int of_fixed_mmio_clk_probe(struct platform_device *pdev)
    {
    struct clk_hw *clk;
    clk = fixed_mmio_clk_setup(pdev.dev.of_node);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    platform_set_drvdata(pdev, clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn of_fixed_mmio_clk_remove(pdev: *mut platform_device) {
    static void of_fixed_mmio_clk_remove(struct platform_device *pdev)
    {
    struct clk_hw *clk = platform_get_drvdata(pdev);
    of_clk_del_provider(pdev.dev.of_node);
    clk_hw_unregister_fixed_rate(clk);
    }
    static const struct of_device_id of_fixed_mmio_clk_ids[] = {
    { .compatible = "fixed-mmio-clock" },
    { }
    };
    MODULE_DEVICE_TABLE(of, of_fixed_mmio_clk_ids);
    static struct platform_driver of_fixed_mmio_clk_driver = {
    .driver = {
    .name = "of_fixed_mmio_clk",
    .of_match_table = of_fixed_mmio_clk_ids,
    },
    .probe = of_fixed_mmio_clk_probe,
    .remove = of_fixed_mmio_clk_remove,
    };
    module_platform_driver(of_fixed_mmio_clk_driver);
    MODULE_AUTHOR("Jan Kotas <jank@cadence.com>");
    MODULE_DESCRIPTION("Memory Mapped IO Fixed clock driver");
