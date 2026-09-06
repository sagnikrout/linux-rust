//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-moxart.c
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
// MOXA ART SoCs clock driver.
//
// Copyright (C) 2013 Jonas Jensen
//
// Jonas Jensen <jonas.jensen@gmail.com>
//

#[no_mangle]
unsafe extern "C" fn moxart_of_pll_clk_init(node: *mut device_node) -> void __init {
    static void __init moxart_of_pll_clk_init(struct device_node *node)
    {
    void __iomem *base;
    struct clk_hw *hw;
    unsigned int mul;
    const char *name = node.name;
    const char *parent_name;
    of_property_read_string(node, "clock-output-names", &name);
    parent_name = of_clk_get_parent_name(node, 0);
    base = of_iomap(node, 0);
    if (!base) {
    pr_err("%pOF: of_iomap failed\n", node);
    return;
    }
    mul = readl(base + 0x30) >> 3 & 0x3f;
    iounmap(base);
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), name, parent_name, 0, mul, 1);
    if (IS_ERR(hw)) {
    pr_err("%pOF: failed to register clock\n", node);
    return;
    }
    clk_hw_register_clkdev(hw, core::ptr::null_mut(), name);
    of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw);
    }
    CLK_OF_DECLARE(moxart_pll_clock, "moxa,moxart-pll-clock",
    moxart_of_pll_clk_init);
#[no_mangle]
unsafe extern "C" fn moxart_of_apb_clk_init(node: *mut device_node) -> void __init {
    static void __init moxart_of_apb_clk_init(struct device_node *node)
    {
    void __iomem *base;
    struct clk_hw *hw;
    unsigned int div, val;
    unsigned int div_idx[] = { 2, 3, 4, 6, 8};
    const char *name = node.name;
    const char *parent_name;
    of_property_read_string(node, "clock-output-names", &name);
    parent_name = of_clk_get_parent_name(node, 0);
    base = of_iomap(node, 0);
    if (!base) {
    pr_err("%pOF: of_iomap failed\n", node);
    return;
    }
    val = readl(base + 0xc) >> 4 & 0x7;
    iounmap(base);
    if (val > 4)
    val = 0;
    div = div_idx[val] * 2;
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), name, parent_name, 0, 1, div);
    if (IS_ERR(hw)) {
    pr_err("%pOF: failed to register clock\n", node);
    return;
    }
    clk_hw_register_clkdev(hw, core::ptr::null_mut(), name);
    of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw);
    }
    CLK_OF_DECLARE(moxart_apb_clock, "moxa,moxart-apb-clock",
    moxart_of_apb_clk_init);
