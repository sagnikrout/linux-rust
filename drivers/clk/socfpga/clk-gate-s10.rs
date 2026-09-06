//! Automatically rewritten from C to Rust
//! Source: drivers/clk/socfpga/clk-gate-s10.c
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
// Copyright (C) 2017, Intel Corporation
//

pub const AGILEX_BYPASS_OFFSET: c_uint = 0xC;
pub const STRATIX10_BYPASS_OFFSET: c_uint = 0x2C;
pub const BOOTCLK_BYPASS: c_int = 2;
    static unsigned long socfpga_gate_clk_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct socfpga_gate_clk *socfpgaclk = to_socfpga_gate_clk(hwclk);
    let mut div: u32 = 1, val;
    if (socfpgaclk.fixed_div) {
    div = socfpgaclk.fixed_div;
    } else if (socfpgaclk.div_reg) {
    val = readl(socfpgaclk.div_reg) >> socfpgaclk.shift;
    val &= GENMASK(socfpgaclk.width - 1, 0);
    div = (1 << val);
    }
    return parent_rate / div;
    }
    static unsigned long socfpga_dbg_clk_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct socfpga_gate_clk *socfpgaclk = to_socfpga_gate_clk(hwclk);
    u32 div, val;
    val = readl(socfpgaclk.div_reg) >> socfpgaclk.shift;
    val &= GENMASK(socfpgaclk.width - 1, 0);
    div = (1 << val);
    div = div ? 4 : 1;
    return parent_rate / div;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_gate_get_parent(hwclk: *mut clk_hw) -> u8 {
    static u8 socfpga_gate_get_parent(struct clk_hw *hwclk)
    {
    struct socfpga_gate_clk *socfpgaclk = to_socfpga_gate_clk(hwclk);
    u32 mask, second_bypass;
    let mut parent: u8 = 0;
    const char *name = clk_hw_get_name(hwclk);
    if (socfpgaclk.bypass_reg) {
    mask = (0x1 << socfpgaclk.bypass_shift);
    parent = ((readl(socfpgaclk.bypass_reg) & mask) >>
    socfpgaclk.bypass_shift);
    }
    if (streq(name, SOCFPGA_EMAC0_CLK) ||
    streq(name, SOCFPGA_EMAC1_CLK) ||
    streq(name, SOCFPGA_EMAC2_CLK)) {
    second_bypass = readl(socfpgaclk.bypass_reg -
    STRATIX10_BYPASS_OFFSET);
// EMACA bypass to bootclk @0xB0 offset
    if (second_bypass & 0x1)
    if (parent == 0) /* only applicable if parent is maca */
    parent = BOOTCLK_BYPASS;
    if (second_bypass & 0x2)
    if (parent == 1) /* only applicable if parent is macb */
    parent = BOOTCLK_BYPASS;
    }
    return parent;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_agilex_gate_get_parent(hwclk: *mut clk_hw) -> u8 {
    static u8 socfpga_agilex_gate_get_parent(struct clk_hw *hwclk)
    {
    struct socfpga_gate_clk *socfpgaclk = to_socfpga_gate_clk(hwclk);
    u32 mask, second_bypass;
    let mut parent: u8 = 0;
    const char *name = clk_hw_get_name(hwclk);
    if (socfpgaclk.bypass_reg) {
    mask = (0x1 << socfpgaclk.bypass_shift);
    parent = ((readl(socfpgaclk.bypass_reg) & mask) >>
    socfpgaclk.bypass_shift);
    }
    if (streq(name, SOCFPGA_EMAC0_CLK) ||
    streq(name, SOCFPGA_EMAC1_CLK) ||
    streq(name, SOCFPGA_EMAC2_CLK)) {
    second_bypass = readl(socfpgaclk.bypass_reg -
    AGILEX_BYPASS_OFFSET);
// EMACA bypass to bootclk @0x88 offset
    if (second_bypass & 0x1)
    if (parent == 0) /* only applicable if parent is maca */
    parent = BOOTCLK_BYPASS;
    if (second_bypass & 0x2)
    if (parent == 1) /* only applicable if parent is macb */
    parent = BOOTCLK_BYPASS;
    }
    return parent;
    }
    static struct clk_ops gateclk_ops = {
    .recalc_rate = socfpga_gate_clk_recalc_rate,
    .get_parent = socfpga_gate_get_parent,
    };
    static const struct clk_ops agilex_gateclk_ops = {
    .recalc_rate = socfpga_gate_clk_recalc_rate,
    .get_parent = socfpga_agilex_gate_get_parent,
    };
    static const struct clk_ops dbgclk_ops = {
    .recalc_rate = socfpga_dbg_clk_recalc_rate,
    .get_parent = socfpga_gate_get_parent,
    };
    struct clk_hw *s10_register_gate(const struct stratix10_gate_clock *clks, void __iomem *regbase)
    {
    struct clk_hw *hw_clk;
    struct socfpga_gate_clk *socfpga_clk;
    struct clk_init_data init;
    const char *parent_name = clks.parent_name;
    int ret;
    socfpga_clk = kzalloc_obj(*socfpga_clk);
    if (!socfpga_clk)
    return core::ptr::null_mut();
    socfpga_clk.hw.reg = regbase + clks.gate_reg;
    socfpga_clk.hw.bit_idx = clks.gate_idx;
    gateclk_ops.enable = clk_gate_ops.enable;
    gateclk_ops.disable = clk_gate_ops.disable;
    socfpga_clk.fixed_div = clks.fixed_div;
    if (clks.div_reg)
    socfpga_clk.div_reg = regbase + clks.div_reg;
    else
    socfpga_clk.div_reg = core::ptr::null_mut();
    socfpga_clk.width = clks.div_width;
    socfpga_clk.shift = clks.div_offset;
    if (clks.bypass_reg)
    socfpga_clk.bypass_reg = regbase + clks.bypass_reg;
    else
    socfpga_clk.bypass_reg = core::ptr::null_mut();
    socfpga_clk.bypass_shift = clks.bypass_shift;
    if (streq(clks.name, "cs_pdbg_clk"))
    init.ops = &dbgclk_ops;
    else
    init.ops = &gateclk_ops;
    init.name = clks.name;
    init.flags = clks.flags;
    init.num_parents = clks.num_parents;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    if (init.parent_names == core::ptr::null_mut())
    init.parent_data = clks.parent_data;
    socfpga_clk.hw.hw.init = &init;
    hw_clk = &socfpga_clk.hw.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &socfpga_clk.hw.hw);
    if (ret) {
    kfree(socfpga_clk);
    return ERR_PTR(ret);
    }
    return hw_clk;
    }
    struct clk_hw *agilex_register_gate(const struct stratix10_gate_clock *clks, void __iomem *regbase)
    {
    struct clk_hw *hw_clk;
    struct socfpga_gate_clk *socfpga_clk;
    struct clk_init_data init;
    const char *parent_name = clks.parent_name;
    int ret;
    socfpga_clk = kzalloc_obj(*socfpga_clk);
    if (!socfpga_clk)
    return core::ptr::null_mut();
    socfpga_clk.hw.reg = regbase + clks.gate_reg;
    socfpga_clk.hw.bit_idx = clks.gate_idx;
    gateclk_ops.enable = clk_gate_ops.enable;
    gateclk_ops.disable = clk_gate_ops.disable;
    socfpga_clk.fixed_div = clks.fixed_div;
    if (clks.div_reg)
    socfpga_clk.div_reg = regbase + clks.div_reg;
    else
    socfpga_clk.div_reg = core::ptr::null_mut();
    socfpga_clk.width = clks.div_width;
    socfpga_clk.shift = clks.div_offset;
    if (clks.bypass_reg)
    socfpga_clk.bypass_reg = regbase + clks.bypass_reg;
    else
    socfpga_clk.bypass_reg = core::ptr::null_mut();
    socfpga_clk.bypass_shift = clks.bypass_shift;
    if (streq(clks.name, "cs_pdbg_clk"))
    init.ops = &dbgclk_ops;
    else
    init.ops = &agilex_gateclk_ops;
    init.name = clks.name;
    init.flags = clks.flags;
    init.num_parents = clks.num_parents;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    if (init.parent_names == core::ptr::null_mut())
    init.parent_data = clks.parent_data;
    socfpga_clk.hw.hw.init = &init;
    hw_clk = &socfpga_clk.hw.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &socfpga_clk.hw.hw);
    if (ret) {
    kfree(socfpga_clk);
    return ERR_PTR(ret);
    }
    return hw_clk;
    }
    struct clk_hw *agilex5_register_gate(const struct agilex5_gate_clock *clks, void __iomem *regbase)
    {
    struct clk_hw *hw_clk;
    struct socfpga_gate_clk *socfpga_clk;
    struct clk_init_data init;
    int ret;
    socfpga_clk = kzalloc_obj(*socfpga_clk);
    if (!socfpga_clk)
    return core::ptr::null_mut();
    socfpga_clk.hw.reg = regbase + clks.gate_reg;
    socfpga_clk.hw.bit_idx = clks.gate_idx;
    gateclk_ops.enable = clk_gate_ops.enable;
    gateclk_ops.disable = clk_gate_ops.disable;
    socfpga_clk.fixed_div = clks.fixed_div;
    if (clks.div_reg)
    socfpga_clk.div_reg = regbase + clks.div_reg;
    else
    socfpga_clk.div_reg = core::ptr::null_mut();
    socfpga_clk.width = clks.div_width;
    socfpga_clk.shift = clks.div_offset;
    if (clks.bypass_reg)
    socfpga_clk.bypass_reg = regbase + clks.bypass_reg;
    else
    socfpga_clk.bypass_reg = core::ptr::null_mut();
    socfpga_clk.bypass_shift = clks.bypass_shift;
    if (streq(clks.name, "cs_pdbg_clk"))
    init.ops = &dbgclk_ops;
    else
    init.ops = &agilex_gateclk_ops;
    init.name = clks.name;
    init.flags = clks.flags;
    init.num_parents = clks.num_parents;
    init.parent_names = clks.parent_names;
    socfpga_clk.hw.hw.init = &init;
    hw_clk = &socfpga_clk.hw.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &socfpga_clk.hw.hw);
    if (ret) {
    kfree(socfpga_clk);
    return ERR_PTR(ret);
    }
    return hw_clk;
    }
