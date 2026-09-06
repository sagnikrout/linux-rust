//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ti/fixed-factor.c
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
// TI Fixed Factor Clock
//
// Copyright (C) 2013 Texas Instruments, Inc.
//
// Tero Kristo <t-kristo@ti.com>
//

//
// of_ti_fixed_factor_clk_setup - Setup function for TI fixed factor clock
// @node: device node for this clock
//
// Sets up a simple fixed factor clock based on device tree info.
//
#[no_mangle]
unsafe extern "C" fn of_ti_fixed_factor_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_ti_fixed_factor_clk_setup(struct device_node *node)
    {
    struct clk *clk;
    const char *clk_name = ti_dt_clk_name(node);
    const char *parent_name;
    u32 div, mult;
    let mut flags: u32 = 0;
    if (of_property_read_u32(node, "ti,clock-div", &div)) {
    pr_err("%pOFn must have a clock-div property\n", node);
    return;
    }
    if (of_property_read_u32(node, "ti,clock-mult", &mult)) {
    pr_err("%pOFn must have a clock-mult property\n", node);
    return;
    }
    if (of_property_read_bool(node, "ti,set-rate-parent"))
    flags |= CLK_SET_RATE_PARENT;
    parent_name = of_clk_get_parent_name(node, 0);
    clk = clk_register_fixed_factor(core::ptr::null_mut(), clk_name, parent_name, flags,
    mult, div);
    if (!IS_ERR(clk)) {
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    of_ti_clk_autoidle_setup(node);
    ti_clk_add_alias(clk, clk_name);
    }
    }
    CLK_OF_DECLARE(ti_fixed_factor_clk, "ti,fixed-factor-clock",
    of_ti_fixed_factor_clk_setup);
