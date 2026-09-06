//! Automatically rewritten from C to Rust
//! Source: drivers/clk/socfpga/clk-periph-s10.c
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

pub const CLK_MGR_FREE_SHIFT: c_int = 16;
pub const CLK_MGR_FREE_MASK: c_uint = 0x7;
pub const SWCTRLBTCLKSEN_SHIFT: c_int = 8;

    static unsigned long n5x_clk_peri_c_clk_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct socfpga_periph_clk *socfpgaclk = to_periph_clk(hwclk);
    unsigned long div;
    let mut shift: c_ulong = socfpgaclk.shift;
    u32 val;
    val = readl(socfpgaclk.hw.reg);
    val &= (0x1f << shift);
    div = (val >> shift) + 1;
    return parent_rate / div;
    }
    static unsigned long clk_peri_c_clk_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct socfpga_periph_clk *socfpgaclk = to_periph_clk(hwclk);
    let mut div: c_ulong = 1;
    u32 val;
    val = readl(socfpgaclk.hw.reg);
    val &= GENMASK(SWCTRLBTCLKSEN_SHIFT - 1, 0);
    parent_rate /= val;
    return parent_rate / div;
    }
    static unsigned long clk_peri_cnt_clk_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct socfpga_periph_clk *socfpgaclk = to_periph_clk(hwclk);
    let mut div: c_ulong = 1;
    if (socfpgaclk.fixed_div) {
    div = socfpgaclk.fixed_div;
    } else {
    if (socfpgaclk.hw.reg)
    div = ((readl(socfpgaclk.hw.reg) & 0x7ff) + 1);
    }
    return parent_rate / div;
    }
#[no_mangle]
unsafe extern "C" fn clk_periclk_get_parent(hwclk: *mut clk_hw) -> u8 {
    static u8 clk_periclk_get_parent(struct clk_hw *hwclk)
    {
    struct socfpga_periph_clk *socfpgaclk = to_periph_clk(hwclk);
    u32 clk_src, mask;
    let mut parent: u8 = 0;
// handle the bypass first
    if (socfpgaclk.bypass_reg) {
    mask = (0x1 << socfpgaclk.bypass_shift);
    parent = ((readl(socfpgaclk.bypass_reg) & mask) >>
    socfpgaclk.bypass_shift);
    if (parent)
    return parent;
    }
    if (socfpgaclk.hw.reg) {
    clk_src = readl(socfpgaclk.hw.reg);
    parent = (clk_src >> CLK_MGR_FREE_SHIFT) &
    CLK_MGR_FREE_MASK;
    }
    return parent;
    }
    static const struct clk_ops n5x_peri_c_clk_ops = {
    .recalc_rate = n5x_clk_peri_c_clk_recalc_rate,
    .get_parent = clk_periclk_get_parent,
    };
    static const struct clk_ops peri_c_clk_ops = {
    .recalc_rate = clk_peri_c_clk_recalc_rate,
    .get_parent = clk_periclk_get_parent,
    };
    static const struct clk_ops peri_cnt_clk_ops = {
    .recalc_rate = clk_peri_cnt_clk_recalc_rate,
    .get_parent = clk_periclk_get_parent,
    };
    struct clk_hw *s10_register_periph(const struct stratix10_perip_c_clock *clks,
    void __iomem *reg)
    {
    struct clk_hw *hw_clk;
    struct socfpga_periph_clk *periph_clk;
    struct clk_init_data init;
    const char *name = clks.name;
    const char *parent_name = clks.parent_name;
    int ret;
    periph_clk = kzalloc_obj(*periph_clk);
    if (WARN_ON(!periph_clk))
    return core::ptr::null_mut();
    periph_clk.hw.reg = reg + clks.offset;
    init.name = name;
    init.ops = &peri_c_clk_ops;
    init.flags = clks.flags;
    init.num_parents = clks.num_parents;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    if (init.parent_names == core::ptr::null_mut())
    init.parent_data = clks.parent_data;
    periph_clk.hw.hw.init = &init;
    hw_clk = &periph_clk.hw.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if (ret) {
    kfree(periph_clk);
    return ERR_PTR(ret);
    }
    return hw_clk;
    }
    struct clk_hw *n5x_register_periph(const struct n5x_perip_c_clock *clks,
    void __iomem *regbase)
    {
    struct clk_hw *hw_clk;
    struct socfpga_periph_clk *periph_clk;
    struct clk_init_data init;
    const char *name = clks.name;
    const char *parent_name = clks.parent_name;
    int ret;
    periph_clk = kzalloc_obj(*periph_clk);
    if (WARN_ON(!periph_clk))
    return core::ptr::null_mut();
    periph_clk.hw.reg = regbase + clks.offset;
    periph_clk.shift = clks.shift;
    init.name = name;
    init.ops = &n5x_peri_c_clk_ops;
    init.flags = clks.flags;
    init.num_parents = clks.num_parents;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    periph_clk.hw.hw.init = &init;
    hw_clk = &periph_clk.hw.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if (ret) {
    kfree(periph_clk);
    return ERR_PTR(ret);
    }
    return hw_clk;
    }
    struct clk_hw *s10_register_cnt_periph(const struct stratix10_perip_cnt_clock *clks,
    void __iomem *regbase)
    {
    struct clk_hw *hw_clk;
    struct socfpga_periph_clk *periph_clk;
    struct clk_init_data init;
    const char *name = clks.name;
    const char *parent_name = clks.parent_name;
    int ret;
    periph_clk = kzalloc_obj(*periph_clk);
    if (WARN_ON(!periph_clk))
    return core::ptr::null_mut();
    if (clks.offset)
    periph_clk.hw.reg = regbase + clks.offset;
    else
    periph_clk.hw.reg = core::ptr::null_mut();
    if (clks.bypass_reg)
    periph_clk.bypass_reg = regbase + clks.bypass_reg;
    else
    periph_clk.bypass_reg = core::ptr::null_mut();
    periph_clk.bypass_shift = clks.bypass_shift;
    periph_clk.fixed_div = clks.fixed_divider;
    init.name = name;
    init.ops = &peri_cnt_clk_ops;
    init.flags = clks.flags;
    init.num_parents = clks.num_parents;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    if (init.parent_names == core::ptr::null_mut())
    init.parent_data = clks.parent_data;
    periph_clk.hw.hw.init = &init;
    hw_clk = &periph_clk.hw.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if (ret) {
    kfree(periph_clk);
    return ERR_PTR(ret);
    }
    return hw_clk;
    }
    struct clk_hw *agilex5_register_cnt_periph(const struct agilex5_perip_cnt_clock *clks,
    void __iomem *regbase)
    {
    struct clk_hw *hw_clk;
    struct socfpga_periph_clk *periph_clk;
    struct clk_init_data init;
    const char *name = clks.name;
    int ret;
    periph_clk = kzalloc_obj(*periph_clk);
    if (WARN_ON(!periph_clk))
    return core::ptr::null_mut();
    if (clks.offset)
    periph_clk.hw.reg = regbase + clks.offset;
    else
    periph_clk.hw.reg = core::ptr::null_mut();
    if (clks.bypass_reg)
    periph_clk.bypass_reg = regbase + clks.bypass_reg;
    else
    periph_clk.bypass_reg = core::ptr::null_mut();
    periph_clk.bypass_shift = clks.bypass_shift;
    periph_clk.fixed_div = clks.fixed_divider;
    init.name = name;
    init.ops = &peri_cnt_clk_ops;
    init.flags = clks.flags;
    init.num_parents = clks.num_parents;
    init.parent_names = clks.parent_names;
    periph_clk.hw.hw.init = &init;
    hw_clk = &periph_clk.hw.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if (ret) {
    kfree(periph_clk);
    return ERR_PTR(ret);
    }
    return hw_clk;
    }
