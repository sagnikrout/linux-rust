//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ti/gate.c
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
// OMAP gate clock support
//
// Copyright (C) 2013 Texas Instruments, Inc.
//
// Tero Kristo <t-kristo@ti.com>
//

    static int omap36xx_gate_clk_enable_with_hsdiv_restore(struct clk_hw *clk);
    static const struct clk_ops omap_gate_clkdm_clk_ops = {
    .init		= &omap2_init_clk_clkdm,
    .enable		= &omap2_clkops_enable_clkdm,
    .disable	= &omap2_clkops_disable_clkdm,
    .restore_context = clk_gate_restore_context,
    };
    const struct clk_ops omap_gate_clk_ops = {
    .init		= &omap2_init_clk_clkdm,
    .enable		= &omap2_dflt_clk_enable,
    .disable	= &omap2_dflt_clk_disable,
    .is_enabled	= &omap2_dflt_clk_is_enabled,
    .restore_context = clk_gate_restore_context,
    };
    static const struct clk_ops omap_gate_clk_hsdiv_restore_ops = {
    .init		= &omap2_init_clk_clkdm,
    .enable		= &omap36xx_gate_clk_enable_with_hsdiv_restore,
    .disable	= &omap2_dflt_clk_disable,
    .is_enabled	= &omap2_dflt_clk_is_enabled,
    .restore_context = clk_gate_restore_context,
    };
//
// omap36xx_gate_clk_enable_with_hsdiv_restore - enable clocks suffering
// from HSDivider PWRDN problem Implements Errata ID: i556.
// @hw: DPLL output struct clk_hw
//
// 3630 only: dpll3_m3_ck, dpll4_m2_ck, dpll4_m3_ck, dpll4_m4_ck,
// dpll4_m5_ck & dpll4_m6_ck dividers gets loaded with reset
// valueafter their respective PWRDN bits are set.  Any dummy write
// (Any other value different from the Read value) to the
// corresponding CM_CLKSEL register will refresh the dividers.
//
#[no_mangle]
unsafe extern "C" fn omap36xx_gate_clk_enable_with_hsdiv_restore(hw: *mut clk_hw) -> c_int {
    static int omap36xx_gate_clk_enable_with_hsdiv_restore(struct clk_hw *hw)
    {
    struct clk_omap_divider *parent;
    struct clk_hw *parent_hw;
    u32 dummy_v, orig_v;
    int ret;
// Clear PWRDN bit of HSDIVIDER
    ret = omap2_dflt_clk_enable(hw);
// Parent is the x2 node, get parent of parent for the m2 div
    parent_hw = clk_hw_get_parent(clk_hw_get_parent(hw));
    parent = to_clk_omap_divider(parent_hw);
// Restore the dividers
    if (!ret) {
    orig_v = ti_clk_ll_ops.clk_readl(&parent.reg);
    dummy_v = orig_v;
// Write any other value different from the Read value
    dummy_v ^= (1 << parent.shift);
    ti_clk_ll_ops.clk_writel(dummy_v, &parent.reg);
// Write the original divider
    ti_clk_ll_ops.clk_writel(orig_v, &parent.reg);
    }
    return ret;
    }
    static struct clk *_register_gate(struct device_node *node, const char *name,
    const char *parent_name, unsigned long flags,
    struct clk_omap_reg *reg, u8 bit_idx,
    u8 clk_gate_flags, const struct clk_ops *ops,
    const struct clk_hw_omap_ops *hw_ops)
    {
    let mut init: clk_init_data = { core::ptr::null_mut() };
    struct clk_hw_omap *clk_hw;
    struct clk *clk;
    clk_hw = kzalloc_obj(*clk_hw);
    if (!clk_hw)
    return ERR_PTR(-ENOMEM);
    clk_hw.hw.init = &init;
    init.name = name;
    init.ops = ops;
    memcpy(&clk_hw.enable_reg, reg, sizeof(*reg));
    clk_hw.enable_bit = bit_idx;
    clk_hw.ops = hw_ops;
    clk_hw.flags = clk_gate_flags;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = flags;
    clk = of_ti_clk_register_omap_hw(node, &clk_hw.hw, name);
    if (IS_ERR(clk))
    kfree(clk_hw);
    return clk;
    }
    static void __init _of_ti_gate_clk_setup(struct device_node *node,
    const struct clk_ops *ops,
    const struct clk_hw_omap_ops *hw_ops)
    {
    struct clk *clk;
    const char *parent_name;
    struct clk_omap_reg reg;
    const char *name;
    let mut enable_bit: u8 = 0;
    let mut flags: u32 = 0;
    let mut clk_gate_flags: u8 = 0;
    if (ops != &omap_gate_clkdm_clk_ops) {
    if (ti_clk_get_reg_addr(node, 0, &reg))
    return;
    enable_bit = reg.bit;
    }
    if (of_clk_get_parent_count(node) != 1) {
    pr_err("%pOFn must have 1 parent\n", node);
    return;
    }
    parent_name = of_clk_get_parent_name(node, 0);
    if (of_property_read_bool(node, "ti,set-rate-parent"))
    flags |= CLK_SET_RATE_PARENT;
    if (of_property_read_bool(node, "ti,set-bit-to-disable"))
    clk_gate_flags |= INVERT_ENABLE;
    name = ti_dt_clk_name(node);
    clk = _register_gate(node, name, parent_name, flags, &reg,
    enable_bit, clk_gate_flags, ops, hw_ops);
    if (!IS_ERR(clk))
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    }
    static void __init
    _of_ti_composite_gate_clk_setup(struct device_node *node,
    const struct clk_hw_omap_ops *hw_ops)
    {
    struct clk_hw_omap *gate;
    gate = kzalloc_obj(*gate);
    if (!gate)
    return;
    if (ti_clk_get_reg_addr(node, 0, &gate.enable_reg))
    goto cleanup;
    gate.enable_bit = gate.enable_reg.bit;
    gate.ops = hw_ops;
    if (!ti_clk_add_component(node, &gate.hw, CLK_COMPONENT_TYPE_GATE))
    return;
    cleanup:
    kfree(gate);
    }
    static void __init
    of_ti_composite_no_wait_gate_clk_setup(struct device_node *node)
    {
    _of_ti_composite_gate_clk_setup(node, core::ptr::null_mut());
    }
    CLK_OF_DECLARE(ti_composite_no_wait_gate_clk, "ti,composite-no-wait-gate-clock",
    of_ti_composite_no_wait_gate_clk_setup);

#[no_mangle]
unsafe extern "C" fn of_ti_composite_interface_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_ti_composite_interface_clk_setup(struct device_node *node)
    {
    _of_ti_composite_gate_clk_setup(node, &clkhwops_iclk_wait);
    }
    CLK_OF_DECLARE(ti_composite_interface_clk, "ti,composite-interface-clock",
    of_ti_composite_interface_clk_setup);

#[no_mangle]
unsafe extern "C" fn of_ti_composite_gate_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_ti_composite_gate_clk_setup(struct device_node *node)
    {
    _of_ti_composite_gate_clk_setup(node, &clkhwops_wait);
    }
    CLK_OF_DECLARE(ti_composite_gate_clk, "ti,composite-gate-clock",
    of_ti_composite_gate_clk_setup);
#[no_mangle]
unsafe extern "C" fn of_ti_clkdm_gate_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_ti_clkdm_gate_clk_setup(struct device_node *node)
    {
    _of_ti_gate_clk_setup(node, &omap_gate_clkdm_clk_ops, core::ptr::null_mut());
    }
    CLK_OF_DECLARE(ti_clkdm_gate_clk, "ti,clkdm-gate-clock",
    of_ti_clkdm_gate_clk_setup);
#[no_mangle]
unsafe extern "C" fn of_ti_hsdiv_gate_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_ti_hsdiv_gate_clk_setup(struct device_node *node)
    {
    _of_ti_gate_clk_setup(node, &omap_gate_clk_hsdiv_restore_ops,
    &clkhwops_wait);
    }
    CLK_OF_DECLARE(ti_hsdiv_gate_clk, "ti,hsdiv-gate-clock",
    of_ti_hsdiv_gate_clk_setup);
#[no_mangle]
unsafe extern "C" fn of_ti_gate_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_ti_gate_clk_setup(struct device_node *node)
    {
    _of_ti_gate_clk_setup(node, &omap_gate_clk_ops, core::ptr::null_mut());
    }
    CLK_OF_DECLARE(ti_gate_clk, "ti,gate-clock", of_ti_gate_clk_setup);
#[no_mangle]
unsafe extern "C" fn of_ti_wait_gate_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_ti_wait_gate_clk_setup(struct device_node *node)
    {
    _of_ti_gate_clk_setup(node, &omap_gate_clk_ops, &clkhwops_wait);
    }
    CLK_OF_DECLARE(ti_wait_gate_clk, "ti,wait-gate-clock",
    of_ti_wait_gate_clk_setup);

#[no_mangle]
unsafe extern "C" fn of_ti_am35xx_gate_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_ti_am35xx_gate_clk_setup(struct device_node *node)
    {
    _of_ti_gate_clk_setup(node, &omap_gate_clk_ops,
    &clkhwops_am35xx_ipss_module_wait);
    }
    CLK_OF_DECLARE(ti_am35xx_gate_clk, "ti,am35xx-gate-clock",
    of_ti_am35xx_gate_clk_setup);
#[no_mangle]
unsafe extern "C" fn of_ti_dss_gate_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_ti_dss_gate_clk_setup(struct device_node *node)
    {
    _of_ti_gate_clk_setup(node, &omap_gate_clk_ops,
    &clkhwops_omap3430es2_dss_usbhost_wait);
    }
    CLK_OF_DECLARE(ti_dss_gate_clk, "ti,dss-gate-clock",
    of_ti_dss_gate_clk_setup);
