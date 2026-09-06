//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ti/mux.c
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
// TI Multiplexer Clock
//
// Copyright (C) 2013 Texas Instruments, Inc.
//
// Tero Kristo <t-kristo@ti.com>
//

#[no_mangle]
unsafe extern "C" fn ti_clk_mux_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 ti_clk_mux_get_parent(struct clk_hw *hw)
    {
    struct clk_omap_mux *mux = to_clk_omap_mux(hw);
    let mut num_parents: c_int = clk_hw_get_num_parents(hw);
    u32 val;
//
// FIXME need a mux-specific flag to determine if val is bitwise or
// numeric. e.g. sys_clkin_ck's clksel field is 3 bits wide, but ranges
// from 0x1 to 0x7 (index starts at one)
// OTOH, pmd_trace_clk_mux_ck uses a separate bit for each clock, so
// val = 0x4 really means "bit 2, index starts at bit 0"
//
    val = ti_clk_ll_ops.clk_readl(&mux.reg) >> mux.shift;
    val &= mux.mask;
    if (mux.table) {
    int i;
    for (i = 0; i < num_parents; i++)
    if (mux.table[i] == val)
    return i;
    return -EINVAL;
    }
    if (val && (mux.flags & CLK_MUX_INDEX_BIT))
    val = ffs(val) - 1;
    if (val && (mux.flags & CLK_MUX_INDEX_ONE))
    val--;
    if (val >= num_parents)
    return -EINVAL;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn ti_clk_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int ti_clk_mux_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_omap_mux *mux = to_clk_omap_mux(hw);
    u32 val;
    if (mux.table) {
    index = mux.table[index];
    } else {
    if (mux.flags & CLK_MUX_INDEX_BIT)
    index = (1 << ffs(index));
    if (mux.flags & CLK_MUX_INDEX_ONE)
    index++;
    }
    if (mux.flags & CLK_MUX_HIWORD_MASK) {
    val = mux.mask << (mux.shift + 16);
    } else {
    val = ti_clk_ll_ops.clk_readl(&mux.reg);
    val &= ~(mux.mask << mux.shift);
    }
    val |= index << mux.shift;
    ti_clk_ll_ops.clk_writel(val, &mux.reg);
    ti_clk_latch(&mux.reg, mux.latch);
    return 0;
    }
//
// clk_mux_save_context - Save the parent selected in the mux
// @hw: pointer  struct clk_hw
//
// Save the parent mux value.
//
#[no_mangle]
unsafe extern "C" fn clk_mux_save_context(hw: *mut clk_hw) -> c_int {
    static int clk_mux_save_context(struct clk_hw *hw)
    {
    struct clk_omap_mux *mux = to_clk_omap_mux(hw);
    mux.saved_parent = ti_clk_mux_get_parent(hw);
    return 0;
    }
//
// clk_mux_restore_context - Restore the parent in the mux
// @hw: pointer  struct clk_hw
//
// Restore the saved parent mux value.
//
#[no_mangle]
unsafe extern "C" fn clk_mux_restore_context(hw: *mut clk_hw) {
    static void clk_mux_restore_context(struct clk_hw *hw)
    {
    struct clk_omap_mux *mux = to_clk_omap_mux(hw);
    ti_clk_mux_set_parent(hw, mux.saved_parent);
    }
    const struct clk_ops ti_clk_mux_ops = {
    .get_parent = ti_clk_mux_get_parent,
    .set_parent = ti_clk_mux_set_parent,
    .determine_rate = __clk_mux_determine_rate,
    .save_context = clk_mux_save_context,
    .restore_context = clk_mux_restore_context,
    };
    static struct clk *_register_mux(struct device_node *node, const char *name,
    const struct clk_parent_data *parent_data,
    u8 num_parents, unsigned long flags,
    struct clk_omap_reg *reg, u8 shift, u32 mask,
    s8 latch, u8 clk_mux_flags, u32 *table)
    {
    let mut init: clk_init_data = {};
    struct clk_omap_mux *mux;
    struct clk *clk;
// allocate the mux
    mux = kzalloc_obj(*mux);
    if (!mux)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &ti_clk_mux_ops;
    init.flags = flags;
    init.parent_data = parent_data;
    init.num_parents = num_parents;
// struct clk_mux assignments
    memcpy(&mux.reg, reg, sizeof(*reg));
    mux.shift = shift;
    mux.mask = mask;
    mux.latch = latch;
    mux.flags = clk_mux_flags;
    mux.table = table;
    mux.hw.init = &init;
    clk = of_ti_clk_register(node, &mux.hw, name);
    if (IS_ERR(clk))
    kfree(mux);
    return clk;
    }
//
// of_mux_clk_setup - Setup function for simple mux rate clock
// @node: DT node for the clock
//
// Sets up a basic clock multiplexer.
//
#[no_mangle]
unsafe extern "C" fn of_mux_clk_setup(node: *mut device_node) {
    static void of_mux_clk_setup(struct device_node *node)
    {
    struct clk *clk;
    struct clk_omap_reg reg;
    unsigned int num_parents;
    struct clk_parent_data *parent_data;
    const char *name;
    let mut clk_mux_flags: u8 = 0;
    let mut mask: u32 = 0;
    let mut shift: u32 = 0;
    let mut latch: i32 = -EINVAL;
    let mut flags: u32 = CLK_SET_RATE_NO_REPARENT;
    int i;
    num_parents = of_clk_get_parent_count(node);
    if (num_parents < 2) {
    pr_err("mux-clock %pOFn must have parents\n", node);
    return;
    }
    parent_data = kzalloc_objs(*parent_data, num_parents);
    if (!parent_data)
    return;
    for (i = 0; i < num_parents; i++)
    parent_data[i].index = i;
    if (ti_clk_get_reg_addr(node, 0, &reg))
    goto cleanup;
    shift = reg.bit;
    of_property_read_u32(node, "ti,latch-bit", &latch);
    if (of_property_read_bool(node, "ti,index-starts-at-one"))
    clk_mux_flags |= CLK_MUX_INDEX_ONE;
    if (of_property_read_bool(node, "ti,set-rate-parent"))
    flags |= CLK_SET_RATE_PARENT;
// Generate bit-mask based on parent info
    mask = num_parents;
    if (!(clk_mux_flags & CLK_MUX_INDEX_ONE))
    mask--;
    mask = (1 << fls(mask)) - 1;
    name = ti_dt_clk_name(node);
    clk = _register_mux(node, name, parent_data, num_parents,
    flags, &reg, shift, mask, latch, clk_mux_flags,
    core::ptr::null_mut());
    if (!IS_ERR(clk))
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    cleanup:
    kfree(parent_data);
    }
    CLK_OF_DECLARE(mux_clk, "ti,mux-clock", of_mux_clk_setup);
    struct clk_hw *ti_clk_build_component_mux(struct ti_clk_mux *setup)
    {
    struct clk_omap_mux *mux;
    int num_parents;
    if (!setup)
    return core::ptr::null_mut();
    mux = kzalloc_obj(*mux);
    if (!mux)
    return ERR_PTR(-ENOMEM);
    mux.shift = setup.bit_shift;
    mux.latch = -EINVAL;
    mux.reg.index = setup.module;
    mux.reg.offset = setup.reg;
    if (setup.flags & CLKF_INDEX_STARTS_AT_ONE)
    mux.flags |= CLK_MUX_INDEX_ONE;
    num_parents = setup.num_parents;
    mux.mask = num_parents - 1;
    mux.mask = (1 << fls(mux.mask)) - 1;
    return &mux.hw;
    }
#[no_mangle]
unsafe extern "C" fn of_ti_composite_mux_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_ti_composite_mux_clk_setup(struct device_node *node)
    {
    struct clk_omap_mux *mux;
    unsigned int num_parents;
    mux = kzalloc_obj(*mux);
    if (!mux)
    return;
    if (ti_clk_get_reg_addr(node, 0, &mux.reg))
    goto cleanup;
    mux.shift = mux.reg.bit;
    if (of_property_read_bool(node, "ti,index-starts-at-one"))
    mux.flags |= CLK_MUX_INDEX_ONE;
    num_parents = of_clk_get_parent_count(node);
    if (num_parents < 2) {
    pr_err("%pOFn must have parents\n", node);
    goto cleanup;
    }
    mux.mask = num_parents - 1;
    mux.mask = (1 << fls(mux.mask)) - 1;
    if (!ti_clk_add_component(node, &mux.hw, CLK_COMPONENT_TYPE_MUX))
    return;
    cleanup:
    kfree(mux);
    }
    CLK_OF_DECLARE(ti_composite_mux_clk_setup, "ti,composite-mux-clock",
    of_ti_composite_mux_clk_setup);
