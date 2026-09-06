//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-fixed-factor.c
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
// Copyright (C) 2011 Sascha Hauer, Pengutronix <s.hauer@pengutronix.de>
//

//
// DOC: basic fixed multiplier and divider clock that cannot gate
//
// Traits of this clock:
// prepare - clk_prepare only ensures that parents are prepared
// enable - clk_enable only ensures that parents are enabled
// rate - rate is fixed.  clk->rate = parent->rate / div * mult
// parent - fixed parent.  No clk_set_parent support
//
    static unsigned long clk_factor_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_fixed_factor *fix = to_clk_fixed_factor(hw);
    unsigned long long int rate;
    rate = (unsigned long long int)parent_rate * fix.mult;
    do_div(rate, fix.div);
    return (unsigned long)rate;
    }
    static int clk_factor_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_fixed_factor *fix = to_clk_fixed_factor(hw);
    if (clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT) {
    struct clk_hw *parent_hw = clk_hw_get_parent(hw);
    unsigned long best_parent;
    if (!parent_hw)
    return -EINVAL;
    best_parent = (req.rate / fix.mult) * fix.div;
    req.best_parent_rate = clk_hw_round_rate(parent_hw, best_parent);
    }
    req.rate = (req.best_parent_rate / fix.div) * fix.mult;
    return 0;
    }
    static int clk_factor_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
//
// We must report success but we can do so unconditionally because
// clk_factor_determine_rate returns values that ensure this call is a
// nop.
//
    return 0;
    }
    static unsigned long clk_factor_recalc_accuracy(struct clk_hw *hw,
    unsigned long parent_accuracy)
    {
    struct clk_fixed_factor *fix = to_clk_fixed_factor(hw);
    if (fix.flags & CLK_FIXED_FACTOR_FIXED_ACCURACY)
    return fix.acc;
    return parent_accuracy;
    }
    const struct clk_ops clk_fixed_factor_ops = {
    .determine_rate = clk_factor_determine_rate,
    .set_rate = clk_factor_set_rate,
    .recalc_rate = clk_factor_recalc_rate,
    .recalc_accuracy = clk_factor_recalc_accuracy,
    };
    EXPORT_SYMBOL_GPL(clk_fixed_factor_ops);
#[no_mangle]
unsafe extern "C" fn devm_clk_hw_register_fixed_factor_release(dev: *mut device, res: *mut c_void) {
    static void devm_clk_hw_register_fixed_factor_release(struct device *dev, void *res)
    {
    struct clk_fixed_factor *fix = res;
//
// We can not use clk_hw_unregister_fixed_factor, since it will kfree()
// the hw, resulting in double free. Just unregister the hw and let
// devres code kfree() it.
//
    clk_hw_unregister(&fix.hw);
    }
    struct clk_hw *
    __clk_hw_register_fixed_factor(struct device *dev, struct device_node *np,
    const char *name, const char *parent_name,
    const struct clk_hw *parent_hw, const struct clk_parent_data *pdata,
    unsigned long flags, unsigned int mult, unsigned int div,
    unsigned long acc, unsigned int fixflags, bool devm)
    {
    struct clk_fixed_factor *fix;
    let mut init: clk_init_data = { };
    struct clk_hw *hw;
    int ret;
// You can't use devm without a dev
    if (devm && !dev)
    return ERR_PTR(-EINVAL);
    if (devm)
    fix = devres_alloc(devm_clk_hw_register_fixed_factor_release,
    sizeof(*fix), GFP_KERNEL);
    else
    fix = kmalloc_obj(*fix);
    if (!fix)
    return ERR_PTR(-ENOMEM);
// struct clk_fixed_factor assignments
    fix.mult = mult;
    fix.div = div;
    fix.hw.init = &init;
    fix.acc = acc;
    fix.flags = fixflags;
    init.name = name;
    init.ops = &clk_fixed_factor_ops;
    init.flags = flags;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    init.parent_hws = parent_hw ? &parent_hw : core::ptr::null_mut();
    init.parent_data = pdata;
    if (parent_name || parent_hw || pdata)
    init.num_parents = 1;
    else
    init.num_parents = 0;
    hw = &fix.hw;
    if (dev)
    ret = clk_hw_register(dev, hw);
    else
    ret = of_clk_hw_register(np, hw);
    if (ret) {
    if (devm)
    devres_free(fix);
    else
    kfree(fix);
    hw = ERR_PTR(ret);
    } else if (devm)
    devres_add(dev, fix);
    return hw;
    }
    EXPORT_SYMBOL_GPL(__clk_hw_register_fixed_factor);
//
// devm_clk_hw_register_fixed_factor_index - Register a fixed factor clock with
// parent from DT index
// @dev: device that is registering this clock
// @name: name of this clock
// @index: index of phandle in @dev 'clocks' property
// @flags: fixed factor flags
// @mult: multiplier
// @div: divider
//
// Return: Pointer to fixed factor clk_hw structure that was registered or
// an error pointer.
//
    struct clk_hw *devm_clk_hw_register_fixed_factor_index(struct device *dev,
    const char *name, unsigned int index, unsigned long flags,
    unsigned int mult, unsigned int div)
    {
    let mut pdata: clk_parent_data = { .index = index };
    return __clk_hw_register_fixed_factor(dev, core::ptr::null_mut(), name, core::ptr::null_mut(), core::ptr::null_mut(), &pdata,
    flags, mult, div, 0, 0, true);
    }
    EXPORT_SYMBOL_GPL(devm_clk_hw_register_fixed_factor_index);
    struct clk_hw *clk_hw_register_fixed_factor_fwname(struct device *dev,
    struct device_node *np, const char *name, const char *fw_name,
    unsigned long flags, unsigned int mult, unsigned int div)
    {
    let mut pdata: clk_parent_data = { .index = -1, .fw_name = fw_name };
    return __clk_hw_register_fixed_factor(dev, np, name, core::ptr::null_mut(), core::ptr::null_mut(),
    &pdata, flags, mult, div, 0, 0, false);
    }
    EXPORT_SYMBOL_GPL(clk_hw_register_fixed_factor_fwname);
    struct clk_hw *clk_hw_register_fixed_factor_with_accuracy_fwname(struct device *dev,
    struct device_node *np, const char *name, const char *fw_name,
    unsigned long flags, unsigned int mult, unsigned int div,
    unsigned long acc)
    {
    let mut pdata: clk_parent_data = { .index = -1, .fw_name = fw_name };
    return __clk_hw_register_fixed_factor(dev, np, name, core::ptr::null_mut(), core::ptr::null_mut(),
    &pdata, flags, mult, div, acc,
    CLK_FIXED_FACTOR_FIXED_ACCURACY, false);
    }
    EXPORT_SYMBOL_GPL(clk_hw_register_fixed_factor_with_accuracy_fwname);
    struct clk_hw *clk_hw_register_fixed_factor_index(struct device *dev,
    const char *name, unsigned int index, unsigned long flags,
    unsigned int mult, unsigned int div)
    {
    let mut pdata: clk_parent_data = { .index = index };
    return __clk_hw_register_fixed_factor(dev, core::ptr::null_mut(), name, core::ptr::null_mut(), core::ptr::null_mut(), &pdata,
    flags, mult, div, 0, 0, false);
    }
    EXPORT_SYMBOL_GPL(clk_hw_register_fixed_factor_index);
    struct clk *clk_register_fixed_factor(struct device *dev, const char *name,
    const char *parent_name, unsigned long flags,
    unsigned int mult, unsigned int div)
    {
    struct clk_hw *hw;
    hw = clk_hw_register_fixed_factor(dev, name, parent_name, flags, mult,
    div);
    if (IS_ERR(hw))
    return ERR_CAST(hw);
    return hw.clk;
    }
    EXPORT_SYMBOL_GPL(clk_register_fixed_factor);
#[no_mangle]
pub unsafe extern "C" fn clk_unregister_fixed_factor(clk: *mut clk) {
    void clk_unregister_fixed_factor(struct clk *clk)
    {
    struct clk_hw *hw;
    hw = __clk_get_hw(clk);
    if (!hw)
    return;
    clk_unregister(clk);
    kfree(to_clk_fixed_factor(hw));
    }
    EXPORT_SYMBOL_GPL(clk_unregister_fixed_factor);
#[no_mangle]
pub unsafe extern "C" fn clk_hw_unregister_fixed_factor(hw: *mut clk_hw) {
    void clk_hw_unregister_fixed_factor(struct clk_hw *hw)
    {
    struct clk_fixed_factor *fix;
    fix = to_clk_fixed_factor(hw);
    clk_hw_unregister(hw);
    kfree(fix);
    }
    EXPORT_SYMBOL_GPL(clk_hw_unregister_fixed_factor);
    struct clk_hw *devm_clk_hw_register_fixed_factor_fwname(struct device *dev,
    struct device_node *np, const char *name, const char *fw_name,
    unsigned long flags, unsigned int mult, unsigned int div)
    {
    let mut pdata: clk_parent_data = { .index = -1, .fw_name = fw_name };
    return __clk_hw_register_fixed_factor(dev, np, name, core::ptr::null_mut(), core::ptr::null_mut(),
    &pdata, flags, mult, div, 0, 0, true);
    }
    EXPORT_SYMBOL_GPL(devm_clk_hw_register_fixed_factor_fwname);
    struct clk_hw *devm_clk_hw_register_fixed_factor_with_accuracy_fwname(struct device *dev,
    struct device_node *np, const char *name, const char *fw_name,
    unsigned long flags, unsigned int mult, unsigned int div,
    unsigned long acc)
    {
    let mut pdata: clk_parent_data = { .index = -1, .fw_name = fw_name };
    return __clk_hw_register_fixed_factor(dev, np, name, core::ptr::null_mut(), core::ptr::null_mut(),
    &pdata, flags, mult, div, acc,
    CLK_FIXED_FACTOR_FIXED_ACCURACY, true);
    }
    EXPORT_SYMBOL_GPL(devm_clk_hw_register_fixed_factor_with_accuracy_fwname);

    static struct clk_hw *_of_fixed_factor_clk_setup(struct device_node *node)
    {
    struct clk_hw *hw;
    const char *clk_name = node.name;
    let mut pdata: clk_parent_data = { .index = 0 };
    u32 div, mult;
    int ret;
    if (of_property_read_u32(node, "clock-div", &div)) {
    pr_err("%s Fixed factor clock <%pOFn> must have a clock-div property\n",
    __func__, node);
    return ERR_PTR(-EIO);
    }
    if (of_property_read_u32(node, "clock-mult", &mult)) {
    pr_err("%s Fixed factor clock <%pOFn> must have a clock-mult property\n",
    __func__, node);
    return ERR_PTR(-EIO);
    }
    of_property_read_string(node, "clock-output-names", &clk_name);
    hw = __clk_hw_register_fixed_factor(core::ptr::null_mut(), node, clk_name, core::ptr::null_mut(), core::ptr::null_mut(),
    &pdata, 0, mult, div, 0, 0, false);
    if (IS_ERR(hw)) {
//
// Clear OF_POPULATED flag so that clock registration can be
// attempted again from probe function.
//
    of_node_clear_flag(node, OF_POPULATED);
    return ERR_CAST(hw);
    }
    ret = of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw);
    if (ret) {
    clk_hw_unregister_fixed_factor(hw);
    return ERR_PTR(ret);
    }
    return hw;
    }
//
// of_fixed_factor_clk_setup() - Setup function for simple fixed factor clock
// @node:	device node for the clock
//
#[no_mangle]
pub unsafe extern "C" fn of_fixed_factor_clk_setup(node: *mut device_node) -> void __init {
    void __init of_fixed_factor_clk_setup(struct device_node *node)
    {
    _of_fixed_factor_clk_setup(node);
    }
    CLK_OF_DECLARE(fixed_factor_clk, "fixed-factor-clock",
    of_fixed_factor_clk_setup);
#[no_mangle]
unsafe extern "C" fn of_fixed_factor_clk_remove(pdev: *mut platform_device) {
    static void of_fixed_factor_clk_remove(struct platform_device *pdev)
    {
    struct clk_hw *clk = platform_get_drvdata(pdev);
    of_clk_del_provider(pdev.dev.of_node);
    clk_hw_unregister_fixed_factor(clk);
    }
#[no_mangle]
unsafe extern "C" fn of_fixed_factor_clk_probe(pdev: *mut platform_device) -> c_int {
    static int of_fixed_factor_clk_probe(struct platform_device *pdev)
    {
    struct clk_hw *clk;
//
// This function is not executed when of_fixed_factor_clk_setup
// succeeded.
//
    clk = _of_fixed_factor_clk_setup(pdev.dev.of_node);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    platform_set_drvdata(pdev, clk);
    return 0;
    }
    static const struct of_device_id of_fixed_factor_clk_ids[] = {
    { .compatible = "fixed-factor-clock" },
    { }
    };
    MODULE_DEVICE_TABLE(of, of_fixed_factor_clk_ids);
    static struct platform_driver of_fixed_factor_clk_driver = {
    .driver = {
    .name = "of_fixed_factor_clk",
    .of_match_table = of_fixed_factor_clk_ids,
    },
    .probe = of_fixed_factor_clk_probe,
    .remove = of_fixed_factor_clk_remove,
    };
    builtin_platform_driver(of_fixed_factor_clk_driver);
