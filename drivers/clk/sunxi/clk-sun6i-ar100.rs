//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-sun6i-ar100.c
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
// Copyright (C) 2014 Free Electrons
//
// Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
//
// Allwinner A31 AR100 clock driver
//

//
// sun6i_get_ar100_factors - Calculates factors p, m for AR100
//
// AR100 rate is calculated as follows
// rate = (parent_rate >> p) / (m + 1);
//
#[no_mangle]
unsafe extern "C" fn sun6i_get_ar100_factors(req: *mut factors_request) {
    static void sun6i_get_ar100_factors(struct factors_request *req)
    {
    unsigned long div;
    int shift;
// clock only divides
    if (req.rate > req.parent_rate)
    req.rate = req.parent_rate;
    div = DIV_ROUND_UP(req.parent_rate, req.rate);
    if (div < 32)
    shift = 0;
#[no_mangle]
pub unsafe extern "C" fn if(32: div >> 1 <) -> else {
    else if (div >> 1 < 32)
    shift = 1;
#[no_mangle]
pub unsafe extern "C" fn if(32: div >> 2 <) -> else {
    else if (div >> 2 < 32)
    shift = 2;
    else
    shift = 3;
    div >>= shift;
    if (div > 32)
    div = 32;
    req.rate = (req.parent_rate >> shift) / div;
    req.m = div - 1;
    req.p = shift;
    }
    static const struct clk_factors_config sun6i_ar100_config = {
    .mwidth = 5,
    .mshift = 8,
    .pwidth = 2,
    .pshift = 4,
    };
    static const struct factors_data sun6i_ar100_data = {
    .mux = 16,
    .muxmask = GENMASK(1, 0),
    .table = &sun6i_ar100_config,
    .getter = sun6i_get_ar100_factors,
    };
    static DEFINE_SPINLOCK(sun6i_ar100_lock);
#[no_mangle]
unsafe extern "C" fn sun6i_a31_ar100_clk_probe(pdev: *mut platform_device) -> c_int {
    static int sun6i_a31_ar100_clk_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    void __iomem *reg;
    struct clk *clk;
    reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg))
    return PTR_ERR(reg);
    clk = sunxi_factors_register(np, &sun6i_ar100_data, &sun6i_ar100_lock,
    reg);
    if (!clk)
    return -ENOMEM;
    platform_set_drvdata(pdev, clk);
    return 0;
    }
    static const struct of_device_id sun6i_a31_ar100_clk_dt_ids[] = {
    { .compatible = "allwinner,sun6i-a31-ar100-clk" },
    { /* sentinel */ }
    };
    static struct platform_driver sun6i_a31_ar100_clk_driver = {
    .driver = {
    .name = "sun6i-a31-ar100-clk",
    .of_match_table = sun6i_a31_ar100_clk_dt_ids,
    .suppress_bind_attrs = true,
    },
    .probe = sun6i_a31_ar100_clk_probe,
    };
    builtin_platform_driver(sun6i_a31_ar100_clk_driver);
