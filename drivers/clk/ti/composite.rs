//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ti/composite.c
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
// TI composite clock support
//
// Copyright (C) 2013 Texas Instruments, Inc.
//
// Tero Kristo <t-kristo@ti.com>
//

    static unsigned long ti_composite_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return ti_clk_divider_ops.recalc_rate(hw, parent_rate);
    }
    static int ti_composite_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    return -EINVAL;
    }
    static int ti_composite_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    return -EINVAL;
    }
    static const struct clk_ops ti_composite_divider_ops = {
    .recalc_rate	= &ti_composite_recalc_rate,
    .determine_rate	= &ti_composite_determine_rate,
    .set_rate	= &ti_composite_set_rate,
    };
    static const struct clk_ops ti_composite_gate_ops = {
    .enable		= &omap2_dflt_clk_enable,
    .disable	= &omap2_dflt_clk_disable,
    .is_enabled	= &omap2_dflt_clk_is_enabled,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct component_clk {
    pub num_parents: c_int,
    pub parent_data: *mut clk_parent_data,
    pub node: *mut device_node,
    pub type: c_int,
    pub hw: *mut clk_hw,
    pub link: list_head,
}

    static const char * const component_clk_types[] __initconst = {
    "gate", "divider", "mux"
    };
    static LIST_HEAD(component_clks);
    static struct device_node *_get_component_node(struct device_node *node, int i)
    {
    int rc;
    struct of_phandle_args clkspec;
    rc = of_parse_phandle_with_args(node, "clocks", "#clock-cells", i,
    &clkspec);
    if (rc)
    return core::ptr::null_mut();
    return clkspec.np;
    }
    static struct component_clk *_lookup_component(struct device_node *node)
    {
    struct component_clk *comp;
    list_for_each_entry(comp, &component_clks, link) {
    if (comp.node == node)
    return comp;
    }
    return core::ptr::null_mut();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_hw_omap_comp {
    pub hw: clk_hw,
    pub comp_nodes: [*mut device_node; CLK_COMPONENT_TYPE_MAX],
    pub comp_clks: [*mut component_clk; CLK_COMPONENT_TYPE_MAX],
}

    static inline struct clk_hw *_get_hw(struct clk_hw_omap_comp *clk, int idx)
    {
    if (!clk)
    return core::ptr::null_mut();
    if (!clk.comp_clks[idx])
    return core::ptr::null_mut();
    return clk.comp_clks[idx].hw;
    }

    static void __init _register_composite(void *user,
    struct device_node *node)
    {
    struct clk_hw *hw = user;
    struct clk *clk;
    struct clk_hw_omap_comp *cclk = to_clk_hw_comp(hw);
    struct component_clk *comp;
    let mut num_parents: c_int = 0;
    struct clk_parent_data *parent_data = core::ptr::null_mut();
    const char *name;
    int i;
    int ret;
// Check for presence of each component clock
    for (i = 0; i < CLK_COMPONENT_TYPE_MAX; i++) {
    if (!cclk.comp_nodes[i])
    continue;
    comp = _lookup_component(cclk.comp_nodes[i]);
    if (!comp) {
    pr_debug("component %s not ready for %pOFn, retry\n",
    cclk.comp_nodes[i].name, node);
    if (!ti_clk_retry_init(node, hw,
    _register_composite))
    return;
    goto cleanup;
    }
    if (cclk.comp_clks[comp.type] != core::ptr::null_mut()) {
    pr_err("duplicate component types for %pOFn (%s)!\n",
    node, component_clk_types[comp.type]);
    goto cleanup;
    }
    cclk.comp_clks[comp.type] = comp;
// Mark this node as found
    cclk.comp_nodes[i] = core::ptr::null_mut();
    }
// All components exists, proceed with registration
    for (i = CLK_COMPONENT_TYPE_MAX - 1; i >= 0; i--) {
    comp = cclk.comp_clks[i];
    if (!comp)
    continue;
    if (comp.num_parents) {
    num_parents = comp.num_parents;
    parent_data = comp.parent_data;
    break;
    }
    }
    if (!num_parents) {
    pr_err("%s: no parents found for %pOFn!\n", __func__, node);
    goto cleanup;
    }
    name = ti_dt_clk_name(node);
    clk = clk_register_composite_pdata(core::ptr::null_mut(), name,
    parent_data, num_parents,
    _get_hw(cclk, CLK_COMPONENT_TYPE_MUX),
    &ti_clk_mux_ops,
    _get_hw(cclk, CLK_COMPONENT_TYPE_DIVIDER),
    &ti_composite_divider_ops,
    _get_hw(cclk, CLK_COMPONENT_TYPE_GATE),
    &ti_composite_gate_ops, 0);
    if (!IS_ERR(clk)) {
    ret = ti_clk_add_alias(clk, name);
    if (ret) {
    clk_unregister(clk);
    goto cleanup;
    }
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    }
    cleanup:
// Free component clock list entries
    for (i = 0; i < CLK_COMPONENT_TYPE_MAX; i++) {
    if (!cclk.comp_clks[i])
    continue;
    list_del(&cclk.comp_clks[i].link);
    kfree(cclk.comp_clks[i].parent_data);
    kfree(cclk.comp_clks[i]);
    }
    kfree(cclk);
    }
#[no_mangle]
unsafe extern "C" fn of_ti_composite_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_ti_composite_clk_setup(struct device_node *node)
    {
    unsigned int num_clks;
    int i;
    struct clk_hw_omap_comp *cclk;
// Number of component clocks to be put inside this clock
    num_clks = of_clk_get_parent_count(node);
    if (!num_clks) {
    pr_err("composite clk %pOFn must have component(s)\n", node);
    return;
    }
    cclk = kzalloc_obj(*cclk);
    if (!cclk)
    return;
// Get device node pointers for each component clock
    for (i = 0; i < num_clks; i++)
    cclk.comp_nodes[i] = _get_component_node(node, i);
    _register_composite(&cclk.hw, node);
    }
    CLK_OF_DECLARE(ti_composite_clock, "ti,composite-clock",
    of_ti_composite_clk_setup);
//
// ti_clk_add_component - add a component clock to the pool
// @node: device node of the component clock
// @hw: hardware clock definition for the component clock
// @type: type of the component clock
//
// Adds a component clock to the list of available components, so that
// it can be registered by a composite clock.
//
    int __init ti_clk_add_component(struct device_node *node, struct clk_hw *hw,
    int type)
    {
    unsigned int num_parents;
    struct clk_parent_data *parent_data;
    struct component_clk *clk;
    unsigned int i;
    num_parents = of_clk_get_parent_count(node);
    if (!num_parents) {
    pr_err("component-clock %pOFn must have parent(s)\n", node);
    return -EINVAL;
    }
    parent_data = kzalloc_objs(*parent_data, num_parents);
    if (!parent_data)
    return -ENOMEM;
    for (i = 0; i < num_parents; i++)
    parent_data[i].index = i;
    clk = kzalloc_obj(*clk);
    if (!clk) {
    kfree(parent_data);
    return -ENOMEM;
    }
    clk.num_parents = num_parents;
    clk.parent_data = parent_data;
    clk.hw = hw;
    clk.node = node;
    clk.type = type;
    list_add(&clk.link, &component_clks);
    return 0;
    }
