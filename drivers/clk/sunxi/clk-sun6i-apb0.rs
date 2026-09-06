//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-sun6i-apb0.c
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
// Allwinner A31 APB0 clock driver
//

//
// The APB0 clk has a configurable divisor.
//
// We must use a clk_div_table and not a regular power of 2
// divisor here, because the first 2 values divide the clock
// by 2.
//
    static const struct clk_div_table sun6i_a31_apb0_divs[] = {
    { .val = 0, .div = 2, },
    { .val = 1, .div = 2, },
    { .val = 2, .div = 4, },
    { .val = 3, .div = 8, },
    { /* sentinel */ },
    };
#[no_mangle]
unsafe extern "C" fn sun6i_a31_apb0_clk_probe(pdev: *mut platform_device) -> c_int {
    static int sun6i_a31_apb0_clk_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    const char *clk_name = np.name;
    const char *clk_parent;
    void __iomem *reg;
    struct clk *clk;
    reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg))
    return PTR_ERR(reg);
    clk_parent = of_clk_get_parent_name(np, 0);
    if (!clk_parent)
    return -EINVAL;
    of_property_read_string(np, "clock-output-names", &clk_name);
    clk = clk_register_divider_table(&pdev.dev, clk_name, clk_parent,
    0, reg, 0, 2, 0, sun6i_a31_apb0_divs,
    core::ptr::null_mut());
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    return of_clk_add_provider(np, of_clk_src_simple_get, clk);
    }
    static const struct of_device_id sun6i_a31_apb0_clk_dt_ids[] = {
    { .compatible = "allwinner,sun6i-a31-apb0-clk" },
    { /* sentinel */ }
    };
    static struct platform_driver sun6i_a31_apb0_clk_driver = {
    .driver = {
    .name = "sun6i-a31-apb0-clk",
    .of_match_table = sun6i_a31_apb0_clk_dt_ids,
    },
    .probe = sun6i_a31_apb0_clk_probe,
    };
    builtin_platform_driver(sun6i_a31_apb0_clk_driver);
