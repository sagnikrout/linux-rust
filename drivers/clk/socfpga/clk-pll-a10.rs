//! Automatically rewritten from C to Rust
//! Source: drivers/clk/socfpga/clk-pll-a10.c
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

// Clock Manager offsets
pub const CLK_MGR_PLL_CLK_SRC_SHIFT: c_int = 8;
pub const CLK_MGR_PLL_CLK_SRC_MASK: c_uint = 0x3;
// Clock bypass bits
pub const SOCFPGA_PLL_BG_PWRDWN: c_int = 0;
pub const SOCFPGA_PLL_PWR_DOWN: c_int = 1;
pub const SOCFPGA_PLL_EXT_ENA: c_int = 2;
pub const SOCFPGA_PLL_DIVF_MASK: c_uint = 0x00001FFF;
pub const SOCFPGA_PLL_DIVF_SHIFT: c_int = 0;
pub const SOCFPGA_PLL_DIVQ_MASK: c_uint = 0x003F0000;
pub const SOCFPGA_PLL_DIVQ_SHIFT: c_int = 16;
pub const SOCFGPA_MAX_PARENTS: c_int = 5;

    void __iomem *clk_mgr_a10_base_addr;
    static unsigned long clk_pll_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct socfpga_pll *socfpgaclk = to_socfpga_clk(hwclk);
    u32 divf, divq, reg;
    unsigned long long vco_freq;
// read VCO1 reg for numerator and denominator
    reg = readl(socfpgaclk.hw.reg + 0x4);
    divf = (reg & SOCFPGA_PLL_DIVF_MASK) >> SOCFPGA_PLL_DIVF_SHIFT;
    divq = (reg & SOCFPGA_PLL_DIVQ_MASK) >> SOCFPGA_PLL_DIVQ_SHIFT;
    vco_freq = (unsigned long long)parent_rate * (divf + 1);
    do_div(vco_freq, (1 + divq));
    return (unsigned long)vco_freq;
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_get_parent(hwclk: *mut clk_hw) -> u8 {
    static u8 clk_pll_get_parent(struct clk_hw *hwclk)
    {
    struct socfpga_pll *socfpgaclk = to_socfpga_clk(hwclk);
    u32 pll_src;
    pll_src = readl(socfpgaclk.hw.reg);
    return (pll_src >> CLK_MGR_PLL_CLK_SRC_SHIFT) &
    CLK_MGR_PLL_CLK_SRC_MASK;
    }
    static const struct clk_ops clk_pll_ops = {
    .recalc_rate = clk_pll_recalc_rate,
    .get_parent = clk_pll_get_parent,
    };
    static void __init __socfpga_pll_init(struct device_node *node,
    const struct clk_ops *ops)
    {
    u32 reg;
    struct clk_hw *hw_clk;
    struct socfpga_pll *pll_clk;
    const char *clk_name = node.name;
    const char *parent_name[SOCFGPA_MAX_PARENTS];
    struct clk_init_data init;
    struct device_node *clkmgr_np;
    int rc;
    let mut i: c_int = 0;
    of_property_read_u32(node, "reg", &reg);
    pll_clk = kzalloc_obj(*pll_clk);
    if (WARN_ON(!pll_clk))
    return;
    clkmgr_np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "altr,clk-mgr");
    clk_mgr_a10_base_addr = of_iomap(clkmgr_np, 0);
    of_node_put(clkmgr_np);
    BUG_ON(!clk_mgr_a10_base_addr);
    pll_clk.hw.reg = clk_mgr_a10_base_addr + reg;
    of_property_read_string(node, "clock-output-names", &clk_name);
    init.name = clk_name;
    init.ops = ops;
    init.flags = 0;
    while (i < SOCFGPA_MAX_PARENTS && (parent_name[i] =
    of_clk_get_parent_name(node, i)) != core::ptr::null_mut())
    i++;
    init.num_parents = i;
    init.parent_names = parent_name;
    pll_clk.hw.hw.init = &init;
    pll_clk.hw.bit_idx = SOCFPGA_PLL_EXT_ENA;
    hw_clk = &pll_clk.hw.hw;
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
    kfree(pll_clk);
    }
#[no_mangle]
pub unsafe extern "C" fn socfpga_a10_pll_init(node: *mut device_node) -> void __init {
    void __init socfpga_a10_pll_init(struct device_node *node)
    {
    __socfpga_pll_init(node, &clk_pll_ops);
    }
