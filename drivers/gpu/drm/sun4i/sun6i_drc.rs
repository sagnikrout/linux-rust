//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/sun4i/sun6i_drc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016 Free Electrons
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_drc {
    pub bus_clk: *mut clk,
    pub mod_clk: *mut clk,
    pub reset: *mut reset_control,
}

    static int sun6i_drc_bind(struct device *dev, struct device *master,
    void *data)
    {
    struct sun6i_drc *drc;
    int ret;
    drc = devm_kzalloc(dev, sizeof(*drc), GFP_KERNEL);
    if (!drc)
    return -ENOMEM;
    dev_set_drvdata(dev, drc);
    drc.reset = devm_reset_control_get(dev, core::ptr::null_mut());
    if (IS_ERR(drc.reset)) {
    dev_err(dev, "Couldn't get our reset line\n");
    return PTR_ERR(drc.reset);
    }
    ret = reset_control_deassert(drc.reset);
    if (ret) {
    dev_err(dev, "Couldn't deassert our reset line\n");
    return ret;
    }
    drc.bus_clk = devm_clk_get(dev, "ahb");
    if (IS_ERR(drc.bus_clk)) {
    dev_err(dev, "Couldn't get our bus clock\n");
    ret = PTR_ERR(drc.bus_clk);
    goto err_assert_reset;
    }
    clk_prepare_enable(drc.bus_clk);
    drc.mod_clk = devm_clk_get(dev, "mod");
    if (IS_ERR(drc.mod_clk)) {
    dev_err(dev, "Couldn't get our mod clock\n");
    ret = PTR_ERR(drc.mod_clk);
    goto err_disable_bus_clk;
    }
    ret = clk_set_rate_exclusive(drc.mod_clk, 300000000);
    if (ret) {
    dev_err(dev, "Couldn't set the module clock frequency\n");
    goto err_disable_bus_clk;
    }
    clk_prepare_enable(drc.mod_clk);
    return 0;
    err_disable_bus_clk:
    clk_disable_unprepare(drc.bus_clk);
    err_assert_reset:
    reset_control_assert(drc.reset);
    return ret;
    }
    static void sun6i_drc_unbind(struct device *dev, struct device *master,
    void *data)
    {
    struct sun6i_drc *drc = dev_get_drvdata(dev);
    clk_rate_exclusive_put(drc.mod_clk);
    clk_disable_unprepare(drc.mod_clk);
    clk_disable_unprepare(drc.bus_clk);
    reset_control_assert(drc.reset);
    }
    static const struct component_ops sun6i_drc_ops = {
    .bind	= sun6i_drc_bind,
    .unbind	= sun6i_drc_unbind,
    };
#[no_mangle]
unsafe extern "C" fn sun6i_drc_probe(pdev: *mut platform_device) -> c_int {
    static int sun6i_drc_probe(struct platform_device *pdev)
    {
    return component_add(&pdev.dev, &sun6i_drc_ops);
    }
#[no_mangle]
unsafe extern "C" fn sun6i_drc_remove(pdev: *mut platform_device) {
    static void sun6i_drc_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &sun6i_drc_ops);
    }
    static const struct of_device_id sun6i_drc_of_table[] = {
    { .compatible = "allwinner,sun6i-a31-drc" },
    { .compatible = "allwinner,sun6i-a31s-drc" },
    { .compatible = "allwinner,sun8i-a23-drc" },
    { .compatible = "allwinner,sun8i-a33-drc" },
    { .compatible = "allwinner,sun9i-a80-drc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sun6i_drc_of_table);
    static struct platform_driver sun6i_drc_platform_driver = {
    .probe		= sun6i_drc_probe,
    .remove		= sun6i_drc_remove,
    .driver		= {
    .name		= "sun6i-drc",
    .of_match_table	= sun6i_drc_of_table,
    },
    };
    module_platform_driver(sun6i_drc_platform_driver);
    MODULE_AUTHOR("Maxime Ripard <maxime.ripard@free-electrons.com>");
    MODULE_DESCRIPTION("Allwinner A31 Dynamic Range Control (DRC) Driver");
    MODULE_LICENSE("GPL");
