//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-a10-pll2.c
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
// Copyright 2013 Emilio López
// Emilio López <emilio@elopez.com.ar>
//
// Copyright 2015 Maxime Ripard
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

pub const SUN4I_PLL2_ENABLE: c_int = 31;
pub const SUN4I_PLL2_PRE_DIV_SHIFT: c_int = 0;
pub const SUN4I_PLL2_PRE_DIV_WIDTH: c_int = 5;

pub const SUN4I_PLL2_N_SHIFT: c_int = 8;
pub const SUN4I_PLL2_N_WIDTH: c_int = 7;

pub const SUN4I_PLL2_POST_DIV_SHIFT: c_int = 26;
pub const SUN4I_PLL2_POST_DIV_WIDTH: c_int = 4;

pub const SUN4I_PLL2_POST_DIV_VALUE: c_int = 4;
pub const SUN4I_PLL2_OUTPUTS: c_int = 4;
    static DEFINE_SPINLOCK(sun4i_a10_pll2_lock);
    static void __init sun4i_pll2_setup(struct device_node *node,
    int post_div_offset)
    {
    const char *clk_name = node.name, *parent;
    struct clk **clks, *base_clk, *prediv_clk;
    struct clk_onecell_data *clk_data;
    struct clk_multiplier *mult;
    struct clk_gate *gate;
    void __iomem *reg;
    u32 val;
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg))
    return;
    clk_data = kzalloc_obj(*clk_data);
    if (!clk_data)
    goto err_unmap;
    clks = kzalloc_objs(struct clk *, SUN4I_PLL2_OUTPUTS);
    if (!clks)
    goto err_free_data;
    parent = of_clk_get_parent_name(node, 0);
    prediv_clk = clk_register_divider(core::ptr::null_mut(), "pll2-prediv",
    parent, 0, reg,
    SUN4I_PLL2_PRE_DIV_SHIFT,
    SUN4I_PLL2_PRE_DIV_WIDTH,
    CLK_DIVIDER_ONE_BASED | CLK_DIVIDER_ALLOW_ZERO,
    &sun4i_a10_pll2_lock);
    if (IS_ERR(prediv_clk)) {
    pr_err("Couldn't register the prediv clock\n");
    goto err_free_array;
    }
// Setup the gate part of the PLL2
    gate = kzalloc_obj(struct clk_gate);
    if (!gate)
    goto err_unregister_prediv;
    gate.reg = reg;
    gate.bit_idx = SUN4I_PLL2_ENABLE;
    gate.lock = &sun4i_a10_pll2_lock;
// Setup the multiplier part of the PLL2
    mult = kzalloc_obj(struct clk_multiplier);
    if (!mult)
    goto err_free_gate;
    mult.reg = reg;
    mult.shift = SUN4I_PLL2_N_SHIFT;
    mult.width = 7;
    mult.flags = CLK_MULTIPLIER_ZERO_BYPASS |
    CLK_MULTIPLIER_ROUND_CLOSEST;
    mult.lock = &sun4i_a10_pll2_lock;
    parent = __clk_get_name(prediv_clk);
    base_clk = clk_register_composite(core::ptr::null_mut(), "pll2-base",
    &parent, 1,
    core::ptr::null_mut(), core::ptr::null_mut(),
    &mult.hw, &clk_multiplier_ops,
    &gate.hw, &clk_gate_ops,
    CLK_SET_RATE_PARENT);
    if (IS_ERR(base_clk)) {
    pr_err("Couldn't register the base multiplier clock\n");
    goto err_free_multiplier;
    }
    parent = __clk_get_name(base_clk);
//
// PLL2-1x
//
// This is supposed to have a post divider, but we won't need
// to use it, we just need to initialise it to 4, and use a
// fixed divider.
//
    val = readl(reg);
    val &= ~(SUN4I_PLL2_POST_DIV_MASK << SUN4I_PLL2_POST_DIV_SHIFT);
    val |= (SUN4I_PLL2_POST_DIV_VALUE - post_div_offset) << SUN4I_PLL2_POST_DIV_SHIFT;
    writel(val, reg);
    of_property_read_string_index(node, "clock-output-names",
    SUN4I_A10_PLL2_1X, &clk_name);
    clks[SUN4I_A10_PLL2_1X] = clk_register_fixed_factor(core::ptr::null_mut(), clk_name,
    parent,
    CLK_SET_RATE_PARENT,
    1,
    SUN4I_PLL2_POST_DIV_VALUE);
    WARN_ON(IS_ERR(clks[SUN4I_A10_PLL2_1X]));
//
// PLL2-2x
//
// This clock doesn't use the post divider, and really is just
// a fixed divider from the PLL2 base clock.
//
    of_property_read_string_index(node, "clock-output-names",
    SUN4I_A10_PLL2_2X, &clk_name);
    clks[SUN4I_A10_PLL2_2X] = clk_register_fixed_factor(core::ptr::null_mut(), clk_name,
    parent,
    CLK_SET_RATE_PARENT,
    1, 2);
    WARN_ON(IS_ERR(clks[SUN4I_A10_PLL2_2X]));
// PLL2-4x
    of_property_read_string_index(node, "clock-output-names",
    SUN4I_A10_PLL2_4X, &clk_name);
    clks[SUN4I_A10_PLL2_4X] = clk_register_fixed_factor(core::ptr::null_mut(), clk_name,
    parent,
    CLK_SET_RATE_PARENT,
    1, 1);
    WARN_ON(IS_ERR(clks[SUN4I_A10_PLL2_4X]));
// PLL2-8x
    of_property_read_string_index(node, "clock-output-names",
    SUN4I_A10_PLL2_8X, &clk_name);
    clks[SUN4I_A10_PLL2_8X] = clk_register_fixed_factor(core::ptr::null_mut(), clk_name,
    parent,
    CLK_SET_RATE_PARENT,
    2, 1);
    WARN_ON(IS_ERR(clks[SUN4I_A10_PLL2_8X]));
    clk_data.clks = clks;
    clk_data.clk_num = SUN4I_PLL2_OUTPUTS;
    of_clk_add_provider(node, of_clk_src_onecell_get, clk_data);
    return;
    err_free_multiplier:
    kfree(mult);
    err_free_gate:
    kfree(gate);
    err_unregister_prediv:
    clk_unregister_divider(prediv_clk);
    err_free_array:
    kfree(clks);
    err_free_data:
    kfree(clk_data);
    err_unmap:
    iounmap(reg);
    }
#[no_mangle]
unsafe extern "C" fn sun4i_a10_pll2_setup(node: *mut device_node) -> void __init {
    static void __init sun4i_a10_pll2_setup(struct device_node *node)
    {
    sun4i_pll2_setup(node, 0);
    }
    CLK_OF_DECLARE(sun4i_a10_pll2, "allwinner,sun4i-a10-pll2-clk",
    sun4i_a10_pll2_setup);
#[no_mangle]
unsafe extern "C" fn sun5i_a13_pll2_setup(node: *mut device_node) -> void __init {
    static void __init sun5i_a13_pll2_setup(struct device_node *node)
    {
    sun4i_pll2_setup(node, 1);
    }
    CLK_OF_DECLARE(sun5i_a13_pll2, "allwinner,sun5i-a13-pll2-clk",
    sun5i_a13_pll2_setup);
