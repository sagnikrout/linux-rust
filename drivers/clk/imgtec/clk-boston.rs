//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imgtec/clk-boston.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016-2017 Imagination Technologies
// Author: Paul Burton <paul.burton@mips.com>
//

pub const BOSTON_PLAT_MMCMDIV: c_uint = 0x30;

pub const BOSTON_CLK_COUNT: c_int = 3;
#[no_mangle]
unsafe extern "C" fn ext_field(val: u32, mask: u32) -> u32 {
    static u32 ext_field(u32 val, u32 mask)
    {
    return (val & mask) >> (ffs(mask) - 1);
    }
#[no_mangle]
unsafe extern "C" fn clk_boston_setup(np: *mut device_node) -> void __init {
    static void __init clk_boston_setup(struct device_node *np)
    {
    unsigned long in_freq, cpu_freq, sys_freq;
    uint mmcmdiv, mul, cpu_div, sys_div;
    struct clk_hw_onecell_data *onecell;
    struct regmap *regmap;
    struct clk_hw *hw;
    int err;
    regmap = syscon_node_to_regmap(np.parent);
    if (IS_ERR(regmap)) {
    pr_err("failed to find regmap\n");
    return;
    }
    err = regmap_read(regmap, BOSTON_PLAT_MMCMDIV, &mmcmdiv);
    if (err) {
    pr_err("failed to read mmcm_div register: %d\n", err);
    return;
    }
    in_freq = ext_field(mmcmdiv, BOSTON_PLAT_MMCMDIV_INPUT) * 1000000;
    mul = ext_field(mmcmdiv, BOSTON_PLAT_MMCMDIV_MUL);
    sys_div = ext_field(mmcmdiv, BOSTON_PLAT_MMCMDIV_CLK0DIV);
    sys_freq = mult_frac(in_freq, mul, sys_div);
    cpu_div = ext_field(mmcmdiv, BOSTON_PLAT_MMCMDIV_CLK1DIV);
    cpu_freq = mult_frac(in_freq, mul, cpu_div);
    onecell = kzalloc_flex(*onecell, hws, BOSTON_CLK_COUNT);
    if (!onecell)
    return;
    onecell.num = BOSTON_CLK_COUNT;
    hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), "input", core::ptr::null_mut(), 0, in_freq);
    if (IS_ERR(hw)) {
    pr_err("failed to register input clock: %pe\n", hw);
    goto fail_input;
    }
    onecell.hws[BOSTON_CLK_INPUT] = hw;
    hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), "sys", "input", 0, sys_freq);
    if (IS_ERR(hw)) {
    pr_err("failed to register sys clock: %pe\n", hw);
    goto fail_sys;
    }
    onecell.hws[BOSTON_CLK_SYS] = hw;
    hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), "cpu", "input", 0, cpu_freq);
    if (IS_ERR(hw)) {
    pr_err("failed to register cpu clock: %pe\n", hw);
    goto fail_cpu;
    }
    onecell.hws[BOSTON_CLK_CPU] = hw;
    err = of_clk_add_hw_provider(np, of_clk_hw_onecell_get, onecell);
    if (err) {
    pr_err("failed to add DT provider: %d\n", err);
    goto fail_clk_add;
    }
    return;
    fail_clk_add:
    clk_hw_unregister_fixed_rate(onecell.hws[BOSTON_CLK_CPU]);
    fail_cpu:
    clk_hw_unregister_fixed_rate(onecell.hws[BOSTON_CLK_SYS]);
    fail_sys:
    clk_hw_unregister_fixed_rate(onecell.hws[BOSTON_CLK_INPUT]);
    fail_input:
    kfree(onecell);
    }
//
// Use CLK_OF_DECLARE so that this driver is probed early enough to provide the
// CPU frequency for use with the GIC or cop0 counters/timers.
//
    CLK_OF_DECLARE(clk_boston, "img,boston-clock", clk_boston_setup);
