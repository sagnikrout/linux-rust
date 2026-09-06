//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-composite-8m.c
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
// Copyright 2018 NXP
//

pub const PCG_PREDIV_SHIFT: c_int = 16;
pub const PCG_PREDIV_WIDTH: c_int = 3;
pub const PCG_PREDIV_MAX: c_int = 8;
pub const PCG_DIV_SHIFT: c_int = 0;
pub const PCG_CORE_DIV_WIDTH: c_int = 3;
pub const PCG_DIV_WIDTH: c_int = 6;
pub const PCG_DIV_MAX: c_int = 64;
pub const PCG_PCS_SHIFT: c_int = 24;
pub const PCG_PCS_MASK: c_uint = 0x7;
pub const PCG_CGC_SHIFT: c_int = 28;
    static unsigned long imx8m_clk_composite_divider_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_divider *divider = to_clk_divider(hw);
    unsigned long prediv_rate;
    unsigned int prediv_value;
    unsigned int div_value;
    prediv_value = readl(divider.reg) >> divider.shift;
    prediv_value &= clk_div_mask(divider.width);
    prediv_rate = divider_recalc_rate(hw, parent_rate, prediv_value,
    core::ptr::null_mut(), divider.flags,
    divider.width);
    div_value = readl(divider.reg) >> PCG_DIV_SHIFT;
    div_value &= clk_div_mask(PCG_DIV_WIDTH);
    return divider_recalc_rate(hw, prediv_rate, div_value, core::ptr::null_mut(),
    divider.flags, PCG_DIV_WIDTH);
    }
    static int imx8m_clk_composite_compute_dividers(unsigned long rate,
    unsigned long parent_rate,
    int *prediv, int *postdiv)
    {
    int div1, div2;
    let mut error: c_int = INT_MAX;
    let mut ret: c_int = -EINVAL;
// prediv = 1;
// postdiv = 1;
    for (div1 = 1; div1 <= PCG_PREDIV_MAX; div1++) {
    for (div2 = 1; div2 <= PCG_DIV_MAX; div2++) {
    let mut new_error: c_int = ((parent_rate / div1) / div2) - rate;
    if (abs(new_error) < abs(error)) {
// prediv = div1;
// postdiv = div2;
    error = new_error;
    ret = 0;
    }
    }
    }
    return ret;
    }
    static int imx8m_clk_composite_divider_set_rate(struct clk_hw *hw,
    unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_divider *divider = to_clk_divider(hw);
    unsigned long flags;
    int prediv_value;
    int div_value;
    int ret;
    u32 orig, val;
    ret = imx8m_clk_composite_compute_dividers(rate, parent_rate,
    &prediv_value, &div_value);
    if (ret)
    return -EINVAL;
    spin_lock_irqsave(divider.lock, flags);
    orig = readl(divider.reg);
    val = orig & ~((clk_div_mask(divider.width) << divider.shift) |
    (clk_div_mask(PCG_DIV_WIDTH) << PCG_DIV_SHIFT));
    val |= (u32)(prediv_value  - 1) << divider.shift;
    val |= (u32)(div_value - 1) << PCG_DIV_SHIFT;
    if (val != orig)
    writel(val, divider.reg);
    spin_unlock_irqrestore(divider.lock, flags);
    return ret;
    }
    static int imx8m_divider_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_divider *divider = to_clk_divider(hw);
    int prediv_value;
    int div_value;
// if read only, just return current value
    if (divider.flags & CLK_DIVIDER_READ_ONLY) {
    u32 val;
    val = readl(divider.reg);
    prediv_value = val >> divider.shift;
    prediv_value &= clk_div_mask(divider.width);
    prediv_value++;
    div_value = val >> PCG_DIV_SHIFT;
    div_value &= clk_div_mask(PCG_DIV_WIDTH);
    div_value++;
    return divider_ro_determine_rate(hw, req, divider.table,
    PCG_PREDIV_WIDTH + PCG_DIV_WIDTH,
    divider.flags, prediv_value * div_value);
    }
    return divider_determine_rate(hw, req, divider.table,
    PCG_PREDIV_WIDTH + PCG_DIV_WIDTH,
    divider.flags);
    }
    static const struct clk_ops imx8m_clk_composite_divider_ops = {
    .recalc_rate = imx8m_clk_composite_divider_recalc_rate,
    .set_rate = imx8m_clk_composite_divider_set_rate,
    .determine_rate = imx8m_divider_determine_rate,
    };
#[no_mangle]
unsafe extern "C" fn imx8m_clk_composite_mux_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 imx8m_clk_composite_mux_get_parent(struct clk_hw *hw)
    {
    return clk_mux_ops.get_parent(hw);
    }
#[no_mangle]
unsafe extern "C" fn imx8m_clk_composite_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int imx8m_clk_composite_mux_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_mux *mux = to_clk_mux(hw);
    let mut val: u32 = clk_mux_index_to_val(mux.table, mux.flags, index);
    let mut flags: c_ulong = 0;
    u32 reg;
    if (mux.lock)
    spin_lock_irqsave(mux.lock, flags);
    reg = readl(mux.reg);
    reg &= ~(mux.mask << mux.shift);
    val = val << mux.shift;
    reg |= val;
//
// write twice to make sure non-target interface
// SEL_A/B point the same clk input.
//
    writel(reg, mux.reg);
    writel(reg, mux.reg);
    if (mux.lock)
    spin_unlock_irqrestore(mux.lock, flags);
    return 0;
    }
    static int
    imx8m_clk_composite_mux_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    return clk_mux_ops.determine_rate(hw, req);
    }
    static const struct clk_ops imx8m_clk_composite_mux_ops = {
    .get_parent = imx8m_clk_composite_mux_get_parent,
    .set_parent = imx8m_clk_composite_mux_set_parent,
    .determine_rate = imx8m_clk_composite_mux_determine_rate,
    };
#[no_mangle]
unsafe extern "C" fn imx8m_clk_composite_gate_enable(hw: *mut clk_hw) -> c_int {
    static int imx8m_clk_composite_gate_enable(struct clk_hw *hw)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(gate.lock, flags);
    val = readl(gate.reg);
    val |= BIT(gate.bit_idx);
    writel(val, gate.reg);
    spin_unlock_irqrestore(gate.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8m_clk_composite_gate_disable(hw: *mut clk_hw) {
    static void imx8m_clk_composite_gate_disable(struct clk_hw *hw)
    {
// composite clk requires the disable hook
    }
    static const struct clk_ops imx8m_clk_composite_gate_ops = {
    .enable = imx8m_clk_composite_gate_enable,
    .disable = imx8m_clk_composite_gate_disable,
    .is_enabled = clk_gate_is_enabled,
    };
    struct clk_hw *__imx8m_clk_hw_composite(const char *name,
    const char * const *parent_names,
    int num_parents, void __iomem *reg,
    u32 composite_flags,
    unsigned long flags)
    {
    struct clk_hw *hw = ERR_PTR(-ENOMEM), *mux_hw;
    struct clk_hw *div_hw, *gate_hw = core::ptr::null_mut();
    struct clk_divider *div;
    struct clk_gate *gate = core::ptr::null_mut();
    struct clk_mux *mux;
    const struct clk_ops *divider_ops;
    const struct clk_ops *mux_ops;
    const struct clk_ops *gate_ops;
    mux = kzalloc_obj(*mux);
    if (!mux)
    return ERR_CAST(hw);
    mux_hw = &mux.hw;
    mux.reg = reg;
    mux.shift = PCG_PCS_SHIFT;
    mux.mask = PCG_PCS_MASK;
    mux.lock = &imx_ccm_lock;
    div = kzalloc_obj(*div);
    if (!div)
    goto free_mux;
    div_hw = &div.hw;
    div.reg = reg;
    if (composite_flags & IMX_COMPOSITE_CORE) {
    div.shift = PCG_DIV_SHIFT;
    div.width = PCG_CORE_DIV_WIDTH;
    divider_ops = &clk_divider_ops;
    mux_ops = &imx8m_clk_composite_mux_ops;
    } else if (composite_flags & IMX_COMPOSITE_BUS) {
    div.shift = PCG_PREDIV_SHIFT;
    div.width = PCG_PREDIV_WIDTH;
    divider_ops = &imx8m_clk_composite_divider_ops;
    mux_ops = &imx8m_clk_composite_mux_ops;
    } else {
    div.shift = PCG_PREDIV_SHIFT;
    div.width = PCG_PREDIV_WIDTH;
    divider_ops = &imx8m_clk_composite_divider_ops;
    mux_ops = &clk_mux_ops;
    if (!(composite_flags & IMX_COMPOSITE_FW_MANAGED))
    flags |= CLK_SET_PARENT_GATE;
    }
    div.lock = &imx_ccm_lock;
    div.flags = CLK_DIVIDER_ROUND_CLOSEST;
// skip registering the gate ops if M4 is enabled
    gate = kzalloc_obj(*gate);
    if (!gate)
    goto free_div;
    gate_hw = &gate.hw;
    gate.reg = reg;
    gate.bit_idx = PCG_CGC_SHIFT;
    gate.lock = &imx_ccm_lock;
    if (!mcore_booted)
    gate_ops = &clk_gate_ops;
    else
    gate_ops = &imx8m_clk_composite_gate_ops;
    hw = clk_hw_register_composite(core::ptr::null_mut(), name, parent_names, num_parents,
    mux_hw, mux_ops, div_hw,
    divider_ops, gate_hw, gate_ops, flags);
    if (IS_ERR(hw))
    goto free_gate;
    return hw;
    free_gate:
    kfree(gate);
    free_div:
    kfree(div);
    free_mux:
    kfree(mux);
    return ERR_CAST(hw);
    }
    EXPORT_SYMBOL_GPL(__imx8m_clk_hw_composite);
