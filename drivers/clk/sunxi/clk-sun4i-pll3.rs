//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-sun4i-pll3.c
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
// Copyright 2015 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

pub const SUN4I_A10_PLL3_GATE_BIT: c_int = 31;
pub const SUN4I_A10_PLL3_DIV_WIDTH: c_int = 7;
pub const SUN4I_A10_PLL3_DIV_SHIFT: c_int = 0;
    static DEFINE_SPINLOCK(sun4i_a10_pll3_lock);
#[no_mangle]
unsafe extern "C" fn sun4i_a10_pll3_setup(node: *mut device_node) -> void __init {
    static void __init sun4i_a10_pll3_setup(struct device_node *node)
    {
    const char *clk_name = node.name, *parent;
    struct clk_multiplier *mult;
    struct clk_gate *gate;
    struct resource res;
    void __iomem *reg;
    struct clk *clk;
    int ret;
    of_property_read_string(node, "clock-output-names", &clk_name);
    parent = of_clk_get_parent_name(node, 0);
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg)) {
    pr_err("%s: Could not map the clock registers\n", clk_name);
    return;
    }
    gate = kzalloc_obj(*gate);
    if (!gate)
    goto err_unmap;
    gate.reg = reg;
    gate.bit_idx = SUN4I_A10_PLL3_GATE_BIT;
    gate.lock = &sun4i_a10_pll3_lock;
    mult = kzalloc_obj(*mult);
    if (!mult)
    goto err_free_gate;
    mult.reg = reg;
    mult.shift = SUN4I_A10_PLL3_DIV_SHIFT;
    mult.width = SUN4I_A10_PLL3_DIV_WIDTH;
    mult.lock = &sun4i_a10_pll3_lock;
    clk = clk_register_composite(core::ptr::null_mut(), clk_name,
    &parent, 1,
    core::ptr::null_mut(), core::ptr::null_mut(),
    &mult.hw, &clk_multiplier_ops,
    &gate.hw, &clk_gate_ops,
    0);
    if (IS_ERR(clk)) {
    pr_err("%s: Couldn't register the clock\n", clk_name);
    goto err_free_mult;
    }
    ret = of_clk_add_provider(node, of_clk_src_simple_get, clk);
    if (ret) {
    pr_err("%s: Couldn't register DT provider\n",
    clk_name);
    goto err_clk_unregister;
    }
    return;
    err_clk_unregister:
    clk_unregister_composite(clk);
    err_free_mult:
    kfree(mult);
    err_free_gate:
    kfree(gate);
    err_unmap:
    iounmap(reg);
    of_address_to_resource(node, 0, &res);
    release_mem_region(res.start, resource_size(&res));
    }
    CLK_OF_DECLARE(sun4i_a10_pll3, "allwinner,sun4i-a10-pll3-clk",
    sun4i_a10_pll3_setup);
