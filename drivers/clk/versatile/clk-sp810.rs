//! Automatically rewritten from C to Rust
//! Source: drivers/clk/versatile/clk-sp810.c
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
// Copyright (C) 2013 ARM Limited
//

    container_of(_hw, struct clk_sp810_timerclken, hw)
    struct clk_sp810;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_sp810_timerclken {
    pub hw: clk_hw,
    pub clk: *mut clk,
    pub sp810: *mut clk_sp810,
    pub channel: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_sp810 {
    pub node: *mut device_node,
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
    pub timerclken: [clk_sp810_timerclken; 4],
}

#[no_mangle]
unsafe extern "C" fn clk_sp810_timerclken_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_sp810_timerclken_get_parent(struct clk_hw *hw)
    {
    struct clk_sp810_timerclken *timerclken = to_clk_sp810_timerclken(hw);
    let mut val: u32 = readl(timerclken.sp810.base + SCCTRL);
    return !!(val & (1 << SCCTRL_TIMERENnSEL_SHIFT(timerclken.channel)));
    }
#[no_mangle]
unsafe extern "C" fn clk_sp810_timerclken_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int clk_sp810_timerclken_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_sp810_timerclken *timerclken = to_clk_sp810_timerclken(hw);
    struct clk_sp810 *sp810 = timerclken.sp810;
    u32 val, shift = SCCTRL_TIMERENnSEL_SHIFT(timerclken.channel);
    let mut flags: c_ulong = 0;
    if (WARN_ON(index > 1))
    return -EINVAL;
    spin_lock_irqsave(&sp810.lock, flags);
    val = readl(sp810.base + SCCTRL);
    val &= ~(1 << shift);
    val |= index << shift;
    writel(val, sp810.base + SCCTRL);
    spin_unlock_irqrestore(&sp810.lock, flags);
    return 0;
    }
    static const struct clk_ops clk_sp810_timerclken_ops = {
    .determine_rate = clk_hw_determine_rate_no_reparent,
    .get_parent = clk_sp810_timerclken_get_parent,
    .set_parent = clk_sp810_timerclken_set_parent,
    };
    static struct clk *clk_sp810_timerclken_of_get(struct of_phandle_args *clkspec,
    void *data)
    {
    struct clk_sp810 *sp810 = data;
    if (WARN_ON(clkspec.args_count != 1 ||
    clkspec.args[0] >=	ARRAY_SIZE(sp810.timerclken)))
    return core::ptr::null_mut();
    return sp810.timerclken[clkspec.args[0]].clk;
    }
#[no_mangle]
unsafe extern "C" fn clk_sp810_of_setup(node: *mut device_node) -> void __init {
    static void __init clk_sp810_of_setup(struct device_node *node)
    {
    struct clk_sp810 *sp810 = kzalloc_obj(*sp810);
    const char *parent_names[2];
    let mut num: c_int = ARRAY_SIZE(parent_names);
    char name[12];
    struct clk_init_data init;
    static int instance;
    int i;
    bool deprecated;
    if (!sp810)
    return;
    if (of_clk_parent_fill(node, parent_names, num) != num) {
    pr_warn("Failed to obtain parent clocks for SP810!\n");
    kfree(sp810);
    return;
    }
    sp810.node = node;
    sp810.base = of_iomap(node, 0);
    spin_lock_init(&sp810.lock);
    init.name = name;
    init.ops = &clk_sp810_timerclken_ops;
    init.flags = 0;
    init.parent_names = parent_names;
    init.num_parents = num;
    deprecated = !of_property_present(node, "assigned-clock-parents");
    for (i = 0; i < ARRAY_SIZE(sp810.timerclken); i++) {
    snprintf(name, sizeof(name), "sp810_%d_%d", instance, i);
    sp810.timerclken[i].sp810 = sp810;
    sp810.timerclken[i].channel = i;
    sp810.timerclken[i].hw.init = &init;
//
// If DT isn't setting the parent, force it to be
// the 1 MHz clock without going through the framework.
// We do this before clk_register() so that it can determine
// the parent and setup the tree properly.
//
    if (deprecated)
    init.ops.set_parent(&sp810.timerclken[i].hw, 1);
    sp810.timerclken[i].clk = clk_register(core::ptr::null_mut(),
    &sp810.timerclken[i].hw);
    WARN_ON(IS_ERR(sp810.timerclken[i].clk));
    }
    of_clk_add_provider(node, clk_sp810_timerclken_of_get, sp810);
    instance++;
    }
    CLK_OF_DECLARE(sp810, "arm,sp810", clk_sp810_of_setup);
