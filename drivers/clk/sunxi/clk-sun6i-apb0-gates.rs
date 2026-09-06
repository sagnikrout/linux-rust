//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-sun6i-apb0-gates.c
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
// Allwinner A31 APB0 clock gates driver
//

pub const SUN6I_APB0_GATES_MAX_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gates_data {
    pub SUN6I_APB0_GATES_MAX_SIZE): DECLARE_BITMAP(mask,,
}

    static const struct gates_data sun6i_a31_apb0_gates __initconst = {
    .mask = {0x7F},
    };
    static const struct gates_data sun8i_a23_apb0_gates __initconst = {
    .mask = {0x5D},
    };
    static const struct of_device_id sun6i_a31_apb0_gates_clk_dt_ids[] = {
    { .compatible = "allwinner,sun6i-a31-apb0-gates-clk", .data = &sun6i_a31_apb0_gates },
    { .compatible = "allwinner,sun8i-a23-apb0-gates-clk", .data = &sun8i_a23_apb0_gates },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn sun6i_a31_apb0_gates_clk_probe(pdev: *mut platform_device) -> c_int {
    static int sun6i_a31_apb0_gates_clk_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct clk_onecell_data *clk_data;
    const struct gates_data *data;
    const char *clk_parent;
    const char *clk_name;
    void __iomem *reg;
    int ngates;
    int i;
    let mut j: c_int = 0;
    if (!np)
    return -ENODEV;
    data = of_device_get_match_data(&pdev.dev);
    if (!data)
    return -ENODEV;
    reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg))
    return PTR_ERR(reg);
    clk_parent = of_clk_get_parent_name(np, 0);
    if (!clk_parent)
    return -EINVAL;
    clk_data = devm_kzalloc(&pdev.dev, sizeof(struct clk_onecell_data),
    GFP_KERNEL);
    if (!clk_data)
    return -ENOMEM;
// Worst-case size approximation and memory allocation
    ngates = find_last_bit(data.mask, SUN6I_APB0_GATES_MAX_SIZE);
    clk_data.clks = devm_kcalloc(&pdev.dev, (ngates + 1),
    sizeof(struct clk *), GFP_KERNEL);
    if (!clk_data.clks)
    return -ENOMEM;
    for_each_set_bit(i, data.mask, SUN6I_APB0_GATES_MAX_SIZE) {
    of_property_read_string_index(np, "clock-output-names",
    j, &clk_name);
    clk_data.clks[i] = clk_register_gate(&pdev.dev, clk_name,
    clk_parent, 0, reg, i,
    0, core::ptr::null_mut());
    WARN_ON(IS_ERR(clk_data.clks[i]));
    j++;
    }
    clk_data.clk_num = ngates + 1;
    return of_clk_add_provider(np, of_clk_src_onecell_get, clk_data);
    }
    static struct platform_driver sun6i_a31_apb0_gates_clk_driver = {
    .driver = {
    .name = "sun6i-a31-apb0-gates-clk",
    .of_match_table = sun6i_a31_apb0_gates_clk_dt_ids,
    },
    .probe = sun6i_a31_apb0_gates_clk_probe,
    };
    builtin_platform_driver(sun6i_a31_apb0_gates_clk_driver);
