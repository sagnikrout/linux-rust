//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ti/clockdomain.c
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
// OMAP clockdomain support
//
// Copyright (C) 2013 Texas Instruments, Inc.
//
// Tero Kristo <t-kristo@ti.com>
//

//
// omap2_clkops_enable_clkdm - increment usecount on clkdm of @hw
// @hw: struct clk_hw * of the clock being enabled
//
// Increment the usecount of the clockdomain of the clock pointed to
// by @hw; if the usecount is 1, the clockdomain will be "enabled."
// Only needed for clocks that don't use omap2_dflt_clk_enable() as
// their enable function pointer.  Passes along the return value of
// clkdm_clk_enable(), -EINVAL if @hw is not associated with a
// clockdomain, or 0 if clock framework-based clockdomain control is
// not implemented.
//
#[no_mangle]
pub unsafe extern "C" fn omap2_clkops_enable_clkdm(hw: *mut clk_hw) -> c_int {
    int omap2_clkops_enable_clkdm(struct clk_hw *hw)
    {
    struct clk_hw_omap *clk;
    let mut ret: c_int = 0;
    clk = to_clk_hw_omap(hw);
    if (unlikely(!clk.clkdm)) {
    pr_err("%s: %s: no clkdm set ?!\n", __func__,
    clk_hw_get_name(hw));
    return -EINVAL;
    }
    if (ti_clk_get_features().flags & TI_CLK_DISABLE_CLKDM_CONTROL) {
    pr_err("%s: %s: clkfw-based clockdomain control disabled ?!\n",
    __func__, clk_hw_get_name(hw));
    return 0;
    }
    ret = ti_clk_ll_ops.clkdm_clk_enable(clk.clkdm, hw.clk);
    WARN(ret, "%s: could not enable %s's clockdomain %s: %d\n",
    __func__, clk_hw_get_name(hw), clk.clkdm_name, ret);
    return ret;
    }
//
// omap2_clkops_disable_clkdm - decrement usecount on clkdm of @hw
// @hw: struct clk_hw * of the clock being disabled
//
// Decrement the usecount of the clockdomain of the clock pointed to
// by @hw; if the usecount is 0, the clockdomain will be "disabled."
// Only needed for clocks that don't use omap2_dflt_clk_disable() as their
// disable function pointer.  No return value.
//
#[no_mangle]
pub unsafe extern "C" fn omap2_clkops_disable_clkdm(hw: *mut clk_hw) {
    void omap2_clkops_disable_clkdm(struct clk_hw *hw)
    {
    struct clk_hw_omap *clk;
    clk = to_clk_hw_omap(hw);
    if (unlikely(!clk.clkdm)) {
    pr_err("%s: %s: no clkdm set ?!\n", __func__,
    clk_hw_get_name(hw));
    return;
    }
    if (ti_clk_get_features().flags & TI_CLK_DISABLE_CLKDM_CONTROL) {
    pr_err("%s: %s: clkfw-based clockdomain control disabled ?!\n",
    __func__, clk_hw_get_name(hw));
    return;
    }
    ti_clk_ll_ops.clkdm_clk_disable(clk.clkdm, hw.clk);
    }
//
// omap2_init_clk_clkdm - look up a clockdomain name, store pointer in clk
// @hw: Pointer to clk_hw_omap used to obtain OMAP clock struct ptr to use
//
// Convert a clockdomain name stored in a struct clk 'clk' into a
// clockdomain pointer, and save it into the struct clk.  Intended to be
// called during clk_register(). Returns 0 on success, -EERROR otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn omap2_init_clk_clkdm(hw: *mut clk_hw) -> c_int {
    int omap2_init_clk_clkdm(struct clk_hw *hw)
    {
    struct clk_hw_omap *clk = to_clk_hw_omap(hw);
    struct clockdomain *clkdm;
    const char *clk_name;
    if (!clk.clkdm_name)
    return 0;
    clk_name = __clk_get_name(hw.clk);
    clkdm = ti_clk_ll_ops.clkdm_lookup(clk.clkdm_name);
    if (clkdm) {
    pr_debug("clock: associated clk %s to clkdm %s\n",
    clk_name, clk.clkdm_name);
    clk.clkdm = clkdm;
    } else {
    pr_debug("clock: could not associate clk %s to clkdm %s\n",
    clk_name, clk.clkdm_name);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn of_ti_clockdomain_setup(node: *mut device_node) -> void __init {
    static void __init of_ti_clockdomain_setup(struct device_node *node)
    {
    struct clk *clk;
    struct clk_hw *clk_hw;
    const char *clkdm_name = ti_dt_clk_name(node);
    int i;
    unsigned int num_clks;
    num_clks = of_clk_get_parent_count(node);
    for (i = 0; i < num_clks; i++) {
    clk = of_clk_get(node, i);
    if (IS_ERR(clk)) {
    pr_err("%s: Failed get %pOF' clock nr %d (%ld)\n",
    __func__, node, i, PTR_ERR(clk));
    continue;
    }
    clk_hw = __clk_get_hw(clk);
    if (!omap2_clk_is_hw_omap(clk_hw)) {
    pr_warn("can't setup clkdm for basic clk %s\n",
    __clk_get_name(clk));
    clk_put(clk);
    continue;
    }
    to_clk_hw_omap(clk_hw).clkdm_name = clkdm_name;
    omap2_init_clk_clkdm(clk_hw);
    clk_put(clk);
    }
    }
    static const struct of_device_id ti_clkdm_match_table[] __initconst = {
    { .compatible = "ti,clockdomain" },
    { }
    };
//
// ti_dt_clockdomains_setup - setup device tree clockdomains
//
// Initializes clockdomain nodes for a SoC. This parses through all the
// nodes with compatible = "ti,clockdomain", and add the clockdomain
// info for all the clocks listed under these. This function shall be
// called after rest of the DT clock init has completed and all
// clock nodes have been registered.
//
#[no_mangle]
pub unsafe extern "C" fn ti_dt_clockdomains_setup() -> void __init {
    void __init ti_dt_clockdomains_setup(void)
    {
    struct device_node *np;
    for_each_matching_node(np, ti_clkdm_match_table) {
    of_ti_clockdomain_setup(np);
    }
    }
