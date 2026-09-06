//! Automatically rewritten from C to Rust
//! Source: drivers/clk/socfpga/clk-gate-a10.c
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
// Copyright (C) 2015 Altera Corporation. All rights reserved
//

// SDMMC Group for System Manager defines
pub const SYSMGR_SDMMCGRP_CTRL_OFFSET: c_uint = 0x28;
    static unsigned long socfpga_gate_clk_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct socfpga_gate_clk *socfpgaclk = to_socfpga_gate_clk(hwclk);
    let mut div: u32 = 1, val;
    if (socfpgaclk.fixed_div)
    div = socfpgaclk.fixed_div;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: socfpgaclk->div_reg) -> else {
    val = readl(socfpgaclk.div_reg) >> socfpgaclk.shift;
    val &= GENMASK(socfpgaclk.width - 1, 0);
    div = (1 << val);
    }
    return parent_rate / div;
    }
    static struct clk_ops gateclk_ops = {
    .recalc_rate = socfpga_gate_clk_recalc_rate,
    };
    static void __init __socfpga_gate_init(struct device_node *node,
    const struct clk_ops *ops)
    {
    u32 clk_gate[2];
    u32 div_reg[3];
    u32 fixed_div;
    struct clk_hw *hw_clk;
    struct socfpga_gate_clk *socfpga_clk;
    const char *clk_name = node.name;
    const char *parent_name[SOCFPGA_MAX_PARENTS];
    struct clk_init_data init;
    int rc;
    socfpga_clk = kzalloc_obj(*socfpga_clk);
    if (WARN_ON(!socfpga_clk))
    return;
    rc = of_property_read_u32_array(node, "clk-gate", clk_gate, 2);
    if (rc)
    clk_gate[0] = 0;
    if (clk_gate[0]) {
    socfpga_clk.hw.reg = clk_mgr_a10_base_addr + clk_gate[0];
    socfpga_clk.hw.bit_idx = clk_gate[1];
    gateclk_ops.enable = clk_gate_ops.enable;
    gateclk_ops.disable = clk_gate_ops.disable;
    }
    rc = of_property_read_u32(node, "fixed-divider", &fixed_div);
    if (rc)
    socfpga_clk.fixed_div = 0;
    else
    socfpga_clk.fixed_div = fixed_div;
    rc = of_property_read_u32_array(node, "div-reg", div_reg, 3);
    if (!rc) {
    socfpga_clk.div_reg = clk_mgr_a10_base_addr + div_reg[0];
    socfpga_clk.shift = div_reg[1];
    socfpga_clk.width = div_reg[2];
    } else {
    socfpga_clk.div_reg = core::ptr::null_mut();
    }
    of_property_read_string(node, "clock-output-names", &clk_name);
    init.name = clk_name;
    init.ops = ops;
    init.flags = 0;
    init.num_parents = of_clk_parent_fill(node, parent_name, SOCFPGA_MAX_PARENTS);
    init.parent_names = parent_name;
    socfpga_clk.hw.hw.init = &init;
    hw_clk = &socfpga_clk.hw.hw;
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
    kfree(socfpga_clk);
    }
#[no_mangle]
pub unsafe extern "C" fn socfpga_a10_gate_init(node: *mut device_node) -> void __init {
    void __init socfpga_a10_gate_init(struct device_node *node)
    {
    __socfpga_gate_init(node, &gateclk_ops);
    }
