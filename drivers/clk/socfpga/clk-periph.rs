//! Automatically rewritten from C to Rust
//! Source: drivers/clk/socfpga/clk-periph.c
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
// Copyright 2011-2012 Calxeda, Inc.
// Copyright (C) 2012-2013 Altera Corporation <www.altera.com>
//
// Based from clk-highbank.c
//

    static unsigned long clk_periclk_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct socfpga_periph_clk *socfpgaclk = to_socfpga_periph_clk(hwclk);
    u32 div, val;
    if (socfpgaclk.fixed_div) {
    div = socfpgaclk.fixed_div;
    } else {
    if (socfpgaclk.div_reg) {
    val = readl(socfpgaclk.div_reg) >> socfpgaclk.shift;
    val &= GENMASK(socfpgaclk.width - 1, 0);
    parent_rate /= (val + 1);
    }
    div = ((readl(socfpgaclk.hw.reg) & 0x1ff) + 1);
    }
    return parent_rate / div;
    }
#[no_mangle]
unsafe extern "C" fn clk_periclk_get_parent(hwclk: *mut clk_hw) -> u8 {
    static u8 clk_periclk_get_parent(struct clk_hw *hwclk)
    {
    u32 clk_src;
    clk_src = readl(clk_mgr_base_addr + CLKMGR_DBCTRL);
    return clk_src & 0x1;
    }
    static const struct clk_ops periclk_ops = {
    .recalc_rate = clk_periclk_recalc_rate,
    .get_parent = clk_periclk_get_parent,
    };
    static void __init __socfpga_periph_init(struct device_node *node,
    const struct clk_ops *ops)
    {
    u32 reg;
    struct clk_hw *hw_clk;
    struct socfpga_periph_clk *periph_clk;
    const char *clk_name = node.name;
    const char *parent_name[SOCFPGA_MAX_PARENTS];
    struct clk_init_data init;
    int rc;
    u32 fixed_div;
    u32 div_reg[3];
    of_property_read_u32(node, "reg", &reg);
    periph_clk = kzalloc_obj(*periph_clk);
    if (WARN_ON(!periph_clk))
    return;
    periph_clk.hw.reg = clk_mgr_base_addr + reg;
    rc = of_property_read_u32_array(node, "div-reg", div_reg, 3);
    if (!rc) {
    periph_clk.div_reg = clk_mgr_base_addr + div_reg[0];
    periph_clk.shift = div_reg[1];
    periph_clk.width = div_reg[2];
    } else {
    periph_clk.div_reg = core::ptr::null_mut();
    }
    rc = of_property_read_u32(node, "fixed-divider", &fixed_div);
    if (rc)
    periph_clk.fixed_div = 0;
    else
    periph_clk.fixed_div = fixed_div;
    of_property_read_string(node, "clock-output-names", &clk_name);
    init.name = clk_name;
    init.ops = ops;
    init.flags = 0;
    init.num_parents = of_clk_parent_fill(node, parent_name,
    SOCFPGA_MAX_PARENTS);
    init.parent_names = parent_name;
    periph_clk.hw.hw.init = &init;
    hw_clk = &periph_clk.hw.hw;
    rc = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if (rc) {
    pr_err("Could not register clock:%s\n", clk_name);
    goto err_clk_hw_register;
    }
    rc = of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw_clk);
    if (rc) {
    pr_err("Could not register clock provider for node:%s\n",
    clk_name);
    goto err_of_clk_add_hw_provider;
    }
    return;
    err_of_clk_add_hw_provider:
    clk_hw_unregister(hw_clk);
    err_clk_hw_register:
    kfree(periph_clk);
    }
#[no_mangle]
pub unsafe extern "C" fn socfpga_periph_init(node: *mut device_node) -> void __init {
    void __init socfpga_periph_init(struct device_node *node)
    {
    __socfpga_periph_init(node, &periclk_ops);
    }
