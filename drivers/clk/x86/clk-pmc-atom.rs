//! Automatically rewritten from C to Rust
//! Source: drivers/clk/x86/clk-pmc-atom.c
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
// Intel Atom platform clocks driver for BayTrail and CherryTrail SoCs
//
// Copyright (C) 2016, Intel Corporation
// Author: Irina Tirdea <irina.tirdea@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_plt_fixed {
    pub clk: *mut clk_hw,
    pub lookup: *mut clk_lookup,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_plt {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub lookup: *mut clk_lookup,
// protect access to PMC registers
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_plt_data {
    pub parents: *mut clk_plt_fixed,
    pub nparents: u8,
    pub clks: [*mut clk_plt; PMC_CLK_NUM],
    pub mclk_lookup: *mut clk_lookup,
    pub ether_clk_lookup: *mut clk_lookup,
}

// Return an index in parent table
#[no_mangle]
pub unsafe extern "C" fn plt_reg_to_parent(reg: c_int) -> c_int {
    static inline int plt_reg_to_parent(int reg)
    {
    switch (reg & PMC_MASK_CLK_FREQ) {
    default:
    case PMC_CLK_FREQ_XTAL:
    return 0;
    case PMC_CLK_FREQ_PLL:
    return 1;
    }
    }
// Return clk index of parent
#[no_mangle]
pub unsafe extern "C" fn plt_parent_to_reg(index: c_int) -> c_int {
    static inline int plt_parent_to_reg(int index)
    {
    switch (index) {
    default:
    case 0:
    return PMC_CLK_FREQ_XTAL;
    case 1:
    return PMC_CLK_FREQ_PLL;
    }
    }
// Abstract status in simpler enabled/disabled value
#[no_mangle]
pub unsafe extern "C" fn plt_reg_to_enabled(reg: c_int) -> c_int {
    static inline int plt_reg_to_enabled(int reg)
    {
    switch (reg & PMC_MASK_CLK_CTL) {
    case PMC_CLK_CTL_GATED_ON_D3:
    case PMC_CLK_CTL_FORCE_ON:
    return 1;	/* enabled */
    case PMC_CLK_CTL_FORCE_OFF:
    case PMC_CLK_CTL_RESERVED:
    default:
    return 0;	/* disabled */
    }
    }
#[no_mangle]
unsafe extern "C" fn plt_clk_reg_update(clk: *mut clk_plt, mask: u32, val: u32) {
    static void plt_clk_reg_update(struct clk_plt *clk, u32 mask, u32 val)
    {
    u32 tmp;
    unsigned long flags;
    spin_lock_irqsave(&clk.lock, flags);
    tmp = readl(clk.reg);
    tmp = (tmp & ~mask) | (val & mask);
    writel(tmp, clk.reg);
    spin_unlock_irqrestore(&clk.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn plt_clk_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int plt_clk_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_plt *clk = to_clk_plt(hw);
    plt_clk_reg_update(clk, PMC_MASK_CLK_FREQ, plt_parent_to_reg(index));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn plt_clk_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 plt_clk_get_parent(struct clk_hw *hw)
    {
    struct clk_plt *clk = to_clk_plt(hw);
    u32 value;
    value = readl(clk.reg);
    return plt_reg_to_parent(value);
    }
#[no_mangle]
unsafe extern "C" fn plt_clk_enable(hw: *mut clk_hw) -> c_int {
    static int plt_clk_enable(struct clk_hw *hw)
    {
    struct clk_plt *clk = to_clk_plt(hw);
    plt_clk_reg_update(clk, PMC_MASK_CLK_CTL, PMC_CLK_CTL_FORCE_ON);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn plt_clk_disable(hw: *mut clk_hw) {
    static void plt_clk_disable(struct clk_hw *hw)
    {
    struct clk_plt *clk = to_clk_plt(hw);
    plt_clk_reg_update(clk, PMC_MASK_CLK_CTL, PMC_CLK_CTL_FORCE_OFF);
    }
#[no_mangle]
unsafe extern "C" fn plt_clk_is_enabled(hw: *mut clk_hw) -> c_int {
    static int plt_clk_is_enabled(struct clk_hw *hw)
    {
    struct clk_plt *clk = to_clk_plt(hw);
    u32 value;
    value = readl(clk.reg);
    return plt_reg_to_enabled(value);
    }
    static const struct clk_ops plt_clk_ops = {
    .enable = plt_clk_enable,
    .disable = plt_clk_disable,
    .is_enabled = plt_clk_is_enabled,
    .get_parent = plt_clk_get_parent,
    .set_parent = plt_clk_set_parent,
    .determine_rate = __clk_mux_determine_rate,
    };
    static struct clk_plt *plt_clk_register(struct platform_device *pdev, int id,
    const struct pmc_clk_data *pmc_data,
    const char **parent_names,
    int num_parents)
    {
    struct clk_plt *pclk;
    struct clk_init_data init;
    int ret;
    pclk = devm_kzalloc(&pdev.dev, sizeof(*pclk), GFP_KERNEL);
    if (!pclk)
    return ERR_PTR(-ENOMEM);
    init.name =  kasprintf(GFP_KERNEL, "%s_%d", PLT_CLK_NAME_BASE, id);
    if (!init.name)
    return ERR_PTR(-ENOMEM);
    init.ops = &plt_clk_ops;
    init.flags = 0;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    pclk.hw.init = &init;
    pclk.reg = pmc_data.base + PMC_CLK_CTL_OFFSET + id * PMC_CLK_CTL_SIZE;
    spin_lock_init(&pclk.lock);
//
// On some systems, the pmc_plt_clocks already enabled by the
// firmware are being marked as critical to avoid them being
// gated by the clock framework.
//
    if (pmc_data.critical && plt_clk_is_enabled(&pclk.hw))
    init.flags |= CLK_IS_CRITICAL;
    ret = devm_clk_hw_register(&pdev.dev, &pclk.hw);
    if (ret) {
    pclk = ERR_PTR(ret);
    goto err_free_init;
    }
    pclk.lookup = clkdev_hw_create(&pclk.hw, init.name, core::ptr::null_mut());
    if (!pclk.lookup) {
    pclk = ERR_PTR(-ENOMEM);
    goto err_free_init;
    }
    err_free_init:
    kfree(init.name);
    return pclk;
    }
#[no_mangle]
unsafe extern "C" fn plt_clk_unregister(pclk: *mut clk_plt) {
    static void plt_clk_unregister(struct clk_plt *pclk)
    {
    clkdev_drop(pclk.lookup);
    }
    static struct clk_plt_fixed *plt_clk_register_fixed_rate(struct platform_device *pdev,
    const char *name,
    const char *parent_name,
    unsigned long fixed_rate)
    {
    struct clk_plt_fixed *pclk;
    pclk = devm_kzalloc(&pdev.dev, sizeof(*pclk), GFP_KERNEL);
    if (!pclk)
    return ERR_PTR(-ENOMEM);
    pclk.clk = clk_hw_register_fixed_rate(&pdev.dev, name, parent_name,
    0, fixed_rate);
    if (IS_ERR(pclk.clk))
    return ERR_CAST(pclk.clk);
    pclk.lookup = clkdev_hw_create(pclk.clk, name, core::ptr::null_mut());
    if (!pclk.lookup) {
    clk_hw_unregister_fixed_rate(pclk.clk);
    return ERR_PTR(-ENOMEM);
    }
    return pclk;
    }
#[no_mangle]
unsafe extern "C" fn plt_clk_unregister_fixed_rate(pclk: *mut clk_plt_fixed) {
    static void plt_clk_unregister_fixed_rate(struct clk_plt_fixed *pclk)
    {
    clkdev_drop(pclk.lookup);
    clk_hw_unregister_fixed_rate(pclk.clk);
    }
    static void plt_clk_unregister_fixed_rate_loop(struct clk_plt_data *data,
    unsigned int i)
    {
    while (i--)
    plt_clk_unregister_fixed_rate(data.parents[i]);
    }
    static void plt_clk_free_parent_names_loop(const char **parent_names,
    unsigned int i)
    {
    while (i--)
    kfree_const(parent_names[i]);
    kfree(parent_names);
    }
    static void plt_clk_unregister_loop(struct clk_plt_data *data,
    unsigned int i)
    {
    while (i--)
    plt_clk_unregister(data.clks[i]);
    }
    static const char **plt_clk_register_parents(struct platform_device *pdev,
    struct clk_plt_data *data,
    const struct pmc_clk *clks)
    {
    const char **parent_names;
    unsigned int i;
    int err;
    let mut nparents: c_int = 0;
    data.nparents = 0;
    while (clks[nparents].name)
    nparents++;
    data.parents = devm_kcalloc(&pdev.dev, nparents,
    sizeof(*data.parents), GFP_KERNEL);
    if (!data.parents)
    return ERR_PTR(-ENOMEM);
    parent_names = kcalloc(nparents, sizeof(*parent_names),
    GFP_KERNEL);
    if (!parent_names)
    return ERR_PTR(-ENOMEM);
    for (i = 0; i < nparents; i++) {
    data.parents[i] =
    plt_clk_register_fixed_rate(pdev, clks[i].name,
    clks[i].parent_name,
    clks[i].freq);
    if (IS_ERR(data.parents[i])) {
    err = PTR_ERR(data.parents[i]);
    goto err_unreg;
    }
    parent_names[i] = kstrdup_const(clks[i].name, GFP_KERNEL);
    }
    data.nparents = nparents;
    return parent_names;
    err_unreg:
    plt_clk_unregister_fixed_rate_loop(data, i);
    plt_clk_free_parent_names_loop(parent_names, i);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn plt_clk_unregister_parents(data: *mut clk_plt_data) {
    static void plt_clk_unregister_parents(struct clk_plt_data *data)
    {
    plt_clk_unregister_fixed_rate_loop(data, data.nparents);
    }
#[no_mangle]
unsafe extern "C" fn plt_clk_probe(pdev: *mut platform_device) -> c_int {
    static int plt_clk_probe(struct platform_device *pdev)
    {
    const struct pmc_clk_data *pmc_data;
    const char **parent_names;
    struct clk_plt_data *data;
    unsigned int i;
    int err;
    pmc_data = dev_get_platdata(&pdev.dev);
    if (!pmc_data || !pmc_data.clks)
    return -EINVAL;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    parent_names = plt_clk_register_parents(pdev, data, pmc_data.clks);
    if (IS_ERR(parent_names))
    return PTR_ERR(parent_names);
    for (i = 0; i < PMC_CLK_NUM; i++) {
    data.clks[i] = plt_clk_register(pdev, i, pmc_data,
    parent_names, data.nparents);
    if (IS_ERR(data.clks[i])) {
    err = PTR_ERR(data.clks[i]);
    goto err_unreg_clk_plt;
    }
    }
    data.mclk_lookup = clkdev_hw_create(&data.clks[3].hw, "mclk", core::ptr::null_mut());
    if (!data.mclk_lookup) {
    err = -ENOMEM;
    goto err_unreg_clk_plt;
    }
    data.ether_clk_lookup = clkdev_hw_create(&data.clks[4].hw,
    "ether_clk", core::ptr::null_mut());
    if (!data.ether_clk_lookup) {
    err = -ENOMEM;
    goto err_drop_mclk;
    }
    plt_clk_free_parent_names_loop(parent_names, data.nparents);
    platform_set_drvdata(pdev, data);
    return 0;
    err_drop_mclk:
    clkdev_drop(data.mclk_lookup);
    err_unreg_clk_plt:
    plt_clk_unregister_loop(data, i);
    plt_clk_unregister_parents(data);
    plt_clk_free_parent_names_loop(parent_names, data.nparents);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn plt_clk_remove(pdev: *mut platform_device) {
    static void plt_clk_remove(struct platform_device *pdev)
    {
    struct clk_plt_data *data;
    data = platform_get_drvdata(pdev);
    clkdev_drop(data.ether_clk_lookup);
    clkdev_drop(data.mclk_lookup);
    plt_clk_unregister_loop(data, PMC_CLK_NUM);
    plt_clk_unregister_parents(data);
    }
    static struct platform_driver plt_clk_driver = {
    .driver = {
    .name = "clk-pmc-atom",
    },
    .probe = plt_clk_probe,
    .remove = plt_clk_remove,
    };
    builtin_platform_driver(plt_clk_driver);
